//! Executor: applies planner-selected adaptation actions to the system's
//! signal state.
//!
//! The executor is the "E" in MAPE-K. It:
//! 1. Receives a selected action from the planner.
//! 2. Applies the action to the signal environment (signal_env map).
//! 3. Records before/after snapshots for the knowledge base.
//!
//! In the R-SPU vision, the executor maps to:
//! - SetSignal -> writing to hardware signal registers
//! - SwitchMode -> DPR partial bitstream loading
//! - EmergencyStop -> static safety clamp (single-cycle)
//!
//! No heap allocation in the execution path beyond the snapshot capture.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::planner::AdaptationAction;

// ---------------------------------------------------------------------------
// Execution result
// ---------------------------------------------------------------------------

/// Record of what the executor did on a single tick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    /// The action that was applied.
    pub action: AdaptationAction,
    /// Signal values before the action was applied (relevant signals only).
    pub pre_state: Vec<(String, u64)>,
    /// Signal values after the action was applied.
    pub post_state: Vec<(String, u64)>,
    /// Whether the action was successfully applied.
    pub success: bool,
    /// Error message if the action failed.
    pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Executor
// ---------------------------------------------------------------------------

/// The Executor component of the MAPE-K loop.
///
/// Applies adaptation actions to a mutable signal environment.
/// The signal environment is a `HashMap<String, u64>` matching
/// the MIRR executor's signal_env pattern.
#[derive(Debug, Clone)]
pub struct Executor {
    /// Set of valid signal names (from module declarations).
    /// Used to validate action targets before application.
    valid_signals: Vec<String>,
    /// Whether an emergency stop is currently active.
    emergency_active: bool,
}

impl Executor {
    /// Create an executor that knows about the given signal names.
    pub fn new(valid_signals: Vec<String>) -> Self {
        Self {
            valid_signals,
            emergency_active: false,
        }
    }

    /// Whether an emergency stop is currently active.
    pub fn is_emergency_active(&self) -> bool {
        self.emergency_active
    }

    /// Clear the emergency stop state (e.g., after manual reset).
    pub fn clear_emergency(&mut self) {
        self.emergency_active = false;
    }

    /// Apply an adaptation action to the signal environment.
    ///
    /// `signal_env` is the mutable signal state map (signal name -> value).
    /// Returns an `ExecutionRecord` documenting what happened.
    pub fn apply(
        &mut self,
        action: &AdaptationAction,
        signal_env: &mut HashMap<String, u64>,
    ) -> ExecutionRecord {
        match action {
            AdaptationAction::SetSignal { name, value } => {
                self.apply_set_signal(name, *value, signal_env)
            }
            AdaptationAction::SwitchMode { mode_name } => {
                self.apply_switch_mode(mode_name, signal_env)
            }
            AdaptationAction::EmergencyStop => {
                self.apply_emergency_stop(signal_env)
            }
        }
    }

    fn apply_set_signal(
        &self,
        name: &str,
        value: u64,
        signal_env: &mut HashMap<String, u64>,
    ) -> ExecutionRecord {
        // Validate that the target signal exists.
        if !self.valid_signals.iter().any(|s| s == name) {
            return ExecutionRecord {
                action: AdaptationAction::SetSignal {
                    name: name.to_string(),
                    value,
                },
                pre_state: Vec::new(),
                post_state: Vec::new(),
                success: false,
                error: Some(format!("unknown signal '{name}'")),
            };
        }

        let pre_val = signal_env.get(name).copied().unwrap_or(0);
        signal_env.insert(name.to_string(), value);
        let post_val = value;

        ExecutionRecord {
            action: AdaptationAction::SetSignal {
                name: name.to_string(),
                value,
            },
            pre_state: vec![(name.to_string(), pre_val)],
            post_state: vec![(name.to_string(), post_val)],
            success: true,
            error: None,
        }
    }

    fn apply_switch_mode(
        &self,
        mode_name: &str,
        _signal_env: &mut HashMap<String, u64>,
    ) -> ExecutionRecord {
        // In simulation, mode switching is recorded but signal state
        // is not directly modified (the "bitstream" concept is simulated
        // by the orchestrator loading a different configuration).
        ExecutionRecord {
            action: AdaptationAction::SwitchMode {
                mode_name: mode_name.to_string(),
            },
            pre_state: Vec::new(),
            post_state: Vec::new(),
            success: true,
            error: None,
        }
    }

    fn apply_emergency_stop(
        &mut self,
        signal_env: &mut HashMap<String, u64>,
    ) -> ExecutionRecord {
        self.emergency_active = true;

        // Capture pre-state for all output signals, then zero them.
        let mut pre_state = Vec::with_capacity(signal_env.len());
        let mut post_state = Vec::with_capacity(signal_env.len());

        for name in &self.valid_signals {
            if let Some(val) = signal_env.get(name) {
                pre_state.push((name.clone(), *val));
            }
        }

        // Set all signals to 0 (safe default).
        for name in &self.valid_signals {
            signal_env.insert(name.clone(), 0);
            post_state.push((name.clone(), 0));
        }

        ExecutionRecord {
            action: AdaptationAction::EmergencyStop,
            pre_state,
            post_state,
            success: true,
            error: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_env() -> HashMap<String, u64> {
        let mut env = HashMap::new();
        env.insert("pressure".to_string(), 120);
        env.insert("clamp".to_string(), 0);
        env
    }

    fn make_executor() -> Executor {
        Executor::new(vec!["pressure".to_string(), "clamp".to_string()])
    }

    #[test]
    fn set_signal_success() {
        let mut exec = make_executor();
        let mut env = make_env();
        let action = AdaptationAction::SetSignal {
            name: "clamp".to_string(),
            value: 1,
        };
        let rec = exec.apply(&action, &mut env);
        assert!(rec.success);
        assert_eq!(env["clamp"], 1);
        assert_eq!(rec.pre_state[0].1, 0);
        assert_eq!(rec.post_state[0].1, 1);
    }

    #[test]
    fn set_signal_unknown_fails() {
        let mut exec = make_executor();
        let mut env = make_env();
        let action = AdaptationAction::SetSignal {
            name: "nonexistent".to_string(),
            value: 1,
        };
        let rec = exec.apply(&action, &mut env);
        assert!(!rec.success);
        assert!(rec.error.unwrap().contains("unknown signal"));
    }

    #[test]
    fn emergency_stop_zeros_all() {
        let mut exec = make_executor();
        let mut env = make_env();
        let rec = exec.apply(&AdaptationAction::EmergencyStop, &mut env);
        assert!(rec.success);
        assert!(exec.is_emergency_active());
        assert_eq!(env["pressure"], 0);
        assert_eq!(env["clamp"], 0);
    }

    #[test]
    fn clear_emergency() {
        let mut exec = make_executor();
        let mut env = make_env();
        exec.apply(&AdaptationAction::EmergencyStop, &mut env);
        assert!(exec.is_emergency_active());
        exec.clear_emergency();
        assert!(!exec.is_emergency_active());
    }

    #[test]
    fn switch_mode_records() {
        let mut exec = make_executor();
        let mut env = make_env();
        let action = AdaptationAction::SwitchMode {
            mode_name: "high_precision".to_string(),
        };
        let rec = exec.apply(&action, &mut env);
        assert!(rec.success);
    }
}
