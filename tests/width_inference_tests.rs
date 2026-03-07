//! Phase 4 width inference integration tests.
//!
//! Categories:
//! - Literal width inference
//! - Signal width propagation
//! - Addition overflow detection
//! - Subtraction semantics
//! - Multiplication width explosion
//! - Shift width semantics
//! - Logical/bitwise operators
//! - Comparison operators (boolean output)
//! - Unary operators
//! - Unsafe truncation diagnostics
//! - Integration with Phase 3 simplifier
//! - Edge cases (zero, max, single node, deep tree)
//! - Diagnostic message pinning

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::SignalDecl;
use nasa_rust_project::ast::types::*;
use nasa_rust_project::width;
use nasa_rust_project::width::types::*;
use nasa_rust_project::width::WidthInferenceResult;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn sig(name: &str, ty: SignalType) -> SignalDecl {
    SignalDecl { name: name.to_string(), kind: SignalKind::Internal, ty }
}

fn lit(v: u64) -> Expr {
    Expr::Literal(LiteralValue::Integer(v))
}

fn bool_lit(b: bool) -> Expr {
    Expr::Literal(LiteralValue::Bool(b))
}

fn signal(name: &str) -> Expr {
    Expr::Signal(name.to_string())
}

fn binary(op: BinaryOp, l: Expr, r: Expr) -> Expr {
    Expr::Binary { op, left: Box::new(l), right: Box::new(r) }
}

fn unary(op: UnaryOp, e: Expr) -> Expr {
    Expr::Unary { op, operand: Box::new(e) }
}

fn infer(expr: &Expr, signals: &[SignalDecl]) -> WidthInferenceResult {
    width::infer_widths(expr, signals)
}

fn root_width(result: &WidthInferenceResult) -> u32 {
    result.expr.as_ref().expect("inference should produce WidthExpr").width().0
}

fn has_error_containing(result: &WidthInferenceResult, needle: &str) -> bool {
    result
        .diagnostics
        .iter()
        .any(|d| d.severity == DiagSeverity::Error && d.message.contains(needle))
}

fn has_info_containing(result: &WidthInferenceResult, needle: &str) -> bool {
    result
        .diagnostics
        .iter()
        .any(|d| d.severity == DiagSeverity::Info && d.message.contains(needle))
}

fn error_messages(result: &WidthInferenceResult) -> Vec<String> {
    result
        .diagnostics
        .iter()
        .filter(|d| d.severity == DiagSeverity::Error)
        .map(|d| d.message.clone())
        .collect()
}

// ===========================================================================
// 1. LITERAL WIDTH INFERENCE
// ===========================================================================

#[test]
fn literal_zero_needs_1_bit() {
    let r = infer(&lit(0), &[]);
    assert_eq!(root_width(&r), 1);
}

#[test]
fn literal_one_needs_1_bit() {
    let r = infer(&lit(1), &[]);
    assert_eq!(root_width(&r), 1);
}

#[test]
fn literal_255_needs_8_bits() {
    let r = infer(&lit(255), &[]);
    assert_eq!(root_width(&r), 8);
}

#[test]
fn literal_256_needs_9_bits() {
    let r = infer(&lit(256), &[]);
    assert_eq!(root_width(&r), 9);
}

#[test]
fn literal_u16_max_needs_16_bits() {
    let r = infer(&lit(0xFFFF), &[]);
    assert_eq!(root_width(&r), 16);
}

#[test]
fn literal_u32_max_needs_32_bits() {
    let r = infer(&lit(0xFFFF_FFFF), &[]);
    assert_eq!(root_width(&r), 32);
}

#[test]
fn literal_u64_max_needs_64_bits() {
    let r = infer(&lit(u64::MAX), &[]);
    assert_eq!(root_width(&r), 64);
}

#[test]
fn literal_bool_true_needs_1_bit() {
    let r = infer(&bool_lit(true), &[]);
    assert_eq!(root_width(&r), 1);
}

#[test]
fn literal_bool_false_needs_1_bit() {
    let r = infer(&bool_lit(false), &[]);
    // false = 0 -> min_bits(0) = 1
    assert_eq!(root_width(&r), 1);
}

// ===========================================================================
// 2. SIGNAL WIDTH PROPAGATION
// ===========================================================================

#[test]
fn signal_u8_has_width_8() {
    let sigs = [sig("a", SignalType::Unsigned(8))];
    let r = infer(&signal("a"), &sigs);
    assert_eq!(root_width(&r), 8);
}

#[test]
fn signal_u32_has_width_32() {
    let sigs = [sig("x", SignalType::Unsigned(32))];
    let r = infer(&signal("x"), &sigs);
    assert_eq!(root_width(&r), 32);
}

#[test]
fn signal_bool_has_width_1() {
    let sigs = [sig("flag", SignalType::Bool)];
    let r = infer(&signal("flag"), &sigs);
    assert_eq!(root_width(&r), 1);
}

#[test]
fn signal_undeclared_produces_error() {
    let r = infer(&signal("missing"), &[]);
    assert!(has_error_containing(&r, "signal 'missing' has no declared width"));
}

#[test]
fn signal_undeclared_exact_diagnostic_text() {
    let r = infer(&signal("ghost"), &[]);
    let errs = error_messages(&r);
    assert_eq!(errs.len(), 1);
    assert_eq!(errs[0], "signal 'ghost' has no declared width");
}

// ===========================================================================
// 3. ADDITION OVERFLOW DETECTION
// ===========================================================================

#[test]
fn add_u8_u8_needs_u9() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 9);
}

#[test]
fn add_u8_u16_needs_u17() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(16))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 17);
}

#[test]
fn add_u32_u32_needs_u33() {
    let sigs = [sig("a", SignalType::Unsigned(32)), sig("b", SignalType::Unsigned(32))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 33);
}

#[test]
fn add_chain_u8_three_times_needs_u10() {
    // (a + b) + c, where all are u8.
    // a + b = u9, (a+b) + c = max(9,8)+1 = u10
    let sigs = [
        sig("a", SignalType::Unsigned(8)),
        sig("b", SignalType::Unsigned(8)),
        sig("c", SignalType::Unsigned(8)),
    ];
    let ab = binary(BinaryOp::Add, signal("a"), signal("b"));
    let abc = binary(BinaryOp::Add, ab, signal("c"));
    let r = infer(&abc, &sigs);
    assert_eq!(root_width(&r), 10);
}

// ===========================================================================
// 4. SUBTRACTION SEMANTICS
// ===========================================================================

#[test]
fn sub_u16_u8_needs_u16() {
    let sigs = [sig("a", SignalType::Unsigned(16)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Sub, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 16);
}

#[test]
fn sub_emits_underflow_info() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Sub, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert!(has_info_containing(&r, "unsigned subtraction may underflow"));
}

#[test]
fn sub_underflow_info_exact_text() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Sub, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    let infos: Vec<&str> = r
        .diagnostics
        .iter()
        .filter(|d| d.severity == DiagSeverity::Info)
        .map(|d| d.message.as_str())
        .collect();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0], "unsigned subtraction may underflow (wrapping semantics)");
}

#[test]
fn sub_literal_safe_no_underflow_info() {
    // 10 - 3: left (10) >= right (3), provably safe — no info emitted.
    let r = infer(&binary(BinaryOp::Sub, lit(10), lit(3)), &[]);
    let infos: Vec<&WidthDiag> =
        r.diagnostics.iter().filter(|d| d.severity == DiagSeverity::Info).collect();
    assert!(infos.is_empty(), "expected no underflow info for safe literal subtraction");
}

#[test]
fn sub_literal_underflow_possible_emits_info() {
    // 3 - 10: left (3) < right (10), underflow possible — info emitted.
    let r = infer(&binary(BinaryOp::Sub, lit(3), lit(10)), &[]);
    assert!(has_info_containing(&r, "unsigned subtraction may underflow"));
}
// ===========================================================================

#[test]
fn mul_u8_u8_needs_u16() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Mul, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 16);
}

#[test]
fn mul_u16_u16_needs_u32() {
    let sigs = [sig("a", SignalType::Unsigned(16)), sig("b", SignalType::Unsigned(16))];
    let e = binary(BinaryOp::Mul, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 32);
}

#[test]
fn mul_u32_u32_needs_u64() {
    let sigs = [sig("a", SignalType::Unsigned(32)), sig("b", SignalType::Unsigned(32))];
    let e = binary(BinaryOp::Mul, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 64);
}

#[test]
fn mul_u32_u33_exceeds_64_bits_error() {
    // u32 * u33 = u65 > 64 -> hard error
    let sigs = [sig("a", SignalType::Unsigned(32)), sig("b", SignalType::Unsigned(33))];
    let e = binary(BinaryOp::Mul, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert!(r.has_errors());
    assert!(has_error_containing(&r, "exceeding maximum of 64"));
}

#[test]
fn mul_chain_overflows() {
    // (u16 * u16) * u16 = u32 * u16 = u48 (still ok)
    let sigs = [
        sig("a", SignalType::Unsigned(16)),
        sig("b", SignalType::Unsigned(16)),
        sig("c", SignalType::Unsigned(16)),
    ];
    let ab = binary(BinaryOp::Mul, signal("a"), signal("b"));
    let abc = binary(BinaryOp::Mul, ab, signal("c"));
    let r = infer(&abc, &sigs);
    assert_eq!(root_width(&r), 48);
}

// ===========================================================================
// 6. SHIFT WIDTH SEMANTICS
// ===========================================================================

#[test]
fn shl_u8_by_3_needs_u11() {
    let sigs = [sig("a", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Shl, signal("a"), lit(3));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 11);
}

#[test]
fn shl_u8_by_0_needs_u8() {
    let sigs = [sig("a", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Shl, signal("a"), lit(0));
    let r = infer(&e, &sigs);
    // shift by 0 => left_width + 0 = 8
    assert_eq!(root_width(&r), 8);
}

#[test]
fn shl_by_63_clamped() {
    let sigs = [sig("a", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Shl, signal("a"), lit(100));
    let r = infer(&e, &sigs);
    // 100 clamped to 63, 8 + 63 = 71 > 64 -> error
    assert!(r.has_errors());
    assert!(has_error_containing(&r, "exceeding maximum of 64"));
}

#[test]
fn shl_variable_shift_uses_worst_case() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(6))];
    let e = binary(BinaryOp::Shl, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    // Variable shift: worst case = a_width + 63 = 71 > 64 -> error
    assert!(r.has_errors());
}

#[test]
fn shr_constant_shift_narrows_width() {
    // a: u16 >> 4 — result needs max(1, 16-4) = 12 bits, not 16.
    let sigs = [sig("a", SignalType::Unsigned(16))];
    let e = binary(BinaryOp::Shr, signal("a"), lit(4));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 12);
}

#[test]
fn shr_variable_shift_preserves_full_width() {
    // Variable shift: conservative — result could need full left_width bits.
    let sigs = [sig("a", SignalType::Unsigned(16)), sig("k", SignalType::Unsigned(4))];
    let e = binary(BinaryOp::Shr, signal("a"), signal("k"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 16);
}

#[test]
fn shr_by_full_width_clamps_to_1() {
    // a: u8 >> 8 — shift by full width; result needs max(1, 8-8) = max(1,0) = 1 bit.
    let sigs = [sig("a", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Shr, signal("a"), lit(8));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 1);
}

// ===========================================================================
// 7. LOGICAL / BITWISE OPERATORS
// ===========================================================================

#[test]
fn and_u8_u16_needs_u16() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(16))];
    let e = binary(BinaryOp::And, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 16);
}

#[test]
fn or_u8_u8_needs_u8() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Or, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 8);
}

#[test]
fn xor_u32_u8_needs_u32() {
    let sigs = [sig("a", SignalType::Unsigned(32)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Xor, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 32);
}

// ===========================================================================
// 8. COMPARISON OPERATORS (boolean output)
// ===========================================================================

#[test]
fn lt_produces_1_bit() {
    let sigs = [sig("a", SignalType::Unsigned(32)), sig("b", SignalType::Unsigned(32))];
    let e = binary(BinaryOp::Lt, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 1);
}

#[test]
fn eq_produces_1_bit() {
    let sigs = [sig("a", SignalType::Unsigned(16)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Eq, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 1);
}

#[test]
fn ne_produces_1_bit() {
    let sigs = [sig("a", SignalType::Unsigned(64)), sig("b", SignalType::Unsigned(64))];
    let e = binary(BinaryOp::Ne, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 1);
}

#[test]
fn ge_produces_1_bit() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Ge, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 1);
}

// ===========================================================================
// 9. UNARY OPERATORS
// ===========================================================================

#[test]
fn not_preserves_width_u16() {
    let sigs = [sig("a", SignalType::Unsigned(16))];
    let e = unary(UnaryOp::Not, signal("a"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 16);
}

#[test]
fn not_bool_stays_1_bit() {
    let sigs = [sig("flag", SignalType::Bool)];
    let e = unary(UnaryOp::Not, signal("flag"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 1);
}

// ===========================================================================
// 10. UNSAFE TRUNCATION DIAGNOSTICS
// ===========================================================================

#[test]
fn truncation_u16_to_u8_error() {
    use nasa_rust_project::ast::program::Assignment;
    let sigs = [sig("src", SignalType::Unsigned(16)), sig("dst", SignalType::Unsigned(8))];
    let a = Assignment { target: "dst".to_string(), value: signal("src") };
    let diags = width::check_assignment(&a, &sigs);
    let errors: Vec<&WidthDiag> =
        diags.iter().filter(|d| d.severity == DiagSeverity::Error).collect();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("truncates from 16 bits to 8 bits"));
}

#[test]
fn truncation_exact_text() {
    use nasa_rust_project::ast::program::Assignment;
    let sigs = [sig("wide", SignalType::Unsigned(32)), sig("narrow", SignalType::Unsigned(16))];
    let a = Assignment { target: "narrow".to_string(), value: signal("wide") };
    let diags = width::check_assignment(&a, &sigs);
    let errors: Vec<String> = diags
        .iter()
        .filter(|d| d.severity == DiagSeverity::Error)
        .map(|d| d.message.clone())
        .collect();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0], "assignment to 'narrow' truncates from 32 bits to 16 bits");
}

#[test]
fn no_truncation_when_widths_match() {
    use nasa_rust_project::ast::program::Assignment;
    let sigs = [sig("src", SignalType::Unsigned(8)), sig("dst", SignalType::Unsigned(8))];
    let a = Assignment { target: "dst".to_string(), value: signal("src") };
    let diags = width::check_assignment(&a, &sigs);
    let errors: Vec<&WidthDiag> =
        diags.iter().filter(|d| d.severity == DiagSeverity::Error).collect();
    assert!(errors.is_empty());
}

#[test]
fn no_truncation_when_target_wider() {
    use nasa_rust_project::ast::program::Assignment;
    let sigs = [sig("src", SignalType::Unsigned(8)), sig("dst", SignalType::Unsigned(16))];
    let a = Assignment { target: "dst".to_string(), value: signal("src") };
    let diags = width::check_assignment(&a, &sigs);
    let errors: Vec<&WidthDiag> =
        diags.iter().filter(|d| d.severity == DiagSeverity::Error).collect();
    assert!(errors.is_empty());
}

#[test]
fn truncation_add_overflow_to_narrow_target() {
    use nasa_rust_project::ast::program::Assignment;
    let sigs = [
        sig("a", SignalType::Unsigned(8)),
        sig("b", SignalType::Unsigned(8)),
        sig("out", SignalType::Unsigned(8)),
    ];
    // a + b = u9, assigned to u8 -> truncation
    let a = Assignment {
        target: "out".to_string(),
        value: binary(BinaryOp::Add, signal("a"), signal("b")),
    };
    let diags = width::check_assignment(&a, &sigs);
    let errors: Vec<&WidthDiag> =
        diags.iter().filter(|d| d.severity == DiagSeverity::Error).collect();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("truncates from 9 bits to 8 bits"));
}

// ===========================================================================
// 11. INTEGRATION WITH PHASE 3 SIMPLIFIER
// ===========================================================================

#[test]
fn simplifier_reduces_constant_then_width_inferred() {
    use nasa_rust_project::simplify::simplify_expr;
    // (5 + 3) should simplify to 8, which needs 4 bits.
    let e = binary(BinaryOp::Add, lit(5), lit(3));
    let simplified = simplify_expr(e);
    let r = infer(&simplified, &[]);
    // 8 = 0b1000, needs 4 bits
    assert_eq!(root_width(&r), 4);
}

#[test]
fn simplifier_folds_zero_add_then_width_correct() {
    use nasa_rust_project::simplify::simplify_expr;
    // x + 0 simplifies to x
    let sigs = [sig("x", SignalType::Unsigned(16))];
    let e = binary(BinaryOp::Add, signal("x"), lit(0));
    let simplified = simplify_expr(e);
    let r = infer(&simplified, &sigs);
    assert_eq!(root_width(&r), 16);
}

#[test]
fn simplifier_folds_mul_by_one_then_width_correct() {
    use nasa_rust_project::simplify::simplify_expr;
    // x * 1 simplifies to x
    let sigs = [sig("x", SignalType::Unsigned(32))];
    let e = binary(BinaryOp::Mul, signal("x"), lit(1));
    let simplified = simplify_expr(e);
    let r = infer(&simplified, &sigs);
    assert_eq!(root_width(&r), 32);
}

#[test]
fn simplifier_folds_double_negation_width() {
    use nasa_rust_project::simplify::simplify_expr;
    // !!x simplifies to x
    let sigs = [sig("x", SignalType::Unsigned(8))];
    let e = unary(UnaryOp::Not, unary(UnaryOp::Not, signal("x")));
    let simplified = simplify_expr(e);
    let r = infer(&simplified, &sigs);
    assert_eq!(root_width(&r), 8);
}

// ===========================================================================
// 12. EDGE CASES
// ===========================================================================

#[test]
fn single_literal_node() {
    let r = infer(&lit(42), &[]);
    assert_eq!(root_width(&r), 6); // 42 = 0b101010, 6 bits
    assert!(!r.has_errors());
}

#[test]
fn width_64_is_maximum_allowed() {
    let sigs = [sig("a", SignalType::Unsigned(64))];
    let r = infer(&signal("a"), &sigs);
    assert_eq!(root_width(&r), 64);
    assert!(!r.has_errors());
}

#[test]
fn add_at_u64_boundary() {
    // u63 + u63 = u64 (ok)
    let sigs = [sig("a", SignalType::Unsigned(63)), sig("b", SignalType::Unsigned(63))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(root_width(&r), 64);
    assert!(!r.has_errors());
}

#[test]
fn add_exceeds_u64_boundary() {
    // u64 + u64 = u65 -> hard error
    let sigs = [sig("a", SignalType::Unsigned(64)), sig("b", SignalType::Unsigned(64))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert!(r.has_errors());
    assert!(has_error_containing(&r, "exceeding maximum of 64"));
}

#[test]
fn add_exceeds_u64_exact_diagnostic_text() {
    let sigs = [sig("a", SignalType::Unsigned(64)), sig("b", SignalType::Unsigned(64))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    let errs = error_messages(&r);
    // Should contain a message about exceeding 64 bits at the Add node
    assert!(errs.iter().any(|m| m.contains("requires") && m.contains("65 bits")));
}

#[test]
fn nested_mixed_ops_width() {
    // (u8 + u8) * u4 = u9 * u4 = u13
    let sigs = [
        sig("a", SignalType::Unsigned(8)),
        sig("b", SignalType::Unsigned(8)),
        sig("c", SignalType::Unsigned(4)),
    ];
    let ab = binary(BinaryOp::Add, signal("a"), signal("b"));
    let result = binary(BinaryOp::Mul, ab, signal("c"));
    let r = infer(&result, &sigs);
    assert_eq!(root_width(&r), 13);
}

#[test]
fn display_format_simple() {
    let sigs = [sig("a", SignalType::Unsigned(8))];
    let r = infer(&signal("a"), &sigs);
    let we = r.expr.as_ref().unwrap();
    let formatted = width::display::format_width_expr(we);
    assert_eq!(formatted, "a:u8");
}

#[test]
fn display_format_binary() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    let we = r.expr.as_ref().unwrap();
    let formatted = width::display::format_width_expr(we);
    assert_eq!(formatted, "(a:u8 + b:u8):u9");
}

#[test]
fn stats_report_correct_node_count() {
    let sigs = [sig("a", SignalType::Unsigned(8)), sig("b", SignalType::Unsigned(8))];
    let e = binary(BinaryOp::Add, signal("a"), signal("b"));
    let r = infer(&e, &sigs);
    assert_eq!(r.stats.nodes_analyzed, 3); // signal, signal, binary
}

#[test]
fn stats_propagation_rounds_at_least_one() {
    let r = infer(&lit(42), &[]);
    assert!(r.stats.propagation_rounds >= 1);
}

#[test]
fn width_diagnostic_display_format() {
    let d = WidthDiag::error("test error message");
    assert_eq!(format!("{}", d), "[width:error] test error message");
}

#[test]
fn width_diagnostic_info_display_format() {
    let d = WidthDiag::info("test info message");
    assert_eq!(format!("{}", d), "[width:info] test info message");
}

// ===========================================================================
// 13. FULL PROGRAM INFERENCE
// ===========================================================================

#[test]
fn program_width_inference_basic() {
    use nasa_rust_project::ast::program::*;

    let program = nasa_rust_project::MirrProgram {
        module: Module {
            name: "test_mod".to_string(),
            signals: vec![
                sig("in_a", SignalType::Unsigned(8)),
                sig("out_b", SignalType::Unsigned(16)),
            ],
            guards: vec![Guard {
                name: "g1".to_string(),
                condition: binary(BinaryOp::Lt, signal("in_a"), lit(100)),
                cycles: 1,
            }],
            reflexes: vec![Reflex {
                name: "r1".to_string(),
                guard_names: vec!["g1".to_string()],
                assignments: vec![Assignment {
                    target: "out_b".to_string(),
                    value: signal("in_a"),
                }],
            }],
        },
    };

    let result = width::infer_program_widths(&program);
    assert!(!result.has_errors());
    assert_eq!(result.guard_results.len(), 1);
    assert_eq!(result.assignment_results.len(), 1);
}

#[test]
fn program_detects_truncation_in_reflex() {
    use nasa_rust_project::ast::program::*;

    let program = nasa_rust_project::MirrProgram {
        module: Module {
            name: "trunc_mod".to_string(),
            signals: vec![
                sig("a", SignalType::Unsigned(8)),
                sig("b", SignalType::Unsigned(8)),
                sig("out", SignalType::Unsigned(8)),
            ],
            guards: vec![],
            reflexes: vec![Reflex {
                name: "r1".to_string(),
                guard_names: vec![],
                assignments: vec![Assignment {
                    target: "out".to_string(),
                    value: binary(BinaryOp::Add, signal("a"), signal("b")),
                }],
            }],
        },
    };

    let result = width::infer_program_widths(&program);
    assert!(result.has_errors());
    let all_diags = result.all_diagnostics();
    let errors: Vec<&&WidthDiag> =
        all_diags.iter().filter(|d| d.severity == DiagSeverity::Error).collect();
    assert!(errors.iter().any(|d| d.message.contains("truncates from 9 bits to 8 bits")));
}
