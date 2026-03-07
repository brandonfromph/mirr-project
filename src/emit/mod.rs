//! Phase 6: Output backend module.
//!
//! Re-exports emitters for Graphviz DOT, SystemVerilog RTL, and JSON netlist.

#![forbid(unsafe_code)]

pub mod dot;
pub mod verilog;
pub mod json_netlist;
