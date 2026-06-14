#![forbid(unsafe_code)]

use mirrc::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use mirrc::ast::Expr;
use mirrc::ecs::Registry;
use mirrc::simplify::simplify_expr_with_stats;
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

// --- Shadow Execution Fuzzing ---

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))] // 500 random AST trees

    #[test]
    fn prop_shadow_simplifier_parity(expr in arb_expr()) {
        // 1. Run Legacy AST Simplifier
        let (ast_simplified, ast_stats) = simplify_expr_with_stats(expr.clone());

        // 2. Run Modern ECS Simplifier
        let mut registry = Registry::new();
        let entity = registry.ingest_expr(&expr).expect("Ingestion should succeed for generated AST");

        // The ECS system simplifies in-place within the Registry arrays
        let ecs_stats = mirrc::ecs::systems::simplifier_system(&mut registry);

        let ecs_simplified = registry.reify_expr(entity).expect("Reification should succeed");

        // 3. Assert 100% Mathematical Parity
        // We only compare rules_applied because the node counting mechanism differs
        // between AST (pointer traversal) and ECS (array bounds).
        assert_eq!(
            ast_stats.rules_applied, ecs_stats.rules_applied,
            "Rule application count mismatch between AST and ECS! Legacy={}, ECS={}",
            ast_stats.rules_applied, ecs_stats.rules_applied
        );

        assert_eq!(
            ast_simplified, ecs_simplified,
            "AST mismatch after simplification! Legacy={:?}, ECS={:?}",
            ast_simplified, ecs_simplified
        );
    }
}
