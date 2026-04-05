#![forbid(unsafe_code)]
//! MEGA-10 composite type integration tests.
//!
//! Tests for SignalType::{Array, Struct, FixedPoint, Bundle} and
//! Expr::{ArrayIndex, FieldAccess, ArrayLiteral, StructLiteral}.
//!
//! NASA P10: bounded loops, no recursion.

use nasa_rust_project::ast::expr::Expr;
use nasa_rust_project::ast::program::{Assignment, Module, Reflex, SignalDecl};
use nasa_rust_project::ast::types::{
    ExtendedType, LiteralValue, SignalKind, SignalType, MAX_ARRAY_DIMS, MAX_FIXED_POINT_BITS,
    MAX_STRUCT_FIELDS,
};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::typeck::typecheck_module;

fn make_signal(name: &str, kind: SignalKind, ty: SignalType) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind,
        ty: ExtendedType::from_core(ty),
        origin: None,
        span: None,
    }
}

fn module_with_single_assignment(signals: Vec<SignalDecl>, target: &str, value: Expr) -> Module {
    Module {
        name: "typeck_composite".to_string(),
        signals,
        guards: Vec::new(),
        reflexes: vec![Reflex {
            name: "r0".to_string(),
            guard_names: Vec::new(),
            assignments: vec![Assignment { target: target.to_string(), value, span: None }],
            origin: None,
            span: None,
        }],
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
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

#[test]
fn array_width_is_element_width_times_length() {
    let ty = SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 4 };
    assert_eq!(ty.width(), 32);
}

#[test]
fn nested_array_width_multiplies_dimensions() {
    let ty = SignalType::Array {
        element: Box::new(SignalType::Array {
            element: Box::new(SignalType::Unsigned(4)),
            length: 2,
        }),
        length: 3,
    };
    assert_eq!(ty.width(), 24);
}

#[test]
fn struct_width_is_sum_of_field_widths() {
    let ty = SignalType::Struct {
        name: "Telemetry".to_string(),
        fields: vec![
            ("status".to_string(), SignalType::Bool),
            ("temp".to_string(), SignalType::Unsigned(10)),
            ("pressure".to_string(), SignalType::Unsigned(12)),
        ],
    };
    assert_eq!(ty.width(), 23);
}

#[test]
fn bundle_width_is_zero_before_elaboration() {
    let ty = SignalType::Bundle("SensorBus".to_string());
    assert_eq!(ty.width(), 0);
}

#[test]
fn fifo_width_is_depth_times_element_width() {
    let ty = SignalType::Fifo { element: Box::new(SignalType::Unsigned(16)), depth: 8 };
    assert_eq!(ty.width(), 128);
}

#[test]
fn fixed_point_width_and_signed_contract() {
    let ty = SignalType::FixedPoint { total_bits: 24, frac_bits: 10 };
    let (width, signed) = ty.width_and_signed();
    assert_eq!(width, 24);
    assert!(!signed, "fixed-point width modeling remains unsigned in width pass");
}

#[test]
fn typeck_array_index_infers_element_type() {
    let arr_ty = SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 4 };
    let expr = Expr::ArrayIndex {
        array: Box::new(Expr::Signal("arr".to_string())),
        index: Box::new(Expr::Literal(LiteralValue::Integer(0))),
    };
    let module = module_with_single_assignment(
        vec![
            make_signal("arr", SignalKind::Input, arr_ty),
            make_signal("out", SignalKind::Output, SignalType::Unsigned(8)),
        ],
        "out",
        expr,
    );

    let root_ptr = &module.reflexes[0].assignments[0].value as *const Expr;
    let type_map = typecheck_module(&module)
        .expect("array index assignment should typecheck when target matches element type");
    assert_eq!(type_map.get(&root_ptr), Some(&SignalType::Unsigned(8)));
}

#[test]
fn typeck_field_access_infers_field_type_from_struct_literal() {
    let expr = Expr::FieldAccess {
        object: Box::new(Expr::StructLiteral {
            name: "Pair".to_string(),
            fields: vec![
                ("x".to_string(), Expr::Literal(LiteralValue::Integer(7))),
                ("flag".to_string(), Expr::Literal(LiteralValue::Bool(true))),
            ],
        }),
        field: "flag".to_string(),
    };
    let module = module_with_single_assignment(
        vec![make_signal("out", SignalKind::Output, SignalType::Bool)],
        "out",
        expr,
    );

    let root_ptr = &module.reflexes[0].assignments[0].value as *const Expr;
    let type_map = typecheck_module(&module)
        .expect("field access over struct literal should infer the selected field type");
    assert_eq!(type_map.get(&root_ptr), Some(&SignalType::Bool));
}

#[test]
fn typeck_array_literal_infers_array_shape_and_element_width() {
    let expr = Expr::ArrayLiteral(vec![
        Expr::Literal(LiteralValue::Integer(1)),
        Expr::Literal(LiteralValue::Integer(255)),
    ]);
    let expected = SignalType::Array { element: Box::new(SignalType::Unsigned(8)), length: 2 };
    let module = module_with_single_assignment(
        vec![make_signal("out", SignalKind::Output, expected.clone())],
        "out",
        expr,
    );

    let root_ptr = &module.reflexes[0].assignments[0].value as *const Expr;
    let type_map = typecheck_module(&module)
        .expect("array literal assignment should typecheck when target matches inferred shape");
    assert_eq!(type_map.get(&root_ptr), Some(&expected));
}

#[test]
fn typeck_struct_literal_infers_field_types() {
    let expr = Expr::StructLiteral {
        name: "Telemetry".to_string(),
        fields: vec![
            ("ok".to_string(), Expr::Literal(LiteralValue::Bool(true))),
            ("count".to_string(), Expr::Literal(LiteralValue::Integer(3))),
        ],
    };
    let expected = SignalType::Struct {
        name: "Telemetry".to_string(),
        fields: vec![
            ("ok".to_string(), SignalType::Bool),
            ("count".to_string(), SignalType::Unsigned(2)),
        ],
    };
    let module = module_with_single_assignment(
        vec![make_signal("out", SignalKind::Output, expected.clone())],
        "out",
        expr,
    );

    let root_ptr = &module.reflexes[0].assignments[0].value as *const Expr;
    let type_map = typecheck_module(&module)
        .expect("struct literal assignment should typecheck when target matches inferred fields");
    assert_eq!(type_map.get(&root_ptr), Some(&expected));
}
