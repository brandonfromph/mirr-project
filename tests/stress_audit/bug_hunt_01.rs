// Bug Hunt #1: Ergonomic macro expansion of empty signals block should not crash the parser.
use nasa_rust_project::parser::parse_mirr;

#[test]
fn test_empty_signals_block() {
    let input = "module test_mod { signals {} }";
    let result = parse_mirr(input);
    assert!(result.is_ok(), "Empty signals block should be valid");
}
