#![forbid(unsafe_code)]

use crate::ast::types::SignalKind;
use crate::ecs::components::*;
use crate::ecs::Registry;
use crate::error::{MirrError, PipelineErrors};
use std::collections::HashMap;

impl Registry {
    /// Perform semantic validation on the entire registry.
    /// Operates directly on ECS components.
    pub fn semantic_validate(&self) -> Result<(), PipelineErrors> {
        let mut errors = PipelineErrors::new();

        self.validate_duplicate_names(&mut errors);
        self.validate_guards(&mut errors);
        self.validate_reflexes(&mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_duplicate_names(&self, errors: &mut PipelineErrors) {
        let max_id = self.next_id as usize;
        // Map: (Scope, Name) -> EntityId
        let mut seen_names: HashMap<(Option<String>, String), usize> = HashMap::new();

        for i in 0..max_id {
            if let Some(NameComponent(name)) = &self.names[i] {
                if let Some(KindComponent(EntityKind::PATTERN | EntityKind::ASSIGNMENT)) =
                    self.kinds[i]
                {
                    continue;
                }
                // Determine if this is a signal, guard, reflex, or property.
                // This fulfills the S03 Symbol Shadowing / Collision check.
                let scope = self.modules[i].as_ref().map(|s| s.0 .0.to_string());
                let key = (scope, name.clone());

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

    fn validate_guards(&self, errors: &mut PipelineErrors) {
        let max_id = self.next_id as usize;
        for i in 0..max_id {
            if let Some(KindComponent(EntityKind::GUARD)) = self.kinds[i] {
                let name_is_always = self.names[i].as_ref().map(|n| n.0.as_str()) == Some("always");
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
                        self.validate_expr_entity(*cond_ent, errors);
                    } else {
                        let name_str =
                            self.names[i].as_ref().map(|n| n.0.as_str()).unwrap_or("<no name>");
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

    fn validate_reflexes(&self, errors: &mut PipelineErrors) {
        let max_id = self.next_id as usize;
        for i in 0..max_id {
            if let Some(ReflexComponent { guards, assignments }) = &self.reflex_comps[i] {
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
                        if let Some(KindComponent(EntityKind::SIGNAL(sk))) =
                            self.kinds[target.0 as usize]
                        {
                            if !matches!(sk, SignalKind::Output | SignalKind::Internal) {
                                let msg = format!(
                                    "{} Reflex assigns to non-writable signal.",
                                    crate::error_codes::ec(206)
                                );
                                errors.push(MirrError::SemanticError { message: msg, span: None });
                            }
                        }
                        self.validate_expr_entity(value, errors);
                    }
                }
            }
        }
    }

    fn validate_expr_entity(&self, root_ent: EntityId, errors: &mut PipelineErrors) {
        let mut stack = vec![root_ent];
        let mut iterations = 0;
        let max_iterations = 256; // Bound the expression depth for validation

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

            if let Some(SignalRefComponent(sig_ent)) = self.signal_refs[idx] {
                if self.kinds[sig_ent.0 as usize].is_none() {
                    errors.push(MirrError::SemanticError {
                        message: format!(
                            "{} Expression references undeclared signal.",
                            crate::error_codes::ec(204)
                        ),
                        span: None,
                    });
                }
            } else if let Some(PendingSignalRef(name)) = &self.pending_signal_refs[idx] {
                if let Some(resolved_ent) = self.get_entity_by_name(name) {
                    stack.push(resolved_ent);
                } else {
                    errors.push(MirrError::SemanticError {
                        message: format!(
                            "{} Expression references undeclared signal {:?}.",
                            crate::error_codes::ec(204),
                            name
                        ),
                        span: None,
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
}
