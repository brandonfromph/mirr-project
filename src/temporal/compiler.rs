// Module: temporal/compiler
// Responsibility: Deterministic lowering of high-level temporal guards into
// a bounded, verifiable low-level IR (shift registers / counters).
//
// Phase 3 interface notes:
// - Public APIs validate inputs and return Result; callers must check returns.
// - Fixed bounds: MAX_GUARDS and MAX_STAGES apply; loops must document explicit upper bounds.
// - Resource budgets: worst-case stack <= 64 KiB; post-init heap = 0 (compile-time allocations allowed if bounded and documented).
// - Determinism: any RNG or ordering must be seeded/injected and recorded in provenance.
//
//! Temporal Guard Compiler
//!
//! Implements the compilation pass that transforms high-level temporal guards
//! into low-level representations using shift registers and counters.

use crate::ast::{program::Guard, types::SignalType};
use crate::error::MirrError;
use crate::temporal::low_level_ir::{
    CompiledGuard, ConditionKind, GeneratedSignal, GeneratedSignalKind, TemporalNetlist,
};

/// Adaptive threshold for choosing between shift registers and counters.
///
/// Guards with N ≤ 16 cycles use a shift register chain (direct pipeline).
/// Guards with N > 16 cycles use a counter-comparator (logarithmic area).
const SHIFT_REGISTER_THRESHOLD: u64 = 16;

/// Temporal Guard Compiler
///
/// Compiles high-level temporal guards into low-level hardware representations
/// using an adaptive strategy based on delay length.
pub struct TemporalCompiler {
    /// Generated signal counter for unique naming
    signal_counter: u32,
    /// Current compilation context
    context: CompilationContext,
}

/// Compilation context containing state during one compilation run
#[derive(Debug, Default)]
struct CompilationContext {
    /// Generated signals accumulated across all guards
    signals: Vec<GeneratedSignal>,
    /// Compiled guards (unused at this level; guards live in the netlist)
    guards: Vec<CompiledGuard>,
}

impl Default for TemporalCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl TemporalCompiler {
    /// Create a new temporal compiler
    pub fn new() -> Self {
        Self { signal_counter: 0, context: CompilationContext::default() }
    }

    /// Compile a module's temporal guards into a low-level netlist
    pub fn compile_module(&mut self, guards: &[Guard]) -> Result<TemporalNetlist, MirrError> {
        let mut netlist = TemporalNetlist::new();

        for guard in guards {
            let compiled = self.compile_guard(guard)?;
            netlist.add_guard(compiled);
        }

        for signal in &self.context.signals {
            netlist.add_signal(signal.clone());
        }

        Ok(netlist)
    }

    /// Compile a single temporal guard using the adaptive strategy
    fn compile_guard(&mut self, guard: &Guard) -> Result<CompiledGuard, MirrError> {
        // First, try lowering to a simple ConditionKind.  If that succeeds
        // we keep the existing adaptive strategy.  If it fails with an
        // "unsupported condition expression form" we attempt to detect a
        // binary logical combination (AND/OR) and recursively lower it into a
        // ComplexGuard.
        match ConditionKind::try_from_expr(&guard.condition) {
            Ok(_) => {
                if guard.cycles <= SHIFT_REGISTER_THRESHOLD {
                    self.compile_shift_register_guard(guard)
                } else {
                    self.compile_counter_guard(guard)
                }
            }
            Err(_) => {
                // attempt complex boolean combination
                if let crate::ast::Expr::Binary { op, left, right } = &guard.condition {
                    if *op == crate::ast::types::BinaryOp::And
                        || *op == crate::ast::types::BinaryOp::Or
                    {
                        // recursively compile each side as its own guard
                        let left_name = format!("{}_sub{}", guard.name, self.signal_counter);
                        self.signal_counter += 1;
                        let right_name = format!("{}_sub{}", guard.name, self.signal_counter);
                        self.signal_counter += 1;

                        let left_guard = Guard {
                            name: left_name.clone(),
                            condition: (*left.clone()),
                            cycles: guard.cycles,
                        };
                        let right_guard = Guard {
                            name: right_name.clone(),
                            condition: (*right.clone()),
                            cycles: guard.cycles,
                        };

                        let left_comp = self.compile_guard(&left_guard)?;
                        let right_comp = self.compile_guard(&right_guard)?;

                        // build expression referring to the subguards' output signals
                        let left_out = match &left_comp {
                            CompiledGuard::ShiftRegister(sr) => sr.output_signal.clone(),
                            CompiledGuard::Counter(c) => c.output_signal.clone(),
                            CompiledGuard::Complex(cx) => cx.output_signal.clone(),
                        };
                        let right_out = match &right_comp {
                            CompiledGuard::ShiftRegister(sr) => sr.output_signal.clone(),
                            CompiledGuard::Counter(c) => c.output_signal.clone(),
                            CompiledGuard::Complex(cx) => cx.output_signal.clone(),
                        };

                        let combo_expr = crate::ast::Expr::Binary {
                            left: Box::new(crate::ast::Expr::Signal(left_out.clone())),
                            op: if *op == crate::ast::types::BinaryOp::And {
                                crate::ast::types::BinaryOp::And
                            } else {
                                crate::ast::types::BinaryOp::Or
                            },
                            right: Box::new(crate::ast::Expr::Signal(right_out.clone())),
                        };

                        let complex = crate::temporal::low_level_ir::ComplexGuard::new(
                            guard.name.clone(),
                            vec![left_comp, right_comp],
                            combo_expr.clone(),
                        );

                        // record output signal as a logic gate for stats
                        self.context.signals.push(GeneratedSignal {
                            name: complex.output_signal.clone(),
                            ty: SignalType::Bool,
                            kind: GeneratedSignalKind::LogicGate,
                            source: Some(combo_expr),
                        });

                        return Ok(CompiledGuard::Complex(complex));
                    }
                }
                // otherwise propagate error as before
                Err(MirrError::TemporalCompilationError {
                    message: format!(
                        "guard '{}': condition cannot be lowered to hardware — unsupported form",
                        guard.name
                    ),
                })
            }
        }
    }

    /// Compile a guard using a shift register pipeline
    fn compile_shift_register_guard(&mut self, guard: &Guard) -> Result<CompiledGuard, MirrError> {
        let condition_kind = self.lower_condition(guard)?;
        let input_signal = condition_kind.primary_signal().to_owned();

        let sr_guard = crate::temporal::low_level_ir::ShiftRegisterGuard::new(
            guard.name.clone(),
            input_signal,
            guard.cycles,
            condition_kind,
        );

        // Generate shift register stage signals
        for (i, stage_name) in sr_guard.stages.iter().enumerate() {
            let signal = GeneratedSignal::shift_register_stage(stage_name.clone(), i as u32);
            self.context.signals.push(signal);
        }

        // Generate output signal
        self.context.signals.push(GeneratedSignal {
            name: sr_guard.output_signal.clone(),
            ty: SignalType::Bool,
            kind: GeneratedSignalKind::LogicGate,
            source: None,
        });

        Ok(CompiledGuard::ShiftRegister(sr_guard))
    }

    /// Compile a guard using a counter-comparator circuit
    fn compile_counter_guard(&mut self, guard: &Guard) -> Result<CompiledGuard, MirrError> {
        let condition_kind = self.lower_condition(guard)?;
        let input_signal = condition_kind.primary_signal().to_owned();

        let counter_guard = crate::temporal::low_level_ir::CounterGuard::new(
            guard.name.clone(),
            input_signal,
            guard.cycles,
            condition_kind,
        );

        // Generate counter register signal
        self.context.signals.push(GeneratedSignal::counter(
            counter_guard.counter_signal.clone(),
            counter_guard.counter_width(),
        ));

        // Generate comparator signal
        self.context
            .signals
            .push(GeneratedSignal::comparator(counter_guard.comparator_signal.clone()));

        // Generate output signal
        self.context.signals.push(GeneratedSignal {
            name: counter_guard.output_signal.clone(),
            ty: SignalType::Bool,
            kind: GeneratedSignalKind::LogicGate,
            source: None,
        });

        Ok(CompiledGuard::Counter(counter_guard))
    }

    /// Lower a guard condition to a [`ConditionKind`].
    ///
    /// # Errors
    ///
    /// Returns [`MirrError::TemporalCompilationError`] for any condition form
    /// that is not in the supported set.  There is **no silent fallback** —
    /// unsupported forms are compile errors. (P2-REQ-013)
    fn lower_condition(&self, guard: &Guard) -> Result<ConditionKind, MirrError> {
        ConditionKind::try_from_expr(&guard.condition).map_err(|reason| {
            MirrError::TemporalCompilationError {
                message: format!(
                    "guard '{}': condition cannot be lowered to hardware — {}",
                    guard.name, reason
                ),
            }
        })
    }

    /// Reset compiler state between compilation runs
    pub fn reset(&mut self) {
        self.signal_counter = 0;
        self.context = CompilationContext::default();
    }

    /// Return (signals generated, guards stored in context) for diagnostics
    pub fn get_statistics(&self) -> (usize, usize) {
        (self.context.signals.len(), self.context.guards.len())
    }
}

// ---------------------------------------------------------------------------
// Hardware Resource Estimator
// ---------------------------------------------------------------------------

/// Estimates the hardware resources needed for different compilation strategies
pub struct ResourceEstimator;

impl ResourceEstimator {
    /// Estimate resources for shift register implementation
    pub fn estimate_shift_register_resources(delay_cycles: u64) -> ResourceEstimate {
        ResourceEstimate {
            shift_registers: delay_cycles as usize,
            counters: 0,
            logic_gates: 1,
            total_signals: delay_cycles as usize + 1,
        }
    }

    /// Estimate resources for counter implementation
    pub fn estimate_counter_resources(delay_cycles: u64) -> ResourceEstimate {
        let counter_width =
            if delay_cycles == 0 { 1 } else { (delay_cycles as f64).log2().ceil() as usize + 1 };
        ResourceEstimate {
            shift_registers: 0,
            counters: 1,
            logic_gates: 2,
            total_signals: counter_width + 2,
        }
    }

    /// Choose the optimal implementation strategy for a given delay
    pub fn choose_optimal_strategy(delay_cycles: u64) -> ImplementationStrategy {
        if delay_cycles <= SHIFT_REGISTER_THRESHOLD {
            ImplementationStrategy::ShiftRegister(Self::estimate_shift_register_resources(
                delay_cycles,
            ))
        } else {
            ImplementationStrategy::Counter(Self::estimate_counter_resources(delay_cycles))
        }
    }
}

/// Hardware resource usage estimate
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceEstimate {
    /// Number of shift registers needed
    pub shift_registers: usize,
    /// Number of counters needed
    pub counters: usize,
    /// Number of logic gates needed
    pub logic_gates: usize,
    /// Total number of signals
    pub total_signals: usize,
}

/// The chosen hardware implementation strategy
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplementationStrategy {
    /// Shift register pipeline
    ShiftRegister(ResourceEstimate),
    /// Counter-comparator circuit
    Counter(ResourceEstimate),
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::program::Guard;

    fn signal_guard(name: &str, signal: &str, cycles: u64) -> Guard {
        Guard {
            name: name.to_string(),
            condition: crate::ast::Expr::Signal(signal.to_string()),
            cycles,
        }
    }

    #[test]
    fn test_shift_register_compilation() {
        let mut compiler = TemporalCompiler::new();
        let guard = signal_guard("short_delay", "input_signal", 4);
        let compiled = compiler.compile_guard(&guard).expect("compile failed");

        match compiled {
            CompiledGuard::ShiftRegister(sr) => {
                assert_eq!(sr.name, "short_delay");
                assert_eq!(sr.delay_cycles, 4);
                assert_eq!(sr.stages.len(), 4);
                assert_eq!(sr.input_signal, "input_signal");
            }
            _ => panic!("Expected ShiftRegister guard"),
        }
    }

    #[test]
    fn test_counter_compilation() {
        let mut compiler = TemporalCompiler::new();
        let guard = signal_guard("long_delay", "input_signal", 100);
        let compiled = compiler.compile_guard(&guard).expect("compile failed");

        match compiled {
            CompiledGuard::Counter(c) => {
                assert_eq!(c.name, "long_delay");
                assert_eq!(c.target_count, 100);
                assert_eq!(c.counter_width(), 8); // ceil(log2(100))+1 = 8
                assert_eq!(c.input_signal, "input_signal");
            }
            _ => panic!("Expected Counter guard"),
        }
    }

    #[test]
    fn test_condition_kind_stored_in_compiled_ir() {
        // P2-REQ-016 / P2-REQ-017: the IR must carry the full ConditionKind.
        let mut compiler = TemporalCompiler::new();
        let guard = signal_guard("my_guard", "clk_en", 4);
        let compiled = compiler.compile_guard(&guard).expect("compile failed");

        match compiled {
            CompiledGuard::ShiftRegister(sr) => {
                assert_eq!(sr.condition_kind, ConditionKind::SimpleSignal("clk_en".to_string()));
            }
            _ => panic!("Expected ShiftRegister"),
        }
    }

    #[test]
    fn test_resource_estimation() {
        let sr = ResourceEstimator::estimate_shift_register_resources(4);
        assert_eq!(sr.shift_registers, 4);
        assert_eq!(sr.counters, 0);
        assert_eq!(sr.logic_gates, 1);
        assert_eq!(sr.total_signals, 5);

        let ctr = ResourceEstimator::estimate_counter_resources(100);
        assert_eq!(ctr.shift_registers, 0);
        assert_eq!(ctr.counters, 1);
        assert_eq!(ctr.logic_gates, 2);
        assert_eq!(ctr.total_signals, 10); // 8-bit counter + cmp + out
    }

    #[test]
    fn test_strategy_selection() {
        match ResourceEstimator::choose_optimal_strategy(8) {
            ImplementationStrategy::ShiftRegister(_) => {}
            _ => panic!("Expected ShiftRegister for N=8"),
        }
        match ResourceEstimator::choose_optimal_strategy(100) {
            ImplementationStrategy::Counter(_) => {}
            _ => panic!("Expected Counter for N=100"),
        }
    }
}
