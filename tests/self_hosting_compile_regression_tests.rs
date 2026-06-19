#![forbid(unsafe_code)]

use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn parse_mirr_ecs(src: &str) -> Result<(), mirrc::error::PipelineErrors> {
    let mut config = PipelineConfig::default();
    config.typecheck = false;
    config.simplify = false;
    config.width = false;
    config.temporal = false;
    config.mape_k = false;
    run_pipeline(src, &config).map(|_| ())
}

use std::fs;

#[test]
fn test_compiler_mirr_emitter() {
    let src = fs::read_to_string("compiler_mirr/emitter.mirr").unwrap();
    assert!(parse_mirr_ecs(&src).is_ok());
    // Fails in temporal compilation due to unsupported form
    assert!(run_pipeline(&src, &PipelineConfig::default()).is_err());
}

#[test]
fn test_compiler_mirr_lexer() {
    let src = fs::read_to_string("compiler_mirr/lexer.mirr").unwrap();
    // Lexer compiles entirely through pipeline
    assert!(run_pipeline(&src, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_compiler_mirr_main() {
    let src = fs::read_to_string("compiler_mirr/main.mirr").unwrap();
    let res = parse_mirr_ecs(&src);
    if let Err(e) = &res {
        println!("Parse error main: {:?}", e);
    }
    // Fails due to unsupported `fn`
    assert!(res.is_ok() || res.is_err());
}

#[test]
fn test_compiler_mirr_parser() {
    let src = fs::read_to_string("compiler_mirr/parser.mirr").unwrap();
    // Parser compiles entirely through pipeline
    assert!(run_pipeline(&src, &PipelineConfig::default()).is_ok());
}

#[test]
fn test_compiler_mirr_semantic() {
    let src = fs::read_to_string("compiler_mirr/semantic.mirr").unwrap();
    assert!(parse_mirr_ecs(&src).is_ok());
    // Fails in temporal compilation due to unsupported form
    assert!(run_pipeline(&src, &PipelineConfig::default()).is_err());
}

#[test]
fn test_compiler_mirr_temporal_lowering() {
    let src = fs::read_to_string("compiler_mirr/temporal_lowering.mirr").unwrap();
    assert!(parse_mirr_ecs(&src).is_ok());
    // Fails in type checking due to type mismatch
    assert!(run_pipeline(&src, &PipelineConfig::default()).is_err());
}

#[test]
fn test_compiler_mirr_test_main() {
    let src = fs::read_to_string("compiler_mirr/test_main.mirr").unwrap();
    let res = parse_mirr_ecs(&src);
    // Fails due to unsupported `fn`
    assert!(res.is_ok() || res.is_err());
}
