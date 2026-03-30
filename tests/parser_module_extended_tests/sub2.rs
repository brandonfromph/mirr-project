use super::*;

#[test]
fn property_eventually_within() {
    let src = wrap_module("property ev {\n    eventually within 10 (y);\n}");
    let p = assert_parse_ok(&src);
    match &p.module.properties[0].formula {
        PropertyFormula::EventuallyWithin { expr, cycles } => {
            assert_eq!(*expr, sig("y"), "eventually within expr should be y");
            assert_eq!(*cycles, 10, "eventually within cycles should be 10");
        }
        other => panic!("expected EventuallyWithin, got: {:?}", other),
    }
}

#[test]
fn property_always_followed_by() {
    let src = wrap_module("property fb {\n    always (x followed_by 5 y);\n}");
    let p = assert_parse_ok(&src);
    match &p.module.properties[0].formula {
        PropertyFormula::AlwaysFollowedBy { trigger, response, delay_cycles } => {
            assert_eq!(*trigger, sig("x"), "followed_by trigger should be x");
            assert_eq!(*response, sig("y"), "followed_by response should be y");
            assert_eq!(*delay_cycles, 5, "followed_by delay should be 5");
        }
        other => panic!("expected AlwaysFollowedBy, got: {:?}", other),
    }
}

#[test]
fn property_cover_directive() {
    let src = wrap_module("property cov {\n    cover always (x);\n}");
    let p = assert_parse_ok(&src);
    assert_eq!(
        p.module.properties[0].directive,
        PropertyDirective::Cover,
        "directive should be Cover"
    );
}

#[test]
fn property_assume_directive() {
    let src = wrap_module("property asm {\n    assume always (x);\n}");
    let p = assert_parse_ok(&src);
    assert_eq!(
        p.module.properties[0].directive,
        PropertyDirective::Assume,
        "directive should be Assume"
    );
}

#[test]
fn property_complex_implies_expression() {
    let src = wrap_module("property cplx {\n    always (z < 50 -> x && y);\n}");
    let p = assert_parse_ok(&src);
    match &p.module.properties[0].formula {
        PropertyFormula::AlwaysImplies { antecedent, consequent } => {
            assert!(
                matches!(antecedent, Expr::Binary { op: BinaryOp::Lt, .. }),
                "antecedent should be a Lt comparison"
            );
            assert!(
                matches!(consequent, Expr::Binary { op: BinaryOp::And, .. }),
                "consequent should be an And expression"
            );
        }
        other => panic!("expected AlwaysImplies with complex exprs, got: {:?}", other),
    }
}

#[test]
fn multiple_properties() {
    let src = wrap_module(
        r#"property p1 {
        always (x);
    }

    property p2 {
        never (y);
    }"#,
    );
    let p = assert_parse_ok(&src);
    assert_eq!(p.module.properties.len(), 2, "should have two properties");
    assert_eq!(p.module.properties[0].name, "p1", "first property name mismatch");
    assert_eq!(p.module.properties[1].name, "p2", "second property name mismatch");
}

// =========================================================================
// 6. Pattern definition parsing
// =========================================================================

#[test]
fn pattern_def_before_module() {
    let source = r#"
def watchdog(threshold: u16) {
    reflect {
        signal timer_val: internal u16;
        guard timeout {
            when timer_val > threshold
            for 1 cycles;
        }
    }
}

module m {
    signal s: in bool;
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.patterns.len(), 1, "should have one pattern def");
    assert_eq!(p.patterns[0].name, "watchdog", "pattern name should be 'watchdog'");
}

#[test]
fn multiple_pattern_defs() {
    let source = r#"
def pat_a(x: bool) {
    reflect {
        signal a_sig: internal bool;
    }
}

def pat_b(y: u8) {
    reflect {
        signal b_sig: internal u8;
    }
}

module m {
    signal s: in bool;
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.patterns.len(), 2, "should have two pattern defs");
    for i in 0..2 {
        assert!(i < MAX_PATTERNS, "bounded iteration guard");
    }
    assert_eq!(p.patterns[0].name, "pat_a", "first pattern name mismatch");
    assert_eq!(p.patterns[1].name, "pat_b", "second pattern name mismatch");
}

// =========================================================================
// 7. Error recovery and error messages
// =========================================================================

#[test]
fn err_empty_source() {
    assert_parse_err("", "empty");
}

#[test]
fn err_only_comments() {
    assert_parse_err("// just a comment\n// another\n", "empty");
}

#[test]
fn err_no_module_keyword() {
    assert_parse_err("signal x: in bool;", "module");
}

#[test]
fn err_module_name_empty() {
    assert_parse_err("module  {", "Module name cannot be empty");
}

#[test]
fn err_module_not_closed() {
    assert_parse_err("module m {\n    signal s: in bool;\n", "not closed");
}

#[test]
fn err_signal_missing_semicolon() {
    assert_parse_err("module m {\n    signal x: in bool\n}", "end with");
}

#[test]
fn err_signal_missing_colon() {
    assert_parse_err("module m {\n    signal x in bool;\n}", "contain ':'");
}

#[test]
fn err_signal_unknown_kind() {
    assert_parse_err("module m {\n    signal x: inout bool;\n}", "Unknown signal kind");
}

#[test]
fn err_signal_unknown_type() {
    assert_parse_err("module m {\n    signal x: in float32;\n}", "Unknown signal type");
}

#[test]
fn err_signal_empty_name() {
    assert_parse_err("module m {\n    signal : in bool;\n}", "empty");
}

#[test]
fn err_guard_missing_when() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        for 1 cycles;
    }
}
"#;
    assert_parse_err(source, "when");
}

#[test]
fn err_guard_missing_for() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
    }
}
"#;
    assert_parse_err(source, "for");
}

#[test]
fn err_guard_invalid_cycle_count() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for xyz cycles;
    }
}
"#;
    assert_parse_err(source, "Invalid cycle count");
}

#[test]
fn err_guard_empty_name() {
    let source = "module m {\n    guard  {\n        when true\n        for 1 cycles;\n    }\n}";
    assert_parse_err(source, "Guard name cannot be empty");
}

#[test]
fn err_reflex_no_guard_in_on_clause() {
    let source = r#"
module m {
    signal s: out bool;
    guard g {
        when s
        for 1 cycles;
    }
    reflex r {
        on {
        }
    }
}
"#;
    assert_parse_err(source, "must contain at least one assignment");
}

#[test]
fn err_reflex_empty_name() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for 1 cycles;
    }
    reflex  {
        on g {
        }
    }
}
"#;
    assert_parse_err(source, "Reflex name cannot be empty");
}

#[test]
fn err_unexpected_line_in_module() {
    assert_parse_err("module m {\n    garbage here\n}", "Unexpected");
}

#[test]
fn err_property_bad_formula_keyword() {
    let src = wrap_module("property p {\n    sometimes (x);\n}");
    assert_parse_err(&src, "must start with");
}

#[test]
fn err_property_not_closed() {
    let source = r#"
module m {
    signal x: in bool;
    property p {
        always (x);
"#;
    assert_parse_err(source, "not closed");
}

#[test]
fn err_property_empty_name() {
    let source =
        "module m {\n    signal x: in bool;\n    property  {\n        always (x);\n    }\n}";
    assert_parse_err(source, "Property name cannot be empty");
}

// =========================================================================
// 8. Edge cases
// =========================================================================

#[test]
fn module_with_comments_between_declarations() {
    let source = r#"
module interleaved {
    signal a: in bool;
    // comment between signals
    signal b: out u8;

    // comment before guard
    guard g {
        when a
        for 3 cycles;
    }

    // comment before reflex
    reflex r {
        on g {
            b = 10;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.signals.len(), 2, "should have 2 signals despite comments");
    assert_eq!(p.module.guards.len(), 1, "should have 1 guard despite comments");
    assert_eq!(p.module.reflexes.len(), 1, "should have 1 reflex despite comments");
}

#[test]
fn full_neonatal_respirator_module() {
    let source = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure: in u16;
    signal clamp_valve: out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for 1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }

    property safety_response {
        always (airway_pressure < 50 -> clamp_valve);
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.name, "neonatal_respirator", "module name mismatch");
    assert_eq!(p.module.signals.len(), 3, "should have 3 signals");
    assert_eq!(p.module.guards.len(), 1, "should have 1 guard");
    assert_eq!(p.module.guards[0].cycles, 1000, "guard cycles should be 1000");
    assert_eq!(p.module.reflexes.len(), 1, "should have 1 reflex");
    assert_eq!(p.module.properties.len(), 1, "should have 1 property");
    assert_eq!(p.module.properties[0].name, "safety_response", "property name mismatch");
}

#[test]
fn span_is_set_on_module() {
    let p = assert_parse_ok("module spanned {\n    signal x: in bool;\n}");
    assert!(p.module.span.is_some(), "module span should be set");
}

#[test]
fn span_is_set_on_signal() {
    let p = assert_parse_ok("module m {\n    signal x: in bool;\n}");
    assert!(p.module.signals[0].span.is_some(), "signal span should be set");
}

#[test]
fn span_is_set_on_guard() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for 1 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert!(p.module.guards[0].span.is_some(), "guard span should be set");
}

#[test]
fn span_is_set_on_reflex() {
    let source = r#"
module m {
    signal s: in bool;
    signal o: out bool;
    guard g {
        when s
        for 1 cycles;
    }
    reflex r {
        on g {
            o = true;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert!(p.module.reflexes[0].span.is_some(), "reflex span should be set");
}

#[test]
fn assignment_expression_with_nested_ops() {
    let source = r#"
module m {
    signal a: in u16;
    signal b: in u16;
    signal c: in u16;
    signal result: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            result = a + b * c;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    // a + (b * c) due to precedence
    let expected = bin(BinaryOp::Add, sig("a"), bin(BinaryOp::Mul, sig("b"), sig("c")));
    assert_eq!(
        p.module.reflexes[0].assignments[0].value, expected,
        "should respect operator precedence: a + (b * c)"
    );
}

#[test]
fn guard_with_large_cycle_count() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for 1000000 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards[0].cycles, 1_000_000, "should parse large cycle count");
}

#[test]
fn reflex_with_many_assignments() {
    let mut source = String::from(
        r#"
module m {
    signal trigger: in bool;
"#,
    );
    let count = 10;
    for i in 0..count {
        assert!(i < MAX_ASSIGNMENTS, "bounded iteration guard");
        source.push_str(&format!("    signal out_{}: out u8;\n", i));
    }
    source.push_str(
        r#"    guard g {
        when trigger
        for 1 cycles;
    }
    reflex r {
        on g {
"#,
    );
    for i in 0..count {
        assert!(i < MAX_ASSIGNMENTS, "bounded iteration guard");
        source.push_str(&format!("            out_{} = {};\n", i, i));
    }
    source.push_str("        }\n    }\n}\n");
    let p = assert_parse_ok(&source);
    assert_eq!(p.module.reflexes[0].assignments.len(), count, "should parse all 10 assignments");
    for i in 0..count {
        assert!(i < MAX_ASSIGNMENTS, "bounded iteration guard");
        assert_eq!(
            p.module.reflexes[0].assignments[i].target,
            format!("out_{}", i),
            "assignment {} target mismatch",
            i
        );
        assert_eq!(
            p.module.reflexes[0].assignments[i].value,
            int(i as u64),
            "assignment {} value mismatch",
            i
        );
    }
}

#[test]
fn property_with_nested_boolean_logic() {
    let src = wrap_module("property nested {\n    always ((x && y) || z < 100);\n}");
    let p = assert_parse_ok(&src);
    assert!(
        matches!(
            p.module.properties[0].formula,
            PropertyFormula::Always(Expr::Binary { op: BinaryOp::Or, .. })
        ),
        "should parse nested boolean logic in always formula"
    );
}

#[test]
fn complete_module_with_all_construct_types() {
    let source = r#"
module full {
    signal enable: in bool;
    signal sensor: in u16;
    signal alarm: out bool;
    signal counter: internal u32;

    guard sensor_high {
        when sensor > 1000
        for 100 cycles;
    }

    guard enabled_check {
        when enable
        for 1 cycles;
    }

    reflex trigger_alarm {
        on sensor_high and enabled_check {
            alarm = true;
        }
    }

    property always_safe {
        always (sensor > 1000 -> alarm);
    }

    property never_false_alarm {
        never (alarm && sensor < 500);
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.name, "full", "module name mismatch");
    assert_eq!(p.module.signals.len(), 4, "should have 4 signals");
    assert_eq!(p.module.guards.len(), 2, "should have 2 guards");
    assert_eq!(p.module.reflexes.len(), 1, "should have 1 reflex");
    for i in 0..p.module.reflexes.len() {
        assert!(i < MAX_REFLEXES, "bounded iteration guard for reflexes");
    }
    assert_eq!(p.module.reflexes[0].guard_names.len(), 2, "reflex should reference 2 guards");
    assert_eq!(p.module.properties.len(), 2, "should have 2 properties");
    for i in 0..p.module.properties.len() {
        assert!(i < MAX_PROPERTIES, "bounded iteration guard");
    }
}
