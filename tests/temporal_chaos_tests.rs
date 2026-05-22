#![forbid(unsafe_code)]

use nasa_rust_project::mape_k::analyzer::Analyzer;
use nasa_rust_project::mape_k::ltl::{SignalPredicate, TemporalProperty};
use nasa_rust_project::mape_k::monitor::Monitor;

#[test]
fn test_temporal_chaos_impatient_trigger_false_positive() {
    // Setup a monitor with a window of size 10.
    // We record 5 samples.
    let mut mon = Monitor::new(10, &["p", "q"]);

    // Tick 0-3: Nothing happens.
    for _ in 0..4 {
        mon.record_sample("p", 0);
        mon.record_sample("q", 0);
        mon.advance_tick();
    }

    // Tick 4: Trigger 'p' happens.
    // Requirement: 'q' must happen with delay=2 (at Tick 6).
    mon.record_sample("p", 1);
    mon.record_sample("q", 0);
    mon.advance_tick();

    // Current state: window length is 5.
    // Trigger is at index 4.
    // Target index is 4 + 2 = 6.
    // BUT the window only goes up to index 4.

    let prop = TemporalProperty::AlwaysFollowedBy(
        SignalPredicate::IsTrue("p".to_string()),
        2,
        SignalPredicate::IsTrue("q".to_string()),
    );

    let analyzer = Analyzer::new(vec![prop]);
    let results = analyzer.evaluate(&mon);

    // HARDENED: The analyzer should NOT mark this as violated yet.
    // It should be 'Satisfied-So-Far' because tick 6 hasn't happened.
    assert!(results[0].satisfied, "Hardened Analyzer (Patient Trigger) correctly ignores trigger too close to window boundary.");

    println!("Confirmed: Hardened Analyzer (Patient Trigger) avoided false positive.");
}

#[test]
fn test_temporal_chaos_empty_window_vacuous_truth() {
    let mon = Monitor::new(10, &["p"]);
    // Window is empty.

    let prop = TemporalProperty::Always(SignalPredicate::IsTrue("p".to_string()));

    let analyzer = Analyzer::new(vec![prop]);
    let results = analyzer.evaluate(&mon);

    // HARDENED: No data means NOT satisfied.
    assert!(!results[0].satisfied, "Hardened Analyzer correctly rejects empty window as unsafe.");
}
