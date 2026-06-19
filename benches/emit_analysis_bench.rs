//! Criterion benchmarks for the MIRR emit backends and analysis passes.
//!
//! Four groups — emit_verilog, emit_firrtl, typecheck, width_infer — exercise
//! the downstream stages after parsing using real example files from `examples/`.
//! All inputs are loaded at compile time via `include_str!`.

#![forbid(unsafe_code)]

use criterion::{criterion_group, criterion_main, Criterion};
use mirrc::emit::firrtl::emit_firrtl;
use mirrc::emit::verilog::emit_sv;
use mirrc::{run_pipeline, run_pipeline_with_file, PipelineConfig};
use std::hint::black_box;

fn typecheck_ecs(source: &str) {
    let config = PipelineConfig {
        temporal: false,
        width: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };
    let _ = run_pipeline_with_file(source, "dummy.mirr", &config);
}

fn width_infer_ecs(source: &str) {
    let config = PipelineConfig {
        temporal: false,
        width: true,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };
    let _ = run_pipeline_with_file(source, "dummy.mirr", &config);
}

// ---------------------------------------------------------------------------
// Input sources (real example files, compile-time embedded)
// ---------------------------------------------------------------------------

/// Medium complexity: ~60 lines of MIRR, flight control logic.
const SRC_FC: &str = include_str!("../examples/flight_controller.mirr");

/// High complexity: ~100 lines, Triple Modular Redundancy (TMR) fusion.
const SRC_TMR: &str = include_str!("../examples/tmr_sensor_fusion.mirr");

// ---------------------------------------------------------------------------
// Benchmark Functions
// ---------------------------------------------------------------------------

fn bench_emit_verilog(c: &mut Criterion) {
    let result_fc = run_pipeline(SRC_FC, &PipelineConfig::default()).unwrap();
    let result_tmr = run_pipeline(SRC_TMR, &PipelineConfig::default()).unwrap();

    let mut group = c.benchmark_group("emit_verilog");
    group.bench_function("flight_controller", |b| b.iter(|| emit_sv(black_box(&result_fc))));
    group.bench_function("tmr", |b| b.iter(|| emit_sv(black_box(&result_tmr))));
    group.finish();
}

fn bench_emit_firrtl(c: &mut Criterion) {
    let result_fc = run_pipeline(SRC_FC, &PipelineConfig::default()).unwrap();
    let result_tmr = run_pipeline(SRC_TMR, &PipelineConfig::default()).unwrap();

    let mut group = c.benchmark_group("emit_firrtl");
    group.bench_function("flight_controller", |b| b.iter(|| emit_firrtl(black_box(&result_fc))));
    group.bench_function("tmr", |b| b.iter(|| emit_firrtl(black_box(&result_tmr))));
    group.finish();
}

fn bench_typecheck(c: &mut Criterion) {
    let mut group = c.benchmark_group("typecheck");
    group.bench_function("flight_controller", |b| b.iter(|| typecheck_ecs(black_box(SRC_FC))));
    group.bench_function("tmr", |b| b.iter(|| typecheck_ecs(black_box(SRC_TMR))));
    group.finish();
}

fn bench_width_infer(c: &mut Criterion) {
    let mut group = c.benchmark_group("width_infer");
    group.bench_function("flight_controller", |b| b.iter(|| width_infer_ecs(black_box(SRC_FC))));
    group.bench_function("tmr", |b| b.iter(|| width_infer_ecs(black_box(SRC_TMR))));
    group.finish();
}

criterion_group!(
    benches,
    bench_emit_verilog,
    bench_emit_firrtl,
    bench_typecheck,
    bench_width_infer
);
criterion_main!(benches);
