//! Planner: selects adaptation actions from a pre-defined table when the
//! analyzer detects temporal property violations.
//!
//! The planner is the "P" in MAPE-K. It:
//! 1. Receives violation reports from the analyzer.
//! 2. Matches violations to a pre-defined action table.
//! 3. Selects the highest-priority matching action.
//!
//! Actions are never synthesized on the fly — they come from a
//! pre-verified library (matching the R-SPU's "pre-synthesized bitstream"
//! concept from the roadmap).
//!
//! Action table is fixed-size, loaded at init time.
//! Selection is a bounded linear scan. No heap in the hot path.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use super::ltl::PropertyResult;

/// Maximum action table size (bounded resource, NASA P10).
pub const MAX_ACTION_ENTRIES: usize = 256;

// ---------------------------------------------------------------------------
// Adaptation actions
// ---------------------------------------------------------------------------

/// An action the executor can apply to the system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdaptationAction {
    /// Force a signal to a specific value.
    SetSignal { name: String, value: u64 },
    /// Switch to a named operating mode (reconfiguration).
    /// In the R-SPU vision, this selects a pre-synthesized bitstream.
    SwitchMode { mode_name: String },
    /// Immediate safety clamp — halt all outputs to safe defaults.
    /// Maps to the R-SPU "Immediate Layer (Static)" reflex.
    EmergencyStop,
}

impl AdaptationAction {
    /// Human-readable label for audit logging.
    pub fn label(&self) -> String {
        match self {
            AdaptationAction::SetSignal { name, value } => {
                format!("SetSignal({name}={value})")
            }
            AdaptationAction::SwitchMode { mode_name } => {
                format!("SwitchMode({mode_name})")
            }
            AdaptationAction::EmergencyStop => "EmergencyStop".to_string(),
        }
    }
}

/// Whether to trigger on property violation or satisfaction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum TriggerCondition {
    /// Fire when the property is violated (safety invariant broken).
    #[default]
    OnViolation,
    /// Fire when the property is satisfied (dangerous condition detected).
    OnSatisfaction,
}

// ---------------------------------------------------------------------------
// Action table entry
// ---------------------------------------------------------------------------

/// A mapping from a property evaluation result to an adaptation action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEntry {
    /// Index of the property in the analyzer's property list.
    pub trigger_property_idx: usize,
    /// Action to take when the trigger condition is met.
    pub action: AdaptationAction,
    /// Priority (higher value wins ties). Range 0..=255.
    pub priority: u8,
    /// Whether to trigger on violation or satisfaction.
    #[serde(default)]
    pub trigger_on: TriggerCondition,
}

// ---------------------------------------------------------------------------
// Planner
// ---------------------------------------------------------------------------

/// The Planner component of the MAPE-K loop.
///
/// Holds a fixed-size action table and selects the highest-priority
/// matching action for a set of property violations.
#[derive(Debug, Clone)]
pub struct Planner {
    entries: Vec<ActionEntry>,
}

/// Result of the planner's selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanResult {
    /// The selected action, if any.
    pub action: Option<AdaptationAction>,
    /// Index of the action entry that was selected (in the entries list).
    pub entry_idx: Option<usize>,
    /// Index of the violated property that triggered this action.
    pub trigger_property_idx: Option<usize>,
}

impl Planner {
    /// Create a planner with the given action table.
    /// Entries beyond MAX_ACTION_ENTRIES are silently dropped.
    pub fn new(entries: Vec<ActionEntry>) -> Self {
        let mut e = entries;
        e.truncate(MAX_ACTION_ENTRIES);
        Self { entries: e }
    }

    /// Number of entries in the action table.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Select the highest-priority action matching any triggered condition.
    ///
    /// `results` should contain all `PropertyResult`s from the analyzer.
    /// Each action entry specifies whether it triggers on violation or
    /// satisfaction via `trigger_on`.
    ///
    /// Bounded: linear scan over entries (max MAX_ACTION_ENTRIES).
    pub fn select(&self, results: &[PropertyResult]) -> PlanResult {
        let mut best: Option<(usize, u8, usize)> = None; // (entry_idx, priority, prop_idx)

        for (eidx, entry) in self.entries.iter().enumerate() {
            let triggered = results.iter().any(|r| {
                if r.property_idx != entry.trigger_property_idx {
                    return false;
                }
                match entry.trigger_on {
                    TriggerCondition::OnViolation => !r.satisfied,
                    TriggerCondition::OnSatisfaction => r.satisfied,
                }
            });

            if triggered {
                let dominated = best
                    .map(|(_, bp, _)| entry.priority <= bp)
                    .unwrap_or(false);
                if !dominated {
                    best = Some((eidx, entry.priority, entry.trigger_property_idx));
                }
            }
        }

        match best {
            Some((eidx, _, pidx)) => PlanResult {
                action: Some(self.entries[eidx].action.clone()),
                entry_idx: Some(eidx),
                trigger_property_idx: Some(pidx),
            },
            None => PlanResult {
                action: None,
                entry_idx: None,
                trigger_property_idx: None,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn violation(idx: usize) -> PropertyResult {
        PropertyResult {
            property_idx: idx,
            satisfied: false,
            evidence_tick: Some(0),
        }
    }

    #[test]
    fn select_single_match() {
        let planner = Planner::new(vec![
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::EmergencyStop,
                priority: 10,
                trigger_on: TriggerCondition::OnViolation,
            },
        ]);
        let result = planner.select(&[violation(0)]);
        assert_eq!(result.action, Some(AdaptationAction::EmergencyStop));
    }

    #[test]
    fn select_highest_priority() {
        let planner = Planner::new(vec![
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::SetSignal {
                    name: "low".to_string(), value: 1,
                },
                priority: 5,
                trigger_on: TriggerCondition::OnViolation,
            },
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::EmergencyStop,
                priority: 20,
                trigger_on: TriggerCondition::OnViolation,
            },
        ]);
        let result = planner.select(&[violation(0)]);
        assert_eq!(result.action, Some(AdaptationAction::EmergencyStop));
        assert_eq!(result.entry_idx, Some(1));
    }

    #[test]
    fn select_no_match() {
        let planner = Planner::new(vec![
            ActionEntry {
                trigger_property_idx: 5,
                action: AdaptationAction::EmergencyStop,
                priority: 10,
                trigger_on: TriggerCondition::OnViolation,
            },
        ]);
        let result = planner.select(&[violation(0)]); // violation is for prop 0
        assert_eq!(result.action, None);
    }

    #[test]
    fn select_from_multiple_violations() {
        let planner = Planner::new(vec![
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::SetSignal {
                    name: "a".to_string(), value: 1,
                },
                priority: 5,
                trigger_on: TriggerCondition::OnViolation,
            },
            ActionEntry {
                trigger_property_idx: 1,
                action: AdaptationAction::EmergencyStop,
                priority: 20,
                trigger_on: TriggerCondition::OnViolation,
            },
        ]);
        let result = planner.select(&[violation(0), violation(1)]);
        assert_eq!(result.action, Some(AdaptationAction::EmergencyStop));
        assert_eq!(result.trigger_property_idx, Some(1));
    }

    #[test]
    fn select_on_satisfaction() {
        let planner = Planner::new(vec![
            ActionEntry {
                trigger_property_idx: 0,
                action: AdaptationAction::EmergencyStop,
                priority: 10,
                trigger_on: TriggerCondition::OnSatisfaction,
            },
        ]);
        let satisfied = PropertyResult {
            property_idx: 0,
            satisfied: true,
            evidence_tick: Some(5),
        };
        let result = planner.select(&[satisfied]);
        assert_eq!(result.action, Some(AdaptationAction::EmergencyStop));
    }

    #[test]
    fn action_label_formatting() {
        let a = AdaptationAction::SetSignal {
            name: "clamp".to_string(), value: 1,
        };
        assert_eq!(a.label(), "SetSignal(clamp=1)");
        assert_eq!(AdaptationAction::EmergencyStop.label(), "EmergencyStop");
    }
}
