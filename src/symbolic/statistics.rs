// ---------------------------------------------------------------------------
//! Moving-window statistics for symbolic signal analysis.
//!
//! Implements symbolic calculations for mean, variance, and trend over
//! bounded history buffers of SymValue elements. Follows sound abstract
//! interpretation principles (Cousot & Cousot 1977). All window loops
//! are strictly bounded to satisfy NASA Power-of-10 rules.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use super::{sym_eval_binary, SymValue};
use crate::ast::types::BinaryOp;

/// Maximum sliding window size for statistics.
pub const MAX_STATS_WINDOW: usize = 64;

/// Divide a SymValue by a concrete divisor constant.
///
/// Soundly maps intervals: `[lo / n, hi / n]`.
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

/// Compute the symbolic mean of a window of abstract values.
///
/// Soundly propagates intervals, Unknown, and Top.
/// Bounded to `MAX_STATS_WINDOW` elements.
pub fn symbolic_mean(window: &[SymValue]) -> SymValue {
    if window.is_empty() {
        return SymValue::Top;
    }

    let limit = window.len().min(MAX_STATS_WINDOW);
    let mut sum = SymValue::Concrete(0);

    for val in window.iter().take(limit) {
        sum = sym_eval_binary(BinaryOp::Add, sum, *val);
    }

    div_by_const(sum, limit as u64)
}

/// Compute the symbolic variance of a window of abstract values.
///
/// Uses the sound formula: `Var = E[X^2] - (E[X])^2`.
/// Bounded to `MAX_STATS_WINDOW` elements.
pub fn symbolic_variance(window: &[SymValue]) -> SymValue {
    if window.is_empty() {
        return SymValue::Top;
    }

    let limit = window.len().min(MAX_STATS_WINDOW);

    // 1. Compute Mean(X)
    let mean_x = symbolic_mean(window);

    // 2. Compute (Mean(X))^2
    let mean_x_sq = sym_eval_binary(BinaryOp::Mul, mean_x, mean_x);

    // 3. Compute Mean(X^2)
    let mut sum_x_sq = SymValue::Concrete(0);
    for val in window.iter().take(limit) {
        let x_sq = sym_eval_binary(BinaryOp::Mul, *val, *val);
        sum_x_sq = sym_eval_binary(BinaryOp::Add, sum_x_sq, x_sq);
    }
    let mean_x_sq_expect = div_by_const(sum_x_sq, limit as u64);

    // 4. Var = E[X^2] - (E[X])^2
    sym_eval_binary(BinaryOp::Sub, mean_x_sq_expect, mean_x_sq)
}

/// Compute the symbolic trend of a window of abstract values.
///
/// Compares the mean of the second half of the window against
/// the mean of the first half: `Trend = Mean(Second Half) - Mean(First Half)`.
/// Returns an interval representing the possible slope/trend.
///
/// Bounded to `MAX_STATS_WINDOW` elements.
pub fn symbolic_trend(window: &[SymValue]) -> SymValue {
    if window.len() < 2 {
        return SymValue::Concrete(0);
    }

    let limit = window.len().min(MAX_STATS_WINDOW);
    let mid = limit / 2;

    let first_half = &window[0..mid];
    let second_half = &window[mid..limit];

    let mean_first = symbolic_mean(first_half);
    let mean_second = symbolic_mean(second_half);

    // Trend interval represents the change/slope
    sym_eval_binary(BinaryOp::Sub, mean_second, mean_first)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_by_const() {
        let v_concrete = SymValue::Concrete(10);
        assert_eq!(div_by_const(v_concrete, 2), SymValue::Concrete(5));

        let v_interval = SymValue::Interval { lo: 10, hi: 20 };
        assert_eq!(div_by_const(v_interval, 2), SymValue::Interval { lo: 5, hi: 10 });
        assert_eq!(div_by_const(v_interval, 20), SymValue::Interval { lo: 0, hi: 1 });
    }

    #[test]
    fn test_symbolic_mean_concrete() {
        let window = vec![SymValue::Concrete(10), SymValue::Concrete(20), SymValue::Concrete(30)];
        assert_eq!(symbolic_mean(&window), SymValue::Concrete(20));
    }

    #[test]
    fn test_symbolic_mean_interval() {
        let window =
            vec![SymValue::Interval { lo: 5, hi: 15 }, SymValue::Interval { lo: 15, hi: 25 }];
        // Sum = [20, 40], Mean = [10, 20]
        assert_eq!(symbolic_mean(&window), SymValue::Interval { lo: 10, hi: 20 });
    }

    #[test]
    fn test_symbolic_variance() {
        // Concrete constant sequence has 0 variance
        let window_const =
            vec![SymValue::Concrete(5), SymValue::Concrete(5), SymValue::Concrete(5)];
        assert_eq!(symbolic_variance(&window_const), SymValue::Concrete(0));

        let window_var = vec![SymValue::Concrete(2), SymValue::Concrete(4), SymValue::Concrete(6)];
        // Mean = 4.
        // X^2 = [4, 16, 36], Mean(X^2) = 56 / 3 = 18.
        // Mean(X)^2 = 16.
        // Var = 18 - 16 = 2.
        assert_eq!(symbolic_variance(&window_var), SymValue::Concrete(2));
    }

    #[test]
    fn test_symbolic_trend() {
        // Upward trend
        let window = vec![
            SymValue::Concrete(10),
            SymValue::Concrete(12),
            SymValue::Concrete(20),
            SymValue::Concrete(22),
        ];
        // first half mean = 11, second half mean = 21. Trend = 21 - 11 = 10.
        assert_eq!(symbolic_trend(&window), SymValue::Concrete(10));

        // Downward trend (wrapping subtraction in concrete mode)
        let window_down = vec![
            SymValue::Concrete(20),
            SymValue::Concrete(20),
            SymValue::Concrete(5),
            SymValue::Concrete(5),
        ];
        // first half mean = 20, second half mean = 5. Trend = 5 - 20 = -15 (wrapped)
        assert_eq!(symbolic_trend(&window_down), SymValue::Concrete(5u64.wrapping_sub(20)));
    }
}
