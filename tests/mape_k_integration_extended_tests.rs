//! Extended integration tests for the MAPE-K simulator and partitioning.
//!
//! Tests cover:
//! 1. Threshold violation detection via fault injection
//! 2. Emergency stop triggering on critical violations
//! 3. Knowledge base audit trail recording of adaptations
//! 4. Bounded tick enforcement (NASA P10 MAX_TICKS clamping)
//! 5. Default FPGA partition: monitor + executor
//! 6. Default ARM partition: analyzer + planner
//! 7. Shared knowledge bus across both partitions

#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]

use mirrc::mape_k::partition::{partition_components, ComponentTag, PartitionTarget};
use mirrc::mape_k::MAX_TICKS;
use mirrc::mape_k::{
    ActionEntry, AdaptationAction, MapeKSimulator, SensorConfig, SignalPredicate, SimConfig,
    TemporalProperty, TriggerCondition,
};

// ═══════════════════════════════════════════════════════════════════════════
// Constants — bounded iteration limits (NASA P10)
// ═══════════════════════════════════════════════════════════════════════════

const MAX_TEST_TICKS: u64 = 500;
const MAX_TEST_RECORDS: usize = 256;

// ═══════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Build a SensorConfig with fault injection (zero noise for determinism).
fn fault_sensor(name: &str, base: u64, fault_tick: u64, fault_val: u64, seed: u64) -> SensorConfig {
    SensorConfig {
        name: name.to_string(),
        base_value: base,
        noise_amplitude: 0,
        fault_at_tick: Some(fault_tick),
        fault_value: fault_val,
        fault_end_tick: None,
        seed,
        is_observable: true,
    }
}

/// Build a SensorConfig with no faults and no noise (steady baseline).
fn steady_sensor(name: &str, base: u64, seed: u64) -> SensorConfig {
    SensorConfig {
        name: name.to_string(),
        base_value: base,
        noise_amplitude: 0,
        fault_at_tick: None,
        fault_value: 0,
        fault_end_tick: None,
        seed,
        is_observable: true,
    }
}

/// Build a SimConfig for a single-sensor threshold scenario with emergency stop.
fn threshold_emergency_config() -> SimConfig {
    SimConfig {
        sensors: vec![fault_sensor("temp", 100, 50, 200, 42)],
        properties: vec![
            // Property 0: Always(temp < 150) -- violated when fault pushes temp to 200
            TemporalProperty::Always(SignalPredicate::LessThan("temp".to_string(), 150)),
        ],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 255,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 16,
        knowledge_capacity: MAX_TEST_RECORDS,
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 1. Simulator detects threshold violation via fault injection
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mape_k_simulator_detects_threshold_violation() {
    // Sensor "temp": base=100, fault at tick 50 snaps value to 200.
    // Property: Always(temp < 150) -- violated when temp=200 >= 150.
    let config = threshold_emergency_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(MAX_TEST_TICKS);

    // The fault at tick 50 causes temp=200 > 150, so violations must occur.
    assert!(
        result.total_violations > 0,
        "expected violations from fault injection; got total_violations={}",
        result.total_violations
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Simulator triggers emergency stop on critical violation
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mape_k_simulator_triggers_emergency_stop() {
    // Same config: fault at tick 50 => violation => EmergencyStop (priority 255).
    let config = threshold_emergency_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(MAX_TEST_TICKS);

    assert!(
        result.emergency_triggered,
        "emergency stop should have been triggered by threshold violation"
    );

    // Emergency must occur at or after the fault tick (tick 50).
    // The monitor ticks start at 0, and the fault fires at sensor tick 50,
    // so the emergency tick should be in the neighborhood of tick 50.
    let e_tick = result.emergency_tick.expect("emergency_tick should be Some");
    assert!(
        (50..=55).contains(&e_tick),
        "emergency_tick={e_tick} should be near the fault injection tick 50"
    );

    // Simulation should have halted early (well before MAX_TEST_TICKS=500).
    assert!(
        result.total_ticks < MAX_TEST_TICKS,
        "simulation should halt early on emergency; ran {} ticks",
        result.total_ticks
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. Knowledge base records all adaptations with audit trail
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mape_k_simulator_knowledge_records_adaptations() {
    let config = threshold_emergency_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(MAX_TEST_TICKS);

    // The adaptation_log length must equal total_adaptations.
    assert_eq!(
        result.adaptation_log.len(),
        result.total_adaptations as usize,
        "adaptation_log.len()={} != total_adaptations={}",
        result.adaptation_log.len(),
        result.total_adaptations
    );

    // At least one adaptation must have been recorded (the emergency stop).
    assert!(
        !result.adaptation_log.is_empty(),
        "adaptation_log should contain at least the emergency stop record"
    );

    // Each record must be marked as successful.
    for (i, record) in result.adaptation_log.iter().enumerate() {
        if i >= MAX_TEST_RECORDS {
            break;
        }
        assert!(record.success, "adaptation record at index {i} should have success=true");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Simulator clamps ticks to MAX_TICKS (bounded execution, NASA P10)
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mape_k_simulator_bounded_ticks() {
    // Steady sensor, no faults, no violations — simulation should run to completion.
    let config = SimConfig {
        sensors: vec![steady_sensor("pressure", 50, 7)],
        properties: vec![
            // Always(pressure < 100) — always satisfied since base=50, no noise.
            TemporalProperty::Always(SignalPredicate::LessThan("pressure".to_string(), 100)),
        ],
        action_table: vec![],
        window_size: 8,
        knowledge_capacity: 64,
    };

    let mut sim = MapeKSimulator::new(config);
    // Request more ticks than MAX_TICKS to test clamping.
    let result = sim.run(MAX_TICKS + 1000);

    assert!(
        result.total_ticks <= MAX_TICKS,
        "total_ticks={} should be clamped to MAX_TICKS={}",
        result.total_ticks,
        MAX_TICKS
    );

    // No violations in a clean run.
    assert_eq!(
        result.total_violations, 0,
        "no violations expected for steady sensor under threshold"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Partition: FPGA contains monitor and executor
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mape_k_partition_default_fpga_monitor_executor() {
    let result = partition_components();

    // FPGA partition must contain exactly 2 components.
    assert_eq!(
        result.fpga_components.len(),
        2,
        "FPGA partition should have 2 components, got {}",
        result.fpga_components.len()
    );

    // Verify monitor is present with the correct tag.
    let monitor = result.fpga_components.iter().find(|c| c.name == "monitor");
    assert!(monitor.is_some(), "FPGA partition must contain 'monitor'");
    assert_eq!(
        monitor.unwrap().tag,
        ComponentTag::FpgaMonitor,
        "monitor should have tag FpgaMonitor"
    );

    // Verify executor is present with the correct tag.
    let executor = result.fpga_components.iter().find(|c| c.name == "executor");
    assert!(executor.is_some(), "FPGA partition must contain 'executor'");
    assert_eq!(
        executor.unwrap().tag,
        ComponentTag::FpgaExecutor,
        "executor should have tag FpgaExecutor"
    );

    // Both should target FPGA.
    for comp in &result.fpga_components {
        assert_eq!(
            comp.tag.target(),
            PartitionTarget::Fpga,
            "FPGA component '{}' should target Fpga",
            comp.name
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Partition: ARM contains analyzer and planner
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mape_k_partition_default_arm_analyzer_planner() {
    let result = partition_components();

    // ARM partition must contain exactly 2 components.
    assert_eq!(
        result.arm_components.len(),
        2,
        "ARM partition should have 2 components, got {}",
        result.arm_components.len()
    );

    // Verify analyzer is present with the correct tag.
    let analyzer = result.arm_components.iter().find(|c| c.name == "analyzer");
    assert!(analyzer.is_some(), "ARM partition must contain 'analyzer'");
    assert_eq!(
        analyzer.unwrap().tag,
        ComponentTag::ArmAnalyzer,
        "analyzer should have tag ArmAnalyzer"
    );

    // Verify planner is present with the correct tag.
    let planner = result.arm_components.iter().find(|c| c.name == "planner");
    assert!(planner.is_some(), "ARM partition must contain 'planner'");
    assert_eq!(
        planner.unwrap().tag,
        ComponentTag::ArmPlanner,
        "planner should have tag ArmPlanner"
    );

    // Both should target ARM.
    for comp in &result.arm_components {
        assert_eq!(
            comp.tag.target(),
            PartitionTarget::Arm,
            "ARM component '{}' should target Arm",
            comp.name
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 7. Partition: shared knowledge bus bridges FPGA and ARM
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn mape_k_partition_shared_knowledge() {
    let result = partition_components();

    // Shared partition must contain exactly 1 component.
    assert_eq!(
        result.shared_components.len(),
        1,
        "shared partition should have 1 component, got {}",
        result.shared_components.len()
    );

    // Verify knowledge_bus is present with the correct tag.
    let kb = result.shared_components.iter().find(|c| c.name == "knowledge_bus");
    assert!(kb.is_some(), "shared partition must contain 'knowledge_bus'");
    assert_eq!(
        kb.unwrap().tag,
        ComponentTag::SharedKnowledge,
        "knowledge_bus should have tag SharedKnowledge"
    );

    // SharedKnowledge must target Both (bridging FPGA and ARM).
    assert_eq!(
        ComponentTag::SharedKnowledge.target(),
        PartitionTarget::Both,
        "SharedKnowledge should target PartitionTarget::Both"
    );
}
