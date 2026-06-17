#![forbid(unsafe_code)]
#![deny(warnings)]

//! Temporal guard compilation module (Cement2-inspired).
//!
//! Compiles high-level MIRR guards into low-level hardware primitives:
//! shift registers for short delays, counter-comparators for long delays.

pub mod allocator;
pub mod clock_domain;
pub mod compiler;
pub mod emit;
pub mod low_level_ir;
pub mod retiming;

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
    ///
    /// Phase 3 integration: guard conditions are simplified before lowering so
    /// that expressions like `sensor && true` reduce to `sensor`, enabling
    /// correct ConditionKind classification (SimpleSignal) instead of
    /// unnecessary ComplexGuard wrapping.
    pub fn compile_temporal_guards(
        &mut self,
        module: &Module,
    ) -> Result<TemporalNetlist, MirrError> {
        let mut registry = crate::ecs::Registry::new();
        registry.ingest_module(module)?;
        crate::ecs::systems::temporal_synthesis_system(&mut registry)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expr::Expr;
    use crate::ast::program::Guard;
    use crate::ast::types::LiteralValue;

    #[test]
    fn test_temporal_guard_compiler() {
        let mut compiler = TemporalGuardCompiler;
        let guard = Guard {
            name: "g".to_string(),
            condition: Expr::Literal(LiteralValue::Bool(true)),
            cycles: 5,
            template_cycles: None,
            origin: None,
            span: None,
        };
        let module = Module {
            name: "test".to_string(),
            signals: vec![],
            guards: vec![guard],
            reflexes: vec![],
            properties: vec![],
            pattern_calls: vec![],
            pattern_origins: vec![],
            span: None,
        };

        let netlist = compiler.compile_temporal_guards(&module).expect("compile failed");

        // Test format emitters
        let json = compiler.emit_netlist_json(&netlist).expect("json emit failed");
        assert!(json.contains("ShiftRegister"));

        let dot = compiler.emit_netlist_dot(&netlist).expect("dot emit failed");
        assert!(dot.contains("digraph"));

        let verilog = compiler.emit_netlist_verilog(&netlist).expect("verilog emit failed");
        assert!(verilog.contains("module"));
    }
}
