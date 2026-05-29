#[test]
fn test_macro_expansion_preserves_standard_signals() {
    let input = r#"
    module test_mod {
        signals {
            a: in bool
            b: out u8
        }
    }
    "#;
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(input);
    assert!(
        expanded.contains("a: in bool") || expanded.contains("a : in bool"),
        "Signal 'a' must be preserved"
    );
    assert!(
        expanded.contains("b: out u8") || expanded.contains("b : out u8"),
        "Signal 'b' must be preserved"
    );
    let result = nasa_rust_project::parser::parse_mirr(&expanded);
    assert!(result.is_ok(), "Preserved signals should parse correctly, got: {:?}", result.err());
}
