#![forbid(unsafe_code)]

use crate::ast::types::SignalKind;
use crate::ecs::components::*;
use crate::ecs::Registry;
use crate::error::{MirrError, PipelineErrors};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum ExprValidationContext<'a> {
    Guard(&'a str),
    ReflexAssignment(&'a str),
    Property(&'a str),
    General,
}

impl Registry {
    /// Perform semantic validation on the entire registry.
    /// Operates directly on ECS components.
    pub fn semantic_validate(&self) -> Result<(), PipelineErrors> {
        let mut errors = PipelineErrors::new();

        let mut signal_name_candidates = Vec::new();
        let max_id = self.next_id as usize;
        for i in 0..max_id {
            if let Some(KindComponent(EntityKind::SIGNAL(_))) = self.kinds[i] {
                if let Some(name_comp) = self.names[i] {
                    signal_name_candidates.push(self.resolve_name(name_comp.0));
                }
            }
        }

        self.validate_duplicate_names(&mut errors);
        self.validate_guards(&mut errors, &signal_name_candidates);
        self.validate_reflexes(&mut errors, &signal_name_candidates);
        self.validate_single_writer(&mut errors);
        self.validate_composite_exprs(&mut errors);
        self.validate_properties(&mut errors, &signal_name_candidates);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_duplicate_names(&self, errors: &mut PipelineErrors) {
        let max_id = self.next_id as usize;
        // Map: (module_entity_id, name_intern_id) -> first_seen_idx
        // Both keys are u32 — integer comparison, no heap at lookup time.
        let mut seen_names: HashMap<(Option<u32>, u32), usize> = HashMap::new();

        for i in 0..max_id {
            if let Some(nc) = self.names[i] {
                if let Some(KindComponent(EntityKind::PATTERN | EntityKind::ASSIGNMENT)) =
                    self.kinds[i]
                {
                    continue;
                }
                // Scope key: module entity id (None = global).
                let scope: Option<u32> = self.modules[i].map(|m| m.0 .0);
                let key = (scope, nc.0 .0);

                if let Some(first_idx) = seen_names.get(&key) {
                    let first_idx = *first_idx;

                    // SAFE: Check for KindComponent presence before unwrapping
                    let first_kind = match self.kinds[first_idx] {
                        Some(k) => k.0,
                        None => {
                            errors.push(MirrError::SemanticError {
                                message: format!(
                                    "{} Entity {} has a name but is missing a KindComponent.",
                                    crate::error_codes::ec(200),
                                    first_idx
                                ),
                                span: None,
                            });
                            continue;
                        }
                    };
                    let current_kind = match self.kinds[i] {
                        Some(k) => k.0,
                        None => {
                            errors.push(MirrError::SemanticError {
                                message: format!(
                                    "{} Entity {} has a name but is missing a KindComponent.",
                                    crate::error_codes::ec(200),
                                    i
                                ),
                                span: None,
                            });
                            continue;
                        }
                    };

                    // Resolve name once for the error message.
                    let name = self.resolve_name(nc.0);
                    let (code, msg) = if first_kind == EntityKind::MODULE
                        && current_kind == EntityKind::MODULE
                    {
                        (crate::error_codes::ec(215), format!("Duplicate module name: '{}'.", name))
                    } else if matches!(first_kind, EntityKind::SIGNAL(_))
                        && matches!(current_kind, EntityKind::SIGNAL(_))
                    {
                        (crate::error_codes::ec(201), format!("Duplicate signal name: '{}'.", name))
                    } else if first_kind == EntityKind::GUARD && current_kind == EntityKind::GUARD {
                        (crate::error_codes::ec(213), format!("Duplicate guard name: '{}'.", name))
                    } else if first_kind == EntityKind::REFLEX && current_kind == EntityKind::REFLEX
                    {
                        (crate::error_codes::ec(212), format!("Duplicate reflex name: '{}'.", name))
                    } else if first_kind == EntityKind::PROPERTY
                        && current_kind == EntityKind::PROPERTY
                    {
                        (
                            crate::error_codes::ec(214),
                            format!("Duplicate property name: '{}'.", name),
                        )
                    } else {
                        // Cross-type collision (e.g. signal vs guard)
                        (
                            crate::error_codes::ec(201),
                            format!(
                                "Name collision: '{}' is defined as both a {} and a {}.",
                                name,
                                first_kind.describe(),
                                current_kind.describe()
                            ),
                        )
                    };

                    let span = self.spans[i].as_ref().map(|s| s.0);
                    errors.push(MirrError::SemanticError {
                        message: format!("{} {}", code, msg),
                        span,
                    });
                } else {
                    seen_names.insert(key, i);
                }
            }
        }
    }

    fn validate_guards(&self, errors: &mut PipelineErrors, signal_name_candidates: &[&str]) {
        let max_id = self.next_id as usize;
        for i in 0..max_id {
            if let Some(KindComponent(EntityKind::GUARD)) = self.kinds[i] {
                let name_is_always =
                    self.names[i].map(|n| self.resolve_name(n.0)) == Some("always");
                // All GUARDS must have a name, cycles, and condition
                if self.names[i].is_none() {
                    errors.push(MirrError::SemanticError {
                        message: format!(
                            "{} Guard entity {} is missing a name.",
                            crate::error_codes::ec(213),
                            i
                        ),
                        span: None,
                    });
                }
                if !name_is_always {
                    if self.cycles[i].is_none() {
                        errors.push(MirrError::SemanticError {
                            message: format!(
                                "{} Guard entity {} is missing a CyclesComponent.",
                                crate::error_codes::ec(306),
                                i
                            ),
                            span: None,
                        });
                    }
                    if let Some(ConditionComponent(cond_ent)) = &self.conditions[i] {
                        let name_str =
                            self.names[i].map(|n| self.resolve_name(n.0)).unwrap_or("<no name>");
                        self.validate_expr_entity(
                            *cond_ent,
                            ExprValidationContext::Guard(name_str),
                            errors,
                            signal_name_candidates,
                        );
                    } else {
                        let name_str =
                            self.names[i].map(|n| self.resolve_name(n.0)).unwrap_or("<no name>");
                        errors.push(MirrError::SemanticError {
                            message: format!(
                                "{} Guard entity {} (name: {:?}) is missing a ConditionComponent.",
                                crate::error_codes::ec(306),
                                i,
                                name_str
                            ),
                            span: None,
                        });
                    }
                }
            } else if self.conditions[i].is_some() {
                // Non-GUARD with a condition is a structural error
                errors.push(MirrError::SemanticError {
                    message: format!(
                        "{} Entity {} has a ConditionComponent but is not a GUARD.",
                        crate::error_codes::ec(200),
                        i
                    ),
                    span: None,
                });
            }
        }
    }

    fn validate_single_writer(&self, errors: &mut PipelineErrors) {
        // Track writers per signal by guard to detect conflicts in linear time.
        // HashMap: (target_signal_idx, guard_idx) -> (reflex_idx, origin)
        let mut signal_guard_writers: HashMap<(usize, usize), (usize, Option<&str>)> =
            HashMap::new();
        // Emitted pairs to avoid duplicate E216 for same conflicting reflexes
        let mut emitted_pairs: HashMap<usize, std::collections::HashSet<(usize, usize)>> =
            HashMap::new();

        let max_id = self.next_id as usize;
        for reflex_idx in 0..max_id {
            if let Some(ReflexComponent { guards, assignments, origin }) =
                &self.reflex_comps[reflex_idx]
            {
                let current_origin = origin.as_deref();

                for assign_ent in assignments {
                    let Some(AssignmentComponent { target, .. }) =
                        &self.assignment_comps[assign_ent.0 as usize]
                    else {
                        continue;
                    };

                    let target_idx = target.0 as usize;
                    let emitted = emitted_pairs.entry(target_idx).or_default();

                    for guard_ent in guards {
                        let guard_idx = guard_ent.0 as usize;
                        let key = (target_idx, guard_idx);

                        if let Some(&(existing_reflex, existing_origin)) =
                            signal_guard_writers.get(&key)
                        {
                            let existing_name = self.names[existing_reflex]
                                .map(|n| self.resolve_name(n.0))
                                .unwrap_or("unnamed");
                            let current_name = self.names[reflex_idx]
                                .map(|n| self.resolve_name(n.0))
                                .unwrap_or("unnamed");

                            let existing_base = self.get_reflex_base_name(existing_name);
                            let current_base = self.get_reflex_base_name(current_name);

                            if existing_base == current_base {
                                continue;
                            }

                            let pair = if existing_name <= current_name {
                                (existing_reflex, reflex_idx)
                            } else {
                                (reflex_idx, existing_reflex)
                            };

                            if !emitted.insert(pair) {
                                continue;
                            }

                            let target_name = self.names[target_idx]
                                .map(|n| self.resolve_name(n.0))
                                .unwrap_or("unnamed");

                            let msg = match (existing_origin, current_origin) {
                                (Some(p1), Some(p2)) => format!(
                                    "{} Signal '{}' has multiple writers: reflex '{}' (from pattern '{}') and reflex '{}' (from pattern '{}').",
                                    crate::error_codes::ec(216),
                                    target_name,
                                    existing_name,
                                    p1,
                                    current_name,
                                    p2
                                ),
                                _ => format!(
                                    "{} Signal '{}' has multiple writers: reflex '{}' and reflex '{}'.",
                                    crate::error_codes::ec(216),
                                    target_name,
                                    existing_name,
                                    current_name
                                ),
                            };

                            let span = self.spans[reflex_idx].map(|s| s.0);
                            errors.push(MirrError::SemanticError { message: msg, span });
                        } else {
                            signal_guard_writers.insert(key, (reflex_idx, current_origin));
                        }
                    }
                }
            }
        }
    }

    fn get_reflex_base_name<'a>(&self, name: &'a str) -> &'a str {
        if let Some(idx) = name.find("_split_") {
            &name[..idx]
        } else {
            name
        }
    }

    fn validate_reflexes(&self, errors: &mut PipelineErrors, signal_name_candidates: &[&str]) {
        let max_id = self.next_id as usize;
        for i in 0..max_id {
            if let Some(ReflexComponent { guards, assignments, .. }) = &self.reflex_comps[i] {
                for g_ent in guards {
                    if self.kinds[g_ent.0 as usize].is_none() {
                        let msg = format!(
                            "{} Reflex references unknown guard entity {}.",
                            crate::error_codes::ec(205),
                            g_ent.0
                        );
                        errors.push(MirrError::SemanticError { message: msg, span: None });
                    }
                }

                for a_ent in assignments {
                    if let Some(AssignmentComponent { target, value }) =
                        self.assignment_comps[a_ent.0 as usize]
                    {
                        let target_name = self.names[target.0 as usize]
                            .map(|n| self.resolve_name(n.0))
                            .unwrap_or("<unknown>");
                        let rname =
                            self.names[i].map(|n| self.resolve_name(n.0)).unwrap_or("<no name>");

                        let has_bracket = target_name.contains('[');
                        let clean_target = if let Some(pos) = target_name.find('[') {
                            target_name[..pos].trim()
                        } else {
                            target_name
                        };

                        if let Some(sig_ent) = self.get_entity_by_name(clean_target) {
                            if has_bracket {
                                if let Some(TypeComponent(ty)) = &self.types[sig_ent.0 as usize] {
                                    if !matches!(
                                        ty.core,
                                        crate::ast::types::SignalType::Array { .. }
                                    ) {
                                        let msg = format!(
                                            "{} Reflex '{}' assigns to non-array signal '{}' with indexing.",
                                            crate::error_codes::ec(207),
                                            rname,
                                            target_name
                                        );
                                        errors.push(MirrError::SemanticError {
                                            message: msg,
                                            span: self.spans[i].map(|s| s.0),
                                        });
                                        continue;
                                    }
                                }
                            }

                            if let Some(KindComponent(EntityKind::SIGNAL(sk))) =
                                self.kinds[sig_ent.0 as usize]
                            {
                                if !matches!(sk, SignalKind::Output | SignalKind::Internal) {
                                    let msg = format!(
                                        "{} Reflex '{}' assigns to input signal '{}', which is not writable.",
                                        crate::error_codes::ec(206),
                                        rname,
                                        target_name
                                    );
                                    errors.push(MirrError::SemanticError {
                                        message: msg,
                                        span: self.spans[i].map(|s| s.0),
                                    });
                                }
                            }
                        } else {
                            let mut msg = format!(
                                "{} Reflex '{}' assigns to undeclared signal '{}'.",
                                crate::error_codes::ec(207),
                                rname,
                                target_name
                            );
                            if let Some(s) =
                                crate::suggest::closest_match(clean_target, signal_name_candidates)
                            {
                                msg.push_str(&format!(" Did you mean '{}'?", s));
                            }
                            errors.push(MirrError::SemanticError {
                                message: msg,
                                span: self.spans[i].map(|s| s.0),
                            });
                        }
                        self.validate_expr_entity(
                            value,
                            ExprValidationContext::ReflexAssignment(rname),
                            errors,
                            signal_name_candidates,
                        );
                    }
                }
            }
        }
    }

    fn validate_expr_entity(
        &self,
        root_ent: EntityId,
        context: ExprValidationContext<'_>,
        errors: &mut PipelineErrors,
        signal_name_candidates: &[&str],
    ) {
        let mut stack = vec![root_ent];
        let mut iterations = 0;
        let max_iterations = 8192; // Bound the expression depth for validation

        while let Some(ent) = stack.pop() {
            iterations += 1;
            if iterations > max_iterations {
                errors.push(MirrError::SemanticError {
                    message: format!(
                        "{} Expression depth limit exceeded or circular reference detected at entity {}.",
                        crate::error_codes::ec(306),
                        ent.0
                    ),
                    span: None,
                });
                return;
            }

            let idx = ent.0 as usize;
            if idx >= self.next_id as usize {
                errors.push(MirrError::SemanticError {
                    message: format!(
                        "{} Expression references non-existent entity {}.",
                        crate::error_codes::ec(204),
                        idx
                    ),
                    span: None,
                });
                continue;
            }

            if self.signal_refs[idx].is_some() {
                // Resolved signal ref is valid.
            } else if let Some(PendingSignalRef(name)) = &self.pending_signal_refs[idx] {
                // Attempt to resolve it. If it fails, report undeclared.
                if self.get_entity_by_name(name).is_some() {
                    // Valid, it exists. (Though hydration should have resolved it,
                    // we tolerate it if the entity exists).
                } else {
                    let suggestion = crate::suggest::closest_match(name, signal_name_candidates);
                    let mut msg = match context {
                        ExprValidationContext::Guard(gname) => {
                            format!(
                                "{} Guard '{}' references undeclared signal '{}'.",
                                crate::error_codes::ec(204),
                                gname,
                                name
                            )
                        }
                        ExprValidationContext::ReflexAssignment(rname) => {
                            format!(
                                "{} Reflex '{}' assignment references undeclared signal '{}'.",
                                crate::error_codes::ec(208),
                                rname,
                                name
                            )
                        }
                        ExprValidationContext::Property(pname) => {
                            format!(
                                "{} Property '{}' references undeclared signal '{}'.",
                                crate::error_codes::ec(211),
                                pname,
                                name
                            )
                        }
                        ExprValidationContext::General => {
                            format!(
                                "{} Expression references undeclared signal {:?}.",
                                crate::error_codes::ec(204),
                                name
                            )
                        }
                    };
                    if let Some(s) = suggestion {
                        msg.push_str(&format!(" Did you mean '{}'?", s));
                    }
                    errors.push(MirrError::SemanticError {
                        message: msg,
                        span: self.spans[idx].map(|s| s.0),
                    });
                }
            } else if self.literals[idx].is_some() {
                // Literal is a valid leaf node, no further traversal needed.
            } else if let Some(BinaryComponent { left, right, .. }) = self.binary_ops[idx] {
                stack.push(right);
                stack.push(left);
            } else if let Some(UnaryComponent { operand, .. }) = self.unary_ops[idx] {
                stack.push(operand);
            } else if let Some(PrevComponent { signal, delay }) = self.prev_ops[idx] {
                if delay == 0 {
                    errors.push(MirrError::SemanticError {
                        message: format!(
                            "{} prev() with delay 0 is illegal.",
                            crate::error_codes::ec(209)
                        ),
                        span: None,
                    });
                }
                stack.push(signal);
            } else if let Some(ArrayIndexComponent { array, index }) = &self.array_indices[idx] {
                stack.push(*index);
                stack.push(*array);
            } else if let Some(FieldAccessComponent { object, .. }) = &self.field_accesses[idx] {
                stack.push(*object);
            } else if let Some(ArrayLiteralComponent(elements)) = &self.array_literals[idx] {
                for &el in elements {
                    stack.push(el);
                }
            } else if let Some(StructLiteralComponent { fields, .. }) = &self.struct_literals[idx] {
                for &(_, el) in fields {
                    stack.push(el);
                }
            } else if self.unfold_indices[idx].is_some() {
                // Leaf node representing unfold loop index variable, valid.
            } else if let Some(KindComponent(EntityKind::SIGNAL(_) | EntityKind::GUARD)) =
                self.kinds[idx]
            {
                // Direct signal or guard entity reference is a valid leaf node.
            } else {
                // Leaf node that is neither a SignalRef nor a Literal is a broken reference.
                errors.push(MirrError::SemanticError {
                    message: format!(
                        "{} Expression references undeclared or broken signal (entity {}).",
                        crate::error_codes::ec(204),
                        idx
                    ),
                    span: None,
                });
            }
        }
    }

    fn validate_composite_exprs(&self, errors: &mut PipelineErrors) {
        let max_id = self.next_id as usize;

        for i in 0..max_id {
            if let Some(FieldAccessComponent { object, field }) = &self.field_accesses[i] {
                let mut sig_ent_opt = None;
                if let Some(SignalRefComponent(sig_ent)) = self.signal_refs[object.0 as usize] {
                    sig_ent_opt = Some(sig_ent);
                } else if let Some(PendingSignalRef(name)) =
                    &self.pending_signal_refs[object.0 as usize]
                {
                    sig_ent_opt = self.get_entity_by_name(name);
                }

                if let Some(sig_ent) = sig_ent_opt {
                    let sig_idx = sig_ent.0 as usize;
                    if let Some(TypeComponent(ty)) = &self.types[sig_idx] {
                        use crate::ast::types::SignalType;
                        if let SignalType::Struct { fields, .. } = &ty.core {
                            if !fields.iter().any(|(f, _)| f == field) {
                                let sig_name = self.names[sig_idx]
                                    .map(|n| self.resolve_name(n.0))
                                    .unwrap_or("unnamed");
                                errors.push(MirrError::SemanticError {
                                    message: format!(
                                        "{} No field '{}' on struct signal '{}'.",
                                        crate::error_codes::ec(229),
                                        field,
                                        sig_name
                                    ),
                                    span: self.spans[i].map(|s| s.0),
                                });
                            }
                        }
                    }
                }
            }

            if let Some(ArrayIndexComponent { array, .. }) = &self.array_indices[i] {
                let mut sig_ent_opt = None;
                if let Some(SignalRefComponent(sig_ent)) = self.signal_refs[array.0 as usize] {
                    sig_ent_opt = Some(sig_ent);
                } else if let Some(PendingSignalRef(name)) =
                    &self.pending_signal_refs[array.0 as usize]
                {
                    sig_ent_opt = self.get_entity_by_name(name);
                }

                if let Some(sig_ent) = sig_ent_opt {
                    let sig_idx = sig_ent.0 as usize;
                    if let Some(TypeComponent(ty)) = &self.types[sig_idx] {
                        use crate::ast::types::SignalType;
                        if !matches!(
                            ty.core,
                            SignalType::Array { .. }
                                | SignalType::Unsigned(_)
                                | SignalType::Signed(_)
                        ) {
                            let sig_name = self.names[sig_idx]
                                .map(|n| self.resolve_name(n.0))
                                .unwrap_or("unnamed");
                            errors.push(MirrError::SemanticError {
                                message: format!(
                                    "{} Signal '{}' is not an indexable type (array, unsigned, or signed) but is indexed.",
                                    crate::error_codes::ec(230),
                                    sig_name
                                ),
                                span: self.spans[i].map(|s| s.0),
                            });
                        }
                    }
                }
            }
        }
    }

    fn validate_properties(&self, errors: &mut PipelineErrors, signal_name_candidates: &[&str]) {
        let max_id = self.next_id as usize;
        for i in 0..max_id {
            if let Some(KindComponent(EntityKind::PROPERTY)) = self.kinds[i] {
                if let Some(PropertyComponent { formula_exprs, .. }) = &self.property_comps[i] {
                    let prop_name = self.names[i].map(|n| self.resolve_name(n.0));
                    let context = prop_name
                        .map(ExprValidationContext::Property)
                        .unwrap_or(ExprValidationContext::General);
                    for &expr_ent in formula_exprs {
                        self.validate_expr_entity(
                            expr_ent,
                            context,
                            errors,
                            signal_name_candidates,
                        );
                    }
                }
            }
        }
    }
}
