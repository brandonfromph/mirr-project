#[test]
fn test_no_signals_block() {
    let input = "module test_mod {}";
    let expanded = nasa_rust_project::compiler::macro_proc::expand_macros(input);
    let result = nasa_rust_project::parser::parse_mirr(&expanded);
    assert!(
        result.is_ok(),
        "Module with no signals block should be parsed successfully, got: {:?}",
        result.err()
    );
}
