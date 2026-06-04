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
    let result = parse_mirr(input);
    assert!(
        result.is_ok(),
        "Comments inside signals block should be parsed successfully, got: {:?}",
        result.err()
    );
}
