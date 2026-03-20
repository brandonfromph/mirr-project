//! MEGA-13: SELF-HOST bootstrap runner MIRR stage tests.
//!
//! Verifies that the MIRR compiler stages (parser, semantic, temporal, emitter)
//! can be loaded and parsed through the bootstrap runner pipeline.

#![forbid(unsafe_code)]

use nasa_rust_project::bootstrap_runner::{BootstrapOpts, BootstrapRunner};
use std::io::Write;
use tempfile::NamedTempFile;

fn write_temp_mirr(src: &str) -> NamedTempFile {
    let mut f = NamedTempFile::with_suffix(".mirr").expect("tempfile");
    f.write_all(src.as_bytes()).expect("write");
    f
}

const NEONATAL_SRC: &str = r#"
module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure:   in u16;
    signal clamp_valve:       out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for  1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}
"#;

const SIMPLE_MODULE: &str = r#"
module simple {
    signal a: in bool;
    signal b: out bool;

    guard g1 {
        when a
        for 1 cycles;
    }

    reflex r1 {
        on g1 {
            b = true;
        }
    }
}
"#;

const MULTI_SIGNAL: &str = r#"
module multi_signal {
    signal clk_enable: in bool;
    signal sensor_0:   in u8;
    signal sensor_1:   in u8;
    signal alarm:      out bool;
    signal status:     out u8;

    guard high_sensor_0 {
        when sensor_0 > 200
        for 5 cycles;
    }

    guard high_sensor_1 {
        when sensor_1 > 200
        for 5 cycles;
    }

    reflex trigger_alarm {
        on high_sensor_0 {
            alarm = true;
            status = 1;
        }
    }

    reflex trigger_alarm_1 {
        on high_sensor_1 {
            alarm = true;
            status = 2;
        }
    }
}
"#;

const COMPLEX_GUARD: &str = r#"
module complex_guard {
    signal x: in u16;
    signal y: in u16;
    signal z: out bool;

    guard range_check {
        when x > 10 && x < 100
        for 3 cycles;
    }

    reflex set_z {
        on range_check {
            z = true;
        }
    }
}
"#;

const COUNTER_GUARD: &str = r#"
module counter_guard {
    signal trigger: in bool;
    signal done:    out bool;

    guard long_delay {
        when trigger
        for 50 cycles;
    }

    reflex mark_done {
        on long_delay {
            done = true;
        }
    }
}
"#;

#[test]
fn test_mirr_stages_disabled_by_default() {
    let f = write_temp_mirr(SIMPLE_MODULE);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: false, ..Default::default() });
    let result = runner.run(f.path());
    assert!(result.ok, "pipeline must pass without MIRR stages");
    // No MirrParser/Semantic/Temporal/Emitter stages should appear
    assert!(
        result.stages.iter().all(|s| !s.name.starts_with("Mirr")),
        "no MIRR stages when disabled"
    );
}

#[test]
fn test_mirr_stages_enabled_parses_modules() {
    let f = write_temp_mirr(SIMPLE_MODULE);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());
    assert!(result.ok, "pipeline must pass with MIRR stages enabled");

    let mirr_stages: Vec<&_> =
        result.stages.iter().filter(|s| s.name.starts_with("Mirr")).collect();
    assert_eq!(mirr_stages.len(), 4, "must have 4 MIRR stages");

    for stage in &mirr_stages {
        assert!(stage.ok, "MIRR stage '{}' must pass: {}", stage.name, stage.message);
    }
}

#[test]
fn test_mirr_parser_stage_present() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());

    let parser_stage = result.stages.iter().find(|s| s.name == "MirrParser");
    assert!(parser_stage.is_some(), "MirrParser stage must be present");
    assert!(parser_stage.unwrap().ok, "MirrParser stage must pass");
}

#[test]
fn test_mirr_semantic_stage_present() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());

    let semantic_stage = result.stages.iter().find(|s| s.name == "MirrSemantic");
    assert!(semantic_stage.is_some(), "MirrSemantic stage must be present");
    assert!(semantic_stage.unwrap().ok, "MirrSemantic stage must pass");
}

#[test]
fn test_mirr_temporal_stage_present() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());

    let temporal_stage = result.stages.iter().find(|s| s.name == "MirrTemporal");
    assert!(temporal_stage.is_some(), "MirrTemporal stage must be present");
    assert!(temporal_stage.unwrap().ok, "MirrTemporal stage must pass");
}

#[test]
fn test_mirr_emitter_stage_present() {
    let f = write_temp_mirr(NEONATAL_SRC);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());

    let emitter_stage = result.stages.iter().find(|s| s.name == "MirrEmitter");
    assert!(emitter_stage.is_some(), "MirrEmitter stage must be present");
    assert!(emitter_stage.unwrap().ok, "MirrEmitter stage must pass");
}

#[test]
fn test_mirr_stages_with_multi_signal() {
    let f = write_temp_mirr(MULTI_SIGNAL);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());
    assert!(result.ok, "multi-signal module must pass all stages");
}

#[test]
fn test_mirr_stages_with_complex_guard() {
    let f = write_temp_mirr(COMPLEX_GUARD);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());
    assert!(result.ok, "complex guard module must pass all stages");
}

#[test]
fn test_mirr_stages_with_counter_guard() {
    let f = write_temp_mirr(COUNTER_GUARD);
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());
    assert!(result.ok, "counter guard module must pass all stages");
}

#[test]
fn test_mirr_stages_with_parse_error() {
    let f = write_temp_mirr("module bad { JUNK }");
    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(f.path());
    assert!(!result.ok, "parse error must fail pipeline");

    let parse_stage = result.stages.iter().find(|s| s.name == "Parse");
    assert!(parse_stage.is_some());
    assert!(!parse_stage.unwrap().ok);
}

#[test]
fn test_mirr_stages_all_lexer_examples() {
    let examples = [
        ("autonomous_vehicle", NEONATAL_SRC),
        ("fir_filter", SIMPLE_MODULE),
        ("neonatal_respirator", NEONATAL_SRC),
        ("tmr_sensor_fusion", MULTI_SIGNAL),
    ];

    for (name, src) in &examples {
        let f = write_temp_mirr(src);
        let runner =
            BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
        let result = runner.run(f.path());
        assert!(result.ok, "example '{name}' must pass all stages");
    }
}

#[test]
fn test_lexer_driver_with_mirr_stages() {
    let f = write_temp_mirr(SIMPLE_MODULE);
    let runner = BootstrapRunner::new(BootstrapOpts {
        run_lexer_driver: true,
        run_mirr_stages: true,
        ..Default::default()
    });
    let result = runner.run(f.path());
    assert!(result.ok, "lexer driver + MIRR stages must pass together");

    let lexer_stage = result.stages.iter().find(|s| s.name == "LexerDriver");
    assert!(lexer_stage.is_some());
    assert!(lexer_stage.unwrap().ok);
}

#[test]
fn test_self_compilation_lexer() {
    // Self-compilation test: compiler_mirr/lexer.mirr must parse through
    // the full Rust compiler pipeline.
    let lexer_path = std::path::Path::new("compiler_mirr").join("lexer.mirr");
    if !lexer_path.exists() {
        return;
    }

    let runner =
        BootstrapRunner::new(BootstrapOpts { run_mirr_stages: true, ..Default::default() });
    let result = runner.run(&lexer_path);

    // All Rust stages must pass
    let rust_stages = ["Read", "Parse", "Validate", "TemporalLower"];
    for stage_name in &rust_stages {
        let stage = result.stages.iter().find(|s| s.name == *stage_name);
        assert!(stage.is_some(), "stage '{stage_name}' must be present");
        assert!(stage.unwrap().ok, "stage '{stage_name}' must pass");
    }

    // All MIRR stages must pass
    let mirr_stages = ["MirrParser", "MirrSemantic", "MirrTemporal", "MirrEmitter"];
    for stage_name in &mirr_stages {
        let stage = result.stages.iter().find(|s| s.name == *stage_name);
        assert!(stage.is_some(), "MIRR stage '{stage_name}' must be present");
        assert!(
            stage.unwrap().ok,
            "MIRR stage '{stage_name}' must pass: {}",
            stage.unwrap().message
        );
    }
}
