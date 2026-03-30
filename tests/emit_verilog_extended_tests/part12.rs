use super::*;

// Section 12: Condition Expression Rendering
// ===========================================================================

#[test]
fn condition_simple_signal() {
    let source = r#"
module cond_simple {
    signal trigger: in bool;
    signal out: out bool;

    guard g {
        when trigger
        for 3 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(
        sv.contains("assign g_cond = trigger"),
        "simple signal condition must assign directly: got\n{sv}"
    );
}

#[test]
fn condition_negated_signal() {
    let source = r#"
module cond_negated {
    signal active: in bool;
    signal out: out bool;

    guard g {
        when !active
        for 3 cycles;
    }

    reflex r {
        on g {
            out = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("!active"), "negated signal condition must contain !active");
}

#[test]
fn condition_comparison_gt() {
    let source = r#"
module cond_cmp {
    signal pressure: in u16;
    signal alarm: out bool;

    guard g {
        when pressure > 500
        for 4 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }
}
"#;
    let result = run_pipeline(source, &default_config()).unwrap();
    let sv = verilog::emit_sv(&result);

    assert!(sv.contains("pressure > 500"), "comparison condition must contain 'pressure > 500'");
}

// ===========================================================================
