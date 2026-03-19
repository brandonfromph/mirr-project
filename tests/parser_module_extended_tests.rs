#![forbid(unsafe_code)]
#![allow(clippy::needless_range_loop)]
//! Extended module parser tests.
//!
//! ~30+ tests covering module declarations, signal parsing (all kinds and types),
//! guards, reflexes, properties (all formula + directive variants), pattern defs,
//! error recovery, and edge cases.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::property::{PropertyDirective, PropertyFormula};
use nasa_rust_project::ast::types::{
    BinaryOp, EffectQualifier, Linearity, LiteralValue, SignalKind, SignalType, UnaryOp,
};
use nasa_rust_project::parse_mirr;

// =========================================================================
// Bounded iteration constants (NASA Power-of-10)
// =========================================================================

/// Maximum signals inspected in any single test loop.
const MAX_SIGNALS: usize = 64;

/// Maximum guards inspected in any single test loop.
const MAX_GUARDS: usize = 32;

/// Maximum reflexes inspected in any single test loop.
const MAX_REFLEXES: usize = 32;

/// Maximum assignments inspected in any single test loop.
const MAX_ASSIGNMENTS: usize = 64;

/// Maximum properties inspected in any single test loop.
const MAX_PROPERTIES: usize = 32;

/// Maximum pattern definitions inspected in any single test loop.
const MAX_PATTERNS: usize = 64;

// =========================================================================
// Helpers (no recursion, all bounded)
// =========================================================================

fn assert_parse_ok(source: &str) -> nasa_rust_project::MirrProgram {
    parse_mirr(source).expect("expected parse to succeed")
}

fn assert_parse_err(source: &str, msg_contains: &str) {
    let err = parse_mirr(source).expect_err("expected parse to fail");
    assert!(
        err.to_string().contains(msg_contains),
        "error '{}' should contain '{}'",
        err,
        msg_contains
    );
}

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn int(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}

fn bool_lit(v: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(v))
}

fn bin(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    Expr::Binary { op, left: Box::new(left), right: Box::new(right) }
}

fn not(e: Expr) -> Expr {
    Expr::Unary { op: UnaryOp::Not, operand: Box::new(e) }
}

/// Wrap a body of declarations inside a minimal module with base signals and a guard.
fn wrap_module(body: &str) -> String {
    format!(
        r#"
module test_mod {{
    signal x: in bool;
    signal y: out bool;
    signal z: in u16;

    guard g {{
        when x
        for 2 cycles;
    }}

    reflex r {{
        on g {{
            y = true;
        }}
    }}

    {body}
}}
"#
    )
}

// =========================================================================
// 1. Module declaration parsing
// =========================================================================

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
