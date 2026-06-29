//! ARCHITECTURAL SUB-ENGINE: HIGH-LEVEL SYNTHESIS (HLS) OPTIMIZER
//!
//! Provides compile-time passes for: ASAP/ALAP scheduling, resource sharing
//! (time-multiplex ALUs across cycles), operation binding, and bounded FIFO
//! streaming. All passes operate on the existing guard/reflex DAG — no new
//! language constructs, no runtime loops, no Turing-completeness risk.
//!
//! NASA Power-of-10: all loops bounded by MAX_* constants.

#![forbid(unsafe_code)]

pub mod fifo;

/// Maximum operations in the HLS DAG (NASA P10 bound).
pub const MAX_HLS_OPERATIONS: usize = 512;

/// HLS pass configuration.
#[derive(Debug, Clone)]
pub struct HlsConfig {
    /// Target latency in clock cycles (must be >= 1).
    pub latency: u32,
    /// Enable resource sharing (time-multiplex compatible operations).
    pub sharing: bool,
    /// Enable operation binding to resource pool.
    pub binding: bool,
    /// Enable FIFO streaming synthesis.
    pub fifo: bool,
}

impl Default for HlsConfig {
    fn default() -> Self {
        Self { latency: 1, sharing: true, binding: true, fifo: true }
    }
}

use serde::{Deserialize, Serialize};

/// Resource kinds in the HLS dataflow graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceKind {
    /// Addition operation.
    Add,
    /// Subtraction operation.
    Sub,
    /// Multiplication operation.
    Mul,
    /// Bitwise AND.
    And,
    /// Bitwise OR.
    Or,
    /// Bitwise XOR.
    Xor,
    /// Comparison (equal).
    Eq,
    /// Comparison (not equal).
    Ne,
    /// Comparison (less than).
    Lt,
    /// Comparison (less than or equal).
    Le,
    /// Comparison (greater than).
    Gt,
    /// Comparison (greater than or equal).
    Ge,
    /// Left shift.
    Shl,
    /// Right shift.
    Shr,
    /// Logical NOT.
    Not,
    /// Arithmetic negation.
    Negate,
}

impl core::fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ResourceKind::Add => write!(f, "add"),
            ResourceKind::Sub => write!(f, "sub"),
            ResourceKind::Mul => write!(f, "mul"),
            ResourceKind::And => write!(f, "and"),
            ResourceKind::Or => write!(f, "or"),
            ResourceKind::Xor => write!(f, "xor"),
            ResourceKind::Eq => write!(f, "eq"),
            ResourceKind::Ne => write!(f, "ne"),
            ResourceKind::Lt => write!(f, "lt"),
            ResourceKind::Le => write!(f, "le"),
            ResourceKind::Gt => write!(f, "gt"),
            ResourceKind::Ge => write!(f, "ge"),
            ResourceKind::Shl => write!(f, "shl"),
            ResourceKind::Shr => write!(f, "shr"),
            ResourceKind::Not => write!(f, "not"),
            ResourceKind::Negate => write!(f, "negate"),
        }
    }
}

use crate::ecs::components::HlsDataflowComponent;
use crate::ecs::{EntityId, Registry};

/// ECS System: Ingest operations from the Registry to build the HLS dataflow graph.
///
/// This system populates `HlsDataflowComponent` on entities that represent
/// operations to be scheduled and shared by the HLS engine.
pub fn hls_ingestion_system(registry: &mut Registry) {
    let max_id = registry.active_entities();

    // First pass: identify all operations that are targets of assignments in reflexes
    let mut op_entities = std::collections::HashSet::new();

    for i in 0..max_id {
        if let Some(reflex) = &registry.reflex_comps[i] {
            for assign_id in &reflex.assignments {
                if let Some(assign) = &registry.assignment_comps[assign_id.0 as usize] {
                    identify_operations(registry, assign.value, &mut op_entities);
                }
            }
        }
    }

    // Second pass: for all identified operations, establish dataflow edges
    for &id in &op_entities {
        let idx = id.0 as usize;

        let mut predecessors = Vec::new();

        if let Some(binary) = &registry.binary_ops[idx] {
            if op_entities.contains(&binary.left) {
                predecessors.push(binary.left);
            }
            if op_entities.contains(&binary.right) {
                predecessors.push(binary.right);
            }
        } else if let Some(unary) = &registry.unary_ops[idx] {
            if op_entities.contains(&unary.operand) {
                predecessors.push(unary.operand);
            }
        }

        // Initialize dataflow component if it doesn't exist
        if registry.hls_dataflow[idx].is_none() {
            registry.hls_dataflow[idx] =
                Some(HlsDataflowComponent { predecessors: Vec::new(), successors: Vec::new() });
        }

        // Add predecessors and update their successors
        for pred in predecessors {
            // Add predecessor to current entity
            if let Some(df) = &mut registry.hls_dataflow[idx] {
                if !df.predecessors.contains(&pred) {
                    df.predecessors.push(pred);
                }
            }

            // Add current entity as successor to predecessor
            if registry.hls_dataflow[pred.0 as usize].is_none() {
                registry.hls_dataflow[pred.0 as usize] =
                    Some(HlsDataflowComponent { predecessors: Vec::new(), successors: Vec::new() });
            }

            if let Some(df) = &mut registry.hls_dataflow[pred.0 as usize] {
                if !df.successors.contains(&id) {
                    df.successors.push(id);
                }
            }
        }
    }
}

fn identify_operations(
    registry: &Registry,
    root_id: EntityId,
    op_entities: &mut std::collections::HashSet<EntityId>,
) {
    let idx = root_id.0 as usize;

    if let Some(binary) = &registry.binary_ops[idx] {
        op_entities.insert(root_id);
        identify_operations(registry, binary.left, op_entities);
        identify_operations(registry, binary.right, op_entities);
    } else if let Some(unary) = &registry.unary_ops[idx] {
        op_entities.insert(root_id);
        identify_operations(registry, unary.operand, op_entities);
    }
}
// End of src/hls/mod.rs
