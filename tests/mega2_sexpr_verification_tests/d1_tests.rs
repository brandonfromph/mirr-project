use super::*;

// ===========================================================================
// D1: all_examples_roundtrip (12 tests)
// ===========================================================================

#[test]
fn test_d1_roundtrip_tmr_sensor_fusion() {
    roundtrip_mirr(tmr_src(), "tmr_sensor_fusion");
}

#[test]
fn test_d1_roundtrip_flight_controller() {
    roundtrip_mirr(flight_controller_src(), "flight_controller");
}

#[test]
fn test_d1_roundtrip_neonatal() {
    roundtrip_mirr(neonatal_src(), "neonatal_respirator");
}

#[test]
fn test_d1_roundtrip_icu_monitor() {
    roundtrip_mirr(icu_src(), "icu_monitor");
}

#[test]
fn test_d1_roundtrip_industrial_safety() {
    roundtrip_mirr(industrial_src(), "industrial_safety");
}

#[test]
fn test_d1_roundtrip_autonomous_vehicle() {
    roundtrip_mirr(autonomous_src(), "autonomous_vehicle");
}

#[test]
fn test_d1_roundtrip_safety_property() {
    roundtrip_mirr(safety_property_src(), "safety_property");
}

#[test]
fn test_d1_roundtrip_shift_register() {
    roundtrip_mirr(shift_register_src(), "shift_register");
}

#[test]
fn test_d1_roundtrip_multi_guard() {
    roundtrip_mirr(multi_guard_src(), "multi_guard");
}

#[test]
fn test_d1_roundtrip_fir_filter() {
    roundtrip_mirr(fir_src(), "fir_filter");
}

#[test]
fn test_d1_roundtrip_signed() {
    roundtrip_mirr(signed_src(), "signed");
}

#[test]
fn test_d1_roundtrip_minimal_bool() {
    roundtrip_mirr(minimal_bool_src(), "minimal_bool");
}
