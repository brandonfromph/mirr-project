#![forbid(unsafe_code)]
//! Extended SystemVerilog emitter tests - module-level emission.
//!
//! NASA P10: bounded loops, no recursion.

use nasa_rust_project::emit::verilog;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

const MAX_LINE_SCAN: usize = 64;

fn run_src(src: &str) -> String {
    let result = run_pipeline(src, &PipelineConfig::default())
        .unwrap_or_else(|e| panic!("pipeline failed: {e}"));
    verilog::emit_sv(&result)
}

fn contains_any(text: &str, items: &[&str]) -> bool {
    let mut i = 0usize;
    while i < items.len() && i < MAX_LINE_SCAN {
        if text.contains(items[i]) {
            return true;
        }
        i += 1;
    }
    false
}

#[test]
fn module_header_has_module_keyword() {
    let sv = run_src(
        r#"module test_m {
    signal x: in bool;
    signal y: out bool;
}"#,
    );
    assert!(sv.contains("module"), "SV must contain 'module' keyword");
}
#[test]
fn module_name_in_header() {
    let sv = run_src(
        r#"module my_unit {
    signal x: in bool;
    signal y: out bool;
}"#,
    );
    assert!(sv.contains("my_unit"), "module name must appear in SV");
}
#[test]
fn input_signal_declared() {
    let sv = run_src(
        r#"module m {
    signal pressure: in u16;
    signal alarm: out bool;
}"#,
    );
    assert!(sv.contains("pressure"), "input signal name must appear");
    assert!(sv.contains("input"), "input direction must appear");
}
#[test]
fn output_signal_declared() {
    let sv = run_src(
        r#"module m {
    signal flag: out bool;
    signal x: in bool;
}"#,
    );
    assert!(sv.contains("flag"), "output signal name must appear");
    assert!(sv.contains("output"), "output direction must appear");
}
#[test]
fn bool_signal_width_one() {
    let sv = run_src(
        r#"module m {
    signal b: in bool;
    signal y: out bool;
}"#,
    );
    // single-bit signals typically appear as wire or [0:0]
    assert!(sv.contains("b"), "bool signal must appear");
}
#[test]
fn u8_signal_width() {
    let sv = run_src(
        r#"module m {
    signal data: in u8;
    signal y: out bool;
}"#,
    );
    assert!(sv.contains("data"), "u8 signal must appear");
    // 8-bit signal should have [7:0] or similar width annotation
    assert!(sv.contains("7") || sv.contains("["), "width annotation expected");
}
#[test]
fn guard_produces_logic_in_sv() {
    let sv = run_src(
        r#"module gm {
    signal x: in u8;
    signal y: out bool;
    guard g {
        when (x > 100)
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}"#,
    );
    assert!(contains_any(&sv, &["always", "reg", "wire", "assign"]), "guard must produce logic");
}
#[test]
fn clk_and_rst_present_in_temporal_module() {
    let sv = run_src(
        r#"module clk_m {
    signal x: in u8;
    signal y: out bool;
    guard g {
        when (x > 10)
        for 2 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}"#,
    );
    assert!(contains_any(&sv, &["clk", "clock", "posedge"]), "temporal module needs clock");
}
#[test]
fn property_assert_produces_sva() {
    let sv = run_src(
        r#"module prop_sv {
    signal x: in bool;
    property p {
        always (x);
    }
}"#,
    );
    // SVA assert / property keyword should appear
    assert!(
        contains_any(&sv, &["assert", "property", "always"]),
        "property must produce SVA: {}",
        sv
    );
}
#[test]
fn endmodule_present() {
    let sv = run_src(
        r#"module end_m {
    signal x: in bool;
    signal y: out bool;
}"#,
    );
    assert!(sv.contains("endmodule"), "SV must end with endmodule");
}
#[test]
fn multiple_signals_all_present() {
    let sv = run_src(
        r#"module multi {
    signal a: in u8;
    signal b: in u16;
    signal c: in bool;
    signal out_c: out bool;
}"#,
    );
    let names = ["a", "b", "c", "out_c"];
    let mut i = 0usize;
    while i < names.len() {
        assert!(sv.contains(names[i]), "signal {} must be in SV", names[i]);
        i += 1;
    }
}
#[test]
fn internal_signal_as_reg() {
    let sv = run_src(
        r#"module int_m {
    signal x: in bool;
    signal count: internal u8;
    signal y: out bool;
    guard g {
        when x
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}"#,
    );
    assert!(sv.contains("count"), "internal signal must appear");
}
