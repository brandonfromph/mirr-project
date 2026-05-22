#[test]
fn test_multiple_sequential_compilations() {
    let m1 = "module mod1 { signals {} }";
    let m2 = "module mod2 { signals {} }";

    let res1 = nasa_rust_project::parser::parse_mirr(
        &nasa_rust_project::compiler::macro_proc::expand_macros(m1),
    );
    assert!(res1.is_ok(), "First parse should succeed");
    assert_eq!(res1.unwrap().module.name, "mod1");

    let res2 = nasa_rust_project::parser::parse_mirr(
        &nasa_rust_project::compiler::macro_proc::expand_macros(m2),
    );
    assert!(res2.is_ok(), "Second parse should succeed");
    assert_eq!(res2.unwrap().module.name, "mod2");
}
