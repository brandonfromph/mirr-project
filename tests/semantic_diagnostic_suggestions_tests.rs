#![forbid(unsafe_code)]
//! Semantic undeclared-reference suggestion tests.
//!
//! Covers E204/E205/E207/E208 "Did you mean" behavior.

use nasa_rust_project::parse_mirr;
use nasa_rust_project::validate_module;

fn validate_messages(source: &str) -> Vec<String> {
    let program = parse_mirr(source).expect("source should parse");
    let errs = validate_module(&program.module).expect_err("semantic validation should fail");
    errs.errors.iter().map(ToString::to_string).collect()
}

#[test]
fn guard_undeclared_signal_suggests_closest_match_e204() {
    let source = r#"
module e204_suggest {
    signal sensor: in bool;
    signal alarm: out bool;

    guard g {
        when senor
        for 1 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }
}
"#;

    let messages = validate_messages(source);
    assert!(messages.iter().any(|m| m.contains("[E204]")), "expected E204, got: {messages:?}");
    assert!(
        messages.iter().any(|m| m.contains("Did you mean 'sensor'?")),
        "expected suggestion for sensor, got: {messages:?}"
    );
}

#[test]
fn reflex_undeclared_guard_suggests_closest_match_e205() {
    let source = r#"
module e205_suggest {
    signal trigger: in bool;
    signal alarm: out bool;

    guard temperature_drop {
        when trigger
        for 1 cycles;
    }

    reflex r {
        on temprature_drop {
            alarm = true;
        }
    }
}
"#;

    let messages = validate_messages(source);
    assert!(messages.iter().any(|m| m.contains("[E205]")), "expected E205, got: {messages:?}");
    assert!(
        messages.iter().any(|m| m.contains("Did you mean 'temperature_drop'?")),
        "expected suggestion for temperature_drop, got: {messages:?}"
    );
}

#[test]
fn assignment_target_suggests_closest_match_e207() {
    let source = r#"
module e207_suggest {
    signal trigger: in bool;
    signal alarm: out bool;

    guard g {
        when trigger
        for 1 cycles;
    }

    reflex r {
        on g {
            alrm = true;
        }
    }
}
"#;

    let messages = validate_messages(source);
    assert!(messages.iter().any(|m| m.contains("[E207]")), "expected E207, got: {messages:?}");
    assert!(
        messages.iter().any(|m| m.contains("Did you mean 'alarm'?")),
        "expected suggestion for alarm, got: {messages:?}"
    );
}

#[test]
fn assignment_rhs_signal_suggests_closest_match_e208() {
    let source = r#"
module e208_suggest {
    signal trigger: in bool;
    signal pressure: in bool;
    signal alarm: out bool;

    guard g {
        when trigger
        for 1 cycles;
    }

    reflex r {
        on g {
            alarm = presure;
        }
    }
}
"#;

    let messages = validate_messages(source);
    assert!(messages.iter().any(|m| m.contains("[E208]")), "expected E208, got: {messages:?}");
    assert!(
        messages.iter().any(|m| m.contains("Did you mean 'pressure'?")),
        "expected suggestion for pressure, got: {messages:?}"
    );
}
