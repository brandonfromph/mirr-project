#![forbid(unsafe_code)]
//! Semantic assignment-target contract tests.
//!
//! Focuses on writable-target enforcement (E206/E207) and related RHS behavior.

use nasa_rust_project::parse_mirr;
use nasa_rust_project::validate_module;

fn validate_messages(source: &str) -> Vec<String> {
    let program = parse_mirr(source).expect("source should parse");
    let errs = validate_module(&program.module).expect_err("semantic validation should fail");
    errs.errors.iter().map(ToString::to_string).collect()
}

#[test]
fn assignment_to_input_signal_reports_e206() {
    let source = r#"
module e206_input_target {
    signal trigger: in bool;
    signal alarm: out bool;

    guard g {
        when trigger
        for 1 cycles;
    }

    reflex r {
        on g {
            trigger = true;
        }
    }
}
"#;

    let messages = validate_messages(source);
    assert!(messages.iter().any(|m| m.contains("[E206]")), "expected E206, got: {messages:?}");
    assert!(
        messages.iter().any(|m| m.contains("assigns to input signal 'trigger'")),
        "expected input-target detail, got: {messages:?}"
    );
}

#[test]
fn assignment_to_undeclared_signal_reports_e207_with_suggestion() {
    let source = r#"
module e207_target {
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
fn output_and_internal_targets_are_writable() {
    let source = r#"
module writable_targets {
    signal trigger: in bool;
    signal latch: internal bool;
    signal alarm: out bool;

    guard g {
        when trigger
        for 1 cycles;
    }

    reflex r {
        on g {
            latch = trigger;
            alarm = latch;
        }
    }
}
"#;

    let program = parse_mirr(source).expect("source should parse");
    validate_module(&program.module).expect("output/internal targets should be writable");
}

#[test]
fn undeclared_rhs_signal_reports_e208_not_target_error() {
    let source = r#"
module rhs_undeclared {
    signal trigger: in bool;
    signal alarm: out bool;

    guard g {
        when trigger
        for 1 cycles;
    }

    reflex r {
        on g {
            alarm = ghost;
        }
    }
}
"#;

    let messages = validate_messages(source);
    assert!(messages.iter().any(|m| m.contains("[E208]")), "expected E208, got: {messages:?}");
    assert!(
        !messages.iter().any(|m| m.contains("[E207]")),
        "did not expect target-undeclared E207 for declared target, got: {messages:?}"
    );
}
