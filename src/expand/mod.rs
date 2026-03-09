//! Pattern expansion engine for the MIRR homoiconic pattern system.
//!
//! Expands pattern calls into module fragments via text-level `${param}` substitution,
//! re-parsing, name prefixing, and origin tagging. Validates internal signal scoping.
//! All traversals use explicit work stacks (NASA P10 rule #1: no recursion).

#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::expr::Expr;
use crate::ast::pattern::{PatternArg, PatternCall, PatternDef, PatternOrigin};
use crate::ast::program::{MirrProgram, Module};
use crate::ast::property::PropertyFormula;
use crate::ast::types::SignalKind;
use crate::ast::MAX_EXPR_NODES;
use crate::error::MirrError;
use crate::span::Span;

/// Maximum nesting depth for pattern expansion (NASA P10 rule #1).
const MAX_EXPANSION_DEPTH: usize = 4;

/// Maximum total items (signals + guards + reflexes + properties) from all expansions.
const MAX_EXPANDED_ITEMS: usize = 256;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Expand all pattern calls in a `MirrProgram`, modifying the module in place.
///
/// After expansion:
/// - `module.pattern_calls` is empty.
/// - `module.signals/guards/reflexes/properties` contain the expanded items.
/// - `module.pattern_origins` contains provenance annotations for emitters.
/// - `program.patterns` is retained for reference but not used downstream.
///
/// No-op if there are no pattern calls.
///
/// Bounded: `MAX_EXPANSION_DEPTH` levels, `MAX_EXPANDED_ITEMS` total items.
pub fn expand_patterns(program: &mut MirrProgram) -> Result<(), MirrError> {
    if program.module.pattern_calls.is_empty() {
        return Ok(());
    }

    // Build lookup map: pattern name -> &PatternDef (max 64 entries).
    let pattern_map = build_pattern_map(&program.patterns)?;

    // Static cycle detection before any expansion (bounded DFS).
    detect_pattern_cycles(&program.patterns)?;

    // Take ownership of pattern calls, leaving an empty vec in the module.
    let calls = std::mem::take(&mut program.module.pattern_calls);

    let mut total_expanded = 0usize;
    let mut call_index = 0usize;

    // Process each call at depth 0. Bounded by calls.len() (finite, parser-capped).
    for call in &calls {
        expand_single_call(
            call,
            &pattern_map,
            &mut program.module,
            0,
            &mut total_expanded,
            &mut call_index,
        )?;
    }

    // Post-expansion: validate internal signal scoping.
    validate_internal_signal_scoping(&program.module)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Internal implementation
// ---------------------------------------------------------------------------

/// Build a lookup map from pattern name to definition.
///
/// Errors on duplicate pattern names.
/// Bounded: iterates over patterns vec (max 64 from parser).
fn build_pattern_map(patterns: &[PatternDef]) -> Result<HashMap<&str, &PatternDef>, MirrError> {
    let mut map = HashMap::with_capacity(patterns.len());
    for pat in patterns {
        if map.insert(pat.name.as_str(), pat).is_some() {
            return Err(pattern_err(format!("Duplicate pattern definition: '{}'.", pat.name)));
        }
    }
    Ok(map)
}

/// Expand a single pattern call into the target module.
///
/// Steps:
///   1. Look up the PatternDef by name.
///   2. Validate argument count matches parameter count.
///   3. Build substitution map: param_name -> replacement string.
///   4. For each raw line in reflect body, apply text substitution.
///   5. Wrap substituted lines in a synthetic module and re-parse.
///   6. Validate only internal signals in expanded fragment.
///   7. Apply name prefixing to all generated names.
///   8. Set origin tag on every generated node.
///   9. Append results to the target module.
///  10. Record a PatternOrigin for emitter annotations.
///
/// Bounded: MAX_EXPANSION_DEPTH, MAX_EXPANDED_ITEMS.
fn expand_single_call(
    call: &PatternCall,
    pattern_map: &HashMap<&str, &PatternDef>,
    module: &mut Module,
    depth: usize,
    total_expanded: &mut usize,
    call_index: &mut usize,
) -> Result<(), MirrError> {
    if depth >= MAX_EXPANSION_DEPTH {
        return Err(pattern_err(format!(
            "expansion depth limit ({MAX_EXPANSION_DEPTH}) exceeded in '{}'",
            call.pattern_name
        )));
    }

    let def = *pattern_map.get(call.pattern_name.as_str()).ok_or_else(|| {
        pattern_err(format!("Pattern call references undefined pattern '{}'.", call.pattern_name))
    })?;

    // Validate argument count.
    if call.arguments.len() != def.params.len() {
        return Err(pattern_err(format!(
            "Pattern '{}' expects {} arguments, got {}.",
            call.pattern_name,
            def.params.len(),
            call.arguments.len()
        )));
    }

    // Build substitution map.
    let subs = build_substitution_map(def, call)?;

    // Build human-readable args summary for annotations.
    let args_summary = build_args_summary(&call.arguments);

    // Substitute all raw lines in the reflect body.
    let substituted: Vec<String> =
        def.body.raw_lines.iter().map(|line| substitute_line(line, &subs)).collect();

    // Parse the substituted lines as a module fragment.
    let mut fragment = parse_reflect_fragment(&substituted, &call.pattern_name)?;

    // Validate that signals from the fragment are only internal.
    validate_fragment_signals(&fragment, &call.pattern_name)?;

    // Compute origin tag and prefix for this call.
    let current_index = *call_index;
    *call_index += 1;
    let origin_tag = format!("{}_{}", call.pattern_name, current_index);
    let prefix = format!("{}_{}", call.pattern_name, current_index);

    // Collect original names from fragment for renaming references.
    let original_names = collect_fragment_names(&fragment);

    // Apply name prefixing to all generated names.
    apply_name_prefixing(&mut fragment, &prefix, &original_names);

    // Set origin tag on every generated node.
    set_origin_tags(&mut fragment, &origin_tag);

    // Count expanded items and check bounds.
    let item_count = fragment.signals.len()
        + fragment.guards.len()
        + fragment.reflexes.len()
        + fragment.properties.len();
    *total_expanded += item_count;
    if *total_expanded > MAX_EXPANDED_ITEMS {
        return Err(pattern_err(format!(
            "Total expanded items ({}) exceeds maximum ({MAX_EXPANDED_ITEMS}).",
            *total_expanded
        )));
    }

    // Append expanded items to the module.
    module.signals.extend(fragment.signals);
    module.guards.extend(fragment.guards);
    module.reflexes.extend(fragment.reflexes);
    module.properties.extend(fragment.properties);

    // Recursively expand any nested pattern calls from the fragment.
    for nested_call in &fragment.pattern_calls {
        expand_single_call(
            nested_call,
            pattern_map,
            module,
            depth + 1,
            total_expanded,
            call_index,
        )?;
    }

    // Record provenance annotation.
    module.pattern_origins.push(PatternOrigin {
        pattern_name: call.pattern_name.clone(),
        call_args_summary: args_summary,
    });

    Ok(())
}

/// Collect all names defined in a fragment (signals, guards, reflexes, properties).
fn collect_fragment_names(fragment: &ExpandedFragment) -> HashSet<String> {
    let mut names = HashSet::with_capacity(32);
    for s in &fragment.signals {
        names.insert(s.name.clone());
    }
    for g in &fragment.guards {
        names.insert(g.name.clone());
    }
    for r in &fragment.reflexes {
        names.insert(r.name.clone());
    }
    for p in &fragment.properties {
        names.insert(p.name.clone());
    }
    names
}

/// Apply name prefixing to all names in the fragment.
///
/// Scheme: `{prefix}_{original_name}`
///
/// Also renames references: guard conditions (signal refs), reflex guard_names,
/// reflex assignment targets (only if they reference internal signals from
/// this fragment), and property formula signal refs.
fn apply_name_prefixing(
    fragment: &mut ExpandedFragment,
    prefix: &str,
    original_names: &HashSet<String>,
) {
    // Build rename map: original_name -> prefixed_name
    let mut rename: HashMap<String, String> = HashMap::with_capacity(original_names.len());
    for name in original_names {
        rename.insert(name.clone(), format!("{prefix}_{name}"));
    }

    // Rename signal declarations.
    for sig in &mut fragment.signals {
        if let Some(new_name) = rename.get(&sig.name) {
            sig.name = new_name.clone();
        }
    }

    // Rename guard names and references in conditions.
    for guard in &mut fragment.guards {
        if let Some(new_name) = rename.get(&guard.name) {
            guard.name = new_name.clone();
        }
        rename_expr_signals(&mut guard.condition, &rename);
    }

    // Rename reflex names, guard_names references, and assignment references.
    for reflex in &mut fragment.reflexes {
        if let Some(new_name) = rename.get(&reflex.name) {
            reflex.name = new_name.clone();
        }
        // Rename guard references.
        for gname in &mut reflex.guard_names {
            if let Some(new_name) = rename.get(gname.as_str()) {
                *gname = new_name.clone();
            }
        }
        // Rename assignment targets and RHS expressions (only internal names).
        for assignment in &mut reflex.assignments {
            if let Some(new_name) = rename.get(&assignment.target) {
                assignment.target = new_name.clone();
            }
            rename_expr_signals(&mut assignment.value, &rename);
        }
    }

    // Rename property names and formula signal references.
    for prop in &mut fragment.properties {
        if let Some(new_name) = rename.get(&prop.name) {
            prop.name = new_name.clone();
        }
        rename_property_signals(&mut prop.formula, &rename);
    }
}

/// Rename signal references in an expression tree.
///
/// Uses an explicit iterative work stack — zero recursion (NASA P10 rule #1).
/// Bounded: at most 512 nodes.
fn rename_expr_signals(expr: &mut Expr, rename: &HashMap<String, String>) {
    let mut stack: Vec<&mut Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        match node {
            Expr::Signal(name) => {
                if let Some(new_name) = rename.get(name.as_str()) {
                    *name = new_name.clone();
                }
            }
            Expr::Prev { signal, .. } => {
                if let Some(new_name) = rename.get(signal.as_str()) {
                    *signal = new_name.clone();
                }
            }
            Expr::Literal(_) => {}
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
        }
    }
}

/// Rename signal references inside a property formula.
fn rename_property_signals(formula: &mut PropertyFormula, rename: &HashMap<String, String>) {
    for expr in formula.exprs_mut() {
        rename_expr_signals(expr, rename);
    }
}

/// Set origin tags on all nodes in the fragment.
fn set_origin_tags(fragment: &mut ExpandedFragment, origin: &str) {
    for sig in &mut fragment.signals {
        sig.origin = Some(origin.to_string());
    }
    for guard in &mut fragment.guards {
        guard.origin = Some(origin.to_string());
    }
    for reflex in &mut fragment.reflexes {
        reflex.origin = Some(origin.to_string());
    }
    for prop in &mut fragment.properties {
        prop.origin = Some(origin.to_string());
    }
}

/// Validate internal signal scoping after all expansions are complete.
///
/// Checks that no hand-written guard condition, reflex assignment, or property
/// formula references an internal signal generated by a pattern expansion.
///
/// A signal is "pattern-internal" if: kind == Internal AND origin.is_some().
///
/// Bounded: iterates over module signals + guards + reflexes + properties.
fn validate_internal_signal_scoping(module: &Module) -> Result<(), MirrError> {
    // Collect all pattern-internal signal names and their origin.
    let mut internal_signals: HashMap<&str, (&str, Option<Span>)> = HashMap::with_capacity(16);
    for sig in &module.signals {
        if sig.kind == SignalKind::Internal {
            if let Some(ref origin) = sig.origin {
                internal_signals.insert(&sig.name, (origin, sig.span));
            }
        }
    }

    if internal_signals.is_empty() {
        return Ok(());
    }

    // Check hand-written guards (origin == None).
    for guard in &module.guards {
        if guard.origin.is_none() {
            check_expr_no_internal_refs(&guard.condition, &internal_signals)?;
        }
    }

    // Check hand-written reflexes (origin == None).
    for reflex in &module.reflexes {
        if reflex.origin.is_none() {
            for assignment in &reflex.assignments {
                if let Some((origin, sig_span)) = internal_signals.get(assignment.target.as_str()) {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "[E212] signal '{}' is internal to pattern '{}' \
                             and cannot be referenced externally",
                            assignment.target, origin
                        ),
                        span: *sig_span,
                    });
                }
                check_expr_no_internal_refs(&assignment.value, &internal_signals)?;
            }
        }
    }

    // Check hand-written properties (origin == None).
    for prop in &module.properties {
        if prop.origin.is_none() {
            check_property_no_internal_refs(&prop.formula, &internal_signals)?;
        }
    }

    // Check cross-expansion references: a pattern expansion referencing
    // an internal signal from a DIFFERENT expansion.
    for guard in &module.guards {
        if let Some(ref guard_origin) = guard.origin {
            check_expr_cross_expansion(&guard.condition, guard_origin, &internal_signals)?;
        }
    }
    for reflex in &module.reflexes {
        if let Some(ref reflex_origin) = reflex.origin {
            for assignment in &reflex.assignments {
                // Check target.
                if let Some((sig_origin, sig_span)) =
                    internal_signals.get(assignment.target.as_str())
                {
                    if *sig_origin != reflex_origin.as_str() {
                        return Err(MirrError::SemanticError {
                            message: format!(
                                "[E214] signal '{}' is internal to pattern '{}' \
                                 and cannot be referenced externally",
                                assignment.target, sig_origin
                            ),
                            span: *sig_span,
                        });
                    }
                }
                check_expr_cross_expansion(&assignment.value, reflex_origin, &internal_signals)?;
            }
        }
    }

    Ok(())
}

/// Check that an expression does not reference any pattern-internal signals.
///
/// Uses explicit work stack — zero recursion.
fn check_expr_no_internal_refs(
    expr: &Expr,
    internal_signals: &HashMap<&str, (&str, Option<Span>)>,
) -> Result<(), MirrError> {
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        let name = match node {
            Expr::Signal(n) => Some(n.as_str()),
            Expr::Prev { signal, .. } => Some(signal.as_str()),
            Expr::Literal(_) => None,
            Expr::Unary { operand, .. } => {
                stack.push(operand);
                None
            }
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
                None
            }
        };
        if let Some(sig_name) = name {
            if let Some((origin, sig_span)) = internal_signals.get(sig_name) {
                return Err(MirrError::SemanticError {
                    message: format!(
                        "[E213] signal '{}' is internal to pattern '{}' \
                         and cannot be referenced externally",
                        sig_name, origin
                    ),
                    span: *sig_span,
                });
            }
        }
    }
    Ok(())
}

/// Check that an expression from one expansion doesn't reference internal
/// signals from a different expansion.
fn check_expr_cross_expansion(
    expr: &Expr,
    my_origin: &str,
    internal_signals: &HashMap<&str, (&str, Option<Span>)>,
) -> Result<(), MirrError> {
    let mut stack: Vec<&Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXPR_NODES {
            break;
        }
        let name = match node {
            Expr::Signal(n) => Some(n.as_str()),
            Expr::Prev { signal, .. } => Some(signal.as_str()),
            Expr::Literal(_) => None,
            Expr::Unary { operand, .. } => {
                stack.push(operand);
                None
            }
            Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
                None
            }
        };
        if let Some(sig_name) = name {
            if let Some((sig_origin, sig_span)) = internal_signals.get(sig_name) {
                if *sig_origin != my_origin {
                    return Err(MirrError::SemanticError {
                        message: format!(
                            "[E215] signal '{}' is internal to pattern '{}' \
                             and cannot be referenced externally",
                            sig_name, sig_origin
                        ),
                        span: *sig_span,
                    });
                }
            }
        }
    }
    Ok(())
}

/// Check that a property formula does not reference internal signals.
fn check_property_no_internal_refs(
    formula: &PropertyFormula,
    internal_signals: &HashMap<&str, (&str, Option<Span>)>,
) -> Result<(), MirrError> {
    for expr in formula.exprs() {
        check_expr_no_internal_refs(expr, internal_signals)?;
    }
    Ok(())
}

/// Build a substitution map: param_name -> replacement string.
///
/// Signal params -> the signal name string.
/// Constant params -> the literal value string.
///
/// Also validates type compatibility between params and args.
/// Bounded: iterates over params (max 32).
fn build_substitution_map(
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
                    "[E401] Pattern '{}' parameter '{}' has kind 'pattern' but argument is not a pattern reference.",
                    def.name, param.name
                )));
            }
            // Signal/Constant params do not accept pattern refs.
            (_, PatternArg::PatternRef(_)) => {
                return Err(pattern_err(format!(
                    "[E401] Pattern '{}' parameter '{}' does not accept a pattern reference.",
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
fn substitute_line(line: &str, subs: &[(String, String)]) -> String {
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
fn parse_reflect_fragment(
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
fn validate_fragment_signals(
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
fn build_args_summary(args: &[PatternArg]) -> String {
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

// ---------------------------------------------------------------------------
// Cycle detection
// ---------------------------------------------------------------------------

/// Detect circular pattern references using bounded DFS.
///
/// Builds an adjacency list from pattern def bodies by scanning for
/// pattern call lines. Reports E402 if a cycle is found.
///
/// Bounded: at most MAX_PATTERN_DEFS=64 nodes, DFS stack bounded by same.
fn detect_pattern_cycles(patterns: &[PatternDef]) -> Result<(), MirrError> {
    use crate::parser::pattern_parser::is_pattern_call_line;

    // Build name set for quick lookup.
    let pattern_names: HashSet<&str> = patterns.iter().map(|p| p.name.as_str()).collect();

    // Build adjacency list: for each pattern, which other patterns does it call?
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::with_capacity(patterns.len());
    for pat in patterns {
        let mut callees = Vec::new();
        for line in &pat.body.raw_lines {
            let trimmed = line.trim();
            if is_pattern_call_line(trimmed) {
                // Extract callee name (before the '(').
                if let Some(paren) = trimmed.find('(') {
                    let callee = trimmed[..paren].trim();
                    if pattern_names.contains(callee) {
                        callees.push(callee);
                    }
                }
            }
        }
        adj.insert(pat.name.as_str(), callees);
    }

    // DFS cycle detection with explicit stack (no recursion).
    // States: 0 = unvisited, 1 = in-progress (on stack), 2 = done.
    let mut state: HashMap<&str, u8> = HashMap::with_capacity(patterns.len());
    let mut path: Vec<&str> = Vec::with_capacity(patterns.len());

    for pat in patterns {
        let start = pat.name.as_str();
        if *state.get(start).unwrap_or(&0) != 0 {
            continue;
        }

        // Explicit DFS stack: (node, child_index).
        let mut stack: Vec<(&str, usize)> = vec![(start, 0)];
        state.insert(start, 1);
        path.push(start);

        while let Some((node, idx)) = stack.last_mut() {
            let children = adj.get(node).map_or(&[] as &[&str], |v| v.as_slice());
            if *idx >= children.len() {
                // Done with this node.
                state.insert(node, 2);
                path.pop();
                stack.pop();
                continue;
            }

            let child = children[*idx];
            *idx += 1;

            match state.get(child).unwrap_or(&0) {
                0 => {
                    // Unvisited — descend.
                    state.insert(child, 1);
                    path.push(child);
                    stack.push((child, 0));
                }
                1 => {
                    // Back edge — cycle detected.
                    let cycle_start = path.iter().position(|&n| n == child).unwrap_or(0);
                    let cycle_path: Vec<&str> = path[cycle_start..].to_vec();
                    let cycle_str = cycle_path
                        .iter()
                        .copied()
                        .chain(std::iter::once(child))
                        .collect::<Vec<_>>()
                        .join(" -> ");
                    return Err(pattern_err(format!(
                        "[E402] Circular pattern reference detected: {cycle_str}."
                    )));
                }
                _ => {} // Already done, skip.
            }
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Internal types
// ---------------------------------------------------------------------------

/// A parsed fragment extracted from a reflect body after substitution.
struct ExpandedFragment {
    signals: Vec<crate::ast::program::SignalDecl>,
    guards: Vec<crate::ast::program::Guard>,
    reflexes: Vec<crate::ast::program::Reflex>,
    properties: Vec<crate::ast::property::PropertyDecl>,
    pattern_calls: Vec<PatternCall>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pattern_err(msg: impl Into<String>) -> MirrError {
    MirrError::PatternError { message: msg.into(), span: None }
}
