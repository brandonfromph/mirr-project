#![no_main]
use libfuzzer_sys::fuzz_target;
use nasa_rust_project::{
    parse_mirr,
    sexpr::{ast_to_sexpr, parser::parse_sexpr, printer::print_sexpr, sexpr_to_ast},
};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(program) = parse_mirr(s) {
            let sexpr = ast_to_sexpr(&program);
            let printed = print_sexpr(&sexpr);
            let _ = parse_sexpr(&printed);
            let _ = sexpr_to_ast(&sexpr);
        }
    }
});
