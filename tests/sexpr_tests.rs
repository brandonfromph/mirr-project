#![forbid(unsafe_code)]
//! Tests for S-expression parser, printer, and type helpers.

use mirrc::sexpr::parser::parse_sexpr;
use mirrc::sexpr::printer::print_sexpr;
use mirrc::sexpr::types::SExpr;

// =========================================================================
// Parser Tests
// =========================================================================

#[test]
fn parse_atom_symbol() {
    let result = parse_sexpr("hello").unwrap();
    assert_eq!(result, SExpr::sym("hello"));
}

#[test]
fn parse_atom_integer() {
    let result = parse_sexpr("42").unwrap();
    assert_eq!(result, SExpr::Integer(42));
}

#[test]
fn parse_atom_bool_true() {
    let result = parse_sexpr("true").unwrap();
    assert_eq!(result, SExpr::Bool(true));
}

#[test]
fn parse_atom_bool_false() {
    let result = parse_sexpr("false").unwrap();
    assert_eq!(result, SExpr::Bool(false));
}

#[test]
fn parse_atom_string() {
    let result = parse_sexpr("\"hello\"").unwrap();
    assert_eq!(result, SExpr::Str("hello".to_string()));
}

#[test]
fn parse_empty_list() {
    let result = parse_sexpr("()").unwrap();
    assert_eq!(result, SExpr::list(vec![]));
}

#[test]
fn parse_simple_list() {
    let result = parse_sexpr("(a b c)").unwrap();
    assert_eq!(result, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]));
}

#[test]
fn parse_nested_list() {
    let result = parse_sexpr("(a (b c))").unwrap();
    assert_eq!(
        result,
        SExpr::list(vec![SExpr::sym("a"), SExpr::list(vec![SExpr::sym("b"), SExpr::sym("c")])])
    );
}

#[test]
fn parse_quote() {
    let result = parse_sexpr("'(a b)").unwrap();
    assert_eq!(result, SExpr::Quote(Box::new(SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b")]))));
}

#[test]
fn parse_depth_limit() {
    // 65 nested parens should exceed MAX_SEXPR_DEPTH (64).
    let input = "(".repeat(65) + &")".repeat(65);
    let result = parse_sexpr(&input);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("E803") || msg.contains("DEPTH"), "got: {msg}");
}

#[test]
fn parse_unbalanced() {
    let result = parse_sexpr("(a b");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("E802") || msg.contains("paren") || msg.contains("Unbalanced"),
        "got: {msg}"
    );
}

#[test]
fn parse_empty_input() {
    let result = parse_sexpr("");
    assert!(result.is_err());
}

#[test]
fn parse_comment_skipping() {
    let result = parse_sexpr("; comment\n42").unwrap();
    assert_eq!(result, SExpr::Integer(42));
}

#[test]
fn parse_mixed_atoms() {
    let result = parse_sexpr("(signal \"temp\" input 16)").unwrap();
    assert_eq!(
        result,
        SExpr::list(vec![
            SExpr::sym("signal"),
            SExpr::Str("temp".to_string()),
            SExpr::sym("input"),
            SExpr::Integer(16),
        ])
    );
}

// =========================================================================
// Printer Tests
// =========================================================================

#[test]
fn print_atom() {
    assert_eq!(print_sexpr(&SExpr::sym("hello")), "hello");
    assert_eq!(print_sexpr(&SExpr::Integer(42)), "42");
    assert_eq!(print_sexpr(&SExpr::Bool(true)), "true");
    assert_eq!(print_sexpr(&SExpr::Str("hi".to_string())), "\"hi\"");
}

#[test]
fn print_short_list() {
    let expr = SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]);
    assert_eq!(print_sexpr(&expr), "(a b c)");
}

#[test]
fn print_empty_list() {
    assert_eq!(print_sexpr(&SExpr::list(vec![])), "()");
}

#[test]
fn print_nested() {
    let expr =
        SExpr::list(vec![SExpr::sym("a"), SExpr::list(vec![SExpr::sym("b"), SExpr::sym("c")])]);
    let output = print_sexpr(&expr);
    assert!(output.contains("a") && output.contains("b") && output.contains("c"));
}

#[test]
fn print_quote() {
    let expr = SExpr::Quote(Box::new(SExpr::list(vec![SExpr::sym("a")])));
    let output = print_sexpr(&expr);
    assert!(output.contains("'"));
}

#[test]
fn print_parse_roundtrip() {
    let input = "(program (patterns) (module \"test\" (signals) (guards) (reflexes)))";
    let parsed = parse_sexpr(input).unwrap();
    let printed = print_sexpr(&parsed);
    let reparsed = parse_sexpr(&printed).unwrap();
    assert_eq!(parsed, reparsed);
}

// =========================================================================
// Type Helper Tests
// =========================================================================

#[test]
fn type_predicates() {
    assert!(SExpr::sym("x").is_symbol());
    assert!(SExpr::Integer(1).is_integer());
    assert!(SExpr::Bool(true).is_bool());
    assert!(SExpr::Str("s".to_string()).is_str());
    assert!(SExpr::list(vec![]).is_list());
    assert!(!SExpr::sym("x").is_list());
}

#[test]
fn accessor_methods() {
    assert_eq!(SExpr::sym("x").as_symbol(), Some("x"));
    assert_eq!(SExpr::Integer(42).as_integer(), Some(42));
    assert_eq!(SExpr::Bool(true).as_bool(), Some(true));
    assert_eq!(SExpr::Str("hi".to_string()).as_str_val(), Some("hi"));
    assert!(SExpr::list(vec![]).as_list().is_some());
    assert_eq!(SExpr::sym("x").as_integer(), None);
}

#[test]
fn constructor_helpers() {
    assert_eq!(SExpr::sym("x"), SExpr::Symbol("x".to_string()));
    assert_eq!(SExpr::int(42), SExpr::Integer(42));
    assert_eq!(SExpr::bool_val(true), SExpr::Bool(true));
    assert_eq!(SExpr::str_val("hi"), SExpr::Str("hi".to_string()));
}

#[test]
fn is_atom() {
    assert!(SExpr::sym("x").is_atom());
    assert!(SExpr::Integer(1).is_atom());
    assert!(SExpr::Bool(true).is_atom());
    assert!(SExpr::Str("s".to_string()).is_atom());
    assert!(!SExpr::list(vec![]).is_atom());
}

#[test]
fn head_symbol() {
    let expr = SExpr::list(vec![SExpr::sym("signal"), SExpr::str_val("x")]);
    assert_eq!(expr.head_symbol(), Some("signal"));
    assert_eq!(SExpr::list(vec![]).head_symbol(), None);
    assert_eq!(SExpr::sym("x").head_symbol(), None);
}

#[test]
fn node_count() {
    let expr =
        SExpr::list(vec![SExpr::sym("a"), SExpr::list(vec![SExpr::sym("b"), SExpr::sym("c")])]);
    assert_eq!(expr.node_count(), 5); // list + a + list + b + c
}

#[test]
fn equality() {
    let a = SExpr::list(vec![SExpr::sym("x"), SExpr::Integer(1)]);
    let b = SExpr::list(vec![SExpr::sym("x"), SExpr::Integer(1)]);
    assert_eq!(a, b);
}

#[test]
fn clone_independence() {
    let a = SExpr::list(vec![SExpr::sym("x")]);
    let b = a.clone();
    assert_eq!(a, b);
    // Both are independent—modifying one won't affect the other.
}

#[test]
fn display_trait() {
    let expr = SExpr::list(vec![SExpr::sym("a"), SExpr::Integer(42)]);
    let s = expr.to_string();
    assert_eq!(s, "(a 42)");
}
