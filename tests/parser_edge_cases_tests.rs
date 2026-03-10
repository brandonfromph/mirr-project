//! Parser edge-case tests.
//!
//! Covers unbalanced parentheses, expr depth limits, standalone subtraction,
//! empty assignment RHS, too many signal tokens, and reflex not-closed errors.

use nasa_rust_project::parse_mirr;
use nasa_rust_project::parser::expr_parser::parse_expression;

/// Helper: parse and return the error message.
fn parse_err(source: &str) -> String {
    let err = parse_mirr(source).expect_err("should fail");
    err.to_string()
}

fn expr_err(input: &str) -> String {
    let err = parse_expression(input).expect_err("should fail");
    err.to_string()
}

// ---------------------------------------------------------------------------
// Expression parser edge cases
// ---------------------------------------------------------------------------

#[test]
fn unbalanced_parens_open_pinned() {
    let msg = expr_err("(a + b");
    assert_eq!(msg, "[E100] Parse error: [E171] Unbalanced parentheses in expression.");
}

#[test]
fn unbalanced_parens_extra_close_pinned() {
    let msg = expr_err("a + b)");
    assert_eq!(msg, "[E100] Parse error: [E171] Unbalanced parentheses in expression.");
}

#[test]
fn subtraction_standalone_parses_correctly() {
    let expr = parse_expression("a - b").expect("should parse");
    match &expr {
        nasa_rust_project::ast::Expr::Binary { op, .. } => {
            assert_eq!(*op, nasa_rust_project::ast::types::BinaryOp::Sub);
        }
        other => panic!("Expected Binary Sub, got: {other:?}"),
    }
}

#[test]
fn multiplication_standalone_parses_correctly() {
    let expr = parse_expression("a * b").expect("should parse");
    match &expr {
        nasa_rust_project::ast::Expr::Binary { op, .. } => {
            assert_eq!(*op, nasa_rust_project::ast::types::BinaryOp::Mul);
        }
        other => panic!("Expected Binary Mul, got: {other:?}"),
    }
}

#[test]
fn double_nested_parens_parse_correctly() {
    let expr = parse_expression("((a))").expect("should parse");
    match &expr {
        nasa_rust_project::ast::Expr::Signal(name) => {
            assert_eq!(name, "a");
        }
        other => panic!("Expected Signal('a'), got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Module parser edge cases
// ---------------------------------------------------------------------------

#[test]
fn too_many_tokens_in_signal_pinned() {
    let source = r#"
module too_many {
    signal x: in bool extra;
}
"#;
    let msg = parse_err(source);
    // MEGA-1 tokenizer gives a more specific error than the old E114.
    assert!(msg.contains("[E183]"), "Expected E183 (unexpected token), got: {msg}");
    assert!(msg.contains("extra"), "Error should mention the unexpected token 'extra', got: {msg}");
}

#[test]
fn assignment_empty_rhs_pinned() {
    let source = r#"
module empty_rhs {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = ;
        }
    }
}
"#;
    let msg = parse_err(source);
    assert!(
        msg.contains("Assignment to 'b' has empty right-hand side."),
        "expected empty RHS error, got: {msg}"
    );
}

#[test]
fn guard_missing_for_clause_pinned() {
    let source = r#"
module no_for {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
    }

    reflex r {
        on g {
            b = true;
        }
    }
}
"#;
    let msg = parse_err(source);
    // The closing } will be parsed as the for-line, triggering the "expected 'for'" error
    assert!(
        msg.contains("expected 'for'") || msg.contains("missing 'for' clause"),
        "expected for-clause error, got: {msg}"
    );
}

#[test]
fn reflex_not_closed_pinned() {
    let source = r#"
module unclosed_reflex {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = true;
    }
"#;
    let msg = parse_err(source);
    // The reflex or module not-closed error
    assert!(
        msg.contains("not closed with '}'") || msg.contains("was not closed"),
        "expected unclosed error, got: {msg}"
    );
}

#[test]
fn inline_comment_in_assignment_stripped() {
    let source = r#"
module inline_cmt {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = true; // this is a comment
        }
    }
}
"#;
    let program = parse_mirr(source).expect("should parse");
    let assignments = &program.module.reflexes[0].assignments;
    assert_eq!(assignments.len(), 1);
    assert_eq!(assignments[0].target, "b");
}
