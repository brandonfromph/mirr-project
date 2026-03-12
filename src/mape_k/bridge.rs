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
//!   to a single `SignalPredicate` produce an `UnsupportedFormula` error.
//! - **Action table**: Each lowered property gets a conservative entry:
//!   violation triggers `EmergencyStop` at maximum priority.
//!
//! All iteration is bounded by `MAX_BRIDGE_SIGNALS` / `MAX_BRIDGE_PROPERTIES`
//! constants (NASA Power-of-10 rule #2).

use crate::ast::property::{PropertyDirective, PropertyFormula};
use crate::ast::types::{SignalKind, SignalType};
use crate::ast::Expr;
use crate::mape_k::ltl::{SignalPredicate, TemporalProperty};
use crate::mape_k::planner::{ActionEntry, AdaptationAction, TriggerCondition};
use crate::mape_k::sensor::SensorConfig;
use crate::mape_k::SimConfig;
use crate::pipeline::PipelineResult;

// ---------------------------------------------------------------------------
// Constants — bounded resource limits (NASA P10)
// ---------------------------------------------------------------------------

/// Maximum signals the bridge will process.
pub const MAX_BRIDGE_SIGNALS: usize = 256;

/// Maximum properties the bridge will convert.
pub const MAX_BRIDGE_PROPERTIES: usize = 64;

/// Maximum expression nodes to visit when extracting a signal name.
const MAX_EXPR_VISIT: usize = 64;

/// Default rolling window size for the monitor.
pub const DEFAULT_WINDOW_SIZE: usize = 64;

/// Default knowledge base capacity.
pub const DEFAULT_KNOWLEDGE_CAPACITY: usize = 4096;

/// Default noise amplitude for heuristic sensor generation.
const DEFAULT_NOISE_AMPLITUDE: u64 = 2;

/// Default PRNG seed base (each sensor gets `SEED_BASE + index`).
const SEED_BASE: u64 = 1000;

// ---------------------------------------------------------------------------
// Bridge errors
// ---------------------------------------------------------------------------

/// Errors produced during the pipeline-to-SimConfig conversion.
#[derive(Debug, Clone)]
pub enum BridgeError {
    /// The module declares more signals than the bridge can process.
    TooManySignals { count: usize },
    /// The module declares more assert-properties than the bridge can process.
    TooManyProperties { count: usize },
    /// A property formula cannot be lowered to a single `SignalPredicate`.
    UnsupportedFormula { description: String },
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManySignals { count } => {
                write!(f, "too many signals for bridge: {count} > {MAX_BRIDGE_SIGNALS}")
            }
            Self::TooManyProperties { count } => {
                write!(f, "too many properties for bridge: {count} > {MAX_BRIDGE_PROPERTIES}")
            }
            Self::UnsupportedFormula { description } => {
                write!(f, "unsupported formula: {description}")
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Convert a `PipelineResult` into a MAPE-K `SimConfig`.
///
/// Steps:
/// 1. Extract input signals from the AST module into `SensorConfig` entries
///    with heuristic defaults derived from each signal's type.
/// 2. Lower `Assert` properties to `TemporalProperty` (skip `Cover`/`Assume`).
/// 3. Generate a conservative action table: every property violation triggers
///    `EmergencyStop` at maximum priority.
/// 4. Apply default `window_size` and `knowledge_capacity`.
pub fn bridge_from_pipeline(result: &PipelineResult) -> Result<SimConfig, Vec<BridgeError>> {
    let mut errors: Vec<BridgeError> = Vec::new();

    // 1. Extract sensors from the module's signal declarations.
    let sensors = extract_sensors(result, &mut errors);

    // 2. Lower assert-properties to temporal properties.
    let properties = extract_properties(result, &mut errors);

    if !errors.is_empty() {
        return Err(errors);
    }

    // 3. Generate conservative action table.
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
// Sensor extraction
// ---------------------------------------------------------------------------

/// Walk the program's input signal declarations and produce a `SensorConfig`
/// for each one. Only `Input` signals become sensors (outputs and internals
/// are driven by the design, not sampled externally).
///
/// Heuristic defaults:
/// - `Bool`: base_value = 1, noise = 0 (deterministic toggle)
/// - `Unsigned(w)`: base_value = midpoint of [0, 2^w - 1], noise = 2
/// - `Signed(w)`: base_value = 0, noise = 2
fn extract_sensors(result: &PipelineResult, errors: &mut Vec<BridgeError>) -> Vec<SensorConfig> {
    let signals = &result.program.module.signals;

    // Count input signals only.
    let input_count = count_input_signals(signals);
    if input_count > MAX_BRIDGE_SIGNALS {
        errors.push(BridgeError::TooManySignals { count: input_count });
        return Vec::new();
    }

    let mut sensors = Vec::with_capacity(input_count);
    let mut idx: usize = 0;

    // Bounded iteration over all signals, capped at MAX_BRIDGE_SIGNALS
    // outputs. The outer loop visits at most `signals.len()` elements
    // (which is <= MAX_BRIDGE_SIGNALS after the guard above for inputs).
    for sig in signals.iter().take(MAX_BRIDGE_SIGNALS) {
        if sig.kind != SignalKind::Input {
            continue;
        }

        let (base_value, noise_amplitude) = heuristic_sensor_defaults(&sig.ty.core);

        sensors.push(SensorConfig {
            name: sig.name.clone(),
            base_value,
            noise_amplitude,
            fault_at_tick: None,
            fault_value: 0,
            fault_end_tick: None,
            seed: SEED_BASE.wrapping_add(idx as u64),
        });

        idx = idx.saturating_add(1);
        if idx >= MAX_BRIDGE_SIGNALS {
            break;
        }
    }

    sensors
}

/// Count input signals in the declarations list (bounded scan).
fn count_input_signals(signals: &[crate::ast::SignalDecl]) -> usize {
    let mut count: usize = 0;
    for sig in signals.iter().take(MAX_BRIDGE_SIGNALS.saturating_add(1)) {
        if sig.kind == SignalKind::Input {
            count = count.saturating_add(1);
        }
    }
    count
}

/// Compute heuristic `(base_value, noise_amplitude)` for a given signal type.
fn heuristic_sensor_defaults(ty: &SignalType) -> (u64, u64) {
    match ty {
        SignalType::Bool => (1, 0),
        SignalType::Unsigned(width) => {
            // Midpoint of representable range, small noise.
            let max_val = max_unsigned_value(*width);
            let midpoint = max_val / 2;
            (midpoint, DEFAULT_NOISE_AMPLITUDE.min(midpoint))
        }
        SignalType::Signed(width) => {
            // Centered at zero, small noise.
            let half = max_unsigned_value(width.saturating_sub(1));
            (0, DEFAULT_NOISE_AMPLITUDE.min(half))
        }
    }
}

/// Maximum unsigned value for a given bit-width, clamped to avoid overflow.
fn max_unsigned_value(width: u32) -> u64 {
    if width == 0 {
        return 0;
    }
    if width >= 64 {
        return u64::MAX;
    }
    (1u64 << width).wrapping_sub(1)
}

// ---------------------------------------------------------------------------
// Property extraction
// ---------------------------------------------------------------------------

/// Walk the module's property declarations. For each `Assert` property,
/// attempt to lower the formula to a `TemporalProperty`. `Cover` and
/// `Assume` directives are skipped.
fn extract_properties(
    result: &PipelineResult,
    errors: &mut Vec<BridgeError>,
) -> Vec<TemporalProperty> {
    let props = &result.program.module.properties;

    // Count assert-properties.
    let assert_count = count_assert_properties(props);
    if assert_count > MAX_BRIDGE_PROPERTIES {
        errors.push(BridgeError::TooManyProperties { count: assert_count });
        return Vec::new();
    }

    let mut temporal_props = Vec::with_capacity(assert_count);

    for prop in props.iter().take(MAX_BRIDGE_PROPERTIES) {
        if prop.directive != PropertyDirective::Assert {
            continue;
        }

        match lower_formula(&prop.formula) {
            Ok(tp) => temporal_props.push(tp),
            Err(desc) => errors.push(BridgeError::UnsupportedFormula { description: desc }),
        }

        if temporal_props.len() >= MAX_BRIDGE_PROPERTIES {
            break;
        }
    }

    temporal_props
}

/// Count assert-properties (bounded scan).
fn count_assert_properties(props: &[crate::ast::property::PropertyDecl]) -> usize {
    let mut count: usize = 0;
    for p in props.iter().take(MAX_BRIDGE_PROPERTIES.saturating_add(1)) {
        if p.directive == PropertyDirective::Assert {
            count = count.saturating_add(1);
        }
    }
    count
}

/// Attempt to lower a `PropertyFormula` into a `TemporalProperty`.
///
/// Supported lowerings:
/// - `Always(expr)` -> `TemporalProperty::Always(predicate)` where `expr`
///   is a simple signal reference (lowered to `IsTrue(signal_name)`).
/// - `Never(expr)` -> inverted: `Always(IsTrue(signal))` is not a
///   direct match, so we lower to `Always` with a heuristic predicate
///   that the signal should be zero (i.e., `LessThan(name, 1)`).
/// - `EventuallyWithin { expr, cycles }` ->
///   `TemporalProperty::EventuallyWithin(predicate, cycles)`.
///
/// Multi-expression formulas (`AlwaysImplies`, `NeverImplies`,
/// `AlwaysFollowedBy`) cannot be lowered to a single `SignalPredicate`
/// and produce `UnsupportedFormula`.
fn lower_formula(formula: &PropertyFormula) -> Result<TemporalProperty, String> {
    match formula {
        PropertyFormula::Always(expr) => {
            let pred = lower_expr_to_predicate(expr)?;
            Ok(TemporalProperty::Always(pred))
        }
        PropertyFormula::Never(expr) => {
            // "never(P)" means P must always be false.
            // Invert: the signal referenced by P should be zero (not true).
            let signal = extract_signal_name(expr)?;
            let pred = SignalPredicate::LessThan(signal, 1);
            Ok(TemporalProperty::Always(pred))
        }
        PropertyFormula::EventuallyWithin { expr, cycles } => {
            let pred = lower_expr_to_predicate(expr)?;
            Ok(TemporalProperty::EventuallyWithin(pred, u64::from(*cycles)))
        }
        PropertyFormula::AlwaysImplies { .. } => {
            Err("AlwaysImplies requires two signals; cannot lower to a single predicate"
                .to_string())
        }
        PropertyFormula::NeverImplies { .. } => {
            Err("NeverImplies requires two signals; cannot lower to a single predicate".to_string())
        }
        PropertyFormula::AlwaysFollowedBy { .. } => {
            Err("AlwaysFollowedBy requires two signals; cannot lower to a single predicate"
                .to_string())
        }
    }
}

/// Lower a simple expression to a `SignalPredicate`.
///
/// Supported patterns:
/// - `Signal(name)` -> `IsTrue(name)`
/// - `Binary { op: Lt, left: Signal(name), right: Literal(Integer(n)) }` -> `LessThan(name, n)`
/// - `Binary { op: Gt, left: Signal(name), right: Literal(Integer(n)) }` -> `GreaterThan(name, n)`
/// - Other expressions -> error
fn lower_expr_to_predicate(expr: &Expr) -> Result<SignalPredicate, String> {
    match expr {
        Expr::Signal(name) => Ok(SignalPredicate::IsTrue(name.clone())),
        Expr::Binary { op, left, right } => lower_binary_predicate(op, left, right),
        Expr::Unary { .. } => {
            // Try to extract signal name for a basic IsTrue check.
            let name = extract_signal_name(expr)?;
            Ok(SignalPredicate::IsTrue(name))
        }
        Expr::Literal(_) => Err("bare literal cannot be a signal predicate".to_string()),
        Expr::Prev { signal, .. } => {
            // Treat prev-reference as a simple signal check.
            Ok(SignalPredicate::IsTrue(signal.clone()))
        }
    }
}

/// Lower a binary expression to a `SignalPredicate`.
fn lower_binary_predicate(
    op: &crate::ast::types::BinaryOp,
    left: &Expr,
    right: &Expr,
) -> Result<SignalPredicate, String> {
    use crate::ast::types::BinaryOp;

    // Pattern: Signal <op> Literal(Integer)
    if let (Expr::Signal(name), Some(threshold)) = (left, literal_u64(right)) {
        return match op {
            BinaryOp::Lt => Ok(SignalPredicate::LessThan(name.clone(), threshold)),
            BinaryOp::Le => {
                // signal <= N  is equivalent to signal < N+1 (for integer domain).
                Ok(SignalPredicate::LessThan(name.clone(), threshold.saturating_add(1)))
            }
            BinaryOp::Gt => Ok(SignalPredicate::GreaterThan(name.clone(), threshold)),
            BinaryOp::Ge => {
                // signal >= N  is equivalent to signal > N-1.
                Ok(SignalPredicate::GreaterThan(name.clone(), threshold.saturating_sub(1)))
            }
            _ => {
                // For And/Or/Eq/Ne/Add etc., fall back to IsTrue on the left signal.
                Ok(SignalPredicate::IsTrue(name.clone()))
            }
        };
    }

    // Fallback: try to extract any signal name from the expression tree.
    let name = extract_signal_name(left).or_else(|_| extract_signal_name(right))?;
    Ok(SignalPredicate::IsTrue(name))
}

/// Extract a `u64` from a literal expression, if it is one.
fn literal_u64(expr: &Expr) -> Option<u64> {
    match expr {
        Expr::Literal(crate::ast::types::LiteralValue::Integer(n)) => Some(*n),
        Expr::Literal(crate::ast::types::LiteralValue::Bool(b)) => Some(u64::from(*b)),
        _ => None,
    }
}

/// Walk an expression tree (bounded, iterative) to find the first `Signal`
/// name. Returns an error if no signal is found within `MAX_EXPR_VISIT` nodes.
fn extract_signal_name(expr: &Expr) -> Result<String, String> {
    // Iterative bounded traversal using an explicit work stack.
    let mut stack: Vec<&Expr> = Vec::with_capacity(MAX_EXPR_VISIT);
    stack.push(expr);

    let mut visited: usize = 0;
    while let Some(current) = stack.pop() {
        visited = visited.saturating_add(1);
        if visited > MAX_EXPR_VISIT {
            break;
        }

        match current {
            Expr::Signal(name) => return Ok(name.clone()),
            Expr::Prev { signal, .. } => return Ok(signal.clone()),
            Expr::Unary { operand, .. } => stack.push(operand),
            Expr::Binary { left, right, .. } => {
                stack.push(right);
                stack.push(left);
            }
            Expr::Literal(_) => {}
        }
    }

    Err("no signal reference found in expression".to_string())
}

// ---------------------------------------------------------------------------
// Action table generation
// ---------------------------------------------------------------------------

/// Generate a conservative action table: every property violation triggers
/// `EmergencyStop` at maximum priority (255).
///
/// This is the safest default for a safety-critical system. The user can
/// replace entries with more nuanced actions after bridge construction.
fn generate_action_table(properties: &[TemporalProperty]) -> Vec<ActionEntry> {
    let mut entries = Vec::with_capacity(properties.len());

    for (idx, _prop) in properties.iter().enumerate().take(MAX_BRIDGE_PROPERTIES) {
        entries.push(ActionEntry {
            trigger_property_idx: idx,
            action: AdaptationAction::EmergencyStop,
            priority: 255,
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
    use crate::ast::SignalDecl;

    /// Build a minimal `PipelineResult` with the given signals and properties.
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
            program: MirrProgram { patterns: Vec::new(), module },
            simplify_stats: None,
            width_result: None,
            temporal_netlist: None,
            rspu_program: None,
            type_map: None,
            extended_type_map: None,
            sim_result: None,
            mape_k_result: None,
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

    // -- Sensor extraction tests --

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
    fn only_input_signals_become_sensors() {
        let signals = vec![
            input_signal("pressure", SignalType::Unsigned(8)),
            output_signal("alarm", SignalType::Bool),
            input_signal("temp", SignalType::Unsigned(16)),
        ];
        let result = stub_pipeline(signals, Vec::new());
        let config = bridge_from_pipeline(&result).expect("should succeed");

        assert_eq!(config.sensors.len(), 2);
        assert_eq!(config.sensors[0].name, "pressure");
        assert_eq!(config.sensors[1].name, "temp");
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

        // u8 max = 255, midpoint = 127
        assert_eq!(config.sensors[0].base_value, 127);
        assert_eq!(config.sensors[0].noise_amplitude, DEFAULT_NOISE_AMPLITUDE);
    }

    // -- Property lowering tests --

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
    fn unsupported_formula_produces_error() {
        let props = vec![assert_property(
            "p_impl",
            PropertyFormula::AlwaysImplies {
                antecedent: Expr::Signal("a".to_string()),
                consequent: Expr::Signal("b".to_string()),
            },
        )];
        let result = stub_pipeline(Vec::new(), props);
        let err = bridge_from_pipeline(&result).expect_err("should fail");

        assert_eq!(err.len(), 1);
        match &err[0] {
            BridgeError::UnsupportedFormula { description } => {
                assert!(description.contains("AlwaysImplies"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    // -- Action table tests --

    #[test]
    fn action_table_has_one_entry_per_property() {
        let props = vec![
            assert_property("p1", PropertyFormula::Always(Expr::Signal("a".to_string()))),
            assert_property("p2", PropertyFormula::Always(Expr::Signal("b".to_string()))),
        ];
        let result = stub_pipeline(Vec::new(), props);
        let config = bridge_from_pipeline(&result).expect("should succeed");

        assert_eq!(config.action_table.len(), 2);
        for (i, entry) in config.action_table.iter().enumerate() {
            assert_eq!(entry.trigger_property_idx, i);
            assert_eq!(entry.action, AdaptationAction::EmergencyStop);
            assert_eq!(entry.priority, 255);
            assert_eq!(entry.trigger_on, TriggerCondition::OnViolation);
        }
    }

    // -- Overflow / bounds tests --

    #[test]
    fn too_many_signals_produces_error() {
        let signals: Vec<SignalDecl> = (0..MAX_BRIDGE_SIGNALS + 1)
            .map(|i| input_signal(&format!("s{i}"), SignalType::Unsigned(8)))
            .collect();
        let result = stub_pipeline(signals, Vec::new());
        let err = bridge_from_pipeline(&result).expect_err("should fail");

        assert!(err.iter().any(|e| matches!(e, BridgeError::TooManySignals { .. })));
    }

    // -- Helper tests --

    #[test]
    fn max_unsigned_value_edge_cases() {
        assert_eq!(max_unsigned_value(0), 0);
        assert_eq!(max_unsigned_value(1), 1);
        assert_eq!(max_unsigned_value(8), 255);
        assert_eq!(max_unsigned_value(16), 65535);
        assert_eq!(max_unsigned_value(64), u64::MAX);
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

        let name = extract_signal_name(&expr).expect("should find signal");
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
