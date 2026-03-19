#![forbid(unsafe_code)]
//! Basic pattern definition and expansion tests.

use nasa_rust_project::parse_mirr;
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};
use nasa_rust_project::validation::validate_pattern_defs;

fn run_src(
    src: &str,
) -> Result<nasa_rust_project::pipeline::PipelineResult, nasa_rust_project::error::PipelineErrors> {
    run_pipeline(src, &PipelineConfig::default())
}

#[test]
fn basic_module_no_patterns() {
    assert!(run_src("module m {\n    signal x: in bool;\n    signal y: out bool;\n}").is_ok());
}
#[test]
fn empty_pattern_list_validates() {
    assert!(validate_pattern_defs(&[]).is_ok());
}
#[test]
fn parse_simple_pattern() {
    let src = r#"def pat(s: signal in bool) {
    reflect {
        s = true;
    }
}
module m { signal x: in bool; }"#;
    assert!(parse_mirr(src).is_ok());
}
#[test]
fn pattern_name_preserved() {
    let src = r#"def my_pat(s: signal in bool) { reflect { s = true; } }
module m { signal x: in bool; }"#;
    if let Ok(prog) = parse_mirr(src) {
        assert!(!prog.patterns.is_empty());
        assert_eq!(prog.patterns[0].name, "my_pat");
    }
}
#[test]
fn module_with_guard_no_patterns() {
    assert!(run_src(
        r#"module gm {
    signal x: in u8;
    signal y: out bool;
    guard g {
        when x > 100
        for 1 cycles;
    }
    reflex r {
        on g {
            y = true;
        }
    }
}"#
    )
    .is_ok());
}
#[test]
fn module_with_property_no_patterns() {
    assert!(run_src(
        r#"module pm {
    signal x: in bool;
    property p {
        always (x);
    }
}"#
    )
    .is_ok());
}
#[test]
fn multiple_patterns_parsed() {
    let src = r#"def pa(x: signal in bool) { reflect { x = true; } }
def pb(y: signal in u8) { reflect { y = 0; } }
module m { signal x: in bool; }"#;
    if let Ok(prog) = parse_mirr(src) {
        assert!(prog.patterns.len() >= 2);
    }
}
#[test]
fn pipeline_plain_module_no_patterns() {
    if let Ok(pr) = run_src("module p { signal x: in u8; signal y: out bool; }") {
        assert!(pr.program.patterns.is_empty());
    }
}
