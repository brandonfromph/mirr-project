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

use mirrc::sexpr::types::SExpr;
use mirrc::sexpr::{
    ast_to_sexpr, eval, parse_sexpr, print_sexpr, sexpr_to_ast, EvalState, MacroExpander,
    ReaderMacroRegistry, MAX_EVAL_DEPTH, MAX_SEXPR_DEPTH,
};
use mirrc::{parse_mirr, validate_module};

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

fn eval_ok(input: &str) -> SExpr {
    let sexpr = parse_sexpr(input).unwrap_or_else(|e| panic!("parse failed: {e}"));
    let mut st = EvalState::new();
    eval(&sexpr, &mut st).unwrap_or_else(|e| panic!("eval failed for `{input}`: {e}"))
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

mod d10_tests;
mod d1_tests;
mod d2_tests;
mod d3_tests;
mod d4_tests;
mod d5_tests;
mod d6_tests;
mod d7_tests;
mod d8_tests;
mod d9_tests;
