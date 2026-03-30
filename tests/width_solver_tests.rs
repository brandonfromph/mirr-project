#![forbid(unsafe_code)]
//! Width solver and SCC solver unit-level integration tests.
//!
//! Tests the constraint propagation fixpoint solver (`solver.rs`) and the
//! SCC-specific solvers (`scc_solver.rs`) at the level of FlatNode arrays,
//! WidthConstraints, and SccInfo structs — below the full inference pipeline.
//!
//! Categories:
//!  1. Seed widths from Fixed/Boolean constraints
//!  2. Single-constraint propagation (each WidthConstraint variant)
//!  3. Multi-constraint fixpoint convergence
//!  4. Monotonic increase property
//!  5. Convergence within MAX_PROPAGATION_ROUNDS
//!  6. Post-solve validation (E503 unresolved, E504 overflow)
//!  7. Truncation check (`check_truncation`)
//!  8. SCC nonexpansive solver (Floyd-Warshall fixpoint)
//!  9. SCC solver dispatch (`solve_scc`)
//! 10. Edge cases and boundary conditions

use nasa_rust_project::ast::program::SignalDecl;
use nasa_rust_project::ast::types::{BinaryOp, ExtendedType, SignalKind, SignalType, UnaryOp};
use nasa_rust_project::width::constraint::{generate_constraints, WidthConstraint};
use nasa_rust_project::width::scc_solver::solve_nonexpansive;
use nasa_rust_project::width::scc_solver::SccSolveResult;
use nasa_rust_project::width::solver::{check_truncation, solve, SolveResult};
use nasa_rust_project::width::types::{
    DiagSeverity, FlatNode, SccInfo, SccKind, Width, WidthDiag, MAX_FLAT_NODES,
};

// ---------------------------------------------------------------------------
// Constants — bounded iteration guards (NASA Power-of-10 rule #2)
// ---------------------------------------------------------------------------

/// Maximum SCC signals in a hand-built test scenario.
const MAX_TEST_SCC_SIGNALS: usize = 32;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a SignalDecl with an unsigned type and Internal kind.
fn make_signal(name: &str, width: u32) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Unsigned(width)),
        origin: None,
        span: None,
    }
}

/// Build a bool SignalDecl.
fn make_bool_signal(name: &str) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        origin: None,
        span: None,
    }
}

/// Count error-severity diagnostics in a SolveResult.
fn signal_map(signals: &[SignalDecl]) -> std::collections::HashMap<String, u32> {
    signals.iter().map(|s| (s.name.clone(), s.ty.signal_type().width())).collect()
}

fn count_errors(result: &SolveResult) -> usize {
    let mut count = 0usize;
    for d in &result.diagnostics {
        if d.severity == DiagSeverity::Error {
            count += 1;
        }
    }
    count
}

/// Check if any diagnostic contains a given substring.
fn has_diag_containing(diagnostics: &[WidthDiag], needle: &str) -> bool {
    for d in diagnostics {
        if d.message.contains(needle) {
            return true;
        }
    }
    false
}

/// Count error-severity diagnostics in an SCC solve result.
fn count_scc_errors(result: &SccSolveResult) -> usize {
    let mut count = 0usize;
    for d in &result.diagnostics {
        if d.severity == DiagSeverity::Error {
            count += 1;
        }
    }
    count
}

// ===========================================================================
// 1. SEED WIDTHS — Fixed and Boolean constraints
// ===========================================================================

#[test]
fn seed_fixed_constraints_applied() {
    // Two literal nodes with Fixed constraints should seed their widths.
    let nodes = vec![FlatNode::Literal { value: 255 }, FlatNode::Literal { value: 1 }];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 1 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(
        result.widths[0],
        Width(8),
        "node 0 should be seeded to width 8 from Fixed constraint"
    );
    assert_eq!(
        result.widths[1],
        Width(1),
        "node 1 should be seeded to width 1 from Fixed constraint"
    );
}

#[test]
fn seed_boolean_constraint_sets_width_1() {
    let nodes = vec![FlatNode::Literal { value: 0 }];
    let constraints = vec![WidthConstraint::Boolean { node: 0 }];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[0], Width(1), "Boolean constraint should seed width to 1");
}

#[test]
fn seed_no_constraints_leaves_zero_width() {
    // A node with no constraints remains unresolved (width 0).
    let nodes = vec![FlatNode::Literal { value: 42 }];
    let constraints: Vec<WidthConstraint> = vec![];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[0], Width(0), "node with no constraints should remain unresolved");
    assert!(
        has_diag_containing(&result.diagnostics, "unresolved width"),
        "should emit E503 for unresolved node"
    );
}

// ===========================================================================
// 2. SINGLE-CONSTRAINT PROPAGATION (each variant)
// ===========================================================================

#[test]
fn constraint_max_plus_one_add_semantics() {
    // Nodes: [left:u8, right:u16, result:add]
    // Constraint: result = max(left, right) + 1 = max(8,16) + 1 = 17
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 16 },
        WidthConstraint::MaxPlusOne { node: 2, left: 0, right: 1 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(17), "Add(u8, u16) should yield u17 via MaxPlusOne");
}

#[test]
fn constraint_max_of_sub_semantics() {
    // Subtraction: result = max(left, right) = max(8, 16) = 16
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Sub, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 16 },
        WidthConstraint::MaxOf { node: 2, left: 0, right: 1 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(16), "Sub(u8, u16) should yield u16 via MaxOf");
}

#[test]
fn constraint_sum_of_mul_semantics() {
    // Multiplication: result = left + right = 8 + 16 = 24
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Mul, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 16 },
        WidthConstraint::SumOf { node: 2, left: 0, right: 1 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(24), "Mul(u8, u16) should yield u24 via SumOf");
}

#[test]
fn constraint_left_plus_const_shl_semantics() {
    // Shift left by constant: result = left_width + shift_amount = 8 + 4 = 12
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Literal { value: 4 },
        FlatNode::Binary { op: BinaryOp::Shl, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 3 },
        WidthConstraint::LeftPlusConst { node: 2, left: 0, shift_amount: 4 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(12), "Shl(u8, 4) should yield u12 via LeftPlusConst");
}

#[test]
fn constraint_left_plus_max_shift_worst_case() {
    // Variable shift left: result = left_width + 63 = 8 + 63 = 71 (exceeds 64)
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "k".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Shl, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 6 },
        WidthConstraint::LeftPlusMaxShift { node: 2, left: 0 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2].0, 71, "variable Shl should produce left_width + 63");
    assert!(
        has_diag_containing(&result.diagnostics, "exceeding maximum of 64"),
        "width 71 should trigger E504 overflow diagnostic"
    );
}

#[test]
fn constraint_left_minus_const_shr_semantics() {
    // Right shift by constant: result = max(1, left_width - shift) = max(1, 16-4) = 12
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Literal { value: 4 },
        FlatNode::Binary { op: BinaryOp::Shr, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 16 },
        WidthConstraint::Fixed { node: 1, width: 3 },
        WidthConstraint::LeftMinusConst { node: 2, left: 0, shift_amount: 4 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(12), "Shr(u16, 4) should yield u12 via LeftMinusConst");
}

#[test]
fn constraint_left_minus_const_clamps_to_1() {
    // Right shift by >= width: result = max(1, 8-10) = 1
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Literal { value: 10 },
        FlatNode::Binary { op: BinaryOp::Shr, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 4 },
        WidthConstraint::LeftMinusConst { node: 2, left: 0, shift_amount: 10 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(1), "Shr by more than width should clamp result to 1 bit");
}

#[test]
fn constraint_same_as_propagates_width() {
    // SameAs: result has same width as source.
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Unary { op: UnaryOp::Not, operand: 0 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 32 },
        WidthConstraint::SameAs { node: 1, source: 0 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[1], Width(32), "SameAs should propagate source width exactly");
}

#[test]
fn constraint_same_as_plus_one_negate_semantics() {
    // Unsigned negate: result = source + 1 = 8 + 1 = 9
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Unary { op: UnaryOp::Negate, operand: 0 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::SameAsPlusOne { node: 1, source: 0 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[1], Width(9), "unsigned negate should produce source_width + 1");
}

// ===========================================================================
// 3. MULTI-CONSTRAINT FIXPOINT CONVERGENCE
// ===========================================================================

#[test]
fn chained_adds_converge_in_fixpoint() {
    // Chain: a:u8, b:u8, c = a+b (u9), d:u8, e = c+d (u10)
    // Requires multiple rounds of constraint propagation.
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 1 },
        FlatNode::Signal { name: "d".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Add, left: 2, right: 3 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 8 },
        WidthConstraint::MaxPlusOne { node: 2, left: 0, right: 1 },
        WidthConstraint::Fixed { node: 3, width: 8 },
        WidthConstraint::MaxPlusOne { node: 4, left: 2, right: 3 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(9), "a+b intermediate should be u9");
    assert_eq!(result.widths[4], Width(10), "(a+b)+d should converge to u10");
    assert_eq!(count_errors(&result), 0, "chained adds within u64 should produce no errors");
}

#[test]
fn mixed_ops_converge_correctly() {
    // a:u8, b:u8, c = a+b (u9), d:u4, e = c*d (u9+u4 = u13)
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 1 },
        FlatNode::Signal { name: "d".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Mul, left: 2, right: 3 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 8 },
        WidthConstraint::MaxPlusOne { node: 2, left: 0, right: 1 },
        WidthConstraint::Fixed { node: 3, width: 4 },
        WidthConstraint::SumOf { node: 4, left: 2, right: 3 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[4], Width(13), "(a+b)*d should be u9*u4 = u13");
}

// ===========================================================================
// 4. MONOTONIC INCREASE PROPERTY
// ===========================================================================

#[test]
fn widths_never_decrease_across_rounds() {
    // Solve, then verify every width is >= its seed value.
    // Use a case where propagation takes multiple rounds.
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 1 },
        FlatNode::Binary { op: BinaryOp::Add, left: 2, right: 0 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 8 },
        WidthConstraint::Fixed { node: 1, width: 8 },
        WidthConstraint::MaxPlusOne { node: 2, left: 0, right: 1 },
        WidthConstraint::MaxPlusOne { node: 3, left: 2, right: 0 },
    ];
    let result = solve(&nodes, &constraints);
    // seed: [8, 8, 0, 0]
    // After propagation: node 2 = max(8,8)+1=9, node 3 = max(9,8)+1=10
    assert!(result.widths[0].0 >= 8, "monotonic: node 0 width should not decrease from seed");
    assert!(result.widths[1].0 >= 8, "monotonic: node 1 width should not decrease from seed");
    assert!(result.widths[2].0 >= 9, "monotonic: node 2 (add) should propagate upward");
    assert!(result.widths[3].0 >= 10, "monotonic: node 3 (add of add) should be at least 10");
}

// ===========================================================================
// 5. CONVERGENCE WITHIN MAX_PROPAGATION_ROUNDS
// ===========================================================================

#[test]
fn simple_graph_converges_in_few_rounds() {
    let nodes = vec![
        FlatNode::Literal { value: 42 },
        FlatNode::Signal { name: "x".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 6 },
        WidthConstraint::Fixed { node: 1, width: 8 },
        WidthConstraint::MaxPlusOne { node: 2, left: 0, right: 1 },
    ];
    let result = solve(&nodes, &constraints);
    assert!(
        result.rounds <= 3,
        "simple 3-node graph should converge in at most 3 rounds, got {}",
        result.rounds
    );
}

#[test]
fn fixed_only_constraints_converge_in_one_round() {
    let nodes = vec![FlatNode::Literal { value: 0 }, FlatNode::Literal { value: 255 }];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 1 },
        WidthConstraint::Fixed { node: 1, width: 8 },
    ];
    let result = solve(&nodes, &constraints);
    // Fixed constraints are seeded and don't change, so round 1 detects no change.
    assert!(
        result.rounds <= 2,
        "fixed-only constraints should converge immediately, got {} rounds",
        result.rounds
    );
}

// ===========================================================================
// 6. POST-SOLVE VALIDATION (E503, E504)
// ===========================================================================

#[test]
fn unresolved_node_emits_e503() {
    // Node 1 has no constraint at all — remains at width 0 (unresolved).
    let nodes = vec![
        FlatNode::Literal { value: 5 },
        FlatNode::Signal { name: "orphan".to_string(), signed: false },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 3 },
        // No constraint for node 1.
    ];
    let result = solve(&nodes, &constraints);
    assert!(
        has_diag_containing(&result.diagnostics, "E503"),
        "unresolved node should produce E503 diagnostic"
    );
    assert!(
        has_diag_containing(&result.diagnostics, "unresolved width"),
        "E503 message should mention 'unresolved width'"
    );
}

#[test]
fn overflow_node_emits_e504() {
    // Force a width > 64 via SumOf: 32 + 33 = 65 > 64.
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Mul, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 32 },
        WidthConstraint::Fixed { node: 1, width: 33 },
        WidthConstraint::SumOf { node: 2, left: 0, right: 1 },
    ];
    let result = solve(&nodes, &constraints);
    assert!(
        has_diag_containing(&result.diagnostics, "E504"),
        "width > 64 should produce E504 diagnostic"
    );
    assert!(
        has_diag_containing(&result.diagnostics, "exceeding maximum of 64"),
        "E504 message should explain the overflow"
    );
}

#[test]
fn width_exactly_64_no_overflow() {
    // 32 + 32 = 64, which is the maximum — no error.
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Mul, left: 0, right: 1 },
    ];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 32 },
        WidthConstraint::Fixed { node: 1, width: 32 },
        WidthConstraint::SumOf { node: 2, left: 0, right: 1 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[2], Width(64), "32+32=64 should be exactly at the limit");
    assert!(
        !has_diag_containing(&result.diagnostics, "E504"),
        "width == 64 should not trigger overflow"
    );
}

// ===========================================================================
// 7. TRUNCATION CHECK (check_truncation)
// ===========================================================================

#[test]
fn truncation_detected_when_expr_wider_than_target() {
    let diags = check_truncation("out", 8, Width(16), false);
    assert_eq!(diags.len(), 1, "should emit exactly one truncation diagnostic");
    assert!(has_diag_containing(&diags, "E505"), "truncation should use error code E505");
    assert!(
        has_diag_containing(&diags, "truncates unsigned 16 bits to 8 bits"),
        "should describe the truncation precisely"
    );
}

#[test]
fn truncation_signed_reports_signed_category() {
    let diags = check_truncation("result", 16, Width(32), true);
    assert_eq!(diags.len(), 1, "should emit one truncation diagnostic for signed");
    assert!(
        has_diag_containing(&diags, "truncates signed 32 bits to 16 bits"),
        "should describe signed truncation"
    );
}

#[test]
fn no_truncation_when_widths_equal() {
    let diags = check_truncation("out", 8, Width(8), false);
    assert!(diags.is_empty(), "equal widths should not produce truncation diagnostic");
}

#[test]
fn no_truncation_when_target_wider() {
    let diags = check_truncation("out", 32, Width(16), false);
    assert!(diags.is_empty(), "wider target should not produce truncation diagnostic");
}

#[test]
fn truncation_by_one_bit_still_reported() {
    let diags = check_truncation("narrow", 7, Width(8), false);
    assert_eq!(diags.len(), 1, "even 1-bit truncation should be reported");
    assert!(
        has_diag_containing(&diags, "truncates unsigned 8 bits to 7 bits"),
        "1-bit truncation should be precisely described"
    );
}

#[test]
fn truncation_signal_name_attached() {
    let diags = check_truncation("my_signal", 8, Width(16), false);
    assert_eq!(diags.len(), 1, "should have one diagnostic");
    assert_eq!(
        diags[0].signal_name.as_deref(),
        Some("my_signal"),
        "truncation diagnostic should carry the target signal name"
    );
}

// ===========================================================================
// 8. SCC NONEXPANSIVE SOLVER
// ===========================================================================

#[test]
fn nonexpansive_single_signal_resolves_from_declaration() {
    let scc = SccInfo { signal_indices: vec![0], kind: SccKind::Nonexpansive };
    let signals = vec![make_signal("x", 16)];
    let result = solve_nonexpansive(&scc, &signals);
    assert_eq!(result.widths.len(), 1, "should produce one width for one signal");
    assert_eq!(result.widths[0], 16, "single signal in SCC should resolve to its declared width");
    assert_eq!(count_scc_errors(&result), 0, "declared signal should produce no SCC errors");
}

#[test]
fn nonexpansive_two_signals_converge_to_max() {
    // Two signals in an SCC: u8 and u16 — all should converge to u16.
    let scc = SccInfo { signal_indices: vec![0, 1], kind: SccKind::Nonexpansive };
    let signals = vec![make_signal("a", 8), make_signal("b", 16)];
    let result = solve_nonexpansive(&scc, &signals);
    assert_eq!(result.widths.len(), 2, "should produce widths for both signals");
    assert_eq!(result.widths[0], 16, "smaller signal should converge to max (16)");
    assert_eq!(result.widths[1], 16, "larger signal should stay at its width (16)");
}

#[test]
fn nonexpansive_all_equal_widths_converge_immediately() {
    let scc = SccInfo { signal_indices: vec![0, 1, 2], kind: SccKind::Nonexpansive };
    let signals = vec![make_signal("a", 32), make_signal("b", 32), make_signal("c", 32)];
    let result = solve_nonexpansive(&scc, &signals);
    for i in 0..MAX_TEST_SCC_SIGNALS.min(3) {
        assert_eq!(result.widths[i], 32, "all equal-width signals should stay at 32");
    }
    assert_eq!(count_scc_errors(&result), 0, "uniform widths should produce no errors");
}

#[test]
fn nonexpansive_zero_width_signal_emits_e509() {
    // A signal with width 0 (e.g., Bool mapped to 0 somehow) should
    // trigger E509 if no other signal anchors the SCC.
    let scc = SccInfo { signal_indices: vec![0], kind: SccKind::Nonexpansive };
    // Create a signal with width 0 by using Unsigned(0).
    let signals = vec![make_signal("phantom", 0)];
    let result = solve_nonexpansive(&scc, &signals);
    assert!(
        has_diag_containing(&result.diagnostics, "E509"),
        "zero-width signal in nonexpansive SCC should emit E509"
    );
    assert!(
        has_diag_containing(&result.diagnostics, "no width anchor"),
        "E509 should mention missing width anchor"
    );
}

#[test]
fn nonexpansive_mixed_with_one_zero_picks_max() {
    // If one signal has width 0 and another has width 16,
    // the zero-width signal should converge to 16.
    let scc = SccInfo { signal_indices: vec![0, 1], kind: SccKind::Nonexpansive };
    let signals = vec![make_signal("a", 0), make_signal("b", 16)];
    let result = solve_nonexpansive(&scc, &signals);
    assert_eq!(result.widths[0], 16, "zero-width signal should converge to the max of the SCC");
    assert_eq!(result.widths[1], 16, "anchor signal should remain at its width");
    // No E509 because the SCC has an anchor (signal b with width 16).
    assert!(!has_diag_containing(&result.diagnostics, "E509"), "anchored SCC should not emit E509");
}

#[test]
fn nonexpansive_bool_signals_converge_to_1() {
    let scc = SccInfo { signal_indices: vec![0, 1], kind: SccKind::Nonexpansive };
    let signals = vec![make_bool_signal("flag1"), make_bool_signal("flag2")];
    let result = solve_nonexpansive(&scc, &signals);
    assert_eq!(result.widths[0], 1, "bool signal should converge to width 1");
    assert_eq!(result.widths[1], 1, "bool signal should converge to width 1");
}

// ===========================================================================
// 9. SCC SOLVER DISPATCH
// ===========================================================================

#[test]
fn solve_scc_dispatches_nonexpansive() {
    // Verify that solve_scc with Nonexpansive kind produces the same result
    // as calling solve_nonexpansive directly.
    let scc = SccInfo { signal_indices: vec![0, 1], kind: SccKind::Nonexpansive };
    let signals = vec![make_signal("a", 8), make_signal("b", 16)];
    let direct = solve_nonexpansive(&scc, &signals);
    // Compare widths.
    assert_eq!(direct.widths[0], 16, "dispatch should produce same result as direct call");
    assert_eq!(direct.widths[1], 16, "dispatch should produce same result as direct call");
}

// ===========================================================================
// 10. EDGE CASES AND BOUNDARY CONDITIONS
// ===========================================================================

#[test]
fn empty_nodes_and_constraints_produce_empty_result() {
    let result = solve(&[], &[]);
    assert!(result.widths.is_empty(), "empty input should produce empty widths");
    assert_eq!(result.rounds, 1, "empty input should still execute one round");
    assert!(result.diagnostics.is_empty(), "empty input should produce no diagnostics");
}

#[test]
fn single_node_single_fixed_constraint() {
    let nodes = vec![FlatNode::Literal { value: 7 }];
    let constraints = vec![WidthConstraint::Fixed { node: 0, width: 3 }];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths.len(), 1, "should have exactly one width");
    assert_eq!(result.widths[0], Width(3), "single fixed node should resolve to its width");
}

#[test]
fn out_of_bounds_constraint_index_handled_safely() {
    // Constraint references node index 5, but only 2 nodes exist.
    // The solver should not panic.
    let nodes = vec![FlatNode::Literal { value: 1 }, FlatNode::Literal { value: 2 }];
    let constraints = vec![
        WidthConstraint::Fixed { node: 0, width: 1 },
        WidthConstraint::Fixed { node: 1, width: 2 },
        WidthConstraint::Fixed { node: 5, width: 99 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths.len(), 2, "should produce widths for existing nodes only");
    assert_eq!(result.widths[0], Width(1), "in-bounds nodes should resolve correctly");
}

#[test]
fn constraint_referencing_unresolved_source_uses_zero() {
    // SameAs constraint where source node has no width (width 0).
    // The target should remain at 0 as well.
    let nodes = vec![
        FlatNode::Signal { name: "src".to_string(), signed: false },
        FlatNode::Signal { name: "dst".to_string(), signed: false },
    ];
    let constraints = vec![
        // No Fixed constraint for node 0 — it stays at 0.
        WidthConstraint::SameAs { node: 1, source: 0 },
    ];
    let result = solve(&nodes, &constraints);
    assert_eq!(result.widths[1], Width(0), "SameAs from unresolved source should propagate 0");
    // Both should get E503 for unresolved width.
    let e503_count =
        result.diagnostics.iter().filter(|d| d.code.as_deref() == Some("E503")).count();
    assert_eq!(e503_count, 2, "both unresolved nodes should emit E503");
}

#[test]
fn width_min_bits_for_boundary_values() {
    // Verify Width::min_bits_for at key boundaries.
    assert_eq!(Width::min_bits_for(0), Width(1), "min_bits_for(0) should be 1");
    assert_eq!(Width::min_bits_for(1), Width(1), "min_bits_for(1) should be 1");
    assert_eq!(Width::min_bits_for(2), Width(2), "min_bits_for(2) should be 2");
    assert_eq!(Width::min_bits_for(3), Width(2), "min_bits_for(3) should be 2");
    assert_eq!(Width::min_bits_for(255), Width(8), "min_bits_for(255) should be 8");
    assert_eq!(Width::min_bits_for(256), Width(9), "min_bits_for(256) should be 9");
    assert_eq!(Width::min_bits_for(u64::MAX), Width(64), "min_bits_for(u64::MAX) should be 64");
}

#[test]
fn width_display_with_sign_unsigned() {
    let w = Width(16);
    assert_eq!(w.display_with_sign(false), "u16", "unsigned display should use 'u' prefix");
}

#[test]
fn width_display_with_sign_signed() {
    let w = Width(32);
    assert_eq!(w.display_with_sign(true), "i32", "signed display should use 'i' prefix");
}

#[test]
fn width_display_trait() {
    let w = Width(8);
    assert_eq!(format!("{}", w), "u8", "Width Display trait should format as 'u8'");
}

#[test]
fn constraint_generation_for_literals() {
    // Verify constraint generation from flat nodes (integration between
    // generate_constraints and solve).
    let nodes = vec![
        FlatNode::Literal { value: 0 },
        FlatNode::Literal { value: 255 },
        FlatNode::Literal { value: 1024 },
    ];
    let signals: Vec<SignalDecl> = vec![];
    let cset = generate_constraints(&nodes, &signal_map(&signals));
    assert_eq!(cset.constraints.len(), 3, "each literal should generate one Fixed constraint");
    // Verify the widths in the generated constraints.
    let widths: Vec<u32> = cset
        .constraints
        .iter()
        .filter_map(|c| match c {
            WidthConstraint::Fixed { width, .. } => Some(*width),
            _ => None,
        })
        .collect();
    assert_eq!(widths[0], 1, "literal 0 should generate width 1");
    assert_eq!(widths[1], 8, "literal 255 should generate width 8");
    assert_eq!(widths[2], 11, "literal 1024 should generate width 11");
}

#[test]
fn constraint_generation_for_signal_with_declaration() {
    let nodes = vec![FlatNode::Signal { name: "x".to_string(), signed: false }];
    let signals = vec![make_signal("x", 16)];
    let cset = generate_constraints(&nodes, &signal_map(&signals));
    assert_eq!(cset.constraints.len(), 1, "one signal => one constraint");
    match &cset.constraints[0] {
        WidthConstraint::Fixed { node, width } => {
            assert_eq!(*node, 0, "constraint should target node 0");
            assert_eq!(*width, 16, "declared u16 should produce Fixed width 16");
        }
        other => panic!("expected Fixed constraint for declared signal, got {:?}", other),
    }
}

#[test]
fn constraint_generation_for_undeclared_signal_emits_e501() {
    let nodes = vec![FlatNode::Signal { name: "missing".to_string(), signed: false }];
    let signals: Vec<SignalDecl> = vec![];
    let cset = generate_constraints(&nodes, &signal_map(&signals));
    assert!(has_diag_containing(&cset.diagnostics, "E501"), "undeclared signal should emit E501");
    // Should still produce a fallback constraint (width 1) for solver continuity.
    assert_eq!(
        cset.constraints.len(),
        1,
        "undeclared signal should still get a fallback constraint"
    );
}

#[test]
fn constraint_generation_comparison_produces_boolean() {
    let nodes = vec![
        FlatNode::Signal { name: "a".to_string(), signed: false },
        FlatNode::Signal { name: "b".to_string(), signed: false },
        FlatNode::Binary { op: BinaryOp::Lt, left: 0, right: 1 },
    ];
    let signals = vec![make_signal("a", 8), make_signal("b", 8)];
    let cset = generate_constraints(&nodes, &signal_map(&signals));
    // The third constraint (for node 2) should be Boolean.
    let boolean_constraints: Vec<&WidthConstraint> =
        cset.constraints.iter().filter(|c| matches!(c, WidthConstraint::Boolean { .. })).collect();
    assert_eq!(
        boolean_constraints.len(),
        1,
        "comparison operator should produce exactly one Boolean constraint"
    );
}

#[test]
fn solve_result_rounds_field_populated() {
    let nodes = vec![FlatNode::Literal { value: 42 }];
    let constraints = vec![WidthConstraint::Fixed { node: 0, width: 6 }];
    let result = solve(&nodes, &constraints);
    assert!(result.rounds >= 1, "rounds should be at least 1");
    assert!(result.rounds <= 16, "rounds should not exceed MAX_PROPAGATION_ROUNDS (16)");
}

#[test]
fn nonexpansive_scc_large_cluster_converges() {
    // Build an SCC with several signals of varying widths.
    // All should converge to the max declared width.
    let widths_list = [4u32, 8, 12, 16, 20, 24, 28, 32];
    let mut signal_indices = Vec::new();
    let mut signals = Vec::new();
    for (i, &w) in widths_list.iter().enumerate() {
        if i >= MAX_TEST_SCC_SIGNALS {
            break;
        }
        signal_indices.push(i);
        signals.push(make_signal(&format!("s{}", i), w));
    }
    let scc = SccInfo { signal_indices, kind: SccKind::Nonexpansive };
    let result = solve_nonexpansive(&scc, &signals);
    let expected_max = 32u32;
    for i in 0..widths_list.len().min(MAX_TEST_SCC_SIGNALS) {
        assert_eq!(
            result.widths[i], expected_max,
            "signal s{} should converge to max width {} but got {}",
            i, expected_max, result.widths[i]
        );
    }
}

#[test]
fn truncation_check_zero_width_target() {
    // Edge case: target width 0, expr width 1.
    let diags = check_truncation("zero_target", 0, Width(1), false);
    assert_eq!(diags.len(), 1, "assigning to a 0-width target should still detect truncation");
}

#[test]
fn truncation_check_zero_expr_width() {
    // Edge case: expr width 0, target width 8. No truncation.
    let diags = check_truncation("normal_target", 8, Width(0), false);
    assert!(diags.is_empty(), "zero expr width should not trigger truncation");
}

#[test]
fn scc_info_kind_equality() {
    assert_eq!(
        SccKind::Nonexpansive,
        SccKind::Nonexpansive,
        "SccKind::Nonexpansive should equal itself"
    );
    assert_eq!(SccKind::Expansive, SccKind::Expansive, "SccKind::Expansive should equal itself");
    assert_ne!(
        SccKind::Nonexpansive,
        SccKind::Expansive,
        "different SCC kinds should not be equal"
    );
}

#[test]
fn diag_severity_variants_distinct() {
    assert_ne!(DiagSeverity::Error, DiagSeverity::Warning, "Error and Warning should be distinct");
    assert_ne!(DiagSeverity::Error, DiagSeverity::Info, "Error and Info should be distinct");
    assert_ne!(DiagSeverity::Warning, DiagSeverity::Info, "Warning and Info should be distinct");
}

#[test]
fn width_diag_builder_chain() {
    let d = WidthDiag::error("test").with_code("E999").with_signal("sig_a").with_help("try this");
    assert_eq!(d.severity, DiagSeverity::Error, "severity should be Error");
    assert_eq!(d.code.as_deref(), Some("E999"), "code should be E999");
    assert_eq!(d.signal_name.as_deref(), Some("sig_a"), "signal_name should be sig_a");
    assert_eq!(d.help.as_deref(), Some("try this"), "help should be set");
}

#[test]
fn width_diag_display_with_code() {
    let d = WidthDiag::error("overflow detected").with_code("E504");
    let formatted = format!("{}", d);
    assert_eq!(
        formatted, "[width:error E504] overflow detected",
        "display with code should include code"
    );
}

#[test]
fn width_diag_display_without_code() {
    let d = WidthDiag::info("note about something");
    let formatted = format!("{}", d);
    assert_eq!(
        formatted, "[width:info] note about something",
        "display without code should omit code field"
    );
}

#[test]
fn max_flat_nodes_constant_is_512() {
    assert_eq!(MAX_FLAT_NODES, 512, "MAX_FLAT_NODES should be 512 per design");
}

#[test]
fn width_max_is_64() {
    assert_eq!(Width::MAX, Width(64), "Width::MAX should be 64");
}
