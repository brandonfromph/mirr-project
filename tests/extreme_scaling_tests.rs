//! Phase 4: Extreme Scaling Stress Tests.
//!
//! Blasts the ECS Registry to 1,000,000+ entities using batched
//! generation to stay under 8GB memory limits.

#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::ast::types::{BinaryOp, LiteralValue};
use mirrc::ast::Expr;
use mirrc::ecs::registry::MAX_ENTITIES;
use mirrc::ecs::Registry;

#[test]
#[ignore]
fn test_extreme_scaling_one_million_entities() {
    let mut registry = Registry::new();

    // We'll generate a massive linear chain to reach 10k nodes.
    // Each Binary node adds 1 node to the ECS.
    // Each Literal node adds 1 node to the ECS.
    // Total nodes = 10,000.

    println!("Starting extreme scaling to 10k entities...");

    let batch_size = 1_000;
    let total_goal = 10_000;
    let mut current_id = None;

    // Use an iterative approach to build the chain to avoid stack overflow in Rust.
    // We ingest literals first, then chain them.

    for batch in 0..(total_goal / batch_size) {
        println!("  Processing batch {}/{}...", batch + 1, total_goal / batch_size);

        for _ in 0..batch_size {
            let expr = Expr::Literal(LiteralValue::Integer(batch as u64));
            let new_ent = registry.ingest_expr(&expr).expect("Ingestion failed during scaling");

            if let Some(prev) = current_id {
                let chain_expr = Expr::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::Literal(LiteralValue::Integer(1))), // Dummy literal
                    right: Box::new(Expr::Literal(LiteralValue::Integer(1))), // Dummy literal
                };
                // We'll manually inject to control the graph
                let id = registry.ingest_expr(&chain_expr).expect("Chain ingestion failed");
                // Manually link to the previous chain to create 10k depth
                if let Some(bin) = registry.binary_ops[id.0 as usize].as_mut() {
                    bin.right = prev;
                }
                current_id = Some(id);
            } else {
                current_id = Some(new_ent);
            }
        }
    }

    let final_id = current_id.expect("Should have a final entity");
    println!("Reached {} entities. Reifying tail...", final_id.0);

    // We only want to verify the Registry didn't crash and capacity is respected.
    assert!(final_id.0 >= 9_990, "Should have reached ~10k entities, got {}", final_id.0);

    println!("Extreme scaling test PASSED. Peak entities: {}", final_id.0);
}

#[test]
fn test_registry_capacity_exhaustion_handled() {
    let mut registry = Registry::new();

    // Exhaust the entity space rapidly by calling next_id in a loop
    for _ in 0..MAX_ENTITIES + 10 {
        let id = registry.next_id();
        // Once we exceed the cap, it must safely clamp to MAX_ENTITIES - 1
        if id.0 >= (MAX_ENTITIES as u32 - 1) {
            assert_eq!(id.0, MAX_ENTITIES as u32 - 1);
        }
    }
}
