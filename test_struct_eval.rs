//! Test struct literal and field access evaluation.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::types::LiteralValue;
use nasa_rust_project::mirr_executor::eval::eval_expr;
use nasa_rust_project::mirr_runtime::Value;

#[test]
fn test_struct_literal_evaluation() {
    // Create a struct literal: Point { x: 10, y: 20 }
    let struct_expr = Expr::StructLiteral {
        name: "Point".to_string(),
        fields: vec![
            ("x".to_string(), Expr::Literal(LiteralValue::Integer(10))),
            ("y".to_string(), Expr::Literal(LiteralValue::Integer(20))),
        ],
    };

    // Evaluate it with an empty environment
    let env_get = |_name: &str| Value::Integer(0);
    let result = eval_expr(&struct_expr, &env_get);

    // Verify the result is a Struct value
    match result {
        Value::Struct { name, fields } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2);

            // Check x field
            assert_eq!(fields[0].0, "x");
            match &fields[0].1 {
                Value::Integer(v) => assert_eq!(*v, 10),
                _ => panic!("Expected Integer value for x"),
            }

            // Check y field
            assert_eq!(fields[1].0, "y");
            match &fields[1].1 {
                Value::Integer(v) => assert_eq!(*v, 20),
                _ => panic!("Expected Integer value for y"),
            }
        }
        _ => panic!("Expected Struct value, got {:?}", result),
    }
}

#[test]
fn test_field_access_evaluation() {
    // Create a struct literal: Point { x: 42, y: 99 }
    let struct_expr = Expr::StructLiteral {
        name: "Point".to_string(),
        fields: vec![
            ("x".to_string(), Expr::Literal(LiteralValue::Integer(42))),
            ("y".to_string(), Expr::Literal(LiteralValue::Integer(99))),
        ],
    };

    // Create field access: point.x
    let field_access = Expr::FieldAccess {
        object: Box::new(struct_expr),
        field: "x".to_string(),
    };

    // Evaluate it
    let env_get = |_name: &str| Value::Integer(0);
    let result = eval_expr(&field_access, &env_get);

    // Verify the result is the x field value
    match result {
        Value::Integer(v) => assert_eq!(v, 42),
        _ => panic!("Expected Integer(42), got {:?}", result),
    }
}

#[test]
fn test_nested_field_access() {
    // Create outer struct: Container { data: Point { x: 100, y: 200 } }
    let inner_struct = Expr::StructLiteral {
        name: "Point".to_string(),
        fields: vec![
            ("x".to_string(), Expr::Literal(LiteralValue::Integer(100))),
            ("y".to_string(), Expr::Literal(LiteralValue::Integer(200))),
        ],
    };

    let outer_struct = Expr::StructLiteral {
        name: "Container".to_string(),
        fields: vec![("data".to_string(), inner_struct)],
    };

    // Access nested field: container.data
    let field_access = Expr::FieldAccess {
        object: Box::new(outer_struct),
        field: "data".to_string(),
    };

    // Evaluate it
    let env_get = |_name: &str| Value::Integer(0);
    let result = eval_expr(&field_access, &env_get);

    // Verify we get the inner struct
    match result {
        Value::Struct { name, fields } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2);
        }
        _ => panic!("Expected Struct value, got {:?}", result),
    }
}

#[test]
fn test_field_not_found() {
    // Create a struct literal without field 'z'
    let struct_expr = Expr::StructLiteral {
        name: "Point".to_string(),
        fields: vec![
            ("x".to_string(), Expr::Literal(LiteralValue::Integer(42))),
            ("y".to_string(), Expr::Literal(LiteralValue::Integer(99))),
        ],
    };

    // Try to access non-existent field 'z'
    let field_access = Expr::FieldAccess {
        object: Box::new(struct_expr),
        field: "z".to_string(),
    };

    // Evaluate it
    let env_get = |_name: &str| Value::Integer(0);
    let result = eval_expr(&field_access, &env_get);

    // Should return Integer(0) placeholder
    match result {
        Value::Integer(v) => assert_eq!(v, 0),
        _ => panic!("Expected Integer(0) placeholder, got {:?}", result),
    }
}
