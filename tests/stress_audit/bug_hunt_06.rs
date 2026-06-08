use mirrc::parser::parse_mirr;
#[test]
fn test_no_signals_block() {
    let input = "module test_mod {}";
    let result = parse_mirr(input);
    assert!(
        result.is_ok(),
        "Module with no signals block should be parsed successfully, got: {:?}",
        result.err()
    );
}
