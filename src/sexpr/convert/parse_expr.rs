//! Expression and pattern-call parsing from S-expressions.

#![forbid(unsafe_code)]

use crate::ast::expr::Expr;
use crate::ast::pattern::{PatternArg, PatternCall, PatternOrigin};
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::error::MirrError;
use crate::sexpr::parser::sexpr_err;
use crate::sexpr::types::SExpr;

use super::from_sexpr::expect_head;
use super::MAX_CONVERT_DEPTH;

enum ParseWork<'a> {
    /// Parse an S-expression node into an Expr.
    Process(&'a SExpr),
    /// Compose a unary Expr from one result.
    BuildUnary(UnaryOp),
    /// Compose a binary Expr from two results.
    BuildBinary(BinaryOp),
    /// Compose an ArrayIndex Expr from two results (array, index).
    BuildArrayIndex,
    /// Compose a FieldAccess Expr from one result and a field name.
    BuildFieldAccess(String),
    /// Compose an ArrayLiteral from N results on the stack.
    BuildArrayLiteral(usize),
    /// Compose a StructLiteral from a name, field names, and N results on the stack.
    BuildStructLiteral { name: String, field_names: Vec<String> },
}

pub(super) fn parse_expr(sexpr: &SExpr) -> Result<Expr, MirrError> {
    const MAX_ITER: usize = MAX_CONVERT_DEPTH * 4;
    let mut work_stack: Vec<ParseWork<'_>> = Vec::with_capacity(MAX_CONVERT_DEPTH);
    let mut result_stack: Vec<Expr> = Vec::with_capacity(MAX_CONVERT_DEPTH);
    work_stack.push(ParseWork::Process(sexpr));

    let mut iterations: usize = 0;
    while let Some(work) = work_stack.pop() {
        iterations += 1;
        if iterations > MAX_ITER {
            return Err(sexpr_err(format!(
                "{} Expression nesting exceeds maximum depth",
                crate::error_codes::ec(808)
            )));
        }
        match work {
            ParseWork::Process(s) => match s {
                SExpr::Bool(b) => result_stack.push(Expr::Literal(LiteralValue::Bool(*b))),
                SExpr::Integer(n) => result_stack.push(Expr::Literal(LiteralValue::Integer(*n))),
                SExpr::List(items) if !items.is_empty() => {
                    let head = items[0].as_symbol().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} Expression head must be a symbol",
                            crate::error_codes::ec(805)
                        ))
                    })?;
                    match head {
                        "signal" => {
                            if items.len() < 2 {
                                return Err(sexpr_err(format!(
                                    "{} signal-ref requires name",
                                    crate::error_codes::ec(806)
                                )));
                            }
                            let name = items[1]
                                .as_str_val()
                                .ok_or_else(|| {
                                    sexpr_err(format!(
                                        "{} signal name must be string",
                                        crate::error_codes::ec(806)
                                    ))
                                })?
                                .to_string();
                            result_stack.push(Expr::Signal(name));
                        }
                        "prev" => {
                            if items.len() < 3 {
                                return Err(sexpr_err(format!(
                                    "{} prev requires signal and delay",
                                    crate::error_codes::ec(806)
                                )));
                            }
                            let signal = items[1]
                                .as_str_val()
                                .ok_or_else(|| {
                                    sexpr_err(format!(
                                        "{} prev signal must be string",
                                        crate::error_codes::ec(806)
                                    ))
                                })?
                                .to_string();
                            let delay = items[2].as_integer().ok_or_else(|| {
                                sexpr_err(format!(
                                    "{} prev delay must be integer",
                                    crate::error_codes::ec(806)
                                ))
                            })?;
                            result_stack.push(Expr::Prev { signal, delay });
                        }
                        "not" => {
                            if items.len() < 2 {
                                return Err(sexpr_err(format!(
                                    "{} not requires operand",
                                    crate::error_codes::ec(806)
                                )));
                            }
                            work_stack.push(ParseWork::BuildUnary(UnaryOp::Not));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "negate" => {
                            if items.len() < 2 {
                                return Err(sexpr_err(format!(
                                    "{} negate requires operand",
                                    crate::error_codes::ec(806)
                                )));
                            }
                            work_stack.push(ParseWork::BuildUnary(UnaryOp::Negate));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "aref" => {
                            if items.len() != 3 {
                                return Err(sexpr_err(format!(
                                    "{} aref requires array and index",
                                    crate::error_codes::ec(806)
                                )));
                            }
                            work_stack.push(ParseWork::BuildArrayIndex);
                            work_stack.push(ParseWork::Process(&items[2]));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "field-access" => {
                            if items.len() != 3 {
                                return Err(sexpr_err(format!(
                                    "{} field-access requires object and field",
                                    crate::error_codes::ec(806)
                                )));
                            }
                            let field = items[2]
                                .as_str_val()
                                .ok_or_else(|| {
                                    sexpr_err(format!(
                                        "{} field-access field must be string",
                                        crate::error_codes::ec(806)
                                    ))
                                })?
                                .to_string();
                            work_stack.push(ParseWork::BuildFieldAccess(field));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "array-literal" => {
                            let count = items.len().saturating_sub(1).min(512);
                            work_stack.push(ParseWork::BuildArrayLiteral(count));
                            for item in items[1..].iter().take(512).rev() {
                                work_stack.push(ParseWork::Process(item));
                            }
                        }
                        "struct-literal" => {
                            if items.len() < 2 {
                                return Err(sexpr_err(format!(
                                    "{} struct-literal requires a name",
                                    crate::error_codes::ec(806)
                                )));
                            }
                            let name = items[1]
                                .as_str_val()
                                .ok_or_else(|| {
                                    sexpr_err(format!(
                                        "{} struct-literal name must be string",
                                        crate::error_codes::ec(806)
                                    ))
                                })?
                                .to_string();
                            let mut field_names = Vec::new();
                            let field_items = &items[2..];
                            let bounded = field_items.iter().take(32);
                            for item in bounded {
                                let pair = item.as_list().ok_or_else(|| {
                                    sexpr_err(format!(
                                        "{} struct-literal field must be a list",
                                        crate::error_codes::ec(806)
                                    ))
                                })?;
                                if pair.len() != 2 {
                                    return Err(sexpr_err(format!(
                                        "{} struct-literal field requires name and value",
                                        crate::error_codes::ec(806)
                                    )));
                                }
                                let fname = pair[0]
                                    .as_str_val()
                                    .ok_or_else(|| {
                                        sexpr_err(format!(
                                            "{} struct-literal field name must be string",
                                            crate::error_codes::ec(806)
                                        ))
                                    })?
                                    .to_string();
                                field_names.push(fname);
                            }
                            let count = field_names.len();
                            work_stack.push(ParseWork::BuildStructLiteral { name, field_names });
                            for item in field_items.iter().take(32).rev() {
                                let pair = item.as_list().ok_or_else(|| {
                                    sexpr_err(format!(
                                        "{} struct-literal field must be a list",
                                        crate::error_codes::ec(806)
                                    ))
                                })?;
                                if pair.len() == 2 {
                                    work_stack.push(ParseWork::Process(&pair[1]));
                                }
                            }
                            let _ = count;
                        }
                        _ => {
                            // Binary operator
                            if items.len() < 3 {
                                return Err(sexpr_err(format!(
                                    "{} Unknown or incomplete expression form: {head}",
                                    crate::error_codes::ec(805)
                                )));
                            }
                            let op = symbol_to_binop(head)?;
                            work_stack.push(ParseWork::BuildBinary(op));
                            work_stack.push(ParseWork::Process(&items[2]));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                    }
                }
                _ => {
                    return Err(sexpr_err(format!(
                        "{} Invalid expression S-expression",
                        crate::error_codes::ec(805)
                    )))
                }
            },
            ParseWork::BuildUnary(op) => {
                let operand = result_stack.pop().ok_or_else(|| {
                    sexpr_err(format!(
                        "{} Missing operand in expression stack",
                        crate::error_codes::ec(808)
                    ))
                })?;
                result_stack.push(Expr::Unary { op, operand: Box::new(operand) });
            }
            ParseWork::BuildBinary(op) => {
                let right = result_stack.pop().ok_or_else(|| {
                    sexpr_err(format!(
                        "{} Missing right operand in expression stack",
                        crate::error_codes::ec(808)
                    ))
                })?;
                let left = result_stack.pop().ok_or_else(|| {
                    sexpr_err(format!(
                        "{} Missing left operand in expression stack",
                        crate::error_codes::ec(808)
                    ))
                })?;
                result_stack.push(Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
            ParseWork::BuildArrayIndex => {
                let index = result_stack.pop().ok_or_else(|| {
                    sexpr_err(format!(
                        "{} Missing index in expression stack",
                        crate::error_codes::ec(808)
                    ))
                })?;
                let array = result_stack.pop().ok_or_else(|| {
                    sexpr_err(format!(
                        "{} Missing array in expression stack",
                        crate::error_codes::ec(808)
                    ))
                })?;
                result_stack
                    .push(Expr::ArrayIndex { array: Box::new(array), index: Box::new(index) });
            }
            ParseWork::BuildFieldAccess(field) => {
                let object = result_stack.pop().ok_or_else(|| {
                    sexpr_err(format!(
                        "{} Missing object in expression stack",
                        crate::error_codes::ec(808)
                    ))
                })?;
                result_stack.push(Expr::FieldAccess { object: Box::new(object), field });
            }
            ParseWork::BuildArrayLiteral(count) => {
                let mut elems = Vec::with_capacity(count);
                for _ in 0..count {
                    let elem = result_stack.pop().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} Missing element in expression stack",
                            crate::error_codes::ec(808)
                        ))
                    })?;
                    elems.push(elem);
                }
                elems.reverse();
                result_stack.push(Expr::ArrayLiteral(elems));
            }
            ParseWork::BuildStructLiteral { name, field_names } => {
                let count = field_names.len();
                let mut values = Vec::with_capacity(count);
                for _ in 0..count {
                    let val = result_stack.pop().ok_or_else(|| {
                        sexpr_err(format!(
                            "{} Missing struct field value in expression stack",
                            crate::error_codes::ec(808)
                        ))
                    })?;
                    values.push(val);
                }
                values.reverse();
                let fields: Vec<(String, Expr)> = field_names.into_iter().zip(values).collect();
                result_stack.push(Expr::StructLiteral { name, fields });
            }
        }
    }

    result_stack.pop().ok_or_else(|| {
        sexpr_err(format!("{} Empty expression result", crate::error_codes::ec(808)))
    })
}

fn symbol_to_binop(sym: &str) -> Result<BinaryOp, MirrError> {
    match sym {
        "and" => Ok(BinaryOp::And),
        "or" => Ok(BinaryOp::Or),
        "bitor" => Ok(BinaryOp::BitwiseOr),
        "bitand" => Ok(BinaryOp::BitwiseAnd),
        "xor" => Ok(BinaryOp::Xor),
        "<" => Ok(BinaryOp::Lt),
        "<=" => Ok(BinaryOp::Le),
        ">" => Ok(BinaryOp::Gt),
        ">=" => Ok(BinaryOp::Ge),
        "==" => Ok(BinaryOp::Eq),
        "!=" => Ok(BinaryOp::Ne),
        "+" => Ok(BinaryOp::Add),
        "-" => Ok(BinaryOp::Sub),
        "*" => Ok(BinaryOp::Mul),
        "<<" => Ok(BinaryOp::Shl),
        ">>" => Ok(BinaryOp::Shr),
        other => Err(sexpr_err(format!(
            "{} Unknown binary operator: {other}",
            crate::error_codes::ec(805)
        ))),
    }
}

pub(super) fn parse_pattern_call(sexpr: &SExpr) -> Result<PatternCall, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected pattern-call list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "pattern-call")?;
    if items.len() < 2 {
        return Err(sexpr_err(format!(
            "{} pattern-call requires name",
            crate::error_codes::ec(806)
        )));
    }
    let pattern_name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!("{} pattern-call name must be a string", crate::error_codes::ec(806)))
        })?
        .to_string();
    let mut args = Vec::new();
    for item in &items[2..] {
        args.push(parse_pattern_arg(item)?);
    }
    Ok(PatternCall { pattern_name, arguments: args, span: None })
}

pub(super) fn parse_pattern_arg(sexpr: &SExpr) -> Result<PatternArg, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected pattern-arg list", crate::error_codes::ec(805)))
    })?;
    if items.is_empty() {
        return Err(sexpr_err(format!("{} Empty pattern-arg list", crate::error_codes::ec(806))));
    }
    match items[0].as_symbol() {
        Some("signal-ref") => {
            let name = items
                .get(1)
                .and_then(|s| s.as_str_val())
                .ok_or_else(|| {
                    sexpr_err(format!("{} signal-ref requires name", crate::error_codes::ec(806)))
                })?
                .to_string();
            Ok(PatternArg::SignalRef(name))
        }
        Some("const-int") => {
            let n = items.get(1).and_then(|s| s.as_integer()).ok_or_else(|| {
                sexpr_err(format!("{} const-int requires value", crate::error_codes::ec(806)))
            })?;
            Ok(PatternArg::ConstInt(n))
        }
        Some("const-bool") => {
            let b = items.get(1).and_then(|s| s.as_bool()).ok_or_else(|| {
                sexpr_err(format!("{} const-bool requires value", crate::error_codes::ec(806)))
            })?;
            Ok(PatternArg::ConstBool(b))
        }
        Some("pattern-ref") => {
            let name = items
                .get(1)
                .and_then(|s| s.as_str_val())
                .ok_or_else(|| {
                    sexpr_err(format!("{} pattern-ref requires name", crate::error_codes::ec(806)))
                })?
                .to_string();
            Ok(PatternArg::PatternRef(name))
        }
        Some(other) => Err(sexpr_err(format!(
            "{} Unknown pattern arg kind: {other}",
            crate::error_codes::ec(806)
        ))),
        None => Err(sexpr_err(format!(
            "{} Pattern arg head must be a symbol",
            crate::error_codes::ec(806)
        ))),
    }
}

pub(super) fn parse_pattern_origin(sexpr: &SExpr) -> Result<PatternOrigin, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| {
        sexpr_err(format!("{} Expected pattern-origin list", crate::error_codes::ec(805)))
    })?;
    expect_head(items, "pattern-origin")?;
    if items.len() < 3 {
        return Err(sexpr_err(format!(
            "{} pattern-origin requires name and summary",
            crate::error_codes::ec(806)
        )));
    }
    let pattern_name = items[1]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!(
                "{} pattern-origin name must be a string",
                crate::error_codes::ec(806)
            ))
        })?
        .to_string();
    let summary = items[2]
        .as_str_val()
        .ok_or_else(|| {
            sexpr_err(format!(
                "{} pattern-origin summary must be a string",
                crate::error_codes::ec(806)
            ))
        })?
        .to_string();
    Ok(PatternOrigin { pattern_name, call_args_summary: summary })
}
