#![feature(box_patterns)]
#![forbid(unsafe_code)]

use mirrc::ast::Expr;
use mirrc::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use mirrc::parser::parse_expression;

fn ok_expr(s: &str) -> Expr {
    parse_expression(s).unwrap_or_else(|e| panic!("Failed to parse '{}': {:?}", s, e))
}

fn err_expr(s: &str) -> String {
    parse_expression(s).unwrap_err().to_string()
}

#[test]
fn test_01_pemdas_add_mul() -> Result<(), String> {
    let e = ok_expr("a + b * c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Add, right: box Expr::Binary { op: BinaryOp::Mul, .. }, .. }));
    Ok(())
}

#[test]
fn test_02_pemdas_mul_add() -> Result<(), String> {
    let e = ok_expr("a * b + c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Add, left: box Expr::Binary { op: BinaryOp::Mul, .. }, .. }));
    Ok(())
}

#[test]
fn test_03_pemdas_parens_override() -> Result<(), String> {
    let e = ok_expr("(a + b) * c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Mul, left: box Expr::Binary { op: BinaryOp::Add, .. }, .. }));
    Ok(())
}

#[test]
fn test_04_pemdas_sub_div() -> Result<(), String> {
    let e = ok_expr("a - b * c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Sub, right: box Expr::Binary { op: BinaryOp::Mul, .. }, .. }));
    Ok(())
}

#[test]
fn test_05_boolean_and_or() -> Result<(), String> {
    let e = ok_expr("a || b && c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Or, right: box Expr::Binary { op: BinaryOp::And, .. }, .. }));
    Ok(())
}

#[test]
fn test_06_boolean_and_or_parens() -> Result<(), String> {
    let e = ok_expr("(a || b) && c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::And, left: box Expr::Binary { op: BinaryOp::Or, .. }, .. }));
    Ok(())
}

#[test]
fn test_07_bitwise_or_and() -> Result<(), String> {
    let e = ok_expr("a | b & c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::BitwiseOr, right: box Expr::Binary { op: BinaryOp::BitwiseAnd, .. }, .. }));
    Ok(())
}

#[test]
fn test_08_bitwise_xor_and() -> Result<(), String> {
    let e = ok_expr("a ^ b & c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Xor, right: box Expr::Binary { op: BinaryOp::BitwiseAnd, .. }, .. }));
    Ok(())
}

#[test]
fn test_09_bitwise_shift_add() -> Result<(), String> {
    let e = ok_expr("a << b + c");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Shl, right: box Expr::Binary { op: BinaryOp::Add, .. }, .. }));
    Ok(())
}

#[test]
fn test_10_unary_not_and() -> Result<(), String> {
    let e = ok_expr("!a && b");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::And, left: box Expr::Unary { op: UnaryOp::Not, .. }, .. }));
    Ok(())
}

#[test]
fn test_11_unary_neg_add() -> Result<(), String> {
    let e = ok_expr("-a + b");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Add, left: box Expr::Unary { op: UnaryOp::Negate, .. }, .. }));
    Ok(())
}

#[test]
fn test_12_comparisons_and() -> Result<(), String> {
    let e = ok_expr("a < b && c > d");
    assert!(matches!(e, Expr::Binary { 
        op: BinaryOp::And, 
        left: box Expr::Binary { op: BinaryOp::Lt, .. },
        right: box Expr::Binary { op: BinaryOp::Gt, .. }
    }));
    Ok(())
}

#[test]
fn test_13_equality_or() -> Result<(), String> {
    let e = ok_expr("a == b || c != d");
    assert!(matches!(e, Expr::Binary { 
        op: BinaryOp::Or, 
        left: box Expr::Binary { op: BinaryOp::Eq, .. },
        right: box Expr::Binary { op: BinaryOp::Ne, .. }
    }));
    Ok(())
}

#[test]
fn test_14_nested_parens() -> Result<(), String> {
    let e = ok_expr("((a + b) * (c - d))");
    assert!(matches!(e, Expr::Binary { 
        op: BinaryOp::Mul, 
        left: box Expr::Binary { op: BinaryOp::Add, .. },
        right: box Expr::Binary { op: BinaryOp::Sub, .. }
    }));
    Ok(())
}

#[test]
fn test_15_left_associativity_add() -> Result<(), String> {
    let e = ok_expr("a + b + c");
    assert!(matches!(e, Expr::Binary { 
        op: BinaryOp::Add, 
        left: box Expr::Binary { op: BinaryOp::Add, .. },
        right: box Expr::Signal(_)
    }));
    Ok(())
}

#[test]
fn test_16_left_associativity_mul() -> Result<(), String> {
    let e = ok_expr("a * b * c");
    assert!(matches!(e, Expr::Binary { 
        op: BinaryOp::Mul, 
        left: box Expr::Binary { op: BinaryOp::Mul, .. },
        right: box Expr::Signal(_)
    }));
    Ok(())
}

#[test]
fn test_17_left_associativity_shift() -> Result<(), String> {
    let e = ok_expr("a << b << c");
    assert!(matches!(e, Expr::Binary { 
        op: BinaryOp::Shl, 
        left: box Expr::Binary { op: BinaryOp::Shl, .. },
        right: box Expr::Signal(_)
    }));
    Ok(())
}

#[test]
fn test_18_mixed_bitwise_logical() -> Result<(), String> {
    let e = ok_expr("a & b && c | d");
    assert!(matches!(e, Expr::Binary { 
        op: BinaryOp::And, 
        left: box Expr::Binary { op: BinaryOp::BitwiseAnd, .. },
        right: box Expr::Binary { op: BinaryOp::BitwiseOr, .. }
    }));
    Ok(())
}

#[test]
fn test_19_comparison_chain_invalid_associativity() -> Result<(), String> {
    let e = ok_expr("a < b == c > d");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Eq, .. }));
    Ok(())
}

#[test]
fn test_20_ltl_implication_error() -> Result<(), String> {
    let err = err_expr("a -> b");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_21_prefix_precedence() -> Result<(), String> {
    let e = ok_expr("!!a");
    assert!(matches!(e, Expr::Unary { op: UnaryOp::Not, operand: box Expr::Unary { op: UnaryOp::Not, .. } }));
    Ok(())
}

#[test]
fn test_22_prefix_and_postfix() -> Result<(), String> {
    let e = ok_expr("!arr[0]");
    assert!(matches!(e, Expr::Unary { op: UnaryOp::Not, operand: box Expr::ArrayIndex { .. } }));
    Ok(())
}

#[test]
fn test_23_complex_precedence_mix() -> Result<(), String> {
    let e = ok_expr("a + b * c << d & e == f && g");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::And, .. }));
    Ok(())
}

#[test]
fn test_24_deep_nested_expression() -> Result<(), String> {
    let e = ok_expr("a + (b - (c * (d << (e & (f | g)))))");
    assert!(matches!(e, Expr::Binary { op: BinaryOp::Add, .. }));
    Ok(())
}

#[test]
fn test_25_unmatched_parens_error() -> Result<(), String> {
    let err = err_expr("(a + b");
    assert!(err.contains("171") || !err.is_empty());
    Ok(())
}
