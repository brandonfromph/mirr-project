//! Substitution map building, text replacement, and fragment parsing.

#![forbid(unsafe_code)]

use crate::ast::pattern::{PatternArg, PatternCall, PatternDef};
use crate::ast::types::SignalKind;
use crate::error::MirrError;

use super::{pattern_err, ExpandedFragment};

pub(super) fn build_substitution_map(
    def: &PatternDef,
    call: &PatternCall,
) -> Result<Vec<(String, String)>, MirrError> {
    let mut subs = Vec::with_capacity(def.params.len());

    for (param, arg) in def.params.iter().zip(call.arguments.iter()) {
        let replacement = match (&param.kind, arg) {
            (crate::ast::pattern::PatternParamKind::Signal { .. }, PatternArg::SignalRef(name)) => {
                name.clone()
            }
            (
                crate::ast::pattern::PatternParamKind::Signal { .. },
                PatternArg::ConstInt(_) | PatternArg::ConstBool(_),
            ) => {
                return Err(pattern_err(format!(
                    "Pattern '{}' parameter '{}' expects a signal reference, got a constant.",
                    def.name, param.name
                )));
            }
            (crate::ast::pattern::PatternParamKind::Constant { .. }, PatternArg::ConstInt(n)) => {
                format!("{n}")
            }
            (crate::ast::pattern::PatternParamKind::Constant { .. }, PatternArg::ConstBool(b)) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            (crate::ast::pattern::PatternParamKind::Constant { .. }, PatternArg::SignalRef(_)) => {
                return Err(pattern_err(format!(
                    "Pattern '{}' parameter '{}' expects a constant, got a signal reference.",
                    def.name, param.name
                )));
            }
            // Higher-order: pattern parameter accepts a pattern name.
            // PatternRef is produced when the parser explicitly resolves it;
            // SignalRef is the common case because the parser cannot distinguish
            // pattern names from signal names at parse time.
            (crate::ast::pattern::PatternParamKind::Pattern, PatternArg::PatternRef(name))
            | (crate::ast::pattern::PatternParamKind::Pattern, PatternArg::SignalRef(name)) => {
                name.clone()
            }
            (crate::ast::pattern::PatternParamKind::Pattern, _) => {
                return Err(pattern_err(format!(
                    "[E426] Pattern '{}' parameter '{}' has kind 'pattern' but argument is not a pattern reference.",
                    def.name, param.name
                )));
            }
            // Signal/Constant params do not accept pattern refs.
            (_, PatternArg::PatternRef(_)) => {
                return Err(pattern_err(format!(
                    "[E427] Pattern '{}' parameter '{}' does not accept a pattern reference.",
                    def.name, param.name
                )));
            }
        };
        subs.push((param.name.clone(), replacement));
    }

    Ok(subs)
}

/// Apply parameter substitution to a single line.
///
/// Replaces all occurrences of `${param_name}` with the corresponding value.
/// Iterates over all substitution pairs (max 32) for the line.
/// No re-expansion of substituted text (prevents injection).
pub(super) fn substitute_line(line: &str, subs: &[(String, String)]) -> String {
    let mut result = line.to_string();
    for (key, value) in subs {
        let marker = format!("${{{key}}}");
        // Use iterative replacement bounded by marker count.
        // In practice, each marker appears at most a few times per line.
        let mut search_from = 0usize;
        let mut max_replacements = 64usize;
        while max_replacements > 0 {
            if let Some(pos) = result[search_from..].find(&marker) {
                let abs_pos = search_from + pos;
                result.replace_range(abs_pos..abs_pos + marker.len(), value);
                search_from = abs_pos + value.len();
                max_replacements -= 1;
            } else {
                break;
            }
        }
    }
    result
}

/// Parse substituted lines as a module fragment.
///
/// Wraps lines in a synthetic `module __expand__ { ... }` and calls
/// the existing `parse_mirr()` to reuse 100% of parser infrastructure.
///
/// Returns the extracted signals, guards, reflexes, and properties.
pub(super) fn parse_reflect_fragment(
    lines: &[String],
    pattern_name: &str,
) -> Result<ExpandedFragment, MirrError> {
    // Build synthetic source.
    let mut source = String::with_capacity(lines.len() * 80 + 64);
    source.push_str("module __expand__ {\n");
    for line in lines {
        source.push_str("    ");
        source.push_str(line);
        source.push('\n');
    }
    source.push_str("}\n");

    // Parse using existing parser.
    let program = crate::parser::parse_mirr(&source)
        .map_err(|e| pattern_err(format!("In pattern '{}' reflect body: {}", pattern_name, e)))?;

    Ok(ExpandedFragment {
        signals: program.module.signals,
        guards: program.module.guards,
        reflexes: program.module.reflexes,
        properties: program.module.properties,
        pattern_calls: program.module.pattern_calls,
    })
}

/// Validate that all signals in the expanded fragment are internal.
///
/// Input/output signals must be passed as parameters, not declared inside
/// the reflect block. This enforces the explicit-external-references rule.
pub(super) fn validate_fragment_signals(
    fragment: &ExpandedFragment,
    pattern_name: &str,
) -> Result<(), MirrError> {
    for sig in &fragment.signals {
        if sig.kind != SignalKind::Internal {
            return Err(pattern_err(format!(
                "Pattern '{}' reflect block declares {} signal '{}'. \
                 Only internal signals may be declared inside reflect. \
                 Use signal parameters for inputs and outputs.",
                pattern_name,
                match sig.kind {
                    SignalKind::Input => "input",
                    SignalKind::Output => "output",
                    SignalKind::Internal => "internal",
                },
                sig.name,
            )));
        }
    }
    Ok(())
}

/// Build human-readable argument summary for annotations.
pub(super) fn build_args_summary(args: &[PatternArg]) -> String {
    let parts: Vec<String> = args
        .iter()
        .map(|a| match a {
            PatternArg::SignalRef(name) => name.clone(),
            PatternArg::ConstInt(n) => format!("{n}"),
            PatternArg::ConstBool(b) => format!("{b}"),
            PatternArg::PatternRef(name) => name.clone(),
        })
        .collect();
    parts.join(", ")
}
