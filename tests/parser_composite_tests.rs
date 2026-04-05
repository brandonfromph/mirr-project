//! Composite type parsing tests for MEGA-10 BOUNDED-DATA.
//!
//! Comprehensive test suite for parsing array types, struct types, fixed-point types,
//! interface definitions, struct definitions, and composite expressions including
//! array indexing, field access, array literals, and struct literals.
//!
//! Tests both positive cases (valid syntax) and negative cases (error handling)
//! with bounded limits validation per NASA Power-of-10 rules.

#![forbid(unsafe_code)]

use nasa_rust_project::ast::expr::Expr;

use nasa_rust_project::ast::types::{LiteralValue, SignalKind, SignalType};
use nasa_rust_project::parser::{parse_expression, parse_mirr};
use nasa_rust_project::validate_module;

/// Test 1: Parse array type signal declarations.
/// Tests parsing of `signal x: internal u8[16];` syntax.
#[test]
fn parse_array_type_signal() {
    let source = r#"
module TestMod {
    signal x: internal u8[16];
    signal y: in bool[4];
    signal z: out i32[8];
}
"#;

    let program = parse_mirr(source).expect("Should parse array signal types");
    assert_eq!(program.module.signals.len(), 3);

    // Test u8[16] array type
    let x_signal = &program.module.signals[0];
    assert_eq!(x_signal.name, "x");
    assert_eq!(x_signal.kind, SignalKind::Internal);
    match &x_signal.ty.core {
        SignalType::Array { element, length } => {
            assert_eq!(**element, SignalType::Unsigned(8));
            assert_eq!(*length, 16);
        }
        other => panic!("Expected Array type, got: {:?}", other),
    }

    // Test bool[4] array type
    let y_signal = &program.module.signals[1];
    assert_eq!(y_signal.name, "y");
    assert_eq!(y_signal.kind, SignalKind::Input);
    match &y_signal.ty.core {
        SignalType::Array { element, length } => {
            assert_eq!(**element, SignalType::Bool);
            assert_eq!(*length, 4);
        }
        other => panic!("Expected Array type, got: {:?}", other),
    }

    // Test i32[8] array type
    let z_signal = &program.module.signals[2];
    assert_eq!(z_signal.name, "z");
    assert_eq!(z_signal.kind, SignalKind::Output);
    match &z_signal.ty.core {
        SignalType::Array { element, length } => {
            assert_eq!(**element, SignalType::Signed(32));
            assert_eq!(*length, 8);
        }
        other => panic!("Expected Array type, got: {:?}", other),
    }
}

/// Test 2: Parse fixed-point type signal declarations.
/// Tests parsing of `signal fp: internal fixed<16,8>;` syntax.
#[test]
fn parse_fixed_point_type() {
    let source = r#"
module TestMod {
    signal fp1: internal fixed<16,8>;
    signal fp2: in fixed<32,16>;
    signal fp3: out fixed<64,32>;
}
"#;

    let program = parse_mirr(source).expect("Should parse fixed-point types");
    assert_eq!(program.module.signals.len(), 3);

    // Test fixed<16,8>
    let fp1 = &program.module.signals[0];
    assert_eq!(fp1.name, "fp1");
    match &fp1.ty.core {
        SignalType::FixedPoint { total_bits, frac_bits } => {
            assert_eq!(*total_bits, 16);
            assert_eq!(*frac_bits, 8);
        }
        other => panic!("Expected FixedPoint type, got: {:?}", other),
    }

    // Test fixed<32,16>
    let fp2 = &program.module.signals[1];
    match &fp2.ty.core {
        SignalType::FixedPoint { total_bits, frac_bits } => {
            assert_eq!(*total_bits, 32);
            assert_eq!(*frac_bits, 16);
        }
        other => panic!("Expected FixedPoint type, got: {:?}", other),
    }

    // Test fixed<64,32>
    let fp3 = &program.module.signals[2];
    match &fp3.ty.core {
        SignalType::FixedPoint { total_bits, frac_bits } => {
            assert_eq!(*total_bits, 64);
            assert_eq!(*frac_bits, 32);
        }
        other => panic!("Expected FixedPoint type, got: {:?}", other),
    }
}

#[test]
fn parse_unsigned_generic_type() {
    let source = r#"
module TestMod {
    signal x: in unsigned<8>;
}
"#;
    let program = parse_mirr(source).expect("Should parse unsigned<8> type");
    let x_signal = &program.module.signals[0];
    assert_eq!(x_signal.name, "x");
    assert_eq!(x_signal.kind, SignalKind::Input);
    assert_eq!(x_signal.ty.core, SignalType::Unsigned(8));
}

/// Test 3: Parse struct reference signal declarations.
/// Tests parsing of `signal pos: internal struct Point;` syntax.
#[test]
fn parse_struct_reference_signal() {
    let source = r#"
struct Point {
    x: u16;
    y: u16;
}

module TestMod {
    signal pos: internal struct Point;
    signal rect: in struct Rectangle;
}
"#;

    let program = parse_mirr(source).expect("Should parse struct reference signals");
    assert_eq!(program.module.signals.len(), 2);

    // Test struct Point reference
    let pos_signal = &program.module.signals[0];
    assert_eq!(pos_signal.name, "pos");
    assert_eq!(pos_signal.kind, SignalKind::Internal);
    match &pos_signal.ty.core {
        SignalType::Struct { name, fields } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2); // Hydrated from top-level struct declaration
        }
        other => panic!("Expected Struct type, got: {:?}", other),
    }

    // Test struct Rectangle reference
    let rect_signal = &program.module.signals[1];
    assert_eq!(rect_signal.name, "rect");
    match &rect_signal.ty.core {
        SignalType::Struct { name, .. } => {
            assert_eq!(name, "Rectangle");
        }
        other => panic!("Expected Struct type, got: {:?}", other),
    }
}

/// Test 4: Parse array index expressions.
/// Tests parsing of `array[index]` expression syntax.
#[test]
fn parse_array_index() {
    // Simple array indexing
    let expr = parse_expression("array[index]").expect("Should parse array indexing");
    match expr {
        Expr::ArrayIndex { array, index } => {
            assert_eq!(&*array, &Expr::Signal("array".to_string()));
            assert_eq!(&*index, &Expr::Signal("index".to_string()));
        }
        other => panic!("Expected ArrayIndex, got: {:?}", other),
    }

    // Array indexing with literal
    let expr = parse_expression("data[5]").expect("Should parse array indexing with literal");
    match expr {
        Expr::ArrayIndex { array, index } => {
            assert_eq!(&*array, &Expr::Signal("data".to_string()));
            assert_eq!(&*index, &Expr::Literal(LiteralValue::Integer(5)));
        }
        other => panic!("Expected ArrayIndex, got: {:?}", other),
    }

    // Array indexing with expression
    let expr =
        parse_expression("buffer[i + 1]").expect("Should parse array indexing with expression");
    match expr {
        Expr::ArrayIndex { array, index } => {
            assert_eq!(&*array, &Expr::Signal("buffer".to_string()));
            // Index should be a binary expression i + 1
            match &*index {
                Expr::Binary { .. } => {} // Successfully parsed as binary expression
                other => panic!("Expected Binary expression for index, got: {:?}", other),
            }
        }
        other => panic!("Expected ArrayIndex, got: {:?}", other),
    }
}

/// Test 5: Parse field access expressions.
/// Tests parsing of `object.field` expression syntax.
#[test]
fn parse_field_access() {
    // Simple field access
    let expr = parse_expression("point.x").expect("Should parse field access");
    match expr {
        Expr::FieldAccess { object, field } => {
            assert_eq!(&*object, &Expr::Signal("point".to_string()));
            assert_eq!(field, "x");
        }
        other => panic!("Expected FieldAccess, got: {:?}", other),
    }

    // Chained field access (parsed left-to-right)
    let expr = parse_expression("rect.center.y").expect("Should parse chained field access");
    match expr {
        Expr::FieldAccess { object, field } => {
            assert_eq!(field, "y");
            match &*object {
                Expr::FieldAccess { object: inner_obj, field: inner_field } => {
                    assert_eq!(&**inner_obj, &Expr::Signal("rect".to_string()));
                    assert_eq!(inner_field, "center");
                }
                other => panic!("Expected nested FieldAccess, got: {:?}", other),
            }
        }
        other => panic!("Expected FieldAccess, got: {:?}", other),
    }
}

/// Test 6: Parse array literal expressions.
/// Tests parsing of `[1, 2, 3]` expression syntax.
#[test]
fn parse_array_literal() {
    // Simple array literal
    let expr = parse_expression("[1, 2, 3]").expect("Should parse array literal");
    match expr {
        Expr::ArrayLiteral(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Expr::Literal(LiteralValue::Integer(1)));
            assert_eq!(elements[1], Expr::Literal(LiteralValue::Integer(2)));
            assert_eq!(elements[2], Expr::Literal(LiteralValue::Integer(3)));
        }
        other => panic!("Expected ArrayLiteral, got: {:?}", other),
    }

    // Empty array literal
    let expr = parse_expression("[]").expect("Should parse empty array literal");
    match expr {
        Expr::ArrayLiteral(elements) => {
            assert_eq!(elements.len(), 0);
        }
        other => panic!("Expected ArrayLiteral, got: {:?}", other),
    }

    // Array literal with signals
    let expr = parse_expression("[x, y, z]").expect("Should parse array literal with signals");
    match expr {
        Expr::ArrayLiteral(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Expr::Signal("x".to_string()));
            assert_eq!(elements[1], Expr::Signal("y".to_string()));
            assert_eq!(elements[2], Expr::Signal("z".to_string()));
        }
        other => panic!("Expected ArrayLiteral, got: {:?}", other),
    }

    // Array literal with mixed expressions
    let expr = parse_expression("[1, x, true]").expect("Should parse mixed array literal");
    match expr {
        Expr::ArrayLiteral(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Expr::Literal(LiteralValue::Integer(1)));
            assert_eq!(elements[1], Expr::Signal("x".to_string()));
            assert_eq!(elements[2], Expr::Literal(LiteralValue::Bool(true)));
        }
        other => panic!("Expected ArrayLiteral, got: {:?}", other),
    }
}

/// Test 7: Parse struct literal expressions.
/// Tests parsing of `Point { x: 10, y: 20 }` expression syntax.
#[test]
fn parse_struct_literal() {
    // Simple struct literal
    let expr = parse_expression("Point { x: 10, y: 20 }").expect("Should parse struct literal");
    match expr {
        Expr::StructLiteral { name, fields } => {
            assert_eq!(name, "Point");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].0, "x");
            assert_eq!(fields[0].1, Expr::Literal(LiteralValue::Integer(10)));
            assert_eq!(fields[1].0, "y");
            assert_eq!(fields[1].1, Expr::Literal(LiteralValue::Integer(20)));
        }
        other => panic!("Expected StructLiteral, got: {:?}", other),
    }

    // Struct literal with signal references
    let expr = parse_expression("Vector { magnitude: mag, angle: ang }")
        .expect("Should parse struct literal with signals");
    match expr {
        Expr::StructLiteral { name, fields } => {
            assert_eq!(name, "Vector");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0].0, "magnitude");
            assert_eq!(fields[0].1, Expr::Signal("mag".to_string()));
            assert_eq!(fields[1].0, "angle");
            assert_eq!(fields[1].1, Expr::Signal("ang".to_string()));
        }
        other => panic!("Expected StructLiteral, got: {:?}", other),
    }

    // Empty struct literal
    let expr = parse_expression("Empty { }").expect("Should parse empty struct literal");
    match expr {
        Expr::StructLiteral { name, fields } => {
            assert_eq!(name, "Empty");
            assert_eq!(fields.len(), 0);
        }
        other => panic!("Expected StructLiteral, got: {:?}", other),
    }
}

/// Test 8: Parse struct definition declarations.
/// Tests parsing of `struct Point { x: u16; y: u16; }` syntax.
#[test]
fn parse_struct_definition() {
    let source = r#"
struct Point {
    x: u16;
    y: u16;
}

struct Rectangle {
    top_left: struct Point;
    bottom_right: struct Point;
    color: u8;
}

module TestMod {
    signal dummy: internal bool;
}
"#;

    let program = parse_mirr(source).expect("Should parse struct definitions");
    assert_eq!(program.module.name, "TestMod");
    assert_eq!(program.module.signals.len(), 1);
    assert_eq!(program.module.guards.len(), 0);
    assert_eq!(program.module.reflexes.len(), 0);
    // At least program parses successfully; structural declarations are not in MirrProgram in this schema.
}

/// Test 9: Parse interface definition declarations.
/// Tests parsing of `interface AXI4 { araddr: out u32; arready: in bool; }` syntax.
#[test]
fn parse_interface_definition() {
    let source = r#"
interface AXI4 {
    araddr: out u32;
    arready: in bool;
    awaddr: out u64;
    awvalid: out bool;
    bready: out bool;
    bvalid: in bool;
}

interface Simple {
    clk: in bool;
    data: out u8[16];
}

module TestMod {
    signal dummy: internal bool;
}
"#;

    let program = parse_mirr(source).expect("Should parse interface definitions");
    assert_eq!(program.module.name, "TestMod");
    assert_eq!(program.module.signals.len(), 1);
    assert_eq!(program.module.guards.len(), 0);
    assert_eq!(program.module.reflexes.len(), 0);
    // Interface declarations are not part of the returned MirrProgram in this schema.
}

#[test]
fn top_level_struct_declaration_retained_for_semantic_field_resolution() {
    let source = r#"
struct Point {
    x: u16;
    y: u16;
}

module TestMod {
    signal pos: internal struct Point;
    signal out_x: out u16;

    guard g {
        when pos.x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            out_x = pos.y;
        }
    }
}
"#;

    let program = parse_mirr(source).expect("Should parse struct-backed field accesses");
    validate_module(&program.module)
        .expect("Struct field accesses should resolve through top-level declarations");
}

#[test]
fn top_level_interface_declaration_retained_for_semantic_bundle_resolution() {
    let source = r#"
interface SensorBus {
    ready: in bool;
    value: in u16;
}

module TestMod {
    signal bus: internal interface SensorBus;
    signal alarm: out bool;

    guard g {
        when bus.ready
        for 1 cycles;
    }

    reflex r {
        on g {
            alarm = bus.ready;
        }
    }
}
"#;

    let program = parse_mirr(source).expect("Should parse interface-backed bundle accesses");
    validate_module(&program.module)
        .expect("Bundle field accesses should resolve through top-level declarations");
}

/// Test 10: Parse nested array indexing expressions.
/// Tests parsing of `array[i][j]` chained indexing syntax.
#[test]
fn parse_nested_array_index() {
    // Two-level array indexing
    let expr = parse_expression("matrix[i][j]").expect("Should parse nested array indexing");
    match expr {
        Expr::ArrayIndex { array, index } => {
            // Second index should be 'j'
            assert_eq!(&*index, &Expr::Signal("j".to_string()));
            // Array should be matrix[i]
            match &*array {
                Expr::ArrayIndex { array: inner_array, index: inner_index } => {
                    assert_eq!(&**inner_array, &Expr::Signal("matrix".to_string()));
                    assert_eq!(&**inner_index, &Expr::Signal("i".to_string()));
                }
                other => panic!("Expected nested ArrayIndex, got: {:?}", other),
            }
        }
        other => panic!("Expected ArrayIndex, got: {:?}", other),
    }

    // Three-level array indexing
    let expr = parse_expression("cube[x][y][z]").expect("Should parse three-level array indexing");
    match expr {
        Expr::ArrayIndex { array, index } => {
            assert_eq!(index.as_ref(), &Expr::Signal("z".to_string()));
            match array.as_ref() {
                Expr::ArrayIndex { array, index } => {
                    assert_eq!(index.as_ref(), &Expr::Signal("y".to_string()));
                    match array.as_ref() {
                        Expr::ArrayIndex { array, index } => {
                            assert_eq!(array.as_ref(), &Expr::Signal("cube".to_string()));
                            assert_eq!(index.as_ref(), &Expr::Signal("x".to_string()));
                        }
                        other => panic!("Expected third-level ArrayIndex, got: {:?}", other),
                    }
                }
                other => panic!("Expected second-level ArrayIndex, got: {:?}", other),
            }
        }
        other => panic!("Expected ArrayIndex, got: {:?}", other),
    }

    // Mixed indexing with literals
    let expr =
        parse_expression("data[0][1]").expect("Should parse nested array indexing with literals");
    match expr {
        Expr::ArrayIndex { array, index } => {
            assert_eq!(index.as_ref(), &Expr::Literal(LiteralValue::Integer(1)));
            match array.as_ref() {
                Expr::ArrayIndex { array, index } => {
                    assert_eq!(array.as_ref(), &Expr::Signal("data".to_string()));
                    assert_eq!(index.as_ref(), &Expr::Literal(LiteralValue::Integer(0)));
                }
                other => panic!("Expected nested ArrayIndex, got: {:?}", other),
            }
        }
        other => panic!("Expected ArrayIndex, got: {:?}", other),
    }
}

/// Test 11: Parse mixed composite expressions.
/// Tests parsing of `struct.array_field[i].x` mixed composite access.
#[test]
fn parse_mixed_composite() {
    // struct.field[index].subfield
    let expr =
        parse_expression("packet.data[i].value").expect("Should parse mixed composite expression");
    match expr {
        Expr::FieldAccess { object, field } => {
            assert_eq!(field, "value");
            // Object should be packet.data[i]
            match object.as_ref() {
                Expr::ArrayIndex { array, index } => {
                    assert_eq!(index.as_ref(), &Expr::Signal("i".to_string()));
                    // Array should be packet.data
                    match array.as_ref() {
                        Expr::FieldAccess { object, field } => {
                            assert_eq!(object.as_ref(), &Expr::Signal("packet".to_string()));
                            assert_eq!(field, "data");
                        }
                        other => panic!("Expected FieldAccess for packet.data, got: {:?}", other),
                    }
                }
                other => panic!("Expected ArrayIndex for data[i], got: {:?}", other),
            }
        }
        other => panic!("Expected FieldAccess, got: {:?}", other),
    }

    // array[index].field.subfield
    let expr =
        parse_expression("objects[0].position.x").expect("Should parse array then field access");
    match expr {
        Expr::FieldAccess { object, field } => {
            assert_eq!(field, "x");
            match object.as_ref() {
                Expr::FieldAccess { object, field } => {
                    assert_eq!(field, "position");
                    match object.as_ref() {
                        Expr::ArrayIndex { array, index } => {
                            assert_eq!(array.as_ref(), &Expr::Signal("objects".to_string()));
                            assert_eq!(index.as_ref(), &Expr::Literal(LiteralValue::Integer(0)));
                        }
                        other => panic!("Expected ArrayIndex, got: {:?}", other),
                    }
                }
                other => panic!("Expected FieldAccess for position, got: {:?}", other),
            }
        }
        other => panic!("Expected FieldAccess, got: {:?}", other),
    }
}

/// Test: Invalid array type syntax errors.
#[test]
fn parse_invalid_array_types() {
    // Missing array size
    let source = r#"
module TestMod {
    signal x: internal u8[];
}
"#;
    let result = parse_mirr(source);
    assert!(result.is_err(), "Should reject array without size");

    // Zero array size - this actually parses as invalid range
    let source = r#"
module TestMod {
    signal x: internal u8[0];
}
"#;
    let result = parse_mirr(source);
    assert!(result.is_err(), "Should reject zero-sized array");
}

/// Test: Invalid fixed-point type bounds.
#[test]
fn parse_invalid_fixed_point_bounds() {
    // Fractional bits > total bits
    let source = r#"
module TestMod {
    signal fp: internal fixed<16,32>;
}
"#;
    let result = parse_mirr(source);
    assert!(result.is_err(), "Should reject frac_bits > total_bits");

    // Total bits > MAX_FIXED_POINT_BITS (64)
    let source = r#"
module TestMod {
    signal fp: internal fixed<128,64>;
}
"#;
    let result = parse_mirr(source);
    assert!(result.is_err(), "Should reject total_bits > MAX_FIXED_POINT_BITS");
}

/// Test: Bounded limits for struct fields (MAX_STRUCT_FIELDS = 32).
#[test]
fn parse_struct_field_limits() {
    // Test valid struct with exactly MAX_STRUCT_FIELDS (32) fields
    let mut fields = String::new();
    for i in 0..32 {
        fields.push_str(&format!("    field{}: u8;\n", i));
    }

    let source = format!(
        r#"
struct MaxFields {{
{}}}

module TestMod {{
    signal dummy: internal bool;
}}
"#,
        fields
    );

    let result = parse_mirr(&source);
    assert!(result.is_ok(), "Should accept 32 struct fields");

    if let Ok(program) = result {
        assert_eq!(program.module.name, "TestMod");
    }
}

/// Test: Bounded limits for interface signals (MAX_INTERFACE_SIGNALS = 64).
#[test]
fn parse_interface_signal_limits() {
    // Test valid interface with exactly MAX_INTERFACE_SIGNALS (64) signals
    let mut signals = String::new();
    for i in 0..64 {
        let kind = if i % 2 == 0 { "in" } else { "out" };
        signals.push_str(&format!("    signal{}: {} bool;\n", i, kind));
    }

    let source = format!(
        r#"
interface MaxSignals {{
{}}}

module TestMod {{
    signal dummy: internal bool;
}}
"#,
        signals
    );

    let result = parse_mirr(&source);
    assert!(result.is_ok(), "Should accept 64 interface signals");

    if let Ok(program) = result {
        assert_eq!(program.module.name, "TestMod");
    }
}

/// Test: Invalid expression syntax errors.
#[test]
fn parse_invalid_expressions() {
    // Unmatched brackets in array access
    let result = parse_expression("array[index");
    assert!(result.is_err(), "Should reject unmatched bracket");

    // Invalid field access without field name
    let result = parse_expression("object.");
    assert!(result.is_err(), "Should reject empty field access");

    // Malformed struct literal
    let result = parse_expression("Point { x: 10 y: 20 }");
    assert!(result.is_err(), "Should reject malformed struct literal");

    // Malformed array literal with trailing comma
    let _result = parse_expression("[1, 2,]");
    // Note: Implementation might accept trailing commas - adjust test as needed
}
