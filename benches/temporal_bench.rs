use criterion::{criterion_group, criterion_main, Criterion};
use nasa_rust_project::pipeline::{run_pipeline, PipelineConfig};

fn bench_temporal_lowering(c: &mut Criterion) {
    let source = r#"
module temporal_test {
    signal in clk: Bool;
    signal in rst: Bool;
    signal in trigger: Bool;
    signal out delayed: Bool;
    signal out counted: Bool;

    guard shift_g: when trigger for 5;
    guard counter_g: when trigger for 10;

    reflex r: on shift_g, counter_g {
        delayed = trigger;
        counted = trigger;
    }

    property safety: always (trigger -> eventually within 5 (delayed));
}
"#;

    let config = PipelineConfig {
        typecheck: true,
        simplify: true,
        width: true,
        temporal: true,
        ..PipelineConfig::default()
    };

    c.bench_function("temporal_lowering", |b| b.iter(|| run_pipeline(source, &config)));
}

criterion_group!(benches, bench_temporal_lowering);
criterion_main!(benches);
