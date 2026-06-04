//! Contract: Signals Block Validation
//! This test enforces that the compiler correctly handles ergonomic 'signals { ... }' blocks,
//! ensuring they are expanded into the standard syntax expected by the formal parser.

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

    // Verify parser acceptance
    let program = parse_mirr(input).expect("Parser should accept expanded signals");
    assert_eq!(program.module.signals.len(), 2);
    assert!(program.module.signals.iter().any(|s| s.name == "a"));
    assert!(program.module.signals.iter().any(|s| s.name == "b"));
}
