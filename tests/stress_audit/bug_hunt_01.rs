// Bug Hunt #1: Ergonomic macro expansion of empty signals block should not crash the parser.
use nasa_rust_project::compiler::macro_proc::expand_macros;
use nasa_rust_project::parser::parse_mirr;

#[test]
fn test_empty_signals_block() {
    let input = "module test_mod { signals {} }";
    let expanded = expand_macros(input);
    let result = parse_mirr(&expanded);
    assert!(result.is_ok(), "Empty signals block should be valid");
}
