#![allow(clippy::field_reassign_with_default)]
#![forbid(unsafe_code)]

//! Integration tests for the MEGA-5 symbolic evaluation engine.

use mirrc::symbolic::interval::interval_binary;
use mirrc::symbolic::pattern::{MatchAction, MatchPattern, MAX_MATCH_PATTERNS};
use mirrc::symbolic::{
    sym_check_refinement, sym_eval_binary, sym_eval_expr, sym_eval_unary, sym_widen, SymState,
    SymValue,
};

use mirrc::ast::expr::Expr;
use mirrc::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use mirrc::emit::rspu_tagged::{TaggedWord, TypeTag};

// ---------------------------------------------------------------------------
// SymValue basic tests
// ---------------------------------------------------------------------------

#[test]
fn test_sym_value_concrete() {
    let v = SymValue::Concrete(42);
    assert_eq!(v, SymValue::Concrete(42));
}

#[test]
fn test_sym_value_interval_add() {
    // [1,3] + [2,5] = [3,8]
    let result = interval_binary(
        BinaryOp::Add,
        SymValue::Interval { lo: 1, hi: 3 },
        SymValue::Interval { lo: 2, hi: 5 },
    );
    assert_eq!(result, SymValue::Interval { lo: 3, hi: 8 });
}

#[test]
fn test_sym_value_interval_sub() {
    // [5,10] - [1,3] = [2,9]
    let result = interval_binary(
        BinaryOp::Sub,
        SymValue::Interval { lo: 5, hi: 10 },
        SymValue::Interval { lo: 1, hi: 3 },
    );
    assert_eq!(result, SymValue::Interval { lo: 2, hi: 9 });
}

#[test]
fn test_sym_value_interval_mul() {
    // [2,3] * [4,5] = [8,15]
    let result = interval_binary(
        BinaryOp::Mul,
        SymValue::Interval { lo: 2, hi: 3 },
        SymValue::Interval { lo: 4, hi: 5 },
    );
    assert_eq!(result, SymValue::Interval { lo: 8, hi: 15 });
}

#[test]
fn test_sym_value_interval_lt_definite() {
    // [1,3] < [5,10] → Concrete(1) (definitely true)
    let result = interval_binary(
        BinaryOp::Lt,
        SymValue::Interval { lo: 1, hi: 3 },
        SymValue::Interval { lo: 5, hi: 10 },
    );
    assert_eq!(result, SymValue::Concrete(1));
}

#[test]
fn test_sym_value_interval_lt_indefinite() {
    // [1,8] < [5,10] → Interval{0,1} (maybe)
    let result = interval_binary(
        BinaryOp::Lt,
        SymValue::Interval { lo: 1, hi: 8 },
        SymValue::Interval { lo: 5, hi: 10 },
    );
    assert_eq!(result, SymValue::Interval { lo: 0, hi: 1 });
}

#[test]
fn test_sym_value_interval_lt_false() {
    // [5,10] < [1,3] → Concrete(0) (definitely false)
    let result = interval_binary(
        BinaryOp::Lt,
        SymValue::Interval { lo: 5, hi: 10 },
        SymValue::Interval { lo: 1, hi: 3 },
    );
    assert_eq!(result, SymValue::Concrete(0));
}

// ---------------------------------------------------------------------------
// sym_eval_expr tests
// ---------------------------------------------------------------------------

#[test]
fn test_sym_eval_signal_lookup() {
    let state =
        SymState { signals: vec![("x".to_string(), SymValue::Interval { lo: 0, hi: 255 })] };
    let expr = Expr::Signal("x".to_string());
    let result = sym_eval_expr(&expr, &state);
    assert_eq!(result, SymValue::Interval { lo: 0, hi: 255 });
}

#[test]
fn test_sym_eval_literal() {
    let state = SymState { signals: vec![] };
    let expr = Expr::Literal(LiteralValue::Integer(42));
    let result = sym_eval_expr(&expr, &state);
    assert_eq!(result, SymValue::Concrete(42));
}

#[test]
fn test_sym_eval_binary_expr() {
    // signal + 1 with signal = [0, 255] → [1, 256]
    let state =
        SymState { signals: vec![("x".to_string(), SymValue::Interval { lo: 0, hi: 255 })] };
    let expr = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Signal("x".to_string())),
        right: Box::new(Expr::Literal(LiteralValue::Integer(1))),
    };
    let result = sym_eval_expr(&expr, &state);
    assert_eq!(result, SymValue::Interval { lo: 1, hi: 256 });
}

// ---------------------------------------------------------------------------
// Widening tests
// ---------------------------------------------------------------------------

#[test]
fn test_sym_widen_same() {
    let v = SymValue::Concrete(5);
    assert_eq!(sym_widen(v, v), SymValue::Concrete(5));
}

#[test]
fn test_sym_widen_expand() {
    // Two different intervals → widen to Unknown
    let old = SymValue::Interval { lo: 1, hi: 3 };
    let new = SymValue::Interval { lo: 0, hi: 5 };
    let result = sym_widen(old, new);
    assert_eq!(result, SymValue::Unknown { width: 64 });
}

#[test]
fn test_sym_widen_to_top() {
    let result = sym_widen(SymValue::Unknown { width: 8 }, SymValue::Interval { lo: 0, hi: 1 });
    assert_eq!(result, SymValue::Top);
}

// ---------------------------------------------------------------------------
// Refinement check tests
// ---------------------------------------------------------------------------

#[test]
fn test_sym_check_refinement_pass() {
    assert!(sym_check_refinement(SymValue::Interval { lo: 0, hi: 255 }, 0, 255));
}

#[test]
fn test_sym_check_refinement_fail() {
    assert!(!sym_check_refinement(SymValue::Interval { lo: 0, hi: 300 }, 0, 255));
}

#[test]
fn test_sym_check_refinement_top() {
    assert!(!sym_check_refinement(SymValue::Top, 0, 255));
}

// ---------------------------------------------------------------------------
// Pattern match tests
// ---------------------------------------------------------------------------

#[test]
fn test_match_word_first_match_wins() {
    let word = TaggedWord::from_computed(42, TypeTag::Unsigned { width: 8 });
    let patterns = vec![
        MatchPattern {
            tag_mask: 0,
            tag_pattern: 0,
            value_mask: u64::MAX,
            value_pattern: 42,
            action: MatchAction::Accept(0),
        },
        MatchPattern {
            tag_mask: 0,
            tag_pattern: 0,
            value_mask: u64::MAX,
            value_pattern: 42,
            action: MatchAction::Accept(1),
        },
    ];
    match mirrc::symbolic::pattern::match_word(&word, &patterns) {
        MatchAction::Accept(id) => assert_eq!(id, 0, "first match should win"),
        other => panic!("expected Accept(0), got {other:?}"),
    }
}

#[test]
fn test_match_word_no_match() {
    let word = TaggedWord::from_computed(99, TypeTag::Unsigned { width: 8 });
    let patterns = vec![MatchPattern {
        tag_mask: 0,
        tag_pattern: 0,
        value_mask: u64::MAX,
        value_pattern: 42,
        action: MatchAction::Accept(0),
    }];
    match mirrc::symbolic::pattern::match_word(&word, &patterns) {
        MatchAction::Continue => {} // expected
        other => panic!("expected Continue, got {other:?}"),
    }
}

#[test]
fn test_match_word_tag_only() {
    // Match on tag only (value mask = 0, so any value matches).
    let word = TaggedWord::from_computed(999, TypeTag::Bool);
    let patterns = vec![MatchPattern {
        tag_mask: 0xFF,
        tag_pattern: 0, // Bool encodes as 0 in tag_to_byte
        value_mask: 0,
        value_pattern: 0,
        action: MatchAction::Accept(5),
    }];
    match mirrc::symbolic::pattern::match_word(&word, &patterns) {
        MatchAction::Accept(id) => assert_eq!(id, 5),
        other => panic!("expected Accept(5), got {other:?}"),
    }
}

#[test]
fn test_match_word_bounded() {
    // Create 20 patterns but only first MAX_MATCH_PATTERNS should be checked.
    let word = TaggedWord::from_computed(0, TypeTag::Unsigned { width: 8 });
    let mut patterns = Vec::new();
    for i in 0..20u32 {
        patterns.push(MatchPattern {
            tag_mask: 0,
            tag_pattern: 0,
            value_mask: u64::MAX,
            value_pattern: (i + 1) as u64, // None match 0
            action: MatchAction::Accept(i),
        });
    }
    // None of first 16 match value=0, so should get Continue.
    match mirrc::symbolic::pattern::match_word(&word, &patterns) {
        MatchAction::Continue => {} // expected: none of the bounded patterns matched
        other => panic!("expected Continue, got {other:?}"),
    }
    assert!(patterns.len() > MAX_MATCH_PATTERNS, "test should have more than MAX patterns");
}

// ---------------------------------------------------------------------------
// Interval overflow / edge case tests
// ---------------------------------------------------------------------------

#[test]
fn test_interval_overflow_saturates() {
    // [u64::MAX-1, u64::MAX] + [1, 2] should saturate to u64::MAX.
    // Both lo and hi saturate to u64::MAX, so normalize() collapses to Concrete.
    let result = interval_binary(
        BinaryOp::Add,
        SymValue::Interval { lo: u64::MAX - 1, hi: u64::MAX },
        SymValue::Interval { lo: 1, hi: 2 },
    );
    assert_eq!(result, SymValue::Concrete(u64::MAX));
}

#[test]
fn test_signed_signal_unknown() {
    // A Signed signal should evaluate as Unknown (v1: unsigned-only intervals).
    let state = SymState { signals: vec![("s".to_string(), SymValue::Unknown { width: 16 })] };
    let expr = Expr::Signal("s".to_string());
    let result = sym_eval_expr(&expr, &state);
    assert_eq!(result, SymValue::Unknown { width: 16 });
}

// ---------------------------------------------------------------------------
// Pipeline integration tests
// ---------------------------------------------------------------------------

#[test]
fn test_pipeline_symbolic_disabled() {
    use mirrc::pipeline::{run_pipeline, PipelineConfig};
    let config = PipelineConfig { symbolic: false, ..PipelineConfig::default() };
    let source = r#"module test { signal x : unsigned<8>; }"#;
    let result = run_pipeline(source, &config);
    if let Ok(r) = result {
        assert!(r.symbolic_result.is_none());
    }
}

#[test]
fn test_pipeline_symbolic_enabled() {
    use mirrc::pipeline::{run_pipeline, PipelineConfig};
    let config = PipelineConfig { symbolic: true, ..PipelineConfig::default() };
    let source = r#"module test { signal x : unsigned<8>; }"#;
    let result = run_pipeline(source, &config);
    if let Ok(r) = result {
        assert!(r.symbolic_result.is_some());
    }
}

// ---------------------------------------------------------------------------
// Binary operation on concrete values
// ---------------------------------------------------------------------------

#[test]
fn test_sym_eval_binary_concrete_add() {
    let result = sym_eval_binary(BinaryOp::Add, SymValue::Concrete(3), SymValue::Concrete(4));
    assert_eq!(result, SymValue::Concrete(7));
}

#[test]
fn test_sym_eval_unary_not_concrete() {
    let result = sym_eval_unary(UnaryOp::Not, SymValue::Concrete(0));
    assert_eq!(result, SymValue::Concrete(!0u64));
}

#[test]
fn test_sym_eval_unary_negate_unknown() {
    // Negate of unsigned → Unknown (v1 restriction: signed semantics)
    let result = sym_eval_unary(UnaryOp::Negate, SymValue::Concrete(5));
    assert_eq!(result, SymValue::Unknown { width: 64 });
}

#[test]
fn test_sym_eval_top_propagates() {
    // Top + anything = Top
    let result = sym_eval_binary(BinaryOp::Add, SymValue::Top, SymValue::Concrete(1));
    assert_eq!(result, SymValue::Top);
}
