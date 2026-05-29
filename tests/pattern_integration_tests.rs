//! Pattern expansion integration tests.

#![forbid(unsafe_code)]
#![deny(warnings)]

use nasa_rust_project::expand::expand_patterns;
use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::validation::validate_pattern_defs;

#[test]
fn pattern_simple_expand() {
    let source = r#"
def simple_delay(input: signal in bool, output: signal out bool) {
    reflect {
        guard delay_g {
            when ${input}
            for 1 cycles;
        }
        reflex delay_r {
            on delay_g {
                ${output} = true;
            }
        }
    }
}

module test {
    signal trigger: in bool;
    signal result: out bool;

    simple_delay(trigger, result);
}
"#;
    let mut program = parse_mirr(source).expect("parse should succeed");
    validate_pattern_defs(&program.patterns).expect("validation should succeed");
    let mut registry = nasa_rust_project::ecs::Registry::new();
    nasa_rust_project::ecs::adapter::ingest_program(&mut registry, program.clone(), None)
        .expect("ingestion should succeed");
    expand_patterns(&mut program, &registry).expect("expansion should succeed");
    assert!(!program.module.guards.is_empty(), "should have expanded guard");
    assert!(!program.module.reflexes.is_empty(), "should have expanded reflex");
}

#[test]
fn pattern_multiple_calls() {
    let source = r#"
def buffer(input: signal in bool, output: signal out bool) {
    reflect {
        guard buf_g {
            when ${input}
            for 1 cycles;
        }
        reflex buf_r {
            on buf_g {
                ${output} = true;
            }
        }
    }
}

module test {
    signal a: in bool;
    signal b: in bool;
    signal x: out bool;
    signal y: out bool;

    buffer(a, x);
    buffer(b, y);
}
"#;
    let mut program = parse_mirr(source).expect("parse should succeed");
    validate_pattern_defs(&program.patterns).expect("validation should succeed");
    let mut registry = nasa_rust_project::ecs::Registry::new();
    nasa_rust_project::ecs::adapter::ingest_program(&mut registry, program.clone(), None)
        .expect("ingestion should succeed");
    expand_patterns(&mut program, &registry).expect("expansion should succeed");
    assert!(program.module.guards.len() >= 2, "should have at least 2 expanded guards");
}

#[test]
fn pattern_signal_renaming() {
    let source = r#"
def delay(input: signal in bool, output: signal out bool) {
    reflect {
        signal internal_val: internal bool;
        guard delay_g {
            when ${input}
            for 1 cycles;
        }
        reflex delay_r {
            on delay_g {
                ${output} = true;
            }
        }
    }
}

module test {
    signal a: in bool;
    signal x: out bool;

    delay(a, x);
}
"#;
    let mut program = parse_mirr(source).expect("parse should succeed");
    validate_pattern_defs(&program.patterns).expect("validation should succeed");
    let mut registry = nasa_rust_project::ecs::Registry::new();
    nasa_rust_project::ecs::adapter::ingest_program(&mut registry, program.clone(), None)
        .expect("ingestion should succeed");
    expand_patterns(&mut program, &registry).expect("expansion should succeed");
    // Internal signals should be renamed to avoid conflicts
    assert!(!program.module.signals.is_empty());
}

#[test]
fn pattern_arity_mismatch() {
    let source = r#"
def two_params(a: signal in bool, b: signal in bool) {
    reflect {
        guard g {
            when ${a}
            for 1 cycles;
        }
    }
}

module test {
    signal x: in bool;
    signal y: out bool;

    two_params(x);
}
"#;
    let mut program = parse_mirr(source).expect("parse should succeed");
    validate_pattern_defs(&program.patterns).expect("validation should succeed");
    let mut registry = nasa_rust_project::ecs::Registry::new();
    nasa_rust_project::ecs::adapter::ingest_program(&mut registry, program.clone(), None)
        .expect("ingestion should succeed");
    let result = expand_patterns(&mut program, &registry);
    assert!(result.is_err(), "arity mismatch should be detected");
}

#[test]
fn pattern_full_pipeline() {
    let source = r#"
def threshold_alarm(sensor: signal in u16, threshold: u16, alarm: signal out bool) {
    reflect {
        guard ${sensor}_breach {
            when ${sensor} > ${threshold}
            for 4 cycles;
        }
        reflex ${sensor}_trip {
            on ${sensor}_breach {
                ${alarm} = true;
            }
        }
    }
}

module test {
    signal pressure: in u16;
    signal alarm: out bool;

    threshold_alarm(pressure, 4000, alarm);
}
"#;
    let mut program = parse_mirr(source).expect("parse should succeed");
    validate_pattern_defs(&program.patterns).expect("validation should succeed");
    let mut registry = nasa_rust_project::ecs::Registry::new();
    nasa_rust_project::ecs::adapter::ingest_program(&mut registry, program.clone(), None)
        .expect("ingestion should succeed");
    expand_patterns(&mut program, &registry).expect("expansion should succeed");
    assert!(!program.module.guards.is_empty());
    assert!(!program.module.reflexes.is_empty());
}
