// //! TYPE-003: Signed-aware width inference tests.
// //!
// //! Verifies:
// //! 1. Negate width: unsigned N → N+1, signed N → N (SameAsPlusOne vs SameAs)
// //! 2. Signed subtraction suppresses underflow warning
// //! 3. Signed truncation messages use "signed" category
// //! 4. TypeMap is returned from typecheck_module
// //! 5. display_with_sign renders i16 vs u16
// //! 6. FlatNode::Signal carries signed flag
//
// #![forbid(unsafe_code)]
// #![deny(warnings)]
//
// extern crate mirrc;
//
// use mirrc::ast::expr::Expr;
// use mirrc::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
// use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType, UnaryOp};
// use mirrc::pipeline::{run_pipeline, PipelineConfig};
// use mirrc::width;
//
// // ───────────────────── helpers ─────────────────────
//
// fn sig(name: &str, ty: SignalType) -> SignalDecl {
//     SignalDecl {
//         name: name.to_string(),
//         kind: SignalKind::Internal,
//         ty: ExtendedType::from_core(ty),
//         origin: None,
//         span: None,
//     }
// }
//
// fn signal(name: &str) -> Expr {
//     Expr::Signal(name.to_string())
// }
//
// fn int_lit(v: u64) -> Expr {
//     Expr::Literal(LiteralValue::Integer(v))
// }
//
// fn negate(e: Expr) -> Expr {
//     Expr::Unary { op: UnaryOp::Negate, operand: Box::new(e) }
// }
//
// fn sub(left: Expr, right: Expr) -> Expr {
//     Expr::Binary { op: BinaryOp::Sub, left: Box::new(left), right: Box::new(right) }
// }
//
// fn signal_map(signals: &[SignalDecl]) -> std::collections::HashMap<String, u32> {
//     signals.iter().map(|s| (s.name.clone(), s.ty.signal_type().width())).collect()
// }
//
// fn add(left: Expr, right: Expr) -> Expr {
//     Expr::Binary { op: BinaryOp::Add, left: Box::new(left), right: Box::new(right) }
// }
//
// // ───────────────────── Width::display_with_sign ─────────────────────
//
// #[test]
// fn display_with_sign_unsigned() {
//     let w = width::types::Width(16);
//     assert_eq!(w.display_with_sign(false), "u16");
// }
//
// #[test]
// fn display_with_sign_signed() {
//     let w = width::types::Width(16);
//     assert_eq!(w.display_with_sign(true), "i16");
// }
//
// #[test]
// fn display_with_sign_one_bit() {
//     let w = width::types::Width(1);
//     assert_eq!(w.display_with_sign(true), "i1");
//     assert_eq!(w.display_with_sign(false), "u1");
// }
//
// // ───────────────────── FlatNode signed flag ─────────────────────
//
// #[test]
// fn flat_node_signal_carries_signed_flag() {
//     let signals = vec![sig("x", SignalType::Signed(8)), sig("y", SignalType::Unsigned(8))];
//     let expr = signal("x");
//     let nodes = width::flatten::flatten_expr(&expr, &signals).unwrap();
//     assert_eq!(nodes.len(), 1);
//     match &nodes[0] {
//         width::types::FlatNode::Signal { name, signed } => {
//             assert_eq!(name, "x");
//             assert!(*signed, "signal 'x' should be marked signed");
//         }
//         other => panic!("expected FlatNode::Signal, got {:?}", other),
//     }
// }
//
// #[test]
// fn flat_node_unsigned_signal_not_signed() {
//     let signals = vec![sig("y", SignalType::Unsigned(8))];
//     let expr = signal("y");
//     let nodes = width::flatten::flatten_expr(&expr, &signals).unwrap();
//     match &nodes[0] {
//         width::types::FlatNode::Signal { name, signed } => {
//             assert_eq!(name, "y");
//             assert!(!*signed, "signal 'y' should NOT be marked signed");
//         }
//         other => panic!("expected FlatNode::Signal, got {:?}", other),
//     }
// }
//
// #[test]
// fn flat_node_prev_carries_signed_flag() {
//     let signals = vec![sig("s", SignalType::Signed(16))];
//     let expr = Expr::Prev { signal: "s".to_string(), delay: 1 };
//     let nodes = width::flatten::flatten_expr(&expr, &signals).unwrap();
//     match &nodes[0] {
//         width::types::FlatNode::Prev { signal, signed, .. } => {
//             assert_eq!(signal, "s");
//             assert!(*signed, "prev('s') should be marked signed");
//         }
//         other => panic!("expected FlatNode::Prev, got {:?}", other),
//     }
// }
//
// // ───────────────────── Negate width: unsigned → +1, signed → same ─────────────────────
//
// #[test]
// fn negate_unsigned_signal_width_plus_one() {
//     // -x where x : u8 → result should be 9 bits (SameAsPlusOne)
//     let signals = vec![sig("x", SignalType::Unsigned(8))];
//     let expr = negate(signal("x"));
//     let result = width::infer_widths(&expr, &signals);
//     let w = result.expr.unwrap().width();
//     assert_eq!(w.0, 9, "negate of u8 should produce 9-bit result");
// }
//
// #[test]
// fn negate_signed_signal_width_same() {
//     // -x where x : i8 → result should still be 8 bits (SameAs)
//     let signals = vec![sig("x", SignalType::Signed(8))];
//     let expr = negate(signal("x"));
//     let result = width::infer_widths(&expr, &signals);
//     let w = result.expr.unwrap().width();
//     assert_eq!(w.0, 8, "negate of i8 should preserve 8-bit width");
// }
//
// #[test]
// fn negate_unsigned_literal_width_plus_one() {
//     // -42 → literal 42 needs 6 bits, negate → 7 bits
//     let signals = vec![];
//     let expr = negate(int_lit(42));
//     let result = width::infer_widths(&expr, &signals);
//     let w = result.expr.unwrap().width();
//     // Literal 42 = 6 bits. Negate of an unsigned literal → +1 = 7.
//     assert_eq!(w.0, 7, "negate of literal 42 (u6) should produce 7-bit result");
// }
//
// // ───────────────────── Signed subtraction: no underflow warning ─────────────────────
//
// #[test]
// fn signed_subtraction_no_underflow_warning() {
//     // i8 - i8 → should NOT emit "unsigned subtraction may underflow"
//     let signals = vec![sig("a", SignalType::Signed(8)), sig("b", SignalType::Signed(8))];
//     let expr = sub(signal("a"), signal("b"));
//     let result = width::infer_widths(&expr, &signals);
//     for d in &result.diagnostics {
//         assert!(
//             !d.message.contains("unsigned subtraction"),
//             "signed subtraction should NOT produce unsigned underflow warning, got: {}",
//             d.message
//         );
//     }
// }
//
// #[test]
// fn unsigned_subtraction_still_warns() {
//     // u8 - u8 → should still emit "unsigned subtraction may underflow"
//     let signals = vec![sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
//     let expr = sub(signal("a"), signal("b"));
//     let result = width::infer_widths(&expr, &signals);
//     let has_underflow_warning =
//         result.diagnostics.iter().any(|d| d.message.contains("unsigned subtraction"));
//     assert!(has_underflow_warning, "unsigned subtraction should still warn about underflow");
// }
//
// // ───────────────────── Signed truncation message ─────────────────────
//
// #[test]
// fn signed_truncation_message_uses_signed_category() {
//     // Assign a 16-bit expression to an i8 target → truncation diagnostic
//     // should say "signed"
//     let target_w = 8u32;
//     let expr_w = width::types::Width(16);
//     let diags = width::check_truncation("out", target_w, expr_w, true);
//     assert_eq!(diags.len(), 1);
//     assert!(
//         diags[0].message.contains("signed"),
//         "truncation message should contain 'signed', got: {}",
//         diags[0].message
//     );
// }
//
// #[test]
// fn unsigned_truncation_message_uses_unsigned_category() {
//     let target_w = 8u32;
//     let expr_w = width::types::Width(16);
//     let diags = width::check_truncation("out", target_w, expr_w, false);
//     assert_eq!(diags.len(), 1);
//     assert!(
//         diags[0].message.contains("unsigned"),
//         "truncation message should contain 'unsigned', got: {}",
//         diags[0].message
//     );
// }
//
// // ───────────────────── TypeMap returned by typecheck_module ─────────────────────
//
// #[test]
// fn typecheck_module_returns_type_map() {
//     let module = Module {
//         name: "tm".to_string(),
//         signals: vec![
//             sig("a", SignalType::Unsigned(8)),
//             sig("b", SignalType::Unsigned(8)),
//             sig("out", SignalType::Unsigned(8)),
//         ],
//         guards: vec![Guard {
//             name: "g".to_string(),
//             condition: Expr::Literal(LiteralValue::Bool(true)),
//             cycles: 1,
//             template_cycles: None,
//             origin: None,
//             span: None,
//         }],
//         reflexes: vec![Reflex {
//             name: "r".to_string(),
//             guard_names: vec!["g".to_string()],
//             assignments: vec![Assignment {
//                 target: "out".to_string(),
//                 value: add(signal("a"), signal("b")),
//                 span: None,
//             }],
//             origin: None,
//             span: None,
//         }],
//         properties: vec![],
//         pattern_calls: vec![],
//         pattern_origins: vec![],
//         span: None,
//     };
//     let type_map = mirrc::typeck::typecheck_module(&module).unwrap();
//     // Should have entries for: literal true, signal a, signal b, a+b
//     assert!(type_map.len() >= 3, "type_map should have at least 3 entries, got {}", type_map.len());
// }
//
// // ───────────────────── Pipeline threads type_map ─────────────────────
//
// #[test]
// fn pipeline_result_has_type_map() {
//     let source = r#"
//         module test_mod {
//             signal x: in u8;
//             signal y: in u8;
//             signal out: out u8;
//             guard g {
//                 when x < y
//                 for 1 cycles;
//             }
//             reflex r {
//                 on g {
//                     out = y;
//                 }
//             }
//         }
//     "#;
//     let config = PipelineConfig {
//         typecheck: true,
//         simplify: true,
//         width: true,
//         temporal: false,
//         rspu: false,
//         extended_typecheck: false,
//         simulate: false,
//         mape_k: false,
//         ..PipelineConfig::default()
//     };
//     let result = run_pipeline(source, &config).unwrap();
//     assert!(result.type_map.is_some(), "type_map should be Some when typecheck is enabled");
// }
//
// #[test]
// fn pipeline_result_no_type_map_when_skipped() {
//     let source = r#"
//         module test_mod {
//             signal x: in u8;
//             signal out: out u8;
//             guard g {
//                 when x < x
//                 for 1 cycles;
//             }
//             reflex r {
//                 on g {
//                     out = 1;
//                 }
//             }
//         }
//     "#;
//     let config = PipelineConfig {
//         typecheck: false,
//         simplify: false,
//         width: false,
//         temporal: false,
//         rspu: false,
//         extended_typecheck: false,
//         simulate: false,
//         mape_k: false,
//         ..PipelineConfig::default()
//     };
//     let result = run_pipeline(source, &config).unwrap();
//     assert!(result.type_map.is_none(), "type_map should be None when typecheck is disabled");
// }
//
// // ───────────────────── E2E: signed signal through full pipeline ─────────────────────
//
// #[test]
// fn signed_signal_e2e_pipeline() {
//     let source = r#"
//         module signed_e2e {
//             signal a: in i16;
//             signal b: in i16;
//             signal out: out i16;
//             guard g {
//                 when a < b
//                 for 1 cycles;
//             }
//             reflex r {
//                 on g {
//                     out = a;
//                 }
//             }
//         }
//     "#;
//     let config = PipelineConfig {
//         typecheck: true,
//         simplify: true,
//         width: true,
//         temporal: false,
//         rspu: false,
//         extended_typecheck: false,
//         simulate: false,
//         mape_k: false,
//         ..PipelineConfig::default()
//     };
//     let result = run_pipeline(source, &config).unwrap();
//     assert!(result.type_map.is_some());
//     assert!(!result.has_width_errors());
// }
//
// #[test]
// fn negate_unsigned_e2e_pipeline() {
//     // Negate u8 → result is i9 (9 bits), assign to i16 (ok, widening)
//     let source = r#"
//         module neg_e2e {
//             signal x: in u8;
//             signal y: in u8;
//             signal out: out i16;
//             guard g {
//                 when x < y
//                 for 1 cycles;
//             }
//             reflex r {
//                 on g {
//                     out = -x;
//                 }
//             }
//         }
//     "#;
//     let config = PipelineConfig {
//         typecheck: true,
//         simplify: true,
//         width: true,
//         temporal: false,
//         rspu: false,
//         extended_typecheck: false,
//         simulate: false,
//         mape_k: false,
//         ..PipelineConfig::default()
//     };
//     let result = run_pipeline(source, &config).unwrap();
//     assert!(!result.has_width_errors(), "negating u8 into i16 should not produce width errors");
// }
//
// // ───────────────────── SameAsPlusOne constraint variant ─────────────────────
//
// #[test]
// fn same_as_plus_one_constraint_generated_for_unsigned_negate() {
//     let signals = vec![sig("x", SignalType::Unsigned(8))];
//     let expr = negate(signal("x"));
//     let nodes = width::flatten::flatten_expr(&expr, &signals).unwrap();
//     let cset = width::constraint::generate_constraints(&nodes, &signal_map(&signals));
//     // Should have a SameAsPlusOne constraint (not SameAs) for the negate node
//     let has_plus_one = cset
//         .constraints
//         .iter()
//         .any(|c| matches!(c, width::constraint::WidthConstraint::SameAsPlusOne { .. }));
//     assert!(has_plus_one, "unsigned negate should generate SameAsPlusOne constraint");
// }
//
// #[test]
// fn same_as_constraint_generated_for_signed_negate() {
//     let signals = vec![sig("x", SignalType::Signed(8))];
//     let expr = negate(signal("x"));
//     let nodes = width::flatten::flatten_expr(&expr, &signals).unwrap();
//     let cset = width::constraint::generate_constraints(&nodes, &signal_map(&signals));
//     // Should have a SameAs constraint (not SameAsPlusOne) for the negate node
//     let has_same_as = cset
//         .constraints
//         .iter()
//         .any(|c| matches!(c, width::constraint::WidthConstraint::SameAs { .. }));
//     let has_plus_one = cset
//         .constraints
//         .iter()
//         .any(|c| matches!(c, width::constraint::WidthConstraint::SameAsPlusOne { .. }));
//     assert!(has_same_as, "signed negate should generate SameAs constraint");
//     assert!(!has_plus_one, "signed negate should NOT generate SameAsPlusOne constraint");
// }
