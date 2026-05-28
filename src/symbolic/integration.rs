// ---------------------------------------------------------------------------
//! Symbolic discrete integration approximations.
//!
//! Implements rectangular and trapezoidal integration approximations over
//! bounded sliding windows of abstract SymValue elements. All loops are
//! strictly bounded to satisfy NASA Power-of-10 rules.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use super::{sym_eval_binary, SymValue};
use crate::ast::types::BinaryOp;

/// Maximum sliding window size for integration.
pub const MAX_INTEGRATION_WINDOW: usize = 64;

/// Divide a SymValue by a concrete divisor constant.
fn div_by_const(val: SymValue, n: u64) -> SymValue {
    if n == 0 {
        return SymValue::Top;
    }
    match val {
        SymValue::Concrete(v) => SymValue::Concrete(v / n),
        SymValue::Interval { lo, hi } => {
            let new_lo = lo / n;
            let new_hi = hi / n;
            if new_lo == new_hi {
                SymValue::Concrete(new_lo)
            } else {
                SymValue::Interval { lo: new_lo, hi: new_hi }
            }
        }
        SymValue::Unknown { width } => SymValue::Unknown { width },
        SymValue::Top => SymValue::Top,
    }
}

/// Computes the left-hand rectangular integration approximation.
///
/// Area = Sum_{i=0}^{N-2} y_i (assuming Delta t = 1)
/// Bounded to `MAX_INTEGRATION_WINDOW` elements.
pub fn rectangular_integration_left(window: &[SymValue]) -> SymValue {
    if window.len() < 2 {
        return SymValue::Concrete(0);
    }

    let limit = (window.len() - 1).min(MAX_INTEGRATION_WINDOW);
    let mut area = SymValue::Concrete(0);

    for val in window.iter().take(limit) {
        area = sym_eval_binary(BinaryOp::Add, area, *val);
    }

    area
}

/// Computes the right-hand rectangular integration approximation.
///
/// Area = Sum_{i=1}^{N-1} y_i (assuming Delta t = 1)
/// Bounded to `MAX_INTEGRATION_WINDOW` elements.
pub fn rectangular_integration_right(window: &[SymValue]) -> SymValue {
    if window.len() < 2 {
        return SymValue::Concrete(0);
    }

    let limit = window.len().min(MAX_INTEGRATION_WINDOW);
    let mut area = SymValue::Concrete(0);

    for val in window.iter().skip(1).take(limit - 1) {
        area = sym_eval_binary(BinaryOp::Add, area, *val);
    }

    area
}

/// Computes the trapezoidal integration approximation.
///
/// Area = Sum_{i=0}^{N-2} (y_i + y_{i+1}) / 2 (assuming Delta t = 1)
/// Bounded to `MAX_INTEGRATION_WINDOW` elements.
pub fn trapezoidal_integration(window: &[SymValue]) -> SymValue {
    if window.len() < 2 {
        return SymValue::Concrete(0);
    }

    let limit = (window.len() - 1).min(MAX_INTEGRATION_WINDOW);
    let mut area = SymValue::Concrete(0);

    for i in 0..limit {
        let sum = sym_eval_binary(BinaryOp::Add, window[i], window[i + 1]);
        let avg = div_by_const(sum, 2);
        area = sym_eval_binary(BinaryOp::Add, area, avg);
    }

    area
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangular_integration_concrete() {
        let window = vec![SymValue::Concrete(5), SymValue::Concrete(10), SymValue::Concrete(15)];
        // Left: 5 + 10 = 15
        assert_eq!(rectangular_integration_left(&window), SymValue::Concrete(15));
        // Right: 10 + 15 = 25
        assert_eq!(rectangular_integration_right(&window), SymValue::Concrete(25));
    }

    #[test]
    fn test_trapezoidal_integration_concrete() {
        let window = vec![SymValue::Concrete(10), SymValue::Concrete(20), SymValue::Concrete(30)];
        // (10+20)/2 + (20+30)/2 = 15 + 25 = 40
        assert_eq!(trapezoidal_integration(&window), SymValue::Concrete(40));
    }

    #[test]
    fn test_trapezoidal_integration_intervals() {
        let window =
            vec![SymValue::Interval { lo: 8, hi: 12 }, SymValue::Interval { lo: 18, hi: 22 }];
        // Sum = [26, 34], Div 2 = [13, 17]
        assert_eq!(trapezoidal_integration(&window), SymValue::Interval { lo: 13, hi: 17 });
    }
}
