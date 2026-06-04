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
/// Uses an explicit continuation stack — no recursion, per NASA Power-of-10.
pub fn eval(expr: &SExpr, state: &mut EvalState) -> Result<SExpr, MirrError> {
    /// What to do with any value produced by evaluating a sub-expression.
    enum Cont {
        /// (if _ then else) — condition just evaluated.
        IfCond { then_expr: SExpr, else_expr: SExpr },
        /// Collecting evaluated items for a list / generic form.
        Collect { done: Vec<SExpr>, remaining: Vec<SExpr> },
        /// car — extract head from evaluated arg.
        Car,
        /// cdr — extract tail from evaluated arg.
        Cdr,
        /// cons — head evaluated, now need to eval tail.
        ConsHead { tail_expr: SExpr },
        /// cons — tail evaluated, head already known.
        ConsTail { head_val: SExpr },
        /// eq? — first arg done, need to eval second.
        EqFirst { second_expr: SExpr },
        /// eq? — second arg done, first already known.
        EqSecond { first_val: SExpr },
        /// Type predicate (symbol?, list?, integer?, bool?).
        TypePred(TypePredKind),
        /// match-type — type value evaluated, now dispatch clauses.
        MatchType { clauses: Vec<SExpr> },
        /// match-type clause body evaluated — truncate env.
        MatchTypeRestore { env_depth: usize },
        /// Quasiquote: collecting items for a list within quasiquote.
        QQCollect { done: Vec<SExpr>, remaining: Vec<SExpr> },
    }

    enum TypePredKind {
        Symbol,
        List,
        Integer,
        Bool,
    }

    let mut stack: Vec<Cont> = Vec::new();
    let mut current = expr.clone();

    let max_iters = MAX_EVAL_STEPS;
    let mut iter_count = 0usize;

    'main: loop {
        iter_count += 1;
        if iter_count > max_iters {
            return Err(sexpr_err(format!(
                "{} Evaluation steps exceed MAX_EVAL_STEPS",
                crate::error_codes::ec(812)
            )));
        }
        if stack.len() + state.depth >= MAX_EVAL_DEPTH {
            return Err(sexpr_err(format!(
                "{} Evaluation depth exceeds MAX_EVAL_DEPTH",
                crate::error_codes::ec(811)
            )));
        }

        state.steps_remaining = state.steps_remaining.checked_sub(1).ok_or_else(|| {
            sexpr_err(format!(
                "{} Evaluation steps exceed MAX_EVAL_STEPS",
                crate::error_codes::ec(812)
            ))
        })?;

        // ── Evaluate `current` to produce a value ──────────────────
        let value = match &current {
            SExpr::Integer(_) | SExpr::Bool(_) | SExpr::Str(_) => current.clone(),
            SExpr::Symbol(name) => lookup(&state.env, name)?,
            SExpr::Quote(inner) => (**inner).clone(),
            SExpr::Unquote(_) => {
                return Err(sexpr_err(format!(
                    "{} Unquote outside quasiquote",
                    crate::error_codes::ec(805)
                )));
            }
            SExpr::Quasiquote(inner) => {
                // Quasiquote: evaluate unquotes, leave rest as-is.
                match inner.as_ref() {
                    SExpr::Unquote(qq_inner) => {
                        current = (**qq_inner).clone();
                        continue 'main;
                    }
                    SExpr::List(items) if !items.is_empty() => {
                        let remaining: Vec<SExpr> = items[1..].to_vec();
                        stack.push(Cont::QQCollect { done: Vec::new(), remaining });
                        // Quasiquote-evaluate items[0].
                        current = qq_wrap(items[0].clone());
                        continue 'main;
                    }
                    other => other.clone(),
                }
            }
            SExpr::List(items) if items.is_empty() => SExpr::list(vec![]),
            SExpr::List(items) => {
                let head = items[0].as_symbol();
                match head {
                    Some("quote") => {
                        if items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} quote requires one argument",
                                crate::error_codes::ec(806)
                            )));
                        }
                        items[1].clone()
                    }
                    Some("if") => {
                        if items.len() < 4 {
                            return Err(sexpr_err(format!(
                                "{} if requires condition, then, else",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::IfCond {
                            then_expr: items[2].clone(),
                            else_expr: items[3].clone(),
                        });
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("list") => {
                        if items.len() <= 1 {
                            SExpr::list(vec![])
                        } else {
                            let remaining: Vec<SExpr> = items[2..].to_vec();
                            stack.push(Cont::Collect { done: Vec::new(), remaining });
                            current = items[1].clone();
                            continue 'main;
                        }
                    }
                    Some("car") => {
                        if items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} car requires one argument",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::Car);
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("cdr") => {
                        if items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} cdr requires one argument",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::Cdr);
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("cons") => {
                        if items.len() < 3 {
                            return Err(sexpr_err(format!(
                                "{} cons requires head and tail",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::ConsHead { tail_expr: items[2].clone() });
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("eq?") => {
                        if items.len() < 3 {
                            return Err(sexpr_err(format!(
                                "{} eq? requires two arguments",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::EqFirst { second_expr: items[2].clone() });
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("symbol?") => {
                        if items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} symbol? requires one argument",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::TypePred(TypePredKind::Symbol));
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("list?") => {
                        if items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} list? requires one argument",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::TypePred(TypePredKind::List));
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("integer?") => {
                        if items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} integer? requires one argument",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::TypePred(TypePredKind::Integer));
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("bool?") => {
                        if items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} bool? requires one argument",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::TypePred(TypePredKind::Bool));
                        current = items[1].clone();
                        continue 'main;
                    }
                    Some("match-type") => {
                        if items.len() < 3 {
                            return Err(sexpr_err(format!(
                                "{} match-type requires type and clauses",
                                crate::error_codes::ec(806)
                            )));
                        }
                        stack.push(Cont::MatchType { clauses: items[2..].to_vec() });
                        current = items[1].clone();
                        continue 'main;
                    }
                    _ => {
                        // Unknown form: evaluate all items and return as list.
                        let remaining: Vec<SExpr> = items[1..].to_vec();
                        stack.push(Cont::Collect { done: Vec::new(), remaining });
                        current = items[0].clone();
                        continue 'main;
                    }
                }
            }
        };

        // ── Apply continuations to value ───────────────────────────
        let mut val = value;
        loop {
            let cont = match stack.pop() {
                Some(c) => c,
                None => return Ok(val),
            };
            match cont {
                Cont::IfCond { then_expr, else_expr } => {
                    let is_true = match &val {
                        SExpr::Bool(b) => *b,
                        SExpr::Integer(n) => *n != 0,
                        SExpr::List(l) => !l.is_empty(),
                        _ => true,
                    };
                    current = if is_true { then_expr } else { else_expr };
                    continue 'main;
                }
                Cont::Collect { mut done, remaining } => {
                    done.push(val);
                    if remaining.is_empty() {
                        val = SExpr::list(done);
                        // keep unwinding
                    } else {
                        stack.push(Cont::Collect { done, remaining: remaining[1..].to_vec() });
                        current = remaining[0].clone();
                        continue 'main;
                    }
                }
                Cont::Car => match val {
                    SExpr::List(ref l) if !l.is_empty() => {
                        val = l[0].clone();
                    }
                    _ => {
                        return Err(sexpr_err(format!(
                            "{} car requires a non-empty list",
                            crate::error_codes::ec(805)
                        )));
                    }
                },
                Cont::Cdr => match val {
                    SExpr::List(ref l) if !l.is_empty() => {
                        val = SExpr::list(l[1..].to_vec());
                    }
                    _ => {
                        return Err(sexpr_err(format!(
                            "{} cdr requires a non-empty list",
                            crate::error_codes::ec(805)
                        )));
                    }
                },
                Cont::ConsHead { tail_expr } => {
                    stack.push(Cont::ConsTail { head_val: val });
                    current = tail_expr;
                    continue 'main;
                }
                Cont::ConsTail { head_val } => match val {
                    SExpr::List(mut l) => {
                        l.insert(0, head_val);
                        val = SExpr::list(l);
                    }
                    _ => {
                        val = SExpr::list(vec![head_val, val]);
                    }
                },
                Cont::EqFirst { second_expr } => {
                    stack.push(Cont::EqSecond { first_val: val });
                    current = second_expr;
                    continue 'main;
                }
                Cont::EqSecond { first_val } => {
                    val = SExpr::Bool(first_val == val);
                }
                Cont::TypePred(kind) => {
                    val = SExpr::Bool(match kind {
                        TypePredKind::Symbol => val.is_symbol(),
                        TypePredKind::List => val.is_list(),
                        TypePredKind::Integer => val.is_integer(),
                        TypePredKind::Bool => val.is_bool(),
                    });
                }
                Cont::MatchType { clauses } => {
                    let type_val = val;
                    let mut matched = false;
                    let mut clause_iters = 0usize;
                    for clause in &clauses {
                        clause_iters += 1;
                        if clause_iters > MAX_SEXPR_NODES {
                            return Err(sexpr_err(format!(
                                "{} match-type exceeded iteration budget",
                                crate::error_codes::ec(804)
                            )));
                        }
                        let clause_items = clause.as_list().ok_or_else(|| {
                            sexpr_err(format!(
                                "{} match-type clause must be a list",
                                crate::error_codes::ec(806)
                            ))
                        })?;
                        if clause_items.len() < 2 {
                            return Err(sexpr_err(format!(
                                "{} match-type clause requires pattern and body",
                                crate::error_codes::ec(806)
                            )));
                        }
                        let pattern = &clause_items[0];
                        let body = &clause_items[1];
                        let env_depth = state.env.len();
                        if match_type_pattern(&type_val, pattern, state)? {
                            stack.push(Cont::MatchTypeRestore { env_depth });
                            current = body.clone();
                            matched = true;
                            break;
                        }
                    }
                    if matched {
                        continue 'main;
                    }
                    return Err(sexpr_err(format!(
                        "{} No match-type clause matched",
                        crate::error_codes::ec(805)
                    )));
                }
                Cont::MatchTypeRestore { env_depth } => {
                    state.env.truncate(env_depth);
                    // val passes through
                }
                Cont::QQCollect { mut done, remaining } => {
                    done.push(val);
                    if remaining.is_empty() {
                        val = SExpr::list(done);
                        // keep unwinding
                    } else {
                        stack.push(Cont::QQCollect { done, remaining: remaining[1..].to_vec() });
                        current = qq_wrap(remaining[0].clone());
                        continue 'main;
                    }
                }
            }
        }
    }
}

/// Convert an S-expression for quasiquote evaluation:
/// - Unquote(x) → just x (evaluate it normally)
/// - List → wrap in Quasiquote so the main loop enters QQ mode
/// - Atoms → wrap in Quote so they self-evaluate
fn qq_wrap(expr: SExpr) -> SExpr {
    match expr {
        SExpr::Unquote(inner) => *inner,
        SExpr::List(_) => SExpr::Quasiquote(Box::new(expr)),
        other => SExpr::Quote(Box::new(other)),
    }
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

/// Look up a symbol binding in the environment (most-recent first).
fn lookup(env: &[(String, SExpr)], name: &str) -> Result<SExpr, MirrError> {
    env.iter().rev().find(|(k, _)| k == name).map(|(_, v)| v.clone()).ok_or_else(|| {
        sexpr_err(format!("{} Undefined symbol: {name}", crate::error_codes::ec(813)))
    })
}
