use mirrc::parser::parse_mirr;

#[test]
fn reproduce_structural_desync_with_comment_brace() {
    let src = r#"
        def my_pattern(x: u16) {
            reflect {
                // This comment has a closing brace } that should be ignored
                guard g { when x for 1 cycles; }
            }
        }
        module m {}
    "#;

    let result = parse_mirr(src);

    // If it's buggy, it will fail to parse because the '}' in the comment
    // will be treated as the end of the 'reflect' block.
    assert!(
        result.is_ok(),
        "Structural desync: comment brace prematurely terminated block. Error: {:?}",
        result.err()
    );
}
