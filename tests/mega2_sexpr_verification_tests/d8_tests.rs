use super::*;

// ===========================================================================
// D8: parser_error_codes (15 tests)
// ===========================================================================

#[test]
fn test_d8_empty_input_error() {
    let r = parse_sexpr("");
    assert!(r.is_err(), "Empty input should fail");
}

#[test]
fn test_d8_unbalanced_open_paren() {
    let r = parse_sexpr("(a b");
    assert!(r.is_err());
}

#[test]
fn test_d8_unbalanced_close_paren() {
    let r = parse_sexpr("a b)");
    assert!(r.is_err());
}

#[test]
fn test_d8_just_close_paren() {
    let r = parse_sexpr(")");
    assert!(r.is_err());
}

#[test]
fn test_d8_multiple_unbalanced() {
    let r = parse_sexpr("(((a b)");
    assert!(r.is_err());
}

#[test]
fn test_d8_unterminated_string() {
    let r = parse_sexpr("\"hello");
    assert!(r.is_err());
}

#[test]
fn test_d8_deeply_nested_error() {
    let open: String = "(".repeat(MAX_SEXPR_DEPTH + 5);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH + 5);
    let r = parse_sexpr(&format!("{open}x{close}"));
    assert!(r.is_err());
    let msg = r.unwrap_err().to_string();
    assert!(
        msg.contains("E803") || msg.contains("depth") || msg.contains("DEPTH"),
        "Should mention depth: {msg}"
    );
}

#[test]
fn test_d8_only_whitespace() {
    let r = parse_sexpr("   ");
    assert!(r.is_err());
}

#[test]
fn test_d8_only_comment() {
    let r = parse_sexpr("; just a comment\n");
    assert!(r.is_err());
}

#[test]
fn test_d8_extra_close_parens() {
    let r = parse_sexpr("(a))");
    assert!(r.is_err());
}

#[test]
fn test_d8_null_byte_in_input() {
    let r = parse_sexpr("(\0)");
    // Should either error or produce something — not panic
    let _ = r;
}

#[test]
fn test_d8_very_long_symbol() {
    let sym: String = "a".repeat(10_000);
    let input = format!("({sym})");
    let r = parse_sexpr(&input);
    // Should not panic even with very long symbols
    let _ = r;
}

#[test]
fn test_d8_negative_is_symbol() {
    // u64-based Integer cannot hold negatives; parser treats "-42" as a symbol.
    let r = parse_sexpr("-42");
    assert!(r.is_ok(), "Negative literal should parse as symbol");
    match r.unwrap() {
        SExpr::Symbol(_) => {} // expected
        other => panic!("Expected Symbol for '-42', got {other:?}"),
    }
}

#[test]
fn test_d8_large_integer() {
    let r = parse_sexpr("9999999999999");
    assert!(r.is_ok());
}

#[test]
fn test_d8_mixed_content() {
    let r = parse_sexpr("(define (f x) (+ x 1))");
    assert!(r.is_ok());
}
