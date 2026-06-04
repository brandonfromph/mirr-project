//! AST Macro Expansion Pass.
//!
//! Lowers structural macros (`ForLoop`, `Match`, `IfElse`, `LetBinding`, `OnBlock`)
//! into the core MIRR flat AST. Replaces string-based template substitution
//! with deterministic, purely iterative AST traversal.
//!
//! NASA P10 Constraints:
//! - No recursion: Iterative stacks are used for all tree traversals.
//! - Bounded execution: Max bounds enforced on loops, AST depths, and generation.

#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};

use crate::ast::expr::Expr;
use crate::ast::macro_nodes::{
    ModuleMacroStmt, ReflexMacroStmt, UnexpandedModule, UnexpandedReflex,
};
use crate::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use crate::ast::types::SignalType;
use crate::error::MirrError;

const MAX_EXPANSION_ITERATIONS: usize = 1024;

struct ExpansionCtx {
    declared_guards: HashSet<String>,
    bool_signals: HashSet<String>,
    auto_guard_counter: usize,
}

pub fn expand_module(unexpanded: UnexpandedModule) -> Result<Module, MirrError> {
    let mut module = Module {
        name: unexpanded.name,
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: unexpanded.properties,
        pattern_calls: unexpanded.pattern_calls,
        pattern_origins: Vec::new(),
        span: None,
    };

    let mut ctx = ExpansionCtx {
        declared_guards: HashSet::new(),
        bool_signals: HashSet::new(),
        auto_guard_counter: 0,
    };

    // Pre-pass: Collect manually declared guards and bool signals
    for stmt in &unexpanded.statements {
        match stmt {
            ModuleMacroStmt::Guard(g) => {
                ctx.declared_guards.insert(g.name.clone());
            }
            ModuleMacroStmt::Signal(s) if s.ty.core == SignalType::Bool => {
                ctx.bool_signals.insert(s.name.clone());
            }
            _ => {}
        }
    }

    let mut stmts_to_process: Vec<(ModuleMacroStmt, HashMap<String, i32>)> =
        unexpanded.statements.into_iter().map(|s| (s, HashMap::new())).rev().collect(); // stack pops from end, so rev()

    let mut iterations = 0;
    while let Some((stmt, env)) = stmts_to_process.pop() {
        iterations += 1;
        if iterations > MAX_EXPANSION_ITERATIONS * 10 {
            return Err(MirrError::SemanticError {
                message: "AST expansion exceeded iteration limit.".into(),
                span: None,
            });
        }

        match stmt {
            ModuleMacroStmt::Signal(mut sig) => {
                expand_signal_template(&mut sig, &env);
                module.signals.push(sig);
            }
            ModuleMacroStmt::Guard(mut guard) => {
                expand_guard_template(&mut guard, &env);
                module.guards.push(guard);
            }
            ModuleMacroStmt::Property(mut prop) => {
                prop.name = expand_string(&prop.name, &env);
                for expr in prop.formula.exprs_mut() {
                    *expr = expand_expr(expr, &env);
                }
                module.properties.push(prop);
            }
            ModuleMacroStmt::PatternCall(mut call) => {
                call.pattern_name = expand_string(&call.pattern_name, &env);
                for arg in &mut call.arguments {
                    match arg {
                        crate::ast::pattern::PatternArg::SignalRef(name)
                        | crate::ast::pattern::PatternArg::PatternRef(name) => {
                            *name = expand_string(name, &env);
                        }
                        _ => {}
                    }
                }
                module.pattern_calls.push(call);
            }
            ModuleMacroStmt::LetBinding { name, ty, value } => {
                let expanded_name = expand_string(&name, &env);
                let sig = SignalDecl {
                    name: expanded_name.clone(),
                    kind: crate::ast::types::SignalKind::Internal,
                    ty: crate::ast::types::ExtendedType::new(
                        crate::parser::parse_signal_type_str(&ty).ok_or_else(|| {
                            MirrError::SemanticError {
                                message: format!("Invalid type '{}' in let binding.", ty),
                                span: None,
                            }
                        })?,
                        crate::ast::types::TypeAnnotations::default(),
                    ),
                    origin: None,
                    span: None,
                };
                module.signals.push(sig);

                // Create a reflex for the let binding assignment
                let assign = Assignment {
                    target: expanded_name.clone(),
                    value: expand_expr(&value, &env),
                    span: None,
                };
                let reflex = Reflex {
                    name: format!("{}_bind", expanded_name),
                    guard_names: vec!["always".to_string()],
                    assignments: vec![assign],
                    origin: None,
                    span: None,
                };
                module.reflexes.push(reflex);
            }
            ModuleMacroStmt::ForLoop { var, start, end, body } => {
                if end < start {
                    return Err(MirrError::SemanticError {
                        message: format!("For-loop end {} < start {}", end, start),
                        span: None,
                    });
                }
                for i in (start..end).rev() {
                    let mut new_env = env.clone();
                    new_env.insert(var.clone(), i);
                    for b in body.iter().rev() {
                        stmts_to_process.push((b.clone(), new_env.clone()));
                    }
                }
            }
            ModuleMacroStmt::Reflex(unexp_reflex) => {
                let expanded_reflexes =
                    expand_reflex_internal(unexp_reflex, &env, &mut ctx, &mut module.guards)?;
                module.reflexes.extend(expanded_reflexes);
            }
        }
    }

    Ok(module)
}

fn expand_reflex_internal(
    unexp: UnexpandedReflex,
    env: &HashMap<String, i32>,
    ctx: &mut ExpansionCtx,
    global_guards: &mut Vec<Guard>,
) -> Result<Vec<Reflex>, MirrError> {
    let reflex_span = unexp.span;
    let base_name = expand_string(&unexp.name, env);
    let initial_guards: Vec<String> =
        unexp.guard_names.iter().map(|n| expand_string(n, env)).collect();

    let mut result: Vec<Reflex> = Vec::new();

    // A stack item pairs a ReflexMacroStmt with its active guard context and loop environment
    let mut stmts_to_process: Vec<(ReflexMacroStmt, Vec<String>, HashMap<String, i32>)> = unexp
        .statements
        .into_iter()
        .map(|s| (s, initial_guards.clone(), env.clone()))
        .rev()
        .collect();

    let mut iterations = 0;
    while let Some((stmt, active_guards, cur_env)) = stmts_to_process.pop() {
        iterations += 1;
        if iterations > MAX_EXPANSION_ITERATIONS * 10 {
            return Err(MirrError::SemanticError {
                message: "Reflex AST expansion exceeded iteration limit.".into(),
                span: reflex_span,
            });
        }

        match stmt {
            ReflexMacroStmt::Assignment(mut assign) => {
                assign.target = expand_string(&assign.target, &cur_env);
                assign.value = expand_expr(&assign.value, &cur_env);

                // Group assignments into a Reflex by guard context
                if let Some(last) = result.last_mut() {
                    if last.guard_names == active_guards {
                        last.assignments.push(assign);
                        continue;
                    }
                }

                let reflex = Reflex {
                    name: format!("{}_c{}", base_name, result.len()),
                    guard_names: if active_guards.is_empty() {
                        vec!["always".to_string()]
                    } else {
                        active_guards
                    },
                    assignments: vec![assign],
                    origin: None,
                    span: reflex_span,
                };
                result.push(reflex);
            }
            ReflexMacroStmt::OnBlock { guard_names, body } => {
                let mut nested_guards = active_guards.clone();
                for g in guard_names {
                    nested_guards.push(expand_string(&g, &cur_env));
                }
                for s in body.into_iter().rev() {
                    stmts_to_process.push((s, nested_guards.clone(), cur_env.clone()));
                }
            }
            ReflexMacroStmt::ForLoop { var, start, end, body } => {
                for i in (start..end).rev() {
                    let mut new_env = cur_env.clone();
                    new_env.insert(var.clone(), i);
                    for b in body.iter().rev() {
                        stmts_to_process.push((b.clone(), active_guards.clone(), new_env.clone()));
                    }
                }
            }
            ReflexMacroStmt::IfElse { condition, true_branch, false_branch } => {
                let cond_expr = expand_expr(&condition, &cur_env);

                // True branch
                let true_guard_name = synthesize_guard(&cond_expr, ctx, global_guards)?;
                let mut true_guards = active_guards.clone();
                true_guards.push(true_guard_name);
                for b in true_branch.into_iter().rev() {
                    stmts_to_process.push((b, true_guards.clone(), cur_env.clone()));
                }

                // False branch (negated condition)
                if !false_branch.is_empty() {
                    let false_expr = Expr::Unary {
                        op: crate::ast::types::UnaryOp::Not,
                        operand: Box::new(cond_expr),
                    };
                    let false_guard_name = synthesize_guard(&false_expr, ctx, global_guards)?;
                    let mut false_guards = active_guards.clone();
                    false_guards.push(false_guard_name);
                    for b in false_branch.into_iter().rev() {
                        stmts_to_process.push((b, false_guards.clone(), cur_env.clone()));
                    }
                }
            }
            ReflexMacroStmt::Match { expr, arms } => {
                let match_expr = expand_expr(&expr, &cur_env);
                for arm in arms.into_iter().rev() {
                    let arm_cond = if arm.pattern == "_" {
                        Expr::Literal(crate::ast::types::LiteralValue::Bool(true))
                    } else {
                        let pat_expr = crate::parser::expr_parser::parse_expression(&arm.pattern)
                            .map_err(|e| MirrError::SemanticError {
                            message: format!("Invalid match pattern '{}': {}", arm.pattern, e),
                            span: None,
                        })?;
                        Expr::Binary {
                            op: crate::ast::types::BinaryOp::Eq,
                            left: Box::new(match_expr.clone()),
                            right: Box::new(pat_expr),
                        }
                    };

                    let arm_guard_name = synthesize_guard(&arm_cond, ctx, global_guards)?;
                    let mut arm_guards = active_guards.clone();
                    arm_guards.push(arm_guard_name);
                    for b in arm.body.into_iter().rev() {
                        stmts_to_process.push((b, arm_guards.clone(), cur_env.clone()));
                    }
                }
            }
        }
    }

    if result.is_empty() {
        return Err(MirrError::SemanticError {
            message: format!("Reflex '{}' must contain at least one assignment.", base_name),
            span: reflex_span,
        });
    }

    // MEGA-10: Preserve original name if no splitting occurred.
    if result.len() == 1 {
        result[0].name = base_name;
    }

    Ok(result)
}

fn synthesize_guard(
    cond: &Expr,
    ctx: &mut ExpansionCtx,
    global_guards: &mut Vec<Guard>,
) -> Result<String, MirrError> {
    let name = format!("auto_g_{}", ctx.auto_guard_counter);
    ctx.auto_guard_counter += 1;
    global_guards.push(Guard {
        name: name.clone(),
        condition: cond.clone(),
        cycles: 1,
        template_cycles: None,
        origin: None,
        span: None,
    });
    Ok(name)
}

fn expand_string(s: &str, env: &HashMap<String, i32>) -> String {
    let mut res = s.to_string();

    // Sort keys by length descending to prevent partial match collisions (e.g. ${x} vs ${xx})
    let mut keys: Vec<&String> = env.keys().collect();
    keys.sort_by_key(|b| std::cmp::Reverse(b.len()));

    for var in keys {
        let val = env[var];
        res = res.replace(&format!("[{var}]"), &format!("_{val}"));
        res = res.replace(&format!("${{{var}}}"), &val.to_string());
    }
    res
}

fn expand_signal_template(sig: &mut SignalDecl, env: &HashMap<String, i32>) {
    sig.name = expand_string(&sig.name, env);
}

fn expand_guard_template(guard: &mut Guard, env: &HashMap<String, i32>) {
    guard.name = expand_string(&guard.name, env);
    if let Some(ref mut tc) = guard.template_cycles {
        *tc = expand_string(tc, env);
    }
    guard.condition = expand_expr(&guard.condition, env);
}

fn expand_expr(expr: &Expr, env: &HashMap<String, i32>) -> Expr {
    let mut new_expr = expr.clone();
    let mut rename_map = HashMap::new();
    for (k, v) in env {
        rename_map.insert(format!("${{{}}}", k), v.to_string());
        rename_map.insert(k.clone(), v.to_string());
    }
    crate::expand::rename::rename_expr_signals(&mut new_expr, &rename_map);
    new_expr
}
