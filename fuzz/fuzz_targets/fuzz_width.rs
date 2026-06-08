#![no_main]
use libfuzzer_sys::fuzz_target;
use mirrc::parse_mirr;
use mirrc::width;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(program) = parse_mirr(s) {
            let _ = width::infer_program_widths_with_scc(&program, None);
        }
    }
});
