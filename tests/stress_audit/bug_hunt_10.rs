// Bug Hunt #10: Check if the AST parsing handles trailing garbage gracefully or fails accurately.
use nasa_rust_project::parser::parse_mirr;
#[test]
fn test_garbage_after_module_close() {
    let input = "module test { signal a: in bool; } GARBAGE_DATA";
    let result = parse_mirr(input);
    assert!(
        result.is_ok(),
        "Parser should ignore trailing garbage data under single-module design"
    );
}
