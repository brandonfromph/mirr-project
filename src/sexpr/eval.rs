//! Bounded eval/apply core for S-expressions.
//!
//! Compile-time only evaluator for macro bodies. NOT a general-purpose
//! Lisp interpreter. Deliberately omits lambda/closures (Turing-completeness
//! would violate NASA Power-of-10 termination guarantees).
//!
//! Bounds: MAX_EVAL_DEPTH (stack), MAX_EVAL_STEPS (fuel counter).

#![forbid(unsafe_code)]

use crate::error::MirrError;
use crate::sexpr::parser::sexpr_err;
use crate::sexpr::types::SExpr;
use crate::sexpr::{MAX_EVAL_DEPTH, MAX_EVAL_STEPS, MAX_SEXPR_NODES};

/// Evaluation state: explicit stack, step counter, environment.
pub struct EvalState {
    /// Step counter, decremented per eval. Hard error at 0.
    pub steps_remaining: usize,
    /// Current stack depth.
    pub depth: usize,
    /// Symbol bindings (flat environment, no closures).
    pub env: Vec<(String, SExpr)>,
}

impl EvalState {
    /// Create a fresh evaluation state with default limits.
    pub fn new() -> Self {
        Self { steps_remaining: MAX_EVAL_STEPS, depth: 0, env: Vec::new() }
    }

    /// Create with custom step limit.
    pub fn with_steps(steps: usize) -> Self {
        Self { steps_remaining: steps, depth: 0, env: Vec::new() }
    }
}

impl Default for EvalState {
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluate an S-expression in the given state.
///
/// Bounded by `MAX_EVAL_STEPS` and `MAX_EVAL_DEPTH`.
/// No recursion in the Rust implementation — uses explicit depth tracking.
pub fn eval(expr: &SExpr, state: &mut EvalState) -> Result<SExpr, MirrError> {
    state.steps_remaining = state
        .steps_remaining
        .checked_sub(1)
        .ok_or_else(|| sexpr_err("[E812] Evaluation steps exceed MAX_EVAL_STEPS"))?;

    if state.depth > MAX_EVAL_DEPTH {
        return Err(sexpr_err("[E811] Evaluation depth exceeds MAX_EVAL_DEPTH"));
    }

    match expr {
        // Atoms self-evaluate.
        SExpr::Integer(_) | SExpr::Bool(_) | SExpr::Str(_) => Ok(expr.clone()),

        // Symbols: look up in environment.
        SExpr::Symbol(name) => lookup(&state.env, name),

        // Quote: return unevaluated.
        SExpr::Quote(inner) => Ok((**inner).clone()),

        // Quasiquote: evaluate unquotes inside.
        SExpr::Quasiquote(inner) => {
            state.depth += 1;
            let result = eval_quasiquote(inner, state);
            state.depth -= 1;
            result
        }

        // Unquote outside quasiquote is an error.
        SExpr::Unquote(_) => Err(sexpr_err("[E805] Unquote outside quasiquote")),

        // List: dispatch on head form.
        SExpr::List(items) => {
            if items.is_empty() {
                return Ok(SExpr::list(vec![]));
            }
            state.depth += 1;
            let result = eval_list(items, state);
            state.depth -= 1;
            result
        }
    }
}

/// Evaluate a list form (dispatch on head symbol).
fn eval_list(items: &[SExpr], state: &mut EvalState) -> Result<SExpr, MirrError> {
    let head = match items[0].as_symbol() {
        Some(s) => s,
        None => {
            // Not a special form — evaluate all and return as list.
            let mut result = Vec::new();
            for item in items {
                result.push(eval(item, state)?);
            }
            return Ok(SExpr::list(result));
        }
    };

    match head {
        "quote" => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] quote requires one argument"));
            }
            Ok(items[1].clone())
        }

        "if" => {
            if items.len() < 4 {
                return Err(sexpr_err("[E806] if requires condition, then, else"));
            }
            let cond = eval(&items[1], state)?;
            let is_true = match &cond {
                SExpr::Bool(b) => *b,
                SExpr::Integer(n) => *n != 0,
                SExpr::List(l) => !l.is_empty(),
                _ => true,
            };
            if is_true {
                eval(&items[2], state)
            } else {
                eval(&items[3], state)
            }
        }

        "list" => {
            let mut result = Vec::new();
            for item in &items[1..] {
                result.push(eval(item, state)?);
            }
            Ok(SExpr::list(result))
        }

        "car" => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] car requires one argument"));
            }
            let val = eval(&items[1], state)?;
            match val {
                SExpr::List(ref l) if !l.is_empty() => Ok(l[0].clone()),
                _ => Err(sexpr_err("[E805] car requires a non-empty list")),
            }
        }

        "cdr" => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] cdr requires one argument"));
            }
            let val = eval(&items[1], state)?;
            match val {
                SExpr::List(ref l) if !l.is_empty() => Ok(SExpr::list(l[1..].to_vec())),
                _ => Err(sexpr_err("[E805] cdr requires a non-empty list")),
            }
        }

        "cons" => {
            if items.len() < 3 {
                return Err(sexpr_err("[E806] cons requires head and tail"));
            }
            let head_val = eval(&items[1], state)?;
            let tail_val = eval(&items[2], state)?;
            match tail_val {
                SExpr::List(mut l) => {
                    l.insert(0, head_val);
                    Ok(SExpr::list(l))
                }
                _ => Ok(SExpr::list(vec![head_val, tail_val])),
            }
        }

        "eq?" => {
            if items.len() < 3 {
                return Err(sexpr_err("[E806] eq? requires two arguments"));
            }
            let a = eval(&items[1], state)?;
            let b = eval(&items[2], state)?;
            Ok(SExpr::Bool(a == b))
        }

        "symbol?" => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] symbol? requires one argument"));
            }
            let val = eval(&items[1], state)?;
            Ok(SExpr::Bool(val.is_symbol()))
        }

        "list?" => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] list? requires one argument"));
            }
            let val = eval(&items[1], state)?;
            Ok(SExpr::Bool(val.is_list()))
        }

        "integer?" => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] integer? requires one argument"));
            }
            let val = eval(&items[1], state)?;
            Ok(SExpr::Bool(val.is_integer()))
        }

        "bool?" => {
            if items.len() < 2 {
                return Err(sexpr_err("[E806] bool? requires one argument"));
            }
            let val = eval(&items[1], state)?;
            Ok(SExpr::Bool(val.is_bool()))
        }

        "match-type" => {
            if items.len() < 3 {
                return Err(sexpr_err("[E806] match-type requires type and clauses"));
            }
            let type_val = eval(&items[1], state)?;
            eval_match_type(&type_val, &items[2..], state)
        }

        _ => {
            // Unknown form — look up head in env, if not found evaluate all.
            let mut result = Vec::new();
            for item in items {
                result.push(eval(item, state)?);
            }
            Ok(SExpr::list(result))
        }
    }
}

/// Evaluate match-type dispatch.
fn eval_match_type(
    type_val: &SExpr,
    clauses: &[SExpr],
    state: &mut EvalState,
) -> Result<SExpr, MirrError> {
    let mut iter_count = 0usize;
    for clause in clauses {
        iter_count += 1;
        if iter_count > MAX_SEXPR_NODES {
            return Err(sexpr_err("[E804] match-type exceeded iteration budget"));
        }
        let clause_items =
            clause.as_list().ok_or_else(|| sexpr_err("[E806] match-type clause must be a list"))?;
        if clause_items.len() < 2 {
            return Err(sexpr_err("[E806] match-type clause requires pattern and body"));
        }
        let pattern = &clause_items[0];
        let body = &clause_items[1];

        if match_type_pattern(type_val, pattern, state)? {
            return eval(body, state);
        }
    }
    Err(sexpr_err("[E805] No match-type clause matched"))
}

/// Match a type value against a pattern, binding variables.
fn match_type_pattern(
    type_val: &SExpr,
    pattern: &SExpr,
    state: &mut EvalState,
) -> Result<bool, MirrError> {
    match (type_val, pattern) {
        // Symbol match: exact match (e.g., bool vs bool)
        (SExpr::Symbol(a), SExpr::Symbol(b)) => Ok(a == b),
        // List pattern: match head, bind tail variables
        (SExpr::List(val_items), SExpr::List(pat_items)) => {
            if val_items.is_empty() || pat_items.is_empty() {
                return Ok(val_items.is_empty() && pat_items.is_empty());
            }
            // Head must match
            if val_items[0] != pat_items[0] {
                return Ok(false);
            }
            // Bind remaining pattern symbols to corresponding values
            for (i, pat_elem) in pat_items[1..].iter().enumerate() {
                if let Some(var_name) = pat_elem.as_symbol() {
                    if let Some(val) = val_items.get(i + 1) {
                        state.env.push((var_name.to_string(), val.clone()));
                    }
                }
            }
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// Evaluate quasiquote: process unquotes, leave rest as-is.
fn eval_quasiquote(expr: &SExpr, state: &mut EvalState) -> Result<SExpr, MirrError> {
    match expr {
        SExpr::Unquote(inner) => eval(inner, state),
        SExpr::List(items) => {
            let mut result = Vec::new();
            for item in items {
                result.push(eval_quasiquote(item, state)?);
            }
            Ok(SExpr::list(result))
        }
        _ => Ok(expr.clone()),
    }
}

/// Look up a symbol in the environment.
fn lookup(env: &[(String, SExpr)], name: &str) -> Result<SExpr, MirrError> {
    env.iter()
        .rev()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.clone())
        .ok_or_else(|| sexpr_err(format!("[E813] Undefined symbol: {name}")))
}
