//! Contract: Loop Expansion Validation
//! This test enforces that the compiler correctly expands 'for' loops within
//! signal blocks, ensuring index-based signal generation works as expected.

use nasa_rust_project::compiler::macro_proc::expand_macros;

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

    // The macro expansion should replace [i] with _0, _1, etc.
    // The parser expects 'signal <name>: <kind> <type>;'
    let expanded = expand_macros(input);

    assert!(expanded.contains("signal s_0: in bool;"));
    assert!(expanded.contains("signal s_1: in bool;"));
    assert!(!expanded.contains("for i in"));
    assert!(!expanded.contains("s[i]"));
}
