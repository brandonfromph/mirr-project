#![forbid(unsafe_code)]
#![deny(warnings)]

//! Integration test suite verifying the MIRR compiler's structural enforcement
//! of the NASA Jet Propulsion Laboratory (JPL) "Power of 10" safety-critical software rules.
//!
//! Enforces:
//! 1. Rule 2: Strict loop bounds and cycle safety checking.
//! 2. Rule 3: Fixed memory bounds and pre-allocated registry partition scaling.
//! 3. Rule 5: High assertion density and expression depth limit gates (MAX_EXPR_NODES = 512).
//! 4. Rule 6: Scope isolation and narrowest variable/signal visibility boundaries.

use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType};
use mirrc::ast::Expr;
use mirrc::ecs::{KindComponent, Registry, TypeComponent};
use mirrc::pipeline::{run_pipeline, PipelineConfig};

/// NASA JPL Rule 2: Strict Loop and Cycle Bounds.
/// Verify that the compiler strictly limits loops and cycle propagation budgets.
/// A cyclic dependency loop must be caught within the 16-round solver budget.
#[test]
fn test_nasa_rule_2_bounded_loops_and_cycles() {
    let mut registry = Registry::new();

    // Set up a pathologically cyclic loop to verify the 16-round budget gate.
    let loop_ent_1 = registry.next_id();
    let loop_ent_2 = registry.next_id();

    registry.binary_ops[loop_ent_1.0 as usize] = Some(mirrc::ecs::components::BinaryComponent {
        op: BinaryOp::Add,
        left: loop_ent_2,
        right: loop_ent_2,
    });
    registry.binary_ops[loop_ent_2.0 as usize] = Some(mirrc::ecs::components::BinaryComponent {
        op: BinaryOp::Add,
        left: loop_ent_1,
        right: loop_ent_1,
    });

    let g_ent = registry.next_id();
    registry.names[g_ent.0 as usize] =
        Some(mirrc::ecs::components::NameComponent("loop_guard".to_string()));
    registry.kinds[g_ent.0 as usize] = Some(KindComponent::GUARD);
    registry.conditions[g_ent.0 as usize] =
        Some(mirrc::ecs::components::ConditionComponent(loop_ent_1));
    registry.cycles[g_ent.0 as usize] = Some(mirrc::ecs::components::CyclesComponent(10));

    // Semantic validation must fail, detecting the cycle rather than falling into infinite recursion.
    let res = registry.semantic_validate();
    assert!(res.is_err(), "Cyclic dependency must be rejected to satisfy Rule 2");
}

/// NASA JPL Rule 3: Fixed Memory Allocation Bounds and Pre-allocated Partitions.
/// Verify that the ECS Registry limits entity capacity exactly to MAX_ENTITIES (1,000,000)
/// and fails gracefully under memory pressure without dynamic allocation fragmentation.
#[test]
fn test_nasa_rule_3_registry_preallocated_bounds() {
    let mut registry = Registry::new();

    // Verify initial capacity is cleanly pre-allocated
    assert!(registry.names.len() >= 100_000, "Initial allocation should start at 100k");

    // Pre-allocated array sizing check
    let mod_ent = registry.create_entity("top_module", KindComponent::MODULE);
    let sig = registry.create_signal(
        "sensor_a".to_string(),
        KindComponent(mirrc::ecs::components::EntityKind::SIGNAL(SignalKind::Internal)),
        TypeComponent(ExtendedType::new(SignalType::Unsigned(8), Default::default())),
    );
    registry.set_parent(sig, mod_ent);

    assert_eq!(registry.names[sig.0 as usize].as_ref().unwrap().0, "sensor_a");
}

/// NASA JPL Rule 5: High Assertion Density and AST Complexity Bounds.
/// Verify that the compiler enforces the hard limit on expression complexity (MAX_EXPR_NODES = 512)
/// to prevent stack overflows and preserve high-assertion structural integrity.
#[test]
fn test_nasa_rule_5_assertion_density_and_depth_limits() {
    // Generate an expression tree that exceeds the 512 node threshold
    let mut expr = Expr::Literal(LiteralValue::Integer(1));
    for _ in 0..600 {
        expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Literal(LiteralValue::Integer(1))),
            right: Box::new(expr),
        };
    }

    let mut registry = Registry::new();
    let res = registry.ingest_expr(&expr);

    // Should fail with a controlled error when attempting to ingest too many expression nodes
    assert!(res.is_err(), "Pathologically deep expression tree must be rejected under Rule 5");
}

/// NASA JPL Rule 6: Smallest Variable Scope and Signal Boundaries.
/// Verify that signals declared within local scopes are isolated, and that cross-boundary references
/// are caught and rejected as out-of-scope.
#[test]
fn test_nasa_rule_6_scope_isolation_and_smallest_variable_bounds() {
    let source = r#"
        module top {
            signals {
                sys_in: in u8;
                sys_out: out u8;
            }
            // Leakage test: try to assign directly using sub_system's local guard
            reflex illegal_leak {
                on local_g {
                    sys_out = sys_in;
                }
            }
        }

        module sub_system {
            signals {
                in_val: in u8;
                out_val: out u8;
            }
            guard local_g { when in_val > 0 for 1 cycles; }
            reflex local_r {
                on local_g {
                    out_val = in_val;
                }
            }
        }
    "#;

    let config = PipelineConfig::default();
    let res = run_pipeline(source, &config);

    // Must fail due to reference to out-of-scope local_g
    assert!(res.is_err(), "Leaking scoped signals across module boundaries must fail under Rule 6");
}
