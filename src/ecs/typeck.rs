#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, SignalType, UnaryOp};
use crate::ecs::components::*;
use crate::ecs::Registry;
use crate::error::{MirrError, PipelineErrors};

impl Registry {
    /// Perform type checking on the entire registry.
    /// Operates directly on ECS components.
    pub fn typecheck(&mut self, bootstrap_mode: bool) -> Result<(), PipelineErrors> {
        let mut errors = PipelineErrors::new();
        let max_id = self.next_id as usize;

        // T14: Guard conditions must be Bool.
        for i in 0..max_id {
            if let Some(ConditionComponent(cond_ent)) = self.conditions[i] {
                if let Some(KindComponent(EntityKind::GUARD)) = self.kinds[i] {
                    let context_span = self.spans[i].as_ref().map(|s| s.0);
                    match self.infer_type(cond_ent, bootstrap_mode, context_span) {
                        Ok(ty) => {
                            // Persist inferred type
                            self.types[cond_ent.0 as usize] =
                                Some(TypeComponent(crate::ast::types::ExtendedType::new(
                                    ty.clone(),
                                    Default::default(),
                                )));
                            if ty != SignalType::Bool {
                                let name = self.names[i]
                                    .map(|nc| self.resolve_name(nc.0).to_string())
                                    .unwrap_or_default();
                                errors.push(MirrError::TypeError {
                                    message: format!(
                                        "{} Guard '{}' condition must be bool, got {}.",
                                        crate::error_codes::ec(601),
                                        name,
                                        ty
                                    ),
                                    span: context_span,
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
            if let Some(AssignmentComponent { target, value, target_index }) =
                self.assignment_comps[i]
            {
                let mut target_ty = match &self.types[target.0 as usize] {
                    Some(TypeComponent(et)) => et.signal_type(),
                    None => continue,
                };
                if target_index.is_some() {
                    target_ty = crate::ast::types::SignalType::Bool;
                }
                let context_span = self.spans[i].as_ref().map(|s| s.0);
                match self.infer_type(value, bootstrap_mode, context_span) {
                    Ok(expr_ty) => {
                        // Persist inferred type
                        self.types[value.0 as usize] =
                            Some(TypeComponent(crate::ast::types::ExtendedType::new(
                                expr_ty.clone(),
                                Default::default(),
                            )));
                        if !self.types_compatible(&target_ty, &expr_ty) {
                            let target_name = self.names[target.0 as usize]
                                .map(|nc| self.resolve_name(nc.0).to_string())
                                .unwrap_or_default();
                            errors.push(MirrError::TypeError {
                                message: format!("{} Assignment to '{}' ({}): expression type {} is not compatible.", crate::error_codes::ec(601),
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

    pub fn infer_type(
        &self,
        root_ent: EntityId,
        bootstrap_mode: bool,
        context_span: Option<crate::span::Span>,
    ) -> Result<SignalType, MirrError> {
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
                            .or_else(|| {
                                if let Some(KindComponent(EntityKind::GUARD)) =
                                    self.kinds[sig_ent.0 as usize]
                                {
                                    Some(SignalType::Bool)
                                } else {
                                    None
                                }
                            })
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
                        stack.push(Work::Visit(signal));
                    } else if let Some(ArrayLiteralComponent(elems)) = &self.array_literals[idx] {
                        stack.push(Work::CombineArrayLiteral(elems.len()));
                        for elem in elems.iter().rev() {
                            stack.push(Work::Visit(*elem));
                        }
                    } else if let Some(StructLiteralComponent { name, fields }) =
                        &self.struct_literals[idx]
                    {
                        let field_names: Vec<String> =
                            fields.iter().map(|(n, _)| n.clone()).collect();
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

                    let require_numeric =
                        |ty: &SignalType, op_sym: &str| -> Result<(u32, bool), MirrError> {
                            match ty {
                                SignalType::Unsigned(w) => Ok((*w, false)),
                                SignalType::Signed(w) => Ok((*w, true)),
                                SignalType::Bool => {
                                    if bootstrap_mode {
                                        Ok((1, false))
                                    } else {
                                        Err(MirrError::TypeError {
                                            message: format!(
                                                "{} Operator '{}' requires numeric operands.",
                                                crate::error_codes::ec(603),
                                                op_sym
                                            ),
                                            span: context_span,
                                        })
                                    }
                                }
                                _ => Err(MirrError::TypeError {
                                    message: format!(
                                        "{} Operator '{}' cannot be applied to composite type.",
                                        crate::error_codes::ec(607),
                                        op_sym
                                    ),
                                    span: context_span,
                                }),
                            }
                        };

                    let check_mixed_sign = |left_signed: bool,
                                            right_signed: bool,
                                            op_sym: &str|
                     -> Result<(), MirrError> {
                        if left_signed != right_signed {
                            Err(MirrError::TypeError {
                                message: format!(
                                    "{} Operator '{}' cannot mix signed and unsigned operands.",
                                    crate::error_codes::ec(608),
                                    op_sym
                                ),
                                span: context_span,
                            })
                        } else {
                            Ok(())
                        }
                    };

                    results.push(match op {
                        BinaryOp::And | BinaryOp::Or => {
                            if left_ty == SignalType::Bool && right_ty == SignalType::Bool {
                                SignalType::Bool
                            } else if bootstrap_mode {
                                let op_sym = if op == BinaryOp::And { "&&" } else { "||" };
                                let (w1, s1) = match left_ty {
                                    SignalType::Bool => (1, false),
                                    _ => require_numeric(&left_ty, op_sym)?,
                                };
                                let (w2, s2) = match right_ty {
                                    SignalType::Bool => (1, false),
                                    _ => require_numeric(&right_ty, op_sym)?,
                                };
                                check_mixed_sign(s1, s2, op_sym)?;
                                let max_w = w1.max(w2);
                                if s1 {
                                    SignalType::Signed(max_w)
                                } else {
                                    SignalType::Unsigned(max_w)
                                }
                            } else {
                                let op_sym = if op == BinaryOp::And { "&&" } else { "||" };
                                return Err(MirrError::TypeError {
                                    message: format!(
                                        "{} Logical operator '{}' requires bool operands, got {} and {}.",
                                        crate::error_codes::ec(604),
                                        op_sym,
                                        left_ty,
                                        right_ty
                                    ),
                                    span: context_span,
                                });
                            }
                        }
                        BinaryOp::BitwiseAnd | BinaryOp::BitwiseOr => {
                            let op_sym = if op == BinaryOp::BitwiseAnd { "&" } else { "|" };
                            let (w1, s1) = match left_ty {
                                SignalType::Bool => (1, false),
                                _ => require_numeric(&left_ty, op_sym)?,
                            };
                            let (w2, s2) = match right_ty {
                                SignalType::Bool => (1, false),
                                _ => require_numeric(&right_ty, op_sym)?,
                            };
                            check_mixed_sign(s1, s2, op_sym)?;
                            let max_w = w1.max(w2);
                            if s1 {
                                SignalType::Signed(max_w)
                            } else {
                                SignalType::Unsigned(max_w)
                            }
                        }
                        BinaryOp::Xor => {
                            if left_ty.is_composite() || right_ty.is_composite() {
                                return Err(MirrError::TypeError {
                                    message: format!(
                                        "{} Operator '^' cannot be applied to composite type '{}' and '{}'.",
                                        crate::error_codes::ec(226),
                                        left_ty,
                                        right_ty
                                    ),
                                    span: context_span,
                                });
                            }
                            if left_ty != right_ty {
                                let compatible = match (&left_ty, &right_ty) {
                                    (SignalType::Bool, SignalType::Unsigned(1)) |
                                    (SignalType::Unsigned(1), SignalType::Bool) => true,
                                    (SignalType::Unsigned(w1), SignalType::Unsigned(w2)) => w1 == w2,
                                    (SignalType::Signed(w1), SignalType::Signed(w2)) => w1 == w2,
                                    _ => false,
                                };
                                if !compatible {
                                    return Err(MirrError::TypeError {
                                        message: format!(
                                            "{} Operator '^' (xor) requires matching types, got {} and {}.",
                                            crate::error_codes::ec(607),
                                            left_ty,
                                            right_ty
                                        ),
                                        span: context_span,
                                    });
                                }
                            }
                            left_ty.clone()
                        }
                        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul => {
                            let op_sym = match op {
                                BinaryOp::Add => "+",
                                BinaryOp::Sub => "-",
                                _ => "*",
                            };
                            let (w1, s1) = require_numeric(&left_ty, op_sym)?;
                            let (w2, s2) = require_numeric(&right_ty, op_sym)?;
                            check_mixed_sign(s1, s2, op_sym)?;
                            if s1 {
                                SignalType::Signed(w1.max(w2))
                            } else {
                                SignalType::Unsigned(w1.max(w2))
                            }
                        }
                        BinaryOp::Eq | BinaryOp::Ne => {
                            if matches!(left_ty, SignalType::Bool) || matches!(right_ty, SignalType::Bool) {
                                if left_ty != right_ty {
                                    return Err(MirrError::TypeError {
                                        message: format!("{} Cross-category equality comparison.", crate::error_codes::ec(606)),
                                        span: None,
                                    });
                                }
                            } else {
                                let (_, s1) = require_numeric(&left_ty, "==")?;
                                let (_, s2) = require_numeric(&right_ty, "==")?;
                                check_mixed_sign(s1, s2, "==")?;
                            }
                            SignalType::Bool
                        }
                        BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
                            let op_sym = match op {
                                BinaryOp::Lt => "<",
                                BinaryOp::Le => "<=",
                                BinaryOp::Gt => ">",
                                _ => ">=",
                            };
                            if matches!(left_ty, SignalType::Bool) || matches!(right_ty, SignalType::Bool) {
                                return Err(MirrError::TypeError {
                                    message: format!("{} Ordering comparison on bool.", crate::error_codes::ec(605)),
                                    span: None,
                                });
                            } else {
                                let (_, s1) = require_numeric(&left_ty, op_sym)?;
                                let (_, s2) = require_numeric(&right_ty, op_sym)?;
                                check_mixed_sign(s1, s2, op_sym)?;
                            }
                            SignalType::Bool
                        }
                        BinaryOp::Shl | BinaryOp::Shr => {
                            let op_sym = if op == BinaryOp::Shl { "<<" } else { ">>" };
                            let (w1, s1) = match left_ty {
                                SignalType::Bool => (1, false),
                                _ => require_numeric(&left_ty, op_sym)?,
                            };
                            let (_w2, _s2) = match right_ty {
                                SignalType::Bool => (1, false),
                                _ => require_numeric(&right_ty, op_sym)?,
                            };
                            if s1 {
                                SignalType::Signed(w1)
                            } else {
                                SignalType::Unsigned(w1)
                            }
                        }
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
                        UnaryOp::ReductionOr => SignalType::Bool,
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
                                MirrError::InternalError(
                                    "Type stack underflow (array literal)".to_string(),
                                )
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
                            MirrError::InternalError(
                                "Type stack underflow (struct literal)".to_string(),
                            )
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
            (SignalType::Unsigned(tw), SignalType::Signed(ew)) if tw == ew => true,
            (SignalType::Signed(tw), SignalType::Unsigned(ew)) if tw == ew => true,
            (SignalType::Bool, SignalType::Unsigned(1))
            | (SignalType::Unsigned(1), SignalType::Bool) => true,
            _ => false,
        }
    }
}
