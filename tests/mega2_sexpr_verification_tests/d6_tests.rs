use super::*;

// ===========================================================================
// D6: reader_macros_all (10 tests)
// ===========================================================================

#[test]
fn test_d6_reader_macro_registry_new() {
    let reg = ReaderMacroRegistry::new();
    assert!(!reg.is_empty(), "Default registry should have built-in macros");
}

#[test]
fn test_d6_reader_freq_hz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "1000Hz");
    assert!(result.is_ok(), "freq Hz should be recognized");
}

#[test]
fn test_d6_reader_freq_khz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "100KHz");
    assert!(result.is_ok(), "freq KHz should be recognized");
}

#[test]
fn test_d6_reader_freq_mhz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "50MHz");
    assert!(result.is_ok(), "freq MHz should be recognized");
}

#[test]
fn test_d6_reader_freq_ghz() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("freq", "2GHz");
    assert!(result.is_ok(), "freq GHz should be recognized");
}

#[test]
fn test_d6_reader_delay() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("delay", "5");
    assert!(result.is_ok(), "delay should be recognized");
}

#[test]
fn test_d6_reader_range() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("range", "0..255");
    assert!(result.is_ok(), "range should be recognized");
}

#[test]
fn test_d6_reader_unknown_macro_returns_err() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("unknown_macro", "1 2 3");
    assert!(result.is_err(), "Unknown macro should return error");
}

#[test]
fn test_d6_reader_registry_is_not_empty() {
    let reg = ReaderMacroRegistry::new();
    assert!(!reg.is_empty(), "Default registry should not be empty");
}

#[test]
fn test_d6_reader_delay_zero() {
    let reg = ReaderMacroRegistry::new();
    let result = reg.expand("delay", "0");
    assert!(result.is_ok(), "delay 0 should be recognized");
}
