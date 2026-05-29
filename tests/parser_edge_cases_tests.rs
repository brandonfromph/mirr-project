#![forbid(unsafe_code)]
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

#[test]
fn canonical_prev_parses_to_prev_expr() {
    let expr = parse_expression("prev(sensor, 3)").expect("should parse");
    assert_eq!(expr, nasa_rust_project::ast::Expr::Prev { signal: "sensor".to_string(), delay: 3 });
}

#[test]
fn canonical_prev_zero_delay_parses_for_semantic_validation() {
    let expr = parse_expression("prev(sensor, 0)").expect("should parse");
    assert_eq!(expr, nasa_rust_project::ast::Expr::Prev { signal: "sensor".to_string(), delay: 0 });
}

#[test]
fn canonical_prev_in_binary_expression_parses() {
    let expr = parse_expression("prev(x, 2) > 5").expect("should parse");
    match expr {
        nasa_rust_project::ast::Expr::Binary { op, left, right } => {
            assert_eq!(op, nasa_rust_project::ast::types::BinaryOp::Gt);
            assert_eq!(
                *left,
                nasa_rust_project::ast::Expr::Prev { signal: "x".to_string(), delay: 2 }
            );
            assert_eq!(
                *right,
                nasa_rust_project::ast::Expr::Literal(
                    nasa_rust_project::ast::types::LiteralValue::Integer(5)
                )
            );
        }
        other => panic!("Expected Binary expression, got: {other:?}"),
    }
}

#[test]
fn prev_missing_args_rejected_with_strict_arity_message() {
    let msg = expr_err("prev()");
    assert!(
        msg.contains("prev() expects exactly 2 arguments"),
        "expected strict arity error, got: {msg}"
    );
}

#[test]
fn prev_one_arg_rejected_with_strict_arity_message() {
    let msg = expr_err("prev(x)");
    assert!(
        msg.contains("prev() expects exactly 2 arguments"),
        "expected strict arity error, got: {msg}"
    );
}

#[test]
fn prev_extra_arg_rejected_with_strict_arity_message() {
    let msg = expr_err("prev(x, 1, 2)");
    assert!(
        msg.contains("prev() expects exactly 2 arguments"),
        "expected strict arity error, got: {msg}"
    );
}

#[test]
fn prev_non_signal_first_arg_rejected() {
    let msg = expr_err("prev(1, 2)");
    assert!(
        msg.contains("first argument must be a signal identifier"),
        "expected signal-identifier error, got: {msg}"
    );
}

#[test]
fn prev_non_integer_delay_rejected() {
    let msg = expr_err("prev(x, y)");
    assert!(
        msg.contains("delay must be an integer literal"),
        "expected integer-delay error, got: {msg}"
    );
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
    // MEGA-1 tokenizer gives a more specific error than the original E114.
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
        msg.contains("not closed with '}'")
            || msg.contains("was not closed")
            || msg.contains("unclosed"),
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

#[test]
fn reflex_name_containing_when_substring_parses() {
    let source = r#"
module reflex_name_when_substring {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 1 cycles;
    }

    reflex whenever_alarm {
        on g {
            b = true;
        }
    }
}
"#;

    let program = parse_mirr(source).expect("should parse");
    assert_eq!(program.module.reflexes.len(), 1);
    assert_eq!(program.module.reflexes[0].name, "whenever_alarm");
    assert_eq!(program.module.reflexes[0].guard_names, vec!["g"]);
}

#[test]
fn on_clause_guard_name_containing_and_substring_not_split() {
    let source = r#"
module on_and_substring {
    signal a: in bool;
    signal b: out bool;

    guard gandalf {
        when a
        for 1 cycles;
    }

    guard g {
        when a
        for 1 cycles;
    }

    guard alf {
        when a
        for 1 cycles;
    }

    reflex r {
        on gandalf {
            b = true;
        }
    }
}
"#;

    let program = parse_mirr(source).expect("should parse");
    assert_eq!(program.module.reflexes.len(), 1);
    assert_eq!(program.module.reflexes[0].guard_names, vec!["gandalf"]);
}

#[test]
fn inline_when_guard_name_containing_and_substring_not_split() {
    let source = r#"
module inline_when_and_substring {
    signal a: in bool;
    signal b: out bool;

    guard gandalf {
        when a
        for 1 cycles;
    }

    guard g {
        when a
        for 1 cycles;
    }

    guard alf {
        when a
        for 1 cycles;
    }

    reflex alarm when [gandalf] {
        b = true;
    }
}
"#;

    let program = parse_mirr(source).expect("should parse");
    assert_eq!(program.module.reflexes.len(), 1);
    assert_eq!(program.module.reflexes[0].guard_names, vec!["gandalf"]);
}

#[test]
fn test_empty_signals_block() {
    let input = "module test_mod {\n    signals {}\n}";
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(input);
    let result = nasa_rust_project::parser::parse_mirr(&expanded);
    assert!(result.is_ok(), "Empty signals block should be valid");
}

// ---------------------------------------------------------------------------
// Preprocessor & Parser Parity (5 tests: Tests 11-15)
// ---------------------------------------------------------------------------

#[test]
fn test_preprocessor_bypass_standard_on_clause() {
    let input = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 and g2 {
            b = true;
        }
    }
}
"#;
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(input);
    // Standard `on g1 and g2` clause should remain untouched
    assert!(expanded.contains("on g1 and g2"), "Should preserve on clause");
}

#[test]
fn test_preprocessor_nested_on_clause_extraction() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }
    guard g2 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            on g2 {
                b = true;
            }
        }
    }
}
"#;
    let program = parse_mirr(source).expect("should parse nested on clauses");
    assert_eq!(program.module.reflexes.len(), 1);
    // Flattened guard stack is g1 and g2
    assert_eq!(program.module.reflexes[0].guard_names, vec!["g1", "g2"]);
}

#[test]
fn test_preprocessor_constraint_substitution() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out u8;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            let limit: u8 = 10;
            b = limit;
        }
    }
}
"#;
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(source);
    // Preprocessor should extract local let limit to module internal signal and assign it
    assert!(
        expanded.contains("signal limit: internal u8;"),
        "Should extract let limit to internal signal"
    );
    assert!(expanded.contains("limit = 10;"), "Should assign 10 to limit");
    assert!(expanded.contains("b = limit;"), "Should assign limit to b");
}

#[test]
fn test_preprocessor_multiline_reflex_block() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1
        {
            b = true;
        }
    }
}
"#;
    let program = parse_mirr(source).expect("should reconstruct multiline reflex block");
    assert_eq!(program.module.reflexes.len(), 1);
    assert_eq!(program.module.reflexes[0].assignments[0].target, "b");
}

#[test]
fn test_preprocessor_scoping_variables() {
    let source = r#"
module test_mod {
    signal a: in bool;
    signal out_0: out bool;
    signal out_1: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            for i in 0..2 {
                out_${i} = true;
            }
        }
    }
}
"#;
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(source);
    // Loops should expand and replace scoped variable
    assert!(expanded.contains("out_0 = true"), "Should generate out_0");
    assert!(expanded.contains("out_1 = true"), "Should generate out_1");
    assert!(!expanded.contains("out_${i}"), "Should not leak i outside of scope");
}
