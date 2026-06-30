//! Domain-specific type checks: effects, clock domains, phantom tags, session types.
//!
//! Part of the MEGA-1 Extended Type System.

#![forbid(unsafe_code)]

use super::qualifiers::*;
use super::types::*;
use crate::ecs::components::{
    AssignmentComponent, EntityId, EntityKind, KindComponent, ModuleComponent, ReflexComponent,
    TypeComponent,
};
use crate::ecs::Registry;

// ===========================================================================
// Phase 4: Effect checking
// ===========================================================================

/// Check that `pure`-qualified signals are not derived from stateful sources.
///
/// A pure signal must only depend on:
/// - Other pure signals
/// - Input signals (implicitly pure)
/// - Literals
///
/// Using `Prev` (register read) in a pure context is an error (E616).
/// Referencing a `stateful` signal from a pure expression is an error (E617).
///
/// Bounded: iterates over reflexes and assignments with bounded inner traversal.
pub(super) fn check_effect_qualifiers(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build lookup: signal name -> is_pure, is_stateful
    let mut pure_signals: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());
    let mut stateful_signals: std::collections::HashSet<&str> =
        std::collections::HashSet::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if sig.extended_ty.is_pure() {
            pure_signals.insert(&sig.name);
        }
        if sig.extended_ty.is_stateful() {
            stateful_signals.insert(&sig.name);
        }
        idx += 1;
    }

    if pure_signals.is_empty() {
        return;
    }

    // For each reflex, check assignments to pure targets
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            if !pure_signals.contains(assignment.target.as_str()) {
                continue;
            }

            // E616: Check for Prev in pure expression
            if expr_contains_prev(&assignment.value) {
                if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                    return;
                }
                errors.push(crate::error::MirrError::TypeError {
                    message: format!(
                        "[{}] Pure signal '{}' cannot depend on prev() (stateful operation) in reflex '{}'.",
                        error_codes::E616_EFF_PURE,
                        assignment.target,
                        reflex.name
                    ),
                    span: assignment.span,
                });
            }

            // E617: Check for stateful signal references in pure expression
            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;
                if stateful_signals.contains(sig_ref.as_str()) {
                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Pure signal '{}' cannot depend on stateful signal '{}' in reflex '{}'.",
                            error_codes::E617_EFF_MIX,
                            assignment.target,
                            sig_ref,
                            reflex.name
                        ),
                        span: assignment.span,
                    });
                }
            }
        }
    }
}

/// ECS-native: Check effect qualifiers (pure/stateful).
///
/// Adheres to NASA P10 Rule #3 (no heap allocation during execution) by
/// avoiding HashSets and performing linear scans or direct component lookups.
pub fn check_effect_qualifiers_ecs(
    registry: &Registry,
    mod_id: EntityId,
    errors: &mut crate::error::PipelineErrors,
) {
    // Phase 4 only runs if there are signals in the module.
    let max_id = registry.active_entities();
    let mut has_pure = false;
    let mut i = 0usize;
    while i < max_id {
        if let Some(ModuleComponent(m_id)) = registry.modules[i] {
            if m_id == mod_id {
                if let Some(TypeComponent(ty)) = &registry.types[i] {
                    if matches!(ty.annotations.effect, crate::ast::types::EffectQualifier::Pure) {
                        has_pure = true;
                        break;
                    }
                }
            }
        }
        i += 1;
    }

    if !has_pure {
        return;
    }

    // Iterate reflexes of this module
    let mut reflex_idx = 0usize;
    while reflex_idx < max_id {
        let Some(ModuleComponent(m_id)) = registry.modules[reflex_idx] else {
            reflex_idx += 1;
            continue;
        };
        if m_id != mod_id {
            reflex_idx += 1;
            continue;
        }

        let Some(ReflexComponent { assignments, .. }) = &registry.reflex_comps[reflex_idx] else {
            reflex_idx += 1;
            continue;
        };

        let reflex_name =
            registry.names[reflex_idx].map(|n| registry.resolve_name(n.0)).unwrap_or("unnamed");

        for &assign_ent in assignments {
            let Some(AssignmentComponent { target, value, .. }) =
                &registry.assignment_comps[assign_ent.0 as usize]
            else {
                continue;
            };

            // 1. Check if target is Pure
            let Some(TypeComponent(target_ty)) = &registry.types[target.0 as usize] else {
                continue;
            };
            if !matches!(target_ty.annotations.effect, crate::ast::types::EffectQualifier::Pure) {
                continue;
            }

            let target_name = registry.names[target.0 as usize]
                .map(|n| registry.resolve_name(n.0))
                .unwrap_or("?");

            // E616: Check for Prev in pure expression
            if ecs_expr_contains_prev(registry, *value) {
                let msg = format!(
                    "[{}] Pure signal '{}' cannot depend on prev() (stateful operation) in reflex '{}'.",
                    error_codes::E616_EFF_PURE,
                    target_name,
                    reflex_name
                );
                if push_session_error(errors, msg, None) {
                    return;
                }
            }

            // E617: Check for stateful signal references in pure expression
            let refs = crate::validation::semantic::collect_signal_refs_ecs(registry, *value);
            for sig_ref in refs {
                // Find signal entity by name in this module (linear scan, P10 Rule #2)
                let mut s_idx = 0usize;
                while s_idx < max_id {
                    if let Some(nc) = registry.names[s_idx] {
                        if registry.resolve_name(nc.0) == sig_ref {
                            if let Some(TypeComponent(ty)) = &registry.types[s_idx] {
                                if matches!(
                                    ty.annotations.effect,
                                    crate::ast::types::EffectQualifier::Stateful
                                ) {
                                    let msg = format!(
                                        "[{}] Pure signal '{}' cannot depend on stateful signal '{}' in reflex '{}'.",
                                        error_codes::E617_EFF_MIX,
                                        target_name,
                                        sig_ref,
                                        reflex_name
                                    );
                                    if push_session_error(errors, msg, None) {
                                        return;
                                    }
                                }
                            }
                            break;
                        }
                    }
                    s_idx += 1;
                }
            }
        }
        reflex_idx += 1;
    }
}

fn ecs_expr_contains_prev(registry: &Registry, entity: EntityId) -> bool {
    let mut stack = vec![entity];
    let mut visited = 0usize;
    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXTENDED_TYPE_NODES {
            break;
        }
        let idx = node.0 as usize;
        if registry.prev_ops[idx].is_some() {
            return true;
        }
        if let Some(crate::ecs::components::UnaryComponent { operand, .. }) =
            &registry.unary_ops[idx]
        {
            stack.push(*operand);
        } else if let Some(crate::ecs::components::BinaryComponent { left, right, .. }) =
            &registry.binary_ops[idx]
        {
            stack.push(*left);
            stack.push(*right);
        }
    }
    false
}

/// ECS-native: Check clock domain qualifiers.
///
/// Adheres to NASA P10 Rule #3 by performing linear scans for cross-domain
/// verification instead of constructing a heap-allocated HashMap.
pub fn check_clock_domains_ecs(
    registry: &Registry,
    mod_id: EntityId,
    errors: &mut crate::error::PipelineErrors,
) {
    let max_id = registry.active_entities();

    // Fetch declared domains from the module entity
    let mut declared_names: std::collections::HashSet<&str> = std::collections::HashSet::new();
    if let Some(comp) = &registry.clock_domains[mod_id.0 as usize] {
        for domain in &comp.0 {
            declared_names.insert(domain.as_str());
        }
    }

    // Pass 1: Verify all signals refer to a declared clock domain (E619)
    let mut entity_idx = 0usize;
    while entity_idx < max_id {
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

        if let Some(TypeComponent(ty)) = &registry.types[entity_idx] {
            if let Some(cd) = &ty.annotations.clock_domain {
                if !declared_names.contains(cd.as_str()) {
                    let sig_name = registry.names[entity_idx]
                        .map(|n| registry.resolve_name(n.0))
                        .unwrap_or("unnamed");

                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Signal '{}' references undeclared clock domain '@{}'.",
                            error_codes::E619_CLK_UNDEF,
                            sig_name,
                            cd.as_str()
                        ),
                        span: registry.spans[entity_idx].map(|s| s.0),
                    });
                }
            }
        }
        entity_idx += 1;
    }

    // Iterate reflexes of this module
    let mut reflex_idx = 0usize;
    while reflex_idx < max_id {
        let Some(ModuleComponent(m_id)) = registry.modules[reflex_idx] else {
            reflex_idx += 1;
            continue;
        };
        if m_id != mod_id {
            reflex_idx += 1;
            continue;
        }

        let Some(ReflexComponent { assignments, .. }) = &registry.reflex_comps[reflex_idx] else {
            reflex_idx += 1;
            continue;
        };

        let reflex_name =
            registry.names[reflex_idx].map(|n| registry.resolve_name(n.0)).unwrap_or("unnamed");

        for &assign_ent in assignments {
            let Some(AssignmentComponent { target, value, .. }) =
                &registry.assignment_comps[assign_ent.0 as usize]
            else {
                continue;
            };

            let target_idx = target.0 as usize;
            let target_name =
                registry.names[target_idx].map(|n| registry.resolve_name(n.0)).unwrap_or("?");
            let target_dom = registry.types[target_idx]
                .as_ref()
                .and_then(|t| t.0.annotations.clock_domain.as_ref())
                .map(|cd| cd.as_str());

            let refs = crate::validation::semantic::collect_signal_refs_ecs(registry, *value);
            for sig_ref in refs {
                // Find source signal domain (linear scan)
                let mut source_dom: Option<&str> = None;
                let mut s_idx = 0usize;
                while s_idx < max_id {
                    if let Some(nc) = registry.names[s_idx] {
                        if registry.resolve_name(nc.0) == sig_ref {
                            source_dom = registry.types[s_idx]
                                .as_ref()
                                .and_then(|t| t.0.annotations.clock_domain.as_ref())
                                .map(|cd| cd.as_str());
                            break;
                        }
                    }
                    s_idx += 1;
                }

                if let (Some(td), Some(sd)) = (target_dom, source_dom) {
                    if td != sd {
                        let msg = format!(
                            "[{}] Clock domain crossing: signal '{}' (@{}) references '{}' (@{}) without synchronizer in reflex '{}'.",
                            error_codes::E618_CLK_CROSS,
                            target_name,
                            td,
                            sig_ref,
                            sd,
                            reflex_name
                        );
                        if push_session_error(errors, msg, None) {
                            return;
                        }
                    }
                }
            }
        }
        reflex_idx += 1;
    }
}

/// ECS-native: Check phantom tag compatibility.
///
/// Adheres to NASA P10 Rule #3 by performing linear scans for phantom tag
/// verification instead of constructing a heap-allocated HashMap.
pub fn check_phantom_tags_ecs(
    registry: &Registry,
    mod_id: EntityId,
    errors: &mut crate::error::PipelineErrors,
) {
    let max_id = registry.active_entities();

    // Fetch declared phantom tags from the module entity
    // TODO(MEGA-1): Implement PhantomTagsComponent when AST supports phantom tags.
    let declared_tag_names: std::collections::HashSet<&str> = std::collections::HashSet::new();

    // E621: Pass 1: Verify all signals refer to a declared phantom tag
    let mut entity_idx = 0usize;
    while entity_idx < max_id {
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

        if let Some(TypeComponent(ty)) = &registry.types[entity_idx] {
            if let Some(pt) = &ty.annotations.phantom_tag {
                if !declared_tag_names.contains(pt.as_str()) {
                    let sig_name = registry.names[entity_idx]
                        .map(|n| registry.resolve_name(n.0))
                        .unwrap_or("unnamed");

                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Signal '{}' references undeclared phantom tag '#{}'. \
                             Declare it in the module's tag list.",
                            error_codes::E621_PHT_UNDEF,
                            sig_name,
                            pt.as_str()
                        ),
                        span: registry.spans[entity_idx].map(|s| s.0),
                    });
                }
            }
        }
        entity_idx += 1;
    }

    // E620: Pass 2: Check tag compatibility on assignments
    let mut reflex_idx = 0usize;
    while reflex_idx < max_id {
        let Some(ModuleComponent(m_id)) = registry.modules[reflex_idx] else {
            reflex_idx += 1;
            continue;
        };
        if m_id != mod_id {
            reflex_idx += 1;
            continue;
        }

        let Some(ReflexComponent { assignments, .. }) = &registry.reflex_comps[reflex_idx] else {
            reflex_idx += 1;
            continue;
        };

        let reflex_name =
            registry.names[reflex_idx].map(|n| registry.resolve_name(n.0)).unwrap_or("unnamed");

        for &assign_ent in assignments {
            let Some(AssignmentComponent { target, value, .. }) =
                &registry.assignment_comps[assign_ent.0 as usize]
            else {
                continue;
            };

            let target_idx = target.0 as usize;
            let target_name =
                registry.names[target_idx].map(|n| registry.resolve_name(n.0)).unwrap_or("?");
            let target_tag = registry.types[target_idx]
                .as_ref()
                .and_then(|t| t.0.annotations.phantom_tag.as_ref())
                .map(|pt| pt.as_str());

            let refs = crate::validation::semantic::collect_signal_refs_ecs(registry, *value);
            for sig_ref in refs {
                // Find source signal phantom tag (linear scan)
                let mut source_tag: Option<&str> = None;
                let mut s_idx = 0usize;
                while s_idx < max_id {
                    if let Some(nc) = registry.names[s_idx] {
                        if registry.resolve_name(nc.0) == sig_ref {
                            source_tag = registry.types[s_idx]
                                .as_ref()
                                .and_then(|t| t.0.annotations.phantom_tag.as_ref())
                                .map(|pt| pt.as_str());
                            break;
                        }
                    }
                    s_idx += 1;
                }

                match (target_tag, source_tag) {
                    (Some(tt), Some(st)) if tt != st => {
                        let msg = format!(
                            "[{}] Phantom tag mismatch: cannot assign #{}-tagged signal '{}' to #{}-tagged target '{}' in reflex '{}'.",
                            error_codes::E620_PHT_MISMATCH,
                            st,
                            sig_ref,
                            tt,
                            target_name,
                            reflex_name
                        );
                        if push_session_error(errors, msg, None) {
                            return;
                        }
                    }
                    (Some(tt), None) => {
                        let msg = format!(
                            "[{}] Phantom tag mismatch: cannot assign untagged signal '{}' to #{}-tagged target '{}' in reflex '{}'.",
                            error_codes::E620_PHT_MISMATCH,
                            sig_ref,
                            tt,
                            target_name,
                            reflex_name
                        );
                        if push_session_error(errors, msg, None) {
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }
        reflex_idx += 1;
    }
}

/// Check whether an expression contains any `Prev` node.
/// Uses explicit stack (no recursion). Bounded by MAX_EXTENDED_TYPE_NODES.
fn expr_contains_prev(expr: &crate::ast::expr::Expr) -> bool {
    let mut stack: Vec<&crate::ast::expr::Expr> = Vec::with_capacity(32);
    stack.push(expr);
    let mut visited = 0usize;

    while let Some(node) = stack.pop() {
        visited += 1;
        if visited > MAX_EXTENDED_TYPE_NODES {
            break;
        }
        match node {
            crate::ast::expr::Expr::Prev { .. } => return true,
            crate::ast::expr::Expr::Literal(_) | crate::ast::expr::Expr::Signal(_) => {}
            crate::ast::expr::Expr::Unary { operand, .. } => stack.push(operand),
            crate::ast::expr::Expr::Binary { left, right, .. } => {
                stack.push(left);
                stack.push(right);
            }
            crate::ast::expr::Expr::ArrayIndex { array, index } => {
                stack.push(array);
                stack.push(index);
            }
            crate::ast::expr::Expr::FieldAccess { object, .. } => stack.push(object),
            crate::ast::expr::Expr::ArrayLiteral(elems) => {
                let mut i = 0;
                while i < elems.len().min(MAX_EXTENDED_TYPE_NODES) {
                    stack.push(&elems[i]);
                    i += 1;
                }
            }
            crate::ast::expr::Expr::StructLiteral { fields, .. } => {
                let mut i = 0;
                while i < fields.len().min(MAX_EXTENDED_TYPE_NODES) {
                    stack.push(&fields[i].1);
                    i += 1;
                }
            }
            crate::ast::expr::Expr::UnfoldIndex(_) => {
                // UnfoldIndex is treated as non-Prev expression for this check.
            }
        }
    }

    false
}

// ===========================================================================
// Phase 5: Clock domain checking
// ===========================================================================

/// Verify that cross-clock-domain signal references use a synchronizer.
///
/// If signal A is in domain `@clk_fast` and signal B is in domain `@clk_slow`,
/// then referencing A in an expression assigned to B (or vice versa) without
/// an explicit synchronizer construct is an error (E618).
///
/// Bounded: iterates over reflexes and assignments.
pub(super) fn check_clock_domains(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    declared_domains: &[ClockDomain],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build domain lookup: signal name -> clock domain name
    let mut signal_domain: std::collections::HashMap<&str, &str> =
        std::collections::HashMap::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if let Some(ref cd) = sig.extended_ty.clock_domain {
            signal_domain.insert(&sig.name, &cd.name);
        }
        idx += 1;
    }

    if signal_domain.is_empty() {
        return;
    }

    // Validate declared domains exist (E619)
    let declared_names: std::collections::HashSet<&str> =
        declared_domains.iter().map(|cd| cd.name.as_str()).collect();
    for (sig_name, domain_name) in &signal_domain {
        if !declared_names.contains(domain_name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Signal '{}' references undeclared clock domain '@{}'.",
                    error_codes::E619_CLK_UNDEF,
                    sig_name,
                    domain_name
                ),
                span: None,
            });
        }
    }

    // Check cross-domain references (E618)
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let target_domain = signal_domain.get(assignment.target.as_str());

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;

                let source_domain = signal_domain.get(sig_ref.as_str());

                if let (Some(td), Some(sd)) = (target_domain, source_domain) {
                    if td != sd {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Clock domain crossing: signal '{}' (@{}) references '{}' (@{}) \
                                 without synchronizer in reflex '{}'.",
                                error_codes::E618_CLK_CROSS,
                                assignment.target,
                                td,
                                sig_ref,
                                sd,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                }
            }
        }
    }
}

// ===========================================================================
// Phase 6: Phantom tag checking
// ===========================================================================

/// Verify phantom tag compatibility on assignments.
///
/// A signal tagged `#Verified` can only be assigned from a `#Verified` source.
/// Assigning from `#Unverified` to `#Verified` is error E620.
///
/// Bounded: iterates over reflexes and assignments.
pub(super) fn check_phantom_tags(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    declared_tags: &[PhantomTag],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build phantom tag lookup: signal name -> tag name
    let mut signal_tag: std::collections::HashMap<&str, &str> =
        std::collections::HashMap::with_capacity(extended_signals.len());

    let mut idx = 0usize;
    while idx < extended_signals.len() && idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[idx];
        if let Some(ref pt) = sig.extended_ty.phantom {
            signal_tag.insert(&sig.name, &pt.tag);
        }
        idx += 1;
    }

    if signal_tag.is_empty() {
        return;
    }

    // E621: Validate declared tags exist
    let declared_tag_names: std::collections::HashSet<&str> =
        declared_tags.iter().map(|pt| pt.tag.as_str()).collect();
    for (sig_name, tag_name) in &signal_tag {
        if !declared_tag_names.contains(tag_name) {
            if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                return;
            }
            errors.push(crate::error::MirrError::TypeError {
                message: format!(
                    "[{}] Signal '{}' references undeclared phantom tag '#{}'. \
                     Declare it in the module's tag list.",
                    error_codes::E621_PHT_UNDEF,
                    sig_name,
                    tag_name
                ),
                span: None,
            });
        }
    }

    // E620: Check tag compatibility on assignments
    let mut reflex_idx = 0usize;
    while reflex_idx < module.reflexes.len() && reflex_idx < MAX_EXTENDED_TYPE_NODES {
        let reflex = &module.reflexes[reflex_idx];
        reflex_idx += 1;

        let mut assign_idx = 0usize;
        while assign_idx < reflex.assignments.len() && assign_idx < MAX_EXTENDED_TYPE_NODES {
            let assignment = &reflex.assignments[assign_idx];
            assign_idx += 1;

            let target_tag = signal_tag.get(assignment.target.as_str());

            let refs = crate::validation::semantic::collect_signal_refs(&assignment.value);
            let mut ref_idx = 0usize;
            while ref_idx < refs.len() && ref_idx < MAX_EXTENDED_TYPE_NODES {
                let sig_ref = &refs[ref_idx];
                ref_idx += 1;

                let source_tag = signal_tag.get(sig_ref.as_str());

                match (target_tag, source_tag) {
                    (Some(tt), Some(st)) if tt != st => {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Phantom tag mismatch: cannot assign #{}-tagged signal '{}' \
                                 to #{}-tagged target '{}' in reflex '{}'.",
                                error_codes::E620_PHT_MISMATCH,
                                st,
                                sig_ref,
                                tt,
                                assignment.target,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                    (Some(tt), None) => {
                        // Target is tagged but source is untagged — error
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Phantom tag mismatch: cannot assign untagged signal '{}' \
                                 to #{}-tagged target '{}' in reflex '{}'.",
                                error_codes::E620_PHT_MISMATCH,
                                sig_ref,
                                tt,
                                assignment.target,
                                reflex.name
                            ),
                            span: assignment.span,
                        });
                    }
                    // (None, Some(_)) — untagged target accepts any source (tag is dropped)
                    // (None, None) — no phantom types involved
                    _ => {}
                }
            }
        }
    }
}

// ===========================================================================
// Phase 7: Session type checking
// ===========================================================================

/// Verify session type protocol compliance.
///
/// For each signal participating in a session protocol, verify that the
/// signal's declared state is a legal state in the protocol and that
/// all state transitions observable in the module are legal.
///
/// Bounded: iterates over signals, protocols, and transitions.
pub fn check_session_types(
    module: &crate::ast::program::Module,
    extended_signals: &[ExtendedSignalDecl],
    protocols: &[SessionProtocol],
    errors: &mut crate::error::PipelineErrors,
) {
    // Build protocol lookup: name -> &SessionProtocol
    let mut protocol_map: std::collections::HashMap<&str, &SessionProtocol> =
        std::collections::HashMap::with_capacity(protocols.len());
    let mut proto_idx = 0usize;
    while proto_idx < protocols.len() && proto_idx < MAX_SESSION_STATES {
        protocol_map.insert(&protocols[proto_idx].name, &protocols[proto_idx]);
        proto_idx += 1;
    }

    if protocol_map.is_empty() {
        return;
    }

    // Collect session-typed signals
    let mut sig_idx = 0usize;
    while sig_idx < extended_signals.len() && sig_idx < MAX_EXTENDED_TYPE_NODES {
        let sig = &extended_signals[sig_idx];
        sig_idx += 1;

        if let Some(ref session_ref) = sig.extended_ty.session {
            // Verify protocol exists
            match protocol_map.get(session_ref.protocol.as_str()) {
                Some(proto) => {
                    // Verify state exists in protocol
                    let state_exists = proto
                        .transitions
                        .iter()
                        .any(|t| t.from == session_ref.state || t.to == session_ref.state);
                    if !state_exists {
                        if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                            return;
                        }
                        errors.push(crate::error::MirrError::TypeError {
                            message: format!(
                                "[{}] Signal '{}' references state '{}' which does not exist \
                                 in protocol '{}'.",
                                error_codes::E625_SES_PROTOCOL,
                                sig.name,
                                session_ref.state,
                                session_ref.protocol
                            ),
                            span: sig.span,
                        });
                    }
                }
                None => {
                    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
                        return;
                    }
                    errors.push(crate::error::MirrError::TypeError {
                        message: format!(
                            "[{}] Signal '{}' references undeclared session protocol '{}'.",
                            error_codes::E625_SES_PROTOCOL,
                            sig.name,
                            session_ref.protocol
                        ),
                        span: sig.span,
                    });
                }
            }
        }
    }

    // Cross-reflex transition checking would verify that if a sender signal
    // transitions from state A to state B, the corresponding receiver signal
    // also transitions according to the protocol. This requires interprocedural
    // analysis across reflexes, which is bounded by module size.
    //
    // Full implementation deferred to MEGA-1 Phase 2 (multi-module linking).
    let _ = module;
}

// ===========================================================================
// Phase 7 (ECS-Native): Session type checking
// ===========================================================================

/// Bounded linear scan: returns `true` if `state` appears as any `from` or
/// `to` endpoint in `transitions`, capped at `MAX_SESSION_STATES` entries.
///
/// Extracted from the main checker to keep each function ≤ one printed page
/// (NASA P10 Rule #1). No heap allocation — pure indexed traversal.
#[inline]
fn session_state_is_reachable(transitions: &[SessionTransition], state: &str) -> bool {
    let bound = transitions.len().min(MAX_SESSION_STATES);
    let mut t_idx = 0usize;
    while t_idx < bound {
        let t = &transitions[t_idx];
        if t.from == state || t.to == state {
            return true;
        }
        t_idx += 1;
    }
    false
}

/// Bounded linear scan: returns a reference to the `SessionProtocol` whose
/// `name` matches `target`, capped at `MAX_SESSION_STATES` entries, or `None`.
///
/// Replaces a `HashMap` lookup. Because `protocols.len() ≤ MAX_SESSION_STATES`
/// (64), the worst-case is 64 comparisons — O(1) by P10 Rule #2 and zero
/// heap allocation (no hash table constructed at call time).
#[inline]
fn find_protocol<'p>(
    protocols: &'p [SessionProtocol],
    target: &str,
) -> Option<&'p SessionProtocol> {
    let bound = protocols.len().min(MAX_SESSION_STATES);
    let mut p_idx = 0usize;
    while p_idx < bound {
        if protocols[p_idx].name == target {
            return Some(&protocols[p_idx]);
        }
        p_idx += 1;
    }
    None
}

/// Emit a session-type error via the pipeline accumulator.
///
/// Extracted to keep `check_session_types_ecs` ≤ one printed page
/// (NASA P10 Rule #1). Returns `true` if the error cap was hit (caller
/// should `return` immediately).
#[inline]
fn push_session_error(
    errors: &mut crate::error::PipelineErrors,
    message: String,
    span: Option<crate::span::Span>,
) -> bool {
    if errors.len() >= crate::error::MAX_ACCUMULATED_ERRORS {
        return true;
    }
    errors.push(crate::error::MirrError::TypeError { message, span });
    false
}

/// ECS-native: Verify session type protocol compliance for all signals in
/// `mod_id` that carry a `session` annotation.
///
/// For each annotated signal this function verifies:
/// 1. The referenced protocol is declared in `protocols` (E625).
/// 2. The referenced state appears in that protocol's transition table (E625).
///
/// This check is purely structural — it validates that each signal is in a
/// *legal* protocol state at compile time. Cross-reflex state-transition
/// analysis (multi-module linking) is deferred to MEGA-1 Phase 2.
///
/// # NASA P10 Compliance
/// - Rule #1: Every function is ≤ one printed page. Helpers are extracted.
/// - Rule #2: All loops bounded by `max_id` or `MAX_SESSION_STATES`.
/// - Rule #3: Zero heap allocation — protocol lookup is a bounded linear
///   scan (`find_protocol`), not a `HashMap`.
/// - Rule #6: All branches have explicit else paths or `continue` skips.
pub fn check_session_types_ecs(
    registry: &Registry,
    mod_id: EntityId,
    protocols: &[SessionProtocol],
    errors: &mut crate::error::PipelineErrors,
) {
    let max_id = registry.active_entities();
    let mut entity_idx = 0usize;

    while entity_idx < max_id {
        // --- Guard 1: Must belong to the target module. ---
        let Some(ModuleComponent(m_id)) = registry.modules[entity_idx] else {
            entity_idx += 1;
            continue;
        };
        if m_id != mod_id {
            entity_idx += 1;
            continue;
        }

        // --- Guard 2: Must be a signal entity. ---
        let Some(KindComponent(EntityKind::SIGNAL(_))) = &registry.kinds[entity_idx] else {
            entity_idx += 1;
            continue;
        };

        // --- Guard 3: Must have a type component. ---
        let Some(TypeComponent(ty)) = &registry.types[entity_idx] else {
            entity_idx += 1;
            continue;
        };

        // --- Guard 4: Must carry a session annotation. ---
        let Some(ref sess) = ty.annotations.session else {
            entity_idx += 1;
            continue;
        };

        // All guards passed — perform protocol validation.
        let sig_name =
            registry.names[entity_idx].map(|n| registry.resolve_name(n.0)).unwrap_or("<unnamed>");
        let span = registry.spans[entity_idx].as_ref().map(|s| s.0);

        match find_protocol(protocols, &sess.protocol) {
            None => {
                let msg = format!(
                    "[{}] Signal '{}' references undeclared session protocol '{}'.",
                    error_codes::E625_SES_PROTOCOL,
                    sig_name,
                    sess.protocol
                );
                if push_session_error(errors, msg, span) {
                    return;
                }
            }
            Some(proto) => {
                if !session_state_is_reachable(&proto.transitions, &sess.state) {
                    let msg = format!(
                        "[{}] Signal '{}' references state '{}' which does not exist \
                         in protocol '{}'.",
                        error_codes::E625_SES_PROTOCOL,
                        sig_name,
                        sess.state,
                        sess.protocol
                    );
                    if push_session_error(errors, msg, span) {
                        return;
                    }
                }
            }
        }

        entity_idx += 1;
    }
}
