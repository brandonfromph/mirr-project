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
    println!(
        "signals={} guards={} reflexes={}",
        program.module.signals.len(),
        program.module.guards.len(),
        program.module.reflexes.len()
    );
    for r in &program.module.reflexes {
        println!(
            "reflex '{}' guards={:?} assignments={}",
            r.name,
            r.guard_names,
            r.assignments.len()
        );
    }
}
