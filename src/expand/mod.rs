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

use crate::ast::pattern::{PatternCall, PatternOrigin};
use crate::ast::program::MirrProgram;
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
///
/// Bounded: `MAX_EXPANSION_DEPTH` levels, `MAX_EXPANDED_ITEMS` total items.
/// Iterative: Uses an explicit stack to satisfy NASA P10 Rule #1.
pub fn expand_patterns(
    program: &mut MirrProgram,
    registry: &crate::ecs::Registry,
) -> Result<(), MirrError> {
    if program.module.pattern_calls.is_empty() {
        return Ok(());
    }

    // Static cycle detection before any expansion (bounded DFS).
    detect_pattern_cycles(&program.patterns)?;

    // Initial state for iterative expansion.
    let mut total_expanded = 0usize;
    let mut call_index = 0usize;
    let mut telemetry_log: Vec<String> = Vec::new();

    // The work stack stores (call, depth).
    let mut stack: Vec<(PatternCall, usize)> =
        std::mem::take(&mut program.module.pattern_calls).into_iter().map(|c| (c, 0)).collect();

    // Bounded iteration: each loop processes one expansion.
    // Total expansions are bounded by MAX_EXPANDED_ITEMS indirect limit.
    while let Some((call, depth)) = stack.pop() {
        if depth >= MAX_EXPANSION_DEPTH {
            return Err(pattern_err(format!(
                "expansion depth limit ({MAX_EXPANSION_DEPTH}) exceeded in '{}'",
                call.pattern_name
            )));
        }

        let entity_id = registry.get_entity_by_name(&call.pattern_name).ok_or_else(|| {
            MirrError::SemanticError {
                message: format!(
                    "{} Pattern call references undefined pattern '{}'.",
                    crate::error_codes::ec(200),
                    call.pattern_name
                ),
                span: call.span,
            }
        })?;

        let def_comp = registry.pattern_defs[entity_id.0 as usize].as_ref().ok_or_else(|| {
            MirrError::SemanticError {
                message: format!(
                    "{} Entity '{}' is not a pattern definition.",
                    crate::error_codes::ec(200),
                    call.pattern_name
                ),
                span: call.span,
            }
        })?;

        let def = &def_comp.0;

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
        let subs = build_substitution_map(def, &call)?;
        let args_summary = build_args_summary(&call.arguments);

        // Substitute and parse fragment.
        let substituted: Vec<String> =
            def.body.raw_lines.iter().map(|line| substitute_line(line, &subs)).collect();
        let mut fragment = parse_reflect_fragment(&substituted, &call.pattern_name)?;

        // Validate signals.
        validate_fragment_signals(&fragment, &call.pattern_name)?;

        // Renaming and origin tagging.
        let origin_tag = format!("{}_{}", call.pattern_name, call_index);
        let prefix = format!("{}_{}", call.pattern_name, call_index);
        call_index += 1;

        let names = collect_fragment_names(&fragment);
        let mut param_names = std::collections::HashSet::new();
        for p in &def.params {
            param_names.insert(p.name.clone());
        }

        apply_name_prefixing(&mut fragment, &prefix, &names, &param_names);
        set_origin_tags(&mut fragment, &origin_tag);

        // Check total item bounds.
        let item_count = fragment.signals.len()
            + fragment.guards.len()
            + fragment.reflexes.len()
            + fragment.properties.len();
        total_expanded += item_count;
        if total_expanded > MAX_EXPANDED_ITEMS {
            return Err(pattern_err(format!(
                "Total expanded items ({}) exceeds maximum ({MAX_EXPANDED_ITEMS}).",
                total_expanded
            )));
        }

        // Push results to module.
        program.module.signals.extend(fragment.signals);
        program.module.guards.extend(fragment.guards);
        program.module.reflexes.extend(fragment.reflexes);
        program.module.properties.extend(fragment.properties);
        program.module.pattern_origins.push(PatternOrigin {
            pattern_name: call.pattern_name.clone(),
            call_args_summary: args_summary.clone(),
        });

        // Queue nested pattern calls for further expansion (preserving depth).
        for nested_call in fragment.pattern_calls {
            stack.push((nested_call, depth + 1));
        }

        // Record telemetry (to be batched).
        telemetry_log
            .push(format!("Expanded pattern '{}' with args [{}]", call.pattern_name, args_summary));
    }

    // Batch Telemetry Stash: Save all successful pattern expansions in one shot (Phase 3 Integration).
    if !telemetry_log.is_empty() {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                let brain_bin = parent.join("mirr-brain");
                let batch_value = telemetry_log.join("; ");
                let _ = std::process::Command::new(brain_bin)
                    .args([
                        "store",
                        "--key",
                        "last_pattern_expansion_wave",
                        "--value",
                        &batch_value,
                    ])
                    .output();
            }
        }
    }

    // Post-expansion: validate internal signal scoping.
    validate_internal_signal_scoping(&program.module)?;

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
