// ---------------------------------------------------------------------------
//! Symbolic evaluation engine for abstract interpretation of MIRR signals.
//!
//! Implements interval-based abstract interpretation (Cousot & Cousot 1977)
//! applied to MIRR's signal expressions.  All algorithms are bounded by
//! explicit `MAX_*` constants (NASA Power-of-10).
//!
//! ## Error codes: E10xx (symbolic analysis)
//!
//! | Code  | Meaning                                      |
//! |-------|----------------------------------------------|
//! | E1001 | Signal value exceeds declared width bounds    |
//! | E1003 | Signal count exceeds symbolic analysis limit  |
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

pub mod diff;
pub mod interval;
pub mod pattern;

use crate::ast::expr::Expr;
use crate::ast::types::{BinaryOp, LiteralValue, UnaryOp};
use crate::error::MirrError;

// ── NASA Power-of-10 bounds ────────────────────────────────────────────────

/// Maximum number of signals the symbolic engine will analyze in one module.
pub const MAX_SYM_SIGNALS: usize = 4096;

/// Maximum fixpoint iterations before the engine gives up and widens to Top.
pub const MAX_SYM_ITERATIONS: usize = 64;

/// Maximum expression tree depth (work-stack bound for iterative evaluation).
pub const MAX_SYM_DEPTH: usize = 32;

// ── Abstract domain ────────────────────────────────────────────────────────

/// Abstract value in the interval lattice.
///
/// Lattice ordering:  `Concrete ⊏ Interval ⊏ Unknown ⊏ Top`.
///
/// * `Concrete(v)`       — exactly the value `v`.
/// * `Interval { lo, hi }` — any unsigned value in `[lo, hi]` (inclusive).
/// * `Unknown { width }`  — any value representable in `width` bits.
/// * `Top`                — any value whatsoever (no information).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymValue {
    /// Exactly one known value.
    Concrete(u64),
    /// Any value in `[lo, hi]` (unsigned, inclusive).
    Interval { lo: u64, hi: u64 },
    /// Any value of the given bit-width.
    Unknown { width: u32 },
    /// Any value whatsoever (top of the lattice).
    Top,
}

// ── Symbolic state ─────────────────────────────────────────────────────────

/// Maps signal names to their current abstract values.
pub struct SymState {
    pub signals: Vec<(String, SymValue)>,
}

impl Default for SymState {
    fn default() -> Self {
        Self::new()
    }
}

impl SymState {
    /// Create an empty symbolic state (no signals bound).
    pub fn new() -> Self {
        Self { signals: Vec::new() }
    }

    /// Look up the abstract value for `name`.  Returns `Top` if not found.
    ///
    /// Bounded by `MAX_SYM_SIGNALS` (NASA Power-of-10).
    pub fn lookup(&self, name: &str) -> SymValue {
        let limit = self.signals.len().min(MAX_SYM_SIGNALS);
        for i in 0..limit {
            if self.signals[i].0 == name {
                return self.signals[i].1;
            }
        }
        SymValue::Top
    }
}

// ── Analysis results ───────────────────────────────────────────────────────

/// Result of symbolically analyzing an entire module.
#[derive(Debug, Clone)]
pub struct SymbolicResult {
    /// Final abstract values for every assigned signal.
    pub intervals: Vec<(String, SymValue)>,
    /// Width-bound violations detected during the analysis.
    pub violations: Vec<SymbolicViolation>,
    /// Number of fixpoint iterations executed.
    pub iterations: usize,
    /// Whether the analysis reached a fixpoint within the iteration budget.
    pub converged: bool,
}

/// A single width-bound violation detected by the symbolic engine.
#[derive(Debug, Clone)]
pub struct SymbolicViolation {
    /// Name of the signal whose abstract value exceeded its declared width.
    pub signal: String,
    /// The expected (width-legal) abstract range.
    pub expected: SymValue,
    /// The actually computed abstract value.
    pub actual: SymValue,
    /// Human-readable diagnostic message (includes error code).
    pub message: String,
}

// ── Expression evaluation (iterative, no recursion) ────────────────────────

/// Symbolically evaluate an expression tree against a given state.
///
/// Uses an explicit work stack (post-order traversal) bounded by
/// `MAX_SYM_DEPTH * 3` iterations.  Returns `Top` if the expression
/// exceeds the depth budget.
pub fn sym_eval_expr(expr: &Expr, state: &SymState) -> SymValue {
    // Work items for the explicit evaluation stack.
    enum Work<'a> {
        Eval(&'a Expr),
        ApplyUnary(UnaryOp),
        ApplyBinary(BinaryOp),
    }

    let mut work: Vec<Work<'_>> = Vec::with_capacity(MAX_SYM_DEPTH);
    let mut values: Vec<SymValue> = Vec::with_capacity(MAX_SYM_DEPTH);

    work.push(Work::Eval(expr));

    // Bounded: each Expr node pushes at most 3 items (Binary case).
    let max_iters = MAX_SYM_DEPTH * 3;
    let mut iter_count: usize = 0;

    while let Some(item) = work.pop() {
        iter_count += 1;
        if iter_count > max_iters {
            return SymValue::Top;
        }

        match item {
            Work::Eval(e) => match e {
                Expr::Literal(lit) => {
                    let v = match lit {
                        LiteralValue::Bool(b) => SymValue::Concrete(u64::from(*b)),
                        LiteralValue::Integer(n) => SymValue::Concrete(*n),
                    };
                    values.push(v);
                }
                Expr::Signal(name) => {
                    values.push(state.lookup(name));
                }
                Expr::Prev { .. } => {
                    // Conservative: previous-tick values are runtime-dependent.
                    values.push(SymValue::Top);
                }
                Expr::Unary { op, operand } => {
                    work.push(Work::ApplyUnary(*op));
                    work.push(Work::Eval(operand));
                }
                Expr::Binary { op, left, right } => {
                    // Push order (LIFO): left evaluated first, then right,
                    // then ApplyBinary consumes both from the value stack.
                    work.push(Work::ApplyBinary(*op));
                    work.push(Work::Eval(right));
                    work.push(Work::Eval(left));
                }
            },
            Work::ApplyUnary(op) => {
                let val = values.pop().unwrap_or(SymValue::Top);
                values.push(sym_eval_unary(op, val));
            }
            Work::ApplyBinary(op) => {
                // Left was evaluated first (pushed first onto values), right second.
                // Pop order: right (top), then left.
                let rhs = values.pop().unwrap_or(SymValue::Top);
                let lhs = values.pop().unwrap_or(SymValue::Top);
                values.push(sym_eval_binary(op, lhs, rhs));
            }
        }
    }

    values.pop().unwrap_or(SymValue::Top)
}

// ── Binary abstract transfer function ──────────────────────────────────────

/// Evaluate a binary operation on two abstract values.
///
/// * Both `Top` → `Top`.
/// * Both `Concrete` → exact computation.
/// * Either `Unknown` → widen to `Unknown`.
/// * Interval involved → delegate to `interval::interval_binary`.
pub fn sym_eval_binary(op: BinaryOp, lhs: SymValue, rhs: SymValue) -> SymValue {
    match (lhs, rhs) {
        // Top absorbs everything.
        (SymValue::Top, _) | (_, SymValue::Top) => SymValue::Top,

        // Both Unknown: widen to the larger width.
        (SymValue::Unknown { width: w1 }, SymValue::Unknown { width: w2 }) => {
            SymValue::Unknown { width: w1.max(w2) }
        }

        // One Unknown, other Concrete or Interval: conservative widen to 64.
        (SymValue::Unknown { .. }, _) | (_, SymValue::Unknown { .. }) => {
            SymValue::Unknown { width: 64 }
        }

        // Both concrete: compute exactly.
        (SymValue::Concrete(a), SymValue::Concrete(b)) => {
            let result = match op {
                BinaryOp::And => u64::from(a != 0 && b != 0),
                BinaryOp::Or => u64::from(a != 0 || b != 0),
                BinaryOp::Xor => a ^ b,
                BinaryOp::Lt => u64::from(a < b),
                BinaryOp::Le => u64::from(a <= b),
                BinaryOp::Gt => u64::from(a > b),
                BinaryOp::Ge => u64::from(a >= b),
                BinaryOp::Eq => u64::from(a == b),
                BinaryOp::Ne => u64::from(a != b),
                BinaryOp::Add => a.wrapping_add(b),
                BinaryOp::Sub => a.wrapping_sub(b),
                BinaryOp::Mul => a.wrapping_mul(b),
                BinaryOp::Shl => {
                    if b >= 64 {
                        0
                    } else {
                        a.wrapping_shl(b as u32)
                    }
                }
                BinaryOp::Shr => {
                    if b >= 64 {
                        0
                    } else {
                        a.wrapping_shr(b as u32)
                    }
                }
            };
            SymValue::Concrete(result)
        }

        // At least one Interval (and no Top/Unknown): delegate to interval module.
        _ => interval::interval_binary(op, lhs, rhs),
    }
}

// ── Unary abstract transfer function ───────────────────────────────────────

/// Evaluate a unary operation on an abstract value.
///
/// * `Top` → `Top`.
/// * `Unknown` → `Unknown` (width-preserving).
/// * `Concrete` → exact for `Not`; `Negate` on unsigned widens to
///   `Unknown { width: 64 }` (unsigned negation is semantically signed).
/// * `Interval` → delegate to `interval::interval_unary`.
pub fn sym_eval_unary(op: UnaryOp, val: SymValue) -> SymValue {
    match val {
        SymValue::Top => SymValue::Top,
        SymValue::Unknown { width } => SymValue::Unknown { width },
        SymValue::Concrete(v) => match op {
            UnaryOp::Not => SymValue::Concrete(!v),
            UnaryOp::Negate => SymValue::Unknown { width: 64 },
        },
        SymValue::Interval { .. } => interval::interval_unary(op, val),
    }
}

// ── Refinement check ───────────────────────────────────────────────────────

/// Returns `true` if the abstract value is provably within `[lo, hi]`.
///
/// * `Concrete(v)` — `lo <= v && v <= hi`.
/// * `Interval { lo: a, hi: b }` — `lo <= a && b <= hi`.
/// * `Unknown` / `Top` — `false` (cannot prove containment).
pub fn sym_check_refinement(val: SymValue, lo: u64, hi: u64) -> bool {
    match val {
        SymValue::Concrete(v) => lo <= v && v <= hi,
        SymValue::Interval { lo: a, hi: b } => lo <= a && b <= hi,
        SymValue::Unknown { .. } | SymValue::Top => false,
    }
}

// ── Widening operator ──────────────────────────────────────────────────────

/// Widen two successive abstract values to ensure convergence.
///
/// If `old == new`, the fixpoint is stable and `old` is returned.
/// Otherwise the result climbs the lattice toward `Top`.
pub fn sym_widen(old: SymValue, new: SymValue) -> SymValue {
    if old == new {
        return old;
    }
    match (old, new) {
        (SymValue::Concrete(a), SymValue::Concrete(b)) => {
            SymValue::Interval { lo: a.min(b), hi: a.max(b) }
        }
        (SymValue::Interval { .. }, SymValue::Interval { .. }) => SymValue::Unknown { width: 64 },
        (SymValue::Concrete(_), SymValue::Interval { .. })
        | (SymValue::Interval { .. }, SymValue::Concrete(_)) => SymValue::Unknown { width: 64 },
        (SymValue::Unknown { .. }, _) | (_, SymValue::Unknown { .. }) => SymValue::Top,
        _ => SymValue::Top,
    }
}

// ── Module-level analysis ──────────────────────────────────────────────────

/// Symbolically analyze an entire MIRR module (single-pass, no fixpoint yet).
///
/// Builds an initial `SymState` from signal declarations, evaluates every
/// reflex assignment, and checks the result against declared width bounds.
///
/// Returns `Err(MirrError::SymbolicError)` with code E1003 if the module
/// exceeds `MAX_SYM_SIGNALS`.
pub fn analyze_module(module: &crate::ast::program::Module) -> Result<SymbolicResult, MirrError> {
    // Guard: reject modules that exceed the signal-count budget.
    if module.signals.len() > MAX_SYM_SIGNALS {
        return Err(MirrError::SymbolicError {
            message: format!(
                "[E1003] Symbolic analysis: {} signals exceed maximum ({})",
                module.signals.len(),
                MAX_SYM_SIGNALS
            ),
            span: None,
        });
    }

    // Build initial state: every signal starts as Unknown { width }.
    let mut state = SymState::new();
    let sig_count = module.signals.len().min(MAX_SYM_SIGNALS);
    for i in 0..sig_count {
        let sig = &module.signals[i];
        let width = sig.ty.signal_type().width();
        state.signals.push((sig.name.clone(), SymValue::Unknown { width }));
    }

    // Evaluate every reflex assignment symbolically.
    let mut intervals: Vec<(String, SymValue)> = Vec::new();
    let mut violations: Vec<SymbolicViolation> = Vec::new();

    let reflex_limit = module.reflexes.len().min(MAX_SYM_SIGNALS);
    for ri in 0..reflex_limit {
        let reflex = &module.reflexes[ri];
        let assign_limit = reflex.assignments.len().min(MAX_SYM_SIGNALS);
        for ai in 0..assign_limit {
            let assignment = &reflex.assignments[ai];
            let val = sym_eval_expr(&assignment.value, &state);
            intervals.push((assignment.target.clone(), val));

            // Check whether the computed value provably exceeds the
            // target signal's declared width bounds.
            let sig_width = lookup_signal_width(&module.signals, &assignment.target);
            if sig_width > 0 {
                let max_val = if sig_width >= 64 { u64::MAX } else { (1u64 << sig_width) - 1 };

                let definitely_exceeds = match val {
                    SymValue::Concrete(v) => v > max_val,
                    SymValue::Interval { hi, .. } => hi > max_val,
                    SymValue::Unknown { .. } | SymValue::Top => false,
                };

                if definitely_exceeds {
                    violations.push(SymbolicViolation {
                        signal: assignment.target.clone(),
                        expected: SymValue::Interval { lo: 0, hi: max_val },
                        actual: val,
                        message: format!(
                            "[E1001] Signal '{}' may exceed {}-bit width bounds",
                            assignment.target, sig_width
                        ),
                    });
                }
            }
        }
    }

    Ok(SymbolicResult { intervals, violations, iterations: 1, converged: true })
}

/// Look up the declared bit-width of a signal by name.
///
/// Returns 0 if the signal is not found (callers treat 0 as "skip check").
/// Bounded by `MAX_SYM_SIGNALS` (NASA Power-of-10).
fn lookup_signal_width(signals: &[crate::ast::program::SignalDecl], name: &str) -> u32 {
    let limit = signals.len().min(MAX_SYM_SIGNALS);
    for sig in signals.iter().take(limit) {
        if sig.name == name {
            return sig.ty.signal_type().width();
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concrete_add() {
        let result = sym_eval_binary(BinaryOp::Add, SymValue::Concrete(3), SymValue::Concrete(4));
        assert_eq!(result, SymValue::Concrete(7));
    }

    #[test]
    fn concrete_lt_true() {
        let result = sym_eval_binary(BinaryOp::Lt, SymValue::Concrete(2), SymValue::Concrete(5));
        assert_eq!(result, SymValue::Concrete(1));
    }

    #[test]
    fn concrete_lt_false() {
        let result = sym_eval_binary(BinaryOp::Lt, SymValue::Concrete(5), SymValue::Concrete(2));
        assert_eq!(result, SymValue::Concrete(0));
    }

    #[test]
    fn top_absorbs() {
        let result = sym_eval_binary(BinaryOp::Add, SymValue::Top, SymValue::Concrete(1));
        assert_eq!(result, SymValue::Top);
    }

    #[test]
    fn unknown_widens_to_max() {
        let result =
            sym_eval_binary(BinaryOp::Add, SymValue::Unknown { width: 8 }, SymValue::Concrete(1));
        assert_eq!(result, SymValue::Unknown { width: 64 });
    }

    #[test]
    fn unary_not_concrete() {
        let result = sym_eval_unary(UnaryOp::Not, SymValue::Concrete(0));
        assert_eq!(result, SymValue::Concrete(!0u64));
    }

    #[test]
    fn unary_negate_widens() {
        let result = sym_eval_unary(UnaryOp::Negate, SymValue::Concrete(42));
        assert_eq!(result, SymValue::Unknown { width: 64 });
    }

    #[test]
    fn refinement_concrete_in_range() {
        assert!(sym_check_refinement(SymValue::Concrete(100), 0, 255));
    }

    #[test]
    fn refinement_concrete_out_of_range() {
        assert!(!sym_check_refinement(SymValue::Concrete(300), 0, 255));
    }

    #[test]
    fn refinement_top_is_false() {
        assert!(!sym_check_refinement(SymValue::Top, 0, u64::MAX));
    }

    #[test]
    fn widen_same_is_stable() {
        let v = SymValue::Concrete(5);
        assert_eq!(sym_widen(v, v), v);
    }

    #[test]
    fn widen_concrete_to_interval() {
        let result = sym_widen(SymValue::Concrete(3), SymValue::Concrete(7));
        assert_eq!(result, SymValue::Interval { lo: 3, hi: 7 });
    }

    #[test]
    fn widen_intervals_to_unknown() {
        let a = SymValue::Interval { lo: 0, hi: 10 };
        let b = SymValue::Interval { lo: 5, hi: 20 };
        assert_eq!(sym_widen(a, b), SymValue::Unknown { width: 64 });
    }

    #[test]
    fn eval_literal_bool() {
        let state = SymState::new();
        let expr = Expr::Literal(LiteralValue::Bool(true));
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(1));
    }

    #[test]
    fn eval_literal_integer() {
        let state = SymState::new();
        let expr = Expr::Literal(LiteralValue::Integer(42));
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(42));
    }

    #[test]
    fn eval_signal_not_found_is_top() {
        let state = SymState::new();
        let expr = Expr::Signal("missing".to_string());
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Top);
    }

    #[test]
    fn eval_signal_found() {
        let mut state = SymState::new();
        state.signals.push(("x".to_string(), SymValue::Concrete(99)));
        let expr = Expr::Signal("x".to_string());
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(99));
    }

    #[test]
    fn eval_prev_is_top() {
        let state = SymState::new();
        let expr = Expr::Prev { signal: "x".to_string(), delay: 1 };
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Top);
    }

    #[test]
    fn eval_binary_expr() {
        let state = SymState::new();
        let expr = Expr::Binary {
            op: BinaryOp::Add,
            left: Box::new(Expr::Literal(LiteralValue::Integer(10))),
            right: Box::new(Expr::Literal(LiteralValue::Integer(20))),
        };
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(30));
    }

    #[test]
    fn eval_nested_binary() {
        // (3 + 4) * 5 = 35
        let state = SymState::new();
        let expr = Expr::Binary {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Literal(LiteralValue::Integer(3))),
                right: Box::new(Expr::Literal(LiteralValue::Integer(4))),
            }),
            right: Box::new(Expr::Literal(LiteralValue::Integer(5))),
        };
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(35));
    }

    #[test]
    fn eval_unary_expr() {
        let state = SymState::new();
        let expr = Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Literal(LiteralValue::Integer(0))),
        };
        assert_eq!(sym_eval_expr(&expr, &state), SymValue::Concrete(!0u64));
    }

    #[test]
    fn lookup_returns_top_for_empty() {
        let state = SymState::new();
        assert_eq!(state.lookup("anything"), SymValue::Top);
    }
}
