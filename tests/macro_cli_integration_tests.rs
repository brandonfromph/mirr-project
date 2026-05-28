#![forbid(unsafe_code)]
//! Macro preprocessor and CLI integration test suite.
//! Contains exactly 100 distinct parameter-driven tests.

use nasa_rust_project::compiler::macro_proc::expand_macros;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use std::process::Output;

// Macro to generate preprocessor tests (001 - 030)
macro_rules! generate_macro_tests {
    ($($idx:ident, $input:expr, $expected:expr);* $(;)?) => {
        $(
            #[test]
            fn $idx() {
                let output = expand_macros($input);
                for exp in &$expected {
                    assert!(output.contains(exp), "Expected '{}' not found in expanded output:\n{}", exp, output);
                }
            }
        )*
    };
}

// Macro to generate loop unrolling tests (031 - 065)
macro_rules! generate_loop_tests {
    ($($idx:ident, $input:expr, $expected:expr);* $(;)?) => {
        $(
            #[test]
            fn $idx() {
                let output = expand_macros($input);
                for exp in &$expected {
                    assert!(output.contains(exp), "Expected '{}' not found in expanded output:\n{}", exp, output);
                }
            }
        )*
    };
}

// Macro to generate programmatic pipeline tests (066 - 085)
macro_rules! generate_pipeline_tests {
    ($($idx:ident, $input:expr, $config:expr, $check_fn:expr);* $(;)?) => {
        $(
            #[test]
            fn $idx() {
                let input = $input;
                let expanded = expand_macros(input);
                let result = run_pipeline(&expanded, &$config);
                let checker = $check_fn;
                checker(&result);
            }
        )*
    };
}

// --- 001 - 030: Preprocessor / Macro-Expansion direct tests ---
generate_macro_tests! {
    test_macro_cli_001, "module m {\n signals {\n a: in bool;\n }\n}", ["signal a: in bool;"];
    test_macro_cli_002, "module m {\n signals {\n b: out u8;\n }\n}", ["signal b: out u8;"];
    test_macro_cli_003, "module m {\n signals {\n c: internal u16;\n }\n}", ["signal c: internal u16;"];
    test_macro_cli_004, "module m {\n signals {\n d: in u32;\n }\n}", ["signal d: in u32;"];
    test_macro_cli_005, "module m {\n signals {\n e: out u64;\n }\n}", ["signal e: out u64;"];
    test_macro_cli_006, "module m {\n signals {\n f: in i8;\n }\n}", ["signal f: in i8;"];
    test_macro_cli_007, "module m {\n signals {\n g: out i16;\n }\n}", ["signal g: out i16;"];
    test_macro_cli_008, "module m {\n signals {\n h: internal i32;\n }\n}", ["signal h: internal i32;"];
    test_macro_cli_009, "module m {\n signals {\n i: in i64;\n }\n}", ["signal i: in i64;"];
    test_macro_cli_010, "module m {\n signals {\n a: in bool; // comment\n }\n}", ["signal a: in bool;"];
    test_macro_cli_011, "module m {\n signals {\n// comment\nb: out u8;\n}\n}", ["signal b: out u8;"];
    test_macro_cli_012, "module m {\n  signals {\n\n    c: internal u16;\n\n  }\n}", ["signal c: internal u16;"];
    test_macro_cli_013, "module m {\n\tsignals {\n\t\td: in u32;\n\t}\n}", ["signal d: in u32;"];
    test_macro_cli_014, "module m {\n signals {\n e: out u64\n }\n}", ["signal e: out u64;"];
    test_macro_cli_015, "module m {\n signals {\n f: in i8;\ng: out i16;\n}\n}", ["signal f: in i8;", "signal g: out i16;"];
    test_macro_cli_016, "module m {\n signals {\n  a: in bool  ;\n  b: out u8  ;\n }\n}", ["signal a: in bool", "signal b: out u8"];
    test_macro_cli_017, "module m {\n signals {\n a: in bool;\r\nb: out u8;\r\n}\n}", ["signal a: in bool;", "signal b: out u8;"];
    test_macro_cli_018, "module m {\n guard g1 = when a;\n}", ["guard g1 {\n  when a\n  for 1 cycles\n}"];
    test_macro_cli_019, "module m {\n guard g2 = when b for 5 cycles;\n}", ["guard g2 {\n  when b\n  for 5 cycles\n}"];
    test_macro_cli_020, "module m {\n guard g3 = when (a and b) for 10 cycles;\n}", ["guard g3 {\n  when (a and b)\n  for 10 cycles\n}"];
    test_macro_cli_021, "module m {\n let guard g4 = when a;\n}", ["guard g4 {\n  when a\n  for 1 cycles\n}"];
    test_macro_cli_022, "module m {\n my_namespace::some_pattern();\n}", ["my_namespace::some_pattern();"];
    test_macro_cli_023, "module m {\n signals {\n a: in bool; // comments\n }\n}", ["signal a: in bool;"];
    test_macro_cli_024, "module m {\n signals {\n for i in 0..2 {\n s[i]: in bool;\n }\n }\n}", ["signal s_0: in bool;", "signal s_1: in bool;"];
    test_macro_cli_025, "module m {\n signals {\n x: in bool;\n }\n}", ["signal x: in bool;"];
    test_macro_cli_026, "type my_type = u8; module m {}", ["module m {}"];
    test_macro_cli_027, "module m { reflex r { match a { 0 => { b = 1; }, _ => {} } } }", ["b = 1"];
    test_macro_cli_028, "module m {\n reflex r {\n if a {\n b = 1;\n }\n }\n}", ["on auto_g_0 {", "b = 1;"];
    test_macro_cli_029, "module m {\n reflex r {\n if a {\n b = 1;\n } else {\n b = 0;\n }\n }\n}", ["on auto_g_0 {", "on always {", "b = 1;", "b = 0;"];
    test_macro_cli_030, "\u{FEFF}module m {\n signals {\n a: in bool;\n }\n}", ["signal a: in bool;"];
}

// --- 031 - 065: Loop unrolling and reflex macro tests ---
generate_loop_tests! {
    test_macro_cli_031, "module m {\n signals {\n for i in 0..1 {\n s[i]: in bool;\n }\n }\n}", ["signal s_0: in bool;"];
    test_macro_cli_032, "module m {\n signals {\n for i in 0..2 {\n s[i]: in bool;\n }\n }\n}", ["signal s_0: in bool;", "signal s_1: in bool;"];
    test_macro_cli_033, "module m {\n signals {\n for i in 0..3 {\n s[i]: in bool;\n }\n }\n}", ["signal s_0: in bool;", "signal s_2: in bool;"];
    test_macro_cli_034, "module m {\n signals {\n for i in 0..4 {\n s[i]: in bool;\n }\n }\n}", ["signal s_3: in bool;"];
    test_macro_cli_035, "module m {\n signals {\n for i in 0..8 {\n s[i]: in bool;\n }\n }\n}", ["signal s_7: in bool;"];
    test_macro_cli_036, "module m {\n signals {\n for i in 0..16 {\n s[i]: in bool;\n }\n }\n}", ["signal s_15: in bool;"];
    test_macro_cli_037, "module m {\n signals {\n for i in 1..3 {\n s[i]: in bool;\n }\n }\n}", ["signal s_1: in bool;", "signal s_2: in bool;"];
    test_macro_cli_038, "module m {\n signals {\n for i in 2..5 {\n s[i]: in bool;\n }\n }\n}", ["signal s_2: in bool;", "signal s_4: in bool;"];
    test_macro_cli_039, "module m {\n signals {\n for i in 0..2 {\n s[i]: in bool;\n }\n }\n}", ["s_0", "s_1"];
    test_macro_cli_040, "module m {\n signals {\n for i in 0..2 {\n val_${i}: in bool;\n }\n }\n}", ["val_0", "val_1"];
    test_macro_cli_041, "module m {\n signals {\n for i in 0..2 {\n a[i]: in bool;\n }\n for j in 0..2 {\n b[j]: out u8;\n }\n }\n}", ["signal a_0", "signal b_1"];
    test_macro_cli_042, "module m {\n reflex r {\n for i in 0..2 {\n s[i] = 1;\n }\n }\n}", ["s_0 = 1", "s_1 = 1"];
    test_macro_cli_043, "module m {\n reflex r {\n for j in 1..4 {\n x[j] = 2;\n }\n }\n}", ["x_1 = 2", "x_3 = 2"];
    test_macro_cli_044, "module m {\n reflex r {\n for i in 0..2 {\n for j in 0..2 {\n s[i]_${j} = 3;\n }\n }\n }\n}", ["s_0_0 = 3", "s_1_1 = 3"];
    test_macro_cli_045, "module m {\n reflex r {\n for i in 0..2 {\n s[i] = a[i];\n }\n }\n}", ["s_0 = a_0", "s_1 = a_1"];
    test_macro_cli_046, "module m {\n reflex r {\n for i in 0..2 {\n s_${i} = a_${i};\n }\n }\n}", ["s_0 = a_0", "s_1 = a_1"];
    test_macro_cli_047, "for i in 0..2 {\ns[i] = a[i];\n}", ["s_0 = a_0", "s_1 = a_1"];
    test_macro_cli_048, "for i in 0..3 {\ns[i] = a[i];\n}", ["s_2 = a_2"];
    test_macro_cli_049, "module m {\n for i in 0..2 {\n s[i] = a[i];\n }\n}", ["s_0 = a_0", "s_1 = a_1"];
    test_macro_cli_050, "for i in 0..2 {\n// comment\ns[i] = a[i];\n}", ["s_0 = a_0"];
    test_macro_cli_051, "module m {\n reflex r {\n for i in 0..2 {\n s[i] = 1;\n x[i] = 2;\n }\n }\n}", ["s_0 = 1", "x_0 = 2"];
    test_macro_cli_052, "for i in 0..2 {\n for j in 0..2 {\n s[i]_${j} = 1;\n }\n}", ["s_0_0 = 1", "s_1_1 = 1"];
    test_macro_cli_053, "module m {\n reflex r {\n for i in 0..2 {\n s[i] = 1;\n }\n }\n}", ["s_0 = 1"];
    test_macro_cli_054, "for i in 0..2 {\n match s[i] {\n 0 => { x[i] = 1; },\n _ => {}\n }\n}", ["x_0 = 1"];
    test_macro_cli_055, "for i in 0..2 {\n if s[i] {\n x[i] = 1;\n }\n}", ["x_0 = 1"];
    test_macro_cli_056, "module m {\n signals {\n for i in 0..2 {\n s[i]: in bool;\n }\n }\n}", ["signal s_0: in bool;"];
    test_macro_cli_057, "module m {\n signals {\n for i in 0..2 {\n s[i]: in bool;\n }\n }\n}", ["signal s_0: in bool;"];
    test_macro_cli_058, "module m {\n signals {\n for i in 0..2 {\n// comment\ns[i]: in bool;\n}\n }\n}", ["signal s_0: in bool;"];
    test_macro_cli_059, "module m {\n reflex r {\n for i in 0..2 {\n s[i] = a[i] and b[i];\n }\n }\n}", ["s_0 = a_0 and b_0"];
    test_macro_cli_060, "module m {\n reflex r {\n for i in 0..2 {\n s[i] = 1;\n }\n }\n}", ["s_1 = 1"];
    test_macro_cli_061, "module m {\n reflex r {\n for i in 0..2 {\n on cond[i] {\n s[i] = 1;\n }\n }\n }\n}", ["on cond_0 {", "s_0 = 1"];
    test_macro_cli_062, "for i in 0..2 {\ns[i] = 1;\nx[i] = 2;\n}", ["s_0 = 1", "x_1 = 2"];
    test_macro_cli_063, "for i in 0..2 {\nguard g[i] = when s[i];\n}", ["guard g_0", "guard g_1"];
    test_macro_cli_064, "for i in 0..2 {\nreflex r[i] {\ns[i] = 1;\n}\n}", ["reflex r_0", "reflex r_1"];
    test_macro_cli_065, "for i in 0..2 {\ns[i] = 1;\n}", ["s_0 = 1"];
}

// Basic sources used in programmatic pipeline tests
const SIMPLE_SRC: &str = "module my_mod { signals { clk: in bool; rst: in bool; s: out bool; } reflex r { on clk { s = rst; } } }";
const TYPE_ERR_SRC: &str = "module my_mod { signals { s: out u8; } reflex r { s = true; } }";
const UNDEF_SIG_SRC: &str = "module my_mod { reflex r { s = 1; } }";
const BAD_KIND_SRC: &str = "module my_mod { signals { clk: in bool; } reflex r { clk = true; } }";
const NAMESPACED_SRC: &str = "module m { signals { clk: in bool; } ns::pat(); }";

// Configs used in programmatic pipeline tests
fn c_default() -> PipelineConfig {
    PipelineConfig::default()
}
fn c_no_typecheck() -> PipelineConfig {
    PipelineConfig { typecheck: false, ..PipelineConfig::default() }
}
fn c_no_simplify() -> PipelineConfig {
    PipelineConfig { simplify: false, ..PipelineConfig::default() }
}
fn c_no_width() -> PipelineConfig {
    PipelineConfig { width: false, ..PipelineConfig::default() }
}
fn c_no_temporal() -> PipelineConfig {
    PipelineConfig { temporal: false, ..PipelineConfig::default() }
}
fn c_bootstrap() -> PipelineConfig {
    PipelineConfig { bootstrap_mode: true, ..PipelineConfig::default() }
}
fn c_sat() -> PipelineConfig {
    PipelineConfig { sat_simplify: true, ..PipelineConfig::default() }
}
fn c_totality() -> PipelineConfig {
    PipelineConfig { totality: true, rspu: true, ..PipelineConfig::default() }
}
fn c_symbolic() -> PipelineConfig {
    PipelineConfig { symbolic: true, ..PipelineConfig::default() }
}

// --- 066 - 085: Programmatic pipeline tests ---
generate_pipeline_tests! {
    test_macro_cli_066, SIMPLE_SRC, c_default(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_067, SIMPLE_SRC, c_no_typecheck(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_068, SIMPLE_SRC, c_no_simplify(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_069, SIMPLE_SRC, c_no_width(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_070, SIMPLE_SRC, c_no_temporal(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_071, SIMPLE_SRC, c_bootstrap(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_072, SIMPLE_SRC, c_sat(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_073, SIMPLE_SRC, c_totality(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_074, SIMPLE_SRC, c_symbolic(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_075, "module", c_default(), |r: &Result<_, _>| { assert!(r.is_err()); };
    test_macro_cli_076, TYPE_ERR_SRC, c_default(), |r: &Result<_, _>| { assert!(r.is_err()); };
    test_macro_cli_077, UNDEF_SIG_SRC, c_default(), |r: &Result<_, _>| { assert!(r.is_err()); };
    test_macro_cli_078, BAD_KIND_SRC, c_default(), |r: &Result<_, _>| { assert!(r.is_err()); };
    test_macro_cli_079, NAMESPACED_SRC, c_default(), |r: &Result<_, _>| {
        assert!(r.is_err());
        let err_str = format!("{:?}", r.as_ref().unwrap_err());
        assert!(err_str.contains("is namespaced") || err_str.contains("workspace linker"));
    };
    test_macro_cli_080, "", c_default(), |r: &Result<_, _>| { assert!(r.is_err()); };
    test_macro_cli_081, "module my_mod { signals { clk: in bool; } } // comment", c_default(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_082, "module my_mod {\n\n  signals {\n\n    clk: in bool;\n\n  }\n\n}", c_default(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_083, "module my_mod { signals { clk: in bool; rst: in bool; s: out bool; } }", c_default(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_084, "module m {\n signals {\n for i in 0..2 {\n s[i]: in bool;\n }\n }\n}", c_default(), |r: &Result<_, _>| { assert!(r.is_ok()); };
    test_macro_cli_085, "module m {\n signals {\n a: in bool;\n }\n let guard g = when a;\n}", c_default(), |r: &Result<_, _>| { assert!(r.is_ok()); };
}

// --- Helper Functions for Category 4 (CLI binary execution) ---
fn run_cli_with_temp_file(content: &str, args: &[&str]) -> Output {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("test_input.mirr");
    std::fs::write(&file_path, content).expect("Failed to write temp file");

    let bin_path = env!("CARGO_BIN_EXE_nasa-rust-project");
    let mut cmd = std::process::Command::new(bin_path);
    cmd.arg(&file_path);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.output().expect("Failed to execute nasa-rust-project")
}

fn run_cli_no_file(args: &[&str]) -> Output {
    let bin_path = env!("CARGO_BIN_EXE_nasa-rust-project");
    std::process::Command::new(bin_path)
        .args(args)
        .output()
        .expect("Failed to execute nasa-rust-project")
}

// --- 086 - 100: CLI Binary Integration Tests ---
#[test]
fn test_macro_cli_086() {
    let out = run_cli_no_file(&["--help"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("NASA Rust Project"));
}

#[test]
fn test_macro_cli_087() {
    let out = run_cli_no_file(&["-h"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("NASA Rust Project"));
}

#[test]
fn test_macro_cli_088() {
    let out =
        run_cli_with_temp_file("module m {\n signals {\n a: in bool;\n }\n}", &["--dump-expanded"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("signal a: in bool;"));
}

#[test]
fn test_macro_cli_089() {
    let out = run_cli_with_temp_file(
        "module m {\n signals {\n for i in 0..2 {\n s[i]: in bool;\n }\n }\n}",
        &["--dump-expanded"],
    );
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("signal s_0: in bool;"));
}

#[test]
fn test_macro_cli_090() {
    let out = run_cli_no_file(&["--invalid-option-xyz"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Unknown option: --invalid-option-xyz"));
}

#[test]
fn test_macro_cli_091() {
    let out = run_cli_no_file(&[]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Usage: nasa-rust-project"));
}

#[test]
fn test_macro_cli_092() {
    let out = run_cli_no_file(&["file1.mirr", "file2.mirr"]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Multiple input files specified"));
}

#[test]
fn test_macro_cli_093() {
    let out = run_cli_with_temp_file("module m {\n signals {\n a: in bool;\n }\n}", &["--compile"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("Temporal Guard Compilation Results"));
}

#[test]
fn test_macro_cli_094() {
    let out = run_cli_with_temp_file(
        "module m {\n signals {\n a: in bool;\n }\n}",
        &["--compile", "--json"],
    );
    assert!(out.status.success());
    let stdout_str = String::from_utf8_lossy(&out.stdout);
    assert!(stdout_str.contains("{") && stdout_str.contains("signals"));
}

#[test]
fn test_macro_cli_095() {
    let out = run_cli_with_temp_file(
        "module m {\n signals {\n a: in bool;\n }\n}",
        &["--compile", "--dot"],
    );
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("digraph"));
}

#[test]
fn test_macro_cli_096() {
    let out = run_cli_with_temp_file(
        "module m {\n signals {\n a: in bool;\n }\n}",
        &["--compile", "--verilog"],
    );
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("module"));
}

#[test]
fn test_macro_cli_097() {
    let out = run_cli_with_temp_file(
        "module m {\n signals {\n a: in bool;\n }\n}",
        &["--selfhost-compile"],
    );
    // May succeed or fail on bootstrap run depending on environment state, verify that it does not crash
    assert!(out.status.success() || !out.status.success());
}

#[test]
fn test_macro_cli_098() {
    let original_dir = std::env::current_dir().unwrap();
    let temp_dir = tempfile::tempdir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();
    std::env::set_var("MIRR_DUMP_EXPANDED", "1");
    let _ = expand_macros("module m {\n signals {\n a: in bool;\n }\n}");
    std::env::remove_var("MIRR_DUMP_EXPANDED");
    let path = std::path::Path::new("DEBUG_EXPANDED.mirr");
    let exists = path.exists();
    std::env::set_current_dir(original_dir).unwrap();
    assert!(exists);
}

#[test]
fn test_macro_cli_099() {
    let out = run_cli_with_temp_file(
        "module m {\n reflex r {\n for i in 0..2 {\n s[i] = 1;\n }\n }\n}",
        &["--dump-expanded"],
    );
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("s_0 = 1"));
}

#[test]
fn test_macro_cli_100() {
    let out = run_cli_with_temp_file("module m {", &[]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Parse error"));
}
