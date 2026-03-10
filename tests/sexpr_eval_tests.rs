//! Tests for S-expression eval, macros, and reader macros.

use nasa_rust_project::sexpr::eval::{eval, EvalState};
use nasa_rust_project::sexpr::macro_expand::MacroExpander;
use nasa_rust_project::sexpr::parser::parse_sexpr;
use nasa_rust_project::sexpr::reader::ReaderMacroRegistry;
use nasa_rust_project::sexpr::types::SExpr;

// =========================================================================
// Eval Tests
// =========================================================================

fn eval_text(input: &str) -> SExpr {
    let expr = parse_sexpr(input).unwrap();
    let mut state = EvalState::new();
    eval(&expr, &mut state).unwrap()
}

fn eval_text_err(input: &str) -> String {
    let expr = parse_sexpr(input).unwrap();
    let mut state = EvalState::new();
    eval(&expr, &mut state).unwrap_err().to_string()
}

#[test]
fn eval_integer_self() {
    assert_eq!(eval_text("42"), SExpr::Integer(42));
}

#[test]
fn eval_bool_self() {
    assert_eq!(eval_text("true"), SExpr::Bool(true));
}

#[test]
fn eval_string_self() {
    assert_eq!(eval_text("\"hello\""), SExpr::Str("hello".to_string()));
}

#[test]
fn eval_quote() {
    let result = eval_text("'(a b)");
    assert_eq!(result, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b")]));
}

#[test]
fn eval_if_true() {
    assert_eq!(eval_text("(if true 1 2)"), SExpr::Integer(1));
}

#[test]
fn eval_if_false() {
    assert_eq!(eval_text("(if false 1 2)"), SExpr::Integer(2));
}

#[test]
fn eval_car() {
    assert_eq!(eval_text("(car (list 1 2 3))"), SExpr::Integer(1));
}

#[test]
fn eval_cdr() {
    assert_eq!(
        eval_text("(cdr (list 1 2 3))"),
        SExpr::list(vec![SExpr::Integer(2), SExpr::Integer(3)])
    );
}

#[test]
fn eval_cons() {
    assert_eq!(
        eval_text("(cons 1 (list 2 3))"),
        SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2), SExpr::Integer(3)])
    );
}

#[test]
fn eval_eq_true() {
    assert_eq!(eval_text("(eq? 42 42)"), SExpr::Bool(true));
}

#[test]
fn eval_eq_false() {
    assert_eq!(eval_text("(eq? 1 2)"), SExpr::Bool(false));
}

#[test]
fn eval_symbol_pred() {
    assert_eq!(eval_text("(symbol? '(foo))"), SExpr::Bool(false));
    // A quoted symbol:
    assert_eq!(eval_text("(symbol? 'foo)"), SExpr::Bool(true));
}

#[test]
fn eval_list_pred() {
    assert_eq!(eval_text("(list? (list 1 2))"), SExpr::Bool(true));
    assert_eq!(eval_text("(list? 42)"), SExpr::Bool(false));
}

#[test]
fn eval_integer_pred() {
    assert_eq!(eval_text("(integer? 42)"), SExpr::Bool(true));
    assert_eq!(eval_text("(integer? true)"), SExpr::Bool(false));
}

#[test]
fn eval_bool_pred() {
    assert_eq!(eval_text("(bool? true)"), SExpr::Bool(true));
    assert_eq!(eval_text("(bool? 42)"), SExpr::Bool(false));
}

#[test]
fn eval_nested_if() {
    assert_eq!(eval_text("(if true (if false 1 2) 3)"), SExpr::Integer(2));
}

#[test]
fn eval_list_construction() {
    let result = eval_text("(list 1 2 3)");
    assert_eq!(result, SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2), SExpr::Integer(3)]));
}

#[test]
fn eval_depth_limit() {
    // Create a very deeply nested eval by using enormous state.
    let expr = parse_sexpr("42").unwrap();
    let mut state = EvalState::new();
    state.depth = 100; // Force past limit.
    let result = eval(&expr, &mut state);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("E811") || msg.contains("DEPTH"), "got: {msg}");
}

#[test]
fn eval_step_limit() {
    let expr = parse_sexpr("42").unwrap();
    let mut state = EvalState::with_steps(0);
    let result = eval(&expr, &mut state);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("E812") || msg.contains("STEPS"), "got: {msg}");
}

#[test]
fn eval_undefined_symbol() {
    let msg = eval_text_err("undefined_var");
    assert!(msg.contains("E813") || msg.contains("Undefined"), "got: {msg}");
}

#[test]
fn eval_empty_list() {
    assert_eq!(eval_text("()"), SExpr::list(vec![]));
}

// =========================================================================
// Macro Expander Tests
// =========================================================================

#[test]
fn hygiene_no_capture() {
    let mut expander = MacroExpander::new();
    // Template with an internal name.
    let template = SExpr::list(vec![SExpr::sym("signal"), SExpr::str_val("internal_name")]);
    let result = expander.expand_hygienic(&template, &["param1".to_string()], &[], 0).unwrap();
    // The internal name should get a hygiene suffix.
    match &result {
        SExpr::List(items) => {
            let name_str = items[1].as_str_val().unwrap();
            assert!(
                name_str.contains("__hyg"),
                "Internal name should have hygiene suffix: {name_str}"
            );
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn hygiene_two_expansions() {
    let mut expander = MacroExpander::new();
    let template = SExpr::list(vec![SExpr::sym("guard"), SExpr::str_val("temp")]);

    let result1 = expander.expand_hygienic(&template, &[], &[], 0).unwrap();
    let result2 = expander.expand_hygienic(&template, &[], &[], 0).unwrap();

    // Two expansions should produce different hygiene suffixes.
    assert_ne!(result1, result2, "Two expansions must have different hygiene marks");
}

#[test]
fn hygiene_param_passthrough() {
    let mut expander = MacroExpander::new();
    let template = SExpr::list(vec![SExpr::sym("signal"), SExpr::str_val("sensor")]);
    let bindings = vec![("sensor".to_string(), SExpr::str_val("temp_a"))];
    let result =
        expander.expand_hygienic(&template, &["sensor".to_string()], &bindings, 0).unwrap();

    // Parameter should be replaced by its binding.
    match &result {
        SExpr::List(items) => {
            assert_eq!(items[1].as_str_val(), Some("temp_a"));
        }
        _ => panic!("Expected list"),
    }
}

#[test]
fn expand_depth_limit() {
    let mut expander = MacroExpander::new();
    let template = SExpr::sym("x");
    let result = expander.expand_hygienic(&template, &[], &[], 100);
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("E814") || msg.contains("depth"), "got: {msg}");
}

// =========================================================================
// Reader Macro Tests
// =========================================================================

#[test]
fn reader_freq_mhz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "100MHz").unwrap();
    assert_eq!(result, SExpr::list(vec![SExpr::sym("frequency"), SExpr::int(100_000_000)]));
}

#[test]
fn reader_freq_khz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "50KHz").unwrap();
    assert_eq!(result, SExpr::list(vec![SExpr::sym("frequency"), SExpr::int(50_000)]));
}

#[test]
fn reader_freq_ghz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "1GHz").unwrap();
    assert_eq!(result, SExpr::list(vec![SExpr::sym("frequency"), SExpr::int(1_000_000_000)]));
}

#[test]
fn reader_freq_hz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "48000Hz").unwrap();
    assert_eq!(result, SExpr::list(vec![SExpr::sym("frequency"), SExpr::int(48_000)]));
}

#[test]
fn reader_delay() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("delay", "5").unwrap();
    assert_eq!(result, SExpr::list(vec![SExpr::sym("temporal-delay"), SExpr::int(5)]));
}

#[test]
fn reader_range() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("range", "0..1023").unwrap();
    assert_eq!(
        result,
        SExpr::list(vec![SExpr::sym("refinement-range"), SExpr::int(0), SExpr::int(1023),])
    );
}

#[test]
fn reader_unknown() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("unknown", "x");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("E815") || msg.contains("Unknown reader"), "got: {msg}");
}

#[test]
fn reader_registry_builtin_count() {
    let reg = ReaderMacroRegistry::new();
    assert_eq!(reg.len(), 3); // freq, delay, range
}

#[test]
fn reader_registry_limit() {
    let mut reg = ReaderMacroRegistry::new();
    // Register up to the limit (already 3 built-in).
    for i in 3..32 {
        let name = format!("macro_{i}");
        reg.register(&name, |_| Ok(SExpr::sym("ok"))).unwrap();
    }
    // The 33rd should fail.
    let result = reg.register("overflow", |_| Ok(SExpr::sym("ok")));
    assert!(result.is_err());
}

// =========================================================================
// Error Code Tests
// =========================================================================

#[test]
fn error_e800_bad_token() {
    let result = parse_sexpr("#x");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("E800"), "got: {msg}");
}

#[test]
fn error_e801_truncated() {
    let result = parse_sexpr("");
    assert!(result.is_err());
}

#[test]
fn error_e802_unbalanced() {
    let result = parse_sexpr("(a b");
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("E802") || msg.contains("paren"), "got: {msg}");
}

#[test]
fn error_e803_depth() {
    let input = "(".repeat(65) + &")".repeat(65);
    let result = parse_sexpr(&input);
    assert!(result.is_err());
}
