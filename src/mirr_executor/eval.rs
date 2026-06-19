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

/// Evaluate an ECS-resident expression. Bounded by NASA P10 (iterative stack).
pub(super) fn eval_expr_ecs(
    root: crate::ecs::EntityId,
    registry: &crate::ecs::Registry,
    env_get: &impl Fn(&str) -> Value,
) -> Value {
    use crate::ast::types::{BinaryOp, UnaryOp};

    let idx = root.0 as usize;
    if idx >= registry.names.len() {
        return Value::Integer(0);
    }

    // Literals
    if let Some(lit) = &registry.literals[idx] {
        return match &lit.0 {
            crate::ast::types::LiteralValue::Bool(b) => Value::Bool(*b),
            crate::ast::types::LiteralValue::Integer(n) => Value::Integer(*n),
        };
    }

    // Signal references
    if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) = &registry.signal_refs[idx] {
        if let Some(nc) = &registry.names[sig_ent.0 as usize] {
            return env_get(registry.resolve_name(nc.0));
        }
    }

    // Pending Signal references
    if let Some(crate::ecs::components::PendingSignalRef(name)) = &registry.pending_signal_refs[idx]
    {
        return env_get(name);
    }

    // Prev references
    if let Some(p) = &registry.prev_ops[idx] {
        if let Some(crate::ecs::components::SignalRefComponent(sig_ent)) =
            &registry.signal_refs[p.signal.0 as usize]
        {
            if let Some(nc) = &registry.names[sig_ent.0 as usize] {
                // In this executor, prev(x, d) is just env_get(x).
                // Proper delay is handled by the shift-register state machine in drive_*.
                return env_get(registry.resolve_name(nc.0));
            }
        }
    }

    // Unary Ops
    if let Some(u) = &registry.unary_ops[idx] {
        let v = eval_expr_ecs(u.operand, registry, env_get);
        return match u.op {
            UnaryOp::Not => Value::Bool(!v.as_bool()),
            UnaryOp::Negate => Value::Integer(0u64.wrapping_sub(v.as_int())),
            UnaryOp::ReductionOr => Value::Bool(v.as_int() != 0),
        };
    }

    // Binary Ops
    if let Some(bin) = &registry.binary_ops[idx] {
        let l = eval_expr_ecs(bin.left, registry, env_get);
        let r = eval_expr_ecs(bin.right, registry, env_get);
        return match bin.op {
            BinaryOp::And => {
                if let (Value::Integer(li), Value::Integer(ri)) = (&l, &r) {
                    Value::Integer(li & ri)
                } else {
                    Value::Bool(l.as_bool() & r.as_bool())
                }
            }
            BinaryOp::Or => {
                if let (Value::Integer(li), Value::Integer(ri)) = (&l, &r) {
                    Value::Integer(li | ri)
                } else {
                    Value::Bool(l.as_bool() | r.as_bool())
                }
            }
            BinaryOp::Xor => {
                if let (Value::Integer(li), Value::Integer(ri)) = (&l, &r) {
                    Value::Integer(li ^ ri)
                } else {
                    Value::Bool(l.as_bool() ^ r.as_bool())
                }
            }
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
            BinaryOp::BitwiseOr => Value::Integer(l.as_int() | r.as_int()),
            BinaryOp::BitwiseAnd => Value::Integer(l.as_int() & r.as_int()),
        };
    }

    Value::Integer(0)
}

pub(super) fn _eval_expr(e: &Expr, env_get: &impl Fn(&str) -> Value) -> Value {
    use crate::ast::expr::Expr as E;
    use crate::ast::types::{BinaryOp, UnaryOp};

    match e {
        E::Literal(LiteralValue::Bool(b)) => Value::Bool(*b),
        E::Literal(LiteralValue::Integer(i)) => Value::Integer(*i),
        E::Signal(name) => env_get(name),
        E::Prev { signal, .. } => env_get(signal),
        E::UnfoldIndex(_name) => {
            // UnfoldIndex is expected to be fully resolved during compile/lowering passes
            // and should not appear in scalar simulation contexts. Returning safe fallback.
            Value::Integer(0)
        }
        E::Unary { op, operand } => {
            let v = _eval_expr(operand, env_get);
            match op {
                UnaryOp::Not => Value::Bool(!v.as_bool()),
                UnaryOp::Negate => Value::Integer(0u64.wrapping_sub(v.as_int())),
                UnaryOp::ReductionOr => Value::Bool(v.as_int() != 0),
            }
        }
        E::Binary { op, left, right } => {
            let l = _eval_expr(left, env_get);
            let r = _eval_expr(right, env_get);
            match op {
                BinaryOp::And => {
                    if let (Value::Integer(li), Value::Integer(ri)) = (&l, &r) {
                        Value::Integer(li & ri)
                    } else {
                        Value::Bool(l.as_bool() & r.as_bool())
                    }
                }
                BinaryOp::Or => {
                    if let (Value::Integer(li), Value::Integer(ri)) = (&l, &r) {
                        Value::Integer(li | ri)
                    } else {
                        Value::Bool(l.as_bool() | r.as_bool())
                    }
                }
                BinaryOp::Xor => {
                    if let (Value::Integer(li), Value::Integer(ri)) = (&l, &r) {
                        Value::Integer(li ^ ri)
                    } else {
                        Value::Bool(l.as_bool() ^ r.as_bool())
                    }
                }
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
                BinaryOp::BitwiseOr => Value::Integer(l.as_int() | r.as_int()),
                BinaryOp::BitwiseAnd => Value::Integer(l.as_int() & r.as_int()),
            }
        }
        // Array index: look up element by constructing key "array[idx]".
        E::ArrayIndex { array, index } => {
            let idx = _eval_expr(index, env_get).as_int();
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

/// Build a fully initialized RuntimePools for the given registry (init-time only).
pub(super) fn init_pools_for_registry(
    registry: &crate::ecs::Registry,
    fingerprint: (usize, usize, usize, usize),
) -> RuntimePools {
    let mut guard_count = 0;
    let mut signal_count = 0;
    let mut reflex_count = 0;

    for k in registry.kinds.iter().flatten() {
        match k.0 {
            crate::ecs::EntityKind::GUARD => guard_count += 1,
            crate::ecs::EntityKind::SIGNAL(_) => signal_count += 1,
            crate::ecs::EntityKind::REFLEX => reflex_count += 1,
            _ => {}
        }
    }

    let mut p = RuntimePools::new(guard_count, signal_count, reflex_count);
    p.program_fingerprint = fingerprint;

    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind)) = (registry.names[i], &registry.kinds[i]) {
            let name = registry.resolve_name(nc.0);
            match kind.0 {
                crate::ecs::EntityKind::REFLEX => {
                    if name.contains("clear") || name.contains("tick") {
                        p.clear_reflex_names.push(name.to_string());
                    }
                }
                crate::ecs::EntityKind::SIGNAL(crate::ast::types::SignalKind::Input) => {
                    p.env.insert(name.to_string(), Value::Bool(false));
                }
                crate::ecs::EntityKind::GUARD => {
                    p.guard_counters.insert(name.to_string(), 0);
                    p.guard_active.insert(name.to_string(), false);
                }
                _ => {}
            }
        }
    }
    p.clear_reflex_names_snapshot = std::sync::Arc::new(p.clear_reflex_names.clone());

    p.persistent_env.clear();
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind), Some(ty)) =
            (registry.names[i], &registry.kinds[i], &registry.types[i])
        {
            if let crate::ecs::EntityKind::SIGNAL(_) = kind.0 {
                let name = registry.resolve_name(nc.0);
                match ty.0.signal_type() {
                    crate::ast::types::SignalType::Bool => {
                        p.persistent_env.insert(name.to_string(), Value::Bool(false));
                    }
                    crate::ast::types::SignalType::Unsigned(_)
                    | crate::ast::types::SignalType::Signed(_) => {
                        p.persistent_env.insert(name.to_string(), Value::Integer(0));
                    }
                    _ => {
                        p.persistent_env.insert(name.to_string(), Value::Integer(0));
                    }
                }
            }
        }
    }
    p.signal_env = p.persistent_env.clone();

    p.output_signal_names.clear();
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind)) = (registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::SIGNAL(crate::ast::types::SignalKind::Output) = kind.0 {
                p.output_signal_names.push(registry.resolve_name(nc.0).to_string());
            }
        }
    }

    p.sr_signal_names.clear();
    for i in 0..registry.names.len() {
        if let (Some(nc), Some(kind)) = (registry.names[i], &registry.kinds[i]) {
            if let crate::ecs::EntityKind::GUARD = kind.0 {
                let name = registry.resolve_name(nc.0);
                let prefix = format!("{}_sr_", name);
                let mut sr_names: Vec<(usize, String)> = Vec::new();
                for j in 0..registry.names.len() {
                    if let Some(target_nc) = &registry.names[j] {
                        let target_name = registry.resolve_name(target_nc.0);
                        if target_name.starts_with(&prefix) {
                            if let Ok(idx) = target_name[prefix.len()..].parse::<usize>() {
                                sr_names.push((idx, target_name.to_string()));
                            }
                        }
                    }
                }
                sr_names.sort_by_key(|(idx, _)| *idx);
                let mut ordered: Vec<String> = Vec::with_capacity(sr_names.len());
                for (_, n) in sr_names.into_iter() {
                    ordered.push(n);
                }
                p.sr_signal_names.push(ordered);
            }
        }
    }
    let max_sr = p.sr_signal_names.iter().map(|v| v.len()).max().unwrap_or(0);
    p.next_vals = Vec::with_capacity(max_sr);
    p
}
