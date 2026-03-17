//! Preallocated runtime pools used by the MIRR interpreter hot path.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::sync::Arc;

use crate::mirr_runtime::Value;

/// RuntimePools: reusable, preallocated collections used by the interpreter
/// to avoid repeated heap allocations during hot-path execution.
/// - Constructed once per interpreted module run (init-time)
/// - Cleared and reused per tick
/// - Tracks program_fingerprint to detect when a different program is loaded
///   and reinitialize pools accordingly (HIGH-01 fix).
pub(super) struct RuntimePools {
    pub(super) env: HashMap<String, Value>,
    pub(super) signal_env: HashMap<String, Value>,
    pub(super) persistent_env: HashMap<String, Value>,
    pub(super) guard_active: HashMap<String, bool>,
    pub(super) guard_counters: HashMap<String, u64>,
    pub(super) clear_reflex_names: Vec<String>,
    pub(super) clear_reflex_names_snapshot: Arc<Vec<String>>,
    /// Reusable scratch for shift-register next-stage values.
    pub(super) next_vals: Vec<Value>,
    /// Precomputed per-guard ordered shift-register signal names (init-time only).
    pub(super) sr_signal_names: Vec<Vec<String>>,
    /// Pre-collected output signal names for zero-alloc per-tick reset in signal_env.
    pub(super) output_signal_names: Vec<String>,
    /// Fingerprint of the program this pool was initialized for.
    pub(super) program_fingerprint: (usize, usize, usize, usize),
}

impl RuntimePools {
    pub(super) fn new(
        guard_capacity: usize,
        signal_capacity: usize,
        reflex_capacity: usize,
    ) -> Self {
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
    pub(super) fn clear_per_tick(&mut self) {
        for v in self.env.values_mut() {
            *v = Value::Bool(false);
        }
        for v in self.guard_active.values_mut() {
            *v = false;
        }
        for name in &self.output_signal_names {
            if let Some(sv) = self.signal_env.get_mut(name) {
                *sv = Value::Bool(false);
            }
        }
        self.next_vals.clear();
    }
}
