#![forbid(unsafe_code)]

//! Bridge from compiler `PipelineResult` to MAPE-K `SimConfig`.
//!
//! Converts the compiler's AST-level property definitions and signal
//! declarations into the MAPE-K simulator's configuration format.
//!
//! # Lowering strategy
//!
//! - **Signals**: Each `SignalDecl` in the module becomes a `SensorConfig`
//!   with heuristic defaults based on the signal's type and bit-width.
//! - **Properties**: Only `PropertyDirective::Assert` formulas are lowered.
//!   `Cover` and `Assume` are skipped (they are verification-only and do
//!   not map to runtime safety monitors). Formulas that cannot be lowered
//!   to a single `SignalPredicate` are lowered conservatively.
//! - **Action table**: Graduated by property kind — `Always`/`Persists` →
//!   `EmergencyStop` (priority 200); `EventuallyWithin` → `Throttle` (128).
//!
//! All iteration is bounded by `MAX_BRIDGE_SIGNALS` / `MAX_BRIDGE_PROPERTIES`
//! constants (NASA Power-of-10 rule #2).

mod properties;
mod sensors;

use properties::extract_properties;
use sensors::extract_sensors;
use serde::{Deserialize, Serialize};

use crate::mape_k::error::MapeKError;
use crate::mape_k::ltl::TemporalProperty;
use crate::mape_k::planner::{ActionEntry, AdaptationAction, TriggerCondition};
use crate::mape_k::SimConfig;
use crate::pipeline::PipelineResult;

// ---------------------------------------------------------------------------
// Bridge types — resolving E801 refinement gaps
// ---------------------------------------------------------------------------

/// A BridgeSignal represents a telemetry channel from a specific core.
///
/// Proposed in Proposal 045 for cross-core coordination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSignal {
    /// Canonical signal name.
    pub name: String,
    /// Core identifier (0..15 for RS-16).
    pub core_id: u32,
    /// Bit-width of the signal.
    pub width: u32,
}

/// The Bridge manages the collection of signals across the multi-core fabric.
///
/// Proposed in Proposal 045 for aggregating telemetry into MAPE-K.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Bridge {
    /// Registered telemetry signals.
    pub signals: Vec<BridgeSignal>,
    /// Maximum capacity of the bridge (bounded resource).
    pub capacity: usize,
}

// ---------------------------------------------------------------------------
// Constants — bounded resource limits (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum signals the bridge will process.
pub const MAX_BRIDGE_SIGNALS: usize = 256;

/// Maximum properties the bridge will convert.
pub const MAX_BRIDGE_PROPERTIES: usize = 64;

/// Default rolling window size for the monitor.
pub const DEFAULT_WINDOW_SIZE: usize = 64;

/// Default knowledge base capacity.
pub const DEFAULT_KNOWLEDGE_CAPACITY: usize = 4096;

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Convert a `PipelineResult` into a MAPE-K `SimConfig`.
///
/// Steps:
/// 1. Extract input signals from the AST module into `SensorConfig` entries
///    with heuristic defaults derived from each signal's type.
/// 2. Lower `Assert` properties to `TemporalProperty` (skip `Cover`/`Assume`).
/// 3. Generate a graduated action table: `Always`/`Persists` → `EmergencyStop`
///    (priority 200); `EventuallyWithin` → `Throttle` (priority 128).
/// 4. Apply default `window_size` and `knowledge_capacity`.
pub fn bridge_from_pipeline(result: &PipelineResult) -> Result<SimConfig, Vec<MapeKError>> {
    let mut errors: Vec<MapeKError> = Vec::new();

    let sensors = extract_sensors(result, &mut errors);
    let properties = extract_properties(result, &mut errors);

    if !errors.is_empty() {
        return Err(errors);
    }

    let action_table = generate_action_table(&properties);

    Ok(SimConfig {
        sensors,
        properties,
        action_table,
        window_size: DEFAULT_WINDOW_SIZE,
        knowledge_capacity: DEFAULT_KNOWLEDGE_CAPACITY,
    })
}

// ---------------------------------------------------------------------------
// Action table generation
// ---------------------------------------------------------------------------

/// Generate a graduated action table: violation severity determines response.
///
/// - `Always` / `Persists`: safety invariant → `EmergencyStop` priority 200.
/// - `EventuallyWithin`: soft timing constraint → `Throttle` priority 128.
fn generate_action_table(properties: &[TemporalProperty]) -> Vec<ActionEntry> {
    let mut entries = Vec::with_capacity(properties.len());

    for (idx, prop) in properties.iter().enumerate().take(MAX_BRIDGE_PROPERTIES) {
        let (action, priority) = match prop {
            TemporalProperty::Always(_) | TemporalProperty::Persists(_, _) => {
                (AdaptationAction::EmergencyStop, 200u8)
            }
            TemporalProperty::EventuallyWithin(_, _) => (AdaptationAction::Throttle, 128u8),
            TemporalProperty::AlwaysFollowedBy(_, _, _) => (AdaptationAction::LogWarning, 64u8),
            TemporalProperty::AlwaysImplies(_, _) | TemporalProperty::NeverImplies(_, _) => {
                (AdaptationAction::Reduce, 100u8)
            }
        };
        entries.push(ActionEntry {
            trigger_property_idx: idx,
            action,
            priority,
            trigger_on: TriggerCondition::OnViolation,
        });
    }

    entries
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::{MirrProgram, Module};
    use crate::ast::property::{PropertyDecl, PropertyDirective, PropertyFormula};
    use crate::ast::types::{ExtendedType, SignalKind, SignalType};
    use crate::ast::Expr;
    use crate::ast::SignalDecl;
    use crate::mape_k::ltl::SignalPredicate;

    fn stub_pipeline(signals: Vec<SignalDecl>, properties: Vec<PropertyDecl>) -> PipelineResult {
        let module = Module {
            name: "test_mod".to_string(),
            signals,
            guards: Vec::new(),
            reflexes: Vec::new(),
            properties,
            pattern_calls: Vec::new(),
            pattern_origins: Vec::new(),
            span: None,
        };
        PipelineResult {
            program: MirrProgram { patterns: Vec::new(), imports: Vec::new(), module },
            simplify_stats: None,
            sat_stats: None,
            width_result: None,
            temporal_netlist: None,
            rspu_program: None,
            type_map: None,
            extended_type_map: None,
            sim_result: None,
            mape_k_result: None,
            retiming_stats: None,
            totality_result: None,
            symbolic_result: None,
            mape_k_rtl: None,
            hls_result: None,
        }
    }

    fn input_signal(name: &str, ty: SignalType) -> SignalDecl {
        SignalDecl {
            name: name.to_string(),
            kind: SignalKind::Input,
            ty: ExtendedType::from_core(ty),
            origin: None,
            span: None,
        }
    }

    fn output_signal(name: &str, ty: SignalType) -> SignalDecl {
        SignalDecl {
            name: name.to_string(),
            kind: SignalKind::Output,
            ty: ExtendedType::from_core(ty),
            origin: None,
            span: None,
        }
    }

    fn assert_property(name: &str, formula: PropertyFormula) -> PropertyDecl {
        PropertyDecl {
            name: name.to_string(),
            directive: PropertyDirective::Assert,
            formula,
            origin: None,
            span: None,
        }
    }

    #[test]
    fn empty_module_produces_empty_config() {
        let result = stub_pipeline(Vec::new(), Vec::new());
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert!(config.sensors.is_empty());
        assert!(config.properties.is_empty());
        assert!(config.action_table.is_empty());
        assert_eq!(config.window_size, DEFAULT_WINDOW_SIZE);
        assert_eq!(config.knowledge_capacity, DEFAULT_KNOWLEDGE_CAPACITY);
    }

    #[test]
    fn all_signals_become_sensors_with_observable_flag() {
        let signals = vec![
            input_signal("pressure", SignalType::Unsigned(8)),
            output_signal("alarm", SignalType::Bool),
            input_signal("temp", SignalType::Unsigned(16)),
        ];
        let result = stub_pipeline(signals, Vec::new());
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
        let signals = vec![input_signal("flag", SignalType::Bool)];
        let result = stub_pipeline(signals, Vec::new());
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.sensors[0].base_value, 1);
        assert_eq!(config.sensors[0].noise_amplitude, 0);
    }

    #[test]
    fn unsigned_sensor_midpoint() {
        let signals = vec![input_signal("data", SignalType::Unsigned(8))];
        let result = stub_pipeline(signals, Vec::new());
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.sensors[0].base_value, 127);
        assert_eq!(config.sensors[0].noise_amplitude, sensors::DEFAULT_NOISE_AMPLITUDE);
    }

    #[test]
    fn always_signal_lowers_to_always_is_true() {
        let props =
            vec![assert_property("p1", PropertyFormula::Always(Expr::Signal("alive".to_string())))];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.properties.len(), 1);
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::IsTrue("alive".to_string()))
        );
    }

    #[test]
    fn never_signal_lowers_to_always_less_than_one() {
        let props = vec![assert_property(
            "p_never",
            PropertyFormula::Never(Expr::Signal("fault".to_string())),
        )];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.properties.len(), 1);
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::LessThan("fault".to_string(), 1))
        );
    }

    #[test]
    fn eventually_within_lowers_correctly() {
        let props = vec![assert_property(
            "p_ev",
            PropertyFormula::EventuallyWithin {
                expr: Expr::Signal("ready".to_string()),
                cycles: 10,
            },
        )];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.properties.len(), 1);
        assert_eq!(
            config.properties[0],
            TemporalProperty::EventuallyWithin(SignalPredicate::IsTrue("ready".to_string()), 10)
        );
    }

    #[test]
    fn cover_and_assume_are_skipped() {
        let props = vec![
            PropertyDecl {
                name: "c1".to_string(),
                directive: PropertyDirective::Cover,
                formula: PropertyFormula::Always(Expr::Signal("x".to_string())),
                origin: None,
                span: None,
            },
            PropertyDecl {
                name: "a1".to_string(),
                directive: PropertyDirective::Assume,
                formula: PropertyFormula::Always(Expr::Signal("y".to_string())),
                origin: None,
                span: None,
            },
        ];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert!(config.properties.is_empty());
        assert!(config.action_table.is_empty());
    }

    #[test]
    fn always_implies_lowers_to_temporal_property() {
        let props = vec![assert_property(
            "p_impl",
            PropertyFormula::AlwaysImplies {
                antecedent: Expr::Signal("a".to_string()),
                consequent: Expr::Signal("b".to_string()),
            },
        )];
        let result = stub_pipeline(Vec::new(), props);
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
        let props = vec![
            assert_property("p1", PropertyFormula::Always(Expr::Signal("a".to_string()))),
            assert_property(
                "p2",
                PropertyFormula::EventuallyWithin {
                    expr: Expr::Signal("b".to_string()),
                    cycles: 5,
                },
            ),
        ];
        let result = stub_pipeline(Vec::new(), props);
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
        let props = vec![assert_property(
            "p_impl",
            PropertyFormula::AlwaysImplies {
                antecedent: Expr::Signal("a".to_string()),
                consequent: Expr::Signal("b".to_string()),
            },
        )];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(config.action_table.len(), 1);
        assert_eq!(config.action_table[0].action, AdaptationAction::Reduce);
        assert_eq!(config.action_table[0].priority, 100);
    }

    #[test]
    fn too_many_signals_produces_error() {
        let signals: Vec<SignalDecl> = (0..MAX_BRIDGE_SIGNALS + 1)
            .map(|i| input_signal(&format!("s{i}"), SignalType::Unsigned(8)))
            .collect();
        let result = stub_pipeline(signals, Vec::new());
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
        use crate::ast::types::{BinaryOp, UnaryOp};

        let expr = Expr::Unary {
            op: UnaryOp::Not,
            operand: Box::new(Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(Expr::Signal("deep_signal".to_string())),
                right: Box::new(Expr::Literal(crate::ast::types::LiteralValue::Bool(true))),
            }),
        };

        let name = properties::extract_signal_name(&expr).expect("should find signal");
        assert_eq!(name, "deep_signal");
    }

    #[test]
    fn binary_lt_lowers_to_less_than() {
        use crate::ast::types::BinaryOp;

        let props = vec![assert_property(
            "p_lt",
            PropertyFormula::Always(Expr::Binary {
                op: BinaryOp::Lt,
                left: Box::new(Expr::Signal("pressure".to_string())),
                right: Box::new(Expr::Literal(crate::ast::types::LiteralValue::Integer(100))),
            }),
        )];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::LessThan("pressure".to_string(), 100))
        );
    }

    #[test]
    fn binary_gt_lowers_to_greater_than() {
        use crate::ast::types::BinaryOp;

        let props = vec![assert_property(
            "p_gt",
            PropertyFormula::Always(Expr::Binary {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Signal("rate".to_string())),
                right: Box::new(Expr::Literal(crate::ast::types::LiteralValue::Integer(50))),
            }),
        )];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");
        assert_eq!(
            config.properties[0],
            TemporalProperty::Always(SignalPredicate::GreaterThan("rate".to_string(), 50))
        );
    }
}
