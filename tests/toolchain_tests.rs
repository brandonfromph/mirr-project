//! Integration tests for src/toolchain/ module.

#![forbid(unsafe_code)]

use mirrc::toolchain::eqy::*;
use mirrc::toolchain::sby::*;
use mirrc::toolchain::*;
use std::path::Path;

#[test]
fn test_tool_registry_new_is_empty() {
    let reg = ToolRegistry::new();
    assert!(!reg.is_available(Tool::Yosys));
    assert!(!reg.is_available(Tool::Sby));
    assert!(!reg.is_available(Tool::Verilator));
}

#[test]
fn test_tool_binary_names_exhaustive() {
    // Every Tool variant has a non-empty binary name
    let tools = [
        Tool::Yosys,
        Tool::Sby,
        Tool::Verilator,
        Tool::IcarusVerilog,
        Tool::NextpnrIce40,
        Tool::NextpnrEcp5,
        Tool::NextpnrNexus,
        Tool::Icepack,
        Tool::Icetime,
        Tool::Ecppack,
        Tool::Eqy,
    ];
    for tool in tools {
        assert!(!tool.binary_name().is_empty(), "{:?} has empty binary name", tool);
    }
}

#[test]
fn test_normalize_path_backslashes() {
    let p = Path::new("C:\\Users\\test\\file.sv");
    let norm = normalize_path_for_mingw(p);
    assert!(!norm.contains('\\'));
    assert!(norm.contains("C:/Users/test/file.sv"));
}

#[test]
fn test_normalize_path_forward_slashes_unchanged() {
    let p = Path::new("/tmp/test/file.sv");
    assert_eq!(normalize_path_for_mingw(p), "/tmp/test/file.sv");
}

#[test]
fn test_sby_config_bmc_default() {
    let cfg = SbyConfig::default();
    assert_eq!(cfg.bmc_depth, 20);
    assert!(!cfg.prove);
    assert_eq!(cfg.engine, SbyEngine::Z3);
}

#[test]
fn test_sby_config_bmc_generation() {
    let cfg = SbyConfig::default();
    let out = generate_sby_config("mod1", Path::new("mod1.sv"), None, &cfg);
    assert!(out.contains("[tasks]"));
    assert!(out.contains("bmc"));
    assert!(out.contains("depth 20"));
    assert!(out.contains("smtbmc z3"));
    assert!(out.contains("-top mod1"));
}

#[test]
fn test_sby_config_with_prove_and_bind() {
    let cfg = SbyConfig { prove: true, engine: SbyEngine::Yices, ..Default::default() };
    let out = generate_sby_config("top", Path::new("top.sv"), Some(Path::new("top_bind.sv")), &cfg);
    assert!(out.contains("prove"));
    assert!(out.contains("mode prove"));
    assert!(out.contains("smtbmc yices"));
    assert!(out.contains("top_bind.sv"));
}

#[test]
fn test_sby_depth_clamped_to_max() {
    let cfg = SbyConfig { bmc_depth: 9999, ..Default::default() };
    let out = generate_sby_config("m", Path::new("m.sv"), None, &cfg);
    assert!(out.contains("depth 200")); // MAX_BMC_DEPTH
}

#[test]
fn test_sby_engine_from_str() {
    assert_eq!(SbyEngine::from_str_name("z3"), Some(SbyEngine::Z3));
    assert_eq!(SbyEngine::from_str_name("yices"), Some(SbyEngine::Yices));
    assert_eq!(SbyEngine::from_str_name("bitwuzla"), Some(SbyEngine::Bitwuzla));
    assert_eq!(SbyEngine::from_str_name("btor"), Some(SbyEngine::Boolector));
    assert_eq!(SbyEngine::from_str_name("magic"), None);
}

#[test]
fn test_eqy_config_generation() {
    let out = generate_eqy_config("top", Path::new("gold.sv"), Path::new("gate.sv"));
    assert!(out.contains("[gold]"));
    assert!(out.contains("[gate]"));
    assert!(out.contains("gold.sv"));
    assert!(out.contains("gate.sv"));
    assert!(out.contains("-top top"));
    assert!(out.contains("[strategy simple]"));
}

#[test]
fn test_eqy_config_mingw_paths() {
    let out = generate_eqy_config("t", Path::new("C:\\a\\gold.sv"), Path::new("C:\\a\\gate.sv"));
    assert!(!out.contains('\\'));
}

#[test]
fn test_invoke_tool_not_found() {
    let reg = ToolRegistry::new(); // empty registry
    let result = invoke_tool(&reg, Tool::Yosys, &["--version"], Path::new("."));
    assert!(result.is_err());
}

#[test]
fn test_toolchain_error_display_messages() {
    let e1 = ToolchainError::ToolNotFound { tool: "yosys".into() };
    assert!(e1.to_string().contains("yosys"));
    assert!(e1.to_string().contains("not found"));

    let e2 = ToolchainError::Invocation { tool: "sby".into(), message: "spawn failed".into() };
    assert!(e2.to_string().contains("sby"));
    assert!(e2.to_string().contains("spawn failed"));

    let e3 = ToolchainError::ToolFailed {
        tool: "verilator".into(),
        exit_code: Some(1),
        stderr: "error line\nsecond".into(),
    };
    assert!(e3.to_string().contains("verilator"));
    assert!(e3.to_string().contains("error line"));

    let e4 = ToolchainError::ParseError { tool: "icetime".into(), message: "bad format".into() };
    assert!(e4.to_string().contains("icetime"));
}

#[test]
fn test_tool_version_flags_non_empty() {
    let tools = [
        Tool::Yosys,
        Tool::Sby,
        Tool::Verilator,
        Tool::IcarusVerilog,
        Tool::NextpnrIce40,
        Tool::NextpnrEcp5,
        Tool::NextpnrNexus,
        Tool::Icepack,
        Tool::Icetime,
        Tool::Ecppack,
        Tool::Eqy,
    ];

    for tool in tools {
        assert!(!tool.version_flag().is_empty(), "{:?} has empty version flag", tool);
    }
}

#[test]
fn test_toolchain_constants_are_sane() {
    let mut reg = ToolRegistry::new();
    reg.probe_all();
    assert!(reg.tools.len() <= MAX_TOOLS, "registry size must stay within MAX_TOOLS bound");

    let version_sample = "v".repeat(MAX_VERSION_LEN + 8);
    let clamped = if version_sample.len() > MAX_VERSION_LEN {
        version_sample[..MAX_VERSION_LEN].to_string()
    } else {
        version_sample
    };
    assert_eq!(clamped.len(), MAX_VERSION_LEN);

    let timeout = std::time::Duration::from_secs(MAX_TOOL_TIMEOUT_SECS);
    assert!(std::time::Instant::now().elapsed() <= timeout);
}

#[test]
fn test_registry_default_matches_new() {
    let default_registry: ToolRegistry = Default::default();
    let new_registry = ToolRegistry::new();
    assert_eq!(default_registry.tools.len(), new_registry.tools.len());
}

#[test]
fn test_unavailable_tool_entry_has_no_version() {
    let mut reg = ToolRegistry::new();
    reg.tools.insert(
        Tool::Yosys,
        ToolInfo { path: "yosys".to_string(), version: "0.0".to_string(), available: false },
    );

    assert!(!reg.is_available(Tool::Yosys));
    assert!(reg.version(Tool::Yosys).is_none());
}
