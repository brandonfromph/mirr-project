use super::*;

// ===========================================================================
// D2: eval_car_cdr_cons (20 tests)
// ===========================================================================

fn eval_ok(input: &str) -> SExpr {
    let sexpr = parse_sexpr(input).unwrap_or_else(|e| panic!("parse failed: {e}"));
    let mut st = EvalState::new();
    eval(&sexpr, &mut st).unwrap_or_else(|e| panic!("eval failed for `{input}`: {e}"))
}

#[test]
fn test_d2_car_simple_list() {
    let r = eval_ok("(car '(1 2 3))");
    assert_eq!(r, SExpr::Integer(1));
}

#[test]
fn test_d2_car_singleton() {
    let r = eval_ok("(car '(42))");
    assert_eq!(r, SExpr::Integer(42));
}

#[test]
fn test_d2_car_nested() {
    let r = eval_ok("(car '((a b) c))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b")]));
}

#[test]
fn test_d2_car_bool_list() {
    let r = eval_ok("(car '(true false))");
    assert_eq!(r, SExpr::Bool(true));
}

#[test]
fn test_d2_cdr_simple_list() {
    let r = eval_ok("(cdr '(1 2 3))");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(2), SExpr::Integer(3)]));
}

#[test]
fn test_d2_cdr_singleton() {
    let r = eval_ok("(cdr '(42))");
    assert_eq!(r, SExpr::list(vec![]));
}

#[test]
fn test_d2_cdr_two_elements() {
    let r = eval_ok("(cdr '(a b))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("b")]));
}

#[test]
fn test_d2_cdr_nested() {
    let r = eval_ok("(cdr '((a) (b) (c)))");
    assert_eq!(
        r,
        SExpr::list(vec![SExpr::list(vec![SExpr::sym("b")]), SExpr::list(vec![SExpr::sym("c")]),])
    );
}

#[test]
fn test_d2_cons_prepend() {
    let r = eval_ok("(cons 0 '(1 2))");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(0), SExpr::Integer(1), SExpr::Integer(2)]));
}

#[test]
fn test_d2_cons_onto_empty() {
    let r = eval_ok("(cons 1 '())");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(1)]));
}

#[test]
fn test_d2_cons_symbol() {
    let r = eval_ok("(cons 'a '(b c))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]));
}

#[test]
fn test_d2_cons_nested_list() {
    let r = eval_ok("(cons '(1 2) '(3))");
    assert_eq!(
        r,
        SExpr::list(vec![
            SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2)]),
            SExpr::Integer(3),
        ])
    );
}

#[test]
fn test_d2_car_of_cons() {
    let r = eval_ok("(car (cons 10 '(20 30)))");
    assert_eq!(r, SExpr::Integer(10));
}

#[test]
fn test_d2_cdr_of_cons() {
    let r = eval_ok("(cdr (cons 10 '(20 30)))");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(20), SExpr::Integer(30)]));
}

#[test]
fn test_d2_car_empty_list_errors() {
    let sexpr = parse_sexpr("(car '())").unwrap();
    let mut st = EvalState::new();
    let err = eval(&sexpr, &mut st);
    assert!(err.is_err(), "car of empty list should error");
}

#[test]
fn test_d2_cdr_empty_list_errors() {
    let sexpr = parse_sexpr("(cdr '())").unwrap();
    let mut st = EvalState::new();
    let err = eval(&sexpr, &mut st);
    assert!(err.is_err(), "cdr of empty list should error");
}

#[test]
fn test_d2_list_form() {
    let r = eval_ok("(list 1 2 3)");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2), SExpr::Integer(3)]));
}

#[test]
fn test_d2_list_empty() {
    let r = eval_ok("(list)");
    assert_eq!(r, SExpr::list(vec![]));
}

#[test]
fn test_d2_car_of_list() {
    let r = eval_ok("(car (list 5 6 7))");
    assert_eq!(r, SExpr::Integer(5));
}

#[test]
fn test_d2_nested_car_cdr() {
    let r = eval_ok("(car (cdr '(1 2 3)))");
    assert_eq!(r, SExpr::Integer(2));
}

