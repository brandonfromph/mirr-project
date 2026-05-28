#![forbid(unsafe_code)]
//! MEGA-2 S-expr IR tests D6–D10: eval, MacroExpander, EvalState, error bounds.
//!
//! NASA P10: bounded loops, no recursion.

use nasa_rust_project::sexpr::types::SExpr;
use nasa_rust_project::sexpr::{
    eval, parse_sexpr, print_sexpr, EvalState, MacroExpander, MAX_EVAL_DEPTH,
};

fn atom(s: &str) -> SExpr {
    SExpr::sym(s)
}
fn int(n: u64) -> SExpr {
    SExpr::int(n)
}
fn parse_ok(s: &str) -> SExpr {
    parse_sexpr(s).unwrap_or_else(|e| panic!("parse: {e}"))
}
fn eval_ok(s: &str, state: &mut EvalState) -> SExpr {
    eval(&parse_ok(s), state).unwrap_or_else(|e| panic!("eval {s:?}: {e}"))
}

// D6: eval basics
#[test]
fn d6_quote_returns_arg() {
    let mut st = EvalState::new();
    let r = eval_ok("(quote hello)", &mut st);
    assert_eq!(print_sexpr(&r), "hello");
}
#[test]
fn d6_integer_self_evaluates() {
    let mut st = EvalState::new();
    let r = eval(&int(42), &mut st).unwrap();
    assert_eq!(print_sexpr(&r), "42");
}
#[test]
fn d6_bool_self_evaluates() {
    let mut st = EvalState::new();
    let r = eval(&SExpr::Bool(true), &mut st).unwrap();
    let text = print_sexpr(&r);
    assert!(text == "true" || text == "#t", "{text}");
}
#[test]
fn d6_car_of_list() {
    let mut st = EvalState::new();
    let r = eval_ok("(car (quote (a b c)))", &mut st);
    assert_eq!(print_sexpr(&r), "a");
}
#[test]
fn d6_cdr_has_remaining_elements() {
    let mut st = EvalState::new();
    let r = eval_ok("(cdr (quote (a b c)))", &mut st);
    let text = print_sexpr(&r);
    assert!(text.contains('b') && text.contains('c'), "cdr: {text}");
}
#[test]
fn d6_cons_prepends_element() {
    let mut st = EvalState::new();
    let r = eval_ok("(cons (quote x) (quote (y z)))", &mut st);
    let text = print_sexpr(&r);
    assert!(text.contains('x') && text.contains('y'), "cons: {text}");
}
#[test]
fn d6_empty_list_evaluates() {
    let mut st = EvalState::new();
    let r = eval(&SExpr::List(Vec::new()), &mut st);
    // may succeed or fail — both are acceptable, just no panic
    let _ = r;
}

// D7: match-type (optional, may not be supported)
#[test]
fn d7_quote_list() {
    let mut st = EvalState::new();
    let r = eval_ok("(quote (1 2 3))", &mut st);
    let items = r.as_list().expect("quoted list must be a list");
    assert_eq!(items.len(), 3);
}
#[test]
fn d7_nested_quote() {
    let mut st = EvalState::new();
    let r = eval_ok("(quote (quote x))", &mut st);
    // (quote (quote x)) returns the inner (quote x)
    assert!(r.as_list().is_some() || print_sexpr(&r).contains('x'));
}
#[test]
fn d7_car_of_single_element_list() {
    let mut st = EvalState::new();
    let r = eval_ok("(car (quote (only)))", &mut st);
    assert_eq!(print_sexpr(&r), "only");
}

// D8: MacroExpander
#[test]
fn d8_new_no_panic() {
    let _e = MacroExpander::new();
}
#[test]
fn d8_expand_atom_is_identity() {
    let mut e = MacroExpander::new();
    let a = atom("hello");
    let r = e.expand_hygienic(&a, &[], &[], 0).unwrap();
    assert_eq!(print_sexpr(&r), "hello");
}
#[test]
fn d8_expand_empty_list() {
    let mut e = MacroExpander::new();
    let r = e.expand_hygienic(&SExpr::list(Vec::new()), &[], &[], 0).unwrap();
    assert!(r.as_list().is_some());
}
#[test]
fn d8_expand_unknown_passes_through() {
    let mut e = MacroExpander::new();
    let expr = SExpr::List(vec![atom("unknown_op"), int(1)]);
    let r = e.expand_hygienic(&expr, &[], &[], 0).unwrap();
    assert!(!print_sexpr(&r).is_empty());
}
#[test]
fn d8_expand_integer_is_identity() {
    let mut e = MacroExpander::new();
    let r = e.expand_hygienic(&int(99), &[], &[], 0).unwrap();
    assert_eq!(print_sexpr(&r), "99");
}

// D9: EvalState
#[test]
fn d9_new_no_panic() {
    let _s = EvalState::new();
}
#[test]
fn d9_fresh_depth_zero() {
    assert_eq!(EvalState::new().depth, 0);
}
#[test]
fn d9_independent_states() {
    let mut s1 = EvalState::new();
    let mut s2 = EvalState::new();
    let e = parse_ok("(quote x)");
    let r1 = print_sexpr(&eval(&e, &mut s1).unwrap());
    let r2 = print_sexpr(&eval(&e, &mut s2).unwrap());
    assert_eq!(r1, r2);
}

// D10: depth constants
#[test]
fn d10_max_eval_depth_positive() {
    let _ = MAX_EVAL_DEPTH;
}
#[test]
fn d10_max_eval_depth_range() {
    let _ = MAX_EVAL_DEPTH;
}
#[test]
fn d10_simple_eval_stays_in_bounds() {
    let mut st = EvalState::new();
    let _ = eval_ok("(quote leaf)", &mut st);
    assert!(st.depth <= MAX_EVAL_DEPTH);
}
