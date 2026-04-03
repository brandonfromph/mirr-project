#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

fn lra_source() -> String {
    let path = Path::new("crates/lra-cli/src/main.rs");
    fs::read_to_string(path).expect("lra-cli main.rs must be readable")
}

#[test]
fn rwfi2_lra_uses_pipeline_contract() {
    let src = lra_source();
    assert!(src.contains("run_pipeline"));
    assert!(src.contains("PipelineConfig"));
}

#[test]
fn rwfi2_lra_supports_required_targets() {
    let src = lra_source();
    assert!(src.contains("\"verilog\""));
    assert!(src.contains("\"firrtl\""));
    assert!(src.contains("\"rspu\""));
    assert!(src.contains("\"sexpr\""));
}

#[test]
fn rwfi2_lra_has_unknown_target_guard() {
    let src = lra_source();
    assert!(src.contains("Unknown compile target"));
}

#[test]
fn rwfi2_lra_compile_path_uses_library_not_shell_wrappers() {
    let src = lra_source();
    assert!(src.contains("fn compile_via_library"));
    assert!(!src.contains("Command::new("));
    assert!(!src.contains("spawn("));
}

#[test]
fn rwfi2_lra_compile_supports_target_aliases() {
    let src = lra_source();
    assert!(src.contains("\"verilog\" | \"sv\""));
    assert!(src.contains("\"sexpr\" | \"s-expr\" | \"sexp\""));
}
