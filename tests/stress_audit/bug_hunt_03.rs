use nasa_rust_project::parser::parse_mirr;
#[test]
fn test_signals_block_with_comments() {
    let input = r#"
    module test_mod {
        signals {
            // This is a comment
        }
    }
    "#;
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(input);
    let result = parse_mirr(&expanded);
    assert!(result.is_ok(), "Comments inside signals block should be parsed successfully by stripping them in expand_macros, got: {:?}", result.err());
}
