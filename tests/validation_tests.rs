#![forbid(unsafe_code)]
// ---------------------------------------------------------------------------
// Semantic validation tests
// ---------------------------------------------------------------------------

use mirrc::parse_mirr;
use mirrc::validate_module;
use mirrc::MirrProgram;

// -- Helpers --

fn assert_parse_ok(source: &str) -> MirrProgram {
    parse_mirr(source).expect("expected parse to succeed")
}

fn assert_validate_ok(source: &str) -> MirrProgram {
    let p = assert_parse_ok(source);
    validate_module(&p.module).expect("expected validation to pass");
    p
}

fn assert_validate_err(source: &str, msg_contains: &str) {
    let p = assert_parse_ok(source);
    let err = validate_module(&p.module).expect_err("expected validation to fail");
    assert!(
        err.to_string().contains(msg_contains),
        "error '{}' should contain '{}'",
        err,
        msg_contains
    );
}

// -- Tests --

#[test]
fn validate_neonatal_respirator() {
    let source = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;
    assert_validate_ok(source);
}

#[test]
fn validate_err_guard_references_undeclared_signal() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when nonexistent
        for 1 cycles;
    }
}
"#;
    assert_validate_err(source, "undeclared signal 'nonexistent'");
}

#[test]
fn validate_err_reflex_references_undeclared_guard() {
    let source = r#"
module m {
    signal s: out bool;
    reflex r {
        on missing_guard {
            s = true;
        }
    }
}
"#;
    assert_validate_err(source, "undeclared guard 'missing_guard'");
}

#[test]
fn validate_err_assign_to_input() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for 1 cycles;
    }
    reflex r {
        on g {
            s = true;
        }
    }
}
"#;
    assert_validate_err(source, "input signal 's', which is not writable");
}

#[test]
fn validate_err_assign_to_undeclared() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for 1 cycles;
    }
    reflex r {
        on g {
            ghost = true;
        }
    }
}
"#;
    assert_validate_err(source, "undeclared signal 'ghost'");
}

#[test]
fn validate_err_assignment_rhs_undeclared() {
    let source = r#"
module m {
    signal s: out bool;
    guard g {
        when s
        for 1 cycles;
    }
    reflex r {
        on g {
            s = phantom;
        }
    }
}
"#;
    assert_validate_err(source, "undeclared signal 'phantom'");
}
