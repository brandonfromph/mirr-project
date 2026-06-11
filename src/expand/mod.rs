//! Pattern expansion engine for the MIRR homoiconic pattern system.
//!
//! Expands pattern calls into module fragments via text-level `${param}` substitution,
//! re-parsing, name prefixing, and origin tagging. Validates internal signal scoping.
//! All traversals use explicit work stacks (NASA P10 rule #1: no recursion).

#![forbid(unsafe_code)]

pub mod ast_expand;
mod cycles;
mod rename;
mod scoping;
mod substitution;

use cycles::detect_pattern_cycles;
use rename::{
    apply_name_prefixing, apply_parameter_substitution, collect_fragment_names, set_origin_tags,
};
use scoping::validate_internal_signal_scoping;
use substitution::{build_args_summary, build_substitution_map, validate_fragment_signals};

use crate::ast::macro_nodes::ModuleMacroStmt;
use crate::ast::pattern::{PatternCall, PatternOrigin};
use crate::ast::program::MirrProgram;
use crate::error::MirrError;

/// Maximum nesting depth for pattern expansion (NASA P10 rule #1).
const MAX_EXPANSION_DEPTH: usize = 4;

/// Maximum total items (signals + guards + reflexes + properties) from all expansions.
const MAX_EXPANDED_ITEMS: usize = 262144;

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

    // The work stack stores (call, depth, parent_origin).
    let mut stack: Vec<(PatternCall, usize, Option<String>)> =
        std::mem::take(&mut program.module.pattern_calls)
            .into_iter()
            .map(|c| (c, 0, None))
            .collect();

    let mut parent_map: std::collections::HashMap<String, Option<String>> =
        std::collections::HashMap::new();

    // Bounded iteration: each loop processes one expansion.
    // Total expansions are bounded by MAX_EXPANDED_ITEMS indirect limit.
    while let Some((call, depth, parent_origin)) = stack.pop() {
        if depth >= MAX_EXPANSION_DEPTH {
            return Err(pattern_err(format!(
                "expansion depth limit ({MAX_EXPANSION_DEPTH}) exceeded in '{}'",
                call.pattern_name
            )));
        }

        let entity_id = registry.get_entity_by_name(&call.pattern_name).ok_or_else(|| {
            let is_namespaced = call.pattern_name.contains("::");
            let hint = if is_namespaced {
                format!(
                    "\n  Hint: Pattern '{}' is namespaced. This usually indicates an import, alias, or workspace linker resolution failure at the compilation layer.",
                    call.pattern_name
                )
            } else {
                "".to_string()
            };
            MirrError::SemanticError {
                message: format!(
                    "{} Pattern call references undefined pattern '{}'.{}",
                    crate::error_codes::ec(200),
                    call.pattern_name,
                    hint
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

        // Deep-clone the pre-parsed AST fragment.
        let mut fragment = def.body.clone();

        // Validate signals.
        validate_fragment_signals(&fragment, &call.pattern_name)?;

        // Renaming and origin tagging.
        let sanitized_pattern_name = call.pattern_name.replace("::", "_");
        let origin_tag = format!("{}_{}", sanitized_pattern_name, call_index);
        let prefix = format!("{}_{}", sanitized_pattern_name, call_index);
        call_index += 1;

        parent_map.insert(origin_tag.clone(), parent_origin);

        let mut param_names = std::collections::HashSet::new();
        for p in &def.params {
            param_names.insert(p.name.clone());
        }

        // Check if this is an "Interface Pattern" (contains no logic).
        // Interface patterns are preserved for structural module instantiation.
        let is_interface =
            fragment.statements.iter().all(|s| matches!(s, ModuleMacroStmt::Signal(_)));
        if is_interface {
            let mut preserved_call = call.clone();
            // Apply parameter substitution to the call arguments themselves.
            for arg in &mut preserved_call.arguments {
                match arg {
                    crate::ast::pattern::PatternArg::SignalRef(name) => {
                        if let Some((_, sub)) = subs.iter().find(|(p, _)| p == name) {
                            *name = sub.clone();
                        }
                    }
                    crate::ast::pattern::PatternArg::PatternRef(name) => {
                        if let Some((_, sub)) = subs.iter().find(|(p, _)| p == name) {
                            *name = sub.clone();
                        }
                    }
                    _ => {}
                }
            }
            program.module.pattern_calls.push(preserved_call);
            program.module.pattern_origins.push(PatternOrigin {
                pattern_name: call.pattern_name.clone(),
                call_args_summary: args_summary.clone(),
            });
            continue;
        }

        apply_parameter_substitution(&mut fragment, &subs);

        // Add pattern parameters to the expanded fragment's scope.
        // This ensures the validator can resolve signals passed as arguments.
        let mut temp_module = crate::ast::program::Module {
            name: format!("{}_temp", call.pattern_name),
            signals: Vec::new(),
            guards: Vec::new(),
            reflexes: Vec::new(),
            properties: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            span: None,
        };

        for param in &def.params {
            if let crate::ast::pattern::PatternParamKind::Signal { kind, ty, annotations } =
                &param.kind
            {
                temp_module.signals.push(crate::ast::program::SignalDecl {
                    name: param.name.clone(),
                    kind: *kind,
                    ty: crate::ast::types::ExtendedType::new(ty.clone(), annotations.clone()),
                    origin: Some(origin_tag.clone()),
                    span: None,
                });
            }
        }

        let names = collect_fragment_names(&fragment);
        apply_name_prefixing(&mut fragment, &prefix, &names, &param_names);
        set_origin_tags(&mut fragment, &origin_tag);

        crate::expand::ast_expand::expand_statements_inplace(
            &mut temp_module,
            fragment.statements,
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
            Some(origin_tag.clone()),
        )?;

        // Check total item bounds.
        let item_count = temp_module.signals.len()
            + temp_module.guards.len()
            + temp_module.reflexes.len()
            + temp_module.properties.len();
        total_expanded += item_count;
        if total_expanded > MAX_EXPANDED_ITEMS {
            return Err(pattern_err(format!(
                "Total expanded items ({}) exceeds maximum ({MAX_EXPANDED_ITEMS}).",
                total_expanded
            )));
        }

        // Push results to module.
        // Filter out formal parameters that were only added for internal validation.
        let mut expanded_signals = temp_module.signals;
        expanded_signals.retain(|s| !param_names.contains(&s.name));
        program.module.signals.extend(expanded_signals);

        program.module.guards.extend(temp_module.guards);
        // Prepended so that submodule reflexes are executed before parent reflexes.
        // This ensures parent/coordinator reflexes take precedence over submodules.
        let mut new_reflexes = temp_module.reflexes;
        new_reflexes.extend(std::mem::take(&mut program.module.reflexes));
        program.module.reflexes = new_reflexes;
        program.module.properties.extend(temp_module.properties);
        program.module.pattern_origins.push(PatternOrigin {
            pattern_name: call.pattern_name.clone(),
            call_args_summary: args_summary.clone(),
        });

        // Queue nested pattern calls for further expansion (preserving depth).
        for nested_call in temp_module.pattern_calls {
            stack.push((nested_call, depth + 1, Some(origin_tag.clone())));
        }
    }

    // Post-expansion: validate internal signal scoping.
    validate_internal_signal_scoping(&program.module, &parent_map)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pattern_err(msg: impl Into<String>) -> MirrError {
    MirrError::PatternError { message: msg.into(), span: None }
}
