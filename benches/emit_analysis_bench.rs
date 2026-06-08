//! Criterion benchmarks for the MIRR emit backends and analysis passes.
//!
//! Four groups — emit_verilog, emit_firrtl, typecheck, width_infer — exercise
//! the downstream stages after parsing using real example files from `examples/`.
//! All inputs are loaded at compile time via `include_str!`.

#![forbid(unsafe_code)]

use criterion::{criterion_group, criterion_main, Criterion};
use mirrc::emit::firrtl::emit_firrtl;
use mirrc::emit::verilog::emit_sv;
use mirrc::width::infer_program_widths_with_scc;
use mirrc::{parse_mirr, run_pipeline, typecheck_module, PipelineConfig};
use std::hint::black_box;

// ---------------------------------------------------------------------------
// Input sources (real example files, compile-time embedded)
// ---------------------------------------------------------------------------

/// Medium complexity: ~60 lines of MIRR, flight control logic.
const SRC_FC: &str = include_str!("../examples/flight_controller.mirr");

/// High complexity: ~45 lines, triple modular redundancy.
const SRC_TMR: &str = include_str!("../examples/tmr_sensor_fusion.mirr");

// ---------------------------------------------------------------------------
// Shared config helper
// ---------------------------------------------------------------------------

/// Full pipeline config with all analysis passes enabled.
fn full_config() -> PipelineConfig {
    PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    }
}

// ---------------------------------------------------------------------------
// Benchmark groups
// ---------------------------------------------------------------------------

fn bench_emit_verilog(c: &mut Criterion) {
    let config = full_config();
    let result_fc = run_pipeline(SRC_FC, &config).unwrap();
    let result_tmr = run_pipeline(SRC_TMR, &config).unwrap();

    let mut group = c.benchmark_group("emit_verilog");
    group.bench_function("flight_controller", |b| b.iter(|| emit_sv(black_box(&result_fc))));
    group.bench_function("tmr", |b| b.iter(|| emit_sv(black_box(&result_tmr))));
    group.finish();
}

fn bench_emit_firrtl(c: &mut Criterion) {
    let config = full_config();
    let result_fc = run_pipeline(SRC_FC, &config).unwrap();
    let result_tmr = run_pipeline(SRC_TMR, &config).unwrap();

    let mut group = c.benchmark_group("emit_firrtl");
    group.bench_function("flight_controller", |b| b.iter(|| emit_firrtl(black_box(&result_fc))));
    group.bench_function("tmr", |b| b.iter(|| emit_firrtl(black_box(&result_tmr))));
    group.finish();
}

fn bench_typecheck(c: &mut Criterion) {
    let prog_fc = parse_mirr(SRC_FC).unwrap();
    let prog_tmr = parse_mirr(SRC_TMR).unwrap();

    let mut group = c.benchmark_group("typecheck");
    group.bench_function("flight_controller", |b| {
        b.iter(|| typecheck_module(black_box(&prog_fc.module)))
    });
    group.bench_function("tmr", |b| b.iter(|| typecheck_module(black_box(&prog_tmr.module))));
    group.finish();
}

fn bench_width_infer(c: &mut Criterion) {
    let prog_fc = parse_mirr(SRC_FC).unwrap();
    let prog_tmr = parse_mirr(SRC_TMR).unwrap();

    let mut group = c.benchmark_group("width_infer");
    group.bench_function("flight_controller", |b| {
        b.iter(|| infer_program_widths_with_scc(black_box(&prog_fc), None))
    });
    group.bench_function("tmr", |b| {
        b.iter(|| infer_program_widths_with_scc(black_box(&prog_tmr), None))
    });
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
