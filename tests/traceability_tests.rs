#![allow(clippy::field_reassign_with_default)]
#![cfg(any())]
use mirrc::pipeline::{run_pipeline_with_file, PipelineConfig};

#[test]
fn test_sv_emits_source_comments() {
    let source = r#"
target profile { name: "test"; word_size: 32; reg_width: 8; op_width: 6; }
module my_mod {
    signal clk: in bool;
    signal internal_sig: internal bool;

    guard g1 { when true for 1 cycles; }

    reflex r1 {
        on g1 {
            internal_sig = true;
        }
    }

    property p1 {
        always (internal_sig == true);
    }
}
"#;

    let config = PipelineConfig::default();
    let result = run_pipeline_with_file(source, "my_mod.mirr", &config).unwrap();

    // Test the FileTable is populated
    assert_eq!(result.file_table.len(), 1);

    let sv = mirrc::emit::verilog::emit_sv(&result);

    // Verify source comments exist
    assert!(sv.contains("// source: my_mod.mirr:8")); // module decl
    assert!(sv.contains("// source: my_mod.mirr:10")); // internal_sig
    assert!(sv.contains("// source: my_mod.mirr:12")); // guard g1
    assert!(sv.contains("// source: my_mod.mirr:16")); // reflex r1
    assert!(sv.contains("// source: my_mod.mirr:22")); // property p1
}
