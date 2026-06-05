//! Substitution map building, text replacement, and fragment parsing.

#![forbid(unsafe_code)]

use crate::ast::macro_nodes::ModuleMacroStmt;
use crate::ast::pattern::{PatternArg, PatternCall, PatternDef, ReflectBlock};
use crate::ast::types::SignalKind;
use crate::error::MirrError;

use super::pattern_err;

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
                return Err(MirrError::PatternError {
                    message: format!("{} Pattern '{}' parameter '{}' expects a signal reference, got a constant.", crate::error_codes::ec(400),
                        def.name, param.name
                    ),
                    span: call.span
                });
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
            (
                crate::ast::pattern::PatternParamKind::Constant { .. },
                PatternArg::SignalRef(name),
            ) => {
                // BUG-4: If we have a composed pattern call like ${m}(${s}, ${t}, ${out}),
                // '${t}' is parsed as a SignalRef, but it might be substituted with a constant later.
                // However, at this point, if it starts with '${', we should allow it.
                // ALSO, if it's a numeric literal string, treat it as a constant.
                if name.starts_with("${")
                    || name.parse::<u64>().is_ok()
                    || name == "true"
                    || name == "false"
                {
                    name.clone()
                } else {
                    return Err(pattern_err(format!(
                        "Pattern '{}' parameter '{}' expects a constant, got a signal reference '{}'.",
                        def.name, param.name, name
                    )));
                }
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
                return Err(pattern_err(format!("{} Pattern '{}' parameter '{}' has kind 'pattern' but argument is not a pattern reference.", crate::error_codes::ec(426),
                    def.name, param.name
                )));
            }
            // Signal/Constant params do not accept pattern refs.
            (_, PatternArg::PatternRef(_)) => {
                return Err(pattern_err(format!(
                    "{} Pattern '{}' parameter '{}' does not accept a pattern reference.",
                    crate::error_codes::ec(427),
                    def.name,
                    param.name
                )));
            }
        };
        subs.push((param.name.clone(), replacement.clone()));
    }

    Ok(subs)
}

/// Validate that all signals in the expanded fragment are internal.
///
/// Input/output signals must be passed as parameters, not declared inside
/// the reflect block. This enforces the explicit-external-references rule.
pub(super) fn validate_fragment_signals(
    fragment: &ReflectBlock,
    pattern_name: &str,
) -> Result<(), MirrError> {
    for stmt in &fragment.statements {
        validate_stmt_signals(stmt, pattern_name)?;
    }
    Ok(())
}

fn validate_stmt_signals(stmt: &ModuleMacroStmt, pattern_name: &str) -> Result<(), MirrError> {
    match stmt {
        ModuleMacroStmt::Signal(sig) => {
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
        ModuleMacroStmt::ForLoop { body, .. } => {
            for s in body {
                validate_stmt_signals(s, pattern_name)?;
            }
        }
        _ => {}
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
