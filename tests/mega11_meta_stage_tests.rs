#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::width::constraint::{generate_constraints, WidthConstraint};
use nasa_rust_project::width::types::FlatNode;

#[test]
fn unfold_index_parsing_placeholder() {
    // Currently UnfoldIndex is only an AST artifact, not parsed directly from user code.
    // It's used internally by the meta-stage unroller.
    let expr = Expr::UnfoldIndex("i".to_string());
    assert!(matches!(expr, Expr::UnfoldIndex(_)));
}

#[test]
fn array_literal_generates_sum_all_constraint() {
    let nodes = vec![
        FlatNode::Literal { value: 1 },                            // idx 0
        FlatNode::Literal { value: 2 },                            // idx 1
        FlatNode::ArrayLiteral { elements: vec![0, 1], width: 8 }, // idx 2
    ];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    let sum_all = cset.constraints.iter().find(|c| matches!(c, WidthConstraint::SumAll { .. }));
    assert!(sum_all.is_some(), "Expected SumAll constraint for ArrayLiteral");
    if let Some(WidthConstraint::SumAll { node, elements }) = sum_all {
        assert_eq!(*node, 2);
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], 0);
        assert_eq!(elements[1], 1);
    }
}
