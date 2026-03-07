//! Analyzer: evaluates bounded LTL temporal properties over the monitor's
//! rolling signal windows.
//!
//! The analyzer is the "A" in MAPE-K. Each tick it:
//! 1. Receives the current monitor state (rolling windows).
//! 2. Evaluates each registered temporal property.
//! 3. Reports which properties are satisfied and which are violated.
//!
//! All evaluation is O(window_size) per property per tick.
//! All loops bounded. No recursion (NASA P10 rule #1).

#![forbid(unsafe_code)]

use super::ltl::{PropertyResult, SignalPredicate, TemporalProperty};
use super::monitor::Monitor;

/// Maximum number of properties an analyzer can track.
pub const MAX_PROPERTIES: usize = 256;

// ---------------------------------------------------------------------------
// Analyzer
// ---------------------------------------------------------------------------

/// The Analyzer component of the MAPE-K loop.
///
/// Holds a set of temporal properties and evaluates them against
/// the monitor's rolling windows each tick.
#[derive(Debug, Clone)]
pub struct Analyzer {
    properties: Vec<TemporalProperty>,
}

impl Analyzer {
    /// Create a new analyzer with the given properties.
    /// Excess properties beyond MAX_PROPERTIES are silently dropped.
    pub fn new(properties: Vec<TemporalProperty>) -> Self {
        let mut props = properties;
        props.truncate(MAX_PROPERTIES);
        Self { properties: props }
    }

    /// Number of registered properties.
    pub fn property_count(&self) -> usize {
        self.properties.len()
    }

    /// Evaluate all properties against the current monitor state.
    /// Returns one `PropertyResult` per property, in registration order.
    pub fn evaluate(&self, monitor: &Monitor) -> Vec<PropertyResult> {
        let mut results = Vec::with_capacity(self.properties.len());
        for (idx, prop) in self.properties.iter().enumerate() {
            results.push(self.evaluate_one(idx, prop, monitor));
        }
        results
    }

    /// Return only violated properties from the last evaluation.
    pub fn violations(&self, monitor: &Monitor) -> Vec<PropertyResult> {
        self.evaluate(monitor).into_iter().filter(|r| !r.satisfied).collect()
    }
}

// ---------------------------------------------------------------------------
// Per-property evaluation (private)
// ---------------------------------------------------------------------------

impl Analyzer {
    fn evaluate_one(
        &self,
        idx: usize,
        prop: &TemporalProperty,
        monitor: &Monitor,
    ) -> PropertyResult {
        match prop {
            TemporalProperty::Always(pred) => self.eval_always(idx, pred, monitor),
            TemporalProperty::EventuallyWithin(pred, n) => {
                self.eval_eventually_within(idx, pred, *n, monitor)
            }
            TemporalProperty::Persists(pred, n) => self.eval_persists(idx, pred, *n, monitor),
        }
    }

    /// G(P): P must hold at every tick in the window.
    fn eval_always(&self, idx: usize, pred: &SignalPredicate, monitor: &Monitor) -> PropertyResult {
        let window = match monitor.window(pred.signal_name()) {
            Some(w) => w,
            None => {
                return PropertyResult {
                    property_idx: idx,
                    satisfied: true, // no data => vacuously true
                    evidence_tick: None,
                };
            }
        };

        if window.is_empty() {
            return PropertyResult { property_idx: idx, satisfied: true, evidence_tick: None };
        }

        // Scan window from oldest to newest. First violation = evidence.
        for i in 0..window.len() {
            if let Some(val) = window.get(i) {
                if !pred.evaluate(val) {
                    return PropertyResult {
                        property_idx: idx,
                        satisfied: false,
                        evidence_tick: Some(i as u64),
                    };
                }
            }
        }

        PropertyResult {
            property_idx: idx,
            satisfied: true,
            evidence_tick: Some(window.len().saturating_sub(1) as u64),
        }
    }

    /// F<=N(P): P must become true at least once within the last N ticks.
    fn eval_eventually_within(
        &self,
        idx: usize,
        pred: &SignalPredicate,
        n: u64,
        monitor: &Monitor,
    ) -> PropertyResult {
        let window = match monitor.window(pred.signal_name()) {
            Some(w) => w,
            None => {
                return PropertyResult {
                    property_idx: idx,
                    satisfied: false, // no data => never satisfied
                    evidence_tick: None,
                };
            }
        };

        if window.is_empty() {
            return PropertyResult { property_idx: idx, satisfied: false, evidence_tick: None };
        }

        // Check the last N entries (or full window if shorter).
        let check_count = (n as usize).min(window.len());
        let start = window.len().saturating_sub(check_count);

        for i in start..window.len() {
            if let Some(val) = window.get(i) {
                if pred.evaluate(val) {
                    return PropertyResult {
                        property_idx: idx,
                        satisfied: true,
                        evidence_tick: Some(i as u64),
                    };
                }
            }
        }

        PropertyResult {
            property_idx: idx,
            satisfied: false,
            evidence_tick: Some(window.len().saturating_sub(1) as u64),
        }
    }

    /// Persists(P, N): P must hold for at least N consecutive ticks in window.
    fn eval_persists(
        &self,
        idx: usize,
        pred: &SignalPredicate,
        n: u64,
        monitor: &Monitor,
    ) -> PropertyResult {
        let window = match monitor.window(pred.signal_name()) {
            Some(w) => w,
            None => {
                return PropertyResult { property_idx: idx, satisfied: false, evidence_tick: None }
            }
        };

        if window.is_empty() || n == 0 {
            return PropertyResult {
                property_idx: idx,
                // N=0 is vacuously satisfied; empty window is not.
                satisfied: n == 0,
                evidence_tick: None,
            };
        }

        // Scan for a run of N consecutive ticks where P holds.
        let mut consecutive: u64 = 0;
        let target = n;

        for i in 0..window.len() {
            if let Some(val) = window.get(i) {
                if pred.evaluate(val) {
                    consecutive += 1;
                    if consecutive >= target {
                        return PropertyResult {
                            property_idx: idx,
                            satisfied: true,
                            evidence_tick: Some(i as u64),
                        };
                    }
                } else {
                    consecutive = 0;
                }
            }
        }

        PropertyResult {
            property_idx: idx,
            satisfied: false,
            evidence_tick: Some(window.len().saturating_sub(1) as u64),
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mape_k::ltl::{SignalPredicate, TemporalProperty};

    fn make_monitor_with(name: &str, values: &[u64]) -> Monitor {
        let mut mon = Monitor::new(64, &[name]);
        for &v in values {
            mon.record_sample(name, v);
            mon.advance_tick();
        }
        mon
    }

    #[test]
    fn always_satisfied() {
        let mon = make_monitor_with("p", &[50, 51, 52, 53]);
        let a = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "p".to_string(),
            40,
        ))]);
        let results = a.evaluate(&mon);
        assert!(results[0].satisfied);
    }

    #[test]
    fn always_violated() {
        let mon = make_monitor_with("p", &[50, 51, 30, 53]);
        let a = Analyzer::new(vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "p".to_string(),
            40,
        ))]);
        let results = a.evaluate(&mon);
        assert!(!results[0].satisfied);
        assert_eq!(results[0].evidence_tick, Some(2));
    }

    #[test]
    fn eventually_within_satisfied() {
        let mon = make_monitor_with("flag", &[0, 0, 1, 0]);
        let a = Analyzer::new(vec![TemporalProperty::EventuallyWithin(
            SignalPredicate::IsTrue("flag".to_string()),
            4,
        )]);
        let results = a.evaluate(&mon);
        assert!(results[0].satisfied);
    }

    #[test]
    fn eventually_within_violated() {
        let mon = make_monitor_with("flag", &[0, 0, 0, 0]);
        let a = Analyzer::new(vec![TemporalProperty::EventuallyWithin(
            SignalPredicate::IsTrue("flag".to_string()),
            4,
        )]);
        let results = a.evaluate(&mon);
        assert!(!results[0].satisfied);
    }

    #[test]
    fn eventually_within_window_boundary() {
        // Only check last 2 ticks. True at tick 0, but not in last 2.
        let mon = make_monitor_with("s", &[1, 0, 0, 0]);
        let a = Analyzer::new(vec![TemporalProperty::EventuallyWithin(
            SignalPredicate::IsTrue("s".to_string()),
            2,
        )]);
        let results = a.evaluate(&mon);
        assert!(!results[0].satisfied);
    }

    #[test]
    fn persists_satisfied() {
        let mon = make_monitor_with("p", &[10, 20, 30, 40, 50]);
        let a = Analyzer::new(vec![TemporalProperty::Persists(
            SignalPredicate::GreaterThan("p".to_string(), 5),
            3,
        )]);
        let results = a.evaluate(&mon);
        assert!(results[0].satisfied);
    }

    #[test]
    fn persists_broken_run() {
        // Run of 2, then break, then run of 1. Need 3 consecutive.
        let mon = make_monitor_with("p", &[50, 50, 2, 50]);
        let a = Analyzer::new(vec![TemporalProperty::Persists(
            SignalPredicate::GreaterThan("p".to_string(), 10),
            3,
        )]);
        let results = a.evaluate(&mon);
        assert!(!results[0].satisfied);
    }

    #[test]
    fn persists_exact_boundary() {
        // Exactly 3 consecutive at the end.
        let mon = make_monitor_with("x", &[0, 1, 1, 1]);
        let a = Analyzer::new(vec![TemporalProperty::Persists(
            SignalPredicate::IsTrue("x".to_string()),
            3,
        )]);
        let results = a.evaluate(&mon);
        assert!(results[0].satisfied);
    }

    #[test]
    fn multiple_properties() {
        let mon = make_monitor_with("v", &[100, 200, 300]);
        let a = Analyzer::new(vec![
            TemporalProperty::Always(SignalPredicate::GreaterThan("v".to_string(), 50)),
            TemporalProperty::Always(SignalPredicate::LessThan("v".to_string(), 250)),
        ]);
        let results = a.evaluate(&mon);
        assert!(results[0].satisfied); // all > 50
        assert!(!results[1].satisfied); // 300 >= 250
    }
}
