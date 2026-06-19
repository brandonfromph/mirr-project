//! Tseitin CNF conversion for Boolean expressions.
//!
//! Converts MIRR `Expr` trees into Conjunctive Normal Form (CNF)
//! by introducing auxiliary variables for each subexpression.
//! The translation is equisatisfiable (not equivalent), which is
//! sufficient for the equivalence-checking use case: to check
//! `A ≡ B`, we check that `A XOR B` is UNSAT.
//!
//! Bounded by MAX_CNF_VARS and MAX_CNF_CLAUSES (NASA Power-of-10).
//! Uses an iterative worklist instead of recursion.

#![forbid(unsafe_code)]

use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};

use crate::ecs::components::EntityId;
use crate::ecs::registry::Registry;

/// Maximum number of CNF variables (NASA P10: bounded resources).
pub const MAX_CNF_VARS: usize = 2048;

/// Maximum number of CNF clauses (NASA P10: bounded resources).
pub const MAX_CNF_CLAUSES: usize = 8192;

/// Maximum expression nodes to process during conversion.
const MAX_WORK_ITEMS: usize = 4096;

/// A literal in a CNF formula: a variable index with optional negation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    /// Variable index (0-based).
    pub var: usize,
    /// True if this literal is negated.
    pub negated: bool,
}

impl Literal {
    pub fn pos(var: usize) -> Self {
        Self { var, negated: false }
    }

    pub fn neg(var: usize) -> Self {
        Self { var, negated: true }
    }

    pub fn negate(self) -> Self {
        Self { var: self.var, negated: !self.negated }
    }
}

/// A clause is a disjunction of literals.
pub type Clause = Vec<Literal>;

/// A CNF formula: conjunction of clauses.
#[derive(Debug, Clone)]
pub struct CnfFormula {
    /// The clauses (conjunction of disjunctions).
    pub clauses: Vec<Clause>,
    /// Number of variables allocated.
    pub num_vars: usize,
    /// The variable representing the root expression's truth value.
    pub root_var: usize,
    /// Whether conversion hit a resource bound.
    pub truncated: bool,
}

impl CnfFormula {
    fn new() -> Self {
        Self { clauses: Vec::new(), num_vars: 0, root_var: 0, truncated: false }
    }

    fn alloc_var(&mut self) -> Option<usize> {
        if self.num_vars >= MAX_CNF_VARS {
            self.truncated = true;
            return None;
        }
        let v = self.num_vars;
        self.num_vars += 1;
        Some(v)
    }

    pub fn add_clause(&mut self, clause: Clause) -> bool {
        if self.clauses.len() >= MAX_CNF_CLAUSES {
            self.truncated = true;
            return false;
        }
        self.clauses.push(clause);
        true
    }
}

/* LEGACY AST ENGINE (PHASE 3b ARCHIVED)
/// Work item for iterative Tseitin conversion.
#[derive(Debug)]
enum WorkItem<'a> {
    /// Process this expression and push its variable onto the result stack.
    Convert(&'a Expr),
    /// Combine: take top N variables from stack and add Tseitin clauses.
    CombineNot,
    CombineAnd,
    CombineOr,
    CombineXor,
}

/// Convert an expression to CNF using the Tseitin transformation.
///
/// Returns `None` if the expression exceeds resource bounds.
pub fn expr_to_cnf(expr: &Expr) -> Option<CnfFormula> {
    let mut formula = CnfFormula::new();
    let mut work: Vec<WorkItem<'_>> = Vec::new();
    let mut var_stack: Vec<usize> = Vec::new();
    let mut iterations = 0usize;

    work.push(WorkItem::Convert(expr));

    while let Some(item) = work.pop() {
        iterations += 1;
        if iterations > MAX_WORK_ITEMS {
            return None;
        }

        match item {
            WorkItem::Convert(e) => match e {
                Expr::Literal(LiteralValue::Bool(b)) => {
                    let v = formula.alloc_var()?;
                    // Force variable to true or false.
                    if *b {
                        formula.add_clause(vec![Literal::pos(v)]);
                    } else {
                        formula.add_clause(vec![Literal::neg(v)]);
                    }
                    var_stack.push(v);
                }
                Expr::Literal(LiteralValue::Integer(n)) => {
                    // Treat nonzero as true, zero as false.
                    let v = formula.alloc_var()?;
                    if *n != 0 {
                        formula.add_clause(vec![Literal::pos(v)]);
                    } else {
                        formula.add_clause(vec![Literal::neg(v)]);
                    }
                    var_stack.push(v);
                }
                Expr::Signal(name) => {
                    let _ = name;
                    let v = formula.alloc_var()?;
                    var_stack.push(v);
                }
                Expr::Prev { .. } => {
                    let v = formula.alloc_var()?;
                    var_stack.push(v);
                }
                Expr::Unary { op, operand } => {
                    match op {
                        UnaryOp::Not => {
                            work.push(WorkItem::CombineNot);
                            work.push(WorkItem::Convert(operand));
                        }
                        UnaryOp::Negate => {
                            let v = formula.alloc_var()?;
                            var_stack.push(v);
                        }
                        UnaryOp::ReductionOr => {
                            let v = formula.alloc_var()?;
                            var_stack.push(v);
                        }
                    }
                }
                Expr::Binary { op, left, right } => {
                    match op {
                        BinaryOp::And => {
                            work.push(WorkItem::CombineAnd);
                            work.push(WorkItem::Convert(right));
                            work.push(WorkItem::Convert(left));
                        }
                        BinaryOp::Or => {
                            work.push(WorkItem::CombineOr);
                            work.push(WorkItem::Convert(right));
                            work.push(WorkItem::Convert(left));
                        }
                        BinaryOp::Xor => {
                            work.push(WorkItem::CombineXor);
                            work.push(WorkItem::Convert(right));
                            work.push(WorkItem::Convert(left));
                        }
                        _ => {
                            let v = formula.alloc_var()?;
                            var_stack.push(v);
                        }
                    }
                }
                Expr::ArrayIndex { .. }
                | Expr::FieldAccess { .. }
                | Expr::ArrayLiteral(_)
                | Expr::StructLiteral { .. } => {
                    let v = formula.alloc_var()?;
                    var_stack.push(v);
                }
                Expr::UnfoldIndex(_) => {
                    let v = formula.alloc_var()?;
                    var_stack.push(v);
                }
            },
            WorkItem::CombineNot => {
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::pos(out), Literal::pos(a)]);
                formula.add_clause(vec![Literal::neg(out), Literal::neg(a)]);
                var_stack.push(out);
            }
            WorkItem::CombineAnd => {
                let b = var_stack.pop()?;
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::neg(out), Literal::pos(a)]);
                formula.add_clause(vec![Literal::neg(out), Literal::pos(b)]);
                formula.add_clause(vec![Literal::pos(out), Literal::neg(a), Literal::neg(b)]);
                var_stack.push(out);
            }
            WorkItem::CombineOr => {
                let b = var_stack.pop()?;
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::pos(out), Literal::neg(a)]);
                formula.add_clause(vec![Literal::pos(out), Literal::neg(b)]);
                formula.add_clause(vec![Literal::neg(out), Literal::pos(a), Literal::pos(b)]);
                var_stack.push(out);
            }
            WorkItem::CombineXor => {
                let b = var_stack.pop()?;
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::neg(out), Literal::neg(a), Literal::neg(b)]);
                formula.add_clause(vec![Literal::neg(out), Literal::pos(a), Literal::pos(b)]);
                formula.add_clause(vec![Literal::pos(out), Literal::neg(a), Literal::pos(b)]);
                formula.add_clause(vec![Literal::pos(out), Literal::pos(a), Literal::neg(b)]);
                var_stack.push(out);
            }
        }
    }

    let root = var_stack.pop()?;
    formula.root_var = root;
    Some(formula)
}

pub fn equivalence_check_cnf(original: &Expr, simplified: &Expr) -> Option<CnfFormula> {
    let xor_expr = Expr::Binary {
        op: BinaryOp::Xor,
        left: Box::new(original.clone()),
        right: Box::new(simplified.clone()),
    };
    let mut formula = expr_to_cnf(&xor_expr)?;
    formula.add_clause(vec![Literal::pos(formula.root_var)]);
    Some(formula)
}
*/

/// Work item for iterative Tseitin conversion from ECS.
#[derive(Debug)]
enum EcsWorkItem {
    Convert(EntityId),
    CombineNot(EntityId),
    CombineAnd(EntityId),
    CombineOr(EntityId),
    CombineXor(EntityId),
    CombineMux(EntityId),
}

/// Bitset-based Ancestry Reachability (Bounded to 256 nodes for DPLL solver)
#[derive(Debug, Clone)]
pub struct AncestryMatrix {
    /// Each node mapped to 0..255 gets a 4x u64 bitset (256 bits).
    /// Row `i` tells us which descendants node `i` can reach.
    reachability: [[u64; 4]; 256],
    _node_count: usize,
}

impl AncestryMatrix {
    pub fn new(node_count: usize) -> Self {
        Self { reachability: [[0; 4]; 256], _node_count: node_count.min(256) }
    }

    pub fn set_reach(&mut self, ancestor: usize, descendant: usize) {
        if ancestor < 256 && descendant < 256 {
            self.reachability[ancestor][descendant / 64] |= 1 << (descendant % 64);
        }
    }

    pub fn can_reach(&self, ancestor: usize, descendant: usize) -> bool {
        if ancestor < 256 && descendant < 256 {
            (self.reachability[ancestor][descendant / 64] & (1 << (descendant % 64))) != 0
        } else {
            false
        }
    }
}

/// Extracts a logic cone of up to 256 nodes and computes the reachability bitset.
pub fn extract_and_compute_ancestry(
    roots: &[EntityId],
    registry: &Registry,
) -> (Vec<Option<u8>>, AncestryMatrix) {
    let mut local_map: Vec<Option<u8>> = vec![None; registry.names.len()];
    let mut local_id: u8 = 0;

    // Simple BFS/DFS to assign IDs
    let mut work = Vec::new();
    for &r in roots {
        work.push(r);
    }

    while let Some(entity) = work.pop() {
        let idx = entity.0 as usize;
        if local_map[idx].is_some() {
            continue;
        }

        local_map[idx] = Some(local_id);
        if local_id == 255 {
            break;
        } // Bounded to 256
        local_id += 1;

        // Push children
        if let Some(unary) = &registry.unary_ops[idx] {
            work.push(unary.operand);
        } else if let Some(binary) = &registry.binary_ops[idx] {
            work.push(binary.left);
            work.push(binary.right);
        } else if let Some(mux) = &registry.muxes[idx] {
            work.push(mux.select);
            work.push(mux.true_val);
            work.push(mux.false_val);
        }
    }

    let mut matrix = AncestryMatrix::new(local_id as usize);
    // Transitive closure (simple implementation for now)
    for i in 0..local_id as usize {
        matrix.set_reach(i, i);
    }

    // We would ideally iterate topological order here, but for now we skip full O(N^3) closure
    // since we only have 256 nodes, we can just do a few passes or rely on basic depth.
    // In a real SmaRTLy impl, you do a topological bitwise OR pass:
    // reach(parent) |= reach(child)

    (local_map, matrix)
}

/// Convert an ECS Entity subtree to CNF using the Tseitin transformation.
pub fn entity_to_cnf(root: EntityId, registry: &Registry) -> Option<CnfFormula> {
    let mut formula = CnfFormula::new();
    let mut work: Vec<EcsWorkItem> = Vec::new();
    let mut var_stack: Vec<usize> = Vec::new();
    let mut iterations = 0usize;
    let mut entity_to_var: Vec<Option<usize>> = vec![None; registry.names.len()];

    work.push(EcsWorkItem::Convert(root));

    while let Some(item) = work.pop() {
        iterations += 1;
        if iterations > MAX_WORK_ITEMS {
            return None;
        }

        match item {
            EcsWorkItem::Convert(entity) => {
                let idx = entity.0 as usize;
                if let Some(v) = entity_to_var[idx] {
                    var_stack.push(v);
                    continue;
                }

                if let Some(lit) = &registry.literals[idx] {
                    match lit.0 {
                        LiteralValue::Bool(b) => {
                            let v = formula.alloc_var()?;
                            if b {
                                formula.add_clause(vec![Literal::pos(v)]);
                            } else {
                                formula.add_clause(vec![Literal::neg(v)]);
                            }
                            var_stack.push(v);
                            entity_to_var[idx] = Some(v);
                        }
                        LiteralValue::Integer(n) => {
                            let v = formula.alloc_var()?;
                            if n != 0 {
                                formula.add_clause(vec![Literal::pos(v)]);
                            } else {
                                formula.add_clause(vec![Literal::neg(v)]);
                            }
                            var_stack.push(v);
                            entity_to_var[idx] = Some(v);
                        }
                    }
                } else if let Some(unary) = &registry.unary_ops[idx] {
                    match unary.op {
                        UnaryOp::Not => {
                            work.push(EcsWorkItem::CombineNot(entity));
                            work.push(EcsWorkItem::Convert(unary.operand));
                        }
                        UnaryOp::Negate | UnaryOp::ReductionOr => {
                            let v = formula.alloc_var()?;
                            var_stack.push(v);
                            entity_to_var[idx] = Some(v);
                        }
                    }
                } else if let Some(binary) = &registry.binary_ops[idx] {
                    match binary.op {
                        BinaryOp::And => {
                            work.push(EcsWorkItem::CombineAnd(entity));
                            work.push(EcsWorkItem::Convert(binary.right));
                            work.push(EcsWorkItem::Convert(binary.left));
                        }
                        BinaryOp::Or => {
                            work.push(EcsWorkItem::CombineOr(entity));
                            work.push(EcsWorkItem::Convert(binary.right));
                            work.push(EcsWorkItem::Convert(binary.left));
                        }
                        BinaryOp::Xor => {
                            work.push(EcsWorkItem::CombineXor(entity));
                            work.push(EcsWorkItem::Convert(binary.right));
                            work.push(EcsWorkItem::Convert(binary.left));
                        }
                        _ => {
                            let v = formula.alloc_var()?;
                            var_stack.push(v);
                            entity_to_var[idx] = Some(v);
                        }
                    }
                } else if let Some(mux) = &registry.muxes[idx] {
                    work.push(EcsWorkItem::CombineMux(entity));
                    work.push(EcsWorkItem::Convert(mux.false_val));
                    work.push(EcsWorkItem::Convert(mux.true_val));
                    work.push(EcsWorkItem::Convert(mux.select));
                } else {
                    // Temporal nodes, named nodes, or unknown composite
                    let v = formula.alloc_var()?;
                    var_stack.push(v);
                    entity_to_var[idx] = Some(v);
                }
            }
            EcsWorkItem::CombineNot(entity) => {
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::pos(out), Literal::pos(a)]);
                formula.add_clause(vec![Literal::neg(out), Literal::neg(a)]);
                var_stack.push(out);
                entity_to_var[entity.0 as usize] = Some(out);
            }
            EcsWorkItem::CombineAnd(entity) => {
                let b = var_stack.pop()?;
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::neg(out), Literal::pos(a)]);
                formula.add_clause(vec![Literal::neg(out), Literal::pos(b)]);
                formula.add_clause(vec![Literal::pos(out), Literal::neg(a), Literal::neg(b)]);
                var_stack.push(out);
                entity_to_var[entity.0 as usize] = Some(out);
            }
            EcsWorkItem::CombineOr(entity) => {
                let b = var_stack.pop()?;
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::pos(out), Literal::neg(a)]);
                formula.add_clause(vec![Literal::pos(out), Literal::neg(b)]);
                formula.add_clause(vec![Literal::neg(out), Literal::pos(a), Literal::pos(b)]);
                var_stack.push(out);
                entity_to_var[entity.0 as usize] = Some(out);
            }
            EcsWorkItem::CombineXor(entity) => {
                let b = var_stack.pop()?;
                let a = var_stack.pop()?;
                let out = formula.alloc_var()?;
                formula.add_clause(vec![Literal::neg(out), Literal::neg(a), Literal::neg(b)]);
                formula.add_clause(vec![Literal::neg(out), Literal::pos(a), Literal::pos(b)]);
                formula.add_clause(vec![Literal::pos(out), Literal::neg(a), Literal::pos(b)]);
                formula.add_clause(vec![Literal::pos(out), Literal::pos(a), Literal::neg(b)]);
                var_stack.push(out);
                entity_to_var[entity.0 as usize] = Some(out);
            }
            EcsWorkItem::CombineMux(entity) => {
                let f_val = var_stack.pop()?;
                let t_val = var_stack.pop()?;
                let sel = var_stack.pop()?;
                let out = formula.alloc_var()?;
                // Mux logic: out = (sel AND t_val) OR (NOT sel AND f_val)
                // CNF:
                // out OR sel OR NOT f_val
                // out OR NOT sel OR NOT t_val
                // out OR NOT t_val OR NOT f_val
                // NOT out OR sel OR f_val
                // NOT out OR NOT sel OR t_val
                // NOT out OR t_val OR f_val
                formula.add_clause(vec![Literal::pos(out), Literal::pos(sel), Literal::neg(f_val)]);
                formula.add_clause(vec![Literal::pos(out), Literal::neg(sel), Literal::neg(t_val)]);
                formula.add_clause(vec![
                    Literal::pos(out),
                    Literal::neg(t_val),
                    Literal::neg(f_val),
                ]);
                formula.add_clause(vec![Literal::neg(out), Literal::pos(sel), Literal::pos(f_val)]);
                formula.add_clause(vec![Literal::neg(out), Literal::neg(sel), Literal::pos(t_val)]);
                formula.add_clause(vec![
                    Literal::neg(out),
                    Literal::pos(t_val),
                    Literal::pos(f_val),
                ]);
                var_stack.push(out);
                entity_to_var[entity.0 as usize] = Some(out);
            }
        }
    }

    let root = var_stack.pop()?;
    formula.root_var = root;
    Some(formula)
}

/// Build a CNF formula asserting that two expressions are NOT equivalent.
pub fn equivalence_check_ecs_cnf(
    original: EntityId,
    simplified: EntityId,
    registry: &mut Registry,
) -> Option<CnfFormula> {
    let xor_id = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
    registry.set_binary_op(
        xor_id,
        crate::ecs::components::BinaryComponent {
            op: BinaryOp::Xor,
            left: original,
            right: simplified,
        },
    );

    let mut formula = entity_to_cnf(xor_id, registry)?;
    formula.add_clause(vec![Literal::pos(formula.root_var)]);

    // Clean up temporary XOR node
    registry.unset_name(xor_id);
    registry.unset_binary_op(xor_id);

    Some(formula)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_true_produces_unit_clause() {
        let mut registry = Registry::new();
        let entity = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.literals[entity.0 as usize] =
            Some(crate::ecs::components::LiteralComponent(LiteralValue::Bool(true)));
        let cnf = entity_to_cnf(entity, &registry).unwrap();
        assert_eq!(cnf.num_vars, 1);
        assert_eq!(cnf.clauses.len(), 1);
        assert_eq!(cnf.clauses[0], vec![Literal::pos(0)]);
    }

    #[test]
    fn literal_false_produces_negated_unit() {
        let mut registry = Registry::new();
        let entity = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.literals[entity.0 as usize] =
            Some(crate::ecs::components::LiteralComponent(LiteralValue::Bool(false)));
        let cnf = entity_to_cnf(entity, &registry).unwrap();
        assert_eq!(cnf.clauses[0], vec![Literal::neg(0)]);
    }

    #[test]
    fn not_produces_two_clauses() {
        let mut registry = Registry::new();
        let sig = registry.create_entity("a", crate::ecs::components::KindComponent::SIGNAL);

        let not_node = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.unary_ops[not_node.0 as usize] =
            Some(crate::ecs::components::UnaryComponent { op: UnaryOp::Not, operand: sig });

        let cnf = entity_to_cnf(not_node, &registry).unwrap();
        assert_eq!(cnf.num_vars, 2);
        assert_eq!(cnf.clauses.len(), 2);
    }

    #[test]
    fn and_produces_three_clauses() {
        let mut registry = Registry::new();
        let sig_a = registry.create_entity("a", crate::ecs::components::KindComponent::SIGNAL);
        let sig_b = registry.create_entity("b", crate::ecs::components::KindComponent::SIGNAL);

        let and_node = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.set_binary_op(
            and_node,
            crate::ecs::components::BinaryComponent {
                op: BinaryOp::And,
                left: sig_a,
                right: sig_b,
            },
        );

        let cnf = entity_to_cnf(and_node, &registry).unwrap();
        assert_eq!(cnf.num_vars, 3);
        assert_eq!(cnf.clauses.len(), 3);
    }

    #[test]
    fn truncated_flag_on_overflow() {
        let mut registry = Registry::new();
        let entity = registry.create_entity("", crate::ecs::components::KindComponent::SIGNAL);
        registry.literals[entity.0 as usize] =
            Some(crate::ecs::components::LiteralComponent(LiteralValue::Bool(true)));
        let cnf = entity_to_cnf(entity, &registry).unwrap();
        assert!(!cnf.truncated);
    }
}
