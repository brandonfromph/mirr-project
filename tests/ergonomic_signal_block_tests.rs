//! Contract: Signals Block Validation
//! This test enforces that the compiler correctly handles ergonomic 'signals { ... }' blocks,
//! ensuring they are expanded into the standard syntax expected by the formal parser.

use nasa_rust_project::compiler::macro_proc::expand_macros;
use nasa_rust_project::parser::parse_mirr;

#[test]
fn test_signals_block_to_standard_syntax() {
    let input = r#"
module test_mod {
    signals {
        a: in bool;
        b: out u8;
    }
}"#;

    // The expansion must result in standard declarations that the parser accepts
    let expanded = expand_macros(input);

    // Check expansion output
    assert!(expanded.contains("signal a: in bool;"));
    assert!(expanded.contains("signal b: out u8;"));
    assert!(!expanded.contains("signals {"));

    // Verify parser acceptance
    let program = parse_mirr(&expanded).expect("Parser should accept expanded signals");
    assert_eq!(program.module.signals.len(), 2);
}
