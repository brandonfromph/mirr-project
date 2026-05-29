#![forbid(unsafe_code)]

#[test]
fn debug_parse_multi_guard() {
    let src = r#"
module multi_guard_mod {
    signal temp: in u16;
    signal pressure: in u16;
    signal alarm_a: out bool;
    signal alarm_b: out bool;

    guard high_temp {
        when temp > 100
        for 5 cycles;
    }

    guard low_pressure {
        when pressure < 20
        for 10 cycles;
    }

    reflex temp_alarm {
        on high_temp {
            alarm_a = true;
        }
    }

    reflex pressure_alarm {
        on low_pressure {
            alarm_b = true;
        }
    }
}
"#;

    let program = nasa_rust_project::parser::parse_mirr(src).expect("parse failed");

    // Assert structural element counts.
    assert_eq!(program.module.signals.len(), 4, "Expected exactly 4 signals");
    assert_eq!(program.module.guards.len(), 2, "Expected exactly 2 guards");
    assert_eq!(program.module.reflexes.len(), 2, "Expected exactly 2 reflexes");

    // Assert details of the guards.
    let high_temp_guard = &program.module.guards[0];
    assert_eq!(high_temp_guard.name, "high_temp");
    assert_eq!(high_temp_guard.cycles, 5);

    let low_pressure_guard = &program.module.guards[1];
    assert_eq!(low_pressure_guard.name, "low_pressure");
    assert_eq!(low_pressure_guard.cycles, 10);

    // Assert details of the reflexes.
    let reflex_temp = &program.module.reflexes[0];
    assert_eq!(reflex_temp.name, "temp_alarm");
    assert_eq!(reflex_temp.guard_names, vec!["high_temp"]);
    assert_eq!(reflex_temp.assignments.len(), 1);
    assert_eq!(reflex_temp.assignments[0].target, "alarm_a");

    let reflex_pressure = &program.module.reflexes[1];
    assert_eq!(reflex_pressure.name, "pressure_alarm");
    assert_eq!(reflex_pressure.guard_names, vec!["low_pressure"]);
    assert_eq!(reflex_pressure.assignments.len(), 1);
    assert_eq!(reflex_pressure.assignments[0].target, "alarm_b");
}
