#[test]
fn scratch_test() {
    let src = "module m { signal s1: internal bool; reflex r1 { on always { s1 = true; } } }";
    let res = mirrc::pipeline::run_pipeline(src, &mirrc::pipeline::PipelineConfig::default());
    println!("Error: {:?}", res.err());
}
