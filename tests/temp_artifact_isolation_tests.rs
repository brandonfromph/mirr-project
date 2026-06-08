#![forbid(unsafe_code)]
//! Temporary artifact isolation tests.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use mirrc::emit;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

const SRC_A: &str = r#"
module temp_iso_a {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 1 cycles;
    }

    reflex r {
        on g {
            b = true;
        }
    }
}
"#;

const SRC_B: &str = r#"
module temp_iso_b {
    signal a: in bool;
    signal b: out bool;

    guard g {
        when a
        for 2 cycles;
    }

    reflex r {
        on g {
            b = a;
        }
    }
}
"#;

fn write_artifact(base: &Path, name: &str, content: &str) -> Result<PathBuf, String> {
    if name.is_empty()
        || name == "."
        || name == ".."
        || name.chars().any(|ch| ch == '/' || ch == '\\')
    {
        return Err(format!("invalid artifact name: {name}"));
    }

    if PathBuf::from(name).file_name() != Some(OsStr::new(name)) {
        return Err(format!("invalid artifact name: {name}"));
    }

    let path = base.join(name);
    fs::write(&path, content).map_err(|e| format!("artifact write should succeed: {e}"))?;
    Ok(path)
}

#[test]
fn temp_dirs_keep_artifacts_isolated() {
    let cfg = PipelineConfig::default();
    let a = run_pipeline(SRC_A, &cfg).expect("pipeline A should succeed");
    let b = run_pipeline(SRC_B, &cfg).expect("pipeline B should succeed");

    let sv_a = emit::verilog::emit_sv(&a);
    let sv_b = emit::verilog::emit_sv(&b);
    assert_ne!(sv_a, sv_b, "fixtures should produce distinct outputs");

    let dir_a = tempfile::tempdir().expect("tempdir A should be created");
    let dir_b = tempfile::tempdir().expect("tempdir B should be created");
    let base_a = dir_a.path().to_path_buf();
    let base_b = dir_b.path().to_path_buf();

    let file_a = write_artifact(&base_a, "out.sv", &sv_a).expect("artifact A should be written");
    let file_b = write_artifact(&base_b, "out.sv", &sv_b).expect("artifact B should be written");

    let loaded_a = fs::read_to_string(file_a).expect("artifact A should be readable");
    let loaded_b = fs::read_to_string(file_b).expect("artifact B should be readable");
    assert_eq!(loaded_a, sv_a);
    assert_eq!(loaded_b, sv_b);
    assert_ne!(loaded_a, loaded_b, "same file name in separate temp dirs must stay isolated");
}

#[test]
fn artifact_writer_rejects_path_traversal_names() {
    let dir = tempfile::tempdir().expect("tempdir should be created");
    let base = dir.path().to_path_buf();

    assert!(write_artifact(&base, "../escape.sv", "x").is_err());
    assert!(write_artifact(&base, "nested/escape.sv", "x").is_err());
}
