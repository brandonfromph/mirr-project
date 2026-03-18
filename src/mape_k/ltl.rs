//! Bounded Linear Temporal Logic (LTL) property types for MAPE-K analysis.
//!
//! Provides a 3-operator subset of LTL sufficient for safety-critical
//! invariant checking over bounded signal windows:
//!
//! - `Always(P)` — G(P): P must hold every tick in the window.
//! - `EventuallyWithin(P, N)` — F<=N(P): P must become true within N ticks.
//! - `Persists(P, N)` — P must hold for at least N consecutive ticks.
//!
//! All evaluation is bounded by the window size (no unbounded history).
//! All traversal is iterative (NASA P10 rule #1).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Signal predicates — atomic propositions over signal values
// ---------------------------------------------------------------------------

/// An atomic proposition evaluated against a single signal value each tick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalPredicate {
    /// Signal equals boolean true (or integer != 0).
    IsTrue(String),
    /// Signal value < threshold.
    LessThan(String, u64),
    /// Signal value > threshold.
    GreaterThan(String, u64),
    /// Signal value is within [low, high] inclusive.
    InRange(String, u64, u64),
}

impl SignalPredicate {
    /// Return the signal name this predicate references.
    pub fn signal_name(&self) -> &str {
        match self {
            SignalPredicate::IsTrue(n)
            | SignalPredicate::LessThan(n, _)
            | SignalPredicate::GreaterThan(n, _)
            | SignalPredicate::InRange(n, _, _) => n,
        }
    }

    /// Evaluate this predicate against a concrete u64 value.
    pub fn evaluate(&self, value: u64) -> bool {
        match self {
            SignalPredicate::IsTrue(_) => value != 0,
            SignalPredicate::LessThan(_, threshold) => value < *threshold,
            SignalPredicate::GreaterThan(_, threshold) => value > *threshold,
            SignalPredicate::InRange(_, low, high) => value >= *low && value <= *high,
        }
    }
}

// ---------------------------------------------------------------------------
// Temporal properties — bounded LTL operators
// ---------------------------------------------------------------------------

/// A bounded temporal property to be checked over a rolling signal window.
///
/// These correspond to a restricted subset of LTL where all operators
/// are bounded by the finite window size, ensuring decidability and
/// O(window_size) evaluation per property per tick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalProperty {
    /// G(P): P must hold at every tick in the current window.
    /// Violation: any tick where P is false.
    Always(SignalPredicate),

    /// F<=N(P): P must become true at least once within the last N ticks.
    /// Violation: P has been false for N consecutive ticks.
    EventuallyWithin(SignalPredicate, u64),

    /// P must hold for at least N consecutive ticks (anywhere in the window).
    /// Used for "sustained condition" checks like pressure-drop guards.
    /// Violation: no run of N consecutive true ticks exists in the window.
    Persists(SignalPredicate, u64),

    /// G(P -> Q): whenever P holds, Q must also hold (same cycle).
    /// Violation: some tick where P is true and Q is false.
    AlwaysImplies(SignalPredicate, SignalPredicate),

    /// never (P -> Q): there must exist a tick where P is true and Q is false.
    /// Violation: no such tick exists in the window.
    NeverImplies(SignalPredicate, SignalPredicate),

    /// always (P followed_by N Q): whenever P holds, Q must hold N cycles later.
    /// Violation: for some tick where P holds, the response does not hold N cycles after.
    AlwaysFollowedBy(SignalPredicate, u64, SignalPredicate),
}

impl TemporalProperty {
    /// Return a representative signal name this property references.
    /// For multi-signal properties this returns the antecedent's signal.
    pub fn signal_name(&self) -> &str {
        match self {
            TemporalProperty::Always(p)
            | TemporalProperty::EventuallyWithin(p, _)
            | TemporalProperty::Persists(p, _)
            | TemporalProperty::AlwaysImplies(p, _)
            | TemporalProperty::NeverImplies(p, _) => p.signal_name(),
            TemporalProperty::AlwaysFollowedBy(p, _, _) => p.signal_name(),
        }
    }
}

// ---------------------------------------------------------------------------
// Property evaluation result
// ---------------------------------------------------------------------------

/// Result of evaluating a temporal property over the current window.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyResult {
    /// Index of this property in the analyzer's property list.
    pub property_idx: usize,
    /// Whether the property is currently satisfied.
    pub satisfied: bool,
    /// Tick offset (within the window) where violation was first detected,
    /// or where the property was last confirmed. `None` if window is empty.
    pub evidence_tick: Option<u64>,
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predicate_is_true_nonzero() {
        let p = SignalPredicate::IsTrue("flag".to_string());
        assert!(p.evaluate(1));
        assert!(p.evaluate(255));
        assert!(!p.evaluate(0));
    }

    #[test]
    fn predicate_less_than() {
        let p = SignalPredicate::LessThan("pressure".to_string(), 50);
        assert!(p.evaluate(49));
        assert!(!p.evaluate(50));
        assert!(!p.evaluate(100));
    }

    #[test]
    fn predicate_greater_than() {
        let p = SignalPredicate::GreaterThan("rate".to_string(), 100);
        assert!(p.evaluate(101));
        assert!(!p.evaluate(100));
        assert!(!p.evaluate(0));
    }

    #[test]
    fn predicate_in_range() {
        let p = SignalPredicate::InRange("temp".to_string(), 36, 38);
        assert!(p.evaluate(36));
        assert!(p.evaluate(37));
        assert!(p.evaluate(38));
        assert!(!p.evaluate(35));
        assert!(!p.evaluate(39));
    }

    #[test]
    fn predicate_signal_name() {
        let p = SignalPredicate::IsTrue("abc".to_string());
        assert_eq!(p.signal_name(), "abc");
    }

    #[test]
    fn temporal_property_signal_name() {
        let t = TemporalProperty::Always(SignalPredicate::IsTrue("x".to_string()));
        assert_eq!(t.signal_name(), "x");
    }

    #[test]
    fn temporal_property_signal_name_for_multi_signal() {
        let t = TemporalProperty::AlwaysImplies(
            SignalPredicate::IsTrue("a".to_string()),
            SignalPredicate::IsTrue("b".to_string()),
        );
        assert_eq!(t.signal_name(), "a");
    }
}
