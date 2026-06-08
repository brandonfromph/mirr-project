use mirrc::parser::parse_mirr;
#[test]
fn test_signals_block_syntax_error() {
    let input = "module test_mod { signals { invalid_syntax } }";
    let result = parse_mirr(input);
    assert!(result.is_err(), "Signals block with invalid syntax must be rejected by the parser");
}
