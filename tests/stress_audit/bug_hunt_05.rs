#[test]
fn test_signals_block_syntax_error() {
    let input = "module test_mod { signals { invalid_syntax } }";
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(input);
    let result = nasa_rust_project::parser::parse_mirr(&expanded);
    assert!(result.is_err(), "Signals block with invalid syntax must be rejected by the parser");
}
