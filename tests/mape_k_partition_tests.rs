#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

//! Integration tests for the MAPE-K partition module (`src/mape_k/partition.rs`).
//!
//! Validates FPGA/ARM component partitioning, tag-to-target mapping,
//! label correctness, config defaults, bounds constants, determinism,
//! serialization round-trips, and cross-partition invariants.

use std::collections::HashSet;

use mirrc::mape_k::partition::{
    partition_components, total_components, ComponentTag, PartitionConfig, PartitionResult,
    PartitionTarget, PartitionedComponent, MAX_PARTITION_COMPONENTS,
};

// ---------------------------------------------------------------------------
// Constants — bounded iteration limits (NASA P10)
// ---------------------------------------------------------------------------

const MAX_TEST_COMPONENTS: usize = 64;
const MAX_TEST_ITERATIONS: usize = 16;

// ---------------------------------------------------------------------------
// Helpers — no recursion, bounded iteration
// ---------------------------------------------------------------------------

/// Collect all component names from a `PartitionResult` into a `Vec`.
fn all_names(result: &PartitionResult) -> Vec<String> {
    let mut names = Vec::with_capacity(MAX_TEST_COMPONENTS);
    for i in 0..result.fpga_components.len().min(MAX_TEST_COMPONENTS) {
        names.push(result.fpga_components[i].name.clone());
    }
    for i in 0..result.arm_components.len().min(MAX_TEST_COMPONENTS) {
        names.push(result.arm_components[i].name.clone());
    }
    for i in 0..result.shared_components.len().min(MAX_TEST_COMPONENTS) {
        names.push(result.shared_components[i].name.clone());
    }
    names
}

/// Collect all component tags from a `PartitionResult` into a `Vec`.
fn all_tags(result: &PartitionResult) -> Vec<ComponentTag> {
    let mut tags = Vec::with_capacity(MAX_TEST_COMPONENTS);
    for i in 0..result.fpga_components.len().min(MAX_TEST_COMPONENTS) {
        tags.push(result.fpga_components[i].tag.clone());
    }
    for i in 0..result.arm_components.len().min(MAX_TEST_COMPONENTS) {
        tags.push(result.arm_components[i].tag.clone());
    }
    for i in 0..result.shared_components.len().min(MAX_TEST_COMPONENTS) {
        tags.push(result.shared_components[i].tag.clone());
    }
    tags
}

// ═══════════════════════════════════════════════════════════════════════════
// 1. Basic partition structure
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn partition_components_returns_five_total() {
    let result = partition_components();
    assert_eq!(total_components(&result), 5, "default partition must have exactly 5 components");
}

#[test]
fn fpga_partition_contains_monitor_and_executor() {
    let result = partition_components();
    assert_eq!(result.fpga_components.len(), 2);
    assert_eq!(result.fpga_components[0].name, "monitor");
    assert_eq!(result.fpga_components[0].tag, ComponentTag::FpgaMonitor);
    assert_eq!(result.fpga_components[1].name, "executor");
    assert_eq!(result.fpga_components[1].tag, ComponentTag::FpgaExecutor);
}

#[test]
fn arm_partition_contains_analyzer_and_planner() {
    let result = partition_components();
    assert_eq!(result.arm_components.len(), 2);
    assert_eq!(result.arm_components[0].name, "analyzer");
    assert_eq!(result.arm_components[0].tag, ComponentTag::ArmAnalyzer);
    assert_eq!(result.arm_components[1].name, "planner");
    assert_eq!(result.arm_components[1].tag, ComponentTag::ArmPlanner);
}

#[test]
fn shared_partition_contains_knowledge_bus() {
    let result = partition_components();
    assert_eq!(result.shared_components.len(), 1);
    assert_eq!(result.shared_components[0].name, "knowledge_bus");
    assert_eq!(result.shared_components[0].tag, ComponentTag::SharedKnowledge);
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. ComponentTag -> PartitionTarget mapping
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn fpga_tags_map_to_fpga_target() {
    assert_eq!(ComponentTag::FpgaMonitor.target(), PartitionTarget::Fpga);
    assert_eq!(ComponentTag::FpgaExecutor.target(), PartitionTarget::Fpga);
}

#[test]
fn arm_tags_map_to_arm_target() {
    assert_eq!(ComponentTag::ArmAnalyzer.target(), PartitionTarget::Arm);
    assert_eq!(ComponentTag::ArmPlanner.target(), PartitionTarget::Arm);
}

#[test]
fn shared_knowledge_maps_to_both_target() {
    assert_eq!(ComponentTag::SharedKnowledge.target(), PartitionTarget::Both);
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. Label correctness
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn all_component_tag_labels_are_nonempty_and_unique() {
    let tags = [
        ComponentTag::FpgaMonitor,
        ComponentTag::FpgaExecutor,
        ComponentTag::ArmAnalyzer,
        ComponentTag::ArmPlanner,
        ComponentTag::SharedKnowledge,
    ];
    let mut seen = HashSet::new();
    for i in 0..tags.len() {
        let label = tags[i].label();
        assert!(!label.is_empty(), "label for {:?} must not be empty", tags[i]);
        assert!(seen.insert(label), "duplicate label: {}", label);
    }
}

#[test]
fn labels_contain_expected_platform_names() {
    assert!(ComponentTag::FpgaMonitor.label().contains("FPGA"));
    assert!(ComponentTag::FpgaExecutor.label().contains("FPGA"));
    assert!(ComponentTag::ArmAnalyzer.label().contains("ARM"));
    assert!(ComponentTag::ArmPlanner.label().contains("ARM"));
    assert!(ComponentTag::SharedKnowledge.label().contains("Shared"));
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. PartitionConfig
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn partition_config_default_is_disabled_with_100_ticks() {
    let config = PartitionConfig::default();
    assert!(!config.enabled, "default partition config must be disabled");
    assert_eq!(config.ticks_per_partition, 100);
}

#[test]
fn partition_config_custom_values_are_preserved() {
    let config = PartitionConfig { enabled: true, ticks_per_partition: 42 };
    assert!(config.enabled);
    assert_eq!(config.ticks_per_partition, 42);
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Bounds constants (NASA P10 compliance)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn bounds_constants_are_nonzero_and_sane() {
    // These are compile-time constants; verify at runtime that the default
    // partition fits within them.
    let result = partition_components();
    assert!(
        total_components(&result) <= MAX_PARTITION_COMPONENTS,
        "default partition ({}) exceeds MAX_PARTITION_COMPONENTS ({})",
        total_components(&result),
        MAX_PARTITION_COMPONENTS,
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Cross-partition invariants
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn all_component_names_are_unique_across_partitions() {
    let result = partition_components();
    let names = all_names(&result);
    let mut seen = HashSet::new();
    for i in 0..names.len().min(MAX_TEST_COMPONENTS) {
        assert!(
            seen.insert(names[i].clone()),
            "duplicate component name across partitions: {}",
            names[i],
        );
    }
}

#[test]
fn fpga_components_all_resolve_to_fpga_target() {
    let result = partition_components();
    for i in 0..result.fpga_components.len().min(MAX_TEST_COMPONENTS) {
        assert_eq!(
            result.fpga_components[i].tag.target(),
            PartitionTarget::Fpga,
            "FPGA component '{}' has wrong target",
            result.fpga_components[i].name,
        );
    }
}

#[test]
fn arm_components_all_resolve_to_arm_target() {
    let result = partition_components();
    for i in 0..result.arm_components.len().min(MAX_TEST_COMPONENTS) {
        assert_eq!(
            result.arm_components[i].tag.target(),
            PartitionTarget::Arm,
            "ARM component '{}' has wrong target",
            result.arm_components[i].name,
        );
    }
}

#[test]
fn shared_components_all_resolve_to_both_target() {
    let result = partition_components();
    for i in 0..result.shared_components.len().min(MAX_TEST_COMPONENTS) {
        assert_eq!(
            result.shared_components[i].tag.target(),
            PartitionTarget::Both,
            "shared component '{}' has wrong target",
            result.shared_components[i].name,
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. Determinism — partition_components is pure
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn partition_is_deterministic_across_calls() {
    let names_a = all_names(&partition_components());
    let tags_a = all_tags(&partition_components());

    for _round in 0..MAX_TEST_ITERATIONS {
        let names_b = all_names(&partition_components());
        let tags_b = all_tags(&partition_components());
        assert_eq!(names_a, names_b, "partition names must be deterministic");
        assert_eq!(tags_a, tags_b, "partition tags must be deterministic");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 8. Serialization round-trip (serde)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn partition_result_survives_json_round_trip() {
    let result = partition_components();
    let json = serde_json::to_string(&result).expect("serialize PartitionResult");
    let deserialized: PartitionResult =
        serde_json::from_str(&json).expect("deserialize PartitionResult");

    assert_eq!(total_components(&deserialized), total_components(&result));
    assert_eq!(all_names(&deserialized), all_names(&result));
    assert_eq!(all_tags(&deserialized), all_tags(&result));
}

#[test]
fn partition_target_survives_json_round_trip() {
    let targets = [PartitionTarget::Fpga, PartitionTarget::Arm, PartitionTarget::Both];
    for i in 0..targets.len() {
        let json = serde_json::to_string(&targets[i]).expect("serialize PartitionTarget");
        let deserialized: PartitionTarget =
            serde_json::from_str(&json).expect("deserialize PartitionTarget");
        assert_eq!(deserialized, targets[i]);
    }
}

#[test]
fn component_tag_survives_json_round_trip() {
    let tags = [
        ComponentTag::FpgaMonitor,
        ComponentTag::FpgaExecutor,
        ComponentTag::ArmAnalyzer,
        ComponentTag::ArmPlanner,
        ComponentTag::SharedKnowledge,
    ];
    for i in 0..tags.len() {
        let json = serde_json::to_string(&tags[i]).expect("serialize ComponentTag");
        let deserialized: ComponentTag =
            serde_json::from_str(&json).expect("deserialize ComponentTag");
        assert_eq!(deserialized, tags[i]);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 9. Edge case — total_components on empty result
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn total_components_on_empty_result_is_zero() {
    let empty = PartitionResult {
        fpga_components: Vec::new(),
        arm_components: Vec::new(),
        shared_components: Vec::new(),
    };
    assert_eq!(total_components(&empty), 0);
}

#[test]
fn total_components_counts_all_partitions() {
    let result = PartitionResult {
        fpga_components: vec![PartitionedComponent {
            name: "a".to_string(),
            tag: ComponentTag::FpgaMonitor,
        }],
        arm_components: vec![
            PartitionedComponent { name: "b".to_string(), tag: ComponentTag::ArmAnalyzer },
            PartitionedComponent { name: "c".to_string(), tag: ComponentTag::ArmPlanner },
        ],
        shared_components: vec![
            PartitionedComponent { name: "d".to_string(), tag: ComponentTag::SharedKnowledge },
            PartitionedComponent { name: "e".to_string(), tag: ComponentTag::SharedKnowledge },
            PartitionedComponent { name: "f".to_string(), tag: ComponentTag::SharedKnowledge },
        ],
    };
    assert_eq!(total_components(&result), 6);
}
