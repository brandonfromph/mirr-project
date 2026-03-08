//! Integration tests for Phase 4b — SCC-based width inference (FIRWINE Complete).
//!
//! Tests the full SCC pipeline: IR extension (Prev), Width Dependency Graph,
//! iterative Tarjan's SCC detection, SCC classification, nonexpansive solver,
//! expansive solver, and unique least solution verification.
//!
//! Categories:
//!  1. IR extension (Expr::Prev)
//!  2. Flattening (FlatNode::Prev)
//!  3. Graph construction
//!  4. SCC detection
//!  5. SCC classification
//!  6. Expansive SCC solving
//!  7. Nonexpansive SCC solving
//!  8. Least solution verification
//!  9. Full program integration

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Guard, MirrProgram, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{BinaryOp, LiteralValue, SignalKind, SignalType};
use nasa_rust_project::width;

// =========================================================================
// Helpers
// =========================================================================

fn sig(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl { name: name.to_string(), kind, ty, origin: None }
}

fn guard(name: &str, cond: Expr, cycles: u64) -> Guard {
    Guard { name: name.to_string(), condition: cond, cycles, origin: None }
}

fn reflex(name: &str, guard_names: &[&str], assignments: Vec<Assignment>) -> Reflex {
    Reflex {
        name: name.to_string(),
        guard_names: guard_names.iter().map(|s| s.to_string()).collect(),
        assignments,
        origin: None,
    }
}

fn assign(target: &str, value: Expr) -> Assignment {
    Assignment { target: target.to_string(), value }
}

fn prev(signal: &str, delay: u64) -> Expr {
    Expr::Prev { signal: signal.to_string(), delay }
}

fn signal_expr(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn int_lit(v: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(v))
}

fn add(left: Expr, right: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::Add, left: Box::new(left), right: Box::new(right) }
}

fn and_expr(left: Expr, right: Expr) -> Expr {
    Expr::Binary { op: BinaryOp::And, left: Box::new(left), right: Box::new(right) }
}

fn program(
    name: &str,
    signals: Vec<SignalDecl>,
    guards: Vec<Guard>,
    reflexes: Vec<Reflex>,
) -> MirrProgram {
    MirrProgram {
        patterns: Vec::new(),
        module: Module {
            name: name.to_string(),
            signals,
            guards,
            reflexes,
            properties: Vec::new(),
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
        },
    }
}

// =========================================================================
// 1. IR Extension (Expr::Prev)
// =========================================================================

#[test]
fn prev_roundtrips_through_serde() {
    let expr = prev("counter", 1);
    let json = serde_json::to_string(&expr).unwrap();
    let deserialized: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(expr, deserialized);
}

#[test]
fn prev_with_delay_greater_than_one() {
    let expr = prev("sr_out", 3);
    let json = serde_json::to_string(&expr).unwrap();
    assert!(json.contains("\"delay\":3"));
    let back: Expr = serde_json::from_str(&json).unwrap();
    assert_eq!(back, expr);
}

#[test]
fn prev_passes_through_simplifier_unchanged() {
    let expr = prev("x", 1);
    let simplified = nasa_rust_project::simplify::simplify_expr(expr.clone());
    assert_eq!(simplified, expr);
}

#[test]
fn prev_in_binary_expr_passes_through_simplifier() {
    // prev(x) + 0 => prev(x)  (arithmetic identity fires on the add, prev untouched)
    let expr = add(prev("x", 1), int_lit(0));
    let simplified = nasa_rust_project::simplify::simplify_expr(expr);
    assert_eq!(simplified, prev("x", 1));
}

#[test]
fn prev_display_format() {
    let expr = prev("counter", 2);
    let signals = vec![sig("counter", SignalKind::Internal, SignalType::Unsigned(16))];
    let result = width::infer_widths(&expr, &signals);
    assert!(result.expr.is_some());
    let formatted = width::display::format_width_expr(result.expr.as_ref().unwrap());
    assert!(formatted.contains("prev(counter"));
    assert!(formatted.contains("u16"));
}

// =========================================================================
// 2. Flattening (FlatNode::Prev)
// =========================================================================

#[test]
fn prev_flattens_to_flat_node() {
    let expr = prev("x", 1);
    let nodes = width::flatten::flatten_expr(&expr, &[]).unwrap();
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        width::types::FlatNode::Prev { signal, delay, .. } => {
            assert_eq!(signal, "x");
            assert_eq!(*delay, 1);
        }
        _ => panic!("expected FlatNode::Prev"),
    }
}

#[test]
fn prev_in_add_flattens_correctly() {
    // prev(x, 1) + 1
    let expr = add(prev("x", 1), int_lit(1));
    let nodes = width::flatten::flatten_expr(&expr, &[]).unwrap();
    assert_eq!(nodes.len(), 3); // prev, literal, add
}

#[test]
fn prev_reconstructs_with_correct_width() {
    let expr = prev("sensor", 1);
    let signals = vec![sig("sensor", SignalKind::Input, SignalType::Unsigned(12))];
    let result = width::infer_widths(&expr, &signals);
    assert!(!result.has_errors());
    let we = result.expr.unwrap();
    assert_eq!(we.width().0, 12);
}

// =========================================================================
// 3. Graph Construction
// =========================================================================

#[test]
fn acyclic_program_has_no_self_loops() {
    let prog = program(
        "acyclic",
        vec![
            sig("a", SignalKind::Input, SignalType::Unsigned(8)),
            sig("b", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", signal_expr("a"), 1)],
        vec![reflex("r1", &["g1"], vec![assign("b", signal_expr("a"))])],
    );
    let graph = width::graph::build_graph(&prog);
    // b depends on a, but a doesn't depend on b — no self-loops.
    assert_eq!(graph.node_count, 2);
    assert!(!graph.adj[1].contains(&1)); // b doesn't self-loop
}

#[test]
fn self_referencing_prev_creates_self_loop() {
    let prog = program(
        "counter_mod",
        vec![sig("counter", SignalKind::Internal, SignalType::Unsigned(8))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("counter", add(prev("counter", 1), int_lit(1)))])],
    );
    let graph = width::graph::build_graph(&prog);
    assert_eq!(graph.node_count, 1);
    assert!(graph.adj[0].contains(&0)); // counter -> counter (self-loop)
}

#[test]
fn multi_signal_cycle_detected_in_graph() {
    // sr0 := prev(sr2), sr1 := prev(sr0), sr2 := prev(sr1)
    let prog = program(
        "ring",
        vec![
            sig("sr0", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("sr1", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("sr2", SignalKind::Internal, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![
                assign("sr0", prev("sr2", 1)),
                assign("sr1", prev("sr0", 1)),
                assign("sr2", prev("sr1", 1)),
            ],
        )],
    );
    let graph = width::graph::build_graph(&prog);
    assert_eq!(graph.node_count, 3);
    // sr0 depends on sr2, sr1 depends on sr0, sr2 depends on sr1
    assert!(graph.adj[0].contains(&2)); // sr0 <- sr2
    assert!(graph.adj[1].contains(&0)); // sr1 <- sr0
    assert!(graph.adj[2].contains(&1)); // sr2 <- sr1
}

#[test]
fn unconnected_signals_are_isolated() {
    let prog = program(
        "isolated",
        vec![
            sig("a", SignalKind::Input, SignalType::Unsigned(8)),
            sig("b", SignalKind::Input, SignalType::Unsigned(16)),
            sig("c", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        vec![],
        vec![],
    );
    let graph = width::graph::build_graph(&prog);
    assert_eq!(graph.node_count, 3);
    assert!(graph.adj[0].is_empty());
    assert!(graph.adj[1].is_empty());
    assert!(graph.adj[2].is_empty());
}

#[test]
fn diamond_dependency_topology() {
    // a -> c, b -> c, a -> d, b -> d
    let prog = program(
        "diamond",
        vec![
            sig("a", SignalKind::Input, SignalType::Unsigned(8)),
            sig("b", SignalKind::Input, SignalType::Unsigned(8)),
            sig("c", SignalKind::Output, SignalType::Unsigned(9)),
            sig("d", SignalKind::Output, SignalType::Unsigned(9)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![
                assign("c", add(signal_expr("a"), signal_expr("b"))),
                assign("d", add(signal_expr("a"), signal_expr("b"))),
            ],
        )],
    );
    let graph = width::graph::build_graph(&prog);
    // c depends on a,b; d depends on a,b; no cycles.
    assert!(graph.adj[2].contains(&0)); // c <- a
    assert!(graph.adj[2].contains(&1)); // c <- b
    assert!(graph.adj[3].contains(&0)); // d <- a
    assert!(graph.adj[3].contains(&1)); // d <- b
}

// =========================================================================
// 4. SCC Detection
// =========================================================================

#[test]
fn no_sccs_in_acyclic_program() {
    let prog = program(
        "acyclic",
        vec![
            sig("a", SignalKind::Input, SignalType::Unsigned(8)),
            sig("b", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", signal_expr("a"), 1)],
        vec![reflex("r1", &["g1"], vec![assign("b", signal_expr("a"))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.sccs.len(), 0);
    assert_eq!(result.stats.scc_count, 0);
}

#[test]
fn single_self_loop_detected_as_scc() {
    let prog = program(
        "selfloop",
        vec![sig("x", SignalKind::Internal, SignalType::Unsigned(16))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 10)],
        vec![reflex("r1", &["g1"], vec![assign("x", add(prev("x", 1), int_lit(1)))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.sccs.len(), 1);
    assert_eq!(result.sccs[0].signal_indices.len(), 1);
}

#[test]
fn three_node_cycle_detected() {
    let prog = program(
        "ring3",
        vec![
            sig("a", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("c", SignalKind::Internal, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![assign("a", prev("c", 1)), assign("b", prev("a", 1)), assign("c", prev("b", 1))],
        )],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.stats.scc_count, 1);
    assert_eq!(result.sccs[0].signal_indices.len(), 3);
}

#[test]
fn singleton_without_self_loop_excluded() {
    // Signal assigned from a different signal (no cycle).
    let prog = program(
        "noscc",
        vec![
            sig("a", SignalKind::Input, SignalType::Unsigned(8)),
            sig("b", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("b", signal_expr("a"))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.stats.scc_count, 0);
}

// =========================================================================
// 5. SCC Classification
// =========================================================================

#[test]
fn prev_plus_constant_classified_expansive() {
    let prog = program(
        "expansive",
        vec![sig("counter", SignalKind::Internal, SignalType::Unsigned(8))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 100)],
        vec![reflex("r1", &["g1"], vec![assign("counter", add(prev("counter", 1), int_lit(1)))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.sccs.len(), 1);
    assert_eq!(result.sccs[0].kind, width::types::SccKind::Expansive);
    assert_eq!(result.stats.expansive_count, 1);
}

#[test]
fn pure_prev_chain_classified_nonexpansive() {
    let prog = program(
        "nonexpansive",
        vec![
            sig("a", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("a", prev("b", 1)), assign("b", prev("a", 1))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.sccs.len(), 1);
    assert_eq!(result.sccs[0].kind, width::types::SccKind::Nonexpansive);
    assert_eq!(result.stats.nonexpansive_count, 1);
}

#[test]
fn and_in_cycle_classified_nonexpansive() {
    // x := prev(x) & mask — AND is nonexpansive.
    let prog = program(
        "mask_cycle",
        vec![
            sig("x", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("mask", SignalKind::Input, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("x", and_expr(prev("x", 1), signal_expr("mask")))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.sccs.len(), 1);
    assert_eq!(result.sccs[0].kind, width::types::SccKind::Nonexpansive);
}

// =========================================================================
// 6. Expansive SCC Solving
// =========================================================================

#[test]
fn counter_with_explicit_annotation_resolves() {
    let prog = program(
        "counter_u8",
        vec![sig("counter", SignalKind::Internal, SignalType::Unsigned(8))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 255)],
        vec![reflex("r1", &["g1"], vec![assign("counter", add(prev("counter", 1), int_lit(1)))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    // Should resolve to u8 from the explicit annotation.
    let scc = &result.sccs[0];
    let solve = &result.scc_solves[0].1;
    assert_eq!(solve.widths[0], 8);
    assert!(scc.kind == width::types::SccKind::Expansive);
}

#[test]
fn counter_with_guard_bound_infers_width() {
    // counter := prev(counter) + 1, guarded by `for 255 cycles`.
    // Max value = 1 * 255 = 255, needs 8 bits.
    let prog = program(
        "bounded_counter",
        vec![sig("counter", SignalKind::Internal, SignalType::Unsigned(8))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 255)],
        vec![reflex("r1", &["g1"], vec![assign("counter", add(prev("counter", 1), int_lit(1)))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    // Explicit annotation takes priority — u8.
    // Phase 4a flags truncation (u9 -> u8), but SCC solver is clean.
    assert_eq!(result.scc_solves[0].1.widths[0], 8);
    assert!(!result.has_errors());
}

#[test]
fn counter_with_large_guard_bound_infers_u10() {
    // counter := prev(counter) + 1, for 1000 cycles.
    // Max value = 1000, needs 10 bits.
    let prog = program(
        "big_counter",
        vec![sig("counter", SignalKind::Internal, SignalType::Unsigned(10))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1000)],
        vec![reflex("r1", &["g1"], vec![assign("counter", add(prev("counter", 1), int_lit(1)))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.scc_solves[0].1.widths[0], 10);
    assert!(!result.has_errors());
}

#[test]
fn accumulator_plus_sensor_with_annotation() {
    // accum := prev(accum) + sensor, sensor is u8.
    // With explicit u16 annotation, resolves to 16.
    let prog = program(
        "accumulator",
        vec![
            sig("accum", SignalKind::Internal, SignalType::Unsigned(16)),
            sig("sensor", SignalKind::Input, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 100)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![assign("accum", add(prev("accum", 1), signal_expr("sensor")))],
        )],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert!(!result.has_errors());
    // accum resolved from explicit annotation.
    assert!(result.scc_solves[0].1.widths[0] == 16);
}

#[test]
fn expansive_scc_error_pinned_by_signal_name() {
    // This test creates a counter WITHOUT an annotation (SignalType::Unsigned(0))
    // which should trigger the hard error.
    let prog = program(
        "nobound",
        vec![sig("x", SignalKind::Internal, SignalType::Unsigned(0))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 0)],
        vec![reflex("r1", &["g1"], vec![assign("x", add(prev("x", 1), int_lit(1)))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert!(result.has_errors());
    let error_msgs: Vec<&str> = result
        .scc_diagnostics
        .iter()
        .filter(|d| d.severity == width::types::DiagSeverity::Error)
        .map(|d| d.message.as_str())
        .collect();
    assert!(
        error_msgs.iter().any(|m| m.contains("'x'") && m.contains("expansive SCC")),
        "Expected error mentioning signal 'x' in expansive SCC, got: {:?}",
        error_msgs
    );
}

// =========================================================================
// 7. Nonexpansive SCC Solving
// =========================================================================

#[test]
fn two_signal_ring_preserves_width() {
    let prog = program(
        "ring2",
        vec![
            sig("a", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("a", prev("b", 1)), assign("b", prev("a", 1))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.sccs.len(), 1);
    assert_eq!(result.sccs[0].kind, width::types::SccKind::Nonexpansive);
    let solve = &result.scc_solves[0].1;
    // Both should converge to u8 (max of their declarations).
    assert_eq!(solve.widths[0], 8);
    assert_eq!(solve.widths[1], 8);
}

#[test]
fn mixed_width_ring_takes_max() {
    // a is u8, b is u16 — nonexpansive cycle should converge to u16.
    let prog = program(
        "mixed",
        vec![
            sig("a", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(16)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("a", prev("b", 1)), assign("b", prev("a", 1))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    let solve = &result.scc_solves[0].1;
    // Floyd-Warshall converges both to max(8, 16) = 16.
    assert_eq!(solve.widths[0], 16);
    assert_eq!(solve.widths[1], 16);
}

#[test]
fn three_node_ring_buffer_from_cement2() {
    // Simulates a Cement2 shift register: sr0 <- sr2, sr1 <- sr0, sr2 <- sr1.
    let prog = program(
        "sr3",
        vec![
            sig("sr0", SignalKind::Internal, SignalType::Unsigned(1)),
            sig("sr1", SignalKind::Internal, SignalType::Unsigned(1)),
            sig("sr2", SignalKind::Internal, SignalType::Unsigned(1)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![
                assign("sr0", prev("sr2", 1)),
                assign("sr1", prev("sr0", 1)),
                assign("sr2", prev("sr1", 1)),
            ],
        )],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.sccs.len(), 1);
    assert_eq!(result.sccs[0].kind, width::types::SccKind::Nonexpansive);
    let solve = &result.scc_solves[0].1;
    assert!(solve.widths.iter().all(|&w| w == 1));
}

#[test]
fn nonexpansive_no_anchor_produces_error() {
    // All signals have Unsigned(0) — no width anchor.
    let prog = program(
        "noanchor",
        vec![
            sig("a", SignalKind::Internal, SignalType::Unsigned(0)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(0)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("a", prev("b", 1)), assign("b", prev("a", 1))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert!(result.has_errors());
    assert!(
        result.scc_diagnostics.iter().any(|d| d.message.contains("no width anchor")),
        "Expected 'no width anchor' error, got: {:?}",
        result.scc_diagnostics
    );
}

// =========================================================================
// 8. Least Solution Verification
// =========================================================================

#[test]
fn acyclic_solution_verified_minimal() {
    let prog = program(
        "simple",
        vec![
            sig("a", SignalKind::Input, SignalType::Unsigned(8)),
            sig("b", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", signal_expr("a"), 1)],
        vec![reflex("r1", &["g1"], vec![assign("b", signal_expr("a"))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert!(result.verification.is_minimal);
    assert!(result.verification.diagnostics.is_empty());
}

#[test]
fn nonexpansive_solution_verified_minimal() {
    let prog = program(
        "ring_verify",
        vec![
            sig("a", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("a", prev("b", 1)), assign("b", prev("a", 1))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert!(result.verification.is_minimal);
}

#[test]
fn empty_program_trivially_minimal() {
    let prog = program("empty", vec![], vec![], vec![]);
    let result = width::infer_program_widths_with_scc(&prog);
    assert!(result.verification.is_minimal);
    assert_eq!(result.stats.scc_count, 0);
}

// =========================================================================
// 9. Full Program Integration
// =========================================================================

#[test]
fn program_with_mixed_acyclic_and_cyclic() {
    // Acyclic: b := a (input -> output).
    // Cyclic: counter := prev(counter) + 1 (self-loop, expansive).
    let prog = program(
        "mixed_mode",
        vec![
            sig("a", SignalKind::Input, SignalType::Unsigned(8)),
            sig("b", SignalKind::Output, SignalType::Unsigned(8)),
            sig("counter", SignalKind::Internal, SignalType::Unsigned(16)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1000)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![
                assign("b", signal_expr("a")),
                assign("counter", add(prev("counter", 1), int_lit(1))),
            ],
        )],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.stats.scc_count, 1);
    assert_eq!(result.stats.expansive_count, 1);
    assert!(!result.has_errors());
}

#[test]
fn phase3_then_phase4b_integration() {
    // Expression with simplifiable constant + cyclic ref.
    // prev(x) + (1 + 0) => prev(x) + 1 after simplification.
    let prog = program(
        "simplify_then_scc",
        vec![sig("x", SignalKind::Internal, SignalType::Unsigned(8))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 255)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![assign("x", add(prev("x", 1), add(int_lit(1), int_lit(0))))],
        )],
    );
    // First simplify, then run SCC analysis.
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.stats.scc_count, 1);
    assert!(!result.has_errors());
}

#[test]
fn phase4a_tests_still_pass_regression() {
    // Basic Phase 4a test: signal width propagation.
    let expr = signal_expr("x");
    let signals = vec![sig("x", SignalKind::Input, SignalType::Unsigned(8))];
    let result = width::infer_widths(&expr, &signals);
    assert!(!result.has_errors());
    assert_eq!(result.expr.unwrap().width().0, 8);
}

#[test]
fn program_with_both_expansive_and_nonexpansive_sccs() {
    // Expansive: counter := prev(counter) + 1
    // Nonexpansive: a := prev(b), b := prev(a)
    let prog = program(
        "dual_scc",
        vec![
            sig("counter", SignalKind::Internal, SignalType::Unsigned(16)),
            sig("a", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1000)],
        vec![reflex(
            "r1",
            &["g1"],
            vec![
                assign("counter", add(prev("counter", 1), int_lit(1))),
                assign("a", prev("b", 1)),
                assign("b", prev("a", 1)),
            ],
        )],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    assert_eq!(result.stats.scc_count, 2);
    assert_eq!(result.stats.expansive_count, 1);
    assert_eq!(result.stats.nonexpansive_count, 1);
    assert!(!result.has_errors());
}

#[test]
fn scc_report_format_includes_signal_names() {
    let prog = program(
        "report_test",
        vec![
            sig("a", SignalKind::Internal, SignalType::Unsigned(8)),
            sig("b", SignalKind::Internal, SignalType::Unsigned(8)),
        ],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 1)],
        vec![reflex("r1", &["g1"], vec![assign("a", prev("b", 1)), assign("b", prev("a", 1))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    let signal_names: Vec<String> = prog.module.signals.iter().map(|s| s.name.clone()).collect();
    let report = width::display::format_scc_report(&result.sccs, &signal_names);
    assert!(report.contains("SCCs detected: 1"));
    assert!(report.contains("nonexpansive"));
}

#[test]
fn stats_include_scc_fields() {
    let prog = program(
        "stats_test",
        vec![sig("x", SignalKind::Internal, SignalType::Unsigned(8))],
        vec![guard("g1", Expr::Literal(LiteralValue::Bool(true)), 10)],
        vec![reflex("r1", &["g1"], vec![assign("x", add(prev("x", 1), int_lit(1)))])],
    );
    let result = width::infer_program_widths_with_scc(&prog);
    let formatted = width::display::format_stats(&result.stats);
    assert!(formatted.contains("sccs=1"));
    assert!(formatted.contains("expansive=1"));
    assert!(formatted.contains("nonexpansive=0"));
}
