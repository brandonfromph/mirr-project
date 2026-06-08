#![forbid(unsafe_code)]

use mirrc::mape_k::*;

#[test]
fn test_mape_k_thrashing_vulnerability() {
    // 1. Setup a sensor "p" that starts in fault.
    let sensor_cfg = SensorConfig {
        name: "p".to_string(),
        base_value: 100,
        noise_amplitude: 0,
        fault_at_tick: Some(0),
        fault_value: 0, // Faulty value
        fault_end_tick: None,
        seed: 42,
        is_observable: true,
    };

    // 2. Setup a property: p must be > 50.
    let prop = TemporalProperty::Always(SignalPredicate::GreaterThan("p".to_string(), 50));

    // 3. Setup an action: if p_safe is violated, SetSignal(p, 100).
    let action_entry = ActionEntry {
        trigger_property_idx: 0,
        action: AdaptationAction::SetSignal { name: "p".to_string(), value: 100 },
        priority: 10,
        trigger_on: TriggerCondition::OnViolation,
    };

    let config = SimConfig {
        sensors: vec![sensor_cfg],
        properties: vec![prop],
        action_table: vec![action_entry],
        window_size: 10,
        knowledge_capacity: 1000,
    };

    let mut sim = MapeKSimulator::new(config);

    // 4. Run for 5 ticks.
    // EXPECTATION: Since 'p' is faulty at 0, it stays at 0.
    // The executor applies SetSignal(p, 100) at each tick.
    // Because the sensor is an "input" (observable), it OVERWRITES the executor's
    // attempt to set the value every cycle in the simulator's tick loop:
    // tick():
    //   M: value = sensor.sample() (returns 0 due to fault)
    //   A: violation detected
    //   E: executor.apply(SetSignal(p, 100)) -> sets env[p] = 100
    //   Next tick: env[p] is overwritten by sensor.sample() = 0.

    let result = sim.run(5);

    println!("{}", result.summary());

    // In a system with NO damping, we expect 5 adaptations for 5 ticks of violation.
    assert_eq!(
        result.total_adaptations, 5,
        "System thrashed! Expected 5 adaptations for 5 ticks of persistent violation."
    );
}
