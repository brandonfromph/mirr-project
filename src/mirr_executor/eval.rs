//! Expression evaluation, allocation hooks, and pool initialization.

#![forbid(unsafe_code)]

use std::sync::{Mutex, OnceLock};

use crate::ast::{Expr, LiteralValue};
use crate::mirr_runtime::Value;

use super::pools::RuntimePools;

type AllocHookLock = OnceLock<Mutex<Option<fn(&str)>>>;
static ALLOC_HOOK: AllocHookLock = OnceLock::new();

/// Set callback for allocation checkpoints (tests only).
pub fn set_alloc_hook(h: fn(&str)) {
    let m = ALLOC_HOOK.get_or_init(|| Mutex::new(None));
    let mut guard = m.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(h);
}

pub(super) fn maybe_hook(label: &str) {
    if let Some(m) = ALLOC_HOOK.get() {
        if let Some(h) = *m.lock().unwrap_or_else(|e| e.into_inner()) {
            h(label);
        }
    }
}

pub(super) fn eval_expr(e: &Expr, env_get: &impl Fn(&str) -> Value) -> Value {
    use crate::ast::expr::Expr as E;
    use crate::ast::types::{BinaryOp, UnaryOp};

    match e {
        E::Literal(LiteralValue::Bool(b)) => Value::Bool(*b),
        E::Literal(LiteralValue::Integer(i)) => Value::Integer(*i),
        E::Signal(name) => env_get(name),
        E::Prev { signal, .. } => env_get(signal),
        E::UnfoldIndex(name) => panic!("E506: UnfoldIndex reached analysis stage unresolved: {}", name),
        E::Unary { op, operand } => {
            let v = eval_expr(operand, env_get);
            match op {
                UnaryOp::Not => Value::Bool(!v.as_bool()),
                UnaryOp::Negate => Value::Integer(0u64.wrapping_sub(v.as_int())),
            }
        }
        E::Binary { op, left, right } => {
            let l = eval_expr(left, env_get);
            let r = eval_expr(right, env_get);
            match op {
                BinaryOp::And => Value::Bool(l.as_bool() & r.as_bool()),
                BinaryOp::Or => Value::Bool(l.as_bool() | r.as_bool()),
                BinaryOp::Xor => Value::Bool(l.as_bool() ^ r.as_bool()),
                BinaryOp::Lt => Value::Bool(l.as_int() < r.as_int()),
                BinaryOp::Le => Value::Bool(l.as_int() <= r.as_int()),
                BinaryOp::Gt => Value::Bool(l.as_int() > r.as_int()),
                BinaryOp::Ge => Value::Bool(l.as_int() >= r.as_int()),
                BinaryOp::Eq => Value::Bool(l.as_int() == r.as_int()),
                BinaryOp::Ne => Value::Bool(l.as_int() != r.as_int()),
                BinaryOp::Add => Value::Integer(l.as_int().wrapping_add(r.as_int())),
                BinaryOp::Sub => Value::Integer(l.as_int().wrapping_sub(r.as_int())),
                BinaryOp::Mul => Value::Integer(l.as_int().wrapping_mul(r.as_int())),
                BinaryOp::Shl => {
                    let amt = r.as_int().min(63);
                    Value::Integer(l.as_int() << amt)
                }
                BinaryOp::Shr => {
                    let amt = r.as_int().min(63);
                    Value::Integer(l.as_int() >> amt)
                }
            }
        }
        // Array index: look up element by constructing key "array[idx]".
        E::ArrayIndex { array, index } => {
            let idx = eval_expr(index, env_get).as_int();
            // Extract signal name from the array expression.
            let array_name = match array.as_ref() {
                E::Signal(name) => name.clone(),
                _ => return Value::Integer(0),
            };
            let key = format!("{array_name}[{idx}]");
            env_get(&key)
        }
        // Composite expressions: not supported in scalar executor; yield 0.
        E::FieldAccess { .. } | E::ArrayLiteral(_) | E::StructLiteral { .. } => Value::Integer(0),
    }
}

/// Build a fully initialized RuntimePools for the given program (init-time only).
pub(super) fn init_pools_for_program(
    prog: &crate::ast::MirrProgram,
    fingerprint: (usize, usize, usize, usize),
) -> RuntimePools {
    let mut p = RuntimePools::new(
        prog.module.guards.len(),
        prog.module.signals.len(),
        prog.module.reflexes.len(),
    );
    p.program_fingerprint = fingerprint;

    for r in &prog.module.reflexes {
        if r.name.contains("clear") || r.name.contains("tick") {
            p.clear_reflex_names.push(r.name.clone());
        }
    }
    for s in &prog.module.signals {
        if s.kind == crate::ast::types::SignalKind::Input {
            p.env.insert(s.name.clone(), Value::Bool(false));
        }
    }
    p.clear_reflex_names_snapshot = std::sync::Arc::new(p.clear_reflex_names.clone());

    p.guard_counters.clear();
    for g in &prog.module.guards {
        p.guard_counters.insert(g.name.clone(), 0);
        p.guard_active.insert(g.name.clone(), false);
    }
    p.persistent_env.clear();
    for s in &prog.module.signals {
        match s.ty.signal_type() {
            crate::ast::types::SignalType::Bool => {
                p.persistent_env.insert(s.name.clone(), Value::Bool(false));
            }
            crate::ast::types::SignalType::Unsigned(_)
            | crate::ast::types::SignalType::Signed(_) => {
                p.persistent_env.insert(s.name.clone(), Value::Integer(0));
            }
            crate::ast::types::SignalType::Array { .. }
            | crate::ast::types::SignalType::Struct { .. }
            | crate::ast::types::SignalType::FixedPoint { .. }
            | crate::ast::types::SignalType::Bundle(_)
            | crate::ast::types::SignalType::Fifo { .. } => {
                p.persistent_env.insert(s.name.clone(), Value::Integer(0));
            }
        }
    }
    p.signal_env = p.persistent_env.clone();

    p.output_signal_names.clear();
    for s in &prog.module.signals {
        if s.kind == crate::ast::types::SignalKind::Output {
            p.output_signal_names.push(s.name.clone());
        }
    }

    p.sr_signal_names.clear();
    for g in &prog.module.guards {
        let prefix = format!("{}_sr_", g.name);
        let mut names: Vec<(usize, String)> = Vec::new();
        for sig in &prog.module.signals {
            if sig.name.starts_with(&prefix) {
                if let Ok(idx) = sig.name[prefix.len()..].parse::<usize>() {
                    names.push((idx, sig.name.clone()));
                }
            }
        }
        names.sort_by_key(|(i, _)| *i);
        let mut ordered: Vec<String> = Vec::with_capacity(names.len());
        for (_, n) in names.into_iter() {
            ordered.push(n);
        }
        p.sr_signal_names.push(ordered);
    }
    let max_sr = p.sr_signal_names.iter().map(|v| v.len()).max().unwrap_or(0);
    p.next_vals = Vec::with_capacity(max_sr);
    p
}
