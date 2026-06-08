#![feature(box_patterns)]
#![forbid(unsafe_code)]

use mirrc::ast::Expr;
use mirrc::ast::types::LiteralValue;
use mirrc::parser::parse_expression;

fn ok_expr(s: &str) -> Expr {
    parse_expression(s).unwrap_or_else(|e| panic!("Failed to parse '{}': {:?}", s, e))
}
fn err_expr(s: &str) -> String {
    parse_expression(s).unwrap_err().to_string()
}

#[test]
fn test_01_bool_true() -> Result<(), String> {
    assert_eq!(ok_expr("true"), Expr::Literal(LiteralValue::Bool(true)));
    Ok(())
}

#[test]
fn test_02_bool_false() -> Result<(), String> {
    assert_eq!(ok_expr("false"), Expr::Literal(LiteralValue::Bool(false)));
    Ok(())
}

#[test]
fn test_03_integer_zero() -> Result<(), String> {
    assert_eq!(ok_expr("0"), Expr::Literal(LiteralValue::Integer(0)));
    Ok(())
}

#[test]
fn test_04_integer_decimal() -> Result<(), String> {
    assert_eq!(ok_expr("42"), Expr::Literal(LiteralValue::Integer(42)));
    Ok(())
}

#[test]
fn test_05_integer_hex() -> Result<(), String> {
    // Assuming hex is supported, usually parsed as u64 in tokenization or parsing
    // If not, this acts as a test for the literal parsing layer
    let e = ok_expr("0x2A");
    assert_eq!(e, Expr::Literal(LiteralValue::Integer(42)));
    Ok(())
}

#[test]
fn test_06_integer_binary_error() -> Result<(), String> {
    let err = err_expr("0b101010");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_07_signal_basic() -> Result<(), String> {
    assert_eq!(ok_expr("my_signal"), Expr::Signal("my_signal".into()));
    Ok(())
}

#[test]
fn test_08_signal_internal() -> Result<(), String> {
    assert_eq!(ok_expr("_internal"), Expr::Signal("_internal".into()));
    Ok(())
}

#[test]
fn test_09_field_access() -> Result<(), String> {
    let e = ok_expr("obj.field");
    assert_eq!(e, Expr::FieldAccess { object: Box::new(Expr::Signal("obj".into())), field: "field".into() });
    Ok(())
}

#[test]
fn test_10_field_access_nested() -> Result<(), String> {
    let e = ok_expr("obj.field.sub");
    assert!(matches!(e, Expr::FieldAccess { object: box Expr::FieldAccess { .. }, .. }));
    Ok(())
}

#[test]
fn test_11_array_literal_empty() -> Result<(), String> {
    let e = ok_expr("[]");
    assert!(matches!(e, Expr::ArrayLiteral(v) if v.is_empty()));
    Ok(())
}

#[test]
fn test_12_array_literal_single() -> Result<(), String> {
    let e = ok_expr("[1]");
    assert!(matches!(e, Expr::ArrayLiteral(v) if v.len() == 1));
    Ok(())
}

#[test]
fn test_13_array_literal_multiple() -> Result<(), String> {
    let e = ok_expr("[1, 2, 3]");
    assert!(matches!(e, Expr::ArrayLiteral(v) if v.len() == 3));
    Ok(())
}

#[test]
fn test_14_array_index_literal() -> Result<(), String> {
    let e = ok_expr("arr[3]");
    assert!(matches!(e, Expr::ArrayIndex { 
        array: box Expr::Signal(_), 
        index: box Expr::Literal(LiteralValue::Integer(3)) 
    }));
    Ok(())
}

#[test]
fn test_15_array_index_expr() -> Result<(), String> {
    let e = ok_expr("arr[a + 1]");
    assert!(matches!(e, Expr::ArrayIndex { 
        array: box Expr::Signal(_), 
        index: box Expr::Binary { .. } 
    }));
    Ok(())
}

#[test]
fn test_16_array_index_nested() -> Result<(), String> {
    let e = ok_expr("arr[0][1]");
    assert!(matches!(e, Expr::ArrayIndex { array: box Expr::ArrayIndex { .. }, .. }));
    Ok(())
}

#[test]
fn test_17_struct_init_empty() -> Result<(), String> {
    let e = ok_expr("MyStruct {}");
    assert!(matches!(e, Expr::StructLiteral { name, fields } if name == "MyStruct" && fields.is_empty()));
    Ok(())
}

#[test]
fn test_18_struct_init_single() -> Result<(), String> {
    let e = ok_expr("MyStruct { x: 1 }");
    assert!(matches!(e, Expr::StructLiteral { name, fields } if name == "MyStruct" && fields.len() == 1));
    Ok(())
}

#[test]
fn test_19_struct_init_multiple() -> Result<(), String> {
    let e = ok_expr("MyStruct { x: 1, y: 2 }");
    assert!(matches!(e, Expr::StructLiteral { fields, .. } if fields.len() == 2));
    Ok(())
}

#[test]
fn test_20_struct_init_nested() -> Result<(), String> {
    let e = ok_expr("Outer { inner: Inner { x: 1 } }");
    assert!(matches!(e, Expr::StructLiteral { fields, .. } if fields.len() == 1));
    Ok(())
}

#[test]
fn test_21_mixed_field_and_index() -> Result<(), String> {
    let e = ok_expr("obj.arr[0]");
    assert!(matches!(e, Expr::ArrayIndex { array: box Expr::FieldAccess { .. }, .. }));
    Ok(())
}

#[test]
fn test_22_mixed_index_and_field() -> Result<(), String> {
    let e = ok_expr("arr[0].field");
    assert!(matches!(e, Expr::FieldAccess { object: box Expr::ArrayIndex { .. }, .. }));
    Ok(())
}

#[test]
fn test_23_literal_error_empty() -> Result<(), String> {
    let err = err_expr("");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_24_literal_error_invalid_suffix() -> Result<(), String> {
    let err = err_expr("123foo");
    assert!(!err.is_empty());
    Ok(())
}

#[test]
fn test_25_struct_init_missing_brace() -> Result<(), String> {
    let err = err_expr("MyStruct { x: 1 ");
    assert!(!err.is_empty());
    Ok(())
}
