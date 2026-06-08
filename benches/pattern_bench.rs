use criterion::{criterion_group, criterion_main, Criterion};
use mirrc::pipeline::{run_pipeline, PipelineConfig};

fn bench_pattern_expansion(c: &mut Criterion) {
    let source = r#"
module pattern_test {
    signal in a: Bool;
    signal in b: Bool;
    signal out c: Bool;

    pattern and_gate(x: in Bool, y: in Bool, z: out Bool) {
        g: when x && y for 1;
        reflex r: on g { z = x && y; }
    }

    and_gate(a, b, c);
}
"#;

    let config = PipelineConfig {
        typecheck: true,
        simplify: false,
        width: true,
        temporal: false,
        ..PipelineConfig::default()
    };

    c.bench_function("pattern_expansion", |b| b.iter(|| run_pipeline(source, &config)));
}

criterion_group!(benches, bench_pattern_expansion);
criterion_main!(benches);
