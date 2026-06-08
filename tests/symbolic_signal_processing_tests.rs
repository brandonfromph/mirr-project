#![forbid(unsafe_code)]

use mirrc::symbolic::integration::{
    rectangular_integration_left, rectangular_integration_right, trapezoidal_integration,
    MAX_INTEGRATION_WINDOW,
};
use mirrc::symbolic::statistics::{
    symbolic_mean, symbolic_trend, symbolic_variance, MAX_STATS_WINDOW,
};
use mirrc::symbolic::SymValue;

#[test]
fn test_integration_statistics_mean_and_variance() {
    let window = vec![
        SymValue::Interval { lo: 10, hi: 20 },
        SymValue::Interval { lo: 20, hi: 30 },
        SymValue::Interval { lo: 30, hi: 40 },
    ];

    // Sum = [60, 90], Mean = [20, 30]
    let mean = symbolic_mean(&window);
    assert_eq!(mean, SymValue::Interval { lo: 20, hi: 30 });

    // Variance check: Var = E[X^2] - (E[X])^2
    // window elements: [10,20], [20,30], [30,40]
    // window^2: [100, 400], [400, 900], [900, 1600]
    // Sum(window^2): [1400, 2900]
    // E[X^2]: [1400/3, 2900/3] -> [466, 966]
    // (E[X])^2: [400, 900]
    // E[X^2] - E[X]^2 = [466 - 900 (saturating sub -> 0), 966 - 400 -> 566]
    // Let's assert variance is soundly computed within [0, 566].
    let var = symbolic_variance(&window);
    assert_eq!(var, SymValue::Interval { lo: 0, hi: 566 });
}

#[test]
fn test_integration_statistics_trend() {
    let window = vec![
        SymValue::Interval { lo: 0, hi: 5 },
        SymValue::Interval { lo: 0, hi: 5 },
        SymValue::Interval { lo: 20, hi: 25 },
        SymValue::Interval { lo: 20, hi: 25 },
    ];

    // first half mean: [0, 5]
    // second half mean: [20, 25]
    // trend: [20, 25] - [0, 5] -> [20 - 5, 25 - 0] -> [15, 25]
    let trend = symbolic_trend(&window);
    assert_eq!(trend, SymValue::Interval { lo: 15, hi: 25 });
}

#[test]
fn test_integration_approximations_trapezoidal() {
    let window = vec![SymValue::Interval { lo: 10, hi: 20 }, SymValue::Interval { lo: 30, hi: 40 }];

    // ( [10,20] + [30,40] ) / 2 = [40, 60] / 2 = [20, 30]
    let area = trapezoidal_integration(&window);
    assert_eq!(area, SymValue::Interval { lo: 20, hi: 30 });
}

#[test]
fn test_integration_approximations_rectangular() {
    let window = vec![SymValue::Concrete(10), SymValue::Concrete(20), SymValue::Concrete(30)];

    let left = rectangular_integration_left(&window);
    assert_eq!(left, SymValue::Concrete(30)); // 10 + 20

    let right = rectangular_integration_right(&window);
    assert_eq!(right, SymValue::Concrete(50)); // 20 + 30
}

#[test]
fn test_integration_statistics_respects_max_bounds() {
    let super_large = vec![SymValue::Concrete(2); MAX_STATS_WINDOW + 20];

    // Mean of concrete 2s is concrete 2
    let mean = symbolic_mean(&super_large);
    assert_eq!(mean, SymValue::Concrete(2));

    let super_large_int = vec![SymValue::Concrete(2); MAX_INTEGRATION_WINDOW + 20];
    let left = rectangular_integration_left(&super_large_int);
    // Sums strictly up to MAX_INTEGRATION_WINDOW elements (64 * 2 = 128)
    assert_eq!(left, SymValue::Concrete(128));
}
