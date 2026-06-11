//! Phase 1a: Linear-scan Register Allocator for R-SPU.
//!
//! Maps MIRR signals to physical R-SPU registers (R0-R1023).
//! Implements 'Elastic Partitions': 유지 organized starting offsets but allows
//! buckets to overflow into the next partition if needed.

#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::ast::program::Module;
use crate::ast::types::SignalKind;
use crate::emit::rspu::rspu_err;
use crate::emit::rspu_isa::*;
use crate::error::MirrError;

/// Result of register allocation.
#[derive(Debug, Clone)]
pub struct RegAllocResult {
    /// Map of signal name -> register index.
    pub map: HashMap<String, RegId>,
    /// Sorted list of signal/register pairs for deterministic assembly.
    pub entries: Vec<(String, RegId)>,
    /// Total registers consumed.
    pub total_used: usize,
    /// Next available temporary register index.
    pub next_temp: u16,
}

impl RegAllocResult {
    /// Allocate a temporary register for intermediate expression results.
    /// Bounded by `MAX_REGISTERS`.
    pub fn alloc_temp(&mut self) -> Option<RegId> {
        if self.next_temp >= MAX_REGISTERS as u16 {
            eprintln!(
                "R-SPU register allocation failed: exhausted all {} registers (at {}) {:?}",
                MAX_REGISTERS, self.next_temp, self.map
            );
            return None;
        }
        let r = self.next_temp as RegId;
        self.next_temp += 1;
        self.total_used += 1;
        Some(r)
    }

    /// Lookup a register ID by signal name.
    pub fn reg(&self, name: &str) -> RegId {
        *self.map.get(name).unwrap_or_else(|| {
            panic!("RegAllocResult::reg: signal '{}' not found in allocation map", name)
        })
    }
}

/// Perform bounded linear-scan register allocation for a MIRR module.
///
/// Iterates once over `module.signals` (bounded by parser limit).
/// Returns `Err(E701)` if the total hardware limit is exceeded.
pub fn allocate_registers(
    module: &Module,
    target: &TargetSpec,
) -> Result<RegAllocResult, MirrError> {
    let mut map = HashMap::with_capacity(module.signals.len());
    let mut entries = Vec::with_capacity(module.signals.len());

    let max_regs = target.max_registers();
    let (input_base, output_base, internal_base, temp_base) = target.partitions();

    // Separate signals by kind to allocate them in blocks (Elastic Partitions)
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut internals = Vec::new();

    for sig in &module.signals {
        match sig.kind {
            SignalKind::Input => inputs.push(sig),
            SignalKind::Output => outputs.push(sig),
            SignalKind::Internal => internals.push(sig),
        }
    }

    let mut cursor: u16 = input_base;

    // Helper closure to allocate a signal (either scalar or flattened array)
    let allocate_signal = |sig: &crate::ast::program::SignalDecl,
                           cursor: &mut u16,
                           map: &mut HashMap<String, RegId>,
                           entries: &mut Vec<(String, RegId)>|
     -> Result<(), MirrError> {
        let size = match &sig.ty.core {
            crate::ast::types::SignalType::Array { length, .. } => *length as u16,
            _ => 1,
        };

        if *cursor as usize + size as usize > max_regs {
            return Err(rspu_err(
                "R-SPU register allocation failed: too many signals (hardware limit exceeded).",
            ));
        }

        if size == 1 {
            let reg = *cursor as RegId;
            map.insert(sig.name.clone(), reg);
            entries.push((sig.name.clone(), reg));
            *cursor += 1;
        } else {
            for i in 0..size {
                let reg = *cursor as RegId;
                let flat_name = format!("{}[{}]", sig.name, i);
                map.insert(flat_name.clone(), reg);
                entries.push((flat_name, reg));
                *cursor += 1;
            }
        }
        Ok(())
    };

    // 1. Allocate Inputs
    for sig in inputs {
        allocate_signal(sig, &mut cursor, &mut map, &mut entries)?;
    }

    // 2. Allocate Outputs (Start at output_base unless inputs overflowed into it)
    cursor = cursor.max(output_base);
    for sig in outputs {
        allocate_signal(sig, &mut cursor, &mut map, &mut entries)?;
    }

    // 3. Allocate Internals (Start at internal_base unless outputs overflowed into it)
    cursor = cursor.max(internal_base);
    for sig in internals {
        allocate_signal(sig, &mut cursor, &mut map, &mut entries)?;
    }

    // 4. Reserve 'true' constant and setup Temporaries
    // Map "false" to R0 (always 0)
    map.insert("false".to_string(), 0);

    // Map "true" to a register initialized to 1.
    // Start at temp_base unless internals stretched into it.
    let true_reg = cursor.max(temp_base);
    if true_reg as usize >= max_regs {
        return Err(rspu_err(
            "R-SPU register allocation failed: no space for 'true' constant register.",
        ));
    }
    map.insert("true".to_string(), true_reg as RegId);

    let total_used = (true_reg as usize + 1).max(cursor as usize);

    Ok(RegAllocResult { map, entries, total_used, next_temp: true_reg + 1 })
}
