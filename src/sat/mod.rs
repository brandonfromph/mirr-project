//! ARCHITECTURAL SUB-ENGINE: SAT LOGIC SOLVER
//!
//! Provides a bounded iterative DPLL solver and Tseitin CNF conversion
//! for proving expression equivalences during simplification. When the
//! heuristic simplifier in `simplify.rs` produces a candidate, the SAT
//! checker can verify equivalence by testing whether (original XOR simplified)
//! is unsatisfiable.
//!
//! All algorithms are bounded (NASA Power-of-10):
//! - CNF conversion: MAX_CNF_VARS variables, MAX_CNF_CLAUSES clauses
//! - DPLL solver: MAX_DECISIONS decision steps

#![forbid(unsafe_code)]

pub mod cnf;
pub mod simplify_sat;
pub mod solver;

pub use cnf::CnfFormula;
pub use simplify_sat::simplify_entity_with_sat;
pub use solver::{SatResult, SatSolver};
