// #![forbid(unsafe_code)]
// #![deny(warnings)]
//
// use mirrc::ast::types::BinaryOp;
// use mirrc::width::constraint::WidthConstraint;
// use mirrc::width::solver::solve;
// use mirrc::width::types::{FlatNode, Width};
//
// #[test]
// fn test_width_solver_stress_high_density_independent_channels() {
//     // 1. Arrange: Flood the solver with 150 independent 3-level addition channels.
//     // Each channel consists of:
//     //   sig_a (Fixed 8), sig_b (Fixed 8), sig_c (Fixed 8), sig_d (Fixed 8)
//     //   node_ab = sig_a + sig_b (MaxPlusOne => 9)
//     //   node_cd = sig_c + sig_d (MaxPlusOne => 9)
//     //   node_abcd = node_ab + node_cd (MaxPlusOne => 10)
//     //
//     // Total nodes = 150 * 7 = 1050 flat nodes.
//     // Let's cap at MAX_FLAT_NODES (512) -> we will spawn 70 channels => 490 nodes.
//     let channels = 70;
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     for i in 0..channels {
//         let base = i * 7;
//
//         // Leaf inputs
//         nodes.push(FlatNode::Signal { name: format!("ch{}_a", i), signed: false });
//         nodes.push(FlatNode::Signal { name: format!("ch{}_b", i), signed: false });
//         nodes.push(FlatNode::Signal { name: format!("ch{}_c", i), signed: false });
//         nodes.push(FlatNode::Signal { name: format!("ch{}_d", i), signed: false });
//
//         // Intermediate node ab
//         nodes.push(FlatNode::Binary { op: BinaryOp::Add, left: base, right: base + 1 });
//         // Intermediate node cd
//         nodes.push(FlatNode::Binary { op: BinaryOp::Add, left: base + 2, right: base + 3 });
//         // Final node abcd
//         nodes.push(FlatNode::Binary { op: BinaryOp::Add, left: base + 4, right: base + 5 });
//
//         // Leaf constraints (Fixed u8)
//         constraints.push(WidthConstraint::Fixed { node: base, width: 8 });
//         constraints.push(WidthConstraint::Fixed { node: base + 1, width: 8 });
//         constraints.push(WidthConstraint::Fixed { node: base + 2, width: 8 });
//         constraints.push(WidthConstraint::Fixed { node: base + 3, width: 8 });
//
//         // Binary constraints
//         constraints.push(WidthConstraint::MaxPlusOne {
//             node: base + 4,
//             left: base,
//             right: base + 1,
//         });
//         constraints.push(WidthConstraint::MaxPlusOne {
//             node: base + 5,
//             left: base + 2,
//             right: base + 3,
//         });
//         constraints.push(WidthConstraint::MaxPlusOne {
//             node: base + 6,
//             left: base + 4,
//             right: base + 5,
//         });
//     }
//
//     // 2. Act
//     let start = std::time::Instant::now();
//     let result = solve(&nodes, &constraints);
//     let duration = start.elapsed();
//
//     println!("Solved 70-channel tree (490 nodes, 490 constraints) in {:?}", duration);
//
//     // 3. Assert
//     assert!(duration.as_millis() < 50, "Solver took too long: {:?}", duration);
//     assert_eq!(
//         result.diagnostics.len(),
//         0,
//         "Expected zero diagnostics, got: {:?}",
//         result.diagnostics
//     );
//
//     // Convergence must occur in at most 4 rounds because depth is 3.
//     assert!(result.rounds <= 4, "Expected convergence in <= 4 rounds, got {}", result.rounds);
//
//     // Check final leaf values
//     for i in 0..channels {
//         let base = i * 7;
//         assert_eq!(result.widths[base as usize], Width(8));
//         assert_eq!(result.widths[(base + 4) as usize], Width(9));
//         assert_eq!(result.widths[(base + 5) as usize], Width(9));
//         assert_eq!(result.widths[(base + 6) as usize], Width(10));
//     }
// }
//
// #[test]
// fn test_width_solver_stress_pathological_reversed_chain_budget_exhaustion() {
//     // 1. Arrange: Create a pathologically long chain of MaxPlusOne additions:
//     //   node_{i} = node_{i-1} + 1
//     //
//     // However, we insert constraints in REVERSED order:
//     //   node_100 = node_99 + 1
//     //   node_99 = node_98 + 1
//     //   ...
//     //   node_1 = node_0 + 1
//     //
//     // In this reversed order, each round of propagation can only resolve 1 more level.
//     // Since MAX_PROPAGATION_ROUNDS = 16:
//     // - Round 1 resolves node_1 to 2
//     // - Round 2 resolves node_2 to 3
//     // ...
//     // - Round 16 resolves node_16 to 17
//     // - All nodes from node_17 onwards will remain unresolved (width 0), causing E503.
//     let count = 40;
//     let mut nodes = Vec::new();
//     let mut constraints = Vec::new();
//
//     // Node 0 is our seed anchor
//     nodes.push(FlatNode::Signal { name: "node_0".to_string(), signed: false });
//     constraints.push(WidthConstraint::Fixed { node: 0, width: 8 });
//
//     for i in 1..count {
//         nodes.push(FlatNode::Signal { name: format!("node_{}", i), signed: false });
//     }
//
//     // Add constraints in reversed order to force round-by-round lazy propagation
//     for i in (1..count).rev() {
//         constraints.push(WidthConstraint::SameAs { node: i, source: i - 1 });
//     }
//
//     // 2. Act
//     let result = solve(&nodes, &constraints);
//
//     // 3. Assert
//     assert_eq!(
//         result.rounds, 16,
//         "Reversed propagation chain must execute exactly the budget limit of 16 rounds"
//     );
//
//     // We expect unresolved nodes (from index 16 to count-1) to produce E503 diagnostics
//     assert!(
//         !result.diagnostics.is_empty(),
//         "Expected unresolved diagnostics after budget exhaustion"
//     );
//     let e503_count =
//         result.diagnostics.iter().filter(|d| d.code.as_deref() == Some("E503")).count();
//     assert!(e503_count > 0, "Expected at least one E503 unresolved node error");
//
//     // Verify specific nodes that converged vs stayed zero
//     assert_eq!(result.widths[0], Width(8));
//     assert_eq!(result.widths[1], Width(8));
//     assert_eq!(result.widths[15], Width(8));
//     assert_eq!(result.widths[16], Width(8));
//     assert_eq!(
//         result.widths[17],
//         Width(0),
//         "Node 17 should be left unresolved (0) due to budget limit"
//     );
// }
