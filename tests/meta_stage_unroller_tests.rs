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

#[test]
fn unfold_index_retains_identifier_name() {
    let expr = Expr::UnfoldIndex("loop_index".to_string());
    match expr {
        Expr::UnfoldIndex(name) => assert_eq!(name, "loop_index"),
        other => panic!("Expected UnfoldIndex, got {other:?}"),
    }
}

#[test]
fn unfold_index_emits_e506_diagnostic() {
    let nodes = vec![FlatNode::UnfoldIndex { name: "i".to_string() }];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    assert!(
        cset.diagnostics.iter().any(|d| d.message.contains("[E506]")),
        "UnfoldIndex must emit E506 diagnostic"
    );
}

#[test]
fn unfold_index_falls_back_to_fixed_32_bit_width() {
    let nodes = vec![FlatNode::UnfoldIndex { name: "i".to_string() }];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    assert!(
        cset.constraints.iter().any(|c| matches!(c, WidthConstraint::Fixed { node: 0, width: 32 })),
        "UnfoldIndex must emit fallback Fixed(32) width constraint"
    );
}

#[test]
fn empty_array_literal_generates_sum_all_with_no_elements() {
    let nodes = vec![FlatNode::ArrayLiteral { elements: Vec::new(), width: 0 }];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    let sum_all = cset.constraints.iter().find(|c| matches!(c, WidthConstraint::SumAll { .. }));
    assert!(sum_all.is_some(), "Expected SumAll constraint for empty ArrayLiteral");
    if let Some(WidthConstraint::SumAll { node, elements }) = sum_all {
        assert_eq!(*node, 0);
        assert!(elements.is_empty());
    }
}

#[test]
fn struct_literal_generates_sum_all_constraint() {
    let nodes = vec![
        FlatNode::Literal { value: 1 },
        FlatNode::Literal { value: 2 },
        FlatNode::StructLiteral {
            name: "Pair".to_string(),
            fields: vec![("a".to_string(), 0), ("b".to_string(), 1)],
            width: 2,
        },
    ];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    assert!(
        cset.constraints.iter().any(|c| matches!(c, WidthConstraint::SumAll { node: 2, .. })),
        "StructLiteral must produce SumAll at its node index"
    );
}

#[test]
fn struct_literal_sum_all_preserves_field_order() {
    let nodes = vec![
        FlatNode::Literal { value: 9 },
        FlatNode::Literal { value: 4 },
        FlatNode::StructLiteral {
            name: "Pair".to_string(),
            fields: vec![("first".to_string(), 1), ("second".to_string(), 0)],
            width: 2,
        },
    ];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    let sum_all = cset.constraints.iter().find(|c| matches!(c, WidthConstraint::SumAll { .. }));
    assert!(sum_all.is_some());
    if let Some(WidthConstraint::SumAll { elements, .. }) = sum_all {
        assert_eq!(elements, &vec![1, 0]);
    }
}

#[test]
fn array_literal_of_literals_produces_no_diagnostics() {
    let nodes = vec![
        FlatNode::Literal { value: 7 },
        FlatNode::Literal { value: 8 },
        FlatNode::ArrayLiteral { elements: vec![0, 1], width: 4 },
    ];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    assert!(cset.diagnostics.is_empty(), "Literal-only array literal should not emit diagnostics");
}

#[test]
fn array_literal_sum_all_targets_literal_node_index() {
    let nodes = vec![
        FlatNode::Literal { value: 5 },
        FlatNode::ArrayLiteral { elements: vec![0], width: 3 },
    ];
    let signals = std::collections::HashMap::new();
    let cset = generate_constraints(&nodes, &signals);

    assert!(
        cset.constraints.iter().any(
            |c| matches!(c, WidthConstraint::SumAll { node: 1, elements } if elements.len() == 1)
        ),
        "Single-element array literal must target node index 1"
    );
}
