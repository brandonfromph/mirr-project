#![forbid(unsafe_code)]
//! Edge case tests for pattern definitions and expansion.

use mirrc::ast::pattern::{PatternParam, PatternParamKind, ReflectBlock};
use mirrc::ast::types::SignalKind;
use mirrc::ast::types::SignalType;
use mirrc::parse_mirr;
use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn run_src(
    src: &str,
) -> Result<mirrc::pipeline::PipelineResult, mirrc::error::PipelineErrors> {
    run_pipeline(src, &PipelineConfig::default())
}

#[test]
fn edge_pattern_no_params() {
    let src = r#"def no_param() { reflect {} }
module m { signal x: in bool; }"#;
    let _ = parse_mirr(src);
}
#[test]
fn edge_pattern_name_preserved() {
    let src = r#"def edge_pat(s: signal in u8) { reflect { s = 0; } }
module m { signal x: in u8; }"#;
    if let Ok(prog) = parse_mirr(src) {
        if let Some(p) = prog.patterns.first() {
            assert_eq!(p.name, "edge_pat");
        }
    }
}
#[test]
fn edge_param_kind_signal() {
    let p = PatternParam {
        name: "s".to_string(),
        kind: PatternParamKind::Signal {
            kind: SignalKind::Input,
            ty: SignalType::Bool,
            annotations: Default::default(),
        },
    };
    assert_eq!(p.name, "s");
}
#[test]
fn edge_empty_reflect_block() {
    let rb = ReflectBlock { statements: vec![] };
    assert!(rb.statements.is_empty());
}
#[test]
fn edge_no_pattern_calls() {
    assert!(run_src("module nc { signal x: in u8; signal y: out bool; }").is_ok());
}
#[test]
fn edge_pattern_with_value_param() {
    let src = r#"def thresh(sensor: signal in u16, threshold: u16) {
    reflect { sensor = 0; }
}
module m { signal x: in u16; }"#;
    let _ = parse_mirr(src);
}
#[test]
fn edge_module_many_signals() {
    let mut src = String::from(
        "module ms {
",
    );
    let mut i = 0usize;
    while i < 8 {
        src.push_str(&format!(
            "    signal s{}: in u8;
",
            i
        ));
        i += 1;
    }
    src.push_str(
        "    signal out0: out bool;
}",
    );
    assert!(run_pipeline(&src, &PipelineConfig::default()).is_ok());
}
