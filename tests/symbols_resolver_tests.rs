#![forbid(unsafe_code)]

use std::path::PathBuf;

use mirrc::ast::program::{ImportDecl, MirrProgram, Module, SignalDecl};
use mirrc::ast::types::{ExtendedType, SignalKind, SignalType};
use mirrc::error::MirrError;
use mirrc::symbols::resolver::{create_resolver_for_program, CrossModuleResolver};
use mirrc::symbols::ModuleSymbols;

fn create_test_module(name: &str, signals: Vec<(&str, SignalKind, SignalType)>) -> Module {
    let signal_decls = signals
        .into_iter()
        .map(|(name, kind, ty)| SignalDecl {
            name: name.to_string(),
            kind,
            ty: ExtendedType::from_core(ty),
            origin: None,
            span: None,
        })
        .collect();

    Module {
        name: name.to_string(),
        signals: signal_decls,
        guards: Vec::new(),
        reflexes: Vec::new(),
        properties: Vec::new(),
        pattern_calls: Vec::new(),
        pattern_origins: Vec::new(),
        span: None,
    }
}

fn create_program(module: Module, imports: Vec<ImportDecl>) -> MirrProgram {
    MirrProgram { target: None, patterns: Vec::new(), imports, module }
}

#[allow(clippy::ptr_arg)]
fn mock_load_module(path: &PathBuf) -> Result<Module, MirrError> {
    let name = path.file_stem().unwrap().to_str().unwrap();
    if name == "sub" {
        Ok(create_test_module(
            "SubModule",
            vec![
                ("sub_clk", SignalKind::Input, SignalType::Bool),
                ("sub_data", SignalKind::Output, SignalType::Unsigned(8)),
                ("shared_name", SignalKind::Output, SignalType::Bool),
            ],
        ))
    } else if name == "other" {
        Ok(create_test_module(
            "OtherModule",
            vec![
                ("other_sig", SignalKind::Input, SignalType::Bool),
                ("shared_name", SignalKind::Output, SignalType::Bool),
            ],
        ))
    } else {
        Err(MirrError::SymbolError { message: "Not found".to_string(), span: None })
    }
}

#[test]
fn test_list_local_symbols() {
    let main_mod = create_test_module(
        "Main",
        vec![
            ("clk", SignalKind::Input, SignalType::Bool),
            ("reset", SignalKind::Input, SignalType::Bool),
        ],
    );
    let prog = create_program(main_mod, vec![]);
    let resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    let locals = resolver.list_local_symbols().unwrap();
    assert_eq!(locals.len(), 2);
    let names: Vec<_> = locals.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"clk"));
    assert!(names.contains(&"reset"));
}

#[test]
fn test_list_available_imports() {
    let main_mod = create_test_module("Main", vec![]);
    let imports = vec![
        ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None },
        ImportDecl { alias: "oth".to_string(), path: "other.mirr".to_string(), span: None },
    ];
    let prog = create_program(main_mod, imports);
    let resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    let avail = resolver.list_available_imports().unwrap();
    assert_eq!(avail.len(), 2);
    assert!(avail.contains(&"sub".to_string()));
    assert!(avail.contains(&"oth".to_string()));
}

#[test]
fn test_symbol_exists_and_get_type() {
    let main_mod =
        create_test_module("Main", vec![("local_sig", SignalKind::Input, SignalType::Bool)]);
    let imports =
        vec![ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None }];
    let prog = create_program(main_mod, imports);
    let resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    assert!(resolver.symbol_exists("local_sig"));
    assert!(resolver.symbol_exists("sub::sub_clk"));
    assert!(!resolver.symbol_exists("nonexistent"));
    assert!(!resolver.symbol_exists("sub::nonexistent"));

    let ty = resolver.get_symbol_type("local_sig").unwrap();
    assert_eq!(ty.core, SignalType::Bool);

    let ty_sub = resolver.get_symbol_type("sub::sub_data").unwrap();
    assert_eq!(ty_sub.core, SignalType::Unsigned(8));

    assert!(resolver.get_symbol_type("nope").is_err());
}

#[test]
fn test_is_symbol_visible() {
    let main_mod =
        create_test_module("Main", vec![("main_sig", SignalKind::Input, SignalType::Bool)]);
    let imports =
        vec![ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None }];
    let prog = create_program(main_mod, imports);
    let resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    let local_sym = resolver.symbol_table().resolve_local("main_sig").unwrap().clone();
    assert!(resolver.is_symbol_visible(&local_sym, &PathBuf::from("main.mirr")));

    let sub_sym = resolver
        .symbol_table()
        .get_module(&PathBuf::from("sub.mirr"))
        .unwrap()
        .symbols
        .get("sub_clk")
        .unwrap()
        .clone();
    assert!(resolver.is_symbol_visible(&sub_sym, &PathBuf::from("main.mirr")));

    // Not visible from sub
    assert!(!resolver.is_symbol_visible(&local_sym, &PathBuf::from("sub.mirr")));
}

#[test]
fn test_get_visible_symbols() {
    let main_mod =
        create_test_module("Main", vec![("main_sig", SignalKind::Input, SignalType::Bool)]);
    let imports =
        vec![ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None }];
    let prog = create_program(main_mod, imports);
    let resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    let visible = resolver.get_visible_symbols().unwrap();
    // main_sig (local), sub_clk (cross), sub_data (cross), shared_name (cross)
    assert_eq!(visible.len(), 4);
    let names: Vec<_> = visible.iter().map(|s| s.display_name()).collect();
    assert!(names.contains(&"main_sig".to_string()));
    assert!(names.contains(&"sub::sub_clk".to_string()));
}

#[test]
fn test_get_symbols_in_namespace() {
    let main_mod = create_test_module("Main", vec![]);
    let imports =
        vec![ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None }];
    let prog = create_program(main_mod, imports);
    let resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    let ns_syms = resolver.get_symbols_in_namespace("sub").unwrap();
    assert_eq!(ns_syms.len(), 3);
    assert!(resolver.get_symbols_in_namespace("bad_ns").is_err());
}

#[test]
fn test_check_symbol_conflicts() {
    let main_mod =
        create_test_module("Main", vec![("shared_name", SignalKind::Input, SignalType::Bool)]);
    let imports = vec![
        ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None },
        ImportDecl { alias: "oth".to_string(), path: "other.mirr".to_string(), span: None },
    ];
    let prog = create_program(main_mod, imports);
    let resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    let conflicts = resolver.check_symbol_conflicts().unwrap();
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts[0].symbol_name, "shared_name");
    assert_eq!(conflicts[0].conflicting_modules.len(), 3);
}

#[test]
fn test_validate_imports() {
    let main_mod = create_test_module("Main", vec![]);
    let imports =
        vec![ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None }];
    let prog = create_program(main_mod, imports);
    let mut resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    assert!(resolver.validate_imports().is_ok());

    // Switch context to a module with no imports
    resolver.set_current_module(PathBuf::from("sub.mirr"));
    assert!(resolver.validate_imports().is_ok());
}

#[test]
fn test_clear_cache_and_resolve() {
    let main_mod = create_test_module("Main", vec![]);
    let imports =
        vec![ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None }];
    let prog = create_program(main_mod, imports);
    let mut resolver =
        create_resolver_for_program(&prog, PathBuf::from("main.mirr"), mock_load_module).unwrap();

    let s1 = resolver.resolve_qualified_symbol("sub", "sub_clk", None).unwrap();
    let s2 = resolver.resolve_symbol("sub::sub_clk", None).unwrap();
    assert_eq!(s1, s2);

    // Test cache hit via stats
    let stats1 = resolver.get_stats();
    assert_eq!(stats1.cached_qualified_symbols, 1);

    resolver.clear_cache();
    let stats2 = resolver.get_stats();
    assert_eq!(stats2.cached_qualified_symbols, 0);
}

#[test]
fn test_unloaded_import_validation() {
    let main_mod = create_test_module("Main", vec![]);
    let imports =
        vec![ImportDecl { alias: "sub".to_string(), path: "sub.mirr".to_string(), span: None }];
    let _prog = create_program(main_mod.clone(), imports);

    // Create resolver manually to simulate a missing import load
    let mut symbol_table = mirrc::symbols::SymbolTable::new();
    let mut mod_syms = ModuleSymbols::new("Main".to_string(), PathBuf::from("main.mirr"));
    mod_syms
        .add_import(ImportDecl {
            alias: "sub".to_string(),
            path: "sub.mirr".to_string(),
            span: None,
        })
        .unwrap();
    symbol_table.add_module(mod_syms).unwrap();
    symbol_table.set_current_module(PathBuf::from("main.mirr"));

    let ctx = mirrc::symbols::resolver::ImportContext::new();
    let resolver = CrossModuleResolver::new(symbol_table, ctx);

    assert!(resolver.validate_imports().is_err());
}
