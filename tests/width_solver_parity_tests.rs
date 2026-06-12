// #![forbid(unsafe_code)]
// #![deny(warnings)]
//
// //! Integration test suite implementing Phase A (Width Solver Parity)
// //! of the massive test suite expansion plan.
// //!
// //! Validates:
// //! 1. Parity of width inference outcomes across various topological structures.
// //! 2. Correct bit-width propagation bounds for arithmetic, logical, and SCC operations.
// //! 3. Deterministic convergence under pathologically ordered constraints.
//
// use mirrc::ast::types::BinaryOp;
// use mirrc::width::constraint::WidthConstraint;
// use mirrc::width::solver::solve;
// use mirrc::width::types::{FlatNode, Width};
//
// /// Verify arithmetic addition maximum plus one constraint propagation.
// #[test]
// fn test_parity_addition_propagation() {
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     // Spawn two leaf signals
//     nodes.push(FlatNode::Signal { name: "sig_a".to_string(), signed: false });
//     nodes.push(FlatNode::Signal { name: "sig_b".to_string(), signed: false });
//
//     // Binary Add node
//     nodes.push(FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 1 });
//
//     // Fixed widths for inputs (u8 and u12)
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 8 });
//     constraints.push(WidthConstraint::Fixed { node: 1, width: 12 });
//
//     // MaxPlusOne constraint for Add node
//     constraints.push(WidthConstraint::MaxPlusOne { node: 2, left: 0, right: 1 });
//
//     let result = solve(&nodes, &constraints);
//
//     assert_eq!(result.diagnostics.len(), 0);
//     assert_eq!(result.widths[0], Width(8));
//     assert_eq!(result.widths[1], Width(12));
//     assert_eq!(result.widths[2], Width(13)); // max(8, 12) + 1 = 13
// }
//
// /// Verify division/subtraction width extraction (result width is SameAs source).
// #[test]
// fn test_parity_subtraction_propagation() {
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     nodes.push(FlatNode::Signal { name: "numerator".to_string(), signed: false });
//     nodes.push(FlatNode::Signal { name: "denominator".to_string(), signed: false });
//     nodes.push(FlatNode::Binary { op: BinaryOp::Sub, left: 0, right: 1 });
//
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 16 });
//     constraints.push(WidthConstraint::Fixed { node: 1, width: 4 });
//     constraints.push(WidthConstraint::SameAs { node: 2, source: 0 }); // Result SameAs numerator width
//
//     let result = solve(&nodes, &constraints);
//
//     assert_eq!(result.diagnostics.len(), 0);
//     assert_eq!(result.widths[2], Width(16));
// }
//
// /// Verify that logical AND/OR signals establish mutual SameAs constraints.
// #[test]
// fn test_parity_logical_equality() {
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     nodes.push(FlatNode::Signal { name: "a".to_string(), signed: false });
//     nodes.push(FlatNode::Signal { name: "b".to_string(), signed: false });
//     nodes.push(FlatNode::Binary { op: BinaryOp::And, left: 0, right: 1 });
//
//     // a is u8, b is unresolved, and a AND b establishes SameAs
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 8 });
//     constraints.push(WidthConstraint::SameAs { node: 1, source: 0 });
//     constraints.push(WidthConstraint::SameAs { node: 2, source: 0 });
//
//     let result = solve(&nodes, &constraints);
//
//     assert_eq!(result.diagnostics.len(), 0);
//     assert_eq!(result.widths[1], Width(8)); // b should infer u8
//     assert_eq!(result.widths[2], Width(8)); // result should infer u8
// }
//
// /// Verify Strongly Connected Component (SCC) cyclic loops resolve within convergence limits.
// #[test]
// fn test_parity_scc_feedback_convergence() {
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     // Cyclic feedback loop: node_0 relies on node_1, node_1 relies on node_0
//     nodes.push(FlatNode::Signal { name: "node_0".to_string(), signed: false });
//     nodes.push(FlatNode::Signal { name: "node_1".to_string(), signed: false });
//
//     // Seed node_0 with base u4 fixed width
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 4 });
//     constraints.push(WidthConstraint::SameAs { node: 1, source: 0 });
//     constraints.push(WidthConstraint::SameAs { node: 0, source: 1 });
//
//     let result = solve(&nodes, &constraints);
//
//     assert_eq!(result.diagnostics.len(), 0);
//     assert_eq!(result.widths[0], Width(4));
//     assert_eq!(result.widths[1], Width(4));
//     assert!(result.rounds <= 3); // Cyclic loop resolves instantly
// }
//
// /// Verify mixed sign constraints propagate correct width attributes.
// #[test]
// fn test_parity_mixed_sign_propagation() {
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     nodes.push(FlatNode::Signal { name: "sig_signed".to_string(), signed: true });
//     nodes.push(FlatNode::Signal { name: "sig_unsigned".to_string(), signed: false });
//     nodes.push(FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 1 });
//
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 16 });
//     constraints.push(WidthConstraint::Fixed { node: 1, width: 8 });
//     constraints.push(WidthConstraint::MaxPlusOne { node: 2, left: 0, right: 1 });
//
//     let result = solve(&nodes, &constraints);
//
//     assert_eq!(result.diagnostics.len(), 0);
//     assert_eq!(result.widths[2], Width(17)); // max(16, 8) + 1 = 17
// }
//
// /// Verify large chain of additions behaves deterministically under solver execution.
// #[test]
// fn test_parity_long_addition_chain() {
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     let count = 10;
//     nodes.push(FlatNode::Signal { name: "seed".to_string(), signed: false });
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 8 });
//
//     for i in 1..=count {
//         nodes.push(FlatNode::Binary {
//             op: BinaryOp::Add,
//             left: (i - 1) as u32,
//             right: (i - 1) as u32,
//         });
//         constraints.push(WidthConstraint::MaxPlusOne {
//             node: i as u32,
//             left: (i - 1) as u32,
//             right: (i - 1) as u32,
//         });
//     }
//
//     let result = solve(&nodes, &constraints);
//
//     assert_eq!(result.diagnostics.len(), 0);
//     assert_eq!(result.widths[count as usize], Width(18)); // 8 + 10 rounds of +1 = 18
// }
//
// /// Verify that unresolved signals emit E503 constraints.
// #[test]
// fn test_parity_unresolved_signal_emits_error() {
//     let mut nodes = Vec::new();
//     let constraints = Vec::new();
//
//     nodes.push(FlatNode::Signal { name: "dangling".to_string(), signed: false });
//     // Left completely unconstrained
//
//     let result = solve(&nodes, &constraints);
//
//     assert!(!result.diagnostics.is_empty());
//     let error_codes: Vec<String> =
//         result.diagnostics.iter().map(|d| d.code.clone().unwrap_or_default()).collect();
//     assert!(error_codes.contains(&"E503".to_string()));
// }
//
// /// Verify width capacity capping at 64-bit boundaries.
// #[test]
// fn test_parity_bit_width_capping() {
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     nodes.push(FlatNode::Signal { name: "large_sig".to_string(), signed: false });
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 8192 });
//
//     nodes.push(FlatNode::Binary { op: BinaryOp::Add, left: 0, right: 0 });
//     constraints.push(WidthConstraint::MaxPlusOne { node: 1, left: 0, right: 0 });
//
//     let result = solve(&nodes, &constraints);
//
//     // Capped at 8192 bits or reports overflow diagnostic E504
//     assert!(!result.diagnostics.is_empty());
//     let error_codes: Vec<String> =
//         result.diagnostics.iter().map(|d| d.code.clone().unwrap_or_default()).collect();
//     assert!(error_codes.contains(&"E504".to_string()));
// }
