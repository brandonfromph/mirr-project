use std::fs;
use std::process::Command;

#[test]
fn test_ecs_cdc_propagation() {
    let mirr_src = r#"
target profile {
    name: "ECS-CDC-Propagation-Test";
    word_size: 32;
    reg_width: 8;
    op_width: 8;
}

module cdc_top {
    signal clk: in bool;
    signal clk_fast: in bool;
    signal rst_n: in bool;

    signal sensor_data: in u16 @clk_fast;
    signal buffer: internal u16 @clk_fast;
    signal default_buffer: internal u16;

    reflex capture_fast {
        on always {
            buffer = sensor_data;
        }
    }

    reflex capture_default {
        on always {
            default_buffer = sensor_data;
        }
    }
}
"#;

    let test_dir = tempfile::tempdir().expect("failed to create temp dir");
    let src_path = test_dir.path().join("cdc_test.mirr");
    fs::write(&src_path, mirr_src).expect("failed to write mirr source");

    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--bin")
        .arg("mirr")
        .arg("--")
        .arg("compile")
        .arg(src_path.to_str().unwrap())
        .arg("--emit")
        .arg("verilog")
        .arg("--output")
        .arg(test_dir.path().join("cdc_top.sv").to_str().unwrap());

    let output = cmd.output().expect("failed to execute mirr compile");

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Compiler failed:\n{}", stderr);
    }

    let sv_path = test_dir.path().join("cdc_top.sv");
    let sv_content = fs::read_to_string(&sv_path).expect("failed to read generated SV file");

    assert!(
        sv_content.contains("always_ff @(posedge clk_fast or negedge rst_n) begin"),
        "Failed to emit correct clock domain 'clk_fast' for the fast buffer reflex. Generated SV:\n{}",
        sv_content
    );

    assert!(
        sv_content.contains("always_ff @(posedge clk or negedge rst_n) begin"),
        "Failed to emit default clock domain 'clk' for the default buffer reflex. Generated SV:\n{}",
        sv_content
    );
}
