//! Bounded iterative DPLL SAT solver.
//!
//! Implements the Davis–Putnam–Logemann–Loveland (DPLL) algorithm using
//! an iterative worklist instead of recursion (NASA Power-of-10 compliance).
//!
//! Bounded by:
//! - `MAX_DECISIONS`: maximum branching decisions before returning Unknown.
//! - `MAX_PROPAGATIONS`: maximum unit propagation steps per decision level.
//!
//! This solver is designed for small formulas (expression equivalence checks
//! during simplification), not general-purpose SAT solving.

#![forbid(unsafe_code)]

use super::cnf::{CnfFormula, Literal};

/// Maximum decision (branching) steps before giving up.
pub const MAX_DECISIONS: usize = 4096;

/// Maximum unit propagation steps per decision level.
pub const MAX_PROPAGATIONS: usize = 8192;

/// Maximum backtrack depth (limits the decision stack).
pub const MAX_BACKTRACK_DEPTH: usize = 512;

/// Result of a SAT check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatResult {
    /// The formula is satisfiable (a satisfying assignment exists).
    Satisfiable,
    /// The formula is unsatisfiable (no assignment satisfies all clauses).
    Unsatisfiable,
    /// The solver could not determine satisfiability within bounds.
    Unknown,
}

/// Assignment state for a variable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VarState {
    Unassigned,
    True,
    False,
}

/// A decision record for backtracking.
#[derive(Debug, Clone)]
struct Decision {
    /// The variable that was decided.
    var: usize,
    /// Whether we've tried the flipped value yet.
    tried_flip: bool,
    /// The number of assignments when this decision was made,
    /// so we can undo assignments on backtrack.
    assignment_count: usize,
}

/// Bounded iterative DPLL SAT solver.
#[derive(Debug)]
pub struct SatSolver {
    /// Current variable assignments.
    assignments: Vec<VarState>,
    /// Decision stack for backtracking.
    decisions: Vec<Decision>,
    /// History of assignments (var, old_state) for undoing.
    trail: Vec<usize>,
}

impl SatSolver {
    /// Create a new solver for a formula with `num_vars` variables.
    pub fn new(num_vars: usize) -> Self {
        Self {
            assignments: vec![VarState::Unassigned; num_vars],
            decisions: Vec::new(),
            trail: Vec::new(),
        }
    }

    /// Solve the given CNF formula.
    pub fn solve(&mut self, formula: &CnfFormula) -> SatResult {
        let mut total_decisions = 0usize;

        loop {
            // Unit propagation.
            match self.propagate(formula) {
                PropResult::Ok => {}
                PropResult::Conflict => {
                    // Backtrack.
                    if !self.backtrack() {
                        return SatResult::Unsatisfiable;
                    }
                    continue;
                }
                PropResult::Exhausted => return SatResult::Unknown,
            }

            // Check if all clauses are satisfied.
            if self.all_satisfied(formula) {
                return SatResult::Satisfiable;
            }

            // Pick next unassigned variable.
            let next_var = match self.pick_variable(formula) {
                Some(v) => v,
                None => {
                    // All variables assigned but not all clauses satisfied —
                    // this means we have a conflict not caught by propagation.
                    if !self.backtrack() {
                        return SatResult::Unsatisfiable;
                    }
                    continue;
                }
            };

            // Make a decision.
            total_decisions += 1;
            if total_decisions > MAX_DECISIONS {
                return SatResult::Unknown;
            }
            if self.decisions.len() >= MAX_BACKTRACK_DEPTH {
                return SatResult::Unknown;
            }

            self.decisions.push(Decision {
                var: next_var,
                tried_flip: false,
                assignment_count: self.trail.len(),
            });
            self.assign(next_var, VarState::True);
        }
    }

    /// Assign a variable and record on the trail.
    fn assign(&mut self, var: usize, state: VarState) {
        self.assignments[var] = state;
        self.trail.push(var);
    }

    /// Unit propagation: find unit clauses and propagate.
    fn propagate(&mut self, formula: &CnfFormula) -> PropResult {
        let mut steps = 0usize;
        let mut changed = true;

        while changed {
            changed = false;
            for clause in &formula.clauses {
                steps += 1;
                if steps > MAX_PROPAGATIONS {
                    return PropResult::Exhausted;
                }

                let mut unsat_count = 0usize;
                let mut unassigned_lit: Option<Literal> = None;
                let mut satisfied = false;

                for &lit in clause {
                    if lit.var >= self.assignments.len() {
                        continue;
                    }
                    match self.assignments[lit.var] {
                        VarState::Unassigned => {
                            unsat_count += 1;
                            unassigned_lit = Some(lit);
                        }
                        VarState::True => {
                            if !lit.negated {
                                satisfied = true;
                                break;
                            }
                            // Variable is true but literal is negated — unsatisfied.
                        }
                        VarState::False => {
                            if lit.negated {
                                satisfied = true;
                                break;
                            }
                            // Variable is false and literal is positive — unsatisfied.
                        }
                    }
                }

                if satisfied {
                    continue;
                }

                if unsat_count == 0 {
                    // All literals falsified — conflict.
                    return PropResult::Conflict;
                }

                if unsat_count == 1 {
                    // Unit clause — propagate.
                    if let Some(lit) = unassigned_lit {
                        let state = if lit.negated { VarState::False } else { VarState::True };
                        self.assign(lit.var, state);
                        changed = true;
                    }
                }
            }
        }

        PropResult::Ok
    }

    /// Backtrack: undo the last decision of try its flip.
    fn backtrack(&mut self) -> bool {
        while let Some(mut decision) = self.decisions.pop() {
            // Undo all assignments made since this decision.
            while self.trail.len() > decision.assignment_count {
                if let Some(var) = self.trail.pop() {
                    self.assignments[var] = VarState::Unassigned;
                }
            }

            if !decision.tried_flip {
                // Try the opposite value.
                decision.tried_flip = true;
                self.decisions.push(decision.clone());
                let flipped = VarState::False; // First try was True, now try False.
                self.assign(decision.var, flipped);
                return true;
            }
            // Already tried both values — continue backtracking.
        }
        false
    }

    /// Check if all clauses are satisfied under the current assignment.
    fn all_satisfied(&self, formula: &CnfFormula) -> bool {
        for clause in &formula.clauses {
            let satisfied = clause.iter().any(|lit| {
                if lit.var >= self.assignments.len() {
                    return false;
                }
                match self.assignments[lit.var] {
                    VarState::True => !lit.negated,
                    VarState::False => lit.negated,
                    VarState::Unassigned => false,
                }
            });
            if !satisfied {
                return false;
            }
        }
        true
    }

    /// Pick the next unassigned variable using a simple heuristic:
    /// choose the variable that appears most often in unsatisfied clauses.
    fn pick_variable(&self, formula: &CnfFormula) -> Option<usize> {
        // Simple: first unassigned variable.
        // A more sophisticated heuristic (VSIDS, etc.) is overkill
        // for the small formulas we handle.
        for (i, state) in self.assignments.iter().enumerate() {
            if *state == VarState::Unassigned {
                // Verify this variable actually appears in the formula.
                let appears = formula.clauses.iter().any(|c| c.iter().any(|lit| lit.var == i));
                if appears {
                    return Some(i);
                }
            }
        }
        None
    }
}

/// Internal propagation result.
enum PropResult {
    Ok,
    Conflict,
    Exhausted,
}

/// Convenience function: solve a CNF formula and return the result.
pub fn solve(formula: &CnfFormula) -> SatResult {
    let mut solver = SatSolver::new(formula.num_vars);
    solver.solve(formula)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sat::cnf::Literal;

    fn make_formula(clauses: Vec<Vec<Literal>>, num_vars: usize) -> CnfFormula {
        CnfFormula { clauses, num_vars, root_var: 0, truncated: false }
    }

    #[test]
    fn empty_formula_is_sat() {
        let f = make_formula(vec![], 0);
        assert_eq!(solve(&f), SatResult::Satisfiable);
    }

    #[test]
    fn single_positive_literal() {
        // (x0)
        let f = make_formula(vec![vec![Literal::pos(0)]], 1);
        assert_eq!(solve(&f), SatResult::Satisfiable);
    }

    #[test]
    fn contradiction_is_unsat() {
        // (x0) AND (NOT x0)
        let f = make_formula(vec![vec![Literal::pos(0)], vec![Literal::neg(0)]], 1);
        assert_eq!(solve(&f), SatResult::Unsatisfiable);
    }

    #[test]
    fn two_variable_sat() {
        // (x0 OR x1) AND (NOT x0 OR x1) AND (x0 OR NOT x1)
        let f = make_formula(
            vec![
                vec![Literal::pos(0), Literal::pos(1)],
                vec![Literal::neg(0), Literal::pos(1)],
                vec![Literal::pos(0), Literal::neg(1)],
            ],
            2,
        );
        assert_eq!(solve(&f), SatResult::Satisfiable);
    }

    #[test]
    fn two_variable_unsat() {
        // All four combinations of (x0, x1) falsified:
        // (x0 OR x1) AND (NOT x0 OR x1) AND (x0 OR NOT x1) AND (NOT x0 OR NOT x1)
        let f = make_formula(
            vec![
                vec![Literal::pos(0), Literal::pos(1)],
                vec![Literal::neg(0), Literal::pos(1)],
                vec![Literal::pos(0), Literal::neg(1)],
                vec![Literal::neg(0), Literal::neg(1)],
            ],
            2,
        );
        assert_eq!(solve(&f), SatResult::Unsatisfiable);
    }

    #[test]
    fn unit_propagation_works() {
        // (x0) AND (NOT x0 OR x1) AND (NOT x1 OR x2) => SAT with x0=T, x1=T, x2=T
        let f = make_formula(
            vec![
                vec![Literal::pos(0)],
                vec![Literal::neg(0), Literal::pos(1)],
                vec![Literal::neg(1), Literal::pos(2)],
            ],
            3,
        );
        assert_eq!(solve(&f), SatResult::Satisfiable);
    }
}
