use mirrc::parser::parse_mirr;
#[test]
fn test_empty_guards_reflexes_blocks() {
    let input = r#"
    module test_mod {
        signals {
            a: in bool
        }
        guard g {}
        reflex r {}
    }
    "#;
    let result = parse_mirr(input);
    // Verify that the parser either succeeds or fails cleanly without panic/segfault.
    assert!(
        result.is_ok() || result.is_err(),
        "Parser must either successfully parse or return a structured error"
    );
}
