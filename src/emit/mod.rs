//! Phase 6: Output backend module.
//!
//! Re-exports emitters for Graphviz DOT, FIRRTL, SystemVerilog RTL, and JSON netlist.

#![forbid(unsafe_code)]

pub mod dot;
pub mod firrtl;
pub mod json_netlist;
pub mod verilog;
