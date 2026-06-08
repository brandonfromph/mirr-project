//! Width SCC solver stress tests.

#![forbid(unsafe_code)]
#![deny(warnings)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

#[test]
fn width_scc_single_signal() {
    let source = "module test { signal x: in u8; }";
    let config = PipelineConfig { width: true, ..Default::default() };
    let result = run_pipeline(source, &config);
    assert!(result.is_ok(), "single signal should pass");
}

#[test]
fn width_scc_linear_dependency() {
    let source = r#"
module test {
    signal a: in bool;
    signal b: out bool;
    guard g {
        when a
        for 1 cycles;
    }
    reflex r {
        on g {
            b = a;
        }
    }
}
"#;
    let config = PipelineConfig { width: true, ..Default::default() };
    let result = run_pipeline(source, &config);
    if let Err(ref e) = result {
        eprintln!("ERROR: {:?}", e);
    }
    assert!(result.is_ok(), "linear dependency should pass");
}

#[test]
fn width_scc_independent_signals() {
    let source = r#"
module test {
    signal a: in u8;
    signal b: in u8;
    signal x: out u8;
    signal y: out u8;
    guard g1 {
        when a != 0
        for 1 cycles;
    }
    guard g2 {
        when b != 0
        for 1 cycles;
    }
    reflex r1 {
        on g1 {
            x = a;
        }
    }
    reflex r2 {
        on g2 {
            y = b;
        }
    }
}
"#;
    let config = PipelineConfig { width: true, ..Default::default() };
    let result = run_pipeline(source, &config);
    if let Err(ref e) = result {
        eprintln!("ERROR: {:#?}", e);
    }
    assert!(result.is_ok(), "independent signals should pass");
}

#[test]
fn width_scc_multiple_guards() {
    let source = r#"
module test {
    signal a: in u8;
    signal b: in u8;
    signal x: out u8;
    guard g1 {
        when a != 0
        for 1 cycles;
    }
    guard g2 {
        when b != 0
        for 1 cycles;
    }
    reflex r1 {
        on g1 {
            x = a;
        }
    }
}
"#;
    let config = PipelineConfig { width: true, ..Default::default() };
    let result = run_pipeline(source, &config);
    if let Err(ref e) = result {
        eprintln!("ERROR: {:#?}", e);
    }
    assert!(result.is_ok(), "multiple guards should pass");
}
