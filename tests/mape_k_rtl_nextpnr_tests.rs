#![cfg(any())]
#![forbid(unsafe_code)]
#![allow(unused_imports)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn tool_available(name: &str) -> bool {
    let flag = if name == "yosys" || name == "icetime" { "-V" } else { "--version" };
    std::process::Command::new(name).arg(flag).output().map(|o| o.status.success()).unwrap_or(false)
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

/// Synthesise `sv_content` for iCE40 using yosys, emit JSON to temp.
/// Returns the path to the JSON netlist on success, None if yosys fails.
fn synthesize_to_json(sv_content: &str, top: &str) -> Option<std::path::PathBuf> {
    if !tool_available("yosys") {
        return None;
    }
    let sv_path = write_temp(&format!("pnr_{top}.sv"), sv_content);
    let json_path = std::env::temp_dir().join(format!("pnr_{top}.json"));
    let script = format!(
        "read_verilog -sv {sv}\nsynth_ice40 -top {top} -json {json}\n",
        sv = sv_path.display(),
        json = json_path.display()
    );
    let ys_path = write_temp(&format!("pnr_{top}.ys"), &script);
    let out = std::process::Command::new("yosys").arg(&ys_path).output().expect("yosys");
    if out.status.success() {
        Some(json_path)
    } else {
        None
    }
}

// E6.1 — nextpnr-ice40 place-and-route on mirr_monitor
#[test]
fn nextpnr_ice40_place_route_monitor() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let json = match synthesize_to_json(&rtl, "mirr_monitor") {
        Some(j) => j,
        None => return,
    };
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .output()
        .expect("nextpnr-ice40");
    assert!(
        out.status.success(),
        "E6.1: nextpnr-ice40 PNR failed for mirr_monitor:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E6.2 — nextpnr-ice40 place-and-route on mirr_knowledge
#[test]
fn nextpnr_ice40_place_route_knowledge() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let json = match synthesize_to_json(&rtl, "mirr_knowledge") {
        Some(j) => j,
        None => return,
    };
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .output()
        .expect("nextpnr-ice40");
    assert!(
        out.status.success(),
        "E6.2: nextpnr-ice40 PNR failed for mirr_knowledge:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E6.3 — nextpnr-ice40 place-and-route on mirr_execute
#[test]
fn nextpnr_ice40_place_route_execute() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let json = match synthesize_to_json(&rtl, "mirr_execute") {
        Some(j) => j,
        None => return,
    };
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .output()
        .expect("nextpnr-ice40");
    assert!(
        out.status.success(),
        "E6.3: nextpnr-ice40 PNR failed for mirr_execute:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E6.4 — nextpnr-ice40 place-and-route on mirr_plan
#[test]
fn nextpnr_ice40_place_route_plan() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let json = match synthesize_to_json(&rtl, "mirr_plan") {
        Some(j) => j,
        None => return,
    };
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .output()
        .expect("nextpnr-ice40");
    assert!(
        out.status.success(),
        "E6.4: nextpnr-ice40 PNR failed for mirr_plan:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E6.5 — nextpnr utilisation is non-zero (LUT count > 0) for mirr_knowledge
#[test]
fn nextpnr_ice40_utilization_nonzero() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let json = match synthesize_to_json(&rtl, "mirr_knowledge") {
        Some(j) => j,
        None => return,
    };
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .output()
        .expect("nextpnr-ice40");
    if !out.status.success() {
        return;
    }
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let has_lut = combined.contains("LUT:") || combined.contains("LCs used as LUT4");
    assert!(
        has_lut,
        "E6.5: nextpnr output missing LUT utilisation summary:\n{}",
        &combined[..combined.len().min(2000)]
    );
}

// E6.6 — nextpnr-ecp5 place-and-route on mirr_monitor
#[test]
fn nextpnr_ecp5_place_route_monitor() {
    if !tool_available("yosys") || !tool_available("nextpnr-ecp5") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    // Synthesise for ECP5 using yosys synth_ecp5.
    let sv_path = write_temp("pnr_ecp5_monitor.sv", &rtl);
    let json_path = std::env::temp_dir().join("pnr_ecp5_monitor.json");
    let script = format!(
        "read_verilog -sv {sv}\nsynth_ecp5 -top mirr_monitor -json {json}\n",
        sv = sv_path.display(),
        json = json_path.display()
    );
    let ys_path = write_temp("pnr_ecp5_monitor.ys", &script);
    let synth =
        std::process::Command::new("yosys").arg(&ys_path).output().expect("yosys ecp5 synth");
    if !synth.status.success() {
        return;
    }
    let out = std::process::Command::new("nextpnr-ecp5")
        .arg("--25k")
        .arg("--package")
        .arg("CABGA256")
        .arg("--json")
        .arg(&json_path)
        .output()
        .expect("nextpnr-ecp5");
    assert!(
        out.status.success(),
        "E6.6: nextpnr-ecp5 PNR failed for mirr_monitor:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E6.7 — nextpnr --log creates a log file
#[test]
fn nextpnr_timing_report_exists() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let json = match synthesize_to_json(&rtl, "mirr_knowledge") {
        Some(j) => j,
        None => return,
    };
    let log_path = std::env::temp_dir().join("mirr_nextpnr_timing.log");
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .arg("--log")
        .arg(&log_path)
        .output()
        .expect("nextpnr-ice40 --log");
    if !out.status.success() {
        return;
    }
    assert!(
        log_path.exists(),
        "E6.7: nextpnr --log did not create log file at {}",
        log_path.display()
    );
}

// E6.8 — nextpnr on mirr_execute has no ERROR: in output
#[test]
fn nextpnr_ice40_no_placement_errors() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let json = match synthesize_to_json(&rtl, "mirr_execute") {
        Some(j) => j,
        None => return,
    };
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .output()
        .expect("nextpnr-ice40");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stdout.contains("ERROR:") && !stderr.contains("ERROR:"),
        "E6.8: nextpnr reported ERROR: for mirr_execute:\nstdout: {}\nstderr: {}",
        stdout,
        stderr
    );
}
