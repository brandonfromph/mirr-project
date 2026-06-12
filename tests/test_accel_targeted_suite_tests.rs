#![forbid(unsafe_code)]

use std::fs;
use std::path::Path;

fn fixture_paths() -> [&'static str; 10] {
    [
        "tests/fixtures/netlist/struct_packet.json",
        "tests/fixtures/netlist/array_register_file.json",
        "tests/fixtures/netlist/tmr_sensor_fusion.json",
        "tests/fixtures/netlist/watchdog_timer.json",
        "tests/fixtures/netlist/flight_controller.json",
        "tests/fixtures/netlist/industrial_safety.json",
        "tests/fixtures/netlist/thermal_management.json",
        "tests/fixtures/netlist/comm_watchdog.json",
        "tests/fixtures/netlist/fixed_point_control.json",
        "tests/fixtures/netlist/interface_bundle_monitor.json",
    ]
}

fn read_fixture(path: &str) -> serde_json::Value {
    let full_path = Path::new(path);
    let text = fs::read_to_string(full_path)
        .unwrap_or_else(|_| panic!("fixture must be readable: {}", full_path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|_| panic!("fixture must be valid JSON: {}", full_path.display()))
}

#[test]
#[ignore]
fn targeted_suite_all_wave6_fixtures_exist() {
    for path in fixture_paths() {
        assert!(Path::new(path).exists(), "missing fixture: {path}");
    }
}

#[test]
#[ignore]
fn targeted_suite_fixtures_have_required_top_level_keys() {
    for path in fixture_paths() {
        let value = read_fixture(path);
        assert_eq!(value["ir_version"], "2.0", "{path} must use IR 2.0");
        assert!(value["guards"].is_array(), "{path} guards must be array");
        assert!(value["signals"].is_array(), "{path} signals must be array");
        assert!(value["statistics"].is_object(), "{path} statistics must be object");
    }
}

#[test]
#[ignore]
fn targeted_suite_fixtures_have_statistics_contract_fields() {
    for path in fixture_paths() {
        let value = read_fixture(path);
        let stats = &value["statistics"];

        for key in [
            "shift_registers_used",
            "counters_used",
            "logic_gates_used",
            "max_delay_cycles",
            "total_signals",
        ] {
            assert!(stats[key].is_number(), "{path} missing numeric statistics.{key}");
        }
    }
}

#[test]
#[ignore]
fn targeted_suite_statistics_total_matches_signal_count() {
    for path in fixture_paths() {
        let value = read_fixture(path);
        let signals = value["signals"].as_array().expect("signals must be array");
        let total = value["statistics"]["total_signals"]
            .as_u64()
            .unwrap_or_else(|| panic!("{path} statistics.total_signals must be u64"));

        assert_eq!(
            total as usize,
            signals.len(),
            "{path} statistics.total_signals must equal signals.len()"
        );
    }
}

#[test]
#[ignore]
fn targeted_suite_signal_entries_have_non_empty_name_and_kind() {
    for path in fixture_paths() {
        let value = read_fixture(path);
        let signals = value["signals"].as_array().expect("signals must be array");

        for signal in signals {
            let name = signal["name"].as_str().unwrap_or_default();
            let kind = signal["kind"].as_str().unwrap_or_default();
            assert!(!name.trim().is_empty(), "{path} signal.name must be non-empty");
            assert!(!kind.trim().is_empty(), "{path} signal.kind must be non-empty");
        }
    }
}

#[test]
#[ignore]
fn targeted_suite_counter_guard_statistics_are_consistent() {
    for path in fixture_paths() {
        let value = read_fixture(path);
        let guards = value["guards"].as_array().expect("guards must be array");
        let has_counter_guard = guards
            .iter()
            .any(|g| g.as_object().map(|o| o.contains_key("Counter")).unwrap_or(false));

        if has_counter_guard {
            let counters_used = value["statistics"]["counters_used"]
                .as_u64()
                .unwrap_or_else(|| panic!("{path} statistics.counters_used must be u64"));
            let max_delay_cycles = value["statistics"]["max_delay_cycles"]
                .as_u64()
                .unwrap_or_else(|| panic!("{path} statistics.max_delay_cycles must be u64"));

            assert!(counters_used >= 1, "{path} has Counter guard but counters_used == 0");
            assert!(max_delay_cycles >= 1, "{path} has Counter guard but max_delay_cycles == 0");
        }
    }
}
