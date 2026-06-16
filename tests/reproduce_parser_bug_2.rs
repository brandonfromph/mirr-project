use mirrc::parser::parse_mirr;

#[test]
fn reproduce_structural_desync_with_multiple_closing_braces_on_one_line() {
    // If the closing braces are on one line, the naive parser might fail to see them
    // or fail to consume them correctly.
    let src = r#"
        def my_pattern(x: u16) {
            reflect {
                guard g { when x for 1 cycles; } } }
        module m {}
    "#;

    let result = parse_mirr(src);

    assert!(
        result.is_ok(),
        "Structural desync: multiple closing braces on one line failed to parse. Error: {:?}",
        result.err()
    );

    let program = result.expect("checked");
    assert_eq!(
        program.module.name, "m",
        "Module 'm' should have been parsed, but wasn't (likely skipped due to structural desync)"
    );
}
