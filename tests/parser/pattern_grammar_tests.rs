#![feature(box_patterns)]
#![forbid(unsafe_code)]

use mirrc::parse_mirr;
use mirrc::ast::{PatternDef, MirrProgram};

fn ok_pattern(src: &str) -> PatternDef {
    let full_src = format!("{}\nmodule m {{}}", src);
    let prog = parse_mirr(&full_src).unwrap_or_else(|e| panic!("Failed to parse: {:?}", e));
    prog.patterns.into_iter().next().unwrap()
}

fn err_pattern(src: &str) -> String {
    let full_src = format!("{}\nmodule m {{}}", src);
    parse_mirr(&full_src).unwrap_err().to_string()
}

#[test]
fn test_01_empty_pattern() -> Result<(), String> {
    let p = ok_pattern("def p() { reflect {} }");
    assert_eq!(p.name, "p");
    assert!(p.params.is_empty());
    Ok(())
}

#[test]
fn test_02_single_param_signal() -> Result<(), String> {
    let p = ok_pattern("def p(s: signal in bool) { reflect {} }");
    assert_eq!(p.params.len(), 1);
    assert_eq!(p.params[0].name, "s");
    Ok(())
}

#[test]
fn test_03_multiple_params() -> Result<(), String> {
    let p = ok_pattern("def p(s: signal in bool, v: u16) { reflect {} }");
    assert_eq!(p.params.len(), 2);
    assert_eq!(p.params[1].name, "v");
    Ok(())
}

#[test]
fn test_04_constant_type_param() -> Result<(), String> {
    let p = ok_pattern("def p(c: const u32) { reflect {} }");
    assert_eq!(p.params.len(), 1);
    assert_eq!(p.params[0].name, "c");
    Ok(())
}

#[test]
fn test_05_param_duplicate_detection() -> Result<(), String> {
    // Tests duplicate parameter name detection in the parser/AST
    let err = err_pattern("def p(s: bool, s: u8) { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_06_pattern_with_guard() -> Result<(), String> {
    let p = ok_pattern("def p() { reflect { guard g { when true for 1 cycles; } } }");
    assert_eq!(p.guards.len(), 1);
    assert_eq!(p.guards[0].name, "g");
    Ok(())
}

#[test]
fn test_07_pattern_with_multiple_guards() -> Result<(), String> {
    let p = ok_pattern("def p() { reflect { guard g1 { when true; } guard g2 { when false; } } }");
    assert_eq!(p.guards.len(), 2);
    Ok(())
}

#[test]
fn test_08_pattern_with_reflex() -> Result<(), String> {
    let p = ok_pattern("def p() { reflect { reflex r { on g1 -> target = true; } } }");
    assert_eq!(p.reflexes.len(), 1);
    assert_eq!(p.reflexes[0].name, "r");
    Ok(())
}

#[test]
fn test_09_pattern_with_signals() -> Result<(), String> {
    let p = ok_pattern("def p() { reflect { signal x: internal bool; } }");
    assert_eq!(p.signals.len(), 1);
    assert_eq!(p.signals[0].name, "x");
    Ok(())
}

#[test]
fn test_10_pattern_mixed_contents() -> Result<(), String> {
    let p = ok_pattern("def p() { reflect { signal x: internal bool; guard g { when x; } reflex r { on g -> x = false; } } }");
    assert_eq!(p.signals.len(), 1);
    assert_eq!(p.guards.len(), 1);
    assert_eq!(p.reflexes.len(), 1);
    Ok(())
}

#[test]
fn test_11_pattern_missing_reflect() -> Result<(), String> {
    let err = err_pattern("def p() { signal x: internal bool; }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_12_pattern_missing_braces() -> Result<(), String> {
    let err = err_pattern("def p() reflect {}");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_13_pattern_invalid_param_type() -> Result<(), String> {
    let err = err_pattern("def p(x: unknown_type) { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_14_pattern_trailing_comma_in_params() -> Result<(), String> {
    let p = ok_pattern("def p(s: bool,) { reflect {} }");
    assert_eq!(p.params.len(), 1);
    Ok(())
}

#[test]
fn test_15_pattern_no_parens() -> Result<(), String> {
    let err = err_pattern("def p { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_16_pattern_invalid_name() -> Result<(), String> {
    let err = err_pattern("def 123p() { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_17_pattern_missing_name() -> Result<(), String> {
    let err = err_pattern("def () { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_18_pattern_with_property() -> Result<(), String> {
    let p = ok_pattern("def p() { reflect { property safe { assert g1; } } }");
    assert_eq!(p.properties.len(), 1);
    assert_eq!(p.properties[0].name, "safe");
    Ok(())
}

#[test]
fn test_19_pattern_nested_not_allowed() -> Result<(), String> {
    let err = err_pattern("def p() { reflect { def inner() { reflect {} } } }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_20_pattern_duplicate_name_detection() -> Result<(), String> {
    let err = err_pattern("def p() { reflect {} } def p() { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_21_pattern_param_without_type() -> Result<(), String> {
    let err = err_pattern("def p(x) { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_22_pattern_param_direction_out() -> Result<(), String> {
    let p = ok_pattern("def p(x: signal out bool) { reflect {} }");
    assert_eq!(p.params[0].name, "x");
    Ok(())
}

#[test]
fn test_23_pattern_param_direction_inout() -> Result<(), String> {
    let p = ok_pattern("def p(x: signal inout bool) { reflect {} }");
    assert_eq!(p.params[0].name, "x");
    Ok(())
}

#[test]
fn test_24_pattern_param_bus_width() -> Result<(), String> {
    let p = ok_pattern("def p(x: u32) { reflect {} }");
    assert_eq!(p.params[0].name, "x");
    Ok(())
}

#[test]
fn test_25_pattern_multiple_reflect_error() -> Result<(), String> {
    let err = err_pattern("def p() { reflect {} reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}
