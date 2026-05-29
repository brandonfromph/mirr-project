//! Phase 2: Generative Property Testing for MIRR AST and Type System.
//!
//! This suite uses `proptest` to generate tens of thousands of randomized
//! expression trees and verifies the internal consistency of the compiler.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use nasa_rust_project::ast::Expr;
use nasa_rust_project::ecs::Registry;
use nasa_rust_project::parser::expr_parser::parse_expression;
use proptest::prelude::*;

// --- Strategies for Generating AST Nodes ---

fn arb_binary_op() -> impl Strategy<Value = BinaryOp> {
    prop_oneof![
        Just(BinaryOp::And),
        Just(BinaryOp::Or),
        Just(BinaryOp::Add),
        Just(BinaryOp::Sub),
        Just(BinaryOp::Mul),
        Just(BinaryOp::Lt),
        Just(BinaryOp::Gt),
        Just(BinaryOp::Eq),
        Just(BinaryOp::BitwiseAnd),
        Just(BinaryOp::BitwiseOr),
        Just(BinaryOp::Xor),
    ]
}

fn arb_literal() -> impl Strategy<Value = LiteralValue> {
    prop_oneof![
        any::<bool>().prop_map(LiteralValue::Bool),
        any::<u64>().prop_map(LiteralValue::Integer),
    ]
}

fn arb_expr() -> impl Strategy<Value = Expr> {
    let leaf = prop_oneof![
        arb_literal().prop_map(Expr::Literal),
        prop::collection::vec("a..z", 1..5).prop_map(|v| Expr::Signal(v.join(""))),
    ];

    leaf.prop_recursive(
        8,   // 8 levels of nesting
        256, // 256 max nodes
        10,  // up to 10 nodes per branch
        |inner| {
            prop_oneof![
                // Unary Ops
                inner.clone().prop_map(|e| Expr::Unary { op: UnaryOp::Not, operand: Box::new(e) }),
                // Binary Ops
                (arb_binary_op(), inner.clone(), inner).prop_map(|(op, left, right)| {
                    Expr::Binary { op, left: Box::new(left), right: Box::new(right) }
                }),
            ]
        },
    )
}

// --- Property Tests ---

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn prop_reify_roundtrip_consistency(expr in arb_expr()) {
        let mut registry = Registry::new();
        let entity = registry.ingest_expr(&expr).expect("Ingestion should succeed for generated AST");
        let reified = registry.reify_expr(entity).expect("Reification should succeed for valid AST");

        // Note: Formatting/parentheses might differ, so we compare structural equality
        // rather than string representation.
        assert_eq!(expr, reified, "AST must round-trip through ECS Registry exactly");
    }

    #[test]
    fn prop_parser_robustness(s in "\\PC*") {
        // The parser should never panic on random UTF-8 input.
        // It should either return Ok or a valid MirrError.
        let _ = parse_expression(&s);
    }
}
