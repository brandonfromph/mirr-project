//! Knowledge Base: structured audit log of all MAPE-K adaptation decisions.
//!
//! The knowledge base is the "K" in MAPE-K. It:
//! 1. Records every adaptation decision with full context.
//! 2. Maintains a bounded ring buffer of adaptation records.
//! 3. Serializes to JSON for post-run analysis and auditability.
//!
//! This corresponds to the R-SPU's "Knowledge Base of pre-synthesized
//! bitstreams" concept — in simulation, we store the decision history
//! rather than actual bitstreams.
//!
//! Bounded capacity: MAX_LOG_ENTRIES. Ring buffer evicts oldest entries.

#![forbid(unsafe_code)]

use super::executor::ExecutionRecord;
use super::planner::AdaptationAction;
use crate::error::MirrError;
use serde::{Deserialize, Serialize};

/// Maximum number of adaptation records the knowledge base retains.
pub const MAX_LOG_ENTRIES: usize = 4096;

// ---------------------------------------------------------------------------
// Adaptation record — one per MAPE-K decision
// ---------------------------------------------------------------------------

/// A single adaptation decision record with full audit context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationRecord {
    /// Tick at which the adaptation occurred.
    pub tick: u64,
    /// Index of the property whose violation triggered this adaptation.
    pub trigger_property_idx: usize,
    /// Human-readable description of the trigger.
    pub trigger_description: String,
    /// The action that was taken.
    pub action: AdaptationAction,
    /// Whether the action was successfully applied.
    pub success: bool,
    /// Signal state before the action.
    pub pre_state: Vec<(String, u64)>,
    /// Signal state after the action.
    pub post_state: Vec<(String, u64)>,
}

impl AdaptationRecord {
    /// Create a record from an execution record plus tick/trigger context.
    pub fn from_execution(
        tick: u64,
        trigger_property_idx: usize,
        trigger_description: &str,
        exec_record: &ExecutionRecord,
    ) -> Self {
        Self {
            tick,
            trigger_property_idx,
            trigger_description: trigger_description.to_string(),
            action: exec_record.action.clone(),
            success: exec_record.success,
            pre_state: exec_record.pre_state.clone(),
            post_state: exec_record.post_state.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// Knowledge Base
// ---------------------------------------------------------------------------

/// The Knowledge Base component of the MAPE-K loop.
///
/// Stores adaptation records in a bounded ring buffer for audit trail
/// and post-run analysis.
#[derive(Debug, Clone)]
pub struct KnowledgeBase {
    records: Vec<AdaptationRecord>,
    capacity: usize,
    total_recorded: u64,
}

impl KnowledgeBase {
    /// Create a new knowledge base with the given capacity.
    /// Capacity is clamped to MAX_LOG_ENTRIES.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.min(MAX_LOG_ENTRIES);
        Self { records: Vec::with_capacity(cap), capacity: cap, total_recorded: 0 }
    }

    /// Append an adaptation record. If at capacity, evicts the oldest.
    pub fn record(&mut self, entry: AdaptationRecord) {
        if self.records.len() >= self.capacity {
            // Ring buffer: remove oldest (index 0). O(n) but bounded by
            // MAX_LOG_ENTRIES and only happens on overflow.
            self.records.remove(0);
        }
        self.records.push(entry);
        self.total_recorded = self.total_recorded.wrapping_add(1);
    }

    /// Number of records currently stored.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether the knowledge base is empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Total number of records ever recorded (including evicted ones).
    pub fn total_recorded(&self) -> u64 {
        self.total_recorded
    }

    /// Get all stored records (oldest first).
    pub fn records(&self) -> &[AdaptationRecord] {
        &self.records
    }

    /// Serialize the entire knowledge base to pretty-printed JSON.
    pub fn to_json(&self) -> Result<String, MirrError> {
        serde_json::to_string_pretty(&self.records)
            .map_err(|e| MirrError::InternalError(format!("JSON serialization failed: {e}")))
    }

    /// Clear all records.
    pub fn clear(&mut self) {
        self.records.clear();
        self.total_recorded = 0;
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_record(tick: u64) -> AdaptationRecord {
        AdaptationRecord {
            tick,
            trigger_property_idx: 0,
            trigger_description: "pressure drop".to_string(),
            action: AdaptationAction::EmergencyStop,
            success: true,
            pre_state: vec![("p".to_string(), 120)],
            post_state: vec![("p".to_string(), 0)],
        }
    }

    #[test]
    fn record_and_retrieve() {
        let mut kb = KnowledgeBase::new(10);
        kb.record(sample_record(0));
        kb.record(sample_record(1));
        assert_eq!(kb.len(), 2);
        assert_eq!(kb.records()[0].tick, 0);
        assert_eq!(kb.records()[1].tick, 1);
    }

    #[test]
    fn ring_buffer_eviction() {
        let mut kb = KnowledgeBase::new(3);
        kb.record(sample_record(0));
        kb.record(sample_record(1));
        kb.record(sample_record(2));
        kb.record(sample_record(3)); // evicts tick 0
        assert_eq!(kb.len(), 3);
        assert_eq!(kb.records()[0].tick, 1);
        assert_eq!(kb.total_recorded(), 4);
    }

    #[test]
    fn json_serialization() {
        let mut kb = KnowledgeBase::new(10);
        kb.record(sample_record(42));
        let json = kb.to_json().unwrap();
        assert!(json.contains("\"tick\": 42"));
        assert!(json.contains("EmergencyStop"));
    }

    #[test]
    fn clear_resets() {
        let mut kb = KnowledgeBase::new(10);
        kb.record(sample_record(0));
        kb.clear();
        assert!(kb.is_empty());
        assert_eq!(kb.total_recorded(), 0);
    }
}
