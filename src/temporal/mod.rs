#![forbid(unsafe_code)]
#![deny(warnings)]

// ---------------------------------------------------------------------------
// Temporal Guard Compiler — public façade
// ---------------------------------------------------------------------------
// Single responsibility: expose the Phase 2 public API and delegate to
// focused sub-modules. No business logic lives here.
// Ref: MIRR-PHASE2-001 §3 (module structure)
// ---------------------------------------------------------------------------

pub mod compiler;
pub mod emit;
pub mod low_level_ir;

use crate::ast::program::Module;
use crate::error::MirrError;

// Re-export the types callers need so they don't have to know sub-module paths.
pub use compiler::{ImplementationStrategy, ResourceEstimate, ResourceEstimator, TemporalCompiler};
pub use low_level_ir::{TemporalNetlist, TemporalNetlistJson};

/// Top-level Phase 2 entry point.
///
/// Owns no state beyond what is needed to drive a single compilation run.
/// Delegates lowering to `TemporalCompiler` and emission to `emit`.
pub struct TemporalGuardCompiler;

impl TemporalGuardCompiler {
    /// Create a new, stateless compiler handle.
    pub fn new() -> Self {
        Self
    }

    /// Lower every `guard` in `module` to a `TemporalNetlist`.
    ///
    /// Each guard is compiled using the adaptive strategy (shift register vs
    /// counter) defined in `compiler::TemporalCompiler`.
    pub fn compile_temporal_guards(
        &mut self,
        module: &Module,
    ) -> Result<TemporalNetlist, MirrError> {
        let mut inner = TemporalCompiler::new();
        inner.compile_module(&module.guards)
    }

    /// Serialize a netlist to pretty-printed JSON.
    pub fn emit_netlist_json(&self, netlist: &TemporalNetlist) -> Result<String, MirrError> {
        emit::emit_json(netlist)
    }

    /// Serialize a netlist to Graphviz DOT format.
    pub fn emit_netlist_dot(&self, netlist: &TemporalNetlist) -> Result<String, MirrError> {
        emit::emit_dot(netlist)
    }

    /// Serialize a netlist to a simple Verilog module string.
    pub fn emit_netlist_verilog(&self, netlist: &TemporalNetlist) -> Result<String, MirrError> {
        emit::emit_verilog(netlist)
    }
}

impl Default for TemporalGuardCompiler {
    fn default() -> Self {
        Self::new()
    }
}