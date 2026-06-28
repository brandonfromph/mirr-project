#![allow(deprecated)]

use mirrc::ast::program::{ImportDecl, Module, SignalDecl};
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::error::MirrError;
use mirrc::symbols::table::{
    ModuleSymbols, SymbolInfo, SymbolTable, MAX_IMPORT_ALIASES, MAX_MODULES, MAX_SYMBOLS_PER_MODULE,
};
use std::path::PathBuf;

fn mock_signal(name: &str) -> SignalDecl {
    SignalDecl {
        name: name.to_string(),
        kind: SignalKind::Input,
        ty: ExtendedType::from_core(SignalType::Bool),
        origin: None,
        span: None,
    }
}

fn mock_module(name: &str, num_signals: usize) -> Module {
    let mut signals = Vec::new();
    for i in 0..num_signals {
        signals.push(mock_signal(&format!("sig_{}", i)));
    }
    Module {
        name: name.to_string(),
        signals,
        guards: vec![],
        reflexes: vec![],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    }
}

fn mock_import(alias: &str, path: &str) -> ImportDecl {
    ImportDecl { alias: alias.to_string(), path: path.to_string(), span: None }
}

#[test]
fn test_symbol_table_basic_local_resolution() {
    let mut table = SymbolTable::new();
    let mut mod_syms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    mod_syms
        .add_symbol(SymbolInfo::from_signal(&mock_signal("clk"), PathBuf::from("main.mirr")))
        .unwrap();
    table.add_module(mod_syms).unwrap();
    table.set_current_module(PathBuf::from("main.mirr"));

    let resolved = table.resolve_local("clk").expect("should resolve");
    assert_eq!(resolved.name, "clk");
    assert!(table.resolve_local("missing").is_err());
}

#[test]
fn test_symbol_table_duplicate_symbol_error() {
    let mut mod_syms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    let sym = SymbolInfo::from_signal(&mock_signal("clk"), PathBuf::from("main.mirr"));
    mod_syms.add_symbol(sym.clone()).unwrap();
    let err = mod_syms.add_symbol(sym).unwrap_err();
    match err {
        MirrError::SymbolError { message, .. } => assert!(message.contains("already defined")),
        _ => panic!("Expected SymbolError"),
    }
}

#[test]
fn test_symbol_table_max_symbols() {
    let mut mod_syms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    for i in 0..MAX_SYMBOLS_PER_MODULE {
        mod_syms
            .add_symbol(SymbolInfo::from_signal(
                &mock_signal(&format!("s_{}", i)),
                PathBuf::from("main.mirr"),
            ))
            .unwrap();
    }
    let err = mod_syms
        .add_symbol(SymbolInfo::from_signal(&mock_signal("overflow"), PathBuf::from("main.mirr")))
        .unwrap_err();
    match err {
        MirrError::SymbolError { message, .. } => {
            assert!(message.contains("exceeds maximum symbol count"))
        }
        _ => panic!("Expected SymbolError"),
    }
}

#[test]
fn test_symbol_table_import_aliases() {
    let mut mod_syms = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    mod_syms.add_import(mock_import("alias1", "path1.mirr")).unwrap();

    // Duplicate alias
    let err = mod_syms.add_import(mock_import("alias1", "path2.mirr")).unwrap_err();
    match err {
        MirrError::SymbolError { message, .. } => assert!(message.contains("already defined")),
        _ => panic!("Expected SymbolError"),
    }

    // Max aliases
    let mut mod_syms = ModuleSymbols::new("main2".to_string(), PathBuf::from("main2.mirr"));
    for i in 0..MAX_IMPORT_ALIASES {
        mod_syms.add_import(mock_import(&format!("a_{}", i), &format!("p_{}.mirr", i))).unwrap();
    }
    let err = mod_syms.add_import(mock_import("overflow", "over.mirr")).unwrap_err();
    match err {
        MirrError::SymbolError { message, .. } => {
            assert!(message.contains("exceeds maximum import count"))
        }
        _ => panic!("Expected SymbolError"),
    }
}

#[test]
fn test_symbol_table_max_modules() {
    let mut table = SymbolTable::new();
    for i in 0..MAX_MODULES {
        let mod_syms =
            ModuleSymbols::new(format!("m_{}", i), PathBuf::from(format!("m_{}.mirr", i)));
        table.add_module(mod_syms).unwrap();
    }
    let err = table
        .add_module(ModuleSymbols::new("overflow".to_string(), PathBuf::from("o.mirr")))
        .unwrap_err();
    match err {
        MirrError::SymbolError { message, .. } => {
            assert!(message.contains("exceeds maximum module count"))
        }
        _ => panic!("Expected SymbolError"),
    }
}

#[test]
fn test_symbol_table_qualified_resolution() {
    let mut table = SymbolTable::new();

    // Target module
    let mut target_mod = ModuleSymbols::new("target".to_string(), PathBuf::from("target.mirr"));
    target_mod
        .add_symbol(SymbolInfo::from_signal(
            &mock_signal("remote_sig"),
            PathBuf::from("target.mirr"),
        ))
        .unwrap();
    table.add_module(target_mod).unwrap();

    // Main module importing target
    let mut main_mod = ModuleSymbols::new("main".to_string(), PathBuf::from("main.mirr"));
    main_mod.add_import(mock_import("t", "target.mirr")).unwrap();
    table.add_module(main_mod).unwrap();

    table.set_current_module(PathBuf::from("main.mirr"));

    let resolved = table.resolve_qualified("t", "remote_sig").expect("should resolve");
    assert_eq!(resolved.name, "remote_sig");

    assert!(table.resolve_qualified("t", "missing").is_err());
    assert!(table.resolve_qualified("missing_alias", "remote_sig").is_err());
}

#[test]
fn test_symbol_table_from_module() {
    let main_mod = mock_module("main", 2);
    let target_mod = mock_module("target", 1);

    let imports = vec![mock_import("t", "target.mirr")];

    let loader = |path: &PathBuf| -> Result<(Module, Vec<ImportDecl>), MirrError> {
        if path.to_str().unwrap() == "target.mirr" {
            Ok((target_mod.clone(), vec![]))
        } else {
            panic!("Unexpected module load")
        }
    };

    let table =
        SymbolTable::from_module(&main_mod, &imports, PathBuf::from("main.mirr"), loader).unwrap();

    assert_eq!(table.modules.len(), 2);
    let resolved =
        table.resolve_qualified("t", "sig_0").expect("should resolve from loaded module");
    assert_eq!(resolved.name, "sig_0");
}
