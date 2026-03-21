use super::*;

#[test]
fn module_name_simple() {
    let p = assert_parse_ok("module alpha {\n}");
    assert_eq!(p.module.name, "alpha", "module name should be 'alpha'");
}

#[test]
fn module_name_with_underscores() {
    let p = assert_parse_ok("module my_test_module {\n}");
    assert_eq!(p.module.name, "my_test_module", "module name should preserve underscores");
}

#[test]
fn module_name_alphanumeric() {
    let p = assert_parse_ok("module ctrl_v2 {\n}");
    assert_eq!(p.module.name, "ctrl_v2", "module name should allow digits");
}

#[test]
fn empty_module_has_no_signals_guards_reflexes() {
    let p = assert_parse_ok("module empty {\n}");
    assert!(p.module.signals.is_empty(), "empty module should have no signals");
    assert!(p.module.guards.is_empty(), "empty module should have no guards");
    assert!(p.module.reflexes.is_empty(), "empty module should have no reflexes");
    assert!(p.module.properties.is_empty(), "empty module should have no properties");
    assert!(p.module.pattern_calls.is_empty(), "empty module should have no pattern calls");
}

#[test]
fn module_with_leading_comments_and_blanks() {
    let source = "// file header\n\n// another comment\n\nmodule commented {\n}";
    let p = assert_parse_ok(source);
    assert_eq!(p.module.name, "commented", "parser should skip leading comments");
}

#[test]
fn module_brace_on_same_line() {
    let p = assert_parse_ok("module inline_brace {\n}");
    assert_eq!(p.module.name, "inline_brace", "brace on same line should parse");
}

// =========================================================================
// 2. Signal declaration parsing — all kinds and types
// =========================================================================

#[test]
fn signal_input_bool() {
    let p = assert_parse_ok("module m {\n    signal s: in bool;\n}");
    assert_eq!(p.module.signals.len(), 1, "should have exactly one signal");
    assert_eq!(p.module.signals[0].name, "s", "signal name mismatch");
    assert_eq!(p.module.signals[0].kind, SignalKind::Input, "signal kind should be Input");
    assert_eq!(
        p.module.signals[0].ty.signal_type(),
        SignalType::Bool,
        "signal type should be Bool"
    );
}

#[test]
fn signal_output_unsigned_widths() {
    let source = r#"
module widths {
    signal a: out u1;
    signal b: out u8;
    signal c: out u16;
    signal d: out u32;
    signal e: out u64;
}
"#;
    let p = assert_parse_ok(source);
    let expected_widths: [u32; 5] = [1, 8, 16, 32, 64];
    assert_eq!(p.module.signals.len(), 5, "should have 5 signals");
    for i in 0..5 {
        assert!(i < MAX_SIGNALS, "bounded iteration guard");
        assert_eq!(
            p.module.signals[i].kind,
            SignalKind::Output,
            "signal {} should be Output",
            p.module.signals[i].name
        );
        assert_eq!(
            p.module.signals[i].ty.signal_type(),
            SignalType::Unsigned(expected_widths[i]),
            "signal {} width mismatch",
            p.module.signals[i].name
        );
    }
}

#[test]
fn signal_signed_types() {
    let source = r#"
module signed_mod {
    signal a: in i8;
    signal b: in i16;
    signal c: out i32;
    signal d: internal i64;
}
"#;
    let p = assert_parse_ok(source);
    let expected: [(SignalKind, u32); 4] = [
        (SignalKind::Input, 8),
        (SignalKind::Input, 16),
        (SignalKind::Output, 32),
        (SignalKind::Internal, 64),
    ];
    assert_eq!(p.module.signals.len(), 4, "should have 4 signed signals");
    for i in 0..4 {
        assert!(i < MAX_SIGNALS, "bounded iteration guard");
        assert_eq!(
            p.module.signals[i].kind, expected[i].0,
            "signal {} kind mismatch",
            p.module.signals[i].name
        );
        assert_eq!(
            p.module.signals[i].ty.signal_type(),
            SignalType::Signed(expected[i].1),
            "signal {} type mismatch",
            p.module.signals[i].name
        );
    }
}

#[test]
fn signal_internal_kind() {
    let p = assert_parse_ok("module m {\n    signal count: internal u32;\n}");
    assert_eq!(p.module.signals[0].kind, SignalKind::Internal, "should parse 'internal' kind");
    assert_eq!(
        p.module.signals[0].ty.signal_type(),
        SignalType::Unsigned(32),
        "internal signal type mismatch"
    );
}

#[test]
fn signal_all_three_kinds_in_one_module() {
    let source = r#"
module all_kinds {
    signal inp: in bool;
    signal outp: out u16;
    signal intern: internal i8;
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.signals[0].kind, SignalKind::Input, "first signal should be Input");
    assert_eq!(p.module.signals[1].kind, SignalKind::Output, "second signal should be Output");
    assert_eq!(p.module.signals[2].kind, SignalKind::Internal, "third signal should be Internal");
}

#[test]
fn many_signals_module() {
    let mut lines = String::from("module many_sigs {\n");
    let count = 20;
    for i in 0..count {
        assert!(i < MAX_SIGNALS, "bounded iteration guard");
        lines.push_str(&format!("    signal s{}: in u8;\n", i));
    }
    lines.push_str("}\n");
    let p = assert_parse_ok(&lines);
    assert_eq!(p.module.signals.len(), count, "should have 20 signals");
    for i in 0..count {
        assert!(i < MAX_SIGNALS, "bounded iteration guard");
        assert_eq!(
            p.module.signals[i].name,
            format!("s{}", i),
            "signal name mismatch at index {}",
            i
        );
    }
}

// =========================================================================
// 2b. MEGA-1 extended signal annotations
// =========================================================================

#[test]
fn signal_linear_qualifier() {
    let p = assert_parse_ok("module m {\n    signal s: in linear bool;\n}");
    assert_eq!(
        p.module.signals[0].ty.annotations.linearity,
        Linearity::Linear,
        "should parse 'linear' qualifier"
    );
}

#[test]
fn signal_stateful_qualifier() {
    let p = assert_parse_ok("module m {\n    signal s: internal stateful u16;\n}");
    assert_eq!(
        p.module.signals[0].ty.annotations.effect,
        EffectQualifier::Stateful,
        "should parse 'stateful' qualifier"
    );
}

#[test]
fn signal_pure_qualifier() {
    let p = assert_parse_ok("module m {\n    signal s: out pure bool;\n}");
    assert_eq!(
        p.module.signals[0].ty.annotations.effect,
        EffectQualifier::Pure,
        "should parse 'pure' qualifier"
    );
}

#[test]
fn signal_clock_domain_annotation() {
    let p = assert_parse_ok("module m {\n    signal s: in u8 @fast_clk;\n}");
    assert_eq!(
        p.module.signals[0].ty.annotations.clock_domain.as_deref(),
        Some("fast_clk"),
        "should parse clock domain annotation"
    );
}

#[test]
fn signal_phantom_tag_annotation() {
    let p = assert_parse_ok("module m {\n    signal s: out u16 #Pressure;\n}");
    assert_eq!(
        p.module.signals[0].ty.annotations.phantom_tag.as_deref(),
        Some("Pressure"),
        "should parse phantom tag annotation"
    );
}

// =========================================================================
// 3. Guard parsing
// =========================================================================

#[test]
fn guard_simple_condition() {
    let source = r#"
module m {
    signal a: in bool;
    guard g1 {
        when a
        for 5 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards.len(), 1, "should have one guard");
    assert_eq!(p.module.guards[0].name, "g1", "guard name mismatch");
    assert_eq!(p.module.guards[0].condition, sig("a"), "guard condition should be signal 'a'");
    assert_eq!(p.module.guards[0].cycles, 5, "guard cycles should be 5");
}

#[test]
fn guard_complex_boolean_condition() {
    let source = r#"
module m {
    signal a: in bool;
    signal b: in bool;
    guard complex {
        when a && !b
        for 10 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(
        p.module.guards[0].condition,
        bin(BinaryOp::And, sig("a"), not(sig("b"))),
        "guard condition should be 'a && !b'"
    );
    assert_eq!(p.module.guards[0].cycles, 10, "guard cycles should be 10");
}

#[test]
fn guard_comparison_condition() {
    let source = r#"
module m {
    signal pressure: in u16;
    guard over_limit {
        when pressure > 100
        for 500 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(
        p.module.guards[0].condition,
        bin(BinaryOp::Gt, sig("pressure"), int(100)),
        "guard condition should be 'pressure > 100'"
    );
    assert_eq!(p.module.guards[0].cycles, 500, "guard cycles should be 500");
}

#[test]
fn guard_for_clause_without_trailing_semicolon() {
    let source = r#"
module m {
    signal s: in bool;
    guard g {
        when s
        for 7 cycles
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards[0].cycles, 7, "should parse cycles without trailing semicolon");
}

#[test]
fn guard_single_cycle() {
    let source = r#"
module m {
    signal s: in bool;
    guard instant {
        when s
        for 1 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards[0].cycles, 1, "guard with 1 cycle should parse");
}

#[test]
fn multiple_guards() {
    let source = r#"
module m {
    signal a: in bool;
    signal b: in bool;
    guard g1 {
        when a
        for 1 cycles;
    }
    guard g2 {
        when b
        for 2 cycles;
    }
    guard g3 {
        when a && b
        for 3 cycles;
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.guards.len(), 3, "should have three guards");
    for i in 0..3 {
        assert!(i < MAX_GUARDS, "bounded iteration guard");
    }
    assert_eq!(p.module.guards[0].name, "g1", "first guard name mismatch");
    assert_eq!(p.module.guards[1].name, "g2", "second guard name mismatch");
    assert_eq!(p.module.guards[2].name, "g3", "third guard name mismatch");
    assert_eq!(p.module.guards[2].cycles, 3, "third guard cycles mismatch");
}

// =========================================================================
// 4. Reflex parsing
// =========================================================================

#[test]
fn reflex_single_guard_single_assignment() {
    let source = r#"
module m {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.reflexes.len(), 1, "should have one reflex");
    assert_eq!(p.module.reflexes[0].name, "r", "reflex name mismatch");
    assert_eq!(p.module.reflexes[0].guard_names, vec!["g"], "reflex should reference guard 'g'");
    assert_eq!(p.module.reflexes[0].assignments.len(), 1, "reflex should have one assignment");
    assert_eq!(p.module.reflexes[0].assignments[0].target, "b", "assignment target should be 'b'");
    assert_eq!(
        p.module.reflexes[0].assignments[0].value,
        bool_lit(true),
        "assignment value should be true"
    );
}

#[test]
fn reflex_multiple_guards_with_and() {
    let source = r#"
module m {
    signal a: in bool;
    signal b: in bool;
    signal c: out bool;
    guard g1 {
        when a
        for 1 cycles;
    }
    guard g2 {
        when b
        for 1 cycles;
    }
    reflex r {
        on g1 and g2 {
            c = true;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(
        p.module.reflexes[0].guard_names,
        vec!["g1", "g2"],
        "reflex should reference both guards"
    );
}

#[test]
fn reflex_multiple_assignments() {
    let source = r#"
module m {
    signal a: in bool;
    signal b: out bool;
    signal c: out u16;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = false;
            c = 42;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(p.module.reflexes[0].assignments.len(), 2, "reflex should have two assignments");
    assert_eq!(p.module.reflexes[0].assignments[0].target, "b", "first assignment target mismatch");
    assert_eq!(
        p.module.reflexes[0].assignments[0].value,
        bool_lit(false),
        "first assignment value mismatch"
    );
    assert_eq!(
        p.module.reflexes[0].assignments[1].target, "c",
        "second assignment target mismatch"
    );
    assert_eq!(
        p.module.reflexes[0].assignments[1].value,
        int(42),
        "second assignment value mismatch"
    );
}

#[test]
fn reflex_arithmetic_expression_assignment() {
    let source = r#"
module m {
    signal a: in u16;
    signal b: in u16;
    signal result: out u16;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            result = a + b;
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(
        p.module.reflexes[0].assignments[0].value,
        bin(BinaryOp::Add, sig("a"), sig("b")),
        "assignment should be 'a + b'"
    );
}

#[test]
fn reflex_with_inline_comment_in_assignment() {
    let source = r#"
module m {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = true; // emergency shutoff
        }
    }
}
"#;
    let p = assert_parse_ok(source);
    assert_eq!(
        p.module.reflexes[0].assignments[0].value,
        bool_lit(true),
        "inline comment should be stripped from assignment"
    );
}

// =========================================================================
// 5. Property parsing — all formula types and directives
// =========================================================================

#[test]
fn property_always_simple() {
    let src = wrap_module("property p {\n    always (x);\n}");
    let p = assert_parse_ok(&src);
    assert_eq!(p.module.properties.len(), 1, "should have one property");
    assert_eq!(p.module.properties[0].name, "p", "property name mismatch");
    assert!(
        matches!(p.module.properties[0].formula, PropertyFormula::Always(Expr::Signal(ref s)) if s == "x"),
        "should parse always(x) formula"
    );
    assert_eq!(
        p.module.properties[0].directive,
        PropertyDirective::Assert,
        "default directive should be Assert"
    );
}

#[test]
fn property_never_simple() {
    let src = wrap_module("property no_y {\n    never (y);\n}");
    let p = assert_parse_ok(&src);
    assert!(
        matches!(p.module.properties[0].formula, PropertyFormula::Never(Expr::Signal(ref s)) if s == "y"),
        "should parse never(y) formula"
    );
}

#[test]
fn property_always_implies() {
    let src = wrap_module("property imp {\n    always (x -> y);\n}");
    let p = assert_parse_ok(&src);
    match &p.module.properties[0].formula {
        PropertyFormula::AlwaysImplies { antecedent, consequent } => {
            assert_eq!(*antecedent, sig("x"), "antecedent should be signal x");
            assert_eq!(*consequent, sig("y"), "consequent should be signal y");
        }
        other => panic!("expected AlwaysImplies, got: {:?}", other),
    }
}

#[test]
fn property_never_implies() {
    let src = wrap_module("property ni {\n    never (x -> y);\n}");
    let p = assert_parse_ok(&src);
    match &p.module.properties[0].formula {
        PropertyFormula::NeverImplies { antecedent, consequent } => {
            assert_eq!(*antecedent, sig("x"), "NeverImplies antecedent should be x");
            assert_eq!(*consequent, sig("y"), "NeverImplies consequent should be y");
        }
        other => panic!("expected NeverImplies, got: {:?}", other),
    }
}

#[test]
fn property_eventually_within() {
    let src = wrap_module("property ev {\n    eventually within 10 (y);\n}");
    let p = assert_parse_ok(&src);
    match &p.module.properties[0].formula {