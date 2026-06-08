#![no_main]
use libfuzzer_sys::fuzz_target;
use mirrc::{parse_mirr, typecheck_module};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(program) = parse_mirr(s) {
            let _ = typecheck_module(&program.module);
        }
    }
});
