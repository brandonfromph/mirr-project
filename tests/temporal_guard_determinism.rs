#![forbid(unsafe_code)]
use nasa_rust_project::mirr_executor::drive_parsed_module_with_interpreter;
use nasa_rust_project::parser::parse_mirr;
use std::fs;

/// Run the parsed lexer module multiple times with the same input and assert
/// outputs are identical every repeat (guard/reflex determinism).
#[test]
fn temporal_guard_determinism_repeatable() {
    let path = std::path::Path::new("compiler_mirr").join("lexer.mirr");
    let txt = fs::read_to_string(&path).expect("failed to read lexer.mirr");
    let prog = parse_mirr(&txt).expect("failed to parse lexer.mirr");

    let input = b"guard true";
    // baseline
    let baseline = drive_parsed_module_with_interpreter(&prog, input);
    // repeat N times and ensure identical results
    for _ in 0..100 {
        let out = drive_parsed_module_with_interpreter(&prog, input);
        assert_eq!(out, baseline, "Observed non-deterministic outputs between runs");
    }
}
