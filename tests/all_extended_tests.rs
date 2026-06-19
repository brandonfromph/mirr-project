#![cfg(any())]
#![forbid(unsafe_code)]
//! Master linkage file for extended test suites partitioned during the Dark Age (March 17-22).
//! This file re-registers the entire fragmented test suite to restore Golden Era verification levels.

#[cfg(test)]
mod emit_verilog_extended_tests;

#[cfg(test)]
mod mape_k_bridge_tests;

#[cfg(test)]
mod mega2_sexpr_verification_tests;

#[cfg(test)]
mod mega3_rspu_verification_tests;

#[cfg(test)]
mod mega4_totality_verification_tests;

#[cfg(test)]
mod parser_module_extended_tests;

#[cfg(test)]
mod pattern_tests;

#[cfg(test)]
mod rspu_encoding_extended_tests;

#[cfg(test)]
mod rspu_sim_extended_tests;

#[cfg(test)]
mod sexpr_convert_tests;
