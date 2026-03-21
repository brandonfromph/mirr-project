use super::*;

// ===========================================================================
// D3: eval_match_type (15 tests)
// ===========================================================================

#[test]
fn test_d3_match_type_integer() {
    // match-type matches Symbol values by exact name.
    // 'integer → Symbol("integer"), pattern integer → exact match.
    let r = eval_ok(r#"(match-type 'integer (integer "yes") (symbol "no"))"#);
    assert_eq!(r, SExpr::Str("yes".to_string()));
}

#[test]
fn test_d3_match_type_symbol() {
    // Second clause matches when first clause doesn't.
    let r = eval_ok(r#"(match-type 'hello (world "no") (hello "yes"))"#);
    assert_eq!(r, SExpr::Str("yes".to_string()));
}

#[test]
fn test_d3_match_type_bool_true() {
    // Symbol "bool" matches pattern "bool" in first clause.
    let r = eval_ok(r#"(match-type 'bool (bool "matched") (other "no"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_bool_false() {
    // First clause doesn't match, falls through to second.
    let r = eval_ok(r#"(match-type 'other (bool "no") (other "matched"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_string() {
    // Symbol matching with a type-like name.
    let r = eval_ok(r#"(match-type 'string (string "matched") (integer "no"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_list() {
    // List pattern: head must match, rest are bound as variables.
    // Value '(unsigned 16) matches pattern (unsigned w), binding w=16.
    let r = eval_ok("(match-type '(unsigned 16) ((unsigned w) w))");
    assert_eq!(r, SExpr::Integer(16));
}

#[test]
fn test_d3_match_type_first_match_wins() {
    // Two clauses match the same symbol — first one wins.
    let r = eval_ok(r#"(match-type 'x (x "first") (x "second"))"#);
    assert_eq!(r, SExpr::Str("first".to_string()));
}

#[test]
fn test_d3_match_type_quote() {
    // (quote hello) evaluates to Symbol("hello"), matches pattern hello.
    let r = eval_ok(r#"(match-type (quote hello) (hello "yes") (world "no"))"#);
    assert_eq!(r, SExpr::Str("yes".to_string()));
}

#[test]
fn test_d3_match_type_nested_list() {
    // List pattern matching on a value with nested structure.
    // Value '(pair (a) b) matches pattern (pair x y), binding x=List(a), y=Symbol(b).
    let r = eval_ok("(match-type '(pair (a) b) ((pair x y) x))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a")]));
}

#[test]
fn test_d3_match_type_returns_constant() {
    // Body returns a constant value, not a bound variable.
    let r = eval_ok(r#"(match-type 'x (x 42))"#);
    assert_eq!(r, SExpr::Integer(42));
}

#[test]
fn test_d3_match_type_binds_variable() {
    // List pattern binds a variable; body evaluates it.
    let r = eval_ok("(match-type '(width 8) ((width n) n))");
    assert_eq!(r, SExpr::Integer(8));
}

#[test]
fn test_d3_match_type_no_match_errors() {
    let sexpr = parse_sexpr("(match-type 42 (symbol? x x))").unwrap();
    let mut st = EvalState::new();
    let err = eval(&sexpr, &mut st);
    assert!(err.is_err(), "match-type with no matching clause should error");
}

#[test]
fn test_d3_match_type_zero() {
    // Symbol "zero" matches pattern "zero".
    let r = eval_ok(r#"(match-type 'zero (zero "matched"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_negative_integer() {
    // Integer values never match Symbol patterns — should error.
    let expr = SExpr::list(vec![
        SExpr::sym("match-type"),
        SExpr::Integer(5),
        SExpr::list(vec![SExpr::sym("x"), SExpr::Integer(1)]),
    ]);
    let mut st = EvalState::new();
    let r = eval(&expr, &mut st);
    assert!(r.is_err(), "Integer value should not match Symbol pattern");
}

#[test]
fn test_d3_match_type_empty_list() {
    // Empty list matches empty list pattern.
    let r = eval_ok("(match-type '() (() 1))");
    assert_eq!(r, SExpr::Integer(1));
}

