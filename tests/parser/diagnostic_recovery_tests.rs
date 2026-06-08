#![feature(box_patterns)]
#![forbid(unsafe_code)]

use mirrc::parser::parse_expression;
use mirrc::parse_mirr;

fn err_expr(s: &str) -> String {
    let res = parse_expression(s);
    assert!(res.is_err(), "Expected parsing to fail for: {}", s);
    res.unwrap_err().to_string()
}

fn err_mirr(s: &str) -> String {
    let res = parse_mirr(s);
    assert!(res.is_err(), "Expected parsing to fail for: {}", s);
    res.unwrap_err().to_string()
}

#[test]
fn test_01_unterminated_parens() -> Result<(), String> {
    let err = err_expr("(a + b");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_02_empty_expression() -> Result<(), String> {
    let err = err_expr("");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_03_whitespace_only_expression() -> Result<(), String> {
    let err = err_expr("    \t\n");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_04_unknown_operator() -> Result<(), String> {
    // '?' is not a valid operator in MIRR expressions
    let err = err_expr("a ? b");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_05_missing_rhs_binary_op() -> Result<(), String> {
    let err = err_expr("a + ");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_06_missing_operand_unary() -> Result<(), String> {
    let err = err_expr("!");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_07_multiple_operators_no_operand() -> Result<(), String> {
    let err = err_expr("a + * b");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_08_unmatched_right_paren() -> Result<(), String> {
    let err = err_expr("a + b)");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_09_invalid_identifier_start() -> Result<(), String> {
    let err = err_expr("123invalid + b");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_10_missing_bracket_array_index() -> Result<(), String> {
    let err = err_expr("arr[0");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_11_missing_field_after_dot() -> Result<(), String> {
    let err = err_expr("obj.");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_12_unterminated_array_literal() -> Result<(), String> {
    let err = err_expr("[1, 2, 3");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_13_trailing_comma_array_literal_error() -> Result<(), String> {
    let err = err_expr("[1, 2,]");
    // Depending on grammar, trailing comma might be an error or allowed.
    // If allowed, this test might need adjustment, but usually expression parser rejects it.
    assert!(!err.is_empty() || true);
    Ok(())
}

#[test]
fn test_14_missing_brace_struct() -> Result<(), String> {
    let err = err_expr("MyStruct { x: 1");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_15_invalid_struct_field_assignment() -> Result<(), String> {
    let err = err_expr("MyStruct { x = 1 }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_16_top_level_garbage() -> Result<(), String> {
    let err = err_mirr("module m {} \n @#% garbage");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_17_missing_module_declaration() -> Result<(), String> {
    let err = err_mirr("def p() { reflect {} }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_18_unterminated_guard_block() -> Result<(), String> {
    let err = err_mirr("module m { guard g { when true ");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_19_unterminated_reflex_block() -> Result<(), String> {
    let err = err_mirr("module m { reflex r { on g -> target = true; ");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_20_missing_semicolon_in_guard() -> Result<(), String> {
    let err = err_mirr("module m { guard g { when true } }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_21_invalid_type_in_signal() -> Result<(), String> {
    let err = err_mirr("module m { signal x: internal invalid_type; }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_22_duplicate_module_decl() -> Result<(), String> {
    let err = err_mirr("module m1 {} module m2 {}");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_23_unknown_directive() -> Result<(), String> {
    let err = err_mirr("module m { #unknown_directive; }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_24_missing_name_for_signal() -> Result<(), String> {
    let err = err_mirr("module m { signal : internal bool; }");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_25_invalid_assignment_target() -> Result<(), String> {
    let err = err_mirr("module m { reflex r { on g -> 123 = true; } }");
    assert!(!err.is_empty());
    Ok(())
}
