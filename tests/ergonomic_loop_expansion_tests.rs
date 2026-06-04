//! Contract: Loop Expansion Validation
//! This test enforces that the compiler correctly expands 'for' loops within
//! signal blocks, ensuring index-based signal generation works as expected.

use nasa_rust_project::parser::parse_mirr;

#[test]
fn test_macro_expansion_loop_generation() {
    let input = r#"
module test_mod {
    signals {
        for i in 0..2 {
            s[i]: in bool;
        }
    }
}"#;

    let program = parse_mirr(input).unwrap();
    assert!(program.module.signals.iter().any(|s| s.name == "s_0"));
    assert!(program.module.signals.iter().any(|s| s.name == "s_1"));
}
