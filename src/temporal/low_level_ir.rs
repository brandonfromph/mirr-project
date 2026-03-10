//! Low-Level Temporal IR
//!
//! Defines the intermediate representation for compiled temporal guards
//! that can be mapped to hardware primitives like shift registers and counters.

#![forbid(unsafe_code)]

use crate::ast::{
    types::{BinaryOp, LiteralValue, SignalType, UnaryOp},
    Expr,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// ConditionKind — typed representation of supported guard condition forms
// ---------------------------------------------------------------------------
// NASA/JPL rationale: every condition lowered by the compiler must be an
// explicit, named variant.  Unknown/complex expressions are REJECTED at
// compile-time rather than silently synthesised. (P2-REQ-013)
// ---------------------------------------------------------------------------

/// The set of condition forms that the Temporal Guard Compiler can lower.
///
/// Only variants listed here are accepted. Any other `Expr` form causes a
/// `MirrError::TemporalCompilationError` — there is no silent fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionKind {
    /// `when <signal>` — monitor signal going high
    SimpleSignal(String),
    /// `when !<signal>` / `when not <signal>` — monitor signal going low
    NegatedSignal(String),
    /// `when <signal> <op> <literal>` — magnitude or equality comparison.
    ///
    /// Supported operators: `==`, `!=`, `<`, `<=`, `>`, `>=`.
    /// All six forms lower to a hardware comparator circuit. (P2-REQ-015, Step 2.2)
    Comparison {
        /// The signal being compared
        signal: String,
        /// The comparison operator
        op: BinaryOp,
        /// The literal value on the right-hand side
        value: LiteralValue,
    },
}

impl ConditionKind {
    /// Return the primary signal name driven by this condition.
    pub fn primary_signal(&self) -> &str {
        match self {
            ConditionKind::SimpleSignal(s) => s,
            ConditionKind::NegatedSignal(s) => s,
            ConditionKind::Comparison { signal, .. } => signal,
        }
    }

    /// Return a human-readable description suitable for DOT/HTML emission.
    pub fn describe(&self) -> String {
        match self {
            ConditionKind::SimpleSignal(s) => format!("when {s} (high)"),
            ConditionKind::NegatedSignal(s) => format!("when !{s} (low)"),
            ConditionKind::Comparison { signal, op, value } => {
                let op_str = match op {
                    BinaryOp::Eq => "==",
                    BinaryOp::Ne => "!=",
                    BinaryOp::Lt => "<",
                    BinaryOp::Le => "<=",
                    BinaryOp::Gt => ">",
                    BinaryOp::Ge => ">=",
                    BinaryOp::And => "AND",
                    BinaryOp::Or => "OR",
                    BinaryOp::Xor => "XOR",
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Shl => "<<",
                    BinaryOp::Shr => ">>",
                };
                let val_str = match value {
                    LiteralValue::Integer(n) => format!("{n}"),
                    LiteralValue::Bool(b) => format!("{b}"),
                };
                format!("when {signal} {op_str} {val_str}")
            }
        }
    }

    /// Attempt to lower an `Expr` into a `ConditionKind`.
    ///
    /// Returns `Err(&'static str)` for unsupported forms — no heap allocation
    /// on the error path. The caller embeds the guard name in diagnostics.
    pub fn try_from_expr(expr: &Expr) -> Result<Self, &'static str> {
        match expr {
            // `when <signal>`
            Expr::Signal(name) => Ok(ConditionKind::SimpleSignal(name.clone())),

            // `when !<signal>` or `when not <signal>`
            Expr::Unary { op: UnaryOp::Not, operand } => match operand.as_ref() {
                Expr::Signal(name) => Ok(ConditionKind::NegatedSignal(name.clone())),
                _ => Err("negation of non-signal expressions is not supported"),
            },

            // `when <signal> <op> <literal>` — all six comparison operators (Step 2.2)
            Expr::Binary { op, left, right }
                if matches!(
                    op,
                    BinaryOp::Eq
                        | BinaryOp::Ne
                        | BinaryOp::Lt
                        | BinaryOp::Le
                        | BinaryOp::Gt
                        | BinaryOp::Ge
                ) =>
            {
                match (left.as_ref(), right.as_ref()) {
                    (Expr::Signal(s), Expr::Literal(v)) => Ok(ConditionKind::Comparison {
                        signal: s.clone(),
                        op: *op,
                        value: v.clone(),
                    }),
                    _ => Err("comparisons must be of the form <signal> <op> <literal>"),
                }
            }

            // Everything else: reject with a clear static message
            _ => Err("unsupported condition expression form"),
        }
    }
}

// ---------------------------------------------------------------------------
// Netlist and guard types
// ---------------------------------------------------------------------------

/// Low-level temporal IR for compiled guards
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalNetlist {
    /// Compiled temporal guards
    pub guards: Vec<CompiledGuard>,
    /// Generated signals (shift registers, counters, etc.)
    pub signals: Vec<GeneratedSignal>,
    /// Resource usage statistics
    pub statistics: CompilationStatistics,
}

/// A compiled temporal guard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompiledGuard {
    /// Shift register-based guard for short delays (N ≤ 16)
    ShiftRegister(ShiftRegisterGuard),
    /// Counter-based guard for long delays (N > 16)
    Counter(CounterGuard),
    /// Complex guard with multiple components
    Complex(ComplexGuard),
}

/// Shift register-based temporal guard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShiftRegisterGuard {
    /// Original guard name
    pub name: String,
    /// Input signal to monitor
    pub input_signal: String,
    /// Output signal name
    pub output_signal: String,
    /// Shift register stage signal names
    pub stages: Vec<String>,
    /// Number of cycles to delay
    pub delay_cycles: u64,
    /// Lowered condition semantics — self-describing IR for explainability. (P2-REQ-016)
    pub condition_kind: ConditionKind,
}

/// Counter-based temporal guard
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CounterGuard {
    /// Original guard name
    pub name: String,
    /// Input signal to monitor
    pub input_signal: String,
    /// Output signal name
    pub output_signal: String,
    /// Counter register signal name
    pub counter_signal: String,
    /// Comparator output signal name
    pub comparator_signal: String,
    /// Target count value (= delay in cycles)
    pub target_count: u64,
    /// Lowered condition semantics — self-describing IR for explainability. (P2-REQ-017)
    pub condition_kind: ConditionKind,
}

/// Complex guard with multiple temporal components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplexGuard {
    /// Original guard name
    pub name: String,
    /// Sub-guards that make up this complex guard
    pub sub_guards: Vec<CompiledGuard>,
    /// Final output signal
    pub output_signal: String,
    /// Final combination logic
    pub combination_logic: Expr,
}

/// Generated signal for temporal implementation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedSignal {
    /// Signal name
    pub name: String,
    /// Signal type
    pub ty: SignalType,
    /// Signal kind
    pub kind: GeneratedSignalKind,
    /// Source expression (if applicable)
    pub source: Option<Expr>,
}

/// Classification of generated signals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneratedSignalKind {
    /// Shift register pipeline stage
    ShiftRegisterStage,
    /// Counter register
    Counter,
    /// Comparator output
    Comparator,
    /// Logic gate output
    LogicGate,
    /// Intermediate signal
    Intermediate,
}

/// Compilation statistics and hardware resource usage
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CompilationStatistics {
    /// Total shift register stages used
    pub shift_registers_used: u32,
    /// Total counters used
    pub counters_used: u32,
    /// Total logic gates used
    pub logic_gates_used: u32,
    /// Maximum delay cycles across all guards
    pub max_delay_cycles: u64,
    /// Total generated signals
    pub total_signals: u32,
    /// Compilation time in microseconds (if measured)
    pub compilation_time_us: Option<u64>,
}

/// Versioned netlist JSON wrapper for canonical IR contract serialization.
/// Used by IR contract tests and parity gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemporalNetlistJson {
    /// IR version string for contract tracking (currently "1.0").
    pub ir_version: String,
    /// Compiled guard definitions.
    pub guards: Vec<CompiledGuard>,
    /// Generated hardware signals (shift registers, counters, etc.).
    pub signals: Vec<GeneratedSignal>,
    /// Compilation statistics (guard count, signal count, timing).
    pub statistics: CompilationStatistics,
}

impl TemporalNetlistJson {
    /// Wrap a compiled netlist in the versioned JSON envelope.
    pub fn from_netlist(netlist: &TemporalNetlist) -> Self {
        Self {
            ir_version: crate::ast::types::IR_VERSION.to_string(),
            guards: netlist.guards.clone(),
            signals: netlist.signals.clone(),
            statistics: netlist.statistics.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// TemporalNetlist impl
// ---------------------------------------------------------------------------

impl Default for TemporalNetlist {
    fn default() -> Self {
        Self::new()
    }
}

impl TemporalNetlist {
    /// Create a new empty temporal netlist
    pub fn new() -> Self {
        Self {
            guards: Vec::new(),
            signals: Vec::new(),
            statistics: CompilationStatistics::default(),
        }
    }

    /// Add a compiled guard to the netlist
    pub fn add_guard(&mut self, guard: CompiledGuard) {
        self.guards.push(guard);
        self.update_statistics();
    }

    /// Add a generated signal to the netlist
    pub fn add_signal(&mut self, signal: GeneratedSignal) {
        self.signals.push(signal);
        self.update_statistics();
    }

    fn update_statistics(&mut self) {
        let mut shift_registers = 0u32;
        let mut counters = 0u32;
        let mut logic_gates = 0u32;

        for signal in &self.signals {
            match signal.kind {
                GeneratedSignalKind::ShiftRegisterStage => shift_registers += 1,
                GeneratedSignalKind::Counter => counters += 1,
                GeneratedSignalKind::LogicGate => logic_gates += 1,
                _ => {}
            }
        }

        self.statistics.shift_registers_used = shift_registers;
        self.statistics.counters_used = counters;
        self.statistics.logic_gates_used = logic_gates;
        self.statistics.total_signals = self.signals.len() as u32;

        let mut max_delay = 0u64;
        for guard in &self.guards {
            match guard {
                CompiledGuard::ShiftRegister(sr) => {
                    max_delay = max_delay.max(sr.delay_cycles);
                }
                CompiledGuard::Counter(c) => {
                    max_delay = max_delay.max(c.target_count);
                }
                CompiledGuard::Complex(_) => {}
            }
        }
        self.statistics.max_delay_cycles = max_delay;
    }

    /// Return a one-paragraph summary for CLI output.
    pub fn summary(&self) -> String {
        format!(
            "Temporal Netlist Summary:\n\
             - Guards: {}\n\
             - Signals: {}\n\
             - Shift Registers: {}\n\
             - Counters: {}\n\
             - Logic Gates: {}\n\
             - Max Delay: {} cycles",
            self.guards.len(),
            self.signals.len(),
            self.statistics.shift_registers_used,
            self.statistics.counters_used,
            self.statistics.logic_gates_used,
            self.statistics.max_delay_cycles
        )
    }
}

// ---------------------------------------------------------------------------
// GeneratedSignal constructors
// ---------------------------------------------------------------------------

impl GeneratedSignal {
    /// Create a shift register stage signal
    pub fn shift_register_stage(name: String, _stage: u32) -> Self {
        Self {
            name,
            ty: SignalType::Bool,
            kind: GeneratedSignalKind::ShiftRegisterStage,
            source: None,
        }
    }

    /// Create a counter register signal
    pub fn counter(name: String, width: u32) -> Self {
        Self {
            name,
            ty: SignalType::Unsigned(width),
            kind: GeneratedSignalKind::Counter,
            source: None,
        }
    }

    /// Create a comparator output signal
    pub fn comparator(name: String) -> Self {
        Self { name, ty: SignalType::Bool, kind: GeneratedSignalKind::Comparator, source: None }
    }

    /// Create a logic gate output signal
    pub fn logic_gate(name: String, gate_type: &str, inputs: Vec<String>) -> Self {
        let source = if inputs.len() == 1 {
            Some(Expr::Signal(inputs[0].clone()))
        } else if inputs.len() == 2 {
            Some(Expr::Binary {
                left: Box::new(Expr::Signal(inputs[0].clone())),
                op: match gate_type {
                    "AND" => BinaryOp::And,
                    "OR" => BinaryOp::Or,
                    "XOR" => BinaryOp::Xor,
                    _ => BinaryOp::And,
                },
                right: Box::new(Expr::Signal(inputs[1].clone())),
            })
        } else {
            None
        };
        Self { name, ty: SignalType::Bool, kind: GeneratedSignalKind::LogicGate, source }
    }
}

// ---------------------------------------------------------------------------
// Guard constructors
// ---------------------------------------------------------------------------

impl ShiftRegisterGuard {
    /// Create a shift register guard.
    ///
    /// `condition_kind` is stored in the IR for explainability output. (P2-REQ-016)
    pub fn new(
        name: String,
        input_signal: String,
        delay_cycles: u64,
        condition_kind: ConditionKind,
    ) -> Self {
        let stages = (0..delay_cycles).map(|i| format!("{name}_sr_{i}")).collect();
        Self {
            output_signal: format!("{name}_out"),
            name,
            input_signal,
            stages,
            delay_cycles,
            condition_kind,
        }
    }

    /// Number of pipeline stages
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
}

impl CounterGuard {
    /// Create a counter guard.
    ///
    /// `condition_kind` is stored in the IR for explainability output. (P2-REQ-017)
    pub fn new(
        name: String,
        input_signal: String,
        target_count: u64,
        condition_kind: ConditionKind,
    ) -> Self {
        Self {
            counter_signal: format!("{name}_counter"),
            comparator_signal: format!("{name}_cmp"),
            output_signal: format!("{name}_out"),
            name,
            input_signal,
            target_count,
            condition_kind,
        }
    }

    /// Minimum counter width in bits to represent `target_count`
    pub fn counter_width(&self) -> u32 {
        if self.target_count == 0 {
            1
        } else {
            (self.target_count as f64).log2().ceil() as u32 + 1
        }
    }
}

impl ComplexGuard {
    /// Create a complex guard
    pub fn new(name: String, sub_guards: Vec<CompiledGuard>, combination_logic: Expr) -> Self {
        Self { output_signal: format!("{name}_out"), name, sub_guards, combination_logic }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::SignalType;

    fn simple_ck(sig: &str) -> ConditionKind {
        ConditionKind::SimpleSignal(sig.to_string())
    }

    #[test]
    fn test_shift_register_guard_creation() {
        let ck = simple_ck("input_signal");
        let guard = ShiftRegisterGuard::new(
            "test_guard".to_string(),
            "input_signal".to_string(),
            4,
            ck.clone(),
        );

        assert_eq!(guard.name, "test_guard");
        assert_eq!(guard.input_signal, "input_signal");
        assert_eq!(guard.delay_cycles, 4);
        assert_eq!(guard.stage_count(), 4);
        assert_eq!(guard.stages[0], "test_guard_sr_0");
        assert_eq!(guard.stages[3], "test_guard_sr_3");
        assert_eq!(guard.condition_kind, ck);
    }

    #[test]
    fn test_counter_guard_creation() {
        let ck = simple_ck("input_signal");
        let guard = CounterGuard::new(
            "long_delay_guard".to_string(),
            "input_signal".to_string(),
            100,
            ck.clone(),
        );

        assert_eq!(guard.name, "long_delay_guard");
        assert_eq!(guard.target_count, 100);
        assert_eq!(guard.counter_width(), 8); // ceil(log2(100))+1 = 7+1 = 8
        assert_eq!(guard.condition_kind, ck);
    }

    #[test]
    fn test_generated_signal_creation() {
        let sr = GeneratedSignal::shift_register_stage("test_sr".to_string(), 0);
        assert_eq!(sr.kind, GeneratedSignalKind::ShiftRegisterStage);
        assert_eq!(sr.ty, SignalType::Bool);

        let ctr = GeneratedSignal::counter("test_counter".to_string(), 8);
        assert_eq!(ctr.kind, GeneratedSignalKind::Counter);
        assert_eq!(ctr.ty, SignalType::Unsigned(8));
    }

    #[test]
    fn test_netlist_statistics() {
        let mut netlist = TemporalNetlist::new();
        netlist.add_signal(GeneratedSignal::shift_register_stage("sr1".to_string(), 0));
        netlist.add_signal(GeneratedSignal::shift_register_stage("sr2".to_string(), 1));
        netlist.add_signal(GeneratedSignal::counter("counter1".to_string(), 8));
        netlist.add_signal(GeneratedSignal::logic_gate(
            "gate1".to_string(),
            "AND",
            vec!["a".to_string(), "b".to_string()],
        ));

        assert_eq!(netlist.statistics.shift_registers_used, 2);
        assert_eq!(netlist.statistics.counters_used, 1);
        assert_eq!(netlist.statistics.logic_gates_used, 1);
        assert_eq!(netlist.statistics.total_signals, 4);
    }

    #[test]
    fn test_condition_kind_describe() {
        assert_eq!(ConditionKind::SimpleSignal("clk".to_string()).describe(), "when clk (high)");
        assert_eq!(
            ConditionKind::NegatedSignal("reset".to_string()).describe(),
            "when !reset (low)"
        );
        let cmp = ConditionKind::Comparison {
            signal: "pressure".to_string(),
            op: BinaryOp::Lt,
            value: LiteralValue::Integer(50),
        };
        assert_eq!(cmp.describe(), "when pressure < 50");
    }

    #[test]
    fn test_condition_kind_all_comparison_ops_accepted() {
        // Step 2.2: all six comparison operators must be accepted (P2-REQ-015)
        for op in
            [BinaryOp::Eq, BinaryOp::Ne, BinaryOp::Lt, BinaryOp::Le, BinaryOp::Gt, BinaryOp::Ge]
        {
            let expr = Expr::Binary {
                op,
                left: Box::new(Expr::Signal("sig".to_string())),
                right: Box::new(Expr::Literal(LiteralValue::Integer(42))),
            };
            assert!(
                ConditionKind::try_from_expr(&expr).is_ok(),
                "operator {op:?} should be accepted"
            );
        }
    }

    #[test]
    fn test_condition_kind_logical_ops_rejected() {
        // AND / OR are not hardware-reducible single conditions
        let expr = Expr::Binary {
            op: BinaryOp::And,
            left: Box::new(Expr::Signal("a".to_string())),
            right: Box::new(Expr::Signal("b".to_string())),
        };
        assert!(ConditionKind::try_from_expr(&expr).is_err());
    }
}
