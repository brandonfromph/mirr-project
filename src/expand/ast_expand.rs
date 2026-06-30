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

/// Context maintained during AST expansion to ensure generated hardware
/// maintains structural integrity.
struct ExpansionCtx {
    /// Tracks guards to ensure names remain unique after expansion.
    declared_guards: HashSet<String>,
    /// Tracks signals inferred to be boolean (used for condition synthesis).
    bool_signals: HashSet<String>,
    /// Counter for synthesizing unique guard names for `if` and `match` constructs.
    auto_guard_counter: usize,
    /// Prefix to apply to synthesized guard names to prevent collisions during pattern expansion.
    origin_prefix: Option<String>,
}

type ModuleStackItem = (ModuleMacroStmt, HashMap<String, i32>, HashMap<String, String>);
type ReflexStackItem =
    (ReflexMacroStmt, Vec<String>, HashMap<String, i32>, HashMap<String, String>);

pub fn expand_module(unexpanded: UnexpandedModule) -> Result<Module, MirrError> {
    let mut module = Module {
        name: unexpanded.name,
        clock_domains: unexpanded.clock_domains,
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: unexpanded.properties,
        pattern_calls: unexpanded.pattern_calls,
        pattern_origins: Vec::new(),
        span: None,
    };

    expand_statements_inplace(
        &mut module,
        unexpanded.statements,
        HashMap::new(),
        HashMap::new(),
        None,
    )?;

    Ok(module)
}

/// Iteratively expands macro fragments (`ForLoop`, `Match`, etc.) into a flat `Module`.
///
/// This is the core expansion engine, fulfilling NASA P10 constraints by using an
/// explicit work stack instead of recursion. It handles:
/// 1. Signal/Guard/Reflex generation.
/// 2. Iterative loop unrolling.
/// 3. Conditional guard synthesis (for `if`/`match` blocks).
/// 4. Origin tag propagation.
///
/// # Arguments
/// - `module`: The flat module to populate with expanded statements.
/// - `statements`: The AST macro fragments to expand.
/// - `env`: Current loop-variable bindings (e.g., for `${i}` in `for` loops).
/// - `signal_env`: Template substitution map for signal names.
/// - `origin`: Optional origin tag for DO-178C traceability.
pub fn expand_statements_inplace(
    module: &mut Module,
    statements: Vec<ModuleMacroStmt>,
    env: HashMap<String, i32>,
    signal_env: HashMap<String, String>,
    origin: Option<String>,
) -> Result<(), MirrError> {
    let mut max_auto = 0;
    for g in &module.guards {
        if g.name.starts_with("auto_g_") {
            if let Ok(num) = g.name["auto_g_".len()..].parse::<usize>() {
                if num >= max_auto {
                    max_auto = num + 1;
                }
            }
        }
    }

    let mut ctx = ExpansionCtx {
        declared_guards: HashSet::new(),
        bool_signals: HashSet::new(),
        auto_guard_counter: max_auto,
        origin_prefix: origin.clone(),
    };

    // Pre-pass: Collect manually declared guards and bool signals
    for stmt in &statements {
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

    let mut stmts_to_process: Vec<ModuleStackItem> =
        statements.into_iter().map(|s| (s, env.clone(), signal_env.clone())).rev().collect();

    let mut iterations = 0;
    while let Some((stmt, cur_env, cur_signal_env)) = stmts_to_process.pop() {
        iterations += 1;
        if iterations > MAX_EXPANSION_ITERATIONS * 10 {
            return Err(MirrError::SemanticError {
                message: "AST expansion exceeded iteration limit.".into(),
                span: None,
            });
        }

        match stmt {
            ModuleMacroStmt::Signal(mut sig) => {
                expand_signal_template(&mut sig, &cur_env, &cur_signal_env);
                if origin.is_some() {
                    sig.origin = origin.clone();
                }
                module.signals.push(sig);
            }
            ModuleMacroStmt::ClockDomain(mut cd) => {
                cd.name = expand_string(&cd.name, &cur_env, &cur_signal_env);
                if origin.is_some() {
                    cd.span = None; // Can optionally mark origin if needed
                }
                module.clock_domains.push(cd);
            }
            ModuleMacroStmt::Guard(mut guard) => {
                expand_guard_template(&mut guard, &cur_env, &cur_signal_env);
                if origin.is_some() {
                    guard.origin = origin.clone();
                }
                module.guards.push(guard);
            }
            ModuleMacroStmt::Property(mut prop) => {
                prop.name = expand_string(&prop.name, &cur_env, &cur_signal_env);
                for expr in prop.formula.exprs_mut() {
                    *expr = expand_expr(expr, &cur_env, &cur_signal_env);
                }
                if origin.is_some() {
                    prop.origin = origin.clone();
                }
                module.properties.push(prop);
            }
            ModuleMacroStmt::PatternCall(mut call) => {
                call.pattern_name = expand_string(&call.pattern_name, &cur_env, &cur_signal_env);
                for arg in &mut call.arguments {
                    match arg {
                        crate::ast::pattern::PatternArg::SignalRef(name)
                        | crate::ast::pattern::PatternArg::PatternRef(name) => {
                            *name = expand_string(name, &cur_env, &cur_signal_env);
                        }
                        _ => {}
                    }
                }
                module.pattern_calls.push(call);
            }
            ModuleMacroStmt::LetBinding { name, ty, value } => {
                let expanded_name = expand_string(&name, &cur_env, &cur_signal_env);
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
                    origin: origin.clone(),
                    span: None,
                };
                module.signals.push(sig);

                // Create a reflex for the let binding assignment
                let assign = Assignment {
                    target: expanded_name.clone(),
                    value: expand_expr(&value, &cur_env, &cur_signal_env),
                    span: None,
                };
                let reflex = Reflex {
                    name: format!("{}_bind", expanded_name),
                    guard_names: vec!["always".to_string()],
                    assignments: vec![assign],
                    origin: origin.clone(),
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
                    let mut new_env = cur_env.clone();
                    new_env.insert(var.clone(), i);
                    for b in body.iter().rev() {
                        stmts_to_process.push((b.clone(), new_env.clone(), cur_signal_env.clone()));
                    }
                }
            }
            ModuleMacroStmt::Reflex(unexp_reflex) => {
                let mut expanded_reflexes = expand_reflex_internal(
                    unexp_reflex,
                    &cur_env,
                    &cur_signal_env,
                    &mut ctx,
                    &mut module.guards,
                    &mut module.signals,
                )?;
                if let Some(ref o) = origin {
                    for r in &mut expanded_reflexes {
                        r.origin = Some(o.clone());
                    }
                }
                module.reflexes.extend(expanded_reflexes);
            }
        }
    }

    Ok(())
}

fn expand_reflex_internal(
    unexp: UnexpandedReflex,
    env: &HashMap<String, i32>,
    signal_env: &HashMap<String, String>,
    ctx: &mut ExpansionCtx,
    global_guards: &mut Vec<Guard>,
    global_signals: &mut Vec<SignalDecl>,
) -> Result<Vec<Reflex>, MirrError> {
    let reflex_span = unexp.span;
    let base_name = expand_string(&unexp.name, env, signal_env);
    let initial_guards: Vec<String> =
        unexp.guard_names.iter().map(|n| expand_string(n, env, signal_env)).collect();

    let mut result: Vec<Reflex> = Vec::new();

    // A stack item pairs a ReflexMacroStmt with its active guard context and loop environment
    let mut stmts_to_process: Vec<ReflexStackItem> = unexp
        .statements
        .into_iter()
        .map(|s| (s, initial_guards.clone(), env.clone(), signal_env.clone()))
        .rev()
        .collect();

    let mut iterations = 0;
    while let Some((stmt, active_guards, cur_env, cur_signal_env)) = stmts_to_process.pop() {
        iterations += 1;
        if iterations > MAX_EXPANSION_ITERATIONS * 10 {
            return Err(MirrError::SemanticError {
                message: "Reflex AST expansion exceeded iteration limit.".into(),
                span: reflex_span,
            });
        }

        match stmt {
            ReflexMacroStmt::Assignment(mut assign) => {
                assign.target = expand_string(&assign.target, &cur_env, &cur_signal_env);
                assign.value = expand_expr(&assign.value, &cur_env, &cur_signal_env);

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
            ReflexMacroStmt::LetBinding { name, ty, value, span } => {
                let expanded_name = expand_string(&name, &cur_env, &cur_signal_env);

                // Optimization: Signal deduplication.
                // If a signal with this name already exists at the global level,
                // don't create a new one. This restores the behavior expected by
                // NoC Router where 'let dest_id' is used in multiple non-overlapping blocks.
                let mut exists = false;
                for sig in global_signals.iter() {
                    if sig.name == expanded_name {
                        exists = true;
                        break;
                    }
                }

                if !exists {
                    // 1. Create the internal signal
                    let sig = SignalDecl {
                        name: expanded_name.clone(),
                        kind: crate::ast::types::SignalKind::Internal,
                        ty: crate::ast::types::ExtendedType::new(
                            crate::parser::parse_signal_type_str(&ty).ok_or_else(|| {
                                MirrError::SemanticError {
                                    message: format!("Invalid type '{}' in let binding.", ty),
                                    span,
                                }
                            })?,
                            crate::ast::types::TypeAnnotations::default(),
                        ),
                        origin: None,
                        span,
                    };
                    global_signals.push(sig);
                }

                // 2. Add an assignment to the current reflex
                let assign = Assignment {
                    target: expanded_name,
                    value: expand_expr(&value, &cur_env, &cur_signal_env),
                    span,
                };

                // Group assignments into a Reflex by guard context
                if let Some(last) = result.last_mut() {
                    if last.guard_names == active_guards {
                        last.assignments.push(assign);
                    } else {
                        let reflex = Reflex {
                            name: format!("{}_c{}", base_name, result.len()),
                            guard_names: if active_guards.is_empty() {
                                vec!["always".to_string()]
                            } else {
                                active_guards.clone()
                            },
                            assignments: vec![assign],
                            origin: None,
                            span: reflex_span,
                        };
                        result.push(reflex);
                    }
                } else {
                    let reflex = Reflex {
                        name: format!("{}_c{}", base_name, result.len()),
                        guard_names: if active_guards.is_empty() {
                            vec!["always".to_string()]
                        } else {
                            active_guards.clone()
                        },
                        assignments: vec![assign],
                        origin: None,
                        span: reflex_span,
                    };
                    result.push(reflex);
                }
            }
            ReflexMacroStmt::OnBlock { guard_names, body } => {
                let mut nested_guards = active_guards.clone();
                for g in guard_names {
                    nested_guards.push(expand_string(&g, &cur_env, &cur_signal_env));
                }
                for s in body.into_iter().rev() {
                    stmts_to_process.push((
                        s,
                        nested_guards.clone(),
                        cur_env.clone(),
                        cur_signal_env.clone(),
                    ));
                }
            }
            ReflexMacroStmt::ForLoop { var, start, end, body } => {
                for i in (start..end).rev() {
                    let mut new_env = cur_env.clone();
                    new_env.insert(var.clone(), i);
                    for b in body.iter().rev() {
                        stmts_to_process.push((
                            b.clone(),
                            active_guards.clone(),
                            new_env.clone(),
                            cur_signal_env.clone(),
                        ));
                    }
                }
            }
            ReflexMacroStmt::IfElse { condition, true_branch, false_branch } => {
                let cond_expr = expand_expr(&condition, &cur_env, &cur_signal_env);

                // True branch: use or synthesize a guard for the condition
                let true_guard_name = synthesize_guard(&cond_expr, ctx, global_guards)?;
                let mut true_guards = active_guards.clone();
                true_guards.push(true_guard_name);
                for b in true_branch.into_iter().rev() {
                    stmts_to_process.push((
                        b,
                        true_guards.clone(),
                        cur_env.clone(),
                        cur_signal_env.clone(),
                    ));
                }

                // False branch: optimize "else if" by combining conditions to avoid intermediate guards
                if !false_branch.is_empty() {
                    let false_cond = Expr::Unary {
                        op: crate::ast::types::UnaryOp::Not,
                        operand: Box::new(cond_expr),
                    };

                    if false_branch.len() == 1 {
                        if let ReflexMacroStmt::IfElse {
                            condition: ref next_cond,
                            true_branch: ref next_true,
                            false_branch: ref next_false,
                        } = false_branch[0]
                        {
                            // Combine: !cond && next_cond
                            let combined_cond = Expr::Binary {
                                op: crate::ast::types::BinaryOp::And,
                                left: Box::new(false_cond),
                                right: Box::new(expand_expr(next_cond, &cur_env, &cur_signal_env)),
                            };
                            let combined_stmt = ReflexMacroStmt::IfElse {
                                condition: combined_cond,
                                true_branch: next_true.clone(),
                                false_branch: next_false.clone(),
                            };
                            stmts_to_process.push((
                                combined_stmt,
                                active_guards.clone(),
                                cur_env.clone(),
                                cur_signal_env.clone(),
                            ));
                            continue;
                        }
                    }

                    // Fallback: synthesize a guard for the negated condition
                    let false_guard_name = synthesize_guard(&false_cond, ctx, global_guards)?;
                    let mut false_guards = active_guards.clone();
                    false_guards.push(false_guard_name);
                    for b in false_branch.into_iter().rev() {
                        stmts_to_process.push((
                            b,
                            false_guards.clone(),
                            cur_env.clone(),
                            cur_signal_env.clone(),
                        ));
                    }
                }
            }
            ReflexMacroStmt::Match { expr, arms } => {
                let match_expr = expand_expr(&expr, &cur_env, &cur_signal_env);
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
                        stmts_to_process.push((
                            b,
                            arm_guards.clone(),
                            cur_env.clone(),
                            cur_signal_env.clone(),
                        ));
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
    // Optimization: If the condition is JUST a reference to an existing guard, reuse it.
    if let Expr::Signal(ref name) = cond {
        if ctx.declared_guards.contains(name) {
            return Ok(name.clone());
        }
    }

    let base_name = if let Some(prefix) = &ctx.origin_prefix {
        format!("{}_auto_g_{}", prefix, ctx.auto_guard_counter)
    } else {
        format!("auto_g_{}", ctx.auto_guard_counter)
    };

    // BUG-4: Deduplicate identical synthesized conditions to minimize token usage and RTL bloat.
    for g in global_guards.iter() {
        if g.name == base_name && &g.condition == cond && g.cycles == 1 {
            return Ok(g.name.clone());
        }
    }

    let name = base_name;
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

fn expand_string(
    s: &str,
    env: &HashMap<String, i32>,
    signal_env: &HashMap<String, String>,
) -> String {
    let mut res = s.to_string();

    // Sort keys by length descending to prevent partial match collisions (e.g. ${x} vs ${xx})
    let mut keys: Vec<&String> = env.keys().collect();
    keys.sort_by_key(|b| std::cmp::Reverse(b.len()));

    for var in keys {
        let val = env[var];
        res = res.replace(&format!("[{var}]"), &format!("_{val}"));
        res = res.replace(&format!("${{{var}}}"), &val.to_string());
    }

    // Apply signal renames
    // SAFETY: We must NOT replace core keywords or literals like "true"/"false"
    // unless they are explicitly wrapped in ${...} or are the ONLY thing in the string.
    for (k, v) in signal_env {
        res = res.replace(&format!("${{{k}}}"), v);
        // ONLY replace exact identifier match if it's NOT a reserved literal/keyword
        if !matches!(k.as_str(), "true" | "false" | "clk" | "rst_n") {
            if &res == k {
                res = v.clone();
            } else if let Some(bracket_idx) = res.find('[') {
                if &res[..bracket_idx] == k {
                    res = format!("{}{}", v, &res[bracket_idx..]);
                }
            }
        }
    }

    res
}

fn expand_signal_template(
    sig: &mut SignalDecl,
    env: &HashMap<String, i32>,
    signal_env: &HashMap<String, String>,
) {
    sig.name = expand_string(&sig.name, env, signal_env);
}

fn expand_guard_template(
    guard: &mut Guard,
    env: &HashMap<String, i32>,
    signal_env: &HashMap<String, String>,
) {
    guard.name = expand_string(&guard.name, env, signal_env);
    if let Some(ref mut tc) = guard.template_cycles {
        *tc = expand_string(tc, env, signal_env);
    }
    guard.condition = expand_expr(&guard.condition, env, signal_env);
}

fn expand_expr(
    expr: &Expr,
    env: &HashMap<String, i32>,
    signal_env: &HashMap<String, String>,
) -> Expr {
    let mut new_expr = expr.clone();
    let mut rename_map = HashMap::new();
    for (k, v) in env {
        rename_map.insert(format!("${{{}}}", k), v.to_string());
        rename_map.insert(format!("[{}]", k), format!("_{}", v));
        rename_map.insert(k.clone(), v.to_string());
    }
    for (k, v) in signal_env {
        // ONLY insert into rename map if NOT a reserved literal
        if !matches!(k.as_str(), "true" | "false") {
            rename_map.insert(k.clone(), v.clone());
            rename_map.insert(format!("${{{}}}", k), v.clone());
        }
    }
    crate::expand::rename::rename_expr_signals(&mut new_expr, &rename_map);
    new_expr
}
