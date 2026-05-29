//! R-SPU bounded linear-scan register allocator.
//!
//! Partitions the R-SPU register file by signal kind:
//!
//! | Partition   | Range     | Maps to                |
//! |-------------|-----------|------------------------|
//! | Input ports | R0–R63    | `SignalKind::Input`    |
//! | Output ports| R64–R127  | `SignalKind::Output`   |
//! | Internals   | R128–R191 | `SignalKind::Internal`  |
//! | Temporaries | R192–R255 | Expression intermediates|
//!
//! Single bounded pass over `module.signals` → `HashMap<String, RegId>`.
//! No graph coloring. `O(n)` where `n ≤ MAX_REGISTERS`.

#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::ast::program::Module;
use crate::ast::types::SignalKind;
use crate::error::MirrError;

use super::rspu::rspu_err;
use super::rspu_isa::*;

/// Result of register allocation: maps signal names to register IDs
/// and tracks per-partition usage.
#[derive(Debug, Clone)]
pub struct RegAllocResult {
    /// Signal name → RegId mapping.
    pub map: HashMap<String, RegId>,
    /// Ordered (name, reg) pairs for program metadata.
    pub entries: Vec<(String, RegId)>,
    /// Total registers used across all partitions.
    pub total_used: usize,
    /// Next available temporary register (u16 to allow overflow past u8::MAX).
    pub next_temp: u16,
}

impl RegAllocResult {
    /// Look up a signal's register. Panics on unknown signal (should be
    /// impossible after validation).
    pub fn reg(&self, name: &str) -> RegId {
        self.map[name]
    }

    /// Allocate the next temporary register. Returns `None` if exhausted.
    pub fn alloc_temp(&mut self) -> Option<RegId> {
        if self.next_temp > REG_TEMP_MAX as u16 {
            println!(
                "DEBUG alloc_temp EXHAUSTED: next_temp={} map={:#?}",
                self.next_temp, self.map
            );
            return None;
        }
        let r = self.next_temp as RegId;
        self.next_temp += 1;
        self.total_used += 1;
        Some(r)
    }
}

/// Perform bounded linear-scan register allocation for a MIRR module.
///
/// Iterates once over `module.signals` (bounded by parser limit).
/// Returns `Err(E701)` if any partition overflows.
pub fn allocate_registers(module: &Module) -> Result<RegAllocResult, MirrError> {
    let mut map = HashMap::with_capacity(module.signals.len());
    let mut entries = Vec::with_capacity(module.signals.len());

    let mut next_input: RegId = REG_INPUT_BASE;
    let mut next_output: RegId = REG_OUTPUT_BASE;
    let mut next_internal: RegId = REG_INTERNAL_BASE;

    for sig in &module.signals {
        let reg = match sig.kind {
            SignalKind::Input => {
                if next_input > REG_INPUT_MAX {
                    return Err(rspu_err(format!(
                        "{} R-SPU register allocation failed: too many input signals \
                         ({} > {}).",
                        crate::error_codes::ec(701),
                        (next_input as usize - REG_INPUT_BASE as usize) + 1,
                        (REG_INPUT_MAX as usize - REG_INPUT_BASE as usize) + 1,
                    )));
                }
                let r = next_input;
                next_input = next_input.saturating_add(1);
                r
            }
            SignalKind::Output => {
                if next_output > REG_OUTPUT_MAX {
                    return Err(rspu_err(format!(
                        "{} R-SPU register allocation failed: too many output signals \
                         ({} > {}).",
                        crate::error_codes::ec(701),
                        (next_output as usize - REG_OUTPUT_BASE as usize) + 1,
                        (REG_OUTPUT_MAX as usize - REG_OUTPUT_BASE as usize) + 1,
                    )));
                }
                let r = next_output;
                next_output = next_output.saturating_add(1);
                r
            }
            SignalKind::Internal => {
                if next_internal > REG_INTERNAL_MAX {
                    return Err(rspu_err(format!(
                        "{} R-SPU register allocation failed: too many internal signals \
                         ({} > {}).",
                        crate::error_codes::ec(701),
                        (next_internal as usize - REG_INTERNAL_BASE as usize) + 1,
                        (REG_INTERNAL_MAX as usize - REG_INTERNAL_BASE as usize) + 1,
                    )));
                }
                let r = next_internal;
                next_internal = next_internal.saturating_add(1);
                r
            }
        };
        map.insert(sig.name.clone(), reg);
        entries.push((sig.name.clone(), reg));
    }

    // Add constant-0 and constant-1 registers for guard logic.
    // Map "true" to a temporary register initialized to 1.
    // Map "false" to R0 (which is always 0 in the simulation model).
    map.insert("false".to_string(), 0);
    map.insert("true".to_string(), REG_TEMP_BASE); // We'll reserve the first temp for 'true'

    let total_used = (next_input - REG_INPUT_BASE) as usize
        + (next_output - REG_OUTPUT_BASE) as usize
        + (next_internal - REG_INTERNAL_BASE) as usize
        + 1; // +1 for the 'true' register

    Ok(RegAllocResult { map, entries, total_used, next_temp: (REG_TEMP_BASE + 1) as u16 })
}
