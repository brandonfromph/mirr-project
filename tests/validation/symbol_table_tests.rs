#![forbid(unsafe_code)]
pub use mirrc::ast;
pub use mirrc::error;
pub use mirrc::error_codes;
pub use mirrc::span;
#[rustfmt::skip]
#[path = "../src/symbols/table.rs"]
pub mod table;
use mirrc::ast::types::*;
use std::path::PathBuf;
use table::*;
#[test]
fn test_sym_new_1() {
    let st = SymbolTable::new();
    assert_eq!(st.modules.len(), 0);
}
#[test]
fn test_sym_new_2() {
    let st = SymbolTable::new();
    assert_eq!(st.modules.len(), 0);
}
#[test]
fn test_sym_new_3() {
    let st = SymbolTable::new();
    assert_eq!(st.modules.len(), 0);
}
#[test]
fn test_sym_new_4() {
    let st = SymbolTable::new();
    assert_eq!(st.modules.len(), 0);
}
#[test]
fn test_sym_new_5() {
    let st = SymbolTable::new();
    assert_eq!(st.modules.len(), 0);
}
#[test]
fn test_sym_new_6() {
    let st = SymbolTable::new();
    assert_eq!(st.modules.len(), 0);
}
#[test]
fn test_sym_push_pop_7() {
    let m = ModuleSymbols::new("m7".to_string(), PathBuf::from("m7.mirr"));
    assert_eq!(m.name, "m7");
}
#[test]
fn test_sym_push_pop_8() {
    let m = ModuleSymbols::new("m8".to_string(), PathBuf::from("m8.mirr"));
    assert_eq!(m.name, "m8");
}
#[test]
fn test_sym_push_pop_9() {
    let m = ModuleSymbols::new("m9".to_string(), PathBuf::from("m9.mirr"));
    assert_eq!(m.name, "m9");
}
#[test]
fn test_sym_push_pop_10() {
    let m = ModuleSymbols::new("m10".to_string(), PathBuf::from("m10.mirr"));
    assert_eq!(m.name, "m10");
}
#[test]
fn test_sym_push_pop_11() {
    let m = ModuleSymbols::new("m11".to_string(), PathBuf::from("m11.mirr"));
    assert_eq!(m.name, "m11");
}
#[test]
fn test_sym_push_pop_12() {
    let m = ModuleSymbols::new("m12".to_string(), PathBuf::from("m12.mirr"));
    assert_eq!(m.name, "m12");
}
#[test]
fn test_sym_insert_13() {
    let mut m = ModuleSymbols::new("m13".to_string(), PathBuf::from("m13.mirr"));
    let info = SymbolInfo {
        name: "sig13".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m13.mirr"),
    };
    let res = m.add_symbol(info);
    assert!(res.is_ok());
}
#[test]
fn test_sym_insert_14() {
    let mut m = ModuleSymbols::new("m14".to_string(), PathBuf::from("m14.mirr"));
    let info = SymbolInfo {
        name: "sig14".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m14.mirr"),
    };
    let res = m.add_symbol(info);
    assert!(res.is_ok());
}
#[test]
fn test_sym_insert_15() {
    let mut m = ModuleSymbols::new("m15".to_string(), PathBuf::from("m15.mirr"));
    let info = SymbolInfo {
        name: "sig15".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m15.mirr"),
    };
    let res = m.add_symbol(info);
    assert!(res.is_ok());
}
#[test]
fn test_sym_insert_16() {
    let mut m = ModuleSymbols::new("m16".to_string(), PathBuf::from("m16.mirr"));
    let info = SymbolInfo {
        name: "sig16".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m16.mirr"),
    };
    let res = m.add_symbol(info);
    assert!(res.is_ok());
}
#[test]
fn test_sym_insert_17() {
    let mut m = ModuleSymbols::new("m17".to_string(), PathBuf::from("m17.mirr"));
    let info = SymbolInfo {
        name: "sig17".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m17.mirr"),
    };
    let res = m.add_symbol(info);
    assert!(res.is_ok());
}
#[test]
fn test_sym_insert_18() {
    let mut m = ModuleSymbols::new("m18".to_string(), PathBuf::from("m18.mirr"));
    let info = SymbolInfo {
        name: "sig18".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m18.mirr"),
    };
    let res = m.add_symbol(info);
    assert!(res.is_ok());
}
#[test]
fn test_sym_duplicate_19() {
    let mut m = ModuleSymbols::new("m19".to_string(), PathBuf::from("m19.mirr"));
    let info = SymbolInfo {
        name: "sig19".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m19.mirr"),
    };
    let res1 = m.add_symbol(info.clone());
    let res2 = m.add_symbol(info);
    assert!(res1.is_ok());
    assert!(res2.is_err());
}
#[test]
fn test_sym_duplicate_20() {
    let mut m = ModuleSymbols::new("m20".to_string(), PathBuf::from("m20.mirr"));
    let info = SymbolInfo {
        name: "sig20".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m20.mirr"),
    };
    let res1 = m.add_symbol(info.clone());
    let res2 = m.add_symbol(info);
    assert!(res1.is_ok());
    assert!(res2.is_err());
}
#[test]
fn test_sym_duplicate_21() {
    let mut m = ModuleSymbols::new("m21".to_string(), PathBuf::from("m21.mirr"));
    let info = SymbolInfo {
        name: "sig21".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m21.mirr"),
    };
    let res1 = m.add_symbol(info.clone());
    let res2 = m.add_symbol(info);
    assert!(res1.is_ok());
    assert!(res2.is_err());
}
#[test]
fn test_sym_duplicate_22() {
    let mut m = ModuleSymbols::new("m22".to_string(), PathBuf::from("m22.mirr"));
    let info = SymbolInfo {
        name: "sig22".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m22.mirr"),
    };
    let res1 = m.add_symbol(info.clone());
    let res2 = m.add_symbol(info);
    assert!(res1.is_ok());
    assert!(res2.is_err());
}
#[test]
fn test_sym_duplicate_23() {
    let mut m = ModuleSymbols::new("m23".to_string(), PathBuf::from("m23.mirr"));
    let info = SymbolInfo {
        name: "sig23".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m23.mirr"),
    };
    let res1 = m.add_symbol(info.clone());
    let res2 = m.add_symbol(info);
    assert!(res1.is_ok());
    assert!(res2.is_err());
}
#[test]
fn test_sym_duplicate_24() {
    let mut m = ModuleSymbols::new("m24".to_string(), PathBuf::from("m24.mirr"));
    let info = SymbolInfo {
        name: "sig24".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m24.mirr"),
    };
    let res1 = m.add_symbol(info.clone());
    let res2 = m.add_symbol(info);
    assert!(res1.is_ok());
    assert!(res2.is_err());
}
#[test]
fn test_sym_duplicate_25() {
    let mut m = ModuleSymbols::new("m25".to_string(), PathBuf::from("m25.mirr"));
    let info = SymbolInfo {
        name: "sig25".to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from("m25.mirr"),
    };
    let res1 = m.add_symbol(info.clone());
    let res2 = m.add_symbol(info);
    assert!(res1.is_ok());
    assert!(res2.is_err());
}
