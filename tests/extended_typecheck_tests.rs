//! Integration tests for MEGA-1 extended type checking through the pipeline.

#![forbid(unsafe_code)]

use nasa_rust_project::error::PipelineErrors;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig, PipelineResult};

/// Helper: run pipeline with extended typechecking enabled.
fn run_extended(source: &str) -> Result<PipelineResult, PipelineErrors> {
    let config = PipelineConfig { extended_typecheck: true, ..PipelineConfig::default() };
    run_pipeline(source, &config)
}

/// Helper: run pipeline with default config (extended_typecheck = false).
fn run_default(source: &str) -> Result<PipelineResult, PipelineErrors> {
    run_pipeline(source, &PipelineConfig::default())
}

/// Minimal MIRR source with correct line-oriented syntax.
fn plain_module() -> &'static str {
    "module test {\n\
     signal x: in bool;\n\
     signal y: out bool;\n\
     guard g {\n\
     when x\n\
     for 1 cycles;\n\
     }\n\
     reflex r {\n\
     on g {\n\
     y = x;\n\
     }\n\
     }\n\
     }"
}

/// Build a MIRR module with a custom signal declaration line.
fn module_with_signal(sig_line: &str) -> String {
    format!(
        "module test {{\n\
         {}\n\
         signal y: out u16;\n\
         guard g {{\n\
         when x > 0\n\
         for 1 cycles;\n\
         }}\n\
         reflex r {{\n\
         on g {{\n\
         y = x;\n\
         }}\n\
         }}\n\
         }}",
        sig_line
    )
}

/// Build a MIRR module with bool signals and custom signal declaration line.
fn module_with_bool_signal(sig_line: &str) -> String {
    format!(
        "module test {{\n\
         {}\n\
         signal y: out bool;\n\
         guard g {{\n\
         when x\n\
         for 1 cycles;\n\
         }}\n\
         reflex r {{\n\
         on g {{\n\
         y = x;\n\
         }}\n\
         }}\n\
         }}",
        sig_line
    )
}

// ---------------------------------------------------------------------------
// Baseline tests: extended_typecheck=false should behave identically
// ---------------------------------------------------------------------------

#[test]
fn baseline_default_config_no_extended() {
    let result = run_default(plain_module());
    assert!(result.is_ok(), "Default config should succeed: {:?}", result.err());
    assert!(result.unwrap().extended_type_map.is_none());
}

#[test]
fn baseline_extended_on_plain_module() {
    let result = run_extended(plain_module());
    assert!(result.is_ok(), "Extended on plain module should succeed: {:?}", result.err());
    assert!(result.unwrap().extended_type_map.is_some());
}

// ---------------------------------------------------------------------------
// Annotation propagation tests
// ---------------------------------------------------------------------------

#[test]
fn extended_with_linear_annotation() {
    let source = module_with_bool_signal("signal x: in linear bool;");
    let result = run_extended(&source);
    assert!(result.is_ok(), "Linear annotation should pass: {:?}", result.err());
}

#[test]
fn extended_with_stateful_annotation() {
    let source = module_with_signal("signal x: in stateful u16;");
    let result = run_extended(&source);
    assert!(result.is_ok(), "Stateful annotation should pass: {:?}", result.err());
}

#[test]
fn extended_with_pure_annotation() {
    let source = module_with_signal("signal x: in pure u16;");
    let result = run_extended(&source);
    assert!(result.is_ok(), "Pure annotation should pass: {:?}", result.err());
}

#[test]
fn extended_with_refinement_annotation() {
    let source = module_with_signal("signal x: in u16 where 0..200;");
    let result = run_extended(&source);
    assert!(result.is_ok(), "Refinement annotation should pass: {:?}", result.err());
}

#[test]
fn extended_with_clock_domain() {
    // Clock domain checking correctly rejects undeclared domains (E619).
    // The pipeline passes empty clock_domains list, so @sys_clk is undeclared.
    let source = module_with_signal("signal x: in u16 @sys_clk;");
    let result = run_extended(&source);
    assert!(result.is_err(), "Undeclared clock domain should produce E619");
    let errors = match result {
        Err(e) => e,
        Ok(_) => unreachable!(),
    };
    let msg = format!("{:?}", errors);
    assert!(msg.contains("E619"), "Should contain E619: {}", msg);
}

#[test]
fn extended_with_phantom_tag() {
    // Phantom tag checking correctly rejects undeclared tags (E621).
    // The pipeline passes empty phantom_tags list, so #Voltage is undeclared.
    let source = module_with_signal("signal x: in u16 #Voltage;");
    let result = run_extended(&source);
    assert!(result.is_err(), "Undeclared phantom tag should produce E621");
    let errors = match result {
        Err(e) => e,
        Ok(_) => unreachable!(),
    };
    let msg = format!("{:?}", errors);
    assert!(msg.contains("E621"), "Should contain E621: {}", msg);
}

#[test]
fn extended_with_all_annotations() {
    // Clock domain and phantom tag are undeclared, so E619 + E621 expected.
    let source = module_with_signal(
        "signal x: in linear stateful u16 where 0..1000 @fast_clk #Temperature;",
    );
    let result = run_extended(&source);
    assert!(result.is_err(), "Undeclared clock+phantom should produce errors");
    let errors = match result {
        Err(e) => e,
        Ok(_) => unreachable!(),
    };
    let msg = format!("{:?}", errors);
    assert!(msg.contains("E619"), "Should contain E619: {}", msg);
    assert!(msg.contains("E621"), "Should contain E621: {}", msg);
}

// ---------------------------------------------------------------------------
// Extended type map population
// ---------------------------------------------------------------------------

#[test]
fn extended_type_map_is_populated() {
    let source = module_with_signal("signal x: in u16;");
    let result = run_extended(&source).expect("should succeed");
    let ext_map = result.extended_type_map.expect("extended_type_map should be Some");
    // The map should contain entries for the analyzed expressions.
    assert!(!ext_map.is_empty(), "Extended type map should not be empty");
}

// ---------------------------------------------------------------------------
// Existing examples still pass with extended checking
// ---------------------------------------------------------------------------

#[test]
fn extended_on_existing_example_tmr() {
    let source = std::fs::read_to_string("examples/tmr_sensor_fusion.mirr")
        .expect("example file must exist");
    let result = run_extended(&source);
    assert!(result.is_ok(), "TMR example should pass extended checking: {:?}", result.err());
}

#[test]
fn extended_on_existing_example_neonatal() {
    let source = std::fs::read_to_string("examples/neonatal_respirator.mirr")
        .expect("example file must exist");
    let result = run_extended(&source);
    assert!(result.is_ok(), "Neonatal example should pass extended checking: {:?}", result.err());
}

#[test]
fn extended_on_existing_example_icu() {
    let source =
        std::fs::read_to_string("examples/icu_monitor.mirr").expect("example file must exist");
    let result = run_extended(&source);
    assert!(result.is_ok(), "ICU example should pass extended checking: {:?}", result.err());
}
