use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_cdc_emission() {
    let mirr_src = r#"
target profile {
    name: "CDC-Test";
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

    reflex capture {
        on always {
            buffer = sensor_data;
        }
    }
}
"#;

    // Create a temporary directory for the test
    let test_dir = tempfile::tempdir().expect("failed to create temp dir");
    let src_path = test_dir.path().join("cdc_test.mirr");
    fs::write(&src_path, mirr_src).expect("failed to write mirr source");

    // Run the compiler targeting verilog
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--bin")
        .arg("mirr-compile")
        .arg("--")
        .arg(src_path.to_str().unwrap())
        .arg("--emit")
        .arg("verilog")
        .arg("--out-dir")
        .arg(test_dir.path().to_str().unwrap());

    let output = cmd.output().expect("failed to execute mirr-compile");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Compiler failed:\n{}", stderr);
    }

    // Read the generated SystemVerilog file
    let sv_path = test_dir.path().join("cdc_top.sv");
    let sv_content = fs::read_to_string(&sv_path)
        .expect("failed to read generated SV file");

    // Verify CDC logic
    assert!(
        sv_content.contains("always_ff @(posedge clk_fast or negedge rst_n) begin"),
        "Failed to emit correct clock domain 'clk_fast' for the buffer reflex. Generated SV:\n{}",
        sv_content
    );

    // Verify it doesn't emit 'clk' for the buffer
    assert!(
        !sv_content.contains("always_ff @(posedge clk or negedge rst_n) begin"),
        "Compiler emitted default 'clk' domain instead of 'clk_fast'."
    );

    // Verify synchronizer for sensor_data uses clk_fast
    assert!(
        sv_content.contains("2-stage synchronizer for sensor_data (@clk_fast)"),
        "Failed to annotate input synchronizer for sensor_data with @clk_fast. Generated SV:\n{}",
        sv_content
    );
}
