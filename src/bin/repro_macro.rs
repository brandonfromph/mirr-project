use nasa_rust_project::compiler::macro_proc::expand_macros;
use std::fs;

fn main() {
    let content = fs::read_to_string("rspu_chip/rspu_top.mirr").unwrap();
    let expanded = expand_macros(&content);
    println!("{}", expanded);
}
