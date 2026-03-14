#![forbid(unsafe_code)]
//! Criterion benchmarks for the MIRR compiler pipeline.
//!
//! Three tiers — small, medium, large — exercise parse_mirr() and run_pipeline()
//! with increasing input complexity. All input generators use bounded iteration.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nasa_rust_project::{parse_mirr, run_pipeline, PipelineConfig};

// ---------------------------------------------------------------------------
// Input generators (bounded iteration, NASA-compliant)
// ---------------------------------------------------------------------------

/// Small: 1 module, 2 signals, 1 guard, 1 reflex.
fn small_input() -> String {
    r#"module bench_small {
    signal sensor: in u16;
    signal alarm: out bool;

    guard g {
        when sensor > 100
        for 3 cycles;
    }

    reflex r {
        on g {
            alarm = true;
        }
    }
}
"#
    .to_string()
}

/// Medium: 1 module, 8 signals, 4 guards, 4 reflexes.
fn medium_input() -> String {
    let mut src = String::with_capacity(2048);
    src.push_str("module bench_medium {\n");

    // 4 input signals
    const N: usize = 4;
    for i in 0..N {
        src.push_str(&format!("    signal s{i}: in u16;\n"));
    }
    // 4 output signals
    for i in 0..N {
        src.push_str(&format!("    signal a{i}: out bool;\n"));
    }
    src.push('\n');

    // 4 guards
    for i in 0..N {
        src.push_str(&format!(
            "    guard g{i} {{\n        when s{i} > {thresh}\n        for {cyc} cycles;\n    }}\n\n",
            thresh = (i + 1) * 50,
            cyc = (i + 1) * 2,
        ));
    }

    // 4 reflexes
    for i in 0..N {
        src.push_str(&format!(
            "    reflex r{i} {{\n        on g{i} {{\n            a{i} = true;\n        }}\n    }}\n\n",
        ));
    }

    src.push_str("}\n");
    src
}

/// Large: 1 module, 32 signals, 16 guards, 16 reflexes.
fn large_input() -> String {
    let mut src = String::with_capacity(8192);
    src.push_str("module bench_large {\n");

    const N: usize = 16;
    // 16 input signals
    for i in 0..N {
        src.push_str(&format!("    signal s{i}: in u16;\n"));
    }
    // 16 output signals
    for i in 0..N {
        src.push_str(&format!("    signal a{i}: out bool;\n"));
    }
    src.push('\n');

    // 16 guards
    for i in 0..N {
        src.push_str(&format!(
            "    guard g{i} {{\n        when s{i} > {thresh}\n        for {cyc} cycles;\n    }}\n\n",
            thresh = (i + 1) * 25,
            cyc = (i % 8) + 1,
        ));
    }

    // 16 reflexes
    for i in 0..N {
        src.push_str(&format!(
            "    reflex r{i} {{\n        on g{i} {{\n            a{i} = true;\n        }}\n    }}\n\n",
        ));
    }

    src.push_str("}\n");
    src
}

// ---------------------------------------------------------------------------
// Benchmark groups
// ---------------------------------------------------------------------------

fn bench_parse(c: &mut Criterion) {
    let small = small_input();
    let medium = medium_input();
    let large = large_input();

    let mut group = c.benchmark_group("parse");
    group.bench_function("small", |b| b.iter(|| parse_mirr(black_box(&small))));
    group.bench_function("medium", |b| b.iter(|| parse_mirr(black_box(&medium))));
    group.bench_function("large", |b| b.iter(|| parse_mirr(black_box(&large))));
    group.finish();
}

fn bench_pipeline(c: &mut Criterion) {
    let small = small_input();
    let medium = medium_input();
    let large = large_input();

    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        rspu: false,
        extended_typecheck: false,
        simulate: false,
        mape_k: false,
        ..PipelineConfig::default()
    };

    let mut group = c.benchmark_group("pipeline");
    group.bench_function("small", |b| {
        b.iter(|| run_pipeline(black_box(&small), black_box(&config)))
    });
    group.bench_function("medium", |b| {
        b.iter(|| run_pipeline(black_box(&medium), black_box(&config)))
    });
    group.bench_function("large", |b| {
        b.iter(|| run_pipeline(black_box(&large), black_box(&config)))
    });
    group.finish();
}

criterion_group!(benches, bench_parse, bench_pipeline);
criterion_main!(benches);
