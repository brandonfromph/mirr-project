use nasa_rust_project::parser::parse_mirr;
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
    let result = parse_mirr(input);
    assert!(result.is_ok(), "Preserved signals should parse correctly, got: {:?}", result.err());

    let program = result.unwrap();
    assert!(program.module.signals.iter().any(|s| s.name == "a"));
    assert!(program.module.signals.iter().any(|s| s.name == "b"));
}
