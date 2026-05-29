// Bug Hunt #12: Check if duplicate signal names in the same module are caught during semantic analysis.
use nasa_rust_project::parser::parse_mirr;
use nasa_rust_project::validate_module;
#[test]
fn test_duplicate_signal_names() {
    let input = "module test { signal a: in bool; signal a: out u8; }";
    let program = parse_mirr(input).unwrap();
    let result = validate_module(&program.module);
    assert!(result.is_err(), "AST validator should reject duplicate signal names");
    let err_msg = result.err().unwrap().to_string();
    assert!(err_msg.contains("[E201]"), "Expected E201 name collision, got: {}", err_msg);
}
