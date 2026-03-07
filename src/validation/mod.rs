//! Validation module for MIRR programs.
//!
//! Re-exports semantic validation for modules and pattern definitions.

pub mod semantic;

pub use semantic::{collect_signal_refs, validate_module, validate_pattern_defs};
