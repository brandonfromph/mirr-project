#![no_main]
use libfuzzer_sys::fuzz_target;
use nasa_rust_project::{parse_mirr, infer_program_widths_with_scc};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(program) = parse_mirr(s) {
            let _ = infer_program_widths_with_scc(&program, None);
        }
    }
});
