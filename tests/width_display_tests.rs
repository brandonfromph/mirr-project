use mirrc::ast::types::{BinaryOp, UnaryOp};
use mirrc::width::display::{format_scc_report, format_stats, format_width_expr};
use mirrc::width::types::{
    DiagSeverity, SccInfo, SccKind, Width, WidthDiag, WidthExpr, WidthStats,
};

#[test]
fn test_format_width_expr_literal() {
    let expr = WidthExpr::Literal { value: 42, width: Width(8) };
    assert_eq!(format_width_expr(&expr), "42:u8");
}

#[test]
fn test_format_width_expr_signal() {
    let expr = WidthExpr::Signal { name: "clk".to_string(), width: Width(1) };
    assert_eq!(format_width_expr(&expr), "clk:u1");
}

#[test]
fn test_format_width_expr_unary() {
    let operand = Box::new(WidthExpr::Signal { name: "a".to_string(), width: Width(4) });
    let expr = WidthExpr::Unary { op: UnaryOp::Not, operand, width: Width(4) };
    assert_eq!(format_width_expr(&expr), "(Nota:u4):u4");
}

#[test]
fn test_format_width_expr_binary() {
    let left = Box::new(WidthExpr::Signal { name: "a".to_string(), width: Width(4) });
    let right = Box::new(WidthExpr::Literal { value: 2, width: Width(4) });
    let expr = WidthExpr::Binary { op: BinaryOp::Add, left, right, width: Width(5) };
    assert_eq!(format_width_expr(&expr), "(a:u4 + 2:u4):u5");
}

#[test]
fn test_format_width_expr_binary_exhaustive() {
    let ops = vec![
        (BinaryOp::Add, "+"),
        (BinaryOp::Sub, "-"),
        (BinaryOp::Mul, "*"),
        (BinaryOp::Shl, "<<"),
        (BinaryOp::Shr, ">>"),
        (BinaryOp::And, "&"),
        (BinaryOp::Or, "|"),
        (BinaryOp::BitwiseOr, "|"),
        (BinaryOp::BitwiseAnd, "&"),
        (BinaryOp::Xor, "^"),
        (BinaryOp::Lt, "<"),
        (BinaryOp::Le, "<="),
        (BinaryOp::Gt, ">"),
        (BinaryOp::Ge, ">="),
        (BinaryOp::Eq, "=="),
        (BinaryOp::Ne, "!="),
    ];
    for (op, op_str) in ops {
        let left = Box::new(WidthExpr::Signal { name: "a".to_string(), width: Width(4) });
        let right = Box::new(WidthExpr::Literal { value: 2, width: Width(4) });
        let expr = WidthExpr::Binary { op, left, right, width: Width(5) };
        assert_eq!(format_width_expr(&expr), format!("(a:u4 {} 2:u4):u5", op_str));
    }
}

#[test]
fn test_format_width_expr_prev() {
    let expr = WidthExpr::Prev { signal: "a".to_string(), delay: 1, width: Width(4) };
    assert_eq!(format_width_expr(&expr), "prev(a, 1):u4");
}

#[test]
fn test_format_width_expr_deep() {
    let mut expr = WidthExpr::Signal { name: "a".to_string(), width: Width(1) };
    for _ in 0..2000 {
        expr = WidthExpr::Unary { op: UnaryOp::Not, operand: Box::new(expr), width: Width(1) };
    }
    // Just ensure it doesn't panic on recursion
    let formatted = format_width_expr(&expr);
    assert!(formatted.is_empty());
}

#[test]
fn test_format_stats() {
    let stats = WidthStats {
        nodes_analyzed: 10,
        propagation_rounds: 2,
        diagnostics_count: 1,
        scc_count: 3,
        expansive_count: 1,
        nonexpansive_count: 2,
    };
    let out = format_stats(&stats);
    assert_eq!(out, "nodes=10 rounds=2 diagnostics=1 sccs=3 expansive=1 nonexpansive=2");
}

#[test]
fn test_format_scc_report_empty() {
    let sccs = vec![];
    let names = vec!["a".to_string(), "b".to_string()];
    assert_eq!(format_scc_report(&sccs, &names), "No non-trivial SCCs detected.");
}

#[test]
fn test_format_scc_report_nonempty() {
    let sccs = vec![
        SccInfo { signal_indices: vec![0, 1], kind: SccKind::Expansive },
        SccInfo { signal_indices: vec![2], kind: SccKind::Nonexpansive },
    ];
    let names = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let out = format_scc_report(&sccs, &names);
    assert!(out.contains("SCCs detected: 2"));
    assert!(out.contains("SCC 0: expansive [a, b]"));
    assert!(out.contains("SCC 1: nonexpansive [c]"));
}

#[test]
fn test_format_scc_report_truncated() {
    let mut sccs = vec![];
    for _ in 0..300 {
        sccs.push(SccInfo { signal_indices: vec![0], kind: SccKind::Nonexpansive });
    }
    let names = vec!["a".to_string()];
    let out = format_scc_report(&sccs, &names);
    assert!(out.contains("... (truncated)"));
}

#[test]
fn test_width_display() {
    assert_eq!(format!("{}", Width(32)), "u32");
}

#[test]
fn test_width_display_with_sign() {
    assert_eq!(Width(32).display_with_sign(true), "i32");
    assert_eq!(Width(32).display_with_sign(false), "u32");
}

#[test]
fn test_width_diag_display() {
    let diag = WidthDiag {
        severity: DiagSeverity::Error,
        message: "bad".to_string(),
        code: Some("E123".to_string()),
        span: None,
        signal_name: None,
        help: None,
    };
    assert_eq!(format!("{}", diag), "[width:error E123] bad");

    let diag2 = WidthDiag {
        severity: DiagSeverity::Warning,
        message: "warn".to_string(),
        code: None,
        span: None,
        signal_name: None,
        help: None,
    };
    assert_eq!(format!("{}", diag2), "[width:warning] warn");
}

#[test]
fn test_width_diag_builders_and_to_diagnostic() {
    let span = mirrc::span::Span::single_line(1, 1, 10);
    let diag_warn = WidthDiag::error("test warning")
        .with_span(Some(span))
        .with_signal("clk")
        .with_help("check your clock");

    let std_diag = diag_warn.to_diagnostic();
    assert!(matches!(std_diag.severity, mirrc::diagnostic::Severity::Error));
    assert_eq!(std_diag.span, Some(span));
    assert!(std_diag.labels.iter().any(|l| l.message == "check your clock"));

    let diag_info = WidthDiag::info("test info");
    let std_diag_info = diag_info.to_diagnostic();
    assert!(matches!(std_diag_info.severity, mirrc::diagnostic::Severity::Info));
}
