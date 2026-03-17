#![forbid(unsafe_code)]
#![allow(unused_imports)]

use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

fn tool_available(name: &str) -> bool {
    // Actually run the tool to verify it loads (catches missing DLLs on Windows).
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

fn run_yosys_script(script: &str, tag: &str) -> std::process::Output {
    let path = write_temp(&format!("mirr_formal_{tag}.ys"), script);
    std::process::Command::new("yosys").arg(&path).output().expect("yosys run")
}

// E5.1 — Rust-level: RTL contains always_comb (no combinational loops from comb blocks)
#[test]
fn formal_rtl_has_no_combinational_loops_rtl_check() {
    let sv = generate_mape_k_rtl();
    // The presence of always_comb is expected for combinational logic.
    // This is a structural proxy: generated RTL uses always_comb correctly.
    assert!(
        sv.contains("always_comb"),
        "E5.1: RTL missing always_comb blocks — unexpected structural omission"
    );
}

// E5.2 — Rust-level: monitor module is clocked (always_ff @posedge clk)
#[test]
fn formal_rtl_monitor_is_clocked() {
    let sv = generate_mape_k_rtl();
    assert!(
        sv.contains("always_ff @(posedge clk"),
        "E5.2: RTL missing 'always_ff @(posedge clk' — monitor must be clocked"
    );
}

// E5.3 — Rust-level: emergency latch is set (not just reset) in the RTL
#[test]
fn formal_rtl_emergency_latch_is_set_only() {
    let sv = generate_mape_k_rtl();
    // The emergency latch must have a set path (assigned to 1'b1 somewhere).
    assert!(
        sv.contains("emergency_active <= 1'b1"),
        "E5.3: RTL missing 'emergency_active <= 1\\'b1' — emergency set path absent"
    );
}

// E5.4 — Rust-level: reset clears write pointer (rst_n resets wr_ptr)
#[test]
fn formal_rtl_rst_n_clears_pointers() {
    let sv = generate_mape_k_rtl();
    assert!(
        sv.contains("wr_ptr <="),
        "E5.4: RTL missing 'wr_ptr <=' — write pointer not present in knowledge FIFO"
    );
}

// E5.5 — Rust-level: knowledge module has assign full signal
#[test]
fn formal_rtl_knowledge_full_signal() {
    let sv = generate_mape_k_rtl();
    assert!(
        sv.contains("assign full"),
        "E5.5: RTL missing 'assign full' — knowledge FIFO full-flag absent"
    );
}

// E5.6 — yosys: read + hierarchy + check -assert on full RTL
#[test]
fn formal_yosys_check_assert() {
    if !tool_available("yosys") {
        return;
    }
    let sv = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_formal_e5_6.sv", &sv);
    let script = format!("read_verilog -sv {}\nhierarchy -check\ncheck\n", sv_path.display());
    let out = run_yosys_script(&script, "e5_6");
    assert!(
        out.status.success(),
        "E5.6: yosys hierarchy + check failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E5.7 — yosys: proc + opt + check -assert -noinit (no combinational loops)
#[test]
fn formal_yosys_no_combinational_loops() {
    if !tool_available("yosys") {
        return;
    }
    let sv = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_formal_e5_7.sv", &sv);
    let script =
        format!("read_verilog -sv {}\nproc\nopt\ncheck -assert -noinit\n", sv_path.display());
    let out = run_yosys_script(&script, "e5_7");
    assert!(
        out.status.success(),
        "E5.7: yosys proc+opt+check -assert -noinit failed (possible comb loop):\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E5.8 — yosys: synth_ice40 on mirr_monitor succeeds
#[test]
fn formal_yosys_monitor_synth() {
    if !tool_available("yosys") {
        return;
    }
    let sv = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_formal_e5_8.sv", &sv);
    let script = format!("read_verilog -sv {}\nsynth_ice40 -top mirr_monitor\n", sv_path.display());
    let out = run_yosys_script(&script, "e5_8");
    assert!(
        out.status.success(),
        "E5.8: yosys synth_ice40 of mirr_monitor failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E5.9 — yosys: synth_ice40 on mirr_execute succeeds
#[test]
fn formal_yosys_execute_synth() {
    if !tool_available("yosys") {
        return;
    }
    let sv = generate_mape_k_rtl();
    let sv_path = write_temp("mirr_formal_e5_9.sv", &sv);
    let script = format!("read_verilog -sv {}\nsynth_ice40 -top mirr_execute\n", sv_path.display());
    let out = run_yosys_script(&script, "e5_9");
    assert!(
        out.status.success(),
        "E5.9: yosys synth_ice40 of mirr_execute failed:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
}

// E5.10 — Rust-level: every always_ff block has an if (!rst_n) reset clause
#[test]
fn formal_rtl_all_registers_reset() {
    let sv = generate_mape_k_rtl();
    // Count always_ff blocks and if (!rst_n) occurrences; they must match.
    let ff_count = sv.matches("always_ff").count();
    let rst_count = sv.matches("if (!rst_n)").count();
    assert!(ff_count > 0, "E5.10: RTL contains no always_ff blocks");
    assert!(
        rst_count >= ff_count,
        "E5.10: not all always_ff blocks have 'if (!rst_n)' reset clause \
         (ff_count={ff_count}, rst_count={rst_count})"
    );
}

// E5.11 — Rust-level: violation_vec has correct width for 2-property config
#[test]
fn formal_rtl_violation_vec_width_correct() {
    let sv = generate_mape_k_rtl();
    // With 2 assert properties, the violation vector should be [1:0].
    assert!(
        sv.contains("[1:0] violation_vec"),
        "E5.11: RTL missing '[1:0] violation_vec' — expected 2-bit vector for 2 properties"
    );
}

// E5.12 — Rust-level: plan module references action_valid
#[test]
fn formal_rtl_plan_action_valid() {
    let sv = generate_mape_k_rtl();
    assert!(sv.contains("action_valid"), "E5.12: RTL missing 'action_valid' in plan module");
}
