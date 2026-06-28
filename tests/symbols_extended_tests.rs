#![forbid(unsafe_code)]

use std::path::PathBuf;

use mirrc::ast::program::{ImportDecl, Module};
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::error::MirrError;
use mirrc::symbols::table::{
    ModuleSymbols, SymbolInfo, SymbolTable, MAX_IMPORT_ALIASES, MAX_MODULES, MAX_SYMBOLS_PER_MODULE,
};

fn create_symbol_info(name: &str, module: &str) -> SymbolInfo {
    SymbolInfo {
        name: name.to_string(),
        kind: SignalKind::Internal,
        ty: ExtendedType::from_core(SignalType::Bool),
        span: None,
        module_path: PathBuf::from(module),
    }
}

#[test]
fn add_symbol_success() {
    let mut ms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    let si1 = create_symbol_info("sig1", "main.mirr");
    assert!(ms.add_symbol(si1).is_ok());
}

#[test]
fn add_symbol_duplicate_fails_e902() {
    let mut ms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    let si1 = create_symbol_info("sig1", "main.mirr");
    ms.add_symbol(si1.clone()).unwrap();
    let res = ms.add_symbol(si1);
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn symbol_info_signal_type() {
    let si1 = create_symbol_info("sig1", "main.mirr");
    assert_eq!(si1.signal_type(), SignalType::Bool);
}

#[test]
fn add_symbol_exceeds_max_e901() {
    let mut ms_max = ModuleSymbols::new("max_sym".to_string(), PathBuf::from("max_sym.mirr"));
    for i in 0..MAX_SYMBOLS_PER_MODULE {
        ms_max.add_symbol(create_symbol_info(&format!("s{}", i), "max_sym.mirr")).unwrap();
    }
    let res = ms_max.add_symbol(create_symbol_info("overflow", "max_sym.mirr"));
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn add_import_exceeds_max_e903() {
    let mut ms_imports = ModuleSymbols::new("max_imp".to_string(), PathBuf::from("max_imp.mirr"));
    for i in 0..MAX_IMPORT_ALIASES {
        ms_imports
            .add_import(ImportDecl {
                alias: format!("a{}", i),
                path: format!("path{}.mirr", i),
                span: None,
            })
            .unwrap();
    }
    let res = ms_imports.add_import(ImportDecl {
        alias: "overflow".to_string(),
        path: "overflow.mirr".to_string(),
        span: None,
    });
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn add_import_duplicate_alias_e904() {
    let mut ms_dup_imp = ModuleSymbols::new("dup_imp".to_string(), PathBuf::from("dup_imp.mirr"));
    ms_dup_imp
        .add_import(ImportDecl {
            alias: "dup".to_string(),
            path: "dup1.mirr".to_string(),
            span: None,
        })
        .unwrap();
    let res = ms_dup_imp.add_import(ImportDecl {
        alias: "dup".to_string(),
        path: "dup2.mirr".to_string(),
        span: None,
    });
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn import_scope_has_alias() {
    let mut ms_dup_imp = ModuleSymbols::new("dup_imp".to_string(), PathBuf::from("dup_imp.mirr"));
    ms_dup_imp
        .add_import(ImportDecl {
            alias: "dup".to_string(),
            path: "dup1.mirr".to_string(),
            span: None,
        })
        .unwrap();
    let scope = ms_dup_imp.import_scope();
    assert!(scope.has_alias("dup"));
    assert!(!scope.has_alias("none"));
}

#[test]
fn symbol_table_get_module_mut() {
    let mut table = SymbolTable::default();
    let ms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    table.add_module(ms).unwrap();
    assert!(table.get_module_mut(&PathBuf::from("main.mirr")).is_some());
}

#[test]
fn resolve_no_current_module_e906() {
    let empty_table = SymbolTable::new();
    let res = empty_table.resolve_local("sig1");
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
    let res = empty_table.resolve_qualified("alias", "sig1");
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn resolve_qualified_unknown_alias_e907() {
    let mut table = SymbolTable::default();
    let ms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    table.add_module(ms).unwrap();
    table.set_current_module(PathBuf::from("main.mirr"));
    let res = table.resolve_qualified("unknown_alias", "sig1");
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn resolve_qualified_imported_module_not_in_table_e908() {
    let mut table = SymbolTable::default();
    let mut ms_with_import = ModuleSymbols::new("main2".to_string(), PathBuf::from("main2.mirr"));
    ms_with_import
        .add_import(ImportDecl {
            alias: "target".to_string(),
            path: "target.mirr".to_string(),
            span: None,
        })
        .unwrap();
    table.add_module(ms_with_import).unwrap();
    table.set_current_module(PathBuf::from("main2.mirr"));
    let res = table.resolve_qualified("target", "sig1");
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn resolve_qualified_symbol_not_found_e909() {
    let mut table = SymbolTable::default();
    let mut ms_with_import = ModuleSymbols::new("main2".to_string(), PathBuf::from("main2.mirr"));
    ms_with_import
        .add_import(ImportDecl {
            alias: "target".to_string(),
            path: "target.mirr".to_string(),
            span: None,
        })
        .unwrap();
    table.add_module(ms_with_import).unwrap();

    let target_ms = ModuleSymbols::new("target".to_string(), PathBuf::from("target.mirr"));
    table.add_module(target_ms).unwrap();

    table.set_current_module(PathBuf::from("main2.mirr"));
    let res = table.resolve_qualified("target", "unknown_sig");
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn resolve_local_symbol_not_found_e910() {
    let mut table = SymbolTable::default();
    let ms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    table.add_module(ms).unwrap();
    table.set_current_module(PathBuf::from("main.mirr"));
    let res = table.resolve_local("unknown_sig");
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
fn add_module_exceeds_max_e905() {
    let mut table_max = SymbolTable::new();
    for i in 0..MAX_MODULES {
        table_max
            .add_module(ModuleSymbols::new(
                format!("m{}", i),
                PathBuf::from(format!("m{}.mirr", i)),
            ))
            .unwrap();
    }
    let res = table_max
        .add_module(ModuleSymbols::new("overflow".to_string(), PathBuf::from("overflow.mirr")));
    assert!(matches!(res, Err(MirrError::SymbolError { .. })));
}

#[test]
#[allow(deprecated)]
fn symbol_table_from_module_deprecated() {
    let res = SymbolTable::from_module(
        &Module {
            name: "test".to_string(),
            signals: vec![],
            reflexes: vec![],
            properties: vec![],
            guards: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        },
        &[],
        PathBuf::from("test.mirr"),
        |_| Err(MirrError::parse_error("fake load")),
    );
    assert!(res.is_ok());
}
