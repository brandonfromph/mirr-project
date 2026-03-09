//! Minimal MIRR runtime harness scaffolding.
//!
//! Provides Rust-side helpers to map lexer "push" events (from
//! `compiler_mirr/lexer.mirr` `emit_push_*` signals) into host `Token` values.
//! Unknown push kinds are converted into `Ident` tokens for test observability.

#![forbid(unsafe_code)]

use crate::lexer::tokenizer::Token;

/// Map a lexer push-kind (as observed from MIRR emit_push_* signals)
/// to a host Token value. `ident_text` is used when the push corresponds to an
/// identifier-like push; `int_val` is used for integer pushes.
///
/// This helper is intentionally conservative: unknown kinds become
/// Token::Ident(push_kind.to_string()) so parity tests can inspect them.
pub fn map_push_kind_to_token(
    push_kind: &str,
    ident_text: Option<&str>,
    int_val: Option<u64>,
) -> Token {
    match push_kind {
        "integer" | "push_integer" | "emit_push_integer" => Token::Integer(int_val.unwrap_or(0)),
        "ident" | "push_ident" | "emit_push_ident" => match ident_text {
            Some(s) => Token::Ident(s.to_string()),
            None => Token::Ident("".to_string()),
        },
        "eq_eq" | "push_eq_eq" | "emit_push_eq_eq" => Token::EqEq,
        "bang_eq" | "excl_eq" | "push_excl_eq" | "emit_push_excl_eq" => Token::BangEq,
        "le" | "push_le" | "emit_push_le" => Token::Le,
        "ge" | "push_ge" | "emit_push_ge" => Token::Ge,
        // Map arrow/dotdot to an identifier token so tests can observe them,
        // until a dedicated Token variant is added for them.
        "arrow" | "push_arrow" | "emit_push_arrow" => Token::Ident("->".to_string()),
        "dot_dot" | "dotdot" | "push_dot_dot" | "emit_push_dot_dot" => {
            Token::Ident("..".to_string())
        }
        // Keywords
        "kw_when" | "push_kw_when" | "emit_push_kw_when" => Token::Ident("when".to_string()),
        "kw_bool" | "push_kw_bool" | "emit_push_kw_bool" => Token::Ident("bool".to_string()),
        "tok_true" | "push_tok_true" | "emit_push_tok_true" => Token::True,
        "kw_else" | "push_kw_else" | "emit_push_kw_else" => Token::Ident("else".to_string()),
        "kw_loop" | "push_kw_loop" | "emit_push_kw_loop" => Token::Ident("loop".to_string()),
        "kw_enum" | "push_kw_enum" | "emit_push_kw_enum" => Token::Ident("enum".to_string()),
        other => Token::Ident(other.to_string()),
    }
}

/// Token buffer capacity (matches stdlib constant).
pub const TOKEN_BUFFER_CAPACITY: usize = 8192;

/// Simple TokenBuffer struct mirroring the stdlib token buffer.
pub struct TokenBuffer {
    tokens: Vec<Token>,
}

impl Default for TokenBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenBuffer {
    /// Create a new token buffer.
    pub fn new() -> Self {
        Self { tokens: Vec::with_capacity(TOKEN_BUFFER_CAPACITY) }
    }

    /// Return number of tokens currently stored.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Returns true if no tokens are buffered.
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

/// Append a mapped token into the provided TokenBuffer and return true.
/// Returns false if the buffer is full.
pub fn token_buffer_push(buf: &mut TokenBuffer, tok: Token) -> bool {
    if buf.tokens.len() >= TOKEN_BUFFER_CAPACITY {
        false
    } else {
        buf.tokens.push(tok);
        true
    }
}

/// Keep the existing helper for `Vec<Token>` for backwards compatibility/tests.
pub fn push_mapped_token(
    vec: &mut Vec<Token>,
    push_kind: &str,
    ident_text: Option<&str>,
    int_val: Option<u64>,
) -> bool {
    let tok = map_push_kind_to_token(push_kind, ident_text, int_val);
    vec.push(tok);
    true
}

/// Map a lexer push-kind and append into a TokenBuffer (preferred host API).
pub fn push_mapped_token_to_buffer(
    buf: &mut TokenBuffer,
    push_kind: &str,
    ident_text: Option<&str>,
    int_val: Option<u64>,
) -> bool {
    let tok = map_push_kind_to_token(push_kind, ident_text, int_val);
    token_buffer_push(buf, tok)
}

// ---------------------------------------------------------------------------
// Phase 3: Runtime pool types
// ---------------------------------------------------------------------------
// Provide shared Value and RuntimePools types here so runtime-wide pools can be
// allocated at initialization time and reused by hot-path modules such as the
// executor. Fields are public for direct, efficient access by consumers.
// ---------------------------------------------------------------------------
use std::collections::HashMap;

/// A runtime signal value (boolean or integer).
#[derive(Clone, Debug)]
pub enum Value {
    /// Boolean signal value.
    Bool(bool),
    /// Unsigned integer signal value.
    Integer(u64),
}

impl Value {
    /// Coerce to boolean (integers: nonzero = true).
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Integer(i) => *i != 0,
        }
    }

    /// Coerce to integer (booleans: true = 1, false = 0).
    pub fn as_int(&self) -> u64 {
        match self {
            Value::Integer(i) => *i,
            Value::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
        }
    }
}

/// RuntimePools: preallocated, index-based storage for hot-path values.
///
/// Replaces per-tick `HashMap<String,Value>` lookups with index-based `Vec<Value>`
/// accesses. The `index_map` is populated at init-time (allowed). All hot-path
/// reads/writes use indices into preallocated Vecs avoiding further heap ops.
pub struct RuntimePools {
    /// Map from signal name -> index (init-time populated).
    pub index_map: HashMap<String, usize>,

    /// Per-tick mutable signal values (overlay of persistent + inputs).
    pub signal_vals: Vec<Value>,

    /// Persistent signal storage (survives ticks).
    pub persistent_vals: Vec<Value>,

    /// Guard active flags indexed by guard index (guard order == prog.module.guards).
    pub guard_active: Vec<bool>,

    /// Guard counters (for 'for N cycles' lowering).
    pub guard_counters: Vec<u64>,

    /// Reflex names that are clear/tick reflexes.
    pub clear_reflex_names: Vec<String>,

    /// Scratch buffers reused across ticks for shift-register lowering, etc.
    pub sr_pairs: Vec<(usize, String)>,
    /// Scratch buffer for next-tick signal values.
    pub next_vals: Vec<Value>,
}

impl RuntimePools {
    /// Create a new pool with preallocated capacity. Call at init time only.
    pub fn new(guard_capacity: usize, signal_capacity: usize, reflex_capacity: usize) -> Self {
        RuntimePools {
            index_map: HashMap::with_capacity(signal_capacity),
            signal_vals: Vec::with_capacity(signal_capacity),
            persistent_vals: Vec::with_capacity(signal_capacity),
            guard_active: vec![false; guard_capacity],
            guard_counters: vec![0u64; guard_capacity],
            clear_reflex_names: Vec::with_capacity(reflex_capacity),
            sr_pairs: Vec::with_capacity(16),
            next_vals: Vec::with_capacity(16),
        }
    }

    /// Populate index_map for the provided signal names and allocate/resize
    /// backing vectors to the correct length. This is an init-time operation.
    pub fn init_signal_index_map(&mut self, signal_names: &[String]) {
        self.index_map.clear();
        for (i, name) in signal_names.iter().enumerate() {
            self.index_map.insert(name.clone(), i);
        }
        let n = signal_names.len();
        self.signal_vals.resize(n, Value::Bool(false));
        self.persistent_vals.resize(n, Value::Bool(false));
    }

    /// Get a snapshot value from the current per-tick signal values (by name).
    pub fn get_signal(&self, name: &str) -> Value {
        if let Some(&i) = self.index_map.get(name) {
            self.signal_vals.get(i).cloned().unwrap_or(Value::Bool(false))
        } else {
            Value::Bool(false)
        }
    }

    /// Set a per-tick signal value by name (no allocation).
    pub fn set_signal(&mut self, name: &str, val: Value) {
        if let Some(&i) = self.index_map.get(name) {
            if i < self.signal_vals.len() {
                self.signal_vals[i] = val;
            }
        }
    }

    /// Clear per-tick transient containers prior to each tick.
    ///
    /// IMPORTANT: This resets signal_vals to defaults. Callers MUST restore
    /// persistent signal values from persistent_vals after calling this method,
    /// otherwise internal signals that should survive across ticks will be lost.
    /// The mirr_executor.rs RuntimePools.clear_per_tick() handles this correctly
    /// by only resetting output signals and preserving persistent_env separately.
    pub fn clear_per_tick(&mut self, output_indices: &[usize]) {
        // Only reset output signal values to avoid nuking persistent internal
        // signal state (HIGH-02 fix). Previously this reset ALL signal_vals to
        // Bool(false), which would destroy internal signal persistence.
        for &idx in output_indices {
            if idx < self.signal_vals.len() {
                self.signal_vals[idx] = Value::Bool(false);
            }
        }
        for b in &mut self.guard_active {
            *b = false;
        }
        // guard_counters are persistent across ticks; do not reset here.
        self.sr_pairs.clear();
        self.next_vals.clear();
    }
}

/// RuntimeHandle: a thin container that owns runtime-wide pools. Construct
/// a RuntimeHandle at initialization time and reuse it for hot-path modules.
/// This keeps ownership clear and prepares for future shared/global pool use.
pub struct RuntimeHandle {
    /// Preallocated runtime pools for signal, guard, and scratch storage.
    pub pools: RuntimePools,
}

impl RuntimeHandle {
    /// Create a new runtime handle with preallocated pools sized to the
    /// provided worst-case counts. This should be called during module/runtime
    /// initialization (not per tick).
    pub fn new(guard_capacity: usize, signal_capacity: usize, reflex_capacity: usize) -> Self {
        RuntimeHandle { pools: RuntimePools::new(guard_capacity, signal_capacity, reflex_capacity) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenizer::Token;

    #[test]
    fn map_integer_push() {
        let t = map_push_kind_to_token("emit_push_integer", None, Some(42));
        assert_eq!(t, Token::Integer(42));
    }

    #[test]
    fn map_ident_push() {
        let t = map_push_kind_to_token("emit_push_ident", Some("hello"), None);
        assert_eq!(t, Token::Ident("hello".to_string()));
    }

    #[test]
    fn push_mapped_token_appends() {
        let mut v: Vec<Token> = Vec::new();
        let ok = push_mapped_token(&mut v, "emit_push_tok_true", None, None);
        assert!(ok);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], Token::True);
    }

    #[test]
    fn token_buffer_push_overflow() {
        let mut buf = TokenBuffer::new();
        // Fill to capacity
        for _ in 0..TOKEN_BUFFER_CAPACITY {
            let ok = push_mapped_token_to_buffer(&mut buf, "emit_push_ident", Some("x"), None);
            assert!(ok);
        }
        // Buffer should be full now; further pushes must fail and length must remain equal to capacity.
        let ok = push_mapped_token_to_buffer(&mut buf, "emit_push_ident", Some("y"), None);
        assert!(!ok, "Expected push to fail when buffer is full");
        assert_eq!(buf.len(), TOKEN_BUFFER_CAPACITY);
    }

    #[test]
    fn token_buffer_capacity_constant() {
        // Ensure capacity constant matches expected value used by stdlib and tests.
        assert_eq!(TOKEN_BUFFER_CAPACITY, 8192);
    }
}
