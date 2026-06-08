#![forbid(unsafe_code)]

#[path = "mirr-compile/main.rs"]
mod mirr_compile_main;

pub fn main() -> anyhow::Result<()> {
    mirr_compile_main::main()?;
    Ok(())
}
