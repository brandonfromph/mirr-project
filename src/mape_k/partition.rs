//! MAPE-K component partitioning for hybrid FPGA/ARM deployment.
//!
//! Partitions the MAPE-K autonomic loop into hardware (FPGA) and
//! software (ARM) components. The Monitor and Executor run on the FPGA
//! for real-time response; the Analyzer and Planner run on the ARM
//! for complex decision-making. A shared Knowledge Bus bridges the two.
//!
//! This partition follows the R-SPU architectural model: the FPGA handles
//! the Immediate Layer (reflexes + monitoring), while the ARM handles
//! the Deliberative Layer (analysis + planning).
//!
//! All collections bounded (NASA Power-of-10).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Maximum components per partition (NASA P10: bounded collections).
pub const MAX_PARTITION_COMPONENTS: usize = 64;

/// Maximum knowledge bus entries.
pub const MAX_KNOWLEDGE_ENTRIES: usize = 256;

/// Deployment target for a MAPE-K component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PartitionTarget {
    /// Run on FPGA fabric (real-time, low-latency).
    Fpga,
    /// Run on ARM processor (complex logic, software).
    Arm,
    /// Run on both (replicated for redundancy).
    Both,
}

/// Semantic tag for a MAPE-K component's role and partition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentTag {
    /// Monitor: sensor sampling and event detection (FPGA-side).
    FpgaMonitor,
    /// Executor: actuator control and reflex response (FPGA-side).
    FpgaExecutor,
    /// Analyzer: property evaluation and violation detection (ARM-side).
    ArmAnalyzer,
    /// Planner: action selection from pre-verified library (ARM-side).
    ArmPlanner,
    /// Knowledge bus: shared state between FPGA and ARM.
    SharedKnowledge,
}

impl ComponentTag {
    /// Which partition this component belongs to.
    pub fn target(&self) -> PartitionTarget {
        match self {
            ComponentTag::FpgaMonitor | ComponentTag::FpgaExecutor => PartitionTarget::Fpga,
            ComponentTag::ArmAnalyzer | ComponentTag::ArmPlanner => PartitionTarget::Arm,
            ComponentTag::SharedKnowledge => PartitionTarget::Both,
        }
    }

    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            ComponentTag::FpgaMonitor => "Monitor (FPGA)",
            ComponentTag::FpgaExecutor => "Executor (FPGA)",
            ComponentTag::ArmAnalyzer => "Analyzer (ARM)",
            ComponentTag::ArmPlanner => "Planner (ARM)",
            ComponentTag::SharedKnowledge => "Knowledge Bus (Shared)",
        }
    }
}

/// A partitioned component with its tag and deployment info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionedComponent {
    /// Name of the component.
    pub name: String,
    /// Semantic role and partition tag.
    pub tag: ComponentTag,
}

/// Configuration for MAPE-K partitioning.
#[derive(Debug, Clone)]
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

/// Result of partitioning the MAPE-K loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionResult {
    /// Components assigned to FPGA.
    pub fpga_components: Vec<PartitionedComponent>,
    /// Components assigned to ARM.
    pub arm_components: Vec<PartitionedComponent>,
    /// Components shared across both.
    pub shared_components: Vec<PartitionedComponent>,
}

/// Partition the MAPE-K components into FPGA and ARM sets.
///
/// The default partition follows the R-SPU model:
/// - FPGA: Monitor + Executor (real-time layer)
/// - ARM: Analyzer + Planner (deliberative layer)
/// - Shared: Knowledge Bus
pub fn partition_components() -> PartitionResult {
    let fpga_components = vec![
        PartitionedComponent { name: "monitor".to_string(), tag: ComponentTag::FpgaMonitor },
        PartitionedComponent { name: "executor".to_string(), tag: ComponentTag::FpgaExecutor },
    ];

    let arm_components = vec![
        PartitionedComponent { name: "analyzer".to_string(), tag: ComponentTag::ArmAnalyzer },
        PartitionedComponent { name: "planner".to_string(), tag: ComponentTag::ArmPlanner },
    ];

    let shared_components = vec![PartitionedComponent {
        name: "knowledge_bus".to_string(),
        tag: ComponentTag::SharedKnowledge,
    }];

    PartitionResult { fpga_components, arm_components, shared_components }
}

/// Count total components across all partitions.
pub fn total_components(result: &PartitionResult) -> usize {
    result.fpga_components.len() + result.arm_components.len() + result.shared_components.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_partition_has_five_components() {
        let result = partition_components();
        assert_eq!(total_components(&result), 5);
    }

    #[test]
    fn fpga_partition_has_monitor_and_executor() {
        let result = partition_components();
        assert_eq!(result.fpga_components.len(), 2);
        assert_eq!(result.fpga_components[0].tag, ComponentTag::FpgaMonitor);
        assert_eq!(result.fpga_components[1].tag, ComponentTag::FpgaExecutor);
    }

    #[test]
    fn arm_partition_has_analyzer_and_planner() {
        let result = partition_components();
        assert_eq!(result.arm_components.len(), 2);
        assert_eq!(result.arm_components[0].tag, ComponentTag::ArmAnalyzer);
        assert_eq!(result.arm_components[1].tag, ComponentTag::ArmPlanner);
    }

    #[test]
    fn shared_partition_has_knowledge_bus() {
        let result = partition_components();
        assert_eq!(result.shared_components.len(), 1);
        assert_eq!(result.shared_components[0].tag, ComponentTag::SharedKnowledge);
    }

    #[test]
    fn component_tag_target_mapping() {
        assert_eq!(ComponentTag::FpgaMonitor.target(), PartitionTarget::Fpga);
        assert_eq!(ComponentTag::FpgaExecutor.target(), PartitionTarget::Fpga);
        assert_eq!(ComponentTag::ArmAnalyzer.target(), PartitionTarget::Arm);
        assert_eq!(ComponentTag::ArmPlanner.target(), PartitionTarget::Arm);
        assert_eq!(ComponentTag::SharedKnowledge.target(), PartitionTarget::Both);
    }

    #[test]
    fn component_tag_labels() {
        assert_eq!(ComponentTag::FpgaMonitor.label(), "Monitor (FPGA)");
        assert_eq!(ComponentTag::ArmPlanner.label(), "Planner (ARM)");
        assert_eq!(ComponentTag::SharedKnowledge.label(), "Knowledge Bus (Shared)");
    }

    #[test]
    fn partition_config_default() {
        let config = PartitionConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.ticks_per_partition, 100);
    }
}
