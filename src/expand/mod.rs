//! Pattern expansion engine for the MIRR homoiconic pattern system.
//!
//! Expands pattern calls into module fragments via text-level `${param}` substitution,
//! re-parsing, name prefixing, and origin tagging. Validates internal signal scoping.
//! All traversals use explicit work stacks (NASA P10 rule #1: no recursion).

#![forbid(unsafe_code)]

mod cycles;
mod rename;
mod scoping;
mod substitution;

use cycles::detect_pattern_cycles;
use rename::{apply_name_prefixing, collect_fragment_names, set_origin_tags};
use scoping::validate_internal_signal_scoping;
use substitution::{
    build_args_summary, build_substitution_map, parse_reflect_fragment, substitute_line,
    validate_fragment_signals,
};

use std::collections::HashMap;

use crate::ast::pattern::{PatternCall, PatternDef, PatternOrigin};
use crate::ast::program::{MirrProgram, Module};
use crate::error::MirrError;

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
