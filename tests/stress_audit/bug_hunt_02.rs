use nasa_rust_project::parser::parse_mirr;
#[test]
fn test_assignment_empty_rhs_fails_with_e100() {
    let input = "module test_mod { signal a: in bool; reflex r { on g { a = ; } } }";
    let err = parse_mirr(input).expect_err("Should fail");
    assert!(err.to_string().contains("[E100]"));
}
