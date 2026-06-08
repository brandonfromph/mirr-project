use mirrc::parser::parse_mirr;
#[test]
fn test_empty_signals_whitespace_resilience() {
    let input = "module test_mod\t{\r\nsignals\r\n{\r\n}\r\n}";
    let result = parse_mirr(input);
    assert!(
        result.is_ok(),
        "Empty signals block with carriage returns/tabs should be parsed successfully, got: {:?}",
        result.err()
    );
}
