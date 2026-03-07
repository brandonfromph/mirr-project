// ---------------------------------------------------------------------------
// Expression parser unit tests
// ---------------------------------------------------------------------------

use nasa_rust_project::ast::{BinaryOp, Expr, LiteralValue, UnaryOp};
use nasa_rust_project::parser::parse_expression;

// -- Helpers --

fn bin(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    Expr::Binary { op, left: Box::new(left), right: Box::new(right) }
}

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn int(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}

fn bool_lit(v: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(v))
}

fn not(e: Expr) -> Expr {
    Expr::Unary { op: UnaryOp::Not, operand: Box::new(e) }
}

// -- Tests --

#[test]
fn expr_simple_signal() {
    let e = parse_expression("foo").expect("ok");
    assert_eq!(e, sig("foo"));
}

#[test]
fn expr_bool_literals() {
    assert_eq!(parse_expression("true").expect("ok"), bool_lit(true));
    assert_eq!(parse_expression("false").expect("ok"), bool_lit(false));
}

#[test]
fn expr_integer_literal() {
    assert_eq!(parse_expression("42").expect("ok"), int(42));
}

#[test]
fn expr_comparison() {
    let e = parse_expression("x < 50").expect("ok");
    assert_eq!(e, bin(BinaryOp::Lt, sig("x"), int(50)));
}

#[test]
fn expr_logical_and() {
    let e = parse_expression("a && b").expect("ok");
    assert_eq!(e, bin(BinaryOp::And, sig("a"), sig("b")));
}

#[test]
fn expr_logical_or() {
    let e = parse_expression("a || b").expect("ok");
    assert_eq!(e, bin(BinaryOp::Or, sig("a"), sig("b")));
}

#[test]
fn expr_not() {
    let e = parse_expression("!x").expect("ok");
    assert_eq!(e, not(sig("x")));
}

#[test]
fn expr_complex_and_not() {
    let e = parse_expression("eeg_spike && !artifact_noise").expect("ok");
    assert_eq!(e, bin(BinaryOp::And, sig("eeg_spike"), not(sig("artifact_noise"))));
}

#[test]
fn expr_precedence_and_or() {
    // a || b && c  =>  a || (b && c)
    let e = parse_expression("a || b && c").expect("ok");
    assert_eq!(e, bin(BinaryOp::Or, sig("a"), bin(BinaryOp::And, sig("b"), sig("c"))));
}

#[test]
fn expr_precedence_comparison_and_logical() {
    // x < 5 && y > 10  =>  (x < 5) && (y > 10)
    let e = parse_expression("x < 5 && y > 10").expect("ok");
    assert_eq!(
        e,
        bin(
            BinaryOp::And,
            bin(BinaryOp::Lt, sig("x"), int(5)),
            bin(BinaryOp::Gt, sig("y"), int(10))
        )
    );
}

#[test]
fn expr_parentheses() {
    // (a || b) && c
    let e = parse_expression("(a || b) && c").expect("ok");
    assert_eq!(e, bin(BinaryOp::And, bin(BinaryOp::Or, sig("a"), sig("b")), sig("c")));
}

#[test]
fn expr_arithmetic() {
    // a + b * c  =>  a + (b * c)
    let e = parse_expression("a + b * c").expect("ok");
    assert_eq!(e, bin(BinaryOp::Add, sig("a"), bin(BinaryOp::Mul, sig("b"), sig("c"))));
}

#[test]
fn expr_shifts() {
    let e = parse_expression("x << 3").expect("ok");
    assert_eq!(e, bin(BinaryOp::Shl, sig("x"), int(3)));
}

#[test]
fn expr_xor() {
    let e = parse_expression("a ^ b").expect("ok");
    assert_eq!(e, bin(BinaryOp::Xor, sig("a"), sig("b")));
}

#[test]
fn expr_all_comparison_ops() {
    assert_eq!(parse_expression("a <= b").expect("ok"), bin(BinaryOp::Le, sig("a"), sig("b")));
    assert_eq!(parse_expression("a >= b").expect("ok"), bin(BinaryOp::Ge, sig("a"), sig("b")));
    assert_eq!(parse_expression("a == b").expect("ok"), bin(BinaryOp::Eq, sig("a"), sig("b")));
    assert_eq!(parse_expression("a != b").expect("ok"), bin(BinaryOp::Ne, sig("a"), sig("b")));
}

#[test]
fn expr_empty_error() {
    let err = parse_expression("").expect_err("should fail");
    assert!(err.to_string().contains("Empty expression"));
}

#[test]
fn expr_unexpected_token_error() {
    let err = parse_expression("+ x").expect_err("should fail");
    assert!(err.to_string().contains("Unexpected token"));
}
