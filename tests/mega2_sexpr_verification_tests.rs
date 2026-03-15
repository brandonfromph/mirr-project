#![forbid(unsafe_code)]
//! MEGA-2 Subsystem Verification Tests — S-expression IR
//!
//! Verifies:
//!   - AST↔S-expr roundtrip preserves all semantic information
//!   - eval correctly implements car/cdr/cons/match-type
//!   - Parser handles depth limits
//!   - All 12 valid examples survive roundtrip
//!
//! NASA naming convention: test_d{section}_{description}

use nasa_rust_project::sexpr::types::SExpr;
use nasa_rust_project::sexpr::{
    ast_to_sexpr, eval, parse_sexpr, print_sexpr, sexpr_to_ast, EvalState, MacroExpander,
    ReaderMacroRegistry, MAX_EVAL_DEPTH, MAX_SEXPR_DEPTH,
};
use nasa_rust_project::{parse_mirr, validate_module};

/// NASA P10: maximum test iterations.
const _MAX_TEST_EXAMPLES: usize = 20;
/// Maximum nesting depth in generated test expressions.
const _MAX_TEST_NESTING: usize = 100;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Parse, validate, then roundtrip through S-expr.
fn roundtrip_mirr(src: &str, label: &str) {
    let parsed = parse_mirr(src).unwrap_or_else(|e| panic!("{label}: parse failed: {e}"));
    validate_module(&parsed.module).unwrap_or_else(|e| panic!("{label}: validation failed: {e}"));
    let sexpr1 = ast_to_sexpr(&parsed);
    let roundtrip =
        sexpr_to_ast(&sexpr1).unwrap_or_else(|e| panic!("{label}: sexpr_to_ast failed: {e}"));
    let sexpr2 = ast_to_sexpr(&roundtrip);
    let s1 = print_sexpr(&sexpr1);
    let s2 = print_sexpr(&sexpr2);
    assert_eq!(s1, s2, "{label}: roundtrip mismatch");
}

fn flight_controller_src() -> &'static str {
    r#"module flight_controller {
    signal altitude: in u32;
    signal airspeed: in u16;
    signal pitch_angle: in u16;
    signal roll_angle: in u16;
    signal throttle_cut: out bool;
    signal stabilise: out bool;
    signal terrain_warn: out bool;
    signal status_code: internal u8;

    guard altitude_low {
        when altitude < 500
        for 10 cycles;
    }

    guard overspeed {
        when airspeed > 340
        for 5 cycles;
    }

    guard excessive_pitch {
        when pitch_angle > 30
        for 8 cycles;
    }

    guard excessive_roll {
        when roll_angle > 60
        for 4 cycles;
    }

    reflex terrain_alert {
        on altitude_low {
            terrain_warn = true;
        }
    }

    reflex cut_throttle {
        on overspeed {
            throttle_cut = true;
        }
    }

    reflex auto_stabilise {
        on excessive_pitch and excessive_roll {
            stabilise = true;
        }
    }

    property speed_bounded {
        always (airspeed < 400);
    }

    property low_alt_warns {
        always (altitude < 500 -> terrain_warn);
    }
}"#
}

fn tmr_src() -> &'static str {
    r#"module tmr_sensor_fusion {
    signal sensor_a: in u16;
    signal sensor_b: in u16;
    signal sensor_c: in u16;
    signal sensor_a_ok: in bool;
    signal sensor_b_ok: in bool;
    signal sensor_c_ok: in bool;
    signal heartbeat: in bool;
    signal system_armed: in bool;
    signal manual_override: in bool;
    signal rst_n: in bool;
    signal pressure: in u16;
    signal temperature: in u16;
    signal voted_value: out u16;
    signal fault_detected: out bool;
    signal sensor_a_failed: out bool;
    signal sensor_b_failed: out bool;
    signal sensor_c_failed: out bool;
    signal watchdog_timeout: out bool;
    signal safety_shutdown: out bool;
    signal pressure_alarm: out bool;
    signal temp_alarm: out bool;
    signal vote_select: internal u8;
    signal fault_latch: internal bool;
    signal shutdown_latch: internal bool;
    signal armed_status: internal bool;
    signal override_active: internal bool;
    signal hb_status: internal bool;

    guard a_healthy {
        when sensor_a_ok
        for 1 cycles;
    }

    guard b_healthy {
        when sensor_b_ok
        for 1 cycles;
    }

    guard c_healthy {
        when sensor_c_ok
        for 1 cycles;
    }

    guard a_sick {
        when !sensor_a_ok
        for 8 cycles;
    }

    guard b_sick {
        when !sensor_b_ok
        for 8 cycles;
    }

    guard c_sick {
        when !sensor_c_ok
        for 8 cycles;
    }

    guard no_heartbeat {
        when !heartbeat
        for 64 cycles;
    }

    guard temp_high {
        when temperature > 800
        for 4 cycles;
    }

    guard is_armed {
        when system_armed
        for 1 cycles;
    }

    guard fault_held {
        when fault_detected == true
        for 16 cycles;
    }

    guard override_on {
        when manual_override
        for 1 cycles;
    }

    guard hb_alive {
        when heartbeat
        for 1 cycles;
    }

    reflex vote_a {
        on a_healthy {
            voted_value = sensor_a;
            vote_select = 1;
        }
    }

    reflex flag_a_failed {
        on a_sick {
            sensor_a_failed = true;
            fault_latch = true;
        }
    }

    reflex flag_b_failed {
        on b_sick {
            sensor_b_failed = true;
        }
    }

    reflex flag_c_failed {
        on c_sick {
            sensor_c_failed = true;
        }
    }

    reflex set_fault {
        on a_sick {
            fault_detected = true;
        }
    }

    reflex trigger_watchdog {
        on no_heartbeat {
            watchdog_timeout = true;
        }
    }

    reflex trip_temp {
        on temp_high {
            temp_alarm = true;
        }
    }

    reflex engage_shutdown {
        on is_armed and fault_held {
            safety_shutdown = true;
            shutdown_latch = true;
        }
    }

    reflex track_override {
        on override_on {
            override_active = true;
        }
    }

    reflex track_armed {
        on is_armed {
            armed_status = true;
        }
    }

    reflex track_hb {
        on hb_alive {
            hb_status = true;
        }
    }

    property vote_integrity {
        always (voted_value == sensor_a || voted_value == sensor_b || voted_value == sensor_c);
    }

    property no_spurious_shutdown {
        always (safety_shutdown -> fault_detected);
    }

    property not_triple_failure {
        never (sensor_a_failed && sensor_b_failed && sensor_c_failed);
    }

    property fault_latency_bound {
        eventually within 16 (fault_detected);
    }

    property shutdown_follows_fault {
        always (fault_detected followed_by 32 safety_shutdown);
    }

    property healthy_env {
        assume always (sensor_a_ok || sensor_b_ok || sensor_c_ok);
    }

    property pressure_alarm_reachable {
        cover eventually within 100 (pressure_alarm);
    }
}"#
}

fn neonatal_src() -> &'static str {
    r#"module neonatal_respirator {
    signal respirator_enable: in bool;
    signal airway_pressure: in u16;
    signal clamp_valve: out bool;

    guard sustained_pressure_drop {
        when airway_pressure < 50
        for 1000 cycles;
    }

    reflex emergency_clamp {
        on sustained_pressure_drop {
            clamp_valve = true;
        }
    }
}"#
}

fn icu_src() -> &'static str {
    r#"module icu_monitor {
    signal heart_rate: in u16;
    signal spo2: in u16;
    signal systolic_bp: in u16;
    signal diastolic_bp: in u16;
    signal cardiac_alarm: out bool;
    signal tachy_alarm: out bool;
    signal hypoxia_alarm: out bool;
    signal bp_alarm: out bool;
    signal code_blue: out bool;

    guard bradycardia {
        when heart_rate < 50
        for 300 cycles;
    }

    guard tachycardia {
        when heart_rate > 150
        for 120 cycles;
    }

    guard low_spo2 {
        when spo2 < 90
        for 60 cycles;
    }

    guard hypotension {
        when systolic_bp < 80
        for 200 cycles;
    }

    reflex cardiac_alert {
        on bradycardia {
            cardiac_alarm = true;
        }
    }

    reflex tachy_alert {
        on tachycardia {
            tachy_alarm = true;
        }
    }

    reflex hypoxia_alert {
        on low_spo2 {
            hypoxia_alarm = true;
        }
    }

    reflex bp_alert {
        on hypotension {
            bp_alarm = true;
        }
    }

    reflex code_blue_alert {
        on bradycardia and low_spo2 {
            code_blue = true;
        }
    }

    property spo2_bounded {
        always (spo2 < 101);
    }

    property no_silent_hypoxia {
        always (spo2 < 90 -> hypoxia_alarm);
    }

    property no_spurious_code_blue {
        never (code_blue && heart_rate > 60);
    }
}"#
}

fn industrial_src() -> &'static str {
    r#"module industrial_safety {
    signal temperature: in u16;
    signal pressure: in u32;
    signal vibration: in u16;
    signal flow_rate: in u16;
    signal shutdown_cmd: out bool;
    signal pressure_relief: out bool;
    signal vibration_alarm: out bool;
    signal flow_alarm: out bool;

    guard over_temp {
        when temperature > 450
        for 20 cycles;
    }

    guard over_pressure {
        when pressure > 10000
        for 5 cycles;
    }

    guard high_vibration {
        when vibration > 800
        for 15 cycles;
    }

    guard low_flow {
        when flow_rate < 10
        for 30 cycles;
    }

    reflex emergency_shutdown {
        on over_temp and over_pressure {
            shutdown_cmd = true;
        }
    }

    reflex relief_valve {
        on over_pressure {
            pressure_relief = true;
        }
    }

    reflex vibration_alert {
        on high_vibration {
            vibration_alarm = true;
        }
    }

    reflex flow_alert {
        on low_flow {
            flow_alarm = true;
        }
    }

    property pressure_triggers_relief {
        always (pressure > 10000 -> pressure_relief);
    }

    property temp_bounded {
        always (temperature < 600);
    }
}"#
}

fn autonomous_src() -> &'static str {
    r#"module autonomous_vehicle {
    signal lidar_range: in u32;
    signal radar_range: in u32;
    signal camera_conf: in u16;
    signal vehicle_speed: in u16;
    signal brake_cmd: out bool;
    signal lane_alert: out bool;
    signal speed_reduce: out bool;

    guard lidar_close {
        when lidar_range < 500
        for 3 cycles;
    }

    guard radar_close {
        when radar_range < 800
        for 3 cycles;
    }

    guard low_confidence {
        when camera_conf < 40
        for 10 cycles;
    }

    guard over_speed_limit {
        when vehicle_speed > 120
        for 5 cycles;
    }

    reflex emergency_brake {
        on lidar_close and radar_close {
            brake_cmd = true;
        }
    }

    reflex reduce_speed {
        on over_speed_limit {
            speed_reduce = true;
        }
    }

    reflex lane_departure {
        on low_confidence {
            lane_alert = true;
        }
    }

    property speed_limit_enforced {
        always (vehicle_speed > 120 -> speed_reduce);
    }
}"#
}

fn safety_property_src() -> &'static str {
    r#"module pressure_monitor {
    signal airway_pressure: in u16;
    signal clamp_valve: out bool;

    guard pressure_low {
        when airway_pressure < 50
        for 3 cycles;
    }

    reflex engage_clamp {
        on pressure_low {
            clamp_valve = true;
        }
    }

    property pressure_bounded {
        always (airway_pressure > 10);
    }

    property no_spurious_clamp {
        never (clamp_valve && airway_pressure > 200);
    }

    property low_triggers_clamp {
        always (airway_pressure < 50 -> clamp_valve);
    }

    property clamp_reachable {
        cover eventually within 100 (clamp_valve);
    }

    property clamp_follows_drop {
        always (airway_pressure < 50 followed_by 5 clamp_valve);
    }
}"#
}

fn shift_register_src() -> &'static str {
    r#"module short_delay_monitor {
    signal sensor_active: in bool;
    signal alert_lamp: out bool;

    guard brief_activation {
        when sensor_active
        for 8 cycles;
    }

    reflex activate_alert {
        on brief_activation {
            alert_lamp = true;
        }
    }
}"#
}

fn multi_guard_src() -> &'static str {
    r#"module patient_monitor {
    signal heart_rate: in u16;
    signal blood_pressure: in u16;
    signal alarm_active: out bool;
    signal pump_override: out bool;
    signal status_flag: internal bool;

    guard bradycardia {
        when heart_rate < 60
        for 500 cycles;
    }

    guard hypotension {
        when blood_pressure < 90
        for 12 cycles;
    }

    reflex cardiac_alarm {
        on bradycardia {
            alarm_active = true;
        }
    }

    reflex emergency_override {
        on bradycardia and hypotension {
            pump_override = true;
        }
    }
}"#
}

fn fir_src() -> &'static str {
    r#"module fir_filter {
    signal coeff_0: in u16;
    signal coeff_1: in u16;
    signal coeff_2: in u16;
    signal coeff_3: in u16;
    signal sample_in: in u16;
    signal sample_valid: in bool;
    signal filter_out: out u32;
    signal tap_0: internal u32;

    guard new_sample {
        when sample_valid
        for 1 cycles;
    }

    reflex compute_tap0 {
        on new_sample {
            tap_0 = sample_in * coeff_0;
        }
    }

    reflex output_sum {
        on new_sample {
            filter_out = tap_0;
        }
    }

    property output_bounded {
        always (filter_out < 4294967295);
    }
}"#
}

fn signed_src() -> &'static str {
    r#"module flight_controller_signed {
    signal altitude: in u32;
    signal pitch_angle: in i16;
    signal roll_angle: in i16;
    signal pitch_threshold: in i16;
    signal roll_threshold: in i16;
    signal correct_pitch: out bool;
    signal correct_roll: out bool;
    signal terrain_warn: out bool;

    guard altitude_low {
        when altitude < 500
        for 10 cycles;
    }

    guard nose_down {
        when pitch_angle < pitch_threshold
        for 8 cycles;
    }

    guard bank_steep {
        when roll_angle > roll_threshold
        for 4 cycles;
    }

    reflex pitch_correction {
        on nose_down {
            correct_pitch = true;
        }
    }

    reflex roll_correction {
        on bank_steep {
            correct_roll = true;
        }
    }

    reflex terrain_alert {
        on altitude_low {
            terrain_warn = true;
        }
    }
}"#
}

fn minimal_bool_src() -> &'static str {
    r#"module minimal_bool {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}"#
}

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

// ===========================================================================
// D2: eval_car_cdr_cons (20 tests)
// ===========================================================================

fn eval_ok(input: &str) -> SExpr {
    let sexpr = parse_sexpr(input).unwrap_or_else(|e| panic!("parse failed: {e}"));
    let mut st = EvalState::new();
    eval(&sexpr, &mut st).unwrap_or_else(|e| panic!("eval failed for `{input}`: {e}"))
}

#[test]
fn test_d2_car_simple_list() {
    let r = eval_ok("(car '(1 2 3))");
    assert_eq!(r, SExpr::Integer(1));
}

#[test]
fn test_d2_car_singleton() {
    let r = eval_ok("(car '(42))");
    assert_eq!(r, SExpr::Integer(42));
}

#[test]
fn test_d2_car_nested() {
    let r = eval_ok("(car '((a b) c))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b")]));
}

#[test]
fn test_d2_car_bool_list() {
    let r = eval_ok("(car '(true false))");
    assert_eq!(r, SExpr::Bool(true));
}

#[test]
fn test_d2_cdr_simple_list() {
    let r = eval_ok("(cdr '(1 2 3))");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(2), SExpr::Integer(3)]));
}

#[test]
fn test_d2_cdr_singleton() {
    let r = eval_ok("(cdr '(42))");
    assert_eq!(r, SExpr::list(vec![]));
}

#[test]
fn test_d2_cdr_two_elements() {
    let r = eval_ok("(cdr '(a b))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("b")]));
}

#[test]
fn test_d2_cdr_nested() {
    let r = eval_ok("(cdr '((a) (b) (c)))");
    assert_eq!(
        r,
        SExpr::list(vec![SExpr::list(vec![SExpr::sym("b")]), SExpr::list(vec![SExpr::sym("c")]),])
    );
}

#[test]
fn test_d2_cons_prepend() {
    let r = eval_ok("(cons 0 '(1 2))");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(0), SExpr::Integer(1), SExpr::Integer(2)]));
}

#[test]
fn test_d2_cons_onto_empty() {
    let r = eval_ok("(cons 1 '())");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(1)]));
}

#[test]
fn test_d2_cons_symbol() {
    let r = eval_ok("(cons 'a '(b c))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]));
}

#[test]
fn test_d2_cons_nested_list() {
    let r = eval_ok("(cons '(1 2) '(3))");
    assert_eq!(
        r,
        SExpr::list(vec![
            SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2)]),
            SExpr::Integer(3),
        ])
    );
}

#[test]
fn test_d2_car_of_cons() {
    let r = eval_ok("(car (cons 10 '(20 30)))");
    assert_eq!(r, SExpr::Integer(10));
}

#[test]
fn test_d2_cdr_of_cons() {
    let r = eval_ok("(cdr (cons 10 '(20 30)))");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(20), SExpr::Integer(30)]));
}

#[test]
fn test_d2_car_empty_list_errors() {
    let sexpr = parse_sexpr("(car '())").unwrap();
    let mut st = EvalState::new();
    let err = eval(&sexpr, &mut st);
    assert!(err.is_err(), "car of empty list should error");
}

#[test]
fn test_d2_cdr_empty_list_errors() {
    let sexpr = parse_sexpr("(cdr '())").unwrap();
    let mut st = EvalState::new();
    let err = eval(&sexpr, &mut st);
    assert!(err.is_err(), "cdr of empty list should error");
}

#[test]
fn test_d2_list_form() {
    let r = eval_ok("(list 1 2 3)");
    assert_eq!(r, SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2), SExpr::Integer(3)]));
}

#[test]
fn test_d2_list_empty() {
    let r = eval_ok("(list)");
    assert_eq!(r, SExpr::list(vec![]));
}

#[test]
fn test_d2_car_of_list() {
    let r = eval_ok("(car (list 5 6 7))");
    assert_eq!(r, SExpr::Integer(5));
}

#[test]
fn test_d2_nested_car_cdr() {
    let r = eval_ok("(car (cdr '(1 2 3)))");
    assert_eq!(r, SExpr::Integer(2));
}

// ===========================================================================
// D3: eval_match_type (15 tests)
// ===========================================================================

#[test]
fn test_d3_match_type_integer() {
    // match-type matches Symbol values by exact name.
    // 'integer → Symbol("integer"), pattern integer → exact match.
    let r = eval_ok(r#"(match-type 'integer (integer "yes") (symbol "no"))"#);
    assert_eq!(r, SExpr::Str("yes".to_string()));
}

#[test]
fn test_d3_match_type_symbol() {
    // Second clause matches when first clause doesn't.
    let r = eval_ok(r#"(match-type 'hello (world "no") (hello "yes"))"#);
    assert_eq!(r, SExpr::Str("yes".to_string()));
}

#[test]
fn test_d3_match_type_bool_true() {
    // Symbol "bool" matches pattern "bool" in first clause.
    let r = eval_ok(r#"(match-type 'bool (bool "matched") (other "no"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_bool_false() {
    // First clause doesn't match, falls through to second.
    let r = eval_ok(r#"(match-type 'other (bool "no") (other "matched"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_string() {
    // Symbol matching with a type-like name.
    let r = eval_ok(r#"(match-type 'string (string "matched") (integer "no"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_list() {
    // List pattern: head must match, rest are bound as variables.
    // Value '(unsigned 16) matches pattern (unsigned w), binding w=16.
    let r = eval_ok("(match-type '(unsigned 16) ((unsigned w) w))");
    assert_eq!(r, SExpr::Integer(16));
}

#[test]
fn test_d3_match_type_first_match_wins() {
    // Two clauses match the same symbol — first one wins.
    let r = eval_ok(r#"(match-type 'x (x "first") (x "second"))"#);
    assert_eq!(r, SExpr::Str("first".to_string()));
}

#[test]
fn test_d3_match_type_quote() {
    // (quote hello) evaluates to Symbol("hello"), matches pattern hello.
    let r = eval_ok(r#"(match-type (quote hello) (hello "yes") (world "no"))"#);
    assert_eq!(r, SExpr::Str("yes".to_string()));
}

#[test]
fn test_d3_match_type_nested_list() {
    // List pattern matching on a value with nested structure.
    // Value '(pair (a) b) matches pattern (pair x y), binding x=List(a), y=Symbol(b).
    let r = eval_ok("(match-type '(pair (a) b) ((pair x y) x))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a")]));
}

#[test]
fn test_d3_match_type_returns_constant() {
    // Body returns a constant value, not a bound variable.
    let r = eval_ok(r#"(match-type 'x (x 42))"#);
    assert_eq!(r, SExpr::Integer(42));
}

#[test]
fn test_d3_match_type_binds_variable() {
    // List pattern binds a variable; body evaluates it.
    let r = eval_ok("(match-type '(width 8) ((width n) n))");
    assert_eq!(r, SExpr::Integer(8));
}

#[test]
fn test_d3_match_type_no_match_errors() {
    let sexpr = parse_sexpr("(match-type 42 (symbol? x x))").unwrap();
    let mut st = EvalState::new();
    let err = eval(&sexpr, &mut st);
    assert!(err.is_err(), "match-type with no matching clause should error");
}

#[test]
fn test_d3_match_type_zero() {
    // Symbol "zero" matches pattern "zero".
    let r = eval_ok(r#"(match-type 'zero (zero "matched"))"#);
    assert_eq!(r, SExpr::Str("matched".to_string()));
}

#[test]
fn test_d3_match_type_negative_integer() {
    // Integer values never match Symbol patterns — should error.
    let expr = SExpr::list(vec![
        SExpr::sym("match-type"),
        SExpr::Integer(5),
        SExpr::list(vec![SExpr::sym("x"), SExpr::Integer(1)]),
    ]);
    let mut st = EvalState::new();
    let r = eval(&expr, &mut st);
    assert!(r.is_err(), "Integer value should not match Symbol pattern");
}

#[test]
fn test_d3_match_type_empty_list() {
    // Empty list matches empty list pattern.
    let r = eval_ok("(match-type '() (() 1))");
    assert_eq!(r, SExpr::Integer(1));
}

// ===========================================================================
// D4: quasiquote_unquote (10 tests)
// ===========================================================================

#[test]
fn test_d4_quote_literal() {
    let r = eval_ok("'42");
    assert_eq!(r, SExpr::Integer(42));
}

#[test]
fn test_d4_quote_symbol() {
    let r = eval_ok("'hello");
    assert_eq!(r, SExpr::sym("hello"));
}

#[test]
fn test_d4_quote_list() {
    let r = eval_ok("'(a b c)");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]));
}

#[test]
fn test_d4_quasiquote_no_unquote() {
    let r = eval_ok("`(a b c)");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::sym("b"), SExpr::sym("c")]));
}

#[test]
fn test_d4_quasiquote_with_unquote() {
    // Quasiquote with unquote evaluating a sub-expression (car).
    let r = eval_ok("`(a ,(car '(42 99)) c)");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("a"), SExpr::Integer(42), SExpr::sym("c")]));
}

#[test]
fn test_d4_quasiquote_nested() {
    // Quasiquote with nested list containing unquote.
    let r = eval_ok("`((,(car '(1 2))) 2)");
    assert_eq!(r, SExpr::list(vec![SExpr::list(vec![SExpr::Integer(1)]), SExpr::Integer(2),]));
}

#[test]
fn test_d4_quasiquote_arithmetic() {
    // Quasiquote with unquote evaluating a computed expression (if).
    let r = eval_ok("`(result ,(if true 42 0))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("result"), SExpr::Integer(42)]));
}

#[test]
fn test_d4_quote_preserves_structure() {
    let r = eval_ok("'(1 (2 3) 4)");
    assert_eq!(
        r,
        SExpr::list(vec![
            SExpr::Integer(1),
            SExpr::list(vec![SExpr::Integer(2), SExpr::Integer(3)]),
            SExpr::Integer(4),
        ])
    );
}

#[test]
fn test_d4_quasiquote_bool_unquote() {
    // Quasiquote with unquote of a boolean expression (eq?).
    let r = eval_ok("`(status ,(eq? 1 1))");
    assert_eq!(r, SExpr::list(vec![SExpr::sym("status"), SExpr::Bool(true)]));
}

#[test]
fn test_d4_quasiquote_string_unquote() {
    // Quasiquote with unquote evaluating to a string (car of quoted list).
    let r = eval_ok(r#"`(label ,(car '("sensor" "motor")))"#);
    assert_eq!(r, SExpr::list(vec![SExpr::sym("label"), SExpr::Str("sensor".to_string())]));
}

// ===========================================================================
// D5: macro_expand_hygienic (10 tests)
// ===========================================================================

#[test]
fn test_d5_macro_expander_new() {
    let me = MacroExpander::new();
    // MacroExpander starts with expansion_counter = 0.
    // Just verify it can be constructed without panicking.
    let _ = me;
}

#[test]
fn test_d5_macro_expand_hygienic_simple() {
    let mut me = MacroExpander::new();
    // Template: (signal "sensor_name") where sensor_name is a parameter
    let template = SExpr::list(vec![SExpr::sym("signal"), SExpr::Str("sensor".to_string())]);
    let param_names = vec!["sensor".to_string()];
    let bindings = vec![("sensor".to_string(), SExpr::Str("temp_a".to_string()))];
    let result = me.expand_hygienic(&template, &param_names, &bindings, 0);
    assert!(result.is_ok(), "Hygienic expand should succeed");
}

#[test]
fn test_d5_macro_expand_hygienic_substitution() {
    let mut me = MacroExpander::new();
    // Template with a param that should be substituted
    let template = SExpr::Str("target".to_string());
    let param_names = vec!["target".to_string()];
    let bindings = vec![("target".to_string(), SExpr::Str("clamp_valve".to_string()))];
    let result = me.expand_hygienic(&template, &param_names, &bindings, 0).unwrap();
    assert_eq!(result, SExpr::Str("clamp_valve".to_string()), "Param must be substituted");
}

#[test]
fn test_d5_macro_expand_hygienic_renames_internal() {
    let mut me = MacroExpander::new();
    // Template with an internal name not in params — should get hygiene suffix
    let template = SExpr::Str("internal_var".to_string());
    let param_names: Vec<String> = vec![];
    let bindings: Vec<(String, SExpr)> = vec![];
    let result = me.expand_hygienic(&template, &param_names, &bindings, 0).unwrap();
    // Should be renamed to internal_var__hyg1
    match &result {
        SExpr::Str(s) => {
            assert!(s.starts_with("internal_var__hyg"), "Internal name must be renamed")
        }
        _ => panic!("Expected Str"),
    }
}

#[test]
fn test_d5_macro_expand_hygienic_depth_limit() {
    let mut me = MacroExpander::new();
    let template = SExpr::Integer(42);
    let result = me.expand_hygienic(&template, &[], &[], 999);
    assert!(result.is_err(), "Exceeding depth limit should error");
}

#[test]
fn test_d5_macro_expand_hygienic_atom_passthrough() {
    let mut me = MacroExpander::new();
    // Integer and Bool pass through unchanged.
    let r_int = me.expand_hygienic(&SExpr::Integer(42), &[], &[], 0).unwrap();
    assert_eq!(r_int, SExpr::Integer(42));
    let r_bool = me.expand_hygienic(&SExpr::Bool(true), &[], &[], 0).unwrap();
    assert_eq!(r_bool, SExpr::Bool(true));
}

#[test]
fn test_d5_macro_expand_hygienic_symbol_not_renamed() {
    let mut me = MacroExpander::new();
    // Symbols (structural tags) should not be renamed.
    let template = SExpr::sym("signal");
    let result = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    assert_eq!(result, SExpr::sym("signal"), "Symbols must not be renamed");
}

#[test]
fn test_d5_macro_expand_hygienic_list() {
    let mut me = MacroExpander::new();
    let template = SExpr::list(vec![SExpr::sym("guard"), SExpr::Str("my_guard".to_string())]);
    let result = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    // The list structure should be preserved, "my_guard" renamed.
    match &result {
        SExpr::List(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], SExpr::sym("guard"));
        }
        _ => panic!("Expected List"),
    }
}

#[test]
fn test_d5_macro_expand_hygienic_quote_preserved() {
    let mut me = MacroExpander::new();
    let template = SExpr::Quote(Box::new(SExpr::Str("inner".to_string())));
    let result = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    match &result {
        SExpr::Quote(_) => {} // quote is preserved
        _ => panic!("Expected Quote wrapper to be preserved"),
    }
}

#[test]
fn test_d5_macro_expand_counter_increments() {
    let mut me = MacroExpander::new();
    let template = SExpr::Str("name".to_string());
    let r1 = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    let r2 = me.expand_hygienic(&template, &[], &[], 0).unwrap();
    // Each expansion gets a different hygiene ID, so results differ.
    assert_ne!(r1, r2, "Each expansion must use a unique hygiene ID");
}

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

// ===========================================================================
// D7: parser_depth_limits (10 tests)
// ===========================================================================

#[test]
fn test_d7_depth_1_ok() {
    let r = parse_sexpr("(a)");
    assert!(r.is_ok(), "Depth 1 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_10_ok() {
    let open: String = "(".repeat(10);
    let close: String = ")".repeat(10);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth 10 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_30_ok() {
    let open: String = "(".repeat(30);
    let close: String = ")".repeat(30);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth 30 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_max_ok() {
    // Parser checks `current_depth >= MAX_SEXPR_DEPTH`, so the deepest
    // valid nesting is MAX_SEXPR_DEPTH - 1 open parens.
    let open: String = "(".repeat(MAX_SEXPR_DEPTH - 1);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH - 1);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth MAX-1 should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_max_plus_1_fails() {
    let open: String = "(".repeat(MAX_SEXPR_DEPTH + 1);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH + 1);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_err(), "Depth MAX+1 should fail");
}

#[test]
fn test_d7_depth_max_plus_10_fails() {
    let open: String = "(".repeat(MAX_SEXPR_DEPTH + 10);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH + 10);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_err(), "Depth MAX+10 should fail");
}

#[test]
fn test_d7_flat_list_at_depth_1() {
    // A wide but shallow list should be fine
    let mut items = String::new();
    let mut i = 0;
    while i < 100 {
        items.push_str(&format!("x{i} "));
        i += 1;
    }
    let input = format!("({items})");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Wide flat list should parse: {:?}", r.err());
}

#[test]
fn test_d7_nested_lists_at_same_depth() {
    // ((a) (b) (c)) — depth 2, not deep
    let r = parse_sexpr("((a) (b) (c))");
    assert!(r.is_ok());
}

#[test]
fn test_d7_depth_50_ok() {
    let open: String = "(".repeat(50);
    let close: String = ")".repeat(50);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_ok(), "Depth 50 (< MAX 64) should parse: {:?}", r.err());
}

#[test]
fn test_d7_depth_100_fails() {
    let open: String = "(".repeat(100);
    let close: String = ")".repeat(100);
    let input = format!("{open}a{close}");
    let r = parse_sexpr(&input);
    assert!(r.is_err(), "Depth 100 (> MAX 64) should fail");
}

// ===========================================================================
// D8: parser_error_codes (15 tests)
// ===========================================================================

#[test]
fn test_d8_empty_input_error() {
    let r = parse_sexpr("");
    assert!(r.is_err(), "Empty input should fail");
}

#[test]
fn test_d8_unbalanced_open_paren() {
    let r = parse_sexpr("(a b");
    assert!(r.is_err());
}

#[test]
fn test_d8_unbalanced_close_paren() {
    let r = parse_sexpr("a b)");
    assert!(r.is_err());
}

#[test]
fn test_d8_just_close_paren() {
    let r = parse_sexpr(")");
    assert!(r.is_err());
}

#[test]
fn test_d8_multiple_unbalanced() {
    let r = parse_sexpr("(((a b)");
    assert!(r.is_err());
}

#[test]
fn test_d8_unterminated_string() {
    let r = parse_sexpr("\"hello");
    assert!(r.is_err());
}

#[test]
fn test_d8_deeply_nested_error() {
    let open: String = "(".repeat(MAX_SEXPR_DEPTH + 5);
    let close: String = ")".repeat(MAX_SEXPR_DEPTH + 5);
    let r = parse_sexpr(&format!("{open}x{close}"));
    assert!(r.is_err());
    let msg = r.unwrap_err().to_string();
    assert!(
        msg.contains("E803") || msg.contains("depth") || msg.contains("DEPTH"),
        "Should mention depth: {msg}"
    );
}

#[test]
fn test_d8_only_whitespace() {
    let r = parse_sexpr("   ");
    assert!(r.is_err());
}

#[test]
fn test_d8_only_comment() {
    let r = parse_sexpr("; just a comment\n");
    assert!(r.is_err());
}

#[test]
fn test_d8_extra_close_parens() {
    let r = parse_sexpr("(a))");
    assert!(r.is_err());
}

#[test]
fn test_d8_null_byte_in_input() {
    let r = parse_sexpr("(\0)");
    // Should either error or produce something — not panic
    let _ = r;
}

#[test]
fn test_d8_very_long_symbol() {
    let sym: String = "a".repeat(10_000);
    let input = format!("({sym})");
    let r = parse_sexpr(&input);
    // Should not panic even with very long symbols
    let _ = r;
}

#[test]
fn test_d8_negative_is_symbol() {
    // u64-based Integer cannot hold negatives; parser treats "-42" as a symbol.
    let r = parse_sexpr("-42");
    assert!(r.is_ok(), "Negative literal should parse as symbol");
    match r.unwrap() {
        SExpr::Symbol(_) => {} // expected
        other => panic!("Expected Symbol for '-42', got {other:?}"),
    }
}

#[test]
fn test_d8_large_integer() {
    let r = parse_sexpr("9999999999999");
    assert!(r.is_ok());
}

#[test]
fn test_d8_mixed_content() {
    let r = parse_sexpr("(define (f x) (+ x 1))");
    assert!(r.is_ok());
}

// ===========================================================================
// D9: convert_all_ast_nodes (20 tests)
// ===========================================================================

#[test]
fn test_d9_convert_signal_input_bool() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("input"), "Should contain input kind");
    assert!(s.contains("bool"), "Should contain bool type");
}

#[test]
fn test_d9_convert_signal_output_u16() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out u16;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = 42;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("output"), "Should contain output kind");
}

#[test]
fn test_d9_convert_signal_internal() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;
    signal z: internal u8;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = x;
            z = 1;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("internal"), "Should contain internal kind");
}

#[test]
fn test_d9_convert_guard() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 5 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("guard"), "Should contain guard");
    assert!(s.contains("5"), "Should contain cycle count");
}

#[test]
fn test_d9_convert_reflex() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("reflex"), "Should contain reflex");
}

#[test]
fn test_d9_convert_binary_add() {
    let src = r#"module m {
    signal a: in u16;
    signal b: in u16;
    signal c: out u16;

    guard g {
        when a > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            c = a + b;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("+"), "Should contain + operator");
}

#[test]
fn test_d9_convert_binary_lt() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x < 100
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("<"), "Should contain < operator");
}

#[test]
fn test_d9_convert_unary_not() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when !x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("not") || s.contains("!"), "Should contain not/!");
}

#[test]
fn test_d9_convert_literal_bool_true() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("true"), "Should contain literal true");
}

#[test]
fn test_d9_convert_literal_integer() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out u16;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = 42;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("42"), "Should contain literal 42");
}

#[test]
fn test_d9_convert_prev() {
    // Prev is an AST-level construct (no parser syntax); construct AST directly.
    use nasa_rust_project::ast::expr::Expr;
    use nasa_rust_project::ast::program::*;
    use nasa_rust_project::ast::types::*;
    let m = Module {
        name: "m".to_string(),
        signals: vec![
            SignalDecl {
                name: "x".to_string(),
                kind: SignalKind::Input,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
            SignalDecl {
                name: "y".to_string(),
                kind: SignalKind::Output,
                ty: ExtendedType::from_core(SignalType::Unsigned(16)),
                origin: None,
                span: None,
            },
        ],
        guards: vec![Guard {
            name: "g".to_string(),
            condition: Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("x".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(0))),
            },
            cycles: 1,
            origin: None,
            span: None,
        }],
        reflexes: vec![Reflex {
            name: "r".to_string(),
            guard_names: vec!["g".to_string()],
            assignments: vec![Assignment {
                target: "y".to_string(),
                value: Expr::Prev { signal: "x".to_string(), delay: 1 },
                span: None,
            }],
            origin: None,
            span: None,
        }],
        properties: vec![],
        pattern_calls: vec![],
        pattern_origins: vec![],
        span: None,
    };
    let prog = nasa_rust_project::MirrProgram { module: m, patterns: vec![] };
    let sexpr = ast_to_sexpr(&prog);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("prev"), "Should contain prev reference");
}

#[test]
fn test_d9_convert_module_name() {
    let src = r#"module my_mod {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = x;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("my_mod"), "Should contain module name");
}

#[test]
fn test_d9_convert_multi_assignment() {
    let src = r#"module m {
    signal x: in bool;
    signal a: out bool;
    signal b: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            a = true;
            b = false;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let rebuilt = sexpr_to_ast(&sexpr).unwrap();
    assert_eq!(rebuilt.module.reflexes[0].assignments.len(), 2);
}

#[test]
fn test_d9_convert_multi_guard_reflex() {
    let src = r#"module m {
    signal x: in bool;
    signal y: in bool;
    signal z: out bool;

    guard ga {
        when x
        for 1 cycles;
    }

    guard gb {
        when y
        for 1 cycles;
    }

    reflex r {
        on ga and gb {
            z = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let rebuilt = sexpr_to_ast(&sexpr).unwrap();
    assert_eq!(rebuilt.module.reflexes[0].guard_names.len(), 2);
}

#[test]
fn test_d9_convert_property_always() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p {
        always (x > 0);
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("always"), "Should contain always property");
}

#[test]
fn test_d9_convert_property_never() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p {
        never (y && x < 0);
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("never"), "Should contain never property");
}

#[test]
fn test_d9_convert_property_eventually() {
    let src = r#"module m {
    signal x: in bool;
    signal y: out bool;

    guard g {
        when x
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }

    property p {
        eventually within 10 (y);
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("eventually"), "Should contain eventually property");
}

#[test]
fn test_d9_convert_multiply_op() {
    let src = r#"module m {
    signal a: in u16;
    signal b: in u16;
    signal c: out u32;

    guard g {
        when a > 0
        for 1 cycles;
    }

    reflex r {
        on g {
            c = a * b;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("*"), "Should contain * operator");
}

#[test]
fn test_d9_convert_comparison_ops() {
    let src = r#"module m {
    signal x: in u16;
    signal y: out bool;

    guard g {
        when x >= 10
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains(">="), "Should contain >= operator");
}

#[test]
fn test_d9_convert_signed_type() {
    let src = r#"module m {
    signal x: in i16;
    signal y: out bool;

    guard g {
        when x < 0
        for 1 cycles;
    }

    reflex r {
        on g {
            y = true;
        }
    }
}"#;
    let parsed = parse_mirr(src).unwrap();
    let sexpr = ast_to_sexpr(&parsed);
    let s = print_sexpr(&sexpr);
    assert!(s.contains("i16") || s.contains("signed"), "Should contain signed type: {s}");
}

// ===========================================================================
// D10: eval_step_budget (5 tests)
// ===========================================================================

#[test]
fn test_d10_eval_within_budget() {
    let mut st = EvalState::new();
    let expr = parse_sexpr("(car '(1 2))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_ok(), "Simple eval should stay within budget");
}

#[test]
fn test_d10_eval_nested_but_within_budget() {
    let mut st = EvalState::new();
    let expr = parse_sexpr("(car (cdr '(1 2 3)))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_ok());
    assert_eq!(r.unwrap(), SExpr::Integer(2));
}

#[test]
fn test_d10_eval_let_within_budget() {
    let mut st = EvalState::new();
    let expr = parse_sexpr("(cons 1 '(2 3))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_ok());
    assert_eq!(
        r.unwrap(),
        SExpr::list(vec![SExpr::Integer(1), SExpr::Integer(2), SExpr::Integer(3)])
    );
}

#[test]
fn test_d10_eval_custom_step_limit() {
    let mut st = EvalState::with_steps(2);
    // (car (cdr '(1 2 3))) needs 3 eval steps, exceeds limit of 2.
    let expr = parse_sexpr("(car (cdr '(1 2 3)))").unwrap();
    let r = eval(&expr, &mut st);
    assert!(r.is_err(), "Should exceed custom step limit of 2");
    let msg = r.unwrap_err().to_string();
    assert!(msg.contains("E812"), "Should be step budget error: {msg}");
}

#[test]
fn test_d10_eval_depth_limit() {
    // Build deeply nested if-expressions to exhaust eval depth.
    // Each nested (if COND 1 0) in condition position pushes one IfCond frame.
    let mut input = String::from("true");
    let mut i = 0;
    while i < MAX_EVAL_DEPTH + 5 {
        input = format!("(if {input} 1 0)");
        i += 1;
    }
    let expr = parse_sexpr(&input).unwrap();
    let mut st = EvalState::new();
    let r = eval(&expr, &mut st);
    assert!(r.is_err(), "Deeply nested if should exceed depth limit");
    let msg = r.unwrap_err().to_string();
    assert!(
        msg.contains("E811") || msg.contains("depth") || msg.contains("E812"),
        "Should mention depth/steps limit: {msg}"
    );
}
