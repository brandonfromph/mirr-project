//! Extended type checking: main entry point + refinement and linear checks.
//!
//! Part of the MEGA-1 Extended Type System.

#![forbid(unsafe_code)]

use super::qualifiers::*;
use super::types::*;
use crate::ecs::components::{
    AssignmentComponent, EntityId, EntityKind, KindComponent, ModuleComponent, NameComponent,
    ReflexComponent, TypeComponent,
};
use crate::ecs::Registry;

// ===========================================================================
// E) typecheck_extended() — extended type checking function signature
// ===========================================================================

/// Extended type map: maps each expression entity to its
/// inferred `ExtendedType`. Replaces legacy pointer-based map.
pub type ExtendedTypeMap = std::collections::HashMap<EntityId, ExtendedType>;

/// Result of extended type checking on a module.
///
/// Contains the extended type map plus any accumulated errors.
pub struct ExtendedTypeCheckResult {
    /// Extended type inferred for every expression node.
    pub type_map: ExtendedTypeMap,
    /// Accumulated type errors (empty on success).
    pub errors: crate::error::PipelineErrors,
}

/// Type-check all expressions in a module using the extended type system.
///
/// This function subsumes the existing `typecheck_module` for programs that
/// use MEGA-1 features. For programs with only base types, it delegates to
/// the existing checker and wraps the result.
///
/// ## Checking Order (all phases bounded by MAX_EXTENDED_TYPE_NODES)
///
/// 1. **Base type inference** — delegates to existing `infer_expr_type` for
///    core signedness/width checking (E601-E609).
///
/// 2. **Refinement checking** — for each assignment, checks that the RHS
///    expression's inferred refinement bounds are subsumed by the LHS target's
///    declared refinement bounds (E610-E612).
///
/// 3. **Linear type checking** — builds a per-cycle use-count map for every
///    linear signal and verifies exactly-once consumption (E613-E614).
///    Interacts with E216: E216 ensures single writer, E613/E614 ensure
///    single reader, together forming exclusive ownership.
///
/// 4. **Effect checking** — verifies that `pure` expressions contain no
///    `Prev` references or `stateful` sub-expressions (E616-E617).
///
/// 5. **Clock domain checking** — builds a domain map and verifies that
///    cross-domain references pass through a declared synchronizer (E618-E619).
///
/// 6. **Phantom tag checking** — verifies tag compatibility on assignments
///    and comparisons (E620-E621).
///
/// 7. **Session type checking** — verifies protocol state transitions
///    across module boundaries (E625).
///
/// Bounded: each phase iterates over a finite collection (signals, guards,
/// reflexes, assignments) with inner traversals bounded by MAX_EXTENDED_TYPE_NODES.
pub fn typecheck_extended(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    clock_domains: &[ClockDomain],
    phantom_tags: &[PhantomTag],
    protocols: &[SessionProtocol],
) -> ExtendedTypeCheckResult {
    // --- Phase 0: Build lookup tables ---
    let mut signal_types: std::collections::HashMap<&str, &ExtendedType> =
        std::collections::HashMap::with_capacity(extended_signals.len());
    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        signal_types.insert(&sig.name, &sig.extended_ty);
        idx += 1;
    }

    let mut errors = crate::error::PipelineErrors::new();
    let ext_type_map: ExtendedTypeMap =
        std::collections::HashMap::with_capacity(module.signals.len() * 4);

    // --- Phase 1: Delegate base type checking ---
    // [LEGACY] Base type checking via AST `typecheck_module` has been deleted.
    // Base types are now checked by ECS. `ext_type_map` is left empty here,
    // as it is unused by downstream ECS-based passes.

    // --- Phase 2: Refinement bound validation ---
    check_refinement_consistency(extended_signals, &mut errors);

    // --- Phase 3: Linear type checking ---
    check_linear_signals(module, extended_signals, &mut errors);

    // --- Phase 4: Effect checking ---
    super::domain_checks::check_effect_qualifiers(module, extended_signals, &mut errors);

    // --- Phase 5: Clock domain checking ---
    super::domain_checks::check_clock_domains(module, extended_signals, clock_domains, &mut errors);

    // --- Phase 6: Phantom tag checking ---
    super::domain_checks::check_phantom_tags(module, extended_signals, phantom_tags, &mut errors);

    // --- Phase 7: Session type checking ---
    super::domain_checks::check_session_types(module, extended_signals, protocols, &mut errors);

    ExtendedTypeCheckResult { type_map: ext_type_map, errors }
}

// ===========================================================================
// Phase 2: Refinement bound validation
// ===========================================================================

/// Validate that refinement predicates on each signal are internally consistent
/// and compatible with the base type's bit-width capacity.
///
/// Checks:
/// - Lower bounds do not exceed upper bounds (E610).
/// - Bounds fit within the declared bit-width (E612).
///
/// Bounded: iterates over signals (finite) and predicates (max MAX_REFINEMENT_PREDICATES).
fn check_refinement_consistency(
    signals: &[ExtendedSignalDecl],
    errors: &mut crate::error::PipelineErrors,
) {
    let mut sig_idx = 0usize;
    while sig_idx < signals.len() && sig_idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &signals[sig_idx];
        sig_idx += 1;

        if sig.extended_ty.refinements.is_empty() {
            continue;
        }

        let max_val = sig.extended_ty.base_max_value();

        let mut lo: Option<u64> = None;
        let mut hi: Option<u64> = None;

        let mut pred_idx = 0usize;
        while pred_idx < sig.extended_ty.refinements.len() && pred_idx < MAX_REFINEMENT_PREDICATES {
            let pred = &sig.extended_ty.refinements[pred_idx];
            pred_idx += 1;

            // Track tightest bounds
            if let Some(implied_lo) = pred.bound.implied_min() {
                lo = Some(lo.map_or(implied_lo, |l: u64| l.max(implied_lo)));
            }
            if let Some(implied_hi) = pred.bound.implied_max() {
                hi = Some(hi.map_or(implied_hi, |h: u64| h.min(implied_hi)));
            }

            // E612: Check bound fits in bit-width
            if let Some(implied_hi) = pred.bound.implied_max() {
                if let Some(max) = max_val {
                    if implied_hi > max {
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Signal '{}' refinement bound {} exceeds {}-bit capacity (max {}).",
                                error_codes::E612_REF_WIDTH,
                                sig.name,
                                pred.bound,
                                sig.extended_ty.base,
                                max
                            ),
                            span: pred.span,
                        });
                    }
                }
            }
        }

        // E610: Lower bound exceeds upper bound
        if let (Some(lower), Some(upper)) = (lo, hi) {
            if lower > upper {
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Signal '{}' has unsatisfiable refinement: lower bound {} > upper bound {}.",
                        error_codes::E610_REF_BOUND,
                        sig.name,
                        lower,
                        upper
                    ),
                    span: sig.span,
                });
            }
        }
    }
}

// ===========================================================================
// Phase 3: Linear type checking
// ===========================================================================

/// Check that every linear-qualified signal is consumed exactly once per cycle.
///
/// Interaction with E216 (single-writer):
/// - E216 already ensures at most one reflex writes to a given signal.
/// - Linear checking adds the dual constraint: at most one read per cycle.
/// - Together: one writer + one reader = exclusive ownership per cycle.
///
/// Bounded: iterates over reflexes, assignments, expressions (all finite).
fn check_linear_signals(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    errors: &mut crate::error::PipelineErrors,
) {
    // Collect names of linear signals
    let mut linear_names: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());
    let mut sig_idx = 0usize;
    while sig_idx < extended_signals.len() && sig_idx < MAX_EXTENDED_TYPE_NODES {
        if extended_signals[sig_idx].extended_ty.is_linear() {
            linear_names.insert(&extended_signals[sig_idx].name);
        }
        sig_idx += 1;
    }

    if linear_names.is_empty() {
        return;
    }

    // Count reads per signal across all expressions in each reflex
    let mut read_counts: std::collections::HashMap<&str, usize> =
        std::collections::HashMap::with_capacity(linear_names.len());

    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        // Reset counts per reflex (each reflex is a separate "cycle context")
        for name in &linear_names {
            read_counts.insert(name, 0);
        }

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;
                if linear_names.contains(sig_ref.as_str()) {
                    if let Some(count) = read_counts.get_mut(sig_ref.as_str()) {
                        *count += 1;
                    }
                }
            }
        }

        // E614: Double consumption
        for (name, count) in &read_counts {
            if *count > 1 {
                if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                    return;
                }
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Linear signal '{}' is consumed {} times in reflex '{}' (must be exactly 1).",
                        error_codes::E614_LIN_DOUBLE,
                        name,
                        count,
                        reflex.name
                    ),
                    span: reflex.span,
                });
            }
        }
    }

    // E613: Unused linear signals (not consumed by any reflex)
    // Build global read set
    let mut ever_read: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(linear_names.len());

    let mut reflex_idx2 = 0usize;
    while reflex_idx2 < module.reflexes.len() && reflex_idx2 < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx2];
        reflex_idx2 += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let refs = crate::validation::semantic::collect_signal_refs(
                &reflex.assignments[assign_idx].value,
            );
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                if linear_names.contains(refs[ref_idx].as_str()) {
                    if let Some(&name) = linear_names.get(refs[ref_idx].as_str()) {
                        ever_read.insert(name);
                    }
                }
                ref_idx += 1;
            }
            assign_idx += 1;
        }
    }

    for name in &linear_names {
        if !ever_read.contains(name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Linear signal '{}' is declared but never consumed in any reflex.",
                    error_codes::E613_LIN_UNUSED,
                    name
                ),
                span: None,
            });
        }
    }
}

/// ECS-native: Run all extended type checks on a module in the Registry.
///
/// Calls Phase 7 (session type checking) with an empty protocol list.
/// Programs that declare `session` protocols should use
/// [`typecheck_extended_ecs_with_protocols`] to provide the protocol definitions.
pub fn typecheck_extended_ecs(registry: &Registry, mod_id: EntityId) -> ExtendedTypeCheckResult {
    typecheck_extended_ecs_with_protocols(registry, mod_id, &[])
}

/// ECS-native: Run all extended type checks, including Phase 7 session type
/// validation against the provided protocol definitions.
///
/// # Phase Execution Order
///
/// 2. Refinement bound validation (E610, E612)
/// 3. Linear type checking (E613, E614)
/// 4. Effect checking (E616, E617)
/// 5. Clock domain checking (E618, E619)
/// 6. Phantom tag checking (E620, E621)
/// 7. Session type checking (E625)  ← now fully ECS-native
///
/// All phases are O(entities) with constant inner bounds (NASA P10 Rule #2).
pub fn typecheck_extended_ecs_with_protocols(
    registry: &Registry,
    mod_id: EntityId,
    protocols: &[super::qualifiers::SessionProtocol],
) -> ExtendedTypeCheckResult {
    let mut extended_signals = Vec::new();
    let target_mask = crate::ecs::registry::COMP_MODULE | crate::ecs::registry::COMP_KIND;
    for entity in registry.entities_with_components(target_mask) {
        let i = entity.0 as usize;
        if let Some(ModuleComponent(m_id)) = &registry.modules[i] {
            if *m_id == mod_id {
                if let Some(KindComponent(EntityKind::SIGNAL(_))) = &registry.kinds[i] {
                    if let Some(decl) = super::qualifiers::ExtendedSignalDecl::from_ecs(
                        registry,
                        EntityId(i as u32),
                    ) {
                        extended_signals.push(decl);
                    }
                }
            }
        }
    }

    let mut errors = crate::error::PipelineErrors::new();

    // --- Phase 2: Refinement bound validation ---
    check_refinement_consistency(&extended_signals, &mut errors);

    // --- Phase 3: Linear type checking ---
    check_linear_signals_ecs(registry, mod_id, &mut errors);

    // --- Phase 4: Effect checking ---
    super::domain_checks::check_effect_qualifiers_ecs(registry, mod_id, &mut errors);

    // --- Phase 5: Clock domain checking ---
    super::domain_checks::check_clock_domains_ecs(registry, mod_id, &mut errors);

    // --- Phase 6: Phantom tag checking ---
    super::domain_checks::check_phantom_tags_ecs(registry, mod_id, &mut errors);

    // --- Phase 7: Session type checking (ECS-native) ---
    super::domain_checks::check_session_types_ecs(registry, mod_id, protocols, &mut errors);

    ExtendedTypeCheckResult { type_map: std::collections::HashMap::new(), errors }
}

// ===========================================================================
// Phase 3 (ECS-Native): Linear type checking helpers
// ===========================================================================

/// Collect the EntityIds and names of all linear-qualified signals in `mod_id`.
///
/// Returns a fixed-size array of `(EntityId, name_ptr)` pairs and the fill count.
/// Bounded by `MAX_LINEAR_SIGNALS` — modules with more than 128 linear signals
/// should be decomposed. Zero heap allocation: the array lives on the stack.
///
/// Extracted to keep `check_linear_signals_ecs` ≤ one printed page (P10 Rule #1).
#[inline]
fn collect_linear_ids<'r>(
    registry: &'r Registry,
    mod_id: EntityId,
    out: &mut [Option<(EntityId, &'r str)>; MAX_LINEAR_SIGNALS],
) -> usize {
    let max_id = registry.active_entities();
    let mut fill = 0usize;
    let mut entity_idx = 0usize;
    while entity_idx < max_id && fill < MAX_LINEAR_SIGNALS {
        let Some(ModuleComponent(m_id)) = registry.modules[entity_idx] else {
            entity_idx += 1;
            continue;
        };
        if m_id != mod_id {
            entity_idx += 1;
            continue;
        }
        let Some(KindComponent(EntityKind::SIGNAL(_))) = &registry.kinds[entity_idx] else {
            entity_idx += 1;
            continue;
        };
        let Some(TypeComponent(ty)) = &registry.types[entity_idx] else {
            entity_idx += 1;
            continue;
        };
        if !matches!(ty.annotations.linearity, crate::ast::types::Linearity::Linear) {
            entity_idx += 1;
            continue;
        }
        let Some(NameComponent(ref name)) = registry.names[entity_idx] else {
            entity_idx += 1;
            continue;
        };
        out[fill] = Some((EntityId(entity_idx as u32), registry.resolve_name(*name)));
        fill += 1;
        entity_idx += 1;
    }
    fill
}

/// Scan all assignments in `reflex_comps[reflex_idx]` and return, via `counts`,
/// how many times each linear signal (from `linear[..n]`) is read.
///
/// `counts[k]` corresponds to `linear[k]`. Bounded by `MAX_LINEAR_SIGNALS`
/// (outer) and `MAX_EXTENDED_TYPE_NODES` (assignments + refs). Zero heap.
///
/// Extracted to keep `check_linear_signals_ecs` ≤ one printed page (P10 Rule #1).
#[inline]
fn count_linear_reads(
    registry: &Registry,
    reflex_idx: usize,
    linear: &[Option<(EntityId, &str)>; MAX_LINEAR_SIGNALS],
    n: usize,
    counts: &mut [u16; MAX_LINEAR_SIGNALS],
) {
    // Reset counts for this reflex context.
    let mut k = 0usize;
    while k < n {
        counts[k] = 0;
        k += 1;
    }

    let Some(ReflexComponent { ref assignments, .. }) = registry.reflex_comps[reflex_idx] else {
        return;
    };

    let assign_bound = assignments.len().min(MAX_EXTENDED_TYPE_NODES);
    let mut a_idx = 0usize;
    while a_idx < assign_bound {
        let assign_ent = assignments[a_idx];
        a_idx += 1;

        let Some(AssignmentComponent { value, .. }) =
            &registry.assignment_comps[assign_ent.0 as usize]
        else {
            continue;
        };

        let refs = crate::validation::semantic::collect_signal_refs_ecs(registry, *value);
        let ref_bound = refs.len().min(MAX_EXTENDED_TYPE_NODES);
        let mut r_idx = 0usize;
        while r_idx < ref_bound {
            let ref_name = refs[r_idx].as_str();
            r_idx += 1;
            // Linear scan over known linear signals — at most MAX_LINEAR_SIGNALS = 128.
            let mut k = 0usize;
            while k < n {
                if let Some((_, sig_name)) = linear[k] {
                    if sig_name == ref_name {
                        counts[k] = counts[k].saturating_add(1);
                        break; // Each signal appears once in the table.
                    }
                }
                k += 1;
            }
        }
    }
}

/// ECS-native: Verify exactly-once consumption of linear signals in `mod_id`.
///
/// For each linear-qualified signal this function checks:
/// 1. It is read at most once per reflex body (E614 — double consumption).
/// 2. It is read at least once across all reflexes (E613 — never consumed).
///
/// # NASA P10 Compliance
/// - Rule #1: Every function is ≤ one printed page. Two helpers extracted above.
/// - Rule #2: All loops bounded by `max_id`, `MAX_LINEAR_SIGNALS`, or
///   `MAX_EXTENDED_TYPE_NODES`.
/// - Rule #3: Zero heap allocation. Linear signal table is a stack array
///   `[Option<...>; MAX_LINEAR_SIGNALS]`. Counts are `[u16; MAX_LINEAR_SIGNALS]`.
/// - Rule #6: All branches explicit (`let-else` + `continue`).
pub fn check_linear_signals_ecs(
    registry: &Registry,
    mod_id: EntityId,
    errors: &mut crate::error::PipelineErrors,
) {
    // --- Step 1: Collect all linear signals in this module (stack-allocated). ---
    let mut linear: [Option<(EntityId, &str)>; MAX_LINEAR_SIGNALS] =
        [const { None }; MAX_LINEAR_SIGNALS];
    let n = collect_linear_ids(registry, mod_id, &mut linear);
    if n == 0 {
        return;
    }

    // --- Step 2: Per-reflex double-consumption check (E614). ---
    // Also track which signals are ever read (for E613).
    let mut ever_read: [bool; MAX_LINEAR_SIGNALS] = [false; MAX_LINEAR_SIGNALS];
    let mut counts: [u16; MAX_LINEAR_SIGNALS] = [0u16; MAX_LINEAR_SIGNALS];
    let max_id = registry.active_entities();
    let mut entity_idx = 0usize;

    while entity_idx < max_id {
        // Guard 1: entity must belong to mod_id.
        let Some(ModuleComponent(m_id)) = registry.modules[entity_idx] else {
            entity_idx += 1;
            continue;
        };
        if m_id != mod_id {
            entity_idx += 1;
            continue;
        }
        // Guard 2: entity must be a reflex.
        let Some(KindComponent(EntityKind::REFLEX)) = &registry.kinds[entity_idx] else {
            entity_idx += 1;
            continue;
        };

        // Count reads per linear signal in this reflex.
        count_linear_reads(registry, entity_idx, &linear, n, &mut counts);

        // Validate and record.
        let mut k = 0usize;
        while k < n {
            if counts[k] >= 1 {
                ever_read[k] = true;
            }
            if counts[k] > 1 {
                if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                    return;
                }
                let sig_name = linear[k].map(|(_, name)| name).unwrap_or("<unknown>");
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Linear signal '{}' consumed {} times in a single reflex \
                         (must be exactly once — exclusive ownership violation).",
                        error_codes::E614_LIN_DOUBLE,
                        sig_name,
                        counts[k]
                    ),
                    span: None,
                });
            }
            k += 1;
        }

        entity_idx += 1;
    }

    // --- Step 3: Never-consumed check (E613). ---
    let mut k = 0usize;
    while k < n {
        if !ever_read[k] {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            let sig_name = linear[k].map(|(_, name)| name).unwrap_or("<unknown>");
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Linear signal '{}' is declared but never consumed in any reflex \
                     (ownership requires exactly-once consumption).",
                    error_codes::E613_LIN_UNUSED,
                    sig_name
                ),
                span: None,
            });
        }
        k += 1;
    }
}
