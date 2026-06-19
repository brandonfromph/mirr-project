#![forbid(unsafe_code)]
//! Testbench emission tests.
//!
//! Verifies the auto-generated testbench contains all required elements:
//! DUT instantiation, clock generation, reset sequence, and stimulus.

use mirrc::emit::testbench::emit_testbench;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

const TB_MODULE: &str = r#"
module tb_target {
    signal sensor: in u16;
    signal enable: in bool;
    signal alarm: out bool;

    guard g {
        when sensor > 100
        for 5 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }

    property bounded: always (sensor < 65535);
}
"#;

fn tb_output() -> String {
    let config = PipelineConfig::default();
    let result = run_pipeline(TB_MODULE, &config).expect("pipeline should succeed");
    emit_testbench(&result)
}

#[test]
fn testbench_has_timescale() {
    let tb = tb_output();
    assert!(tb.contains("`timescale"), "testbench should have timescale directive");
}

#[test]
fn testbench_has_module_declaration() {
    let tb = tb_output();
    assert!(tb.contains("module tb_target_tb"), "testbench module name should be <name>_tb");
}

#[test]
fn testbench_has_clock_generation() {
    let tb = tb_output();
    assert!(tb.contains("initial clk = 1'b0"), "testbench should init clock");
    assert!(tb.contains("always #5 clk = ~clk"), "testbench should toggle clock");
}

#[test]
fn testbench_has_dut_instantiation() {
    let tb = tb_output();
    assert!(tb.contains("tb_target dut"), "testbench should instantiate DUT");
    assert!(tb.contains(".clk(clk)"), "testbench should connect clk");
    assert!(tb.contains(".rst_n(rst_n)"), "testbench should connect rst_n");
}

#[test]
fn testbench_has_reset_sequence() {
    let tb = tb_output();
    assert!(tb.contains("rst_n = 1'b0"), "testbench should assert reset");
    assert!(tb.contains("rst_n = 1'b1"), "testbench should deassert reset");
}

#[test]
fn testbench_has_input_stimulus() {
    let tb = tb_output();
    assert!(tb.contains("tb_sensor"), "testbench should drive sensor input");
    assert!(tb.contains("tb_enable"), "testbench should drive enable input");
}

#[test]
fn testbench_has_finish() {
    let tb = tb_output();
    assert!(tb.contains("$finish"), "testbench should call $finish");
}

#[test]
fn testbench_has_endmodule() {
    let tb = tb_output();
    assert!(tb.contains("endmodule"), "testbench should end with endmodule");
}
