//! Legacy LRA commands for the interactive paper platform.
//!
//! These commands are deprecated and will be removed in a future version.
//! Use the build certification workflow instead: init → compile → receipt → verify

#![forbid(unsafe_code)]

/// Print deprecation warning for legacy commands.
pub fn warn_deprecated(command: &str) {
    eprintln!("WARNING: `lra {command}` is deprecated and will be removed in a future version.");
    eprintln!("The LRA CLI is now focused on build certification.");
    eprintln!("See `lra --help` for the new workflow: init → compile → receipt → verify");
    eprintln!();
}
