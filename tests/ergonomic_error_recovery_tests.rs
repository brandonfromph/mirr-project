//! Contract: Diagnostic Precision for Ergonomic Blocks
//! This test enforces that when a syntax error occurs inside a signals block,
//! the compiler reports the error at the correct line number of the original file.

use nasa_rust_project::compiler::macro_proc::expand_macros;
use nasa_rust_project::parser::parse_mirr;

#[test]
fn test_signals_block_error_line_reporting() {
    let input = "module test_mod {
    signals {
        a: in bool;
        b: invalid_type; // This should trigger a parse error
    }
}";

    // The expansion should complete even with the error (the parser will catch it later)
    let expanded = expand_macros(input);
    println!("EXPANDED SOURCE:\n{}", expanded);

    // Check if the parser catches the error
    let result = parse_mirr(&expanded);

    assert!(result.is_err(), "Parser should have flagged the invalid type error");

    // Ensure the diagnostic engine provides a line number.
    let error_msg = format!("{:?}", result.err().unwrap());
    println!("ERROR MSG: {}", error_msg);
    assert!(
        error_msg.contains("start_line: 3"),
        "Error should be reported on line 4 (original source, 0-indexed is 3)"
    );
}
