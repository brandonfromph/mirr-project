//! S-expression types and pretty-printer unit tests.
//!
//! Exercises uncovered branches in `src/sexpr/types.rs` and `src/sexpr/printer.rs`:
//!   - SExpr: as_bool, as_str_val, head_symbol, node_count with Quote/Quasiquote/Unquote
//!   - Display: Quote, Quasiquote, Unquote, Bool(false), Str, List
//!   - Printer: multi-line lists, Quote/Quasiquote/Unquote printing, empty list

#![forbid(unsafe_code)]

use mirrc::sexpr::types::SExpr;
use mirrc::sexpr::printer::print_sexpr;

// -----------------------------------------------------------------------
// SExpr accessor coverage
// -----------------------------------------------------------------------
#[test]
fn as_bool_returns_value_for_bool() {
    assert_eq!(SExpr::Bool(true).as_bool(), Some(true));
    assert_eq!(SExpr::Bool(false).as_bool(), Some(false));
}

#[test]
fn as_bool_returns_none_for_non_bool() {
    assert_eq!(SExpr::int(42).as_bool(), None);
    assert_eq!(SExpr::sym("test").as_bool(), None);
}

#[test]
fn as_str_val_returns_value_for_str() {
    let s = SExpr::str_val("hello");
    assert_eq!(s.as_str_val(), Some("hello"));
}

#[test]
fn as_str_val_returns_none_for_non_str() {
    assert_eq!(SExpr::int(42).as_str_val(), None);
}

#[test]
fn head_symbol_returns_first_element_if_symbol() {
    let list = SExpr::list(vec![SExpr::sym("signal"), SExpr::str_val("clk")]);
    assert_eq!(list.head_symbol(), Some("signal"));
}

#[test]
fn head_symbol_returns_none_for_empty_list() {
    let list = SExpr::list(vec![]);
    assert_eq!(list.head_symbol(), None);
}

#[test]
fn head_symbol_returns_none_for_non_symbol_head() {
    let list = SExpr::list(vec![SExpr::int(42), SExpr::sym("signal")]);
    assert_eq!(list.head_symbol(), None);
}

#[test]
fn head_symbol_returns_none_for_non_list() {
    let sym = SExpr::sym("not_a_list");
    assert_eq!(sym.head_symbol(), None);
}

// -----------------------------------------------------------------------
// node_count with Quote/Quasiquote/Unquote
// -----------------------------------------------------------------------
#[test]
fn node_count_quote_counts_inner() {
    let expr = SExpr::Quote(Box::new(SExpr::sym("x")));
    assert_eq!(expr.node_count(), 2); // Quote + Symbol
}

#[test]
fn node_count_quasiquote_counts_inner() {
    let expr = SExpr::Quasiquote(Box::new(SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b")])));
    assert_eq!(expr.node_count(), 4); // Quasiquote + List + Symbol + Symbol
}

#[test]
fn node_count_unquote_counts_inner() {
    let expr = SExpr::Unquote(Box::new(SExpr::int(42)));
    assert_eq!(expr.node_count(), 2);
}

#[test]
fn node_count_nested_list() {
    let expr = SExpr::list(vec![
        SExpr::sym("outer"),
        SExpr::list(vec![SExpr::sym("inner"), SExpr::int(1)]),
    ]);
    assert_eq!(expr.node_count(), 5); // outer_list + sym + inner_list + sym + int
}

// -----------------------------------------------------------------------
// Display coverage: Quote, Quasiquote, Unquote, Bool(false), Str
// -----------------------------------------------------------------------
#[test]
fn display_quote() {
    let expr = SExpr::Quote(Box::new(SExpr::sym("x")));
    assert_eq!(format!("{}", expr), "'x");
}

#[test]
fn display_quasiquote() {
    let expr = SExpr::Quasiquote(Box::new(SExpr::sym("template")));
    assert_eq!(format!("{}", expr), "`template");
}

#[test]
fn display_unquote() {
    let expr = SExpr::Unquote(Box::new(SExpr::sym("val")));
    assert_eq!(format!("{}", expr), ",val");
}

#[test]
fn display_bool_false() {
    assert_eq!(format!("{}", SExpr::Bool(false)), "false");
}

#[test]
fn display_bool_true() {
    assert_eq!(format!("{}", SExpr::Bool(true)), "true");
}

#[test]
fn display_str() {
    assert_eq!(format!("{}", SExpr::str_val("hello world")), "\"hello world\"");
}

#[test]
fn display_list_with_spaces() {
    let list = SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]);
    assert_eq!(format!("{}", list), "(a b c)");
}

#[test]
fn display_empty_list() {
    assert_eq!(format!("{}", SExpr::list(vec![])), "()");
}

// -----------------------------------------------------------------------
// Pretty-printer: multi-line lists
// -----------------------------------------------------------------------
#[test]
fn printer_empty_list() {
    let expr = SExpr::list(vec![]);
    assert_eq!(print_sexpr(&expr), "()");
}

#[test]
fn printer_short_list_on_one_line() {
    let expr = SExpr::list(vec![SExpr::sym("add"), SExpr::int(1), SExpr::int(2)]);
    let output = print_sexpr(&expr);
    assert_eq!(output, "(add 1 2)");
    assert!(!output.contains('\n'));
}

#[test]
fn printer_long_list_multiline() {
    // Create a list with many long children to force multi-line
    let mut items = Vec::new();
    items.push(SExpr::sym("very_long_function_name_that_pushes_the_line_length"));
    for i in 0..5 {
        items.push(SExpr::str_val(&format!("argument_{}_with_extra_padding_text", i)));
    }
    let expr = SExpr::list(items);
    let output = print_sexpr(&expr);
    assert!(output.contains('\n'), "Long list should be multi-line, got: {}", output);
}

#[test]
fn printer_quote_expression() {
    let expr = SExpr::Quote(Box::new(SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b")])));
    let output = print_sexpr(&expr);
    assert!(output.starts_with("'("), "got: {}", output);
}

#[test]
fn printer_quasiquote_expression() {
    let expr = SExpr::Quasiquote(Box::new(SExpr::sym("template")));
    let output = print_sexpr(&expr);
    assert_eq!(output, "`template");
}

#[test]
fn printer_unquote_expression() {
    let expr = SExpr::Unquote(Box::new(SExpr::int(42)));
    let output = print_sexpr(&expr);
    assert_eq!(output, ",42");
}

#[test]
fn printer_nested_structure() {
    let expr = SExpr::list(vec![
        SExpr::sym("module"),
        SExpr::str_val("core"),
        SExpr::list(vec![
            SExpr::sym("signal"),
            SExpr::str_val("clk"),
            SExpr::sym("input"),
            SExpr::sym("bool"),
        ]),
    ]);
    let output = print_sexpr(&expr);
    assert!(output.contains("module"));
    assert!(output.contains("signal"));
    assert!(output.contains("clk"));
}

// -----------------------------------------------------------------------
// Predicate coverage
// -----------------------------------------------------------------------
#[test]
fn is_str_true_for_str() {
    assert!(SExpr::str_val("x").is_str());
}

#[test]
fn is_str_false_for_symbol() {
    assert!(!SExpr::sym("x").is_str());
}

#[test]
fn is_bool_true_for_bool() {
    assert!(SExpr::Bool(true).is_bool());
}

#[test]
fn is_bool_false_for_integer() {
    assert!(!SExpr::int(1).is_bool());
}

#[test]
fn is_atom_true_for_atoms() {
    assert!(SExpr::sym("x").is_atom());
    assert!(SExpr::int(1).is_atom());
    assert!(SExpr::Bool(true).is_atom());
    assert!(SExpr::str_val("s").is_atom());
}

#[test]
fn is_atom_false_for_list() {
    assert!(!SExpr::list(vec![]).is_atom());
}

#[test]
fn is_atom_false_for_quote() {
    assert!(!SExpr::Quote(Box::new(SExpr::sym("x"))).is_atom());
}
