//! Unit tests for the ergonomic macro processor.
//! Ensures stable expansion of 'for' loops in 'signals' and 'calls' blocks.

use nasa_rust_project::compiler::macro_proc::expand_macros;

#[test]
fn test_macro_expansion_loop() {
    let input = r#"
signals {
    for i in 0..2 {
        s[i]: in bool;
    }
}
"#;
    let expected = r#"signals {
        s_0: in bool;
        s_1: in bool;
}
"#;
    let output = expand_macros(input);
    assert_eq!(output, expected);
}

#[test]
fn test_standard_block_preservation() {
    let input = r#"
signals {
    s1: in bool;
    s2: out bool;
}
"#;
    // Standard blocks should be preserved exactly
    let output = expand_macros(input);
    assert_eq!(output, input);
}
