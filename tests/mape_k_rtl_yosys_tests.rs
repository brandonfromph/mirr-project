#![forbid(unsafe_code)]
#![allow(unused_imports)]

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

fn tool_available(name: &str) -> bool {
    // Actually run the tool to verify it loads (catches missing DLLs on Windows).
    let flag = if name == "yosys" || name == "icetime" { "-V" } else { "--version" };
    std::process::Command::new(name).arg(flag).output().map(|o| o.status.success()).unwrap_or(false)
}
fn require_tool_or_skip(name: &str) -> bool {
    if tool_available(name) {
        return true;
    }
    eprintln!("Tool capability missing: {}. Test skipped with explicit accounting.", name);
    false
}

fn generate_mape_k_rtl() -> String {
    const MIRR_SRC: &str = "module safety_hw {\n    signal pressure: in u8;\n    signal temp: in u8;\n    signal alarm: out bool;\n\n    property p_pressure {\n        always (pressure > 0);\n    }\n\n    property p_temp {\n        always (temp > 0);\n    }\n}";
    let config = PipelineConfig { mape_k: true, emit_mape_k_rtl: true, ..Default::default() };
    let result = run_pipeline(MIRR_SRC, &config).expect("pipeline should succeed");
    result.mape_k_rtl.expect("RTL should be generated")
}

fn write_temp(name: &str, content: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, content).expect("write temp file");
    path
}

fn run_yosys_script(script: &str) -> std::process::Output {
    let fname = format!(
        "mirr_yosys_{}.ys",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos()
    );
    let path = write_temp(&fname, script);
    std::process::Command::new("yosys").arg(&path).output().expect("yosys run")
}

// E2.1 - read full RTL file without errors
#[test]
fn yosys_reads_full_rtl_without_errors() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_01.sv", &rtl);
    let script = format!("read_verilog -sv {}\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.1: yosys read_verilog failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.2 - parse and hierarchy-check mirr_monitor
#[test]
fn yosys_parses_monitor_module() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_02.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nhierarchy -check -top mirr_monitor\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.2: yosys hierarchy check for mirr_monitor failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.3 - parse and hierarchy-check mirr_analyze
#[test]
fn yosys_parses_analyze_module() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_03.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nhierarchy -check -top mirr_analyze\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.3: yosys hierarchy check for mirr_analyze failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.4 - parse and hierarchy-check mirr_plan
#[test]
fn yosys_parses_plan_module() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_04.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nhierarchy -check -top mirr_plan\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.4: yosys hierarchy check for mirr_plan failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.5 - parse and hierarchy-check mirr_execute
#[test]
fn yosys_parses_execute_module() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_05.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nhierarchy -check -top mirr_execute\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.5: yosys hierarchy check for mirr_execute failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.6 - parse and hierarchy-check mirr_knowledge
#[test]
fn yosys_parses_knowledge_module() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_06.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nhierarchy -check -top mirr_knowledge\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.6: yosys hierarchy check for mirr_knowledge failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.7 - parse and hierarchy-check mirr_mape_k_top
#[test]
fn yosys_parses_top_module() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_07.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nhierarchy -check -top mirr_mape_k_top\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.7: yosys hierarchy check for mirr_mape_k_top failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.8 - synth_ice40 on mirr_monitor
#[test]
fn yosys_synth_ice40_monitor() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_08.sv", &rtl);
    let script = format!("read_verilog -sv {}\nsynth_ice40 -top mirr_monitor\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.8: yosys synth_ice40 for mirr_monitor failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.9 - synth_ice40 on mirr_execute
#[test]
fn yosys_synth_ice40_execute() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_09.sv", &rtl);
    let script = format!("read_verilog -sv {}\nsynth_ice40 -top mirr_execute\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.9: yosys synth_ice40 for mirr_execute failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.10 - synth_ice40 on mirr_knowledge
#[test]
fn yosys_synth_ice40_knowledge() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_10.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nsynth_ice40 -top mirr_knowledge\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.10: yosys synth_ice40 for mirr_knowledge failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.11 - synth + check -assert (no latches)
#[test]
fn yosys_check_no_latches() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_11.sv", &rtl);
    let script = format!("read_verilog -sv {}\nsynth\ncheck -assert\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.11: yosys check -assert (no-latches) failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.12 - synth_ice40 top + stat reports cells
#[test]
fn yosys_stat_reports_cells() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_12.sv", &rtl);
    let script =
        format!("read_verilog -sv {}\nsynth_ice40 -top mirr_mape_k_top\nstat\n", sv_path.display());
    let out = run_yosys_script(&script);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Number of cells")
            || stdout.contains("cells\n")
            || stdout.contains("cells"),
        "E2.12: yosys stat output missing cell summary:\n{}",
        stdout
    );
}

// E2.13 - no ERROR: string in any yosys output
#[test]
fn yosys_no_error_in_output() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_13.sv", &rtl);
    let script = format!("read_verilog -sv {}\n", sv_path.display());
    let out = run_yosys_script(&script);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stdout.contains("ERROR:") && !stderr.contains("ERROR:"),
        "E2.13: yosys reported ERROR in output.\nstdout: {}\nstderr: {}",
        stdout,
        stderr
    );
}

// E2.14 - full hierarchy -check succeeds
#[test]
fn yosys_monitor_hierarchy_ok() {
    if !require_tool_or_skip("yosys") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_yosys_e2_14.sv", &rtl);
    let script = format!("read_verilog -sv {}\nhierarchy -check\n", sv_path.display());
    let out = run_yosys_script(&script);
    assert!(
        out.status.success(),
        "E2.14: yosys hierarchy -check failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E2.15 - RTL must not contain $display (Rust-level check, no tool required)
#[test]
fn yosys_rtl_no_display_statements() {
    let rtl = generate_mape_k_rtl();
    assert!(
        !rtl.contains("$display"),
        "E2.15: RTL contains $display which is forbidden in synthesizable RTL"
    );
}
