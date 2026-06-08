//! S-expression roundtrip integration tests.

#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::sexpr::parser::parse_sexpr;
use mirrc::sexpr::printer::print_sexpr;
use mirrc::sexpr::types::SExpr;

fn roundtrip(input: &str) {
    let parsed = parse_sexpr(input).expect("parse should succeed");
    let printed = print_sexpr(&parsed);
    let reparsed = parse_sexpr(&printed).expect("reparse should succeed");
    assert_eq!(parsed, reparsed, "roundtrip failed for: {}", input);
}

#[test]
fn sexpr_parse_empty_list() {
    let result = parse_sexpr("()");
    assert!(result.is_ok());
}

#[test]
fn sexpr_parse_atom() {
    let result = parse_sexpr("hello");
    assert!(result.is_ok());
    match result.unwrap() {
        SExpr::Symbol(s) => assert_eq!(s, "hello"),
        _ => panic!("expected symbol"),
    }
}

#[test]
fn sexpr_parse_number() {
    let result = parse_sexpr("42");
    assert!(result.is_ok());
}

#[test]
fn sexpr_parse_list() {
    let result = parse_sexpr("(a b c)");
    assert!(result.is_ok());
    match result.unwrap() {
        SExpr::List(items) => assert_eq!(items.len(), 3),
        _ => panic!("expected list"),
    }
}

#[test]
fn sexpr_parse_nested() {
    let result = parse_sexpr("(a (b c) d)");
    assert!(result.is_ok());
}

#[test]
fn sexpr_roundtrip_atom() {
    roundtrip("hello");
}

#[test]
fn sexpr_roundtrip_list() {
    roundtrip("(a b c)");
}

#[test]
fn sexpr_roundtrip_nested() {
    roundtrip("(a (b (c d)) e)");
}

#[test]
fn sexpr_roundtrip_empty() {
    roundtrip("()");
}

#[test]
fn sexpr_roundtrip_numbers() {
    roundtrip("(1 2 3)");
}

#[test]
fn sexpr_roundtrip_mixed() {
    roundtrip("(module (signal x) (guard g))");
}

#[test]
fn sexpr_print_pretty() {
    let parsed = parse_sexpr("(a (b c) d)").expect("parse");
    let pretty = print_sexpr(&parsed);
    assert!(!pretty.is_empty());
}
