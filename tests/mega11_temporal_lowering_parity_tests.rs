#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

use nasa_rust_project::width::constraint::{generate_constraints, WidthConstraint};
use nasa_rust_project::width::types::FlatNode;

fn read_text(path: &str) -> String {
    let full_path = Path::new(path);
    fs::read_to_string(full_path)
        .unwrap_or_else(|_| panic!("{} must be readable", full_path.display()))
}

#[test]
fn mega11_unfold_index_ast_variant_exists() {
    let ast_expr = read_text("src/ast/expr.rs");
    assert!(ast_expr.contains("UnfoldIndex(String)"));
}

#[test]
fn mega11_unfold_index_emits_e506_and_fixed_width() {
    let nodes = vec![FlatNode::UnfoldIndex { name: "i".to_string() }];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    assert!(
        cset.diagnostics
            .iter()
            .any(|d| d.message.contains("[E506]") && d.message.contains("UnfoldIndex")),
        "Unresolved UnfoldIndex must emit E506 diagnostic"
    );

    assert!(
        cset.constraints.iter().any(|c| matches!(c, WidthConstraint::Fixed { node: 0, width: 32 })),
        "UnfoldIndex fallback width must be 32 bits"
    );
}

#[test]
fn mega11_temporal_lowering_parity_test_still_present() {
    let lowering_tests = read_text("tests/temporal_lowering_tests.rs");
    assert!(lowering_tests.contains("test_netlist_fixture_parity_neonatal_respirator"));
}

#[test]
fn mega11_temporal_compiler_comparison_coverage_present() {
    let compiler_tests = read_text("tests/temporal_compiler_tests.rs");
    assert!(compiler_tests.contains("test_comparison_operators_all_six"));
}
