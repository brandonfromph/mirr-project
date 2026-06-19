//! MIRR signal evaluator and interpreter engine.
//!
//! Drives the execution of parsed MIRR modules by evaluating guard conditions,
//! firing reflexes, and updating signal state. Used by the MAPE-K simulation harness.

#![forbid(unsafe_code)]

mod eval;
mod pools;

pub use eval::set_alloc_hook;
use eval::{init_pools_for_registry, maybe_hook};
use pools::RuntimePools;

use crate::ast::types::SignalKind;
use crate::mirr_driver::ObservedPush;
use crate::mirr_runtime::Value;
use crate::parser::parse_mirr;
use std::fs;
use std::path::Path;
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

static PROBE_OUTPOOL_TAKEN: AtomicUsize = AtomicUsize::new(0);
static PROBE_PUSH_SAMPLE: AtomicUsize = AtomicUsize::new(0);

static LEXER_REGISTRY: OnceLock<Option<crate::ecs::Registry>> = OnceLock::new();

fn load_lexer_registry() -> Option<&'static crate::ecs::Registry> {
    LEXER_REGISTRY.get_or_init(|| {
        let path = Path::new("compiler_mirr").join("lexer.mirr");
        let txt = fs::read_to_string(&path).ok()?;
        let prog = parse_mirr(&txt).ok()?;
        let mut reg = crate::ecs::Registry::new();
        reg.ingest_program(&prog).ok()?;
        Some(reg)
    });
    if let Some(opt) = LEXER_REGISTRY.get() {
        return opt.as_ref();
    }
    None
}

/// Drive a parsed MIRR module through the interpreter using the ECS Registry.
pub fn drive_parsed_module_with_interpreter(
    registry: &crate::ecs::Registry,
    input: &[u8],
) -> Vec<ObservedPush> {
    maybe_hook("start");
    let s = match str::from_utf8(input) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let bytes = s.as_bytes();
    let mut pos = 0usize;
    let len = bytes.len();

    let reflex_count = registry.reflex_comps.iter().flatten().count();
    let initial_out_cap = std::cmp::max((reflex_count.max(8)).saturating_mul(4), 1024);

    maybe_hook("before_out_pool_init");
    static OUT_POOL: OnceLock<Mutex<Vec<Vec<ObservedPush>>>> = OnceLock::new();
    let pool = OUT_POOL.get_or_init(|| Mutex::new(Vec::new()));
    maybe_hook("after_out_pool_init");
    let mut out: Vec<ObservedPush> = {
        let mut guard = pool.lock().unwrap_or_else(|e| e.into_inner());
        if guard.is_empty() {
            for _ in 0..8 {
                guard.push(Vec::with_capacity(initial_out_cap));
            }
        }
        let idx = guard
            .iter()
            .position(|v| v.capacity() >= initial_out_cap)
            .unwrap_or_else(|| guard.len() - 1);
        std::mem::take(&mut guard[idx])
    };
    PROBE_OUTPOOL_TAKEN.fetch_add(1, Ordering::SeqCst);

    static POOLS: OnceLock<Mutex<RuntimePools>> = OnceLock::new();

    let module_name_len = registry
        .kinds
        .iter()
        .enumerate()
        .find_map(|(i, k)| {
            if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::MODULE)) = k {
                registry.names[i].map(|nc| registry.resolve_name(nc.0).len())
            } else {
                None
            }
        })
        .unwrap_or(0);
    let signal_count = registry
        .kinds
        .iter()
        .flatten()
        .filter(|k| matches!(k.0, crate::ecs::EntityKind::SIGNAL(_)))
        .count();
    let guard_count = registry
        .kinds
        .iter()
        .flatten()
        .filter(|k| matches!(k.0, crate::ecs::EntityKind::GUARD))
        .count();

    let current_fingerprint = (module_name_len, signal_count, guard_count, reflex_count);

    maybe_hook("before_pools_init");
    let pools_mutex =
        POOLS.get_or_init(|| Mutex::new(init_pools_for_registry(registry, current_fingerprint)));

    let mut pools_guard = pools_mutex.lock().unwrap_or_else(|e| e.into_inner());
    maybe_hook("after_pools_lock");

    if pools_guard.program_fingerprint != current_fingerprint {
        *pools_guard = init_pools_for_registry(registry, current_fingerprint);
    }

    let pools = &mut *pools_guard;
    let clear_reflex_names_snapshot = pools.clear_reflex_names_snapshot.clone();
    maybe_hook("before_tick_loop");
    while pos < len {
        let b = bytes[pos];

        let is_whitespace = b == b' ' || b == b'\t' || b == b'\n' || b == b'\r';
        if is_whitespace {
            pos += 1;
            continue;
        }

        let mut current_int: Option<u64> = None;

        pools.clear_per_tick();
        let env = &mut pools.env;

        // Simple token detection logic (abridged for brevity, matching existing drive logic)
        if pos + 1 < len {
            match (bytes[pos], bytes[pos + 1]) {
                (b'=', b'=') => {
                    if let Some(v) = env.get_mut("input_two_eq") {
                        *v = Value::Bool(true);
                    }
                }
                (b'!', b'=') => {
                    if let Some(v) = env.get_mut("input_two_ne") {
                        *v = Value::Bool(true);
                    }
                }
                (b'<', b'=') => {
                    if let Some(v) = env.get_mut("input_two_le") {
                        *v = Value::Bool(true);
                    }
                }
                (b'>', b'=') => {
                    if let Some(v) = env.get_mut("input_two_ge") {
                        *v = Value::Bool(true);
                    }
                }
                (b'-', b'>') => {
                    if let Some(v) = env.get_mut("input_arrow") {
                        *v = Value::Bool(true);
                    }
                }
                (b'.', b'.') => {
                    if let Some(v) = env.get_mut("input_dotdot") {
                        *v = Value::Bool(true);
                    }
                }
                _ => {}
            }
        }

        if b.is_ascii_digit() {
            let start = pos;
            while pos < len && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
            let num_str = &s[start..pos];
            current_int = num_str.parse::<u64>().ok();
            if let Some(v) = env.get_mut("input_byte_is_digit") {
                *v = Value::Bool(true);
            }
        } else if b.is_ascii_alphabetic() || b == b'_' {
            let start = pos;
            while pos < len && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_') {
                pos += 1;
            }
            let word_slice = &s[start..pos];
            macro_rules! set_kw {
                ($kw:expr) => {
                    if let Some(v) = env.get_mut(concat!("input_ident_", $kw)) {
                        *v = Value::Bool(true);
                    }
                };
            }
            match word_slice {
                "guard" => set_kw!("guard"),
                "module" => set_kw!("module"),
                "signal" => set_kw!("signal"),
                "reflex" => set_kw!("reflex"),
                "when" => set_kw!("when"),
                "bool" => set_kw!("bool"),
                "true" => set_kw!("true"),
                "false" => set_kw!("false"),
                "else" => set_kw!("else"),
                "loop" => set_kw!("loop"),
                "break" => set_kw!("break"),
                "while" => set_kw!("while"),
                "match" => set_kw!("match"),
                "const" => set_kw!("const"),
                "return" => set_kw!("return"),
                "struct" => set_kw!("struct"),
                "cycles" => set_kw!("cycles"),
                "internal" => set_kw!("internal"),
                _ => {}
            }
        } else {
            pos += 1;
        }

        for (k, v) in env.iter() {
            if let Some(sv) = pools.signal_env.get_mut(k) {
                *sv = v.clone();
            }
        }
        let signal_env = &mut pools.signal_env;
        let guard_active = &mut pools.guard_active;

        // Evaluate guards from ECS
        let mut guard_idx_counter = 0;
        for i in 0..registry.kinds.len() {
            if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::GUARD)) =
                &registry.kinds[i]
            {
                let name_opt = registry.names[i].map(|nc| registry.resolve_name(nc.0));
                let cond_ent_opt = registry.conditions[i].as_ref().map(|c| c.0);

                if let (Some(name), Some(cond_ent)) = (name_opt, cond_ent_opt) {
                    let cond_true = eval_expr_ecs(cond_ent, registry, &|name: &str| {
                        signal_env.get(name).cloned().unwrap_or(Value::Bool(false))
                    })
                    .as_bool();

                    let sr_names = if guard_idx_counter < pools.sr_signal_names.len() {
                        &pools.sr_signal_names[guard_idx_counter]
                    } else {
                        &Vec::new()
                    };

                    if !sr_names.is_empty() {
                        pools.next_vals.clear();
                        for j in 0..sr_names.len() {
                            if j == 0 {
                                pools.next_vals.push(Value::Bool(cond_true));
                            } else {
                                let prev_name = &sr_names[j - 1];
                                let pv = signal_env
                                    .get(prev_name)
                                    .cloned()
                                    .unwrap_or(Value::Bool(false));
                                pools.next_vals.push(pv);
                            }
                        }
                        for (sr_name, val) in sr_names.iter().zip(pools.next_vals.iter()) {
                            if let Some(se) = signal_env.get_mut(sr_name) {
                                *se = val.clone();
                            }
                        }
                        let active = pools.next_vals.iter().any(|v| v.as_bool());
                        if let Some(ga) = guard_active.get_mut(name) {
                            *ga = active;
                        }
                    } else {
                        if cond_true {
                            if let Some(gc) = pools.guard_counters.get_mut(name) {
                                if let Some(cyc) = &registry.cycles[i] {
                                    *gc = cyc.0;
                                }
                            }
                        }
                        let active = *pools.guard_counters.get(name).unwrap_or(&0) > 0;
                        if let Some(ga) = guard_active.get_mut(name) {
                            *ga = active;
                        }
                    }
                }
                guard_idx_counter += 1;
            }
        }

        // Fire reflexes from ECS
        for i in 0..registry.reflex_comps.len() {
            if let Some(reflex) = &registry.reflex_comps[i] {
                let r_name_opt = registry.names[i].map(|nc| registry.resolve_name(nc.0));
                if let Some(r_name) = r_name_opt {
                    if clear_reflex_names_snapshot.iter().any(|n| n == r_name) {
                        continue;
                    }
                    let mut any = false;
                    for g_ent in &reflex.guards {
                        let g_name_opt =
                            registry.names[g_ent.0 as usize].map(|nc| registry.resolve_name(nc.0));
                        if let Some(g_name) = g_name_opt {
                            if *guard_active.get(g_name).unwrap_or(&false) {
                                any = true;
                                break;
                            }
                        }
                    }
                    if any {
                        for asgn_ent in &reflex.assignments {
                            if let Some(asgn) = &registry.assignment_comps[asgn_ent.0 as usize] {
                                let val = eval_expr_ecs(asgn.value, registry, &|name: &str| {
                                    signal_env.get(name).cloned().unwrap_or(Value::Bool(false))
                                });
                                if let Some(target_name) = registry.names[asgn.target.0 as usize]
                                    .map(|nc| registry.resolve_name(nc.0))
                                {
                                    if let Some(sv) = signal_env.get_mut(target_name) {
                                        *sv = val;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Emission and post-tick logic
        const PUSH_KINDS: &[&str] = &[
            "emit_push_integer",
            "emit_push_ident",
            "emit_push_eq_eq",
            "emit_push_excl_eq",
            "emit_push_le",
            "emit_push_ge",
            "emit_push_arrow",
            "emit_push_dot_dot",
            "emit_push_kw_when",
            "emit_push_kw_bool",
            "emit_push_tok_true",
            "emit_push_kw_else",
            "emit_push_kw_loop",
            "emit_push_kw_enum",
            "emit_push_kw_guard",
            "emit_push_tok_false",
            "emit_push_kw_break",
            "emit_push_kw_while",
            "emit_push_kw_match",
            "emit_push_kw_const",
            "emit_push_kw_module",
            "emit_push_kw_signal",
            "emit_push_kw_reflex",
            "emit_push_kw_return",
            "emit_push_kw_struct",
            "emit_push_kw_cycles",
            "emit_push_kw_internal",
        ];
        for pk in PUSH_KINDS.iter().copied() {
            if let Some(Value::Bool(true)) = signal_env.get(pk) {
                out.push(ObservedPush::new(
                    pk,
                    None,
                    if pk == "emit_push_integer" { current_int } else { None },
                ));
            }
        }
        PROBE_PUSH_SAMPLE.fetch_add(1, Ordering::SeqCst);

        // Decrement guard counters
        for i in 0..registry.kinds.len() {
            if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::GUARD)) =
                &registry.kinds[i]
            {
                if let Some(name) = registry.names[i].map(|nc| registry.resolve_name(nc.0)) {
                    if let Some(c) = pools.guard_counters.get_mut(name) {
                        if *c > 0 {
                            *c -= 1;
                        }
                    }
                }
            }
        }

        // Update persistent environment
        for i in 0..registry.kinds.len() {
            if let Some(crate::ecs::components::KindComponent(crate::ecs::EntityKind::SIGNAL(
                SignalKind::Internal,
            ))) = &registry.kinds[i]
            {
                if let Some(name) = registry.names[i].map(|nc| registry.resolve_name(nc.0)) {
                    if let Some(v) = signal_env.get(name) {
                        if let Some(pe) = pools.persistent_env.get_mut(name) {
                            *pe = v.clone();
                        }
                    }
                }
            }
        }
    }

    maybe_hook("after_tick_loop");
    out
}

use crate::mirr_executor::eval::eval_expr_ecs;

/// Drive raw MIRR source bytes through the lexer and interpreter pipeline.
pub fn drive_lexer_with_interpreter(input: &[u8]) -> Vec<ObservedPush> {
    let registry = match load_lexer_registry() {
        Some(r) => r,
        None => {
            return crate::mirr_driver::drive_lexer_from_bytes(input);
        }
    };

    drive_parsed_module_with_interpreter(registry, input)
}
