#![forbid(unsafe_code)]

//! Interval arithmetic for the symbolic evaluation engine.
//!
//! Implements sound interval arithmetic (Moore 1966) for all 14 `BinaryOp`
//! and 2 `UnaryOp` variants in the MIRR expression language.
//!
//! **Restriction:** Unsigned intervals only (v1). Signed operands yield
//! `SymValue::Unknown { width }` (safe over-approximation).

use crate::ast::types::{BinaryOp, UnaryOp};

use super::SymValue;

// ---------------------------------------------------------------------------
// Internal helpers (not public — zero-debt: no dead code, no wrappers)
// ---------------------------------------------------------------------------

/// Collapse `[v, v]` to `Concrete(v)`; otherwise return `Interval { lo, hi }`.
fn normalize(lo: u64, hi: u64) -> SymValue {
    if lo == hi {
        SymValue::Concrete(lo)
    } else {
        SymValue::Interval { lo, hi }
    }
}

/// Extract `(lo, hi)` from a `SymValue`, returning `None` for `Unknown`/`Top`.
///
/// `Concrete(v)` is treated as the degenerate interval `[v, v]`.
fn to_interval(val: SymValue) -> Option<(u64, u64)> {
    match val {
        SymValue::Concrete(v) => Some((v, v)),
        SymValue::Interval { lo, hi } => Some((lo, hi)),
        SymValue::Unknown { .. } | SymValue::Top => None,
    }
}

/// Saturating left shift: returns `u64::MAX` when `val << k` would overflow.
///
/// `u64::checked_shl` only detects shifts >= 64; it silently truncates high
/// bits for smaller shifts. This function detects value overflow as well.
fn saturating_shl(val: u64, k: u32) -> u64 {
    if k >= 64 {
        if val == 0 {
            0
        } else {
            u64::MAX
        }
    } else if val > (u64::MAX >> k) {
        u64::MAX
    } else {
        val << k
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Sound interval arithmetic for all 14 binary operators.
///
/// Both operands are converted to `(lo, hi)` pairs. `Concrete(v)` is treated
/// as `[v, v]`. If either operand is `Top`, the result is `Top`. If either is
/// `Unknown`, the result is `Unknown { width: 64 }`.
///
/// After computing the result interval, `[v, v]` is collapsed to `Concrete(v)`.
///
/// # Operator rules (unsigned)
///
/// | Op    | Rule                                                          |
/// |-------|---------------------------------------------------------------|
/// | Add   | `[a_lo + b_lo, a_hi + b_hi]` (saturating)                    |
/// | Sub   | `[a_lo - b_hi, a_hi - b_lo]` (saturating at 0)               |
/// | Mul   | 4-corner: min/max of all `{a_lo,a_hi} x {b_lo,b_hi}` prods  |
/// | And   | `[0, min(a_hi, b_hi)]`                                       |
/// | Or    | `[max(a_lo, b_lo), a_hi | b_hi]`                             |
/// | Xor   | `[0, max(a_hi, b_hi)]` (over-approximation)                  |
/// | Shl   | Concrete shift k: `[a_lo << k, a_hi << k]` (saturating)      |
/// | Shr   | Concrete shift k: `[a_lo >> k, a_hi >> k]`                   |
/// | Eq/Ne | Point/disjoint analysis -> `{0}`, `{1}`, or `[0,1]`          |
/// | Lt/Le/Gt/Ge | Endpoint comparison -> `{0}`, `{1}`, or `[0,1]`       |
pub fn interval_binary(op: BinaryOp, lhs: SymValue, rhs: SymValue) -> SymValue {
    // Top propagates unconditionally.
    if matches!(lhs, SymValue::Top) || matches!(rhs, SymValue::Top) {
        return SymValue::Top;
    }

    // Unknown propagates as safe over-approximation.
    let (a_lo, a_hi) = match to_interval(lhs) {
        Some(pair) => pair,
        None => return SymValue::Unknown { width: 64 },
    };
    let (b_lo, b_hi) = match to_interval(rhs) {
        Some(pair) => pair,
        None => return SymValue::Unknown { width: 64 },
    };

    match op {
        // ----- Arithmetic -----
        BinaryOp::Add => {
            let lo = a_lo.saturating_add(b_lo);
            let hi = a_hi.saturating_add(b_hi);
            normalize(lo, hi)
        }
        BinaryOp::Sub => {
            let lo = a_lo.saturating_sub(b_hi);
            let hi = a_hi.saturating_sub(b_lo);
            normalize(lo, hi)
        }
        BinaryOp::Mul => {
            // 4-corner multiplication with saturation on overflow.
            let c0 = a_lo.checked_mul(b_lo).unwrap_or(u64::MAX);
            let c1 = a_lo.checked_mul(b_hi).unwrap_or(u64::MAX);
            let c2 = a_hi.checked_mul(b_lo).unwrap_or(u64::MAX);
            let c3 = a_hi.checked_mul(b_hi).unwrap_or(u64::MAX);
            let lo = c0.min(c1).min(c2).min(c3);
            let hi = c0.max(c1).max(c2).max(c3);
            normalize(lo, hi)
        }

        // ----- Bitwise -----
        BinaryOp::And => {
            // AND can zero any bit; lower bound is 0.
            let hi = a_hi.min(b_hi);
            normalize(0, hi)
        }
        BinaryOp::Or => {
            // OR can set any bit present in either operand.
            let lo = a_lo.max(b_lo);
            let hi = a_hi | b_hi;
            normalize(lo, hi)
        }
        BinaryOp::Xor => {
            // XOR over-approximation: result is in [0, max(a_hi, b_hi)].
            let hi = a_hi.max(b_hi);
            normalize(0, hi)
        }

        // ----- Shifts -----
        BinaryOp::Shl => {
            // Shift is only precise when the shift amount is concrete.
            if b_lo == b_hi {
                let k = b_lo.min(63) as u32;
                let lo = saturating_shl(a_lo, k);
                let hi = saturating_shl(a_hi, k);
                normalize(lo, hi)
            } else {
                SymValue::Unknown { width: 64 }
            }
        }
        BinaryOp::Shr => {
            // Logical right shift is monotone for unsigned values.
            if b_lo == b_hi {
                let k = b_lo.min(63) as u32;
                let lo = a_lo >> k;
                let hi = a_hi >> k;
                normalize(lo, hi)
            } else {
                SymValue::Unknown { width: 64 }
            }
        }

        // ----- Comparisons -----
        BinaryOp::Eq => {
            if a_lo == a_hi && b_lo == b_hi && a_lo == b_lo {
                // Both are single points and equal.
                SymValue::Concrete(1)
            } else if a_hi < b_lo || b_hi < a_lo {
                // Intervals are disjoint — definitely not equal.
                SymValue::Concrete(0)
            } else {
                SymValue::Interval { lo: 0, hi: 1 }
            }
        }
        BinaryOp::Ne => {
            if a_lo == a_hi && b_lo == b_hi && a_lo == b_lo {
                // Both are single points and equal — definitely not unequal.
                SymValue::Concrete(0)
            } else if a_hi < b_lo || b_hi < a_lo {
                // Intervals are disjoint — definitely unequal.
                SymValue::Concrete(1)
            } else {
                SymValue::Interval { lo: 0, hi: 1 }
            }
        }
        BinaryOp::Lt => {
            if a_hi < b_lo {
                // All of a is below all of b.
                SymValue::Concrete(1)
            } else if a_lo >= b_hi {
                // All of a is at or above all of b.
                SymValue::Concrete(0)
            } else {
                SymValue::Interval { lo: 0, hi: 1 }
            }
        }
        BinaryOp::Le => {
            if a_hi <= b_lo {
                // Largest a is at most smallest b.
                SymValue::Concrete(1)
            } else if a_lo > b_hi {
                // Smallest a exceeds largest b.
                SymValue::Concrete(0)
            } else {
                SymValue::Interval { lo: 0, hi: 1 }
            }
        }
        BinaryOp::Gt => {
            if a_lo > b_hi {
                // Smallest a exceeds largest b.
                SymValue::Concrete(1)
            } else if a_hi <= b_lo {
                // Largest a is at most smallest b.
                SymValue::Concrete(0)
            } else {
                SymValue::Interval { lo: 0, hi: 1 }
            }
        }
        BinaryOp::Ge => {
            if a_lo >= b_hi {
                // Smallest a is at or above largest b.
                SymValue::Concrete(1)
            } else if a_hi < b_lo {
                // Largest a is below smallest b.
                SymValue::Concrete(0)
            } else {
                SymValue::Interval { lo: 0, hi: 1 }
            }
        }
    }
}

/// Sound interval arithmetic for the 2 unary operators.
///
/// - `Not`: Bitwise complement reverses unsigned ordering — `!hi < !lo`.
///   For `Concrete(v)`: `Concrete(!v)`. For `Unknown`/`Top`: `Top`.
/// - `Negate`: Negation of unsigned is semantically signed. Returns
///   `Unknown { width: 64 }` in v1 (safe over-approximation).
pub fn interval_unary(op: UnaryOp, val: SymValue) -> SymValue {
    match op {
        UnaryOp::Not => match val {
            SymValue::Concrete(v) => SymValue::Concrete(!v),
            SymValue::Interval { lo, hi } => {
                // Bitwise NOT reverses unsigned ordering: !hi < !lo.
                normalize(!hi, !lo)
            }
            SymValue::Unknown { .. } | SymValue::Top => SymValue::Top,
        },
        UnaryOp::Negate => {
            // Negation of unsigned is semantically signed — punt to Unknown (v1).
            SymValue::Unknown { width: 64 }
        }
    }
}
