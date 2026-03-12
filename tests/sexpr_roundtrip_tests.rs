#![forbid(unsafe_code)]
//! Round-trip tests: AST -> S-expr -> AST for all parseable examples.

use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::sexpr::convert::{ast_to_sexpr, sexpr_to_ast};
use nasa_rust_project::sexpr::parser::parse_sexpr;
use nasa_rust_project::sexpr::printer::print_sexpr;

/// Helper: parse MIRR source, convert to S-expr, convert back, compare.
fn roundtrip_check(source: &str) {
    let program = parse_mirr(source).expect("parse_mirr failed");
    let sexpr = ast_to_sexpr(&program);
    let roundtripped = sexpr_to_ast(&sexpr).expect("sexpr_to_ast failed");

    // Compare fields (spans are stripped in conversion).
    assert_eq!(program.patterns.len(), roundtripped.patterns.len(), "pattern count mismatch");
    assert_eq!(program.module.name, roundtripped.module.name, "module name mismatch");
    assert_eq!(
        program.module.signals.len(),
        roundtripped.module.signals.len(),
        "signal count mismatch"
    );
    assert_eq!(
        program.module.guards.len(),
        roundtripped.module.guards.len(),
        "guard count mismatch"
    );
    assert_eq!(
        program.module.reflexes.len(),
        roundtripped.module.reflexes.len(),
        "reflex count mismatch"
    );
    assert_eq!(
        program.module.properties.len(),
        roundtripped.module.properties.len(),
        "property count mismatch"
    );

    // Verify signal names and types match.
    for (orig, rt) in program.module.signals.iter().zip(roundtripped.module.signals.iter()) {
        assert_eq!(orig.name, rt.name, "signal name mismatch");
        assert_eq!(orig.kind, rt.kind, "signal kind mismatch for {}", orig.name);
        assert_eq!(orig.ty.core, rt.ty.core, "signal type mismatch for {}", orig.name);
    }

    // Verify guard names and cycle counts.
    for (orig, rt) in program.module.guards.iter().zip(roundtripped.module.guards.iter()) {
        assert_eq!(orig.name, rt.name, "guard name mismatch");
        assert_eq!(orig.cycles, rt.cycles, "guard cycles mismatch for {}", orig.name);
    }
}

/// Helper: parse -> S-expr -> print -> re-parse S-expr -> back to AST.
fn roundtrip_text_check(source: &str) {
    let program = parse_mirr(source).expect("parse_mirr failed");
    let sexpr = ast_to_sexpr(&program);
    let printed = print_sexpr(&sexpr);
    let reparsed = parse_sexpr(&printed).expect("re-parse of printed S-expr failed");
    let roundtripped = sexpr_to_ast(&reparsed).expect("sexpr_to_ast on reparsed failed");

    assert_eq!(program.module.name, roundtripped.module.name);
    assert_eq!(program.module.signals.len(), roundtripped.module.signals.len());
}

// =========================================================================
// Example file round-trip tests (only files that actually exist)
// =========================================================================

fn read_example(name: &str) -> String {
    std::fs::read_to_string(format!("examples/{name}"))
        .unwrap_or_else(|e| panic!("Failed to read examples/{name}: {e}"))
}

macro_rules! roundtrip_example {
    ($test_name:ident, $file:expr) => {
        #[test]
        fn $test_name() {
            let source = read_example($file);
            roundtrip_check(&source);
        }
    };
}

macro_rules! roundtrip_text_example {
    ($test_name:ident, $file:expr) => {
        #[test]
        fn $test_name() {
            let source = read_example($file);
            roundtrip_text_check(&source);
        }
    };
}

roundtrip_example!(roundtrip_tmr, "tmr_sensor_fusion.mirr");
roundtrip_text_example!(roundtrip_tmr_text, "tmr_sensor_fusion.mirr");

roundtrip_example!(roundtrip_neonatal, "neonatal_respirator.mirr");
roundtrip_text_example!(roundtrip_neonatal_text, "neonatal_respirator.mirr");

roundtrip_example!(roundtrip_icu, "icu_monitor.mirr");
roundtrip_text_example!(roundtrip_icu_text, "icu_monitor.mirr");

roundtrip_example!(roundtrip_industrial, "industrial_safety.mirr");
roundtrip_text_example!(roundtrip_industrial_text, "industrial_safety.mirr");

roundtrip_example!(roundtrip_flight, "flight_controller.mirr");
roundtrip_text_example!(roundtrip_flight_text, "flight_controller.mirr");

roundtrip_example!(roundtrip_flight_signed, "flight_controller_signed.mirr");
roundtrip_text_example!(roundtrip_flight_signed_text, "flight_controller_signed.mirr");

roundtrip_example!(roundtrip_autonomous, "autonomous_vehicle.mirr");
roundtrip_text_example!(roundtrip_autonomous_text, "autonomous_vehicle.mirr");

roundtrip_example!(roundtrip_multi_guard, "multi_guard_monitor.mirr");
roundtrip_text_example!(roundtrip_multi_guard_text, "multi_guard_monitor.mirr");

roundtrip_example!(roundtrip_safety_property, "safety_property.mirr");
roundtrip_text_example!(roundtrip_safety_property_text, "safety_property.mirr");

roundtrip_example!(roundtrip_fir_filter, "fir_filter.mirr");
roundtrip_text_example!(roundtrip_fir_filter_text, "fir_filter.mirr");

roundtrip_example!(roundtrip_shift_register, "shift_register_guard.mirr");
roundtrip_text_example!(roundtrip_shift_register_text, "shift_register_guard.mirr");

roundtrip_example!(roundtrip_pattern_usage, "pattern_usage.mirr");
roundtrip_text_example!(roundtrip_pattern_usage_text, "pattern_usage.mirr");

// =========================================================================
// Targeted type round-trip tests
// =========================================================================

#[test]
fn roundtrip_signal_bool() {
    roundtrip_check("module test {\n  signal x : in bool;\n}");
}

#[test]
fn roundtrip_signal_unsigned() {
    roundtrip_check("module test {\n  signal x : in u16;\n}");
}

#[test]
fn roundtrip_signal_signed() {
    roundtrip_check("module test {\n  signal x : in i32;\n}");
}

#[test]
fn roundtrip_guard() {
    let source =
        "module test {\n  signal x : in bool;\n  guard g {\n    when x\n    for 3 cycles;\n  }\n}";
    roundtrip_check(source);
}

#[test]
fn roundtrip_reflex() {
    let source = "module test {\n  signal x : in bool;\n  signal y : out bool;\n  guard g {\n    when x\n    for 1 cycles;\n  }\n  reflex r {\n    on g {\n      y = true;\n    }\n  }\n}";
    roundtrip_check(source);
}

#[test]
fn roundtrip_all_binary_ops() {
    let source = r#"
module test {
    signal a : in u16;
    signal b : in u16;
    signal out_add : out u16;
    signal out_sub : out u16;
    signal out_lt : out bool;
    guard g {
        when a > 0
        for 1 cycles;
    }
    reflex r {
        on g {
            out_add = a + b;
            out_sub = a - b;
            out_lt = a < b;
        }
    }
}
"#;
    roundtrip_check(source);
}

#[test]
fn roundtrip_nested_expr() {
    let source = "module test {\n  signal x : in u16;\n  signal y : in u16;\n  guard g {\n    when x + y > 0\n    for 1 cycles;\n  }\n}";
    roundtrip_check(source);
}

#[test]
fn roundtrip_empty_module() {
    roundtrip_check("module empty {\n}");
}

#[test]
fn roundtrip_property_always() {
    let source =
        "module test {\n  signal x : in u16;\n  property p {\n    always (x < 1000);\n  }\n}";
    roundtrip_check(source);
}

#[test]
fn sexpr_convert_then_print_then_reparse() {
    let source = "module test {\n  signal x : in bool;\n}";
    let program = parse_mirr(source).unwrap();
    let sexpr = ast_to_sexpr(&program);
    let text = print_sexpr(&sexpr);
    let reparsed = parse_sexpr(&text).unwrap();
    assert_eq!(sexpr, reparsed, "S-expr print -> parse roundtrip failed");
}
