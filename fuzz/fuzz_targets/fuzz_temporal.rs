#![no_main]
use libfuzzer_sys::fuzz_target;
use mirrc::{parse_mirr, temporal::TemporalGuardCompiler};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(program) = parse_mirr(s) {
            let mut compiler = TemporalGuardCompiler::new();
            let _ = compiler.compile_temporal_guards(&program.module);
        }
    }
});
