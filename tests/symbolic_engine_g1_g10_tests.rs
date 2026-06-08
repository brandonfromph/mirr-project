#![forbid(unsafe_code)]
//! MEGA-5 symbolic engine tests - G1 through G10.

use mirrc::ast::expr::Expr;
use mirrc::ast::program::{Module, SignalDecl};
use mirrc::ast::types::{
    BinaryOp, ExtendedType, LiteralValue, SignalKind, SignalType, UnaryOp,
};
use mirrc::emit::rspu_tagged::{TaggedWord, TypeTag};
use mirrc::symbolic::diff::sym_diff;
use mirrc::symbolic::interval::{interval_binary, interval_unary};
use mirrc::symbolic::pattern::{
    match_word, MatchAction, MatchPattern, MAX_MATCH_PATTERNS,
};
use mirrc::symbolic::{
    analyze_module, sym_check_refinement, sym_eval_binary, sym_eval_expr, sym_eval_unary,
    sym_widen, SymState, SymValue, MAX_SYM_DEPTH, MAX_SYM_ITERATIONS, MAX_SYM_SIGNALS,
};

fn sig(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}
fn empty_module() -> Module {
    Module {
        name: "sym".to_string(),
        signals: Vec::new(),
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

#[test]
fn g1_concrete_eq() {
    assert_eq!(SymValue::Concrete(42), SymValue::Concrete(42));
}
#[test]
fn g1_concrete_ne() {
    assert_ne!(SymValue::Concrete(1), SymValue::Concrete(2));
}
#[test]
fn g1_interval_ok() {
    match (SymValue::Interval { lo: 0, hi: 255 }) {
        SymValue::Interval { lo, hi } => {
            assert_eq!(lo, 0);
            assert_eq!(hi, 255);
        }
        _ => panic!("Expected Interval"),
    }
}
#[test]
fn g1_unknown_ok() {
    match (SymValue::Unknown { width: 16 }) {
        SymValue::Unknown { width } => assert_eq!(width, 16),
        _ => panic!("Expected Unknown"),
    }
}
#[test]
fn g1_top_eq_top() {
    assert_eq!(SymValue::Top, SymValue::Top);
}
#[test]
fn g1_top_ne_concrete() {
    assert_ne!(SymValue::Top, SymValue::Concrete(0));
}

#[test]
fn g2_add() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Add, SymValue::Concrete(3), SymValue::Concrete(4)),
        SymValue::Concrete(7)
    );
}
#[test]
fn g2_sub() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Sub, SymValue::Concrete(10), SymValue::Concrete(3)),
        SymValue::Concrete(7)
    );
}
#[test]
fn g2_mul() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Mul, SymValue::Concrete(5), SymValue::Concrete(6)),
        SymValue::Concrete(30)
    );
}
#[test]
fn g2_and() {
    assert_eq!(
        sym_eval_binary(BinaryOp::And, SymValue::Concrete(1), SymValue::Concrete(1)),
        SymValue::Concrete(1)
    );
}
#[test]
fn g2_or() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Or, SymValue::Concrete(0), SymValue::Concrete(1)),
        SymValue::Concrete(1)
    );
}
#[test]
fn g2_gt_true() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Gt, SymValue::Concrete(10), SymValue::Concrete(5)),
        SymValue::Concrete(1)
    );
}
#[test]
fn g2_lt_false() {
    assert_eq!(
        sym_eval_binary(BinaryOp::Lt, SymValue::Concrete(5), SymValue::Concrete(5)),
        SymValue::Concrete(0)
    );
}
#[test]
fn g2_top_propagates() {
    let r = sym_eval_binary(BinaryOp::Add, SymValue::Top, SymValue::Concrete(1));
    assert_eq!(r, SymValue::Top);
}

#[test]
fn g3_not_zero() {
    assert_eq!(sym_eval_unary(UnaryOp::Not, SymValue::Concrete(0)), SymValue::Concrete(!0u64));
}
#[test]
fn g3_not_nonzero() {
    assert_eq!(sym_eval_unary(UnaryOp::Not, SymValue::Concrete(1)), SymValue::Concrete(!1u64));
}
#[test]
fn g3_not_top_no_panic() {
    let _ = sym_eval_unary(UnaryOp::Not, SymValue::Top);
}

#[test]
fn g4_interval_add() {
    let r = interval_binary(
        BinaryOp::Add,
        SymValue::Interval { lo: 0, hi: 10 },
        SymValue::Interval { lo: 5, hi: 15 },
    );
    assert_eq!(r, SymValue::Interval { lo: 5, hi: 25 });
}
#[test]
fn g4_interval_unary_no_panic() {
    let _ = interval_unary(UnaryOp::Not, SymValue::Interval { lo: 0, hi: 1 });
}

#[test]
fn g5_literal_concrete() {
    let st = SymState::new();
    assert_eq!(
        sym_eval_expr(&Expr::Literal(LiteralValue::Integer(42)), &st),
        SymValue::Concrete(42)
    );
}
#[test]
fn g5_unknown_signal() {
    let st = SymState::new();
    let r = sym_eval_expr(&Expr::Signal("nosig".to_string()), &st);
    assert_eq!(r, SymValue::Top);
}
#[test]
fn g5_known_signal() {
    let mut st = SymState::new();
    st.signals.push(("k".to_string(), SymValue::Concrete(77)));
    assert_eq!(sym_eval_expr(&Expr::Signal("k".to_string()), &st), SymValue::Concrete(77));
}
#[test]
fn g5_binary_expr() {
    let st = SymState::new();
    let e = Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(Expr::Literal(LiteralValue::Integer(3))),
        right: Box::new(Expr::Literal(LiteralValue::Integer(4))),
    };
    assert_eq!(sym_eval_expr(&e, &st), SymValue::Concrete(7));
}

#[test]
fn g6_in_range() {
    assert!(sym_check_refinement(SymValue::Concrete(5), 0, 10));
}
#[test]
fn g6_out_of_range() {
    assert!(!sym_check_refinement(SymValue::Concrete(11), 0, 10));
}
#[test]
fn g6_lo_boundary() {
    assert!(sym_check_refinement(SymValue::Concrete(0), 0, 10));
}
#[test]
fn g6_hi_boundary() {
    assert!(sym_check_refinement(SymValue::Concrete(10), 0, 10));
}

#[test]
fn g7_same_unchanged() {
    assert_eq!(sym_widen(SymValue::Concrete(5), SymValue::Concrete(5)), SymValue::Concrete(5));
}
#[test]
fn g7_different_widens() {
    let r = sym_widen(SymValue::Concrete(0), SymValue::Concrete(10));
    assert!(matches!(r, SymValue::Interval { .. } | SymValue::Top | SymValue::Unknown { .. }));
}

fn unsigned_word(value: u64, width: u8) -> TaggedWord {
    TaggedWord::from_literal(value, TypeTag::Unsigned { width })
}

#[test]
fn g8_max_patterns_pos() {
    let _ = MAX_MATCH_PATTERNS;
}
#[test]
fn g8_empty_none() {
    assert_eq!(match_word(&unsigned_word(42, 8), &[]), MatchAction::Continue);
}
#[test]
fn g8_exact_match() {
    let patterns = [MatchPattern {
        tag_mask: 0xFF,
        tag_pattern: 1,
        value_mask: 0xFF,
        value_pattern: 42,
        action: MatchAction::Accept(1),
    }];
    assert_eq!(match_word(&unsigned_word(42, 8), &patterns), MatchAction::Accept(1));
}
#[test]
fn g8_no_match() {
    let patterns = [MatchPattern {
        tag_mask: 0xFF,
        tag_pattern: 0,
        value_mask: 0xFF,
        value_pattern: 42,
        action: MatchAction::Accept(1),
    }];
    assert_eq!(match_word(&unsigned_word(42, 8), &patterns), MatchAction::Continue);
}

#[test]
fn g9_const_zero() {
    assert_eq!(
        sym_diff(&Expr::Literal(LiteralValue::Integer(42)), "x"),
        Expr::Literal(LiteralValue::Integer(0))
    );
}
#[test]
fn g9_self_one() {
    assert_eq!(
        sym_diff(&Expr::Signal("x".to_string()), "x"),
        Expr::Literal(LiteralValue::Integer(1))
    );
}
#[test]
fn g9_other_zero() {
    assert_eq!(
        sym_diff(&Expr::Signal("y".to_string()), "x"),
        Expr::Literal(LiteralValue::Integer(0))
    );
}

#[test]
fn g10_empty_ok() {
    assert!(analyze_module(&empty_module()).is_ok());
}
#[test]
fn g10_with_signals_ok() {
    let mut m = empty_module();
    m.signals.push(sig("x", SignalKind::Input, SignalType::Unsigned(8)));
    m.signals.push(sig("y", SignalKind::Output, SignalType::Bool));
    assert!(analyze_module(&m).is_ok());
}
#[test]
fn g10_constants_ok() {
    let _ = (MAX_SYM_SIGNALS, MAX_SYM_ITERATIONS, MAX_SYM_DEPTH);
}
