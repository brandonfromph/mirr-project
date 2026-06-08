//! MEGA-12: HLS FIFO tests.
//!
//! Tests bounded FIFO synthesis, handshake, SystemVerilog emission,
//! and depth validation.

#![forbid(unsafe_code)]

use mirrc::hls::fifo::{
    emit_fifo_verilog, fifo_stats, synthesize_fifo, validate_fifo, FifoHandshake, FifoHardware,
    MAX_FIFO_DEPTH,
};

#[test]
fn test_fifo_new() {
    let fifo = FifoHardware::new(4, 8).unwrap();
    assert_eq!(fifo.depth, 4);
    assert_eq!(fifo.elem_width, 8);
    assert!(fifo.empty);
    assert!(!fifo.full);
}

#[test]
fn test_fifo_new_invalid_depth() {
    let result = FifoHardware::new(0, 8);
    assert!(result.is_err());
}

#[test]
fn test_fifo_new_exceeds_max() {
    let result = FifoHardware::new(MAX_FIFO_DEPTH + 1, 8);
    assert!(result.is_err());
}

#[test]
fn test_fifo_push_pop_basic() {
    let mut fifo = FifoHardware::new(4, 8).unwrap();
    assert!(fifo.empty);
    fifo.push().unwrap();
    assert!(!fifo.empty);
    assert_eq!(fifo.count(), 1);
    fifo.pop().unwrap();
    assert!(fifo.empty);
}

#[test]
fn test_fifo_full() {
    let mut fifo = FifoHardware::new(2, 8).unwrap();
    fifo.push().unwrap();
    fifo.push().unwrap();
    assert!(fifo.full);
    let result = fifo.push();
    assert!(result.is_err());
}

#[test]
fn test_fifo_empty_pop() {
    let mut fifo = FifoHardware::new(4, 8).unwrap();
    let result = fifo.pop();
    assert!(result.is_err());
}

#[test]
fn test_fifo_total_width() {
    let fifo = FifoHardware::new(8, 16).unwrap();
    assert_eq!(fifo.total_width(), 128);
}

#[test]
fn test_synthesize_fifo() {
    let fifo = synthesize_fifo(4, 8).unwrap();
    assert_eq!(fifo.depth, 4);
}

#[test]
fn test_validate_fifo() {
    assert!(validate_fifo(4, 8).is_ok());
    assert!(validate_fifo(0, 8).is_err());
    assert!(validate_fifo(MAX_FIFO_DEPTH + 1, 8).is_err());
}

#[test]
fn test_fifo_handshake_empty() {
    let fifo = FifoHardware::new(4, 8).unwrap();
    let handshake = FifoHandshake::from_fifo(&fifo);
    assert!(handshake.push_ready);
    assert!(!handshake.pop_ready);
}

#[test]
fn test_emit_fifo_verilog() {
    let fifo = FifoHardware::new(4, 8).unwrap();
    let verilog = emit_fifo_verilog(&fifo, "my_fifo");
    assert!(verilog.contains("module my_fifo"));
    assert!(verilog.contains("DEPTH = 4"));
    assert!(verilog.contains("WIDTH = 8"));
}

#[test]
fn test_fifo_stats() {
    let fifos = vec![FifoHardware::new(4, 8).unwrap(), FifoHardware::new(16, 32).unwrap()];
    let stats = fifo_stats(&fifos);
    assert_eq!(stats.total_fifos, 2);
    assert_eq!(stats.max_depth, 16);
}

#[test]
fn test_pipeline_fifo_type_parse() {
    let source = r#"
module test {
    signal buf: internal fifo<u8, 4>;
    signal x: in u8;
    signal y: out u8;
    signal trigger: in u8;
    guard tick {
        when trigger == 1
        for 1 cycles;
    }
    reflex compute {
        on tick {
            y = x;
        }
    }
}
"#;

    let config =
        mirrc::PipelineConfig { hls: false, rspu: false, mape_k: false, ..Default::default() };

    let result = mirrc::run_pipeline(source, &config);
    if let Err(e) = &result {
        eprintln!("Pipeline error: {e:?}");
    }
    assert!(result.is_ok());
}
