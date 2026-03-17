//! MIRR signal evaluator and interpreter engine.
//!
//! Drives the execution of parsed MIRR modules by evaluating guard conditions,
//! firing reflexes, and updating signal state. Used by the MAPE-K simulation harness.

#![forbid(unsafe_code)]

mod eval;
mod pools;

pub use eval::set_alloc_hook;
use eval::{eval_expr, init_pools_for_program, maybe_hook};
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

static LEXER_PROG: OnceLock<Option<crate::ast::MirrProgram>> = OnceLock::new();

fn load_lexer_module() -> Option<&'static crate::ast::MirrProgram> {
    LEXER_PROG.get_or_init(|| {
        let path = Path::new("compiler_mirr").join("lexer.mirr");
        let txt = fs::read_to_string(&path).ok()?;
        parse_mirr(&txt).ok()
    });
    if let Some(opt) = LEXER_PROG.get() {
        return opt.as_ref();
    }
    None
}

/// Drive a parsed MIRR module through the interpreter, evaluating guards and firing reflexes.
pub fn drive_parsed_module_with_interpreter(
    prog: &crate::ast::MirrProgram,
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
    let initial_out_cap =
        std::cmp::max((prog.module.reflexes.len().max(8)).saturating_mul(4), 1024);

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

    let current_fingerprint = (
        prog.module.name.len(),
        prog.module.signals.len(),
        prog.module.guards.len(),
        prog.module.reflexes.len(),
    );

    maybe_hook("before_pools_init");
    let pools_mutex =
        POOLS.get_or_init(|| Mutex::new(init_pools_for_program(prog, current_fingerprint)));

    let mut pools_guard = pools_mutex.lock().unwrap_or_else(|e| e.into_inner());
    maybe_hook("after_pools_lock");

    if pools_guard.program_fingerprint != current_fingerprint {
        *pools_guard = init_pools_for_program(prog, current_fingerprint);
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

        let mut _current_ident: Option<&str> = None;
        let mut current_int: Option<u64> = None;

        pools.clear_per_tick();
        let env = &mut pools.env;

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
            if let Ok(v) = num_str.parse::<u64>() {
                current_int = Some(v);
            } else {
                current_int = Some(0);
            }
            if let Some(v) = env.get_mut("input_byte_is_digit") {
                *v = Value::Bool(true);
            }
        } else if b.is_ascii_alphabetic() || b == b'_' {
            let start = pos;
            while pos < len && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_') {
                pos += 1;
            }
            let word_slice = &s[start..pos];
            _current_ident = Some(word_slice);
            let l = word_slice.len();
            if l == 5 {
                if let Some(v) = env.get_mut("input_ident_len5") {
                    *v = Value::Bool(true);
                }
            } else if l == 6 {
                if let Some(v) = env.get_mut("input_ident_len6") {
                    *v = Value::Bool(true);
                }
            } else if l == 8 {
                if let Some(v) = env.get_mut("input_ident_len8") {
                    *v = Value::Bool(true);
                }
            }
            match word_slice {
                "guard" => {
                    if let Some(v) = env.get_mut("input_ident_guard") {
                        *v = Value::Bool(true);
                    }
                }
                "false" => {
                    if let Some(v) = env.get_mut("input_ident_false") {
                        *v = Value::Bool(true);
                    }
                }
                "break" => {
                    if let Some(v) = env.get_mut("input_ident_break") {
                        *v = Value::Bool(true);
                    }
                }
                "while" => {
                    if let Some(v) = env.get_mut("input_ident_while") {
                        *v = Value::Bool(true);
                    }
                }
                "match" => {
                    if let Some(v) = env.get_mut("input_ident_match") {
                        *v = Value::Bool(true);
                    }
                }
                "const" => {
                    if let Some(v) = env.get_mut("input_ident_const") {
                        *v = Value::Bool(true);
                    }
                }
                "module" => {
                    if let Some(v) = env.get_mut("input_ident_module") {
                        *v = Value::Bool(true);
                    }
                }
                "signal" => {
                    if let Some(v) = env.get_mut("input_ident_signal") {
                        *v = Value::Bool(true);
                    }
                }
                "reflex" => {
                    if let Some(v) = env.get_mut("input_ident_reflex") {
                        *v = Value::Bool(true);
                    }
                }
                "return" => {
                    if let Some(v) = env.get_mut("input_ident_return") {
                        *v = Value::Bool(true);
                    }
                }
                "struct" => {
                    if let Some(v) = env.get_mut("input_ident_struct") {
                        *v = Value::Bool(true);
                    }
                }
                "cycles" => {
                    if let Some(v) = env.get_mut("input_ident_cycles") {
                        *v = Value::Bool(true);
                    }
                }
                "internal" => {
                    if let Some(v) = env.get_mut("input_ident_internal") {
                        *v = Value::Bool(true);
                    }
                }
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
        for (g_idx, g) in prog.module.guards.iter().enumerate() {
            let cond_true = eval_expr(&g.condition, &|name: &str| {
                signal_env.get(name).cloned().unwrap_or(Value::Bool(false))
            })
            .as_bool();

            let sr_names = if g_idx < pools.sr_signal_names.len() {
                &pools.sr_signal_names[g_idx]
            } else {
                &Vec::new()
            };

            if !sr_names.is_empty() {
                pools.next_vals.clear();
                for i in 0..sr_names.len() {
                    if i == 0 {
                        pools.next_vals.push(Value::Bool(cond_true));
                    } else {
                        let prev_name = &sr_names[i - 1];
                        let pv = signal_env.get(prev_name).cloned().unwrap_or(Value::Bool(false));
                        pools.next_vals.push(pv);
                    }
                }
                for (name, val) in sr_names.iter().zip(pools.next_vals.iter()) {
                    if let Some(se) = signal_env.get_mut(name) {
                        *se = val.clone();
                    }
                }
                let active = pools.next_vals.iter().any(|v| v.as_bool());
                if let Some(ga) = guard_active.get_mut(&g.name) {
                    *ga = active;
                }
            } else {
                if cond_true {
                    if let Some(gc) = pools.guard_counters.get_mut(&g.name) {
                        *gc = g.cycles;
                    }
                }
                let active = *pools.guard_counters.get(&g.name).unwrap_or(&0) > 0;
                if let Some(ga) = guard_active.get_mut(&g.name) {
                    *ga = active;
                } else {
                    guard_active.insert(g.name.clone(), active);
                }
            }
        }

        for r in &prog.module.reflexes {
            if clear_reflex_names_snapshot.contains(&r.name) {
                continue;
            }
            let mut any = false;
            for gn in &r.guard_names {
                if *guard_active.get(gn).unwrap_or(&false) {
                    any = true;
                    break;
                }
            }
            if any {
                for a in &r.assignments {
                    let val = eval_expr(&a.value, &|name: &str| {
                        signal_env.get(name).cloned().unwrap_or(Value::Bool(false))
                    });
                    if let Some(sv) = signal_env.get_mut(&a.target) {
                        *sv = val;
                    }
                }
            }
        }

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
                match pk {
                    "emit_push_integer" => out.push(ObservedPush::new(pk, None, current_int)),
                    _ => {
                        out.push(ObservedPush::new(pk, None, None));
                    }
                }
            }
        }
        PROBE_PUSH_SAMPLE.fetch_add(1, Ordering::SeqCst);

        for r in &prog.module.reflexes {
            if !clear_reflex_names_snapshot.contains(&r.name) {
                continue;
            }
            let mut any = false;
            for gn in &r.guard_names {
                if *guard_active.get(gn).unwrap_or(&false) {
                    any = true;
                    break;
                }
            }
            if any {
                for a in &r.assignments {
                    let val = eval_expr(&a.value, &|name: &str| {
                        signal_env.get(name).cloned().unwrap_or(Value::Bool(false))
                    });
                    if let Some(sv) = signal_env.get_mut(&a.target) {
                        *sv = val;
                    }
                }
            }
        }

        for g in &prog.module.guards {
            if let Some(c) = pools.guard_counters.get_mut(&g.name) {
                if *c > 0 {
                    *c -= 1;
                }
            }
        }

        for s in &prog.module.signals {
            if s.kind == SignalKind::Internal {
                if let Some(v) = signal_env.get(&s.name) {
                    if let Some(pe) = pools.persistent_env.get_mut(&s.name) {
                        *pe = v.clone();
                    }
                }
            }
        }
    }

    maybe_hook("after_tick_loop");
    out
}

/// Drive raw MIRR source bytes through the lexer and interpreter pipeline.
pub fn drive_lexer_with_interpreter(input: &[u8]) -> Vec<ObservedPush> {
    let prog = match load_lexer_module() {
        Some(p) => p,
        None => {
            return crate::mirr_driver::drive_lexer_from_bytes(input);
        }
    };

    drive_parsed_module_with_interpreter(prog, input)
}
