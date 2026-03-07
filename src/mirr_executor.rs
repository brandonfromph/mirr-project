use crate::ast::types::SignalKind;
use crate::ast::{Expr, LiteralValue};
use crate::mirr_driver::ObservedPush;
use crate::mirr_runtime::Value;
use crate::parser::parse_mirr;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

static PROBE_OUTPOOL_TAKEN: AtomicUsize = AtomicUsize::new(0);
static PROBE_PUSH_SAMPLE: AtomicUsize = AtomicUsize::new(0);

/// Minimal executor that parses compiler_mirr/lexer.mirr and executes its
/// guards/reflexes for each input "tick".
///
/// This is a focused, minimal interpreter: it evaluates guard conditions
/// as simple expressions over signal booleans and applies reflex
/// assignments when guards are active. It is intentionally limited to the
/// constructs used by the lexer module (signal reads, boolean literals,
/// simple comparisons).
static LEXER_PROG: OnceLock<Option<crate::ast::MirrProgram>> = OnceLock::new();

fn load_lexer_module() -> Option<&'static crate::ast::MirrProgram> {
    // Cache the parsed lexer program at first call to avoid repeated parse allocations.
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

/// RuntimePools: reusable, preallocated collections used by the interpreter
/// to avoid repeated heap allocations during hot-path execution.
/// - Constructed once per interpreted module run (init-time)
/// - Cleared and reused per tick
/// - Tracks program_fingerprint to detect when a different program is loaded
///   and reinitialize pools accordingly (HIGH-01 fix).
struct RuntimePools {
    env: HashMap<String, Value>,
    signal_env: HashMap<String, Value>,
    persistent_env: HashMap<String, Value>,
    guard_active: HashMap<String, bool>,
    guard_counters: HashMap<String, u64>,
    clear_reflex_names: Vec<String>,
    clear_reflex_names_snapshot: Arc<Vec<String>>,
    /// Reusable scratch for shift-register next-stage values (avoids per-tick Vec alloc).
    next_vals: Vec<Value>,
    /// Precomputed per-guard ordered shift-register signal names (init-time only).
    sr_signal_names: Vec<Vec<String>>,
    /// Pre-collected output signal names for zero-alloc per-tick reset in signal_env.
    output_signal_names: Vec<String>,
    /// Fingerprint of the program this pool was initialized for (module name
    /// length + signal count + guard count + reflex count). When a new program
    /// is loaded with a different shape, pools must be reinitialized. This
    /// prevents stale INPUT_KEYS / guard_counters / sr_signal_names from
    /// causing silent executor/driver divergence (HIGH-01 fix).
    /// Uses lengths only (no String) to avoid hot-path heap allocation.
    program_fingerprint: (usize, usize, usize, usize),
}

impl RuntimePools {
    pub fn new(guard_capacity: usize, signal_capacity: usize, reflex_capacity: usize) -> Self {
        RuntimePools {
            env: HashMap::with_capacity(signal_capacity),
            signal_env: HashMap::with_capacity(signal_capacity),
            persistent_env: HashMap::with_capacity(signal_capacity),
            guard_active: HashMap::with_capacity(guard_capacity),
            guard_counters: HashMap::with_capacity(guard_capacity),
            clear_reflex_names: Vec::with_capacity(reflex_capacity),
            clear_reflex_names_snapshot: Arc::new(Vec::new()),
            next_vals: Vec::new(),
            sr_signal_names: Vec::new(),
            output_signal_names: Vec::new(),
            program_fingerprint: (0, 0, 0, 0),
        }
    }

    /// Clear per-tick transient containers prior to each tick.
    pub fn clear_per_tick(&mut self) {
        // Reset env values to false but keep the pre-seeded INPUT_KEYS so that
        // hot-path `env.get_mut(key)` calls succeed without re-inserting keys.
        // Using clear() would remove all keys, causing get_mut() to return None
        // and preventing input signals from being set during tick evaluation.
        for v in self.env.values_mut() {
            *v = Value::Bool(false);
        }
        // Reset existing guard_active flags in-place to avoid re-inserting keys.
        for v in self.guard_active.values_mut() {
            *v = false;
        }
        // Reset output signals in signal_env to false so stale values from the
        // previous tick do not bleed into the current tick's sampling step.
        for name in &self.output_signal_names {
            if let Some(sv) = self.signal_env.get_mut(name) {
                *sv = Value::Bool(false);
            }
        }
        self.next_vals.clear();
    }
}

fn eval_expr(e: &Expr, env_get: &impl Fn(&str) -> Value) -> Value {
    use crate::ast::expr::Expr as E;
    use crate::ast::types::{BinaryOp, UnaryOp};

    match e {
        E::Literal(LiteralValue::Bool(b)) => Value::Bool(*b),
        E::Literal(LiteralValue::Integer(i)) => Value::Integer(*i),
        E::Signal(name) => env_get(name),
        // Prev references read the signal from a previous tick. In the
        // current executor model, previous-tick state is already in the
        // environment (persisted by the tick loop). Return it directly.
        E::Prev { signal, .. } => env_get(signal),
        E::Unary { op, operand } => {
            let v = eval_expr(operand, env_get);
            match op {
                UnaryOp::Not => Value::Bool(!v.as_bool()),
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
                // Clamp shift amount to 63 to prevent panic on u64 overflow
                // (Rust panics in debug, wraps in release — both wrong for hardware).
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
    }
}

// Execute the MIRR lexer module for the provided input bytes and return
// the observed push events (emit_push_* kinds).
//
// Strategy:
//  - Load and parse compiler_mirr/lexer.mirr
//  - For each input "tick" (driven by a byte or identifier/number token),
//    assert input_* signals in the env, evaluate guards, execute reflexes,
//    sample emit_push_* signals into ObservedPush events, then run the
//    clear reflex to reset outputs before next tick.

// hook code for tests
type AllocHookLock = OnceLock<Mutex<Option<fn(&str)>>>;
static ALLOC_HOOK: AllocHookLock = OnceLock::new();

/// Set callback for allocation checkpoints (tests only).
pub fn set_alloc_hook(h: fn(&str)) {
    let m = ALLOC_HOOK.get_or_init(|| Mutex::new(None));
    // Recover from poisoned Mutex rather than panicking — a previous test
    // panic must not cascade into every subsequent lock() call (HIGH-04 fix).
    let mut guard = m.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(h);
}

fn maybe_hook(label: &str) {
    if let Some(m) = ALLOC_HOOK.get() {
        if let Some(h) = *m.lock().unwrap_or_else(|e| e.into_inner()) {
            h(label);
        }
    }
}

/// Build a fully initialized RuntimePools for the given program (init-time only).
/// Dynamically seeds input signal keys from the program's signal declarations
/// instead of a hardcoded list (HIGH-03 fix). Stores the fingerprint so future
/// calls can detect when a different program is loaded (HIGH-01 fix).
fn init_pools_for_program(
    prog: &crate::ast::MirrProgram,
    fingerprint: (usize, usize, usize, usize),
) -> RuntimePools {
    let mut p = RuntimePools::new(
        prog.module.guards.len(),
        prog.module.signals.len(),
        prog.module.reflexes.len(),
    );
    p.program_fingerprint = fingerprint;

    // Populate clear reflex names.
    for r in &prog.module.reflexes {
        if r.name.contains("clear") || r.name.contains("tick") {
            p.clear_reflex_names.push(r.name.clone());
        }
    }
    // Dynamically seed input signal keys from the parsed program's signal
    // declarations. This replaces the former hardcoded INPUT_KEYS list, ensuring
    // new input signals added to lexer.mirr are automatically recognized (HIGH-03).
    for s in &prog.module.signals {
        if s.kind == crate::ast::types::SignalKind::Input {
            p.env.insert(s.name.clone(), Value::Bool(false));
        }
    }
    p.clear_reflex_names_snapshot = Arc::new(p.clear_reflex_names.clone());

    // Initialize guard counters and active map.
    // SAFETY: These maps are freshly constructed (init-time only). Using clear()
    // here is harmless but explicit. Do NOT move this logic to a per-tick path —
    // clear() would nuke pre-seeded keys (MED-02 note).
    p.guard_counters.clear();
    for g in &prog.module.guards {
        p.guard_counters.insert(g.name.clone(), 0);
        p.guard_active.insert(g.name.clone(), false);
    }
    // Persistent signal state for all signals.
    // SAFETY: clear() on freshly constructed map — see MED-02 note above.
    p.persistent_env.clear();
    for s in &prog.module.signals {
        match s.ty {
            crate::ast::types::SignalType::Bool => {
                p.persistent_env.insert(s.name.clone(), Value::Bool(false));
            }
            crate::ast::types::SignalType::Unsigned(_) => {
                p.persistent_env.insert(s.name.clone(), Value::Integer(0));
            }
        }
    }
    p.signal_env = p.persistent_env.clone();

    // Collect output signal names for zero-alloc per-tick reset.
    p.output_signal_names.clear();
    for s in &prog.module.signals {
        if s.kind == crate::ast::types::SignalKind::Output {
            p.output_signal_names.push(s.name.clone());
        }
    }

    // Precompute shift-register signal lists per guard.
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
    // Preallocate a larger output buffer to avoid Vec growth/reallocations on hot-path.
    let initial_out_cap =
        std::cmp::max((prog.module.reflexes.len().max(8)).saturating_mul(4), 1024);

    // Use a pool of preallocated output Vecs protected by a Mutex.
    // Instead of popping the buffer (which can drain the pool and trigger warm-up
    // allocations on subsequent calls), move the last slot out with mem::take.
    // This preserves the pool length (avoids the guard.is_empty() path) while
    // transferring ownership of an already-allocated Vec to the caller.
    maybe_hook("before_out_pool_init");
    static OUT_POOL: OnceLock<Mutex<Vec<Vec<ObservedPush>>>> = OnceLock::new();
    let pool = OUT_POOL.get_or_init(|| Mutex::new(Vec::new()));
    maybe_hook("after_out_pool_init");
    let mut out: Vec<ObservedPush> = {
        let mut guard = pool.lock().unwrap_or_else(|e| e.into_inner());
        if guard.is_empty() {
            // First-time warm-up: create multiple preallocated buffers (init-time allocations allowed).
            for _ in 0..8 {
                guard.push(Vec::with_capacity(initial_out_cap));
            }
        }
        // Prefer taking an already-allocated slot (capacity >= initial_out_cap)
        // to avoid selecting a previously-drained empty slot which would trigger
        // a heap allocation when pushed into.
        let idx = guard
            .iter()
            .position(|v| v.capacity() >= initial_out_cap)
            .unwrap_or_else(|| guard.len() - 1);
        std::mem::take(&mut guard[idx])
    };
    // Probe: record that we took an out-pool buffer
    PROBE_OUTPOOL_TAKEN.fetch_add(1, Ordering::SeqCst);

    // Prepare reusable runtime pools and initialize them once during the warm-up.
    static POOLS: OnceLock<Mutex<RuntimePools>> = OnceLock::new();

    // Compute a fingerprint for the current program to detect when a different
    // module is loaded and pools need reinitialization (HIGH-01 fix). A static
    // list of INPUT_KEYS is no longer used — input signal keys are dynamically
    // seeded from prog.module.signals (HIGH-03 fix).
    // Uses only Copy-type lengths to avoid hot-path heap allocation.
    let current_fingerprint = (
        prog.module.name.len(),
        prog.module.signals.len(),
        prog.module.guards.len(),
        prog.module.reflexes.len(),
    );

    maybe_hook("before_pools_init");
    let pools_mutex = POOLS.get_or_init(|| {
        // Init-time allocations are allowed here (warm-up).
        // Build pools using the full init_pools helper so that the first call
        // completes all allocation inside get_or_init (zero-alloc on subsequent calls).
        Mutex::new(init_pools_for_program(prog, current_fingerprint))
    });

    // Lock pools with poison recovery (HIGH-04 fix).
    let mut pools_guard = pools_mutex.lock().unwrap_or_else(|e| e.into_inner());
    maybe_hook("after_pools_lock");

    // If the pools were initialized for a different program (or this is a subsequent
    // call with a new module shape), reinitialize. This path only runs when the
    // program shape genuinely changes — not on the normal hot path (HIGH-01 fix).
    if pools_guard.program_fingerprint != current_fingerprint {
        *pools_guard = init_pools_for_program(prog, current_fingerprint);
    }

    let pools = &mut *pools_guard;
    // Clone clear reflex names Arc (cheap) so we don't hold an immutable borrow on `pools`.
    let clear_reflex_names_snapshot = pools.clear_reflex_names_snapshot.clone();
    maybe_hook("before_tick_loop");
    while pos < len {
        let b = bytes[pos];

        // Precompute input detection (same heuristics as emulator).
        let is_whitespace = b == b' ' || b == b'\t' || b == b'\n' || b == b'\r';
        if is_whitespace {
            pos += 1;
            continue;
        }

        // Track current token text / integer value for assigning to pushes.
        let mut _current_ident: Option<&str> = None;
        let mut current_int: Option<u64> = None;

        // Determine which input_* signals would be asserted for this tick.
        // We'll set them in `env` before evaluating guards.
        pools.clear_per_tick();
        let env = &mut pools.env;

        // Detect two-char ops using byte comparisons (safe for multi-byte UTF-8 chars).
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

        // Digits (integer)
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
            // The lexer expects emit_push_integer reflex when digit_guard true.
        }
        // Identifiers / keywords
        else if b.is_ascii_alphabetic() || b == b'_' {
            let start = pos;
            while pos < len && (bytes[pos].is_ascii_alphanumeric() || bytes[pos] == b'_') {
                pos += 1;
            }
            let word_slice = &s[start..pos];
            _current_ident = Some(word_slice);
            // classify length classes (use slice length to avoid allocation)
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
            // specific keywords
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
            // input_byte_is_whitespace remains false here
        }
        // single-char fallback
        else {
            pos += 1;
        }

        // Evaluate guards (except tick_guard) and apply reflexes.
        // Initialize all signals to false unless set in env.
        // We'll collect emitted push signals by looking at the signal names
        // the reflex assignments write to.
        // Start from persistent internal state, then overlay detected inputs.
        // Clone persistent snapshot into the per-tick signal_env (reuse preallocated map).
        // Overlay detected input values into the preinitialized signal_env by
        // updating existing keys in-place (no key cloning).
        for (k, v) in env.iter() {
            if let Some(sv) = pools.signal_env.get_mut(k) {
                *sv = v.clone();
            }
        }
        let signal_env = &mut pools.signal_env;
        // Evaluate guards with 'for N cycles' semantics.
        // Support two lowered implementations:
        //  - Shift-register guards (presence of generated "<guard>_sr_<i>" signals)
        //    implement by shifting condition bits into stage[0] and propagating;
        //    the guard is active while any stage is true.
        //  - Counter guards (fallback) use guard_counters to track remaining cycles.
        // Do NOT clear the guard_active map here — it was prepopulated at init-time
        // and cleared per-tick by RuntimePools::clear_per_tick to avoid reallocations.
        let guard_active = &mut pools.guard_active;
        for (g_idx, g) in prog.module.guards.iter().enumerate() {
            let cond_true = eval_expr(&g.condition, &|name: &str| {
                signal_env.get(name).cloned().unwrap_or(Value::Bool(false))
            })
            .as_bool();

            // Use precomputed shift-register signal names (per-guard) to avoid per-tick allocations.
            let sr_names = if g_idx < pools.sr_signal_names.len() {
                &pools.sr_signal_names[g_idx]
            } else {
                &Vec::new()
            };

            if !sr_names.is_empty() {
                // Build next-stage values by shifting previous stages using reusable scratch.
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
                // Write shifted values into the signal_env so reflex evaluation can observe them.
                for (name, val) in sr_names.iter().zip(pools.next_vals.iter()) {
                    if let Some(se) = signal_env.get_mut(name) {
                        *se = val.clone();
                    }
                }
                // Guard is active while any stage is true.
                let active = pools.next_vals.iter().any(|v| v.as_bool());
                if let Some(ga) = guard_active.get_mut(&g.name) {
                    *ga = active;
                }
            } else {
                // Counter-based semantics (existing behavior).
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

        // Apply reflexes whose guard_names are active and not a clear/tick reflex.
        for r in &prog.module.reflexes {
            // skip clear/tick reflexes for now
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

        // Sample emit_push_* signals from signal_env and produce ObservedPushes.
        // We also attach ident/int payloads when available.
        // Use a static slice to avoid repeated allocations.
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
                        // Do not allocate ident strings on the hot path; omit ident payload here.
                        out.push(ObservedPush::new(pk, None, None));
                    }
                }
            }
        }
        // Probe: record push sampling occurrences
        PROBE_PUSH_SAMPLE.fetch_add(1, Ordering::SeqCst);

        // Finally, run clear reflexes (tick) to reset outputs before next tick.
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

        // Decrement guard counters to consume one cycle per tick.
        for g in &prog.module.guards {
            if let Some(c) = pools.guard_counters.get_mut(&g.name) {
                if *c > 0 {
                    *c -= 1;
                }
            }
        }

        // Persist internal signal values back into persistent_env so they
        // carry over to the next tick.
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

pub fn drive_lexer_with_interpreter(input: &[u8]) -> Vec<ObservedPush> {
    let prog = match load_lexer_module() {
        Some(p) => p,
        None => {
            // Fallback to emulator when parsing the MIRR module fails.
            return crate::mirr_driver::drive_lexer_from_bytes(input);
        }
    };

    drive_parsed_module_with_interpreter(prog, input)
}
