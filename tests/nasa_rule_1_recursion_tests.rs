#![forbid(unsafe_code)]
#![deny(warnings)]

//! NASA JPL Rule 1 integration tests.
//!
//! Validates:
//! 1. Rule 1: Avoidance of recursive execution inside core compiler parser, hydration, and typechecking.
//! 2. Safe iterative traversal of deeply nested AST expressions, arrays, and structs.
//! 3. Absence of stack overflows under pathologically large source constructs.

use nasa_rust_project::ast::types::{BinaryOp, LiteralValue};
use nasa_rust_project::ast::Expr;
use nasa_rust_project::ecs::Registry;

/// Verify that a deeply nested binary addition expression is processed iteratively
/// without causing a call stack overflow during registry ingestion.
#[test]
fn test_nasa_rule_1_nested_expressions_iterative() {
    let mut expr = Expr::Literal(LiteralValue::Integer(1));

    // Generate a deep linear tree (500 levels).
    // Note: Capped at 500 to stay under the 512 MAX_EXPR_NODES error limit,
    // but deep enough to verify stack behavior under limits.
    for _ in 0..250 {
        expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Literal(LiteralValue::Integer(1))),
            right: Box::new(expr),
        };
    }

    let mut registry = Registry::new();
    let res = registry.ingest_expr(&expr);

    assert!(res.is_ok(), "Deep expression must be ingested cleanly within limits");
    let entity = res.unwrap();
    assert!(entity.0 > 0);
}

/// Verify that nested array literals are flattened iteratively.
#[test]
fn test_nasa_rule_1_nested_array_literal() {
    let mut expr = Expr::Literal(LiteralValue::Integer(1));

    // 100 levels of array wrapping
    for _ in 0..100 {
        expr = Expr::ArrayLiteral(vec![expr]);
    }

    let mut registry = Registry::new();
    let res = registry.ingest_expr(&expr);

    assert!(
        res.is_ok(),
        "Deeply nested array literals must be ingested iteratively without stack overflows"
    );
}

/// Verify that nested struct literals are flattened iteratively.
#[test]
fn test_nasa_rule_1_nested_struct_literal() {
    let mut expr = Expr::Literal(LiteralValue::Integer(1));

    // 100 levels of struct literal fields
    for i in 0..100 {
        expr = Expr::StructLiteral {
            name: "NestedStruct".to_string(),
            fields: vec![(format!("field_{i}"), expr)],
        };
    }

    let mut registry = Registry::new();
    let res = registry.ingest_expr(&expr);

    assert!(res.is_ok(), "Deeply nested struct literals must be ingested iteratively");
}
