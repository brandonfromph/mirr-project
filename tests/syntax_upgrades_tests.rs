#![forbid(unsafe_code)]

use mirrc::ast::PatternDef;
use mirrc::parse_mirr;

fn ok_pattern(src: &str) -> PatternDef {
    let full_src = format!("{}\nmodule m {{}}", src);
    let prog = parse_mirr(&full_src).unwrap_or_else(|e| panic!("Failed to parse: {:?}", e));
    prog.patterns.into_iter().next().unwrap()
}

#[test]
fn test_array_parameters_in_pattern() {
    ok_pattern("def my_router(tx_valid: signal in bool[64]) { reflect {} }");
}

#[test]
fn test_flattened_pattern_syntax() {
    ok_pattern("pattern my_core(clk: signal in bool) { signal x: internal bool; }");
}

#[test]
fn test_vector_reduction_or() {
    let src = "
    module m {
        signal arr: internal bool[64];
        signal out: internal bool;
        reflex process {
            on always {
                out = |arr;
            }
        }
    }
    ";
    parse_mirr(src).unwrap_or_else(|e| panic!("Failed to parse: {:?}", e));
}
