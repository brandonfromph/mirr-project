//! Phase 6: Output backend module.
//!
//! Re-exports emitters for Graphviz DOT, FIRRTL, SystemVerilog RTL, JSON netlist,
//! and R-SPU assembly.

#![forbid(unsafe_code)]

pub mod dot;
pub mod firrtl;
pub mod json_netlist;
pub mod rspu;
pub mod rspu_isa;
pub mod rspu_regalloc;
pub mod verilog;
