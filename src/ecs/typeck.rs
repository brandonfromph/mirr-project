#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, SignalType, UnaryOp};
use crate::ecs::components::*;
use crate::ecs::Registry;
use crate::error::{MirrError, PipelineErrors};

impl Registry {
    /// Perform type checking on the entire registry.
    /// Operates directly on ECS components.
    pub fn typecheck(&mut self) -> Result<(), PipelineErrors> {
        let mut errors = PipelineErrors::new();
        let max_id = self.next_id as usize;

        // T14: Guard conditions must be Bool.
        for i in 0..max_id {
            if let Some(ConditionComponent(cond_ent)) = self.conditions[i] {
                if let Some(KindComponent(EntityKind::GUARD)) = self.kinds[i] {
                    match self.infer_type(cond_ent) {
                        Ok(ty) => {
                            // Persist inferred type
                            self.types[cond_ent.0 as usize] =
                                Some(TypeComponent(crate::ast::types::ExtendedType::new(
                                    ty.clone(),
                                    Default::default(),
                                )));
                            if ty != SignalType::Bool {
                                let name =
                                    self.names[i].as_ref().map(|n| n.0.clone()).unwrap_or_default();
                                errors.push(MirrError::TypeError {
                                    message: format!(
                                        "{} Guard '{}' condition must be bool, got {}.",
                                        crate::error_codes::ec(601),
                                        name,
                                        ty
                                    ),
                                    span: self.spans[i].as_ref().map(|s| s.0),
                                });
                            }
                        }
                        Err(e) => errors.push(e),
                    }
                }
            }
        }

        // T1: Assignment type compatibility.
        for i in 0..max_id {
            if let Some(AssignmentComponent { target, value }) = self.assignment_comps[i] {
                let target_ty = match &self.types[target.0 as usize] {
                    Some(TypeComponent(et)) => et.signal_type(),
                    None => continue,
                };
                match self.infer_type(value) {
                    Ok(expr_ty) => {
                        // Persist inferred type
                        self.types[value.0 as usize] =
                            Some(TypeComponent(crate::ast::types::ExtendedType::new(
                                expr_ty.clone(),
                                Default::default(),
                            )));
                        if !self.types_compatible(&target_ty, &expr_ty) {
                            let target_name = self.names[target.0 as usize]
                                .as_ref()
                                .map(|n| n.0.clone())
                                .unwrap_or_default();
                            errors.push(MirrError::TypeError {
                                message: format!("{} Assignment to '{}' ({}): expression type {} is not compatible.", crate::error_codes::ec(602),
                                    target_name, target_ty, expr_ty
                                ),
                                span: self.spans[i].as_ref().map(|s| s.0),
                            });
                        }
                    }
                    Err(e) => errors.push(e),
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn infer_type(&self, root_ent: EntityId) -> Result<SignalType, MirrError> {
        #[derive(Debug)]
        enum Work {
            Visit(EntityId),
            CombineBinary(BinaryOp),
            CombineUnary(UnaryOp),
            CombineArrayIndex,
            CombineFieldAccess(String),
            CombineArrayLiteral(usize),
            CombineStructLiteral { name: String, field_names: Vec<String> },
        }

        let mut stack = vec![Work::Visit(root_ent)];
        let mut results = Vec::new();

        while let Some(work) = stack.pop() {
            match work {
                Work::Visit(ent) => {
                    let idx = ent.0 as usize;
                    if let Some(LiteralComponent(lit)) = &self.literals[idx] {
                        results.push(match lit {
                            crate::ast::types::LiteralValue::Bool(_) => SignalType::Bool,
                            crate::ast::types::LiteralValue::Integer(v) => {
                                SignalType::Unsigned(crate::width::types::Width::min_bits_for(*v).0)
                            }
                        });
                    } else if let Some(SignalRefComponent(sig_ent)) = self.signal_refs[idx] {
                        let ty = self.types[sig_ent.0 as usize]
                            .as_ref()
                            .map(|t| t.0.signal_type())
                            .ok_or_else(|| MirrError::TypeError {
                                message: format!(
                                    "{} Signal entity {} has no type component.",
                                    crate::error_codes::ec(699),
                                    sig_ent.0
                                ),
                                span: None,
                             })?;
                        results.push(ty);
                    } else if let Some(BinaryComponent { op, left, right }) = self.binary_ops[idx] {
                        stack.push(Work::CombineBinary(op));
                        stack.push(Work::Visit(right));
                        stack.push(Work::Visit(left));
                    } else if let Some(UnaryComponent { op, operand }) = self.unary_ops[idx] {
                        stack.push(Work::CombineUnary(op));
                        stack.push(Work::Visit(operand));
                    } else if let Some(ArrayIndexComponent { array, index }) =
                        self.array_indices[idx]
                    {
                        stack.push(Work::CombineArrayIndex);
                        stack.push(Work::Visit(index));
                        stack.push(Work::Visit(array));
                    } else if let Some(FieldAccessComponent { object, field }) =
                        &self.field_accesses[idx]
                    {
                        stack.push(Work::CombineFieldAccess(field.clone()));
                        stack.push(Work::Visit(*object));
                    } else if let Some(PrevComponent { signal, .. }) = self.prev_ops[idx] {
                        let ty = self.types[signal.0 as usize]
                            .as_ref()
                            .map(|t| t.0.signal_type())
                            .ok_or_else(|| MirrError::TypeError {
                                message: format!(
                                    "{} prev() target {} has no type.",
                                    crate::error_codes::ec(699),
                                    signal.0
                                ),
                                span: None,
                            })?;
                        results.push(ty);
                    } else if let Some(ArrayLiteralComponent(elems)) = &self.array_literals[idx] {
                        stack.push(Work::CombineArrayLiteral(elems.len()));
                        for elem in elems.iter().rev() {
                            stack.push(Work::Visit(*elem));
                        }
                    } else if let Some(StructLiteralComponent { name, fields }) = &self.struct_literals[idx] {
                        let field_names: Vec<String> = fields.iter().map(|(n, _)| n.clone()).collect();
                        stack.push(Work::CombineStructLiteral { name: name.clone(), field_names });
                        for (_, f_ent) in fields.iter().rev() {
                            stack.push(Work::Visit(*f_ent));
                        }
                    } else if let Some(UnfoldIndexComponent(_)) = &self.unfold_indices[idx] {
                        results.push(SignalType::Unsigned(32));
                    } else {
                        results.push(SignalType::Bool);
                    }
                }
                Work::CombineBinary(op) => {
                    let right_ty = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Type stack underflow (right)".to_string())
                    })?;
                    let left_ty = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Type stack underflow (left)".to_string())
                    })?;
                    results.push(match op {
                        BinaryOp::And | BinaryOp::Or => {
                            if left_ty == SignalType::Bool && right_ty == SignalType::Bool {
                                SignalType::Bool
                            } else {
                                let (w1, s1) = match left_ty {
                                    SignalType::Unsigned(w) => (w, false),
                                    SignalType::Signed(w) => (w, true),
                                    SignalType::Bool => (1, false),
                                    _ => (1, false),
                                };
                                let (w2, s2) = match right_ty {
                                    SignalType::Unsigned(w) => (w, false),
                                    SignalType::Signed(w) => (w, true),
                                    SignalType::Bool => (1, false),
                                    _ => (1, false),
                                };
                                if s1 != s2 {
                                    SignalType::Bool // Mixed signedness error
                                } else if s1 {
                                    SignalType::Signed(w1.max(w2))
                                } else {
                                    SignalType::Unsigned(w1.max(w2))
                                }
                            }
                        }
                        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                            let w1 = match left_ty {
                                SignalType::Unsigned(w) | SignalType::Signed(w) => w,
                                _ => 1,
                            };
                            let w2 = match right_ty {
                                SignalType::Unsigned(w) | SignalType::Signed(w) => w,
                                _ => 1,
                            };
                            if matches!(left_ty, SignalType::Signed(_)) {
                                SignalType::Signed(w1.max(w2))
                            } else {
                                SignalType::Unsigned(w1.max(w2))
                            }
                        }
                        BinaryOp::Shl | BinaryOp::Shr => {
                            let (w1, s1) = match left_ty {
                                SignalType::Unsigned(w) => (w, false),
                                SignalType::Signed(w) => (w, true),
                                SignalType::Bool => (1, false),
                                _ => (1, false),
                            };
                            if s1 {
                                SignalType::Signed(w1)
                            } else {
                                SignalType::Unsigned(w1)
                            }
                        }
                        _ => SignalType::Bool,
                    });
                }
                Work::CombineArrayIndex => {
                    let _index_ty = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Type stack underflow (index)".to_string())
                    })?;
                    let array_ty = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Type stack underflow (array)".to_string())
                    })?;
                    results.push(match array_ty {
                        SignalType::Array { element, .. } => element.as_ref().clone(),
                        SignalType::Unsigned(_) | SignalType::Signed(_) => SignalType::Bool,
                        _ => SignalType::Bool,
                    });
                }
                Work::CombineFieldAccess(field) => {
                    let object_ty = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Type stack underflow (object)".to_string())
                    })?;
                    results.push(match object_ty {
                        SignalType::Struct { fields, .. } => fields
                            .iter()
                            .find(|(n, _)| n == &field)
                            .map(|(_, t)| t.clone())
                            .unwrap_or(SignalType::Bool),
                        _ => SignalType::Bool,
                    });
                }
                Work::CombineUnary(op) => {
                    let op_ty = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Type stack underflow".to_string())
                    })?;
                    results.push(match op {
                        UnaryOp::Not => op_ty.clone(),
                        UnaryOp::Negate => match op_ty {
                            SignalType::Unsigned(w) => SignalType::Signed(w + 1),
                            SignalType::Signed(w) => SignalType::Signed(w),
                            _ => SignalType::Bool,
                        },
                    });
                }
                Work::CombineArrayLiteral(len) => {
                    if len == 0 {
                        results.push(SignalType::Array {
                            element: Box::new(SignalType::Unsigned(1)),
                            length: 0,
                        });
                    } else {
                        let mut elem_types = Vec::with_capacity(len);
                        for _ in 0..len {
                            let ty = results.pop().ok_or_else(|| {
                                MirrError::InternalError("Type stack underflow (array literal)".to_string())
                            })?;
                            elem_types.push(ty);
                        }
                        elem_types.reverse();

                        let mut element_ty = elem_types[0].clone();
                        for elem_ty in elem_types.iter().skip(1) {
                            if element_ty != *elem_ty {
                                if self.types_compatible(&element_ty, elem_ty) {
                                    // Target type is wider, keep it
                                } else if self.types_compatible(elem_ty, &element_ty) {
                                    element_ty = elem_ty.clone();
                                }
                            }
                        }

                        results.push(SignalType::Array {
                            element: Box::new(element_ty),
                            length: len as u64,
                        });
                    }
                }
                Work::CombineStructLiteral { name, field_names } => {
                    let len = field_names.len();
                    let mut field_types = Vec::with_capacity(len);
                    for _ in 0..len {
                        let ty = results.pop().ok_or_else(|| {
                            MirrError::InternalError("Type stack underflow (struct literal)".to_string())
                        })?;
                        field_types.push(ty);
                    }
                    field_types.reverse();

                    let mut fields = Vec::with_capacity(len);
                    for (f_name, f_ty) in field_names.into_iter().zip(field_types) {
                        fields.push((f_name, f_ty));
                    }
                    results.push(SignalType::Struct { name, fields });
                }
            }
        }

        results
            .pop()
            .ok_or_else(|| MirrError::InternalError("Empty type inference result".to_string()))
    }

    pub fn types_compatible(&self, target: &SignalType, expr: &SignalType) -> bool {
        if target == expr {
            return true;
        }
        match (target, expr) {
            (SignalType::Unsigned(tw), SignalType::Unsigned(ew)) => ew <= tw,
            (SignalType::Signed(tw), SignalType::Signed(ew)) => ew <= tw,
            (SignalType::Bool, SignalType::Unsigned(1))
            | (SignalType::Unsigned(1), SignalType::Bool) => true,
            _ => false,
        }
    }
}
