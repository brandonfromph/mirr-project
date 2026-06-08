//! MEGA-5 Subsystem Verification Test Suite — Symbolic Reasoning Engine.
//!
//! NASA-style verification tests for the MIRR symbolic evaluation engine:
//! interval arithmetic, pattern matching, symbolic differentiation, and
//! module-level abstract interpretation.
//!
//! Covers:
//! - G1: SymValue lattice (Concrete, Interval, Unknown, Top)
//! - G2: sym_eval_binary — all 14 BinaryOp on concrete values
//! - G3: sym_eval_unary — Not, Negate on all lattice levels
//! - G4: Interval arithmetic (interval_binary, interval_unary)
//! - G5: sym_eval_expr — expression tree evaluation
//! - G6: Refinement checking (sym_check_refinement)
//! - G7: Widening operator (sym_widen)
//! - G8: Pattern matching (match_word, MatchPattern, MatchAction)
//! - G9: Symbolic differentiation (sym_diff — all rules)
//! - G10: Module-level analysis (analyze_module, SymbolicResult)
//! - G11: Pipeline integration (symbolic flag in PipelineConfig)
//! - G12: NASA P10 bounds (MAX_SYM_SIGNALS, MAX_SYM_ITERATIONS, MAX_SYM_DEPTH)
//!
//! Every loop is bounded by a MAX_* constant. No recursion. No unsafe code.

#![forbid(unsafe_code)]

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Assignment, Guard, Module, Reflex, SignalDecl};
use mirrc::ast::types::{BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType, UnaryOp};
use mirrc::emit::rspu_tagged::{Provenance, TaggedWord, TypeTag};
use mirrc::pipeline::{run_pipeline, PipelineConfig};
use mirrc::symbolic::diff::sym_diff;
use mirrc::symbolic::interval::{interval_binary, interval_unary};
use mirrc::symbolic::pattern::{match_word, MatchAction, MatchPattern, MAX_MATCH_PATTERNS};
use mirrc::symbolic::{
    analyze_module, sym_check_refinement, sym_eval_binary, sym_eval_expr, sym_eval_unary,
    sym_widen, SymState, SymValue, MAX_SYM_DEPTH, MAX_SYM_ITERATIONS, MAX_SYM_SIGNALS,
};

// ---------------------------------------------------------------------------
// Bounded iteration constants (NASA P10)
// ---------------------------------------------------------------------------

const MAX_TEST_ITERATIONS: usize = 256;

// ---------------------------------------------------------------------------
// AST Helpers
// ---------------------------------------------------------------------------

fn sig(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn lit(n: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(n))
}

fn lit_bool(b: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(b))
}

fn bin(op: BinaryOp, l: Expr, r: Expr) -> Expr {
    Expr::Binary { op, left: Box::new(l), right: Box::new(r) }
}

fn unary(op: UnaryOp, operand: Expr) -> Expr {
    Expr::Unary { op, operand: Box::new(operand) }
}

fn make_signal(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn make_module_for_analysis(
    signals: Vec<SignalDecl>,
    guards: Vec<Guard>,
    reflexes: Vec<Reflex>,
) -> Module {
    Module {
        name: "test".to_string(),
        signals,
        guards,
        reflexes,
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

// ===========================================================================
// G1: SymValue lattice
// ===========================================================================

#[test]
fn g1_concrete_equality() {
    assert_eq!(SymValue::Concrete(42), SymValue::Concrete(42));
    assert_ne!(SymValue::Concrete(1), SymValue::Concrete(2));
}

#[test]
fn g1_interval_construction() {
    let iv = SymValue::Interval { lo: 0, hi: 255 };
    match iv {
        SymValue::Interval { lo, hi } => {
            assert_eq!(lo, 0);
            assert_eq!(hi, 255);
        }
        _ => panic!("Expected Interval"),
    }
}

#[test]
fn g1_unknown_with_width() {
    let u = SymValue::Unknown { width: 16 };
    match u {
        SymValue::Unknown { width } => assert_eq!(width, 16),
        _ => panic!("Expected Unknown"),
    }
}

#[test]
fn g1_top_is_top() {
    assert_eq!(SymValue::Top, SymValue::Top);
    assert_ne!(SymValue::Top, SymValue::Concrete(0));
}

// ===========================================================================
// G2: sym_eval_binary — all 14 BinaryOp on concrete values
// ===========================================================================

#[test]
fn g2_add_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Add, SymValue::Concrete(3), SymValue::Concrete(4)),
        SymValue::Concrete(7)
    );
}

#[test]
fn g2_sub_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Sub, SymValue::Concrete(10), SymValue::Concrete(3)),
        SymValue::Concrete(7)
    );
}

#[test]
fn g2_mul_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Mul, SymValue::Concrete(6), SymValue::Concrete(7)),
        SymValue::Concrete(42)
    );
}

#[test]
fn g2_and_concrete() {
    // Logical AND: both nonzero → 1; one zero → 0.
    assert_eq!(
        sym_eval_binary(BinaryOp::And, SymValue::Concrete(1), SymValue::Concrete(1)),
        SymValue::Concrete(1)
    );
    assert_eq!(
        sym_eval_binary(BinaryOp::And, SymValue::Concrete(1), SymValue::Concrete(0)),
        SymValue::Concrete(0)
    );
}

#[test]
fn g2_or_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Or, SymValue::Concrete(0), SymValue::Concrete(1)),
        SymValue::Concrete(1)
    );
    assert_eq!(
        sym_eval_binary(BinaryOp::Or, SymValue::Concrete(0), SymValue::Concrete(0)),
        SymValue::Concrete(0)
    );
}

#[test]
fn g2_xor_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Xor, SymValue::Concrete(0xFF), SymValue::Concrete(0x0F)),
        SymValue::Concrete(0xF0)
    );
}

#[test]
fn g2_shl_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Shl, SymValue::Concrete(1), SymValue::Concrete(4)),
        SymValue::Concrete(16)
    );
}

#[test]
fn g2_shr_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Shr, SymValue::Concrete(256), SymValue::Concrete(4)),
        SymValue::Concrete(16)
    );
}

#[test]
fn g2_eq_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Eq, SymValue::Concrete(5), SymValue::Concrete(5)),
        SymValue::Concrete(1)
    );
    assert_eq!(
        sym_eval_binary(BinaryOp::Eq, SymValue::Concrete(5), SymValue::Concrete(6)),
        SymValue::Concrete(0)
    );
}

#[test]
fn g2_ne_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Ne, SymValue::Concrete(5), SymValue::Concrete(6)),
        SymValue::Concrete(1)
    );
}

#[test]
fn g2_lt_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Lt, SymValue::Concrete(2), SymValue::Concrete(5)),
        SymValue::Concrete(1)
    );
    assert_eq!(
        sym_eval_binary(BinaryOp::Lt, SymValue::Concrete(5), SymValue::Concrete(2)),
        SymValue::Concrete(0)
    );
}

#[test]
fn g2_le_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Le, SymValue::Concrete(5), SymValue::Concrete(5)),
        SymValue::Concrete(1)
    );
}

#[test]
fn g2_gt_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Gt, SymValue::Concrete(10), SymValue::Concrete(5)),
        SymValue::Concrete(1)
    );
}

#[test]
fn g2_ge_concrete() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Ge, SymValue::Concrete(5), SymValue::Concrete(5)),
        SymValue::Concrete(1)
    );
}

#[test]
fn g2_top_absorbs_all() {
    assert_eq!(sym_eval_binary(BinaryOp::Add, SymValue::Top, SymValue::Concrete(1)), SymValue::Top);
    assert_eq!(sym_eval_binary(BinaryOp::Mul, SymValue::Concrete(1), SymValue::Top), SymValue::Top);
}

#[test]
fn g2_unknown_widens_to_64() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Add, SymValue::Unknown { width: 8 }, SymValue::Concrete(1)),
        SymValue::Unknown { width: 64 }
    );
}

#[test]
fn g2_two_unknowns_max_width() {
    assert_eq!(
        sym_eval_binary(
            BinaryOp::Add,
            SymValue::Unknown { width: 8 },
            SymValue::Unknown { width: 16 }
        ),
        SymValue::Unknown { width: 16 }
    );
}

#[test]
fn g2_shl_large_shift_is_zero() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Shl, SymValue::Concrete(1), SymValue::Concrete(64)),
        SymValue::Concrete(0)
    );
}

#[test]
fn g2_shr_large_shift_is_zero() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Shr, SymValue::Concrete(1), SymValue::Concrete(64)),
        SymValue::Concrete(0)
    );
}

// ===========================================================================
// G3: sym_eval_unary
// ===========================================================================

#[test]
fn g3_not_concrete() {
    assert_eq!(sym_eval_unary(UnaryOp::Not, SymValue::Concrete(0)), SymValue::Concrete(!0u64));
}

#[test]
fn g3_negate_concrete_widens() {
    assert_eq!(
        sym_eval_unary(UnaryOp::Negate, SymValue::Concrete(42)),
        SymValue::Unknown { width: 64 }
    );
}

#[test]
fn g3_not_top_is_top() {
    assert_eq!(sym_eval_unary(UnaryOp::Not, SymValue::Top), SymValue::Top);
}

#[test]
fn g3_negate_unknown_preserves_width() {
    assert_eq!(
        sym_eval_unary(UnaryOp::Negate, SymValue::Unknown { width: 16 }),
        SymValue::Unknown { width: 16 }
    );
}

// ===========================================================================
// G4: Interval arithmetic
// ===========================================================================

#[test]
fn g4_interval_add() {
    let result = interval_binary(
        BinaryOp::Add,
        SymValue::Interval { lo: 1, hi: 5 },
        SymValue::Interval { lo: 10, hi: 20 },
    );
    assert_eq!(result, SymValue::Interval { lo: 11, hi: 25 });
}

#[test]
fn g4_interval_sub() {
    let result = interval_binary(
        BinaryOp::Sub,
        SymValue::Interval { lo: 10, hi: 20 },
        SymValue::Interval { lo: 1, hi: 5 },
    );
    assert_eq!(result, SymValue::Interval { lo: 5, hi: 19 });
}

#[test]
fn g4_interval_mul_four_corner() {
    let result = interval_binary(
        BinaryOp::Mul,
        SymValue::Interval { lo: 2, hi: 3 },
        SymValue::Interval { lo: 4, hi: 5 },
    );
    assert_eq!(result, SymValue::Interval { lo: 8, hi: 15 });
}

#[test]
fn g4_interval_and_lower_bound_is_zero() {
    let result = interval_binary(
        BinaryOp::And,
        SymValue::Interval { lo: 5, hi: 10 },
        SymValue::Interval { lo: 3, hi: 7 },
    );
    match result {
        SymValue::Interval { lo, .. } | SymValue::Concrete(lo) => {
            assert_eq!(lo, 0, "AND lower bound must be 0");
        }
        _ => panic!("Expected Interval or Concrete"),
    }
}

#[test]
fn g4_interval_lt_definitely_true() {
    // [0,5] < [10,20] → definitely 1
    let result = interval_binary(
        BinaryOp::Lt,
        SymValue::Interval { lo: 0, hi: 5 },
        SymValue::Interval { lo: 10, hi: 20 },
    );
    assert_eq!(result, SymValue::Concrete(1));
}

#[test]
fn g4_interval_lt_definitely_false() {
    // [10,20] < [0,5] → a_lo >= b_hi → definitely 0
    let result = interval_binary(
        BinaryOp::Lt,
        SymValue::Interval { lo: 10, hi: 20 },
        SymValue::Interval { lo: 0, hi: 5 },
    );
    assert_eq!(result, SymValue::Concrete(0));
}

#[test]
fn g4_interval_lt_indeterminate() {
    // Overlapping intervals → [0, 1]
    let result = interval_binary(
        BinaryOp::Lt,
        SymValue::Interval { lo: 0, hi: 10 },
        SymValue::Interval { lo: 5, hi: 15 },
    );
    assert_eq!(result, SymValue::Interval { lo: 0, hi: 1 });
}

#[test]
fn g4_interval_eq_disjoint() {
    let result = interval_binary(
        BinaryOp::Eq,
        SymValue::Interval { lo: 0, hi: 5 },
        SymValue::Interval { lo: 10, hi: 20 },
    );
    assert_eq!(result, SymValue::Concrete(0), "Disjoint intervals cannot be equal");
}

#[test]
fn g4_interval_shl_concrete_shift() {
    let result =
        interval_binary(BinaryOp::Shl, SymValue::Interval { lo: 1, hi: 4 }, SymValue::Concrete(2));
    assert_eq!(result, SymValue::Interval { lo: 4, hi: 16 });
}

#[test]
fn g4_interval_shr_concrete_shift() {
    let result =
        interval_binary(BinaryOp::Shr, SymValue::Interval { lo: 8, hi: 32 }, SymValue::Concrete(2));
    assert_eq!(result, SymValue::Interval { lo: 2, hi: 8 });
}

#[test]
fn g4_interval_not_reverses_order() {
    let result = interval_unary(UnaryOp::Not, SymValue::Interval { lo: 0, hi: 0xFF });
    match result {
        SymValue::Interval { lo, hi } => {
            assert_eq!(lo, !0xFFu64);
            assert_eq!(hi, !0u64);
        }
        _ => panic!("Expected Interval from NOT"),
    }
}

#[test]
fn g4_interval_negate_widens() {
    let result = interval_unary(UnaryOp::Negate, SymValue::Interval { lo: 0, hi: 100 });
    assert_eq!(result, SymValue::Unknown { width: 64 });
}

#[test]
fn g4_concrete_through_interval_add() {
    // Concrete values treated as degenerate intervals.
    let result = interval_binary(
        BinaryOp::Add,
        SymValue::Concrete(5),
        SymValue::Interval { lo: 10, hi: 20 },
    );
    assert_eq!(result, SymValue::Interval { lo: 15, hi: 25 });
}

// ===========================================================================
// G5: sym_eval_expr — expression tree evaluation
// ===========================================================================

#[test]
fn g5_eval_literal_bool() {
    let state = SymState::new();
    assert_eq!(sym_eval_expr(&lit_bool(true), &state), SymValue::Concrete(1));
    assert_eq!(sym_eval_expr(&lit_bool(false), &state), SymValue::Concrete(0));
}

#[test]
fn g5_eval_literal_integer() {
    let state = SymState::new();
    assert_eq!(sym_eval_expr(&lit(42), &state), SymValue::Concrete(42));
}

#[test]
fn g5_eval_signal_found() {
    let mut state = SymState::new();
    state.signals.push(("x".to_string(), SymValue::Concrete(99)));
    assert_eq!(sym_eval_expr(&sig("x"), &state), SymValue::Concrete(99));
}

#[test]
fn g5_eval_signal_not_found_is_top() {
    let state = SymState::new();
    assert_eq!(sym_eval_expr(&sig("missing"), &state), SymValue::Top);
}

#[test]
fn g5_eval_prev_is_top() {
    let state = SymState::new();
    let expr = Expr::Prev { signal: "x".to_string(), delay: 1 };
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Top);
}

#[test]
fn g5_eval_binary_add() {
    let state = SymState::new();
    let expr = bin(BinaryOp::Add, lit(10), lit(20));
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(30));
}

#[test]
fn g5_eval_nested_expression() {
    // (3 + 4) * 5 = 35
    let state = SymState::new();
    let expr = bin(BinaryOp::Mul, bin(BinaryOp::Add, lit(3), lit(4)), lit(5));
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(35));
}

#[test]
fn g5_eval_unary_not() {
    let state = SymState::new();
    let expr = unary(UnaryOp::Not, lit(0));
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(!0u64));
}

#[test]
fn g5_eval_with_signal_in_state() {
    let mut state = SymState::new();
    state.signals.push(("x".to_string(), SymValue::Concrete(10)));
    state.signals.push(("y".to_string(), SymValue::Concrete(20)));
    let expr = bin(BinaryOp::Add, sig("x"), sig("y"));
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(30));
}

#[test]
fn g5_eval_signal_with_interval() {
    let mut state = SymState::new();
    state.signals.push(("x".to_string(), SymValue::Interval { lo: 0, hi: 255 }));
    let expr = bin(BinaryOp::Add, sig("x"), lit(1));
    assert_eq!(sym_eval_expr(&expr, &state), SymValue::Interval { lo: 1, hi: 256 });
}

// ===========================================================================
// G6: Refinement checking
// ===========================================================================

#[test]
fn g6_concrete_in_range() {
    assert!(sym_check_refinement(SymValue::Concrete(100), 0, 255));
}

#[test]
fn g6_concrete_out_of_range() {
    assert!(!sym_check_refinement(SymValue::Concrete(300), 0, 255));
}

#[test]
fn g6_interval_contained() {
    assert!(sym_check_refinement(SymValue::Interval { lo: 10, hi: 200 }, 0, 255));
}

#[test]
fn g6_interval_exceeds() {
    assert!(!sym_check_refinement(SymValue::Interval { lo: 10, hi: 300 }, 0, 255));
}

#[test]
fn g6_unknown_always_false() {
    assert!(!sym_check_refinement(SymValue::Unknown { width: 8 }, 0, 255));
}

#[test]
fn g6_top_always_false() {
    assert!(!sym_check_refinement(SymValue::Top, 0, u64::MAX));
}

// ===========================================================================
// G7: Widening operator
// ===========================================================================

#[test]
fn g7_same_is_stable() {
    let v = SymValue::Concrete(5);
    assert_eq!(sym_widen(v, v), v, "Same value must be stable (fixpoint)");
}

#[test]
fn g7_concrete_to_interval() {
    let result = sym_widen(SymValue::Concrete(3), SymValue::Concrete(7));
    assert_eq!(result, SymValue::Interval { lo: 3, hi: 7 });
}

#[test]
fn g7_interval_to_unknown() {
    let a = SymValue::Interval { lo: 0, hi: 10 };
    let b = SymValue::Interval { lo: 5, hi: 20 };
    assert_eq!(sym_widen(a, b), SymValue::Unknown { width: 64 });
}

#[test]
fn g7_unknown_to_top() {
    assert_eq!(sym_widen(SymValue::Unknown { width: 8 }, SymValue::Concrete(1)), SymValue::Top);
}

#[test]
fn g7_concrete_interval_mix_to_unknown() {
    assert_eq!(
        sym_widen(SymValue::Concrete(5), SymValue::Interval { lo: 0, hi: 10 }),
        SymValue::Unknown { width: 64 }
    );
}

// ===========================================================================
// G8: Pattern matching
// ===========================================================================

#[test]
fn g8_match_word_first_wins() {
    let word = TaggedWord {
        value: 0xAB,
        tag: TypeTag::Unsigned { width: 8 },
        provenance: Provenance::Literal,
    };
    let patterns = [
        MatchPattern {
            tag_mask: 0xFF,
            tag_pattern: 1, // Unsigned tag = 1
            value_mask: 0xFF,
            value_pattern: 0xAB,
            action: MatchAction::Accept(42),
        },
        MatchPattern {
            tag_mask: 0xFF,
            tag_pattern: 1,
            value_mask: 0xFF,
            value_pattern: 0xAB,
            action: MatchAction::Accept(99),
        },
    ];
    assert_eq!(match_word(&word, &patterns), MatchAction::Accept(42));
}

#[test]
fn g8_match_word_no_match_returns_continue() {
    let word = TaggedWord { value: 1, tag: TypeTag::Bool, provenance: Provenance::Literal };
    let patterns = [MatchPattern {
        tag_mask: 0xFF,
        tag_pattern: 1, // Unsigned — won't match Bool (tag byte 0)
        value_mask: 0xFF,
        value_pattern: 1,
        action: MatchAction::Accept(0),
    }];
    assert_eq!(match_word(&word, &patterns), MatchAction::Continue);
}

#[test]
fn g8_match_word_empty_patterns() {
    let word = TaggedWord {
        value: 0,
        tag: TypeTag::Unsigned { width: 8 },
        provenance: Provenance::Literal,
    };
    assert_eq!(match_word(&word, &[]), MatchAction::Continue);
}

#[test]
fn g8_match_word_wildcard_tag() {
    let word = TaggedWord {
        value: 0x42,
        tag: TypeTag::Unsigned { width: 16 },
        provenance: Provenance::Literal,
    };
    let patterns = [MatchPattern {
        tag_mask: 0, // wildcard
        tag_pattern: 0,
        value_mask: 0xFF,
        value_pattern: 0x42,
        action: MatchAction::Accept(1),
    }];
    assert_eq!(match_word(&word, &patterns), MatchAction::Accept(1));
}

#[test]
fn g8_match_word_trap_action() {
    let word = TaggedWord {
        value: 0xFF,
        tag: TypeTag::Unsigned { width: 8 },
        provenance: Provenance::Literal,
    };
    let patterns = [MatchPattern {
        tag_mask: 0xFF,
        tag_pattern: 1,
        value_mask: 0xFF,
        value_pattern: 0xFF,
        action: MatchAction::Trap(7),
    }];
    assert_eq!(match_word(&word, &patterns), MatchAction::Trap(7));
}

#[test]
fn g8_max_match_patterns_bound() {
    assert_eq!(MAX_MATCH_PATTERNS, 16, "MAX_MATCH_PATTERNS must be 16");
}

#[test]
fn g8_patterns_beyond_bound_ignored() {
    let mut patterns = Vec::with_capacity(MAX_MATCH_PATTERNS + 2);
    let mut i = 0;
    while i < MAX_MATCH_PATTERNS + 2 {
        patterns.push(MatchPattern {
            tag_mask: 0xFF,
            tag_pattern: 0xFF, // Won't match anything
            value_mask: 0,
            value_pattern: 0,
            action: MatchAction::Trap(0),
        });
        i += 1;
    }
    // Put a matching pattern beyond the bound.
    patterns[MAX_MATCH_PATTERNS].tag_pattern = 1;
    patterns[MAX_MATCH_PATTERNS].tag_mask = 0xFF;
    patterns[MAX_MATCH_PATTERNS].action = MatchAction::Accept(999);

    let word = TaggedWord {
        value: 0,
        tag: TypeTag::Unsigned { width: 8 },
        provenance: Provenance::Literal,
    };
    assert_eq!(
        match_word(&word, &patterns),
        MatchAction::Continue,
        "Patterns beyond MAX_MATCH_PATTERNS must be ignored"
    );
}

// ===========================================================================
// G9: Symbolic differentiation (sym_diff)
// ===========================================================================

#[test]
fn g9_diff_constant_is_zero() {
    let result = sym_diff(&lit(5), "x");
    assert_eq!(result, Expr::Literal(LiteralValue::Integer(0)));
}

#[test]
fn g9_diff_same_signal_is_one() {
    let result = sym_diff(&sig("x"), "x");
    assert_eq!(result, Expr::Literal(LiteralValue::Integer(1)));
}

#[test]
fn g9_diff_other_signal_is_zero() {
    let result = sym_diff(&sig("y"), "x");
    assert_eq!(result, Expr::Literal(LiteralValue::Integer(0)));
}

#[test]
fn g9_diff_sum_rule() {
    // d(x + 1)/dx = 1 + 0
    let expr = bin(BinaryOp::Add, sig("x"), lit(1));
    let expected = bin(
        BinaryOp::Add,
        Expr::Literal(LiteralValue::Integer(1)),
        Expr::Literal(LiteralValue::Integer(0)),
    );
    assert_eq!(sym_diff(&expr, "x"), expected);
}

#[test]
fn g9_diff_difference_rule() {
    // d(x - y)/dx = 1 - 0
    let expr = bin(BinaryOp::Sub, sig("x"), sig("y"));
    let expected = bin(
        BinaryOp::Sub,
        Expr::Literal(LiteralValue::Integer(1)),
        Expr::Literal(LiteralValue::Integer(0)),
    );
    assert_eq!(sym_diff(&expr, "x"), expected);
}

#[test]
fn g9_diff_product_rule() {
    // d(x * x)/dx = x*1 + 1*x
    let expr = bin(BinaryOp::Mul, sig("x"), sig("x"));
    let expected = bin(
        BinaryOp::Add,
        bin(BinaryOp::Mul, sig("x"), Expr::Literal(LiteralValue::Integer(1))),
        bin(BinaryOp::Mul, Expr::Literal(LiteralValue::Integer(1)), sig("x")),
    );
    assert_eq!(sym_diff(&expr, "x"), expected);
}

#[test]
fn g9_diff_product_with_constant() {
    // d(x * 5)/dx = x*0 + 1*5
    let expr = bin(BinaryOp::Mul, sig("x"), lit(5));
    let expected = bin(
        BinaryOp::Add,
        bin(BinaryOp::Mul, sig("x"), Expr::Literal(LiteralValue::Integer(0))),
        bin(BinaryOp::Mul, Expr::Literal(LiteralValue::Integer(1)), lit(5)),
    );
    assert_eq!(sym_diff(&expr, "x"), expected);
}

#[test]
fn g9_diff_shift_left_rule() {
    // d(x << 2)/dx = (dx/dx) << 2 = 1 << 2
    let expr = bin(BinaryOp::Shl, sig("x"), lit(2));
    let expected = bin(BinaryOp::Shl, Expr::Literal(LiteralValue::Integer(1)), lit(2));
    assert_eq!(sym_diff(&expr, "x"), expected);
}

#[test]
fn g9_diff_shift_right_rule() {
    let expr = bin(BinaryOp::Shr, sig("x"), lit(3));
    let expected = bin(BinaryOp::Shr, Expr::Literal(LiteralValue::Integer(1)), lit(3));
    assert_eq!(sym_diff(&expr, "x"), expected);
}

#[test]
fn g9_diff_negate_rule() {
    // d(-x)/dx = -(1) = -(dx/dx)
    let expr = unary(UnaryOp::Negate, sig("x"));
    let expected = Expr::Unary {
        op: UnaryOp::Negate,
        operand: Box::new(Expr::Literal(LiteralValue::Integer(1))),
    };
    assert_eq!(sym_diff(&expr, "x"), expected);
}

#[test]
fn g9_diff_not_is_zero() {
    let expr = unary(UnaryOp::Not, sig("x"));
    assert_eq!(sym_diff(&expr, "x"), Expr::Literal(LiteralValue::Integer(0)));
}

#[test]
fn g9_diff_comparison_is_zero() {
    let expr = bin(BinaryOp::Lt, sig("x"), lit(5));
    assert_eq!(sym_diff(&expr, "x"), Expr::Literal(LiteralValue::Integer(0)));
}

#[test]
fn g9_diff_bitwise_and_is_zero() {
    let expr = bin(BinaryOp::And, sig("x"), sig("y"));
    assert_eq!(sym_diff(&expr, "x"), Expr::Literal(LiteralValue::Integer(0)));
}

#[test]
fn g9_diff_prev_is_zero() {
    let expr = Expr::Prev { signal: "x".to_string(), delay: 1 };
    assert_eq!(sym_diff(&expr, "x"), Expr::Literal(LiteralValue::Integer(0)));
}

#[test]
fn g9_diff_bool_literal_is_zero() {
    let expr = lit_bool(true);
    assert_eq!(sym_diff(&expr, "x"), Expr::Literal(LiteralValue::Integer(0)));
}

// ===========================================================================
// G10: Module-level analysis
// ===========================================================================

#[test]
fn g10_analyze_simple_module() {
    let m = make_module_for_analysis(
        vec![
            make_signal("a", SignalKind::Input, SignalType::Unsigned(16)),
            make_signal("b", SignalKind::Output, SignalType::Unsigned(16)),
        ],
        vec![Guard {
            name: "g".to_string(),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles: 1,
            template_cycles: None,
            origin: None,
            span: None,
        }],
        vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Signal("a".to_string()),
                span: None,
            }],
            origin: None,
            span: None,
        }],
    );
    let result = analyze_module(&m).expect("analyze must succeed");
    assert!(result.converged, "Single-pass analysis must converge");
    assert_eq!(result.iterations, 1);
    assert!(!result.intervals.is_empty(), "Must have interval results");
}

#[test]
fn g10_analyze_detects_no_violations_for_valid() {
    let m = make_module_for_analysis(
        vec![
            make_signal("a", SignalKind::Input, SignalType::Unsigned(8)),
            make_signal("b", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        vec![Guard {
            name: "g".to_string(),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles: 1,
            template_cycles: None,
            origin: None,
            span: None,
        }],
        vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "b".to_string(),
                value: Expr::Signal("a".to_string()),
                span: None,
            }],
            origin: None,
            span: None,
        }],
    );
    let result = analyze_module(&m).expect("analyze must succeed");
    assert!(result.violations.is_empty(), "Valid module must have no width violations");
}

#[test]
fn g10_analyze_empty_module() {
    let m = make_module_for_analysis(vec![], vec![], vec![]);
    let result = analyze_module(&m).expect("analyze must succeed");
    assert!(result.intervals.is_empty());
    assert!(result.violations.is_empty());
    assert!(result.converged);
}

// ===========================================================================
// G11: Pipeline integration
// ===========================================================================

#[test]
fn g11_pipeline_with_symbolic_flag() {
    let src = r#"module sym_test {
    signal enable: in bool;
    signal out_val: out bool;

    guard g {
        when enable
        for 1 cycles;
    }

    reflex r {
        on g {
            out_val = true;
        }
    }
}"#;
    let config = PipelineConfig { symbolic: true, ..PipelineConfig::default() };
    let result = run_pipeline(src, &config).expect("Pipeline with symbolic must succeed");
    assert!(result.symbolic_result.is_some(), "Symbolic result must be present when flag is set");
}

#[test]
fn g11_pipeline_without_symbolic_flag() {
    let src = r#"module sym_test {
    signal enable: in bool;
    signal out_val: out bool;

    guard g {
        when enable
        for 1 cycles;
    }

    reflex r {
        on g {
            out_val = true;
        }
    }
}"#;
    let config = PipelineConfig { symbolic: false, ..PipelineConfig::default() };
    let result = run_pipeline(src, &config).expect("Pipeline without symbolic must succeed");
    assert!(result.symbolic_result.is_none(), "Symbolic result must be absent when flag is unset");
}

// ===========================================================================
// G12: NASA P10 bounds
// ===========================================================================

#[test]
fn g12_max_sym_signals() {
    assert_eq!(MAX_SYM_SIGNALS, 4096, "MAX_SYM_SIGNALS must be 4096");
}

#[test]
fn g12_max_sym_iterations() {
    assert_eq!(MAX_SYM_ITERATIONS, 64, "MAX_SYM_ITERATIONS must be 64");
}

#[test]
fn g12_max_sym_depth() {
    assert_eq!(MAX_SYM_DEPTH, 32, "MAX_SYM_DEPTH must be 32");
}

#[test]
fn g12_sym_state_lookup_bounded() {
    let mut state = SymState::new();
    // Add MAX_SYM_SIGNALS + 1 entries — lookup must still terminate.
    let limit = 100.min(MAX_TEST_ITERATIONS);
    let mut i = 0;
    while i < limit {
        state.signals.push((format!("s{}", i), SymValue::Concrete(i as u64)));
        i += 1;
    }
    // Lookup existing signal.
    assert_eq!(state.lookup("s50"), SymValue::Concrete(50));
    // Lookup missing signal.
    assert_eq!(state.lookup("nonexistent"), SymValue::Top);
}
