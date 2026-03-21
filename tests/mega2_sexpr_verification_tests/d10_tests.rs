use super::*;

// ===========================================================================
// D10: eval_step_budget (5 tests)
// ===========================================================================

#[test]
fn test_d10_eval_within_budget() {
    let mut st = EvalState::new();
    let expr = parse_sexpr("(car '(1 2))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_ok(), "Simple eval should stay within budget");
}

#[test]
fn test_d10_eval_nested_but_within_budget() {
    let mut st = EvalState::new();
    let expr = parse_sexpr("(car (cdr '(1 2 3)))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), SExpr::Integer(2));
}

#[test]
fn test_d10_eval_let_within_budget() {
    let mut st = EvalState::new();
    let expr = parse_sexpr("(cons 1 '(2 3))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_ok());
    assert_eq!(
        r.unwrap(),
        SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2), SExpr::Integer(3)])
    );
}

#[test]
fn test_d10_eval_custom_step_limit() {
    let mut st = EvalState::with_steps(2);
    // (car (cdr '(1 2 3))) needs 3 eval steps, exceeds limit of 2.
    let expr = parse_sexpr("(car (cdr '(1 2 3)))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_err(), "Should exceed custom step limit of 2");
    let msg = r.unwrap_err().to_string();
    assert!(msg.contains("E812"), "Should be step budget error: {msg}");
}

#[test]
fn test_d10_eval_depth_limit() {
    // Build deeply nested if-expressions to exhaust eval depth.
    // Each nested (if COND 1 0) in condition position pushes one IfCond frame.
    let mut input = String::from("true");
    let mut i = 0;
    while i < MAX_EVAL_DEPTH + 5 {
        input = format!("(if {input} 1 0)");
        i += 1;
    }
    let expr = parse_sexpr(&input).unwrap();
    let mut st = EvalState::new();
    let r = eval(&expr, &mut st);
    assert!(r.is_err(), "Deeply nested if should exceed depth limit");
    let msg = r.unwrap_err().to_string();
    assert!(
        msg.contains("E811") || msg.contains("depth") || msg.contains("E812"),
        "Should mention depth/steps limit: {msg}"
    );
}
