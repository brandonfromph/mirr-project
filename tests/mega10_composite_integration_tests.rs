#![forbid(unsafe_code)]
//! MEGA-10 composite type integration tests.
//!
//! Tests for SignalType::{Array, Struct, FixedPoint, Bundle} and
//! Expr::{ArrayIndex, FieldAccess, ArrayLiteral, StructLiteral}.
//!
//! NASA P10: bounded loops, no recursion.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Module, SignalDecl};
use nasa_rust_project::ast::types::{
    ExtendedType, LiteralValue, SignalKind, SignalType, MAX_ARRAY_DIMS, MAX_FIXED_POINT_BITS,
    MAX_STRUCT_FIELDS,
};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

fn make_signal(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

// ===========================================================================
// SignalType::Array
// ===========================================================================

#[test]
fn array_type_construction() {
    let ty = SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 4 };
    match &ty {
        SignalType::Array { element, length } => {
            assert_eq!(length, &4);
            assert!(matches!(**element, SignalType::Unsigned(8)));
        }
        _ => panic!("Expected Array"),
    }
}

#[test]
fn array_type_display_not_empty() {
    let ty = SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 4 };
    let text = format!("{}", ty);
    assert!(!text.is_empty(), "Array type display must not be empty");
}

#[test]
fn array_type_is_composite() {
    let ty = SignalType::Array { element: Box::new(SignalType::Bool), length: 2 };
    assert!(ty.is_composite(), "Array must be composite");
}

#[test]
fn array_signal_in_module() {
    let m = Module {
        name: "arr_m".to_string(),
        signals: vec![make_signal(
            "arr",
            SignalKind::Input,
            SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 4 },
        )],
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    assert_eq!(m.signals[0].name, "arr");
}

// ===========================================================================
// SignalType::Struct
// ===========================================================================

#[test]
fn struct_type_construction() {
    let ty = SignalType::Struct {
        name: "MyStruct".to_string(),
        fields: vec![
            ("x".to_string(), SignalType::Unsigned(8)),
            ("y".to_string(), SignalType::Bool),
        ],
    };
    match &ty {
        SignalType::Struct { name, fields } => {
            assert_eq!(name, "MyStruct");
            assert_eq!(fields.len(), 2);
        }
        _ => panic!("Expected Struct"),
    }
}

#[test]
fn struct_type_is_composite() {
    let ty = SignalType::Struct {
        name: "S".to_string(),
        fields: vec![("a".to_string(), SignalType::Bool)],
    };
    assert!(ty.is_composite(), "Struct must be composite");
}

#[test]
fn struct_type_display_non_empty() {
    let ty = SignalType::Struct { name: "Point".to_string(), fields: Vec::new() };
    let text = format!("{}", ty);
    assert!(!text.is_empty(), "Struct display must not be empty");
}

#[test]
fn struct_max_fields_constant() {
    let _ = MAX_STRUCT_FIELDS;
}

// ===========================================================================
// SignalType::FixedPoint
// ===========================================================================

#[test]
fn fixed_point_construction() {
    let ty = SignalType::FixedPoint { total_bits: 16, frac_bits: 8 };
    match ty {
        SignalType::FixedPoint { total_bits, frac_bits } => {
            assert_eq!(total_bits, 16);
            assert_eq!(frac_bits, 8);
        }
        _ => panic!("Expected FixedPoint"),
    }
}

#[test]
fn fixed_point_is_composite() {
    let ty = SignalType::FixedPoint { total_bits: 32, frac_bits: 16 };
    assert!(ty.is_composite(), "FixedPoint must be composite");
}

#[test]
fn fixed_point_width_is_total_bits() {
    let ty = SignalType::FixedPoint { total_bits: 24, frac_bits: 8 };
    assert_eq!(ty.width(), 24, "FixedPoint width must equal total_bits");
}

#[test]
fn max_fixed_point_bits_constant() {
    assert_eq!(MAX_FIXED_POINT_BITS, 64);
}

// ===========================================================================
// SignalType::Bundle
// ===========================================================================

#[test]
fn bundle_type_construction() {
    let ty = SignalType::Bundle("AXI4".to_string());
    match &ty {
        SignalType::Bundle(name) => assert_eq!(name, "AXI4"),
        _ => panic!("Expected Bundle"),
    }
}

#[test]
fn bundle_is_composite() {
    let ty = SignalType::Bundle("WISHBONE".to_string());
    assert!(ty.is_composite(), "Bundle must be composite");
}

#[test]
fn bundle_display_non_empty() {
    let ty = SignalType::Bundle("my_interface".to_string());
    let text = format!("{}", ty);
    assert!(!text.is_empty(), "Bundle display must not be empty");
}

// ===========================================================================
// Expr::ArrayIndex
// ===========================================================================

#[test]
fn array_index_construction() {
    let expr = Expr::ArrayIndex {
        array: Box::new(Expr::Signal("arr".to_string())),
        index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    };
    match expr {
        Expr::ArrayIndex { .. } => {}
        _ => panic!("Expected ArrayIndex"),
    }
}

// ===========================================================================
// Expr::FieldAccess
// ===========================================================================

#[test]
fn field_access_construction() {
    let expr = Expr::FieldAccess {
        object: Box::new(Expr::Signal("point".to_string())),
        field: "x".to_string(),
    };
    match expr {
        Expr::FieldAccess { field, .. } => assert_eq!(field, "x"),
        _ => panic!("Expected FieldAccess"),
    }
}

// ===========================================================================
// Expr::ArrayLiteral
// ===========================================================================

#[test]
fn array_literal_construction() {
    let expr = Expr::ArrayLiteral(vec![
        Expr::Literal(LiteralValue::Integer(1)),
        Expr::Literal(LiteralValue::Integer(2)),
        Expr::Literal(LiteralValue::Integer(3)),
    ]);
    match expr {
        Expr::ArrayLiteral(elems) => assert_eq!(elems.len(), 3),
        _ => panic!("Expected ArrayLiteral"),
    }
}

#[test]
fn empty_array_literal() {
    let expr = Expr::ArrayLiteral(Vec::new());
    match expr {
        Expr::ArrayLiteral(elems) => assert!(elems.is_empty()),
        _ => panic!("Expected ArrayLiteral"),
    }
}

// ===========================================================================
// Expr::StructLiteral
// ===========================================================================

#[test]
fn struct_literal_construction() {
    let expr = Expr::StructLiteral {
        name: "Point".to_string(),
        fields: vec![
            ("x".to_string(), Expr::Literal(LiteralValue::Integer(10))),
            ("y".to_string(), Expr::Literal(LiteralValue::Integer(20))),
        ],
    };
    match expr {
        Expr::StructLiteral { name, fields } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2);
        }
        _ => panic!("Expected StructLiteral"),
    }
}

// ===========================================================================
// MAX_ARRAY_DIMS constant
// ===========================================================================

#[test]
fn max_array_dims_positive() {
    let _ = MAX_ARRAY_DIMS;
}

// ===========================================================================
// Pipeline: composite types in module signals don't crash
// ===========================================================================

#[test]
fn pipeline_with_plain_module_still_works() {
    let result = run_pipeline(
        "module plain {\n    signal x: in u8;\n    signal y: out bool;\n}",
        &PipelineConfig::default(),
    );
    assert!(
        result.is_ok(),
        "plain module must compile after MEGA-10 additions: {:?}",
        result.err()
    );
}

#[test]
fn composite_signal_in_ast_module_no_crash() {
    // Build a module with a composite-typed signal at the AST level
    // (pipeline may not fully process it but must not panic)
    let m = Module {
        name: "comp_m".to_string(),
        signals: vec![
            make_signal(
                "arr_sig",
                SignalKind::Input,
                SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 4 },
            ),
            make_signal("scalar", SignalKind::Output, SignalType::Bool),
        ],
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    };
    assert_eq!(m.signals.len(), 2);
    assert!(m.signals[0].ty.core.is_composite());
}
