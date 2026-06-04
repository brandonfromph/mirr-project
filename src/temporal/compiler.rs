//! Temporal Guard Compiler
//!
//! Deterministic lowering of high-level temporal guards into
//! a bounded, verifiable low-level IR (shift registers / counters).
//!
//! Phase 3 interface notes:
//! - Public APIs validate inputs and return Result; callers must check returns.
//! - Fixed bounds: MAX_GUARDS and MAX_STAGES apply; loops must document explicit upper bounds.
//! - Resource budgets: worst-case stack <= 64 KiB; post-init heap = 0 (compile-time allocations allowed if bounded and documented).
//! - Determinism: any RNG or ordering must be seeded/injected and recorded in provenance.
//!
//! Implements the compilation pass that transforms high-level temporal guards
//! into low-level representations using shift registers and counters.

#![forbid(unsafe_code)]

use crate::ast::types::BinaryOp;
use crate::ast::{program::Guard, types::SignalType, Expr};
use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};
use crate::temporal::low_level_ir::{
    CompiledGuard, ConditionKind, GeneratedSignal, GeneratedSignalKind, TemporalNetlist,
};

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Adaptive threshold for choosing between shift registers and counters.
///
/// Guards with N ≤ 16 cycles use a shift register chain (direct pipeline).
/// Guards with N > 16 cycles use a counter-comparator (logarithmic area).
const SHIFT_REGISTER_THRESHOLD: u64 = 16;

/// Maximum nesting depth for compound (AND/OR) guard expressions.
///
/// NASA Power-of-10 rule: no unbounded recursion. The iterative work-stack
/// in `compile_guard` is bounded to `MAX_COMPILE_GUARD_DEPTH * 4` iterations.
const MAX_COMPILE_GUARD_DEPTH: usize = 64;

/// Work items for the explicit compilation stack.
/// Either compile a guard or combine two already-compiled sub-guards.
enum WorkItem {
    Compile(Guard),
    Combine { name: String, op: BinaryOp },
}

/// Temporal Guard Compiler
///
/// Responsible for lowering high-level temporal guards from the ECS Registry
/// into a bounded, verifiable low-level IR (shift registers or counters).
///
/// This compiler is "AI-Native" and optimized for the MIRR ECS architecture,
/// ensuring that the Registry remains the single source of truth throughout
/// the synthesis pass.
pub struct TemporalCompiler {
    /// Generated signal counter for unique naming
    pub signal_counter: u32,
    /// Current compilation context (accumulated netlist)
    pub context: TemporalNetlist,
    /// Cache of already compiled subguards to ensure deterministic deduplication
    pub cache: HashMap<String, CompiledGuard>,
}

impl Default for TemporalCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl TemporalCompiler {
    /// Create a new temporal compiler
    pub fn new() -> Self {
        Self { signal_counter: 0, context: TemporalNetlist::new(), cache: HashMap::new() }
    }

    /// Compile a module's temporal guards into a low-level netlist
    pub fn compile_module(&mut self, guards: &[Guard]) -> Result<TemporalNetlist, MirrError> {
        self.cache.clear();
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

    /// Recursively hash an expression for stable deduplication without string allocations.
    fn hash_expr_stable(
        &self,
        expr: &Expr,
        hasher: &mut std::collections::hash_map::DefaultHasher,
    ) {
        match expr {
            Expr::Literal(val) => {
                0u8.hash(hasher);
                match val {
                    crate::ast::types::LiteralValue::Bool(b) => b.hash(hasher),
                    crate::ast::types::LiteralValue::Integer(i) => i.hash(hasher),
                }
            }
            Expr::Signal(s) => {
                1u8.hash(hasher);
                s.hash(hasher);
            }
            Expr::Unary { op, operand } => {
                2u8.hash(hasher);
                op.hash(hasher);
                self.hash_expr_stable(operand, hasher);
            }
            Expr::Binary { op, left, right } => {
                3u8.hash(hasher);
                op.hash(hasher);
                self.hash_expr_stable(left, hasher);
                self.hash_expr_stable(right, hasher);
            }
            Expr::Prev { signal, delay } => {
                4u8.hash(hasher);
                signal.hash(hasher);
                delay.hash(hasher);
            }
            Expr::ArrayIndex { array, index } => {
                5u8.hash(hasher);
                self.hash_expr_stable(array, hasher);
                self.hash_expr_stable(index, hasher);
            }
            Expr::FieldAccess { object, field } => {
                6u8.hash(hasher);
                self.hash_expr_stable(object, hasher);
                field.hash(hasher);
            }
            Expr::ArrayLiteral(elems) => {
                7u8.hash(hasher);
                elems.len().hash(hasher);
                for e in elems {
                    self.hash_expr_stable(e, hasher);
                }
            }
            Expr::StructLiteral { name, fields } => {
                8u8.hash(hasher);
                name.hash(hasher);
                fields.len().hash(hasher);
                for (f, e) in fields {
                    f.hash(hasher);
                    self.hash_expr_stable(e, hasher);
                }
            }
            Expr::UnfoldIndex(s) => {
                9u8.hash(hasher);
                s.hash(hasher);
            }
        }
    }

    /// Generate a human-readable prefix for an expression, depth-bounded to prevent blowup.
    fn format_expr_short(&self, expr: &Expr, depth: usize) -> String {
        if depth > 3 {
            return "complex".to_string();
        }
        match expr {
            Expr::Literal(val) => match val {
                crate::ast::types::LiteralValue::Bool(b) => b.to_string(),
                crate::ast::types::LiteralValue::Integer(i) => i.to_string(),
            },
            Expr::Signal(s) => s.clone(),
            Expr::Unary { op, operand } => {
                let op_str = match op {
                    crate::ast::types::UnaryOp::Not => "not",
                    crate::ast::types::UnaryOp::Negate => "neg",
                };
                format!("{}_{}", op_str, self.format_expr_short(operand, depth + 1))
            }
            Expr::Binary { op, left, right } => {
                let op_str = match op {
                    BinaryOp::And => "and",
                    BinaryOp::Or => "or",
                    _ => "op",
                };
                format!(
                    "{}_{}_{}",
                    self.format_expr_short(left, depth + 1),
                    op_str,
                    self.format_expr_short(right, depth + 1)
                )
            }
            Expr::Prev { signal, delay } => format!("prev_{}_{}", signal, delay),
            _ => "expr".to_string(),
        }
    }

    /// Generate a deterministic, valid identifier guard name based on the expression and cycles.
    fn get_deterministic_name(&self, expr: &Expr, cycles: u64) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash_expr_stable(expr, &mut hasher);
        cycles.hash(&mut hasher);
        let hash_val = hasher.finish();

        let prefix = self.format_expr_short(expr, 0);
        let mut sanitized = String::new();
        for c in prefix.chars() {
            if c.is_alphanumeric() {
                sanitized.push(c);
            } else {
                sanitized.push('_');
            }
        }

        let mut clean = String::new();
        let mut prev_was_underscore = false;
        for c in sanitized.chars() {
            if c == '_' {
                if !prev_was_underscore {
                    clean.push(c);
                    prev_was_underscore = true;
                }
            } else {
                clean.push(c);
                prev_was_underscore = false;
            }
        }
        let clean = clean.trim_matches('_');
        let clean_trunc = if clean.len() > 24 { &clean[0..24] } else { clean };

        if clean_trunc.is_empty() {
            format!("sub_g_{:016x}", hash_val)
        } else {
            format!("sub_g_{}_{:016x}", clean_trunc.trim_end_matches('_'), hash_val)
        }
    }

    /// Compile a single temporal guard using the adaptive strategy.
    ///
    /// Uses an explicit work-stack instead of recursion to satisfy the
    /// NASA Power-of-10 "no recursion" rule. The loop is bounded to
    /// `MAX_COMPILE_GUARD_DEPTH * 4` iterations.
    fn compile_guard(&mut self, guard: &Guard) -> Result<CompiledGuard, MirrError> {
        let mut work_stack: Vec<WorkItem> = Vec::new();
        let mut result_stack: Vec<CompiledGuard> = Vec::new();
        work_stack.push(WorkItem::Compile(guard.clone()));

        let max_iterations = MAX_COMPILE_GUARD_DEPTH * 4;
        for _ in 0..max_iterations {
            let item = match work_stack.pop() {
                Some(item) => item,
                None => break,
            };
            match item {
                WorkItem::Compile(g) => {
                    self.compile_leaf_or_decompound(g, &mut work_stack, &mut result_stack)?
                }
                WorkItem::Combine { name, op } => {
                    self.combine_guard_results(name, op, &mut result_stack)?
                }
            }
        }

        if !work_stack.is_empty() {
            return Err(MirrError::TemporalCompilationError {
                message: format!(
                    "{} guard '{}': compilation exceeded maximum iteration bound ({})",
                    crate::error_codes::ec(304),
                    guard.name,
                    max_iterations
                ),
                span: guard.span,
            });
        }

        result_stack.pop().ok_or_else(|| MirrError::TemporalCompilationError {
            message: format!(
                "{} guard '{}': internal error — no compilation result produced",
                crate::error_codes::ec(305),
                guard.name
            ),
            span: guard.span,
        })
    }

    /// Handle a leaf guard (compile directly) or decompose a compound guard (push sub-guards).
    fn compile_leaf_or_decompound(
        &mut self,
        g: Guard,
        work_stack: &mut Vec<WorkItem>,
        result_stack: &mut Vec<CompiledGuard>,
    ) -> Result<(), MirrError> {
        if let Some(cached) = self.cache.get(&g.name) {
            result_stack.push(cached.clone());
            return Ok(());
        }

        // Bounded search for existing identical guards (NASA P10: bounded search).
        let mut matched = None;
        if let Ok(cond_kind) = ConditionKind::try_from_expr(&g.condition) {
            for existing in self.cache.values() {
                match existing {
                    CompiledGuard::ShiftRegister(sr) => {
                        if sr.condition_kind == cond_kind && sr.delay_cycles == g.cycles {
                            matched = Some(existing.clone());
                            break;
                        }
                    }
                    CompiledGuard::Counter(cg)
                        if cg.condition_kind == cond_kind && cg.target_count == g.cycles =>
                    {
                        matched = Some(existing.clone());
                        break;
                    }
                    _ => {}
                }
            }
        }
        if let Some(existing) = matched {
            self.cache.insert(g.name.clone(), existing.clone());
            result_stack.push(existing);
            return Ok(());
        }

        // WorkItem enum is defined in compile_guard; use fully qualified match
        match ConditionKind::try_from_expr(&g.condition) {
            Ok(_) => {
                let compiled = if g.cycles <= SHIFT_REGISTER_THRESHOLD {
                    self.compile_shift_register_guard(&g)?
                } else {
                    self.compile_counter_guard(&g)?
                };
                self.cache.insert(g.name.clone(), compiled.clone());
                result_stack.push(compiled);
                Ok(())
            }
            Err(_) => {
                if let Expr::Binary { op, left, right } = &g.condition {
                    if *op == BinaryOp::And || *op == BinaryOp::Or {
                        if work_stack.len() >= MAX_COMPILE_GUARD_DEPTH {
                            return Err(MirrError::TemporalCompilationError {
                                message: format!(
                                    "{} guard '{}': exceeded maximum compile guard depth ({})",
                                    crate::error_codes::ec(301),
                                    g.name,
                                    MAX_COMPILE_GUARD_DEPTH
                                ),
                                span: g.span,
                            });
                        }
                        let left_name = self.get_deterministic_name(left, g.cycles);
                        let right_name = self.get_deterministic_name(right, g.cycles);
                        let left_guard = Guard {
                            name: left_name,
                            condition: (*left.clone()),
                            cycles: g.cycles,
                            template_cycles: None,
                            origin: None,
                            span: None,
                        };
                        let right_guard = Guard {
                            name: right_name,
                            condition: (*right.clone()),
                            cycles: g.cycles,
                            template_cycles: None,
                            origin: None,
                            span: None,
                        };
                        // Use the WorkItem enum from compile_guard scope
                        work_stack.push(WorkItem::Combine { name: g.name.clone(), op: *op });
                        work_stack.push(WorkItem::Compile(right_guard));
                        work_stack.push(WorkItem::Compile(left_guard));
                        Ok(())
                    } else {
                        Err(MirrError::TemporalCompilationError {
                            message: format!("{} guard '{}': condition cannot be lowered to hardware — unsupported form", crate::error_codes::ec(302), g.name),
                            span: g.span,
                        })
                    }
                } else {
                    Err(MirrError::TemporalCompilationError {
                        message: format!("{} guard '{}': condition cannot be lowered to hardware — unsupported form", crate::error_codes::ec(302), g.name),
                        span: g.span,
                    })
                }
            }
        }
    }

    /// Pop two compiled sub-guards, combine them into a ComplexGuard, push result.
    fn combine_guard_results(
        &mut self,
        name: String,
        op: BinaryOp,
        result_stack: &mut Vec<CompiledGuard>,
    ) -> Result<(), MirrError> {
        let right_comp = result_stack.pop().ok_or_else(|| MirrError::TemporalCompilationError {
            message: format!(
                "{} guard '{}': internal error — missing right sub-guard result",
                crate::error_codes::ec(303),
                name
            ),
            span: None,
        })?;
        let left_comp = result_stack.pop().ok_or_else(|| MirrError::TemporalCompilationError {
            message: format!(
                "{} guard '{}': internal error — missing left sub-guard result",
                crate::error_codes::ec(303),
                name
            ),
            span: None,
        })?;

        let left_out = match &left_comp {
            CompiledGuard::ShiftRegister(sr) => sr.output_signal.clone(),
            CompiledGuard::Counter(c) => c.output_signal.clone(),
            CompiledGuard::Complex(cx) => cx.output_signal.clone(),
            CompiledGuard::DynamicCounter(dc) => dc.output_signal.clone(),
        };
        let right_out = match &right_comp {
            CompiledGuard::ShiftRegister(sr) => sr.output_signal.clone(),
            CompiledGuard::Counter(c) => c.output_signal.clone(),
            CompiledGuard::Complex(cx) => cx.output_signal.clone(),
            CompiledGuard::DynamicCounter(dc) => dc.output_signal.clone(),
        };

        let combo_expr = Expr::Binary {
            left: Box::new(Expr::Signal(left_out)),
            op: if op == BinaryOp::And { BinaryOp::And } else { BinaryOp::Or },
            right: Box::new(Expr::Signal(right_out)),
        };

        let complex = crate::temporal::low_level_ir::ComplexGuard::new(
            name.clone(),
            vec![left_comp, right_comp],
            combo_expr.clone(),
        );

        self.context.signals.push(GeneratedSignal {
            name: complex.output_signal.clone(),
            ty: SignalType::Bool,
            kind: GeneratedSignalKind::LogicGate,
            source: Some(combo_expr),
        });

        let compiled = CompiledGuard::Complex(complex);
        self.cache.insert(name, compiled.clone());
        result_stack.push(compiled);
        Ok(())
    }

    fn compile_shift_register_guard(&mut self, guard: &Guard) -> Result<CompiledGuard, MirrError> {
        let condition_kind = self.lower_condition(guard)?;
        let input_signal = condition_kind.primary_signal().to_owned();
        let total_delay = match &condition_kind {
            ConditionKind::PrevSignal { delay, .. } => guard.cycles.saturating_add(*delay),
            _ => guard.cycles,
        };

        self.synthesize_shift_register(&guard.name, &input_signal, total_delay, condition_kind)
    }

    fn compile_counter_guard(&mut self, guard: &Guard) -> Result<CompiledGuard, MirrError> {
        let condition_kind = self.lower_condition(guard)?;
        let input_signal = condition_kind.primary_signal().to_owned();
        let total_delay = match &condition_kind {
            ConditionKind::PrevSignal { delay, .. } => guard.cycles.saturating_add(*delay),
            _ => guard.cycles,
        };

        self.synthesize_counter(&guard.name, &input_signal, total_delay, condition_kind)
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
                    "{} guard '{}': condition cannot be lowered to hardware — {}",
                    crate::error_codes::ec(306),
                    guard.name,
                    reason
                ),
                span: guard.span,
            }
        })
    }

    /// Synthesize a guard entity from the ECS Registry into Temporal IR.
    ///
    /// This method is the primary entry point for ECS-native temporal lowering.
    /// It uses an iterative work-stack to handle compound (AND/OR) guards
    /// directly from the Registry, fulfilling the AI-Native synthesis mandate
    /// while complying with NASA Power-of-10 (no recursion).
    ///
    /// # Arguments
    /// * `registry` - The ECS Registry containing the guard and its condition tree.
    /// * `guard_entity` - The ID of the guard entity to be synthesized.
    ///
    /// # Process
    /// 1. Hydrates the guard's name and delay cycles from the Registry.
    /// 2. Decomposes compound conditions (AND/OR) into sub-guards iteratively.
    /// 3. Lowers leaf conditions into `ConditionKind` IR.
    /// 4. Selects an adaptive implementation strategy (ShiftRegister vs Counter)
    ///    based on the total delay depth.
    /// 5. Accumulates generated hardware signals into the compiler's context.
    ///
    /// # Errors
    /// Returns a `String` error if the condition tree is malformed, exceeds
    /// nesting limits, or contains unsupported expression forms.
    pub fn lower_guard_to_ecs(
        &mut self,
        registry: &crate::ecs::Registry,
        guard_entity: crate::ecs::EntityId,
    ) -> Result<CompiledGuard, MirrError> {
        let idx = guard_entity.0 as usize;

        let name = registry.names[idx]
            .as_ref()
            .ok_or_else(|| {
                mirrcode(
                    ErrorCode::TemporalCondLowerFailed,
                    format!("Guard entity {} missing NameComponent", guard_entity.0),
                )
            })?
            .0
            .clone();

        let cycles = registry.cycles[idx]
            .ok_or_else(|| {
                mirrcode(
                    ErrorCode::TemporalCondLowerFailed,
                    format!("Guard entity {} missing CyclesComponent", guard_entity.0),
                )
            })?
            .0;

        let cond_id = if name == "always" {
            None
        } else {
            Some(
                registry.conditions[idx]
                    .ok_or_else(|| {
                        mirrcode(
                            ErrorCode::TemporalCondLowerFailed,
                            format!("Guard entity {} missing ConditionComponent", guard_entity.0),
                        )
                    })?
                    .0,
            )
        };

        // Iterative stack for ECS synthesis
        enum ECSWork {
            Lower(crate::ecs::EntityId, String, u64),
            Combine(String, BinaryOp),
            Always(String, u64),
        }

        let mut work_stack = if let Some(cid) = cond_id {
            vec![ECSWork::Lower(cid, name.clone(), cycles)]
        } else {
            vec![ECSWork::Always(name.clone(), cycles)]
        };
        let mut result_stack: Vec<CompiledGuard> = Vec::new();

        let max_iterations = MAX_COMPILE_GUARD_DEPTH * 4;
        for _ in 0..max_iterations {
            let item = match work_stack.pop() {
                Some(item) => item,
                None => break,
            };

            match item {
                ECSWork::Lower(entity_id, current_name, current_cycles) => {
                    if let Some(cached) = self.cache.get(&current_name) {
                        result_stack.push(cached.clone());
                        continue;
                    }

                    // Bounded search for existing identical guards (NASA P10: bounded search).
                    let mut matched = None;
                    if let Ok(cond_kind) = ConditionKind::try_from_ecs(registry, entity_id) {
                        for existing in self.cache.values() {
                            match existing {
                                CompiledGuard::ShiftRegister(sr) => {
                                    if sr.condition_kind == cond_kind
                                        && sr.delay_cycles == current_cycles
                                    {
                                        matched = Some(existing.clone());
                                        break;
                                    }
                                }
                                CompiledGuard::Counter(cg) => {
                                    if cg.condition_kind == cond_kind
                                        && cg.target_count == current_cycles
                                    {
                                        matched = Some(existing.clone());
                                        break;
                                    }
                                }
                                CompiledGuard::Complex(cx) if cx.name == current_name => {
                                    // Match complex guards by name and cycle consistency
                                    matched = Some(existing.clone());
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    if let Some(existing) = matched {
                        self.cache.insert(current_name.clone(), existing.clone());
                        result_stack.push(existing);
                        continue;
                    }

                    let ent_idx = entity_id.0 as usize;

                    // Check for Compound Guard (AND/OR) in ECS
                    if let Some(binary) = &registry.binary_ops[ent_idx] {
                        if (binary.op == BinaryOp::And || binary.op == BinaryOp::Or)
                            && work_stack.len() < MAX_COMPILE_GUARD_DEPTH
                        {
                            let left_expr = registry.reify_expr(binary.left)?;
                            let right_expr = registry.reify_expr(binary.right)?;
                            let left_name = self.get_deterministic_name(&left_expr, current_cycles);
                            let right_name =
                                self.get_deterministic_name(&right_expr, current_cycles);

                            work_stack.push(ECSWork::Combine(current_name, binary.op));
                            work_stack.push(ECSWork::Lower(
                                binary.right,
                                right_name,
                                current_cycles,
                            ));
                            work_stack.push(ECSWork::Lower(binary.left, left_name, current_cycles));
                            continue;
                        } else if binary.op == BinaryOp::And || binary.op == BinaryOp::Or {
                            return Err(mirrcode(
                                ErrorCode::TemporalGuardDepth,
                                format!("Guard '{}' exceeds maximum nesting depth", current_name),
                            ));
                        }
                    }

                    // Otherwise, lower as a leaf ConditionKind
                    let condition_kind =
                        ConditionKind::try_from_ecs(registry, entity_id).map_err(|e| {
                            // Wrap the condition lowering error into a synthesis failure.
                            mirrcode(
                                ErrorCode::TemporalCondLowerFailed,
                                format!("Temporal synthesis failed for guard '{}': {}", name, e),
                            )
                        })?;

                    let input_signal = condition_kind.primary_signal().to_owned();
                    let total_delay = match &condition_kind {
                        ConditionKind::PrevSignal { delay, .. } => {
                            current_cycles.saturating_add(*delay)
                        }
                        _ => current_cycles,
                    };

                    let compiled = if total_delay <= SHIFT_REGISTER_THRESHOLD {
                        self.synthesize_shift_register(
                            &current_name,
                            &input_signal,
                            total_delay,
                            condition_kind,
                        )?
                    } else {
                        self.synthesize_counter(
                            &current_name,
                            &input_signal,
                            total_delay,
                            condition_kind,
                        )?
                    };
                    self.cache.insert(current_name, compiled.clone());
                    result_stack.push(compiled);
                }
                ECSWork::Always(current_name, total_delay) => {
                    let condition_kind = ConditionKind::AlwaysTrue;
                    let input_signal = "true";

                    let compiled = if total_delay <= SHIFT_REGISTER_THRESHOLD {
                        self.synthesize_shift_register(
                            &current_name,
                            input_signal,
                            total_delay,
                            condition_kind,
                        )?
                    } else {
                        self.synthesize_counter(
                            &current_name,
                            input_signal,
                            total_delay,
                            condition_kind,
                        )?
                    };
                    self.cache.insert(current_name, compiled.clone());
                    result_stack.push(compiled);
                }
                ECSWork::Combine(name, op) => {
                    let right_comp = result_stack.pop().ok_or_else(|| {
                        mirrcode(
                            ErrorCode::TemporalMissingSubguard,
                            "Internal error: missing right sub-guard",
                        )
                    })?;
                    let left_comp = result_stack.pop().ok_or_else(|| {
                        mirrcode(
                            ErrorCode::TemporalMissingSubguard,
                            "Internal error: missing left sub-guard",
                        )
                    })?;

                    let left_out = left_comp.output_signal().to_string();
                    let right_out = right_comp.output_signal().to_string();

                    let combo_expr = Expr::Binary {
                        left: Box::new(Expr::Signal(left_out)),
                        op,
                        right: Box::new(Expr::Signal(right_out)),
                    };

                    let complex = crate::temporal::low_level_ir::ComplexGuard::new(
                        name.clone(),
                        vec![left_comp, right_comp],
                        combo_expr.clone(),
                    );

                    self.context.signals.push(GeneratedSignal {
                        name: complex.output_signal.clone(),
                        ty: SignalType::Bool,
                        kind: GeneratedSignalKind::LogicGate,
                        source: Some(combo_expr),
                    });

                    let compiled = CompiledGuard::Complex(complex);
                    self.cache.insert(name, compiled.clone());
                    result_stack.push(compiled);
                }
            }
        }

        result_stack
            .pop()
            .ok_or_else(|| mirrcode(ErrorCode::TemporalNoResult, "No compilation result produced"))
    }

    /// Core synthesis: Lowers a condition to a Shift Register implementation.
    pub fn synthesize_shift_register(
        &mut self,
        name: &str,
        input_signal: &str,
        delay: u64,
        kind: ConditionKind,
    ) -> Result<CompiledGuard, MirrError> {
        let sr_guard = crate::temporal::low_level_ir::ShiftRegisterGuard::new(
            name.to_string(),
            input_signal.to_string(),
            delay,
            kind,
        );

        for (i, stage_name) in sr_guard.stages.iter().enumerate() {
            self.context
                .signals
                .push(GeneratedSignal::shift_register_stage(stage_name.clone(), i as u32));
        }

        self.context.signals.push(GeneratedSignal {
            name: sr_guard.output_signal.clone(),
            ty: SignalType::Bool,
            kind: GeneratedSignalKind::LogicGate,
            source: None,
        });

        Ok(CompiledGuard::ShiftRegister(sr_guard))
    }

    /// Core synthesis: Lowers a condition to a Counter-Comparator implementation.
    pub fn synthesize_counter(
        &mut self,
        name: &str,
        input_signal: &str,
        delay: u64,
        kind: ConditionKind,
    ) -> Result<CompiledGuard, MirrError> {
        let counter_guard = crate::temporal::low_level_ir::CounterGuard::new(
            name.to_string(),
            input_signal.to_string(),
            delay,
            kind,
        );

        self.context.signals.push(GeneratedSignal::counter(
            counter_guard.counter_signal.clone(),
            counter_guard.counter_width(),
        ));

        self.context
            .signals
            .push(GeneratedSignal::comparator(counter_guard.comparator_signal.clone()));

        self.context.signals.push(GeneratedSignal {
            name: counter_guard.output_signal.clone(),
            ty: SignalType::Bool,
            kind: GeneratedSignalKind::LogicGate,
            source: None,
        });

        Ok(CompiledGuard::Counter(counter_guard))
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
            template_cycles: None,
            origin: None,
            span: None,
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

    #[test]
    fn test_chaos_e303_missing_subguard() {
        let mut compiler = TemporalCompiler::new();
        let mut result_stack = Vec::new();
        // Calling combine_guard_results with empty stack should trigger E303
        let res =
            compiler.combine_guard_results("chaos".to_string(), BinaryOp::And, &mut result_stack);
        match res {
            Err(MirrError::TemporalCompilationError { message, .. }) => {
                assert!(message.contains("[E303]"), "Expected E303, got: {}", message);
            }
            _ => panic!("Expected E303 error, got {:?}", res),
        }
    }

    #[test]
    fn test_chaos_e304_iter_budget() {
        let mut compiler = TemporalCompiler::new();
        fn build_balanced_tree(n: usize, offset: usize) -> Expr {
            if n == 1 {
                Expr::Signal(format!("s{}", offset))
            } else {
                let half = n / 2;
                Expr::Binary {
                    left: Box::new(build_balanced_tree(half, offset)),
                    op: BinaryOp::And,
                    right: Box::new(build_balanced_tree(n - half, offset + half)),
                }
            }
        }
        let expr = build_balanced_tree(200, 0);
        let guard = Guard {
            name: "overloaded".to_string(),
            condition: expr,
            cycles: 1,
            template_cycles: None,
            origin: None,
            span: None,
        };
        let res = compiler.compile_guard(&guard);
        match res {
            Err(MirrError::TemporalCompilationError { message, .. }) => {
                assert!(message.contains("[E304]"), "Expected E304, got: {}", message);
            }
            _ => panic!("Expected E304 error, got {:?}", res),
        }
    }
}
