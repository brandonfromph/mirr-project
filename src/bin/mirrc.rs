#![forbid(unsafe_code)]

#[path = "mirr-compile/main.rs"]
mod mirr_compile_main;

pub fn main() {
    mirr_compile_main::main();
}
