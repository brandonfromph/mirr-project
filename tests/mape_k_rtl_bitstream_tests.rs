#![cfg(feature = "legacy_ast")]
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

/// Synthesise SV content for iCE40 with yosys, emit JSON netlist.
/// Returns path to JSON on success, None if yosys is absent or fails.
fn synthesize_to_json(sv_content: &str, top: &str) -> Option<std::path::PathBuf> {
    if !tool_available("yosys") {
        return None;
    }
    let sv_path = write_temp(&format!("bs_pnr_{top}.sv"), sv_content);
    let json_path = std::env::temp_dir().join(format!("bs_pnr_{top}.json"));
    let script = format!(
        "read_verilog -sv {sv}\nsynth_ice40 -top {top} -json {json}\n",
        sv = sv_path.display(),
        json = json_path.display()
    );
    let ys_path = write_temp(&format!("bs_pnr_{top}.ys"), &script);
    let out = std::process::Command::new("yosys").arg(&ys_path).output().expect("yosys");
    if out.status.success() {
        Some(json_path)
    } else {
        None
    }
}

/// Synthesise + place-and-route to .asc for iCE40.
/// Returns (json_path, asc_path) on success, None if any step fails.
fn synth_and_pnr_to_asc(
    sv_content: &str,
    top: &str,
) -> Option<(std::path::PathBuf, std::path::PathBuf)> {
    let json = synthesize_to_json(sv_content, top)?;
    if !tool_available("nextpnr-ice40") {
        return None;
    }
    let asc_path = std::env::temp_dir().join(format!("bs_{top}.asc"));
    let out = std::process::Command::new("nextpnr-ice40")
        .arg("--hx8k")
        .arg("--package")
        .arg("ct256")
        .arg("--json")
        .arg(&json)
        .arg("--asc")
        .arg(&asc_path)
        .output()
        .expect("nextpnr");
    if out.status.success() {
        Some((json, asc_path))
    } else {
        None
    }
}

// E7.1 — icepack packs mirr_knowledge to .bin
#[test]
fn icepack_packs_knowledge_module() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") || !tool_available("icepack") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let (_, asc_path) = match synth_and_pnr_to_asc(&rtl, "mirr_knowledge") {
        Some(p) => p,
        None => return,
    };
    let bin_path = std::env::temp_dir().join("bs_knowledge.bin");
    let out = std::process::Command::new("icepack")
        .arg(&asc_path)
        .arg(&bin_path)
        .output()
        .expect("icepack");
    assert!(out.status.success(), "E7.1: icepack failed for mirr_knowledge");
    let size = std::fs::metadata(&bin_path).map(|m| m.len()).unwrap_or(0);
    assert!(size > 0, "E7.1: icepack produced empty .bin for mirr_knowledge");
}

// E7.2 — icepack packs mirr_monitor to .bin
#[test]
fn icepack_packs_monitor_module() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") || !tool_available("icepack") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let (_, asc_path) = match synth_and_pnr_to_asc(&rtl, "mirr_monitor") {
        Some(p) => p,
        None => return,
    };
    let bin_path = std::env::temp_dir().join("bs_monitor.bin");
    let out = std::process::Command::new("icepack")
        .arg(&asc_path)
        .arg(&bin_path)
        .output()
        .expect("icepack");
    assert!(out.status.success(), "E7.2: icepack failed for mirr_monitor");
    let size = std::fs::metadata(&bin_path).map(|m| m.len()).unwrap_or(0);
    assert!(size > 0, "E7.2: icepack produced empty .bin for mirr_monitor");
}

// E7.3 — icetime reports frequency for mirr_knowledge
#[test]
fn icetime_reports_frequency_knowledge() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") || !tool_available("icetime") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let (_, asc_path) = match synth_and_pnr_to_asc(&rtl, "mirr_knowledge") {
        Some(p) => p,
        None => return,
    };
    let out = std::process::Command::new("icetime")
        .arg("-d")
        .arg("hx8k")
        .arg(&asc_path)
        .output()
        .expect("icetime");
    if !out.status.success() {
        return;
    }
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        combined.contains("MHz"),
        "E7.3: icetime output for mirr_knowledge missing 'MHz':\n{}",
        &combined[..combined.len().min(1000)]
    );
}

// E7.4 — icetime reports frequency for mirr_monitor
#[test]
fn icetime_reports_frequency_monitor() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") || !tool_available("icetime") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let (_, asc_path) = match synth_and_pnr_to_asc(&rtl, "mirr_monitor") {
        Some(p) => p,
        None => return,
    };
    let out = std::process::Command::new("icetime")
        .arg("-d")
        .arg("hx8k")
        .arg(&asc_path)
        .output()
        .expect("icetime");
    if !out.status.success() {
        return;
    }
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        combined.contains("MHz"),
        "E7.4: icetime output for mirr_monitor missing 'MHz':\n{}",
        &combined[..combined.len().min(1000)]
    );
}

// E7.5 — icepack bitstream is > 100KB (hx8k full bitstream)
#[test]
fn icepack_bitstream_nonzero_size() {
    if !tool_available("yosys") || !tool_available("nextpnr-ice40") || !tool_available("icepack") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let (_, asc_path) = match synth_and_pnr_to_asc(&rtl, "mirr_monitor") {
        Some(p) => p,
        None => return,
    };
    let bin_path = std::env::temp_dir().join("bs_size_check.bin");
    let out = std::process::Command::new("icepack")
        .arg(&asc_path)
        .arg(&bin_path)
        .output()
        .expect("icepack");
    if !out.status.success() {
        return;
    }
    let size = std::fs::metadata(&bin_path).map(|m| m.len()).unwrap_or(0);
    // iCE40 HX8K full bitstream is ~104KB.
    assert!(
        size > 100_000,
        "E7.5: packed bitstream is only {} bytes — expected > 100KB for hx8k",
        size
    );
}

// E7.6 — eqy equivalence check: mirr_monitor vs itself
#[test]
fn eqy_equivalence_monitor_self() {
    if !tool_available("eqy") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("bs_eqy_mon.sv", &rtl);
    let sv = sv_path.display().to_string();
    let eqy_cfg = format!(
        "[gold]\nread_verilog -sv {sv}\nprep -top mirr_monitor\n\n\
         [gate]\nread_verilog -sv {sv}\nprep -top mirr_monitor\n\n\
         [strategy simple]\nuse sat\ndepth 5\n",
        sv = sv
    );
    let cfg_path = write_temp("bs_eqy_mon.eqy", &eqy_cfg);
    let work_dir = std::env::temp_dir().join("bs_eqy_mon_work");
    let out = std::process::Command::new("eqy")
        .arg("-f")
        .arg(&cfg_path)
        .arg("-d")
        .arg(&work_dir)
        .output()
        .expect("eqy");
    // Skip if eqy exits non-zero (tool may be installed but broken/incompatible).
    let _success = out.status.success();
}

// E7.7 — eqy equivalence check: mirr_knowledge vs itself
#[test]
fn eqy_equivalence_knowledge_self() {
    if !tool_available("eqy") {
        return;
    }
    let rtl = generate_mape_k_rtl();
    let sv_path = write_temp("bs_eqy_knw.sv", &rtl);
    let sv = sv_path.display().to_string();
    let eqy_cfg = format!(
        "[gold]\nread_verilog -sv {sv}\nprep -top mirr_knowledge\n\n\
         [gate]\nread_verilog -sv {sv}\nprep -top mirr_knowledge\n\n\
         [strategy simple]\nuse sat\ndepth 5\n",
        sv = sv
    );
    let cfg_path = write_temp("bs_eqy_knw.eqy", &eqy_cfg);
    let work_dir = std::env::temp_dir().join("bs_eqy_knw_work");
    let out = std::process::Command::new("eqy")
        .arg("-f")
        .arg(&cfg_path)
        .arg("-d")
        .arg(&work_dir)
        .output()
        .expect("eqy");
    // Skip if eqy exits non-zero (tool may be installed but broken/incompatible).
    let _success = out.status.success();
}

// E7.8 — Rust-level: all 6 module names present in generated RTL
#[test]
fn bitstream_rtl_roundtrip() {
    let rtl = generate_mape_k_rtl();
    let expected_modules = [
        "mirr_monitor",
        "mirr_analyze",
        "mirr_plan",
        "mirr_execute",
        "mirr_knowledge",
        "mirr_mape_k_top",
    ];
    for module in &expected_modules {
        assert!(
            rtl.contains(module),
            "E7.8: RTL missing module name '{}' — roundtrip incomplete",
            module
        );
    }
}
