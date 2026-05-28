//! Contract: Multi-Declaration Interop
//! This test enforces that the compiler correctly handles mixed signal syntax:
//! individual signal declarations existing alongside new 'signals { ... }' blocks.

use nasa_rust_project::compiler::macro_proc::expand_macros;
use nasa_rust_project::parser::parse_mirr;

#[test]
fn test_mixed_signal_syntax_interop() {
    let input = r#"
module test_mod {
    signal legacy: in bool;

    signals {
        a: in bool;
        b: out u8;
    }
}"#;

    let expanded = expand_macros(input);

    // Check expansion output
    assert!(expanded.contains("signal legacy: in bool;"));
    assert!(expanded.contains("signal a: in bool;"));
    assert!(expanded.contains("signal b: out u8;"));

    // Verify parser acceptance
    let program = parse_mirr(&expanded).expect("Parser should accept mixed signal declarations");
    assert_eq!(program.module.signals.len(), 3);
}
