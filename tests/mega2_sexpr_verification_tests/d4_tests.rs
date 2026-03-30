use super::*;

// ===========================================================================
// D4: quasiquote_unquote (10 tests)
// ===========================================================================

#[test]
fn test_d4_quote_literal() {
    let r = eval_ok("'42");
    assert_eq!(r, SExpr::Integer(42));
}

#[test]
fn test_d4_quote_symbol() {
    let r = eval_ok("'hello");
    assert_eq!(r, SExpr::sym("hello"));
}

#[test]
fn test_d4_quote_list() {
    let r = eval_ok("'(a b c)");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]));
}

#[test]
fn test_d4_quasiquote_no_unquote() {
    let r = eval_ok("`(a b c)");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]));
}

#[test]
fn test_d4_quasiquote_with_unquote() {
    // Quasiquote with unquote evaluating a sub-expression (car).
    let r = eval_ok("`(a ,(car '(42 99)) c)");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::Integer(42), SExpr::sym("c")]));
}

#[test]
fn test_d4_quasiquote_nested() {
    // Quasiquote with nested list containing unquote.
    let r = eval_ok("`((,(car '(1 2))) 2)");
    assert_eq!(r, SExpr::list(vec![SExpr::list(vec![SExpr::Integer(1)]), SExpr::Integer(2),]));
}

#[test]
fn test_d4_quasiquote_arithmetic() {
    // Quasiquote with unquote evaluating a computed expression (if).
    let r = eval_ok("`(result ,(if true 42 0))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("result"), SExpr::Integer(42)]));
}

#[test]
fn test_d4_quote_preserves_structure() {
    let r = eval_ok("'(1 (2 3) 4)");
    assert_eq!(
        r,
        SExpr::list(vec![
            SExpr::Integer(1),
            SExpr::list(vec![SExpr::Integer(2), SExpr::Integer(3)]),
            SExpr::Integer(4),
        ])
    );
}

#[test]
fn test_d4_quasiquote_bool_unquote() {
    // Quasiquote with unquote of a boolean expression (eq?).
    let r = eval_ok("`(status ,(eq? 1 1))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("status"), SExpr::Bool(true)]));
}

#[test]
fn test_d4_quasiquote_string_unquote() {
    // Quasiquote with unquote evaluating to a string (car of quoted list).
    let r = eval_ok(r#"`(label ,(car '("sensor" "motor")))"#);
    assert_eq!(r, SExpr::list(vec![SExpr::sym("label"), SExpr::Str("sensor".to_string())]));
}
