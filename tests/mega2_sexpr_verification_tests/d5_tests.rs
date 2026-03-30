use super::*;

// ===========================================================================
// D5: macro_expand_hygienic (10 tests)
// ===========================================================================

#[test]
fn test_d5_macro_expander_new() {
    let me = MacroExpander::new();
    // MacroExpander starts with expansion_counter = 0.
    // Just verify it can be constructed without panicking.
    let _ = me;
}

#[test]
fn test_d5_macro_expand_hygienic_simple() {
    let mut me = MacroExpander::new();
    // Template: (signal "sensor_name") where sensor_name is a parameter
    let template = SExpr::list(vec![SExpr::sym("signal"), SExpr::Str("sensor".to_string())]);
    let param_names = vec!["sensor".to_string()];
    let bindings = vec![("sensor".to_string(), SExpr::Str("temp_a".to_string()))];
    let result = me.expand_hygienic(&template, &param_names, &bindings, 0);
    assert!(result.is_ok(), "Hygienic expand should succeed");
}

#[test]
fn test_d5_macro_expand_hygienic_substitution() {
    let mut me = MacroExpander::new();
    // Template with a param that should be substituted
    let template = SExpr::Str("target".to_string());
    let param_names = vec!["target".to_string()];
    let bindings = vec![("target".to_string(), SExpr::Str("clamp_valve".to_string()))];
    let result = me.expand_hygienic(&template, &param_names, &bindings, 0).unwrap();
    assert_eq!(result, SExpr::Str("clamp_valve".to_string()), "Param must be substituted");
}

#[test]
fn test_d5_macro_expand_hygienic_renames_internal() {
    let mut me = MacroExpander::new();
    // Template with an internal name not in params — should get hygiene suffix
    let template = SExpr::Str("internal_var".to_string());
    let param_names: Vec<String> = vec![];
    let bindings: Vec<(String, SExpr)> = vec![];
    let result = me.expand_hygienic(&template, &param_names, &bindings, 0).unwrap();
    // Should be renamed to internal_var__hyg1
    match &result {
        SExpr::Str(s) => {
            assert!(s.starts_with("internal_var__hyg"), "Internal name must be renamed")
        }
        _ => panic!("Expected Str"),
    }
}

#[test]
fn test_d5_macro_expand_hygienic_depth_limit() {
    let mut me = MacroExpander::new();
    let template = SExpr::Integer(42);
    let result = me.expand_hygienic(&template, &[], &[], 999);
    assert!(result.is_err(), "Exceeding depth limit should error");
}

#[test]
fn test_d5_macro_expand_hygienic_atom_passthrough() {
    let mut me = MacroExpander::new();
    // Integer and Bool pass through unchanged.
    let r_int = me.expand_hygienic(&SExpr::Integer(42), &[], &[], 0).unwrap();
    assert_eq!(r_int, SExpr::Integer(42));
    let r_bool = me.expand_hygienic(&SExpr::Bool(true), &[], &[], 0).unwrap();
    assert_eq!(r_bool, SExpr::Bool(true));
}

#[test]
fn test_d5_macro_expand_hygienic_symbol_not_renamed() {
    let mut me = MacroExpander::new();
    // Symbols (structural tags) should not be renamed.
    let template = SExpr::sym("signal");
    let result = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    assert_eq!(result, SExpr::sym("signal"), "Symbols must not be renamed");
}

#[test]
fn test_d5_macro_expand_hygienic_list() {
    let mut me = MacroExpander::new();
    let template = SExpr::list(vec![SExpr::sym("guard"), SExpr::Str("my_guard".to_string())]);
    let result = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    // The list structure should be preserved, "my_guard" renamed.
    match &result {
        SExpr::List(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], SExpr::sym("guard"));
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn test_d5_macro_expand_hygienic_quote_preserved() {
    let mut me = MacroExpander::new();
    let template = SExpr::Quote(Box::new(SExpr::Str("inner".to_string())));
    let result = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    match &result {
        SExpr::Quote(_) => {} // quote is preserved
        _ => panic!("Expected Quote wrapper to be preserved"),
    }
}

#[test]
fn test_d5_macro_expand_counter_increments() {
    let mut me = MacroExpander::new();
    let template = SExpr::Str("name".to_string());
    let r1 = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    let r2 = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    // Each expansion gets a different hygiene ID, so results differ.
    assert_ne!(r1, r2, "Each expansion must use a unique hygiene ID");
}
