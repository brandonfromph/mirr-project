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
            return Err(sexpr_err("[E808] Expression nesting exceeds maximum depth"));
        }
        match work {
            ParseWork::Process(s) => match s {
                SExpr::Bool(b) => result_stack.push(Expr::Literal(LiteralValue::Bool(*b))),
                SExpr::Integer(n) => result_stack.push(Expr::Literal(LiteralValue::Integer(*n))),
                SExpr::List(items) if !items.is_empty() => {
                    let head = items[0]
                        .as_symbol()
                        .ok_or_else(|| sexpr_err("[E805] Expression head must be a symbol"))?;
                    match head {
                        "signal" => {
                            if items.len() < 2 {
                                return Err(sexpr_err("[E806] signal-ref requires name"));
                            }
                            let name = items[1]
                                .as_str_val()
                                .ok_or_else(|| sexpr_err("[E806] signal name must be string"))?
                                .to_string();
                            result_stack.push(Expr::Signal(name));
                        }
                        "prev" => {
                            if items.len() < 3 {
                                return Err(sexpr_err("[E806] prev requires signal and delay"));
                            }
                            let signal = items[1]
                                .as_str_val()
                                .ok_or_else(|| sexpr_err("[E806] prev signal must be string"))?
                                .to_string();
                            let delay = items[2]
                                .as_integer()
                                .ok_or_else(|| sexpr_err("[E806] prev delay must be integer"))?;
                            result_stack.push(Expr::Prev { signal, delay });
                        }
                        "not" => {
                            if items.len() < 2 {
                                return Err(sexpr_err("[E806] not requires operand"));
                            }
                            work_stack.push(ParseWork::BuildUnary(UnaryOp::Not));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "negate" => {
                            if items.len() < 2 {
                                return Err(sexpr_err("[E806] negate requires operand"));
                            }
                            work_stack.push(ParseWork::BuildUnary(UnaryOp::Negate));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "aref" => {
                            if items.len() != 3 {
                                return Err(sexpr_err("[E806] aref requires array and index"));
                            }
                            work_stack.push(ParseWork::BuildArrayIndex);
                            work_stack.push(ParseWork::Process(&items[2]));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                        "field-access" => {
                            if items.len() != 3 {
                                return Err(sexpr_err(
                                    "[E806] field-access requires object and field",
                                ));
                            }
                            let field = items[2]
                                .as_str_val()
                                .ok_or_else(|| {
                                    sexpr_err("[E806] field-access field must be string")
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
                                return Err(sexpr_err("[E806] struct-literal requires a name"));
                            }
                            let name = items[1]
                                .as_str_val()
                                .ok_or_else(|| {
                                    sexpr_err("[E806] struct-literal name must be string")
                                })?
                                .to_string();
                            let mut field_names = Vec::new();
                            let field_items = &items[2..];
                            let bounded = field_items.iter().take(32);
                            for item in bounded {
                                let pair = item.as_list().ok_or_else(|| {
                                    sexpr_err("[E806] struct-literal field must be a list")
                                })?;
                                if pair.len() != 2 {
                                    return Err(sexpr_err(
                                        "[E806] struct-literal field requires name and value",
                                    ));
                                }
                                let fname = pair[0]
                                    .as_str_val()
                                    .ok_or_else(|| {
                                        sexpr_err("[E806] struct-literal field name must be string")
                                    })?
                                    .to_string();
                                field_names.push(fname);
                            }
                            let count = field_names.len();
                            work_stack.push(ParseWork::BuildStructLiteral { name, field_names });
                            for item in field_items.iter().take(32).rev() {
                                let pair = item.as_list().ok_or_else(|| {
                                    sexpr_err("[E806] struct-literal field must be a list")
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
                                    "[E805] Unknown or incomplete expression form: {head}"
                                )));
                            }
                            let op = symbol_to_binop(head)?;
                            work_stack.push(ParseWork::BuildBinary(op));
                            work_stack.push(ParseWork::Process(&items[2]));
                            work_stack.push(ParseWork::Process(&items[1]));
                        }
                    }
                }
                _ => return Err(sexpr_err("[E805] Invalid expression S-expression")),
            },
            ParseWork::BuildUnary(op) => {
                let operand = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing operand in expression stack"))?;
                result_stack.push(Expr::Unary { op, operand: Box::new(operand) });
            }
            ParseWork::BuildBinary(op) => {
                let right = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing right operand in expression stack"))?;
                let left = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing left operand in expression stack"))?;
                result_stack.push(Expr::Binary {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
            ParseWork::BuildArrayIndex => {
                let index = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing index in expression stack"))?;
                let array = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing array in expression stack"))?;
                result_stack
                    .push(Expr::ArrayIndex { array: Box::new(array), index: Box::new(index) });
            }
            ParseWork::BuildFieldAccess(field) => {
                let object = result_stack
                    .pop()
                    .ok_or_else(|| sexpr_err("[E808] Missing object in expression stack"))?;
                result_stack.push(Expr::FieldAccess { object: Box::new(object), field });
            }
            ParseWork::BuildArrayLiteral(count) => {
                let mut elems = Vec::with_capacity(count);
                for _ in 0..count {
                    let elem = result_stack
                        .pop()
                        .ok_or_else(|| sexpr_err("[E808] Missing element in expression stack"))?;
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
                        sexpr_err("[E808] Missing struct field value in expression stack")
                    })?;
                    values.push(val);
                }
                values.reverse();
                let fields: Vec<(String, Expr)> = field_names.into_iter().zip(values).collect();
                result_stack.push(Expr::StructLiteral { name, fields });
            }
        }
    }

    result_stack.pop().ok_or_else(|| sexpr_err("[E808] Empty expression result"))
}

fn symbol_to_binop(sym: &str) -> Result<BinaryOp, MirrError> {
    match sym {
        "and" => Ok(BinaryOp::And),
        "or" => Ok(BinaryOp::Or),
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
        other => Err(sexpr_err(format!("[E805] Unknown binary operator: {other}"))),
    }
}

pub(super) fn parse_pattern_call(sexpr: &SExpr) -> Result<PatternCall, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected pattern-call list"))?;
    expect_head(items, "pattern-call")?;
    if items.len() < 2 {
        return Err(sexpr_err("[E806] pattern-call requires name"));
    }
    let pattern_name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] pattern-call name must be a string"))?
        .to_string();
    let mut args = Vec::new();
    for item in &items[2..] {
        args.push(parse_pattern_arg(item)?);
    }
    Ok(PatternCall { pattern_name, arguments: args, span: None })
}

pub(super) fn parse_pattern_arg(sexpr: &SExpr) -> Result<PatternArg, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected pattern-arg list"))?;
    if items.is_empty() {
        return Err(sexpr_err("[E806] Empty pattern-arg list"));
    }
    match items[0].as_symbol() {
        Some("signal-ref") => {
            let name = items
                .get(1)
                .and_then(|s| s.as_str_val())
                .ok_or_else(|| sexpr_err("[E806] signal-ref requires name"))?
                .to_string();
            Ok(PatternArg::SignalRef(name))
        }
        Some("const-int") => {
            let n = items
                .get(1)
                .and_then(|s| s.as_integer())
                .ok_or_else(|| sexpr_err("[E806] const-int requires value"))?;
            Ok(PatternArg::ConstInt(n))
        }
        Some("const-bool") => {
            let b = items
                .get(1)
                .and_then(|s| s.as_bool())
                .ok_or_else(|| sexpr_err("[E806] const-bool requires value"))?;
            Ok(PatternArg::ConstBool(b))
        }
        Some("pattern-ref") => {
            let name = items
                .get(1)
                .and_then(|s| s.as_str_val())
                .ok_or_else(|| sexpr_err("[E806] pattern-ref requires name"))?
                .to_string();
            Ok(PatternArg::PatternRef(name))
        }
        Some(other) => Err(sexpr_err(format!("[E806] Unknown pattern arg kind: {other}"))),
        None => Err(sexpr_err("[E806] Pattern arg head must be a symbol")),
    }
}

pub(super) fn parse_pattern_origin(sexpr: &SExpr) -> Result<PatternOrigin, MirrError> {
    let items = sexpr.as_list().ok_or_else(|| sexpr_err("[E805] Expected pattern-origin list"))?;
    expect_head(items, "pattern-origin")?;
    if items.len() < 3 {
        return Err(sexpr_err("[E806] pattern-origin requires name and summary"));
    }
    let pattern_name = items[1]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] pattern-origin name must be a string"))?
        .to_string();
    let summary = items[2]
        .as_str_val()
        .ok_or_else(|| sexpr_err("[E806] pattern-origin summary must be a string"))?
        .to_string();
    Ok(PatternOrigin { pattern_name, call_args_summary: summary })
}
