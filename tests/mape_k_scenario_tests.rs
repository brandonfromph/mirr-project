//! MAPE-K scenario integration tests — real-world safety-critical scenarios.

#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::mape_k::{
    ActionEntry, AdaptationAction, MapeKSimulator, SensorConfig, SignalPredicate, SimConfig,
    TemporalProperty, TriggerCondition,
};

fn normal_pressure_config() -> SimConfig {
    SimConfig {
        sensors: vec![SensorConfig {
            name: "pressure".into(),
            base_value: 120,
            noise_amplitude: 5,
            fault_at_tick: None,
            fault_value: 0,
            fault_end_tick: None,
            seed: 42,
            is_observable: true,
        }],
        properties: vec![TemporalProperty::Always(SignalPredicate::GreaterThan(
            "pressure".into(),
            50,
        ))],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::SetSignal { name: "pressure".into(), value: 1 },
            priority: 10,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 64,
        knowledge_capacity: 100,
    }
}

#[test]
fn mape_k_normal_operation() {
    let config = normal_pressure_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(1000);
    assert_eq!(result.total_violations, 0);
    assert_eq!(result.total_adaptations, 0);
    assert!(!result.emergency_triggered);
}

#[test]
fn mape_k_single_violation() {
    let mut config = normal_pressure_config();
    config.sensors[0].fault_at_tick = Some(100);
    config.sensors[0].fault_value = 10;
    config.sensors[0].fault_end_tick = Some(110);

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(200);
    assert!(result.total_violations > 0);
    assert!(result.total_adaptations > 0);
}

#[test]
fn mape_k_emergency_stop() {
    let config = SimConfig {
        sensors: vec![SensorConfig {
            name: "pressure".into(),
            base_value: 120,
            noise_amplitude: 0,
            fault_at_tick: Some(10),
            fault_value: 0,
            fault_end_tick: None,
            seed: 1,
            is_observable: true,
        }],
        properties: vec![TemporalProperty::Persists(
            SignalPredicate::LessThan("pressure".into(), 50),
            5,
        )],
        action_table: vec![ActionEntry {
            trigger_property_idx: 0,
            action: AdaptationAction::EmergencyStop,
            priority: 100,
            trigger_on: TriggerCondition::OnViolation,
        }],
        window_size: 32,
        knowledge_capacity: 100,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(1000);
    assert!(result.emergency_triggered);
    assert!(result.total_ticks < 1000);
}

#[test]
fn mape_k_zero_tick() {
    let config = normal_pressure_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(0);
    assert_eq!(result.total_ticks, 0);
    assert_eq!(result.total_violations, 0);
}

#[test]
fn mape_k_multiple_sensors() {
    let config = SimConfig {
        sensors: vec![
            SensorConfig {
                name: "temp".into(),
                base_value: 100,
                noise_amplitude: 3,
                fault_at_tick: None,
                fault_value: 0,
                fault_end_tick: None,
                seed: 10,
                is_observable: true,
            },
            SensorConfig {
                name: "pressure".into(),
                base_value: 200,
                noise_amplitude: 5,
                fault_at_tick: None,
                fault_value: 0,
                fault_end_tick: None,
                seed: 20,
                is_observable: true,
            },
        ],
        properties: vec![
            TemporalProperty::Always(SignalPredicate::GreaterThan("temp".into(), 50)),
            TemporalProperty::Always(SignalPredicate::GreaterThan("pressure".into(), 100)),
        ],
        action_table: vec![],
        window_size: 32,
        knowledge_capacity: 100,
    };

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(500);
    assert_eq!(result.total_violations, 0);
}

#[test]
fn mape_k_knowledge_records() {
    let mut config = normal_pressure_config();
    config.sensors[0].fault_at_tick = Some(50);
    config.sensors[0].fault_value = 10;
    config.sensors[0].fault_end_tick = Some(60);

    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(200);
    assert!(!result.adaptation_log.is_empty());
}

#[test]
fn mape_k_summary_format() {
    let config = normal_pressure_config();
    let mut sim = MapeKSimulator::new(config);
    let result = sim.run(100);
    let summary = result.summary();
    assert!(summary.contains("MAPE-K Simulation:"));
    assert!(summary.contains("Violations detected:"));
}
