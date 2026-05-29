//! MEGA-12: BOUNDED-HLS — High-Level Synthesis optimizations.
//!
//! Provides compile-time passes for: ASAP/ALAP scheduling, resource sharing
//! (time-multiplex ALUs across cycles), operation binding, and bounded FIFO
//! streaming. All passes operate on the existing guard/reflex DAG — no new
//! language constructs, no runtime loops, no Turing-completeness risk.
//!
//! NASA Power-of-10: all loops bounded by MAX_* constants.

#![forbid(unsafe_code)]

pub mod binding;
pub mod fifo;
pub mod schedule;
pub mod sharing;

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

/// Resource kinds in the HLS dataflow graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// A single operation in the HLS dataflow graph.
#[derive(Debug, Clone)]
pub struct HlsOperation {
    /// Unique operation ID.
    pub op_id: u32,
    /// Resource kind (what hardware this maps to).
    pub kind: ResourceKind,
    /// Bit width of the operation result.
    pub width: u32,
    /// Input operand widths.
    pub operand_widths: Vec<u32>,
    /// IDs of predecessor operations (data dependencies).
    pub predecessors: Vec<u32>,
    /// IDs of successor operations (data consumers).
    pub successors: Vec<u32>,
}

/// HLS dataflow graph for scheduling and sharing.
#[derive(Debug, Clone)]
pub struct OpDag {
    /// Operations in the graph.
    pub ops: Vec<HlsOperation>,
}

impl OpDag {
    /// Create a new empty DAG.
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    /// Add an operation to the DAG.
    /// Returns the assigned op_id, or None if the graph is full.
    pub fn add_op(
        &mut self,
        kind: ResourceKind,
        width: u32,
        operand_widths: Vec<u32>,
    ) -> Option<u32> {
        if self.ops.len() >= MAX_HLS_OPERATIONS {
            return None;
        }
        let op_id = self.ops.len() as u32;
        let op = HlsOperation {
            op_id,
            kind,
            width,
            operand_widths,
            predecessors: Vec::new(),
            successors: Vec::new(),
        };
        self.ops.push(op);
        Some(op_id)
    }

    /// Add a data dependency edge from src to dst.
    pub fn add_edge(&mut self, src: u32, dst: u32) {
        if src < self.ops.len() as u32 && dst < self.ops.len() as u32 {
            if !self.ops[dst as usize].predecessors.contains(&src) {
                self.ops[dst as usize].predecessors.push(src);
            }
            if !self.ops[src as usize].successors.contains(&dst) {
                self.ops[src as usize].successors.push(dst);
            }
        }
    }
}

/// HLS pass result with scheduling, sharing, and binding information.
#[derive(Debug, Clone)]
pub struct HlsResult {
    /// Scheduling result (operation → cycle assignment).
    pub schedule: Vec<schedule::ScheduleOp>,
    /// Resource sharing groups (operations sharing a resource).
    pub sharing_groups: Vec<Vec<usize>>,
    /// Binding result (operation → physical resource assignment).
    pub bindings: Vec<u32>,
    /// Number of physical resources required.
    pub resource_count: Vec<(ResourceKind, u32)>,
}

use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};

/// Run the full HLS pass on an operation DAG.
///
/// Steps:
/// 1. ASAP scheduling (earliest cycle for each operation)
/// 2. ALAP scheduling (latest cycle for each operation)
/// 3. Resource sharing (group compatible operations)
/// 4. Operation binding (assign to physical resources)
///
/// Returns HlsResult with all optimization data.
pub fn run_hls_pass(dag: &OpDag, config: &HlsConfig) -> Result<HlsResult, MirrError> {
    if dag.ops.is_empty() {
        return Err(mirrcode(ErrorCode::HlsSchedulingFailed, "Empty operation DAG"));
    }

    // Step 1: ASAP scheduling.
    let mut schedule_ops = schedule::asap_schedule(dag)?;

    // Step 2: ALAP scheduling (if latency is meaningful).
    if config.latency > 1 {
        let alap = schedule::alap_schedule(dag, config.latency)?;
        // Merge ALAP timing into schedule.
        for (i, asap_op) in schedule_ops.iter_mut().enumerate() {
            if i < alap.len() {
                asap_op.latest = alap[i].latest;
            }
        }
    } else {
        // For single-cycle latency, ALAP = ASAP.
        for op in &mut schedule_ops {
            op.latest = op.earliest;
        }
    }

    // Step 3: Resource sharing.
    let sharing_groups =
        if config.sharing { sharing::find_shareable_ops(&schedule_ops) } else { Vec::new() };

    // Step 4: Operation binding.
    let bindings = if config.binding {
        binding::bind_operations(&schedule_ops)
    } else {
        // Trivial binding: each operation gets its own resource.
        let mut trivial = Vec::with_capacity(schedule_ops.len());
        for i in 0..schedule_ops.len() {
            trivial.push(i as u32);
        }
        trivial
    };

    // Count physical resources.
    let resource_count = count_resources(&bindings, &schedule_ops);

    Ok(HlsResult { schedule: schedule_ops, sharing_groups, bindings, resource_count })
}

/// Count physical resources used by the binding.
fn count_resources(
    bindings: &[u32],
    schedule: &[schedule::ScheduleOp],
) -> Vec<(ResourceKind, u32)> {
    let mut counts: Vec<(ResourceKind, u32)> = Vec::new();

    let mut i = 0;
    while i < schedule.len() {
        let kind = schedule[i].resource;
        let _binding = bindings[i];
        // Count unique binding IDs for this resource kind.
        // Note: binding is a physical resource ID, so we just count the max+1.
        let mut found = false;
        let mut j = 0;
        while j < counts.len() {
            if counts[j].0 == kind {
                found = true;
                break;
            }
            j += 1;
        }
        if !found {
            // Count unique bindings for this kind.
            let mut unique_bindings: Vec<u32> = Vec::new();
            let mut k = 0;
            while k < schedule.len() && k < bindings.len() {
                if schedule[k].resource == kind {
                    let b = bindings[k];
                    if !unique_bindings.contains(&b) {
                        unique_bindings.push(b);
                    }
                }
                k += 1;
            }
            counts.push((kind, unique_bindings.len() as u32));
        }
        i += 1;
    }

    counts
}

impl Default for OpDag {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_dag_new() {
        let dag = OpDag::new();
        assert_eq!(dag.ops.len(), 0);
    }

    #[test]
    fn test_op_dag_add_op() {
        let mut dag = OpDag::new();
        let id = dag.add_op(ResourceKind::Add, 8, vec![8, 8]);
        assert_eq!(id, Some(0));
        assert_eq!(dag.ops.len(), 1);
        assert_eq!(dag.ops[0].kind, ResourceKind::Add);
    }

    #[test]
    fn test_op_dag_add_edge() {
        let mut dag = OpDag::new();
        let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8]).unwrap();
        let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8]).unwrap();
        dag.add_edge(a, b);
        assert!(dag.ops[b as usize].predecessors.contains(&a));
        assert!(dag.ops[a as usize].successors.contains(&b));
    }

    #[test]
    fn test_op_dag_max_operations() {
        let mut dag = OpDag::new();
        for _ in 0..MAX_HLS_OPERATIONS {
            dag.add_op(ResourceKind::Add, 8, vec![8, 8]);
        }
        let result = dag.add_op(ResourceKind::Add, 8, vec![8, 8]);
        assert_eq!(result, None);
    }
}
