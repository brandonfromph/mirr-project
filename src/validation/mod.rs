// ---------------------------------------------------------------------------
// Validation module — public interface
// ---------------------------------------------------------------------------

pub mod semantic;

pub use semantic::{collect_signal_refs, validate_module};
