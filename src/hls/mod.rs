//! ARCHITECTURAL SUB-ENGINE: HIGH-LEVEL SYNTHESIS (HLS) OPTIMIZER
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
    /// Map from operation ID to target signal name (traceability).
    pub target_signals: HashMap<u32, String>,
}

use crate::ecs::{EntityId, Registry};
use std::collections::HashMap;

impl OpDag {
    /// Create a new empty DAG.
    pub fn new() -> Self {
        Self { ops: Vec::new(), target_signals: HashMap::new() }
    }

    /// Build an HLS operation DAG from the ECS Registry.
    ///
    /// NASA Power-of-10: bounded by MAX_HLS_OPERATIONS.
    pub fn build_from_registry(registry: &Registry) -> Self {
        let mut dag = Self::new();
        let mut entity_to_op = HashMap::new();

        // Find all reflexes and their assignments.
        for reflex in registry.reflex_comps.iter().flatten() {
            for &assign_id in &reflex.assignments {
                if let Some(assign) = &registry.assignment_comps[assign_id.0 as usize] {
                    if let Some(op_id) =
                        dag.ingest_expr_entity(registry, assign.value, &mut entity_to_op)
                    {
                        if let Some(name_comp) = &registry.names[assign.target.0 as usize] {
                            dag.target_signals.insert(op_id, name_comp.0.clone());
                        }
                    }
                }
            }
        }
        dag
    }

    /// Recursively (iteratively) ingest an expression entity into the DAG.
    fn ingest_expr_entity(
        &mut self,
        registry: &Registry,
        root_id: EntityId,
        memo: &mut HashMap<EntityId, u32>,
    ) -> Option<u32> {
        if let Some(&op_id) = memo.get(&root_id) {
            return Some(op_id);
        }

        let idx = root_id.0 as usize;

        if let Some(binary) = &registry.binary_ops[idx] {
            let left_op = self.ingest_expr_entity(registry, binary.left, memo);
            let right_op = self.ingest_expr_entity(registry, binary.right, memo);

            let kind = match binary.op {
                crate::ast::types::BinaryOp::Add => ResourceKind::Add,
                crate::ast::types::BinaryOp::Sub => ResourceKind::Sub,
                crate::ast::types::BinaryOp::Mul => ResourceKind::Mul,
                crate::ast::types::BinaryOp::And | crate::ast::types::BinaryOp::BitwiseAnd => {
                    ResourceKind::And
                }
                crate::ast::types::BinaryOp::Or | crate::ast::types::BinaryOp::BitwiseOr => {
                    ResourceKind::Or
                }
                crate::ast::types::BinaryOp::Xor => ResourceKind::Xor,
                crate::ast::types::BinaryOp::Eq => ResourceKind::Eq,
                crate::ast::types::BinaryOp::Ne => ResourceKind::Ne,
                crate::ast::types::BinaryOp::Lt => ResourceKind::Lt,
                crate::ast::types::BinaryOp::Le => ResourceKind::Le,
                crate::ast::types::BinaryOp::Gt => ResourceKind::Gt,
                crate::ast::types::BinaryOp::Ge => ResourceKind::Ge,
                crate::ast::types::BinaryOp::Shl => ResourceKind::Shl,
                crate::ast::types::BinaryOp::Shr => ResourceKind::Shr,
            };

            let width = registry.types[idx].as_ref().map(|t| t.0.core.width()).unwrap_or(8);
            let op_id = self.add_op(kind, width, vec![width, width])?;
            memo.insert(root_id, op_id);

            if let Some(l) = left_op {
                self.add_edge(l, op_id);
            }
            if let Some(r) = right_op {
                self.add_edge(r, op_id);
            }
            Some(op_id)
        } else if let Some(unary) = &registry.unary_ops[idx] {
            let operand_op = self.ingest_expr_entity(registry, unary.operand, memo);

            let kind = match unary.op {
                crate::ast::types::UnaryOp::Not => ResourceKind::Not,
                crate::ast::types::UnaryOp::Negate => ResourceKind::Negate,
            };

            let width = registry.types[idx].as_ref().map(|t| t.0.core.width()).unwrap_or(8);
            let op_id = self.add_op(kind, width, vec![width])?;
            memo.insert(root_id, op_id);

            if let Some(o) = operand_op {
                self.add_edge(o, op_id);
            }
            Some(op_id)
        } else {
            // Literals, signals, etc. — leaf nodes in the DAG.
            None
        }
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
    /// Synthesized FIFOs for streaming data across cycles.
    pub fifos: Vec<fifo::FifoHardware>,
    /// Map from operation ID to target signal name.
    pub target_signals: HashMap<u32, String>,
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

    // Step 5: FIFO streaming synthesis.
    let mut fifos = Vec::new();
    if config.fifo {
        // Iterate over DAG edges (successors)
        for src_idx in 0..dag.ops.len() {
            let src_op = &dag.ops[src_idx];
            let src_sched = &schedule_ops[src_idx];

            for &dst_id in &src_op.successors {
                let dst_idx = dst_id as usize;
                let dst_sched = &schedule_ops[dst_idx];

                // If dst starts strictly after src finishes (in terms of ALAP/ASAP cycles), we need a FIFO.
                // Assuming latency difference means cycles.
                if dst_sched.earliest > src_sched.latest {
                    let depth = dst_sched.earliest - src_sched.latest;
                    if let Ok(mut fifo) = fifo::FifoHardware::new(depth, src_op.width) {
                        fifo.name = format!("fifo_edge_{}_to_{}", src_op.op_id, dst_id);
                        fifos.push(fifo);
                    }
                }
            }
        }
    }

    Ok(HlsResult {
        schedule: schedule_ops,
        sharing_groups,
        bindings,
        resource_count,
        fifos,
        target_signals: dag.target_signals.clone(),
    })
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

    #[test]
    fn test_hls_pass_generates_fifo() {
        let mut dag = OpDag::new();
        let a = dag.add_op(ResourceKind::Add, 8, vec![8, 8]).unwrap();
        let b = dag.add_op(ResourceKind::Mul, 16, vec![8, 8]).unwrap();
        dag.add_edge(a, b);

        let config = HlsConfig { latency: 2, ..Default::default() };

        let result = run_hls_pass(&dag, &config).expect("HLS pass failed");

        // `a` is at cycle 0, `b` is at cycle 1.
        // dst starts after src finishes, so a FIFO should be synthesized.
        assert_eq!(result.fifos.len(), 1);
        assert_eq!(result.fifos[0].depth, 1);
        assert_eq!(result.fifos[0].elem_width, 8);
    }
}
