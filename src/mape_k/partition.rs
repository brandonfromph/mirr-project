//! Configuration for MAPE-K partitioning.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Maximum components per partition (NASA P10: bounded collections).
pub const MAX_PARTITION_COMPONENTS: usize = 64;

/// Tag identifying a component's architectural role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentTag {
    Monitor,
    Analyzer,
    Planner,
    Executor,
    Knowledge,
}

/// Metadata for a single component in a partition.
#[derive(Debug, Clone)]
pub struct PartitionComponent {
    pub name: String,
    /// Semantic role and partition tag.
    pub tag: ComponentTag,
}

/// Configuration for MAPE-K partitioning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionConfig {
    /// Whether partitioning is enabled.
    pub enabled: bool,
    /// Number of MAPE-K ticks to simulate per partition.
    pub ticks_per_partition: u32,
}

impl Default for PartitionConfig {
    fn default() -> Self {
        Self { enabled: false, ticks_per_partition: 100 }
    }
}
