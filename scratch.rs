#[cfg(test)]
mod tests {
    use super::*;
    use crate::mape_k::ltl::SignalPredicate;
    use crate::pipeline::PipelineResult;

    fn compile_test_source(source: &str) -> PipelineResult {
        let reg = crate::parser::ecs_parser::parse_mirr_ecs_with_base_dir(source, None).unwrap();
        PipelineResult {
            program: None,
            simplify_stats: None,
            sat_stats: None,
            width_stats: None,
            width_diagnostics: Vec::new(),
            temporal_netlist: None,
            rspu_program: None,
            extended_type_map: None,
            sim_result: None,
            mape_k_result: None,
            retiming_stats: None,
            totality_result: None,
            symbolic_result: None,
            mape_k_rtl: None,
            hls_result: None,
            ecs_registry: Some(reg),
            file_table: crate::span::FileTable::new(),
        }
    }

    #[test]
    fn empty_module_produces_empty_config() {
        let result = compile_test_source("module test_mod { }");
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert!(config.sensors.is_empty());
        assert!(config.properties.is_empty());
        assert!(config.action_table.is_empty());
        assert_eq!(config.window_size, DEFAULT_WINDOW_SIZE);
        assert_eq!(config.knowledge_capacity, DEFAULT_KNOWLEDGE_CAPACITY);
    }

    #[test]
    fn all_signals_become_sensors_with_observable_flag() {
        let src = "
        module test_mod {
            signal pressure: in u8;
            signal alarm: out bool;
            signal temp: in u16;
        }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.sensors.len(), 3);
        let pressure = config.sensors.iter().find(|s| s.name == "pressure").unwrap();
        let alarm = config.sensors.iter().find(|s| s.name == "alarm").unwrap();
        let temp = config.sensors.iter().find(|s| s.name == "temp").unwrap();
        assert!(pressure.is_observable);
        assert!(!alarm.is_observable);
        assert!(temp.is_observable);
    }

    #[test]
    fn bool_sensor_defaults() {
        let result = compile_test_source("module test_mod { signal flag: in bool; }");
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.sensors[0].base_value, 1);
        assert_eq!(config.sensors[0].noise_amplitude, 0);
    }

    #[test]
    fn unsigned_sensor_midpoint() {
        let result = compile_test_source("module test_mod { signal data: in u8; }");
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.sensors[0].base_value, 127);
        assert_eq!(config.sensors[0].noise_amplitude, sensors::DEFAULT_NOISE_AMPLITUDE);
    }

    #[test]
    fn always_signal_lowers_to_always_is_true() {
        let src = "module test_mod { signal alive: in bool; property p1: assert always alive; }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.properties.len(), 1);
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::IsTrue("alive".to_string()))
        );
    }

    #[test]
    fn never_signal_lowers_to_always_less_than_one() {
        let src = "module test_mod { signal fault: in bool; property p_never: assert never fault; }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.properties.len(), 1);
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::LessThan("fault".to_string(), 1))
        );
    }

    #[test]
    fn eventually_within_lowers_correctly() {
        let src = "module test_mod { signal ready: in bool; property p_ev: assert eventually within 10 cycles ready; }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.properties.len(), 1);
        assert_eq!(
            config.properties[0],
            TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("ready".to_string()), 10)
        );
    }

    #[test]
    fn cover_and_assume_are_skipped() {
        let src = "module test_mod { 
            signal x: in bool; 
            signal y: in bool; 
            property c1: cover always x;
            property a1: assume always y;
        }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert!(config.properties.is_empty());
        assert!(config.action_table.is_empty());
    }

    #[test]
    fn always_implies_lowers_to_temporal_property() {
        let src = "module test_mod { 
            signal a: in bool; 
            signal b: in bool; 
            property p_impl: assert always a implies b;
        }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("AlwaysImplies should lower");
        assert_eq!(config.properties.len(), 1);
        assert_eq!(
            config.properties[0],
            TemporalProperty::AlwaysImplies(
                SignalPredicate::IsTrue("a".to_string()),
                SignalPredicate::IsTrue("b".to_string()),
            )
        );
    }

    #[test]
    fn action_table_graduated_by_property_kind() {
        let src = "module test_mod { 
            signal a: in bool; 
            signal b: in bool; 
            property p1: assert always a;
            property p2: assert eventually within 5 cycles b;
        }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.action_table.len(), 2);
        assert_eq!(config.action_table[0].trigger_property_idx, 0);
        assert_eq!(config.action_table[0].action, AdaptationAction::EmergencyStop);
        assert_eq!(config.action_table[0].priority, 200);
        assert_eq!(config.action_table[1].trigger_property_idx, 1);
        assert_eq!(config.action_table[1].action, AdaptationAction::Throttle);
        assert_eq!(config.action_table[1].priority, 128);
    }

    #[test]
    fn action_table_reduces_on_implies_properties() {
        let src = "module test_mod { 
            signal a: in bool; 
            signal b: in bool; 
            property p_impl: assert always a implies b;
        }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.action_table.len(), 1);
        assert_eq!(config.action_table[0].action, AdaptationAction::Reduce);
        assert_eq!(config.action_table[0].priority, 100);
    }

    #[test]
    fn too_many_signals_produces_error() {
        let mut src = String::from("module test_mod { ");
        for i in 0..MAX_BRIDGE_SIGNALS + 1 {
            src.push_str(&format!("signal s{}: in u8; ", i));
        }
        src.push_str("}");
        let result = compile_test_source(&src);
        let err = bridge_from_pipeline(&result).expect_err("should fail");
        assert!(err.iter().any(|e| matches!(e, MapeKError::BridgeConfigError(_))));
    }

    #[test]
    fn max_unsigned_value_edge_cases() {
        assert_eq!(sensors::max_unsigned_value(0), 0);
        assert_eq!(sensors::max_unsigned_value(1), 1);
        assert_eq!(sensors::max_unsigned_value(8), 255);
        assert_eq!(sensors::max_unsigned_value(16), 65535);
        assert_eq!(sensors::max_unsigned_value(64), u64::MAX);
    }

    #[test]
    fn extract_signal_from_nested_expr() {
        let src = "module m { signal deep_signal: in bool; property p1: assert always !(deep_signal && true); }";
        let result = compile_test_source(src);
        let reg = result.ecs_registry.as_ref().unwrap();
        // find the property p1 and extract the formula expr
        let mut formula_ent = None;
        for ent in reg.get_all_entities() {
            if let Some(prop) = reg.get_property_component(ent) {
                if prop.name == "p1" {
                    formula_ent = Some(prop.formula_expr);
                }
            }
        }
        let name = properties::extract_signal_name_ecs(formula_ent.unwrap(), reg).expect("should find signal");
        assert_eq!(name, "deep_signal");
    }

    #[test]
    fn binary_lt_lowers_to_less_than() {
        let src = "module test_mod { signal pressure: in u8; property p_lt: assert always pressure < 100; }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::LessThan("pressure".to_string(), 100))
        );
    }

    #[test]
    fn binary_gt_lowers_to_greater_than() {
        let src = "module test_mod { signal rate: in u8; property p_gt: assert always rate > 50; }";
        let result = compile_test_source(src);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::GreaterThan("rate".to_string(), 50))
        );
    }
}
