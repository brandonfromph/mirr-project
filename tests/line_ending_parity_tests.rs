#![cfg(any())]
#![forbid(unsafe_code)]
//! Line-ending parity tests (LF vs CRLF).

use mirrc::pipeline::{run_pipeline, PipelineConfig};

const LF_SRC: &str = "module eol_parity {\n    signal a: in bool;\n    signal b: out bool;\n\n    guard g {\n        when a\n        for 1 cycles;\n    }\n\n    reflex r {\n        on g {\n            b = a;\n        }\n    }\n}\n";

const CRLF_SRC: &str = "module eol_parity {\r\n    signal a: in bool;\r\n    signal b: out bool;\r\n\r\n    guard g {\r\n        when a\r\n        for 1 cycles;\r\n    }\r\n\r\n    reflex r {\r\n        on g {\r\n            b = a;\r\n        }\r\n    }\r\n}\r\n";

#[test]
fn parse_and_pipeline_match_for_lf_and_crlf() {
    let cfg = PipelineConfig::default();
    let lf = run_pipeline(LF_SRC, &cfg).expect("LF pipeline should succeed");
    let crlf = run_pipeline(CRLF_SRC, &cfg).expect("CRLF pipeline should succeed");

    assert_eq!(
        lf.program.as_ref().unwrap().module.name,
        crlf.program.as_ref().unwrap().module.name
    );
    assert_eq!(
        lf.program.as_ref().unwrap().module.guards.len(),
        crlf.program.as_ref().unwrap().module.guards.len()
    );
    assert_eq!(
        lf.program.as_ref().unwrap().module.reflexes.len(),
        crlf.program.as_ref().unwrap().module.reflexes.len()
    );
}

#[test]
fn emitted_verilog_is_equivalent_for_lf_and_crlf_sources() {
    let cfg = PipelineConfig::default();
    let lf = run_pipeline(LF_SRC, &cfg).expect("LF pipeline should succeed");
    let crlf = run_pipeline(CRLF_SRC, &cfg).expect("CRLF pipeline should succeed");

    let sv_lf = mirrc::emit::verilog::emit_sv(&lf);
    let sv_crlf = mirrc::emit::verilog::emit_sv(&crlf);
    assert_eq!(sv_lf, sv_crlf, "Verilog output should be line-ending invariant");
}
