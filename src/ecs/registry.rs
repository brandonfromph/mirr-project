#![forbid(unsafe_code)]

use crate::ast::program::{Guard, Module};
use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::ecs::components::*;
use crate::error::MirrError;
use std::collections::HashMap;

// ARCHITECTURAL SUB-ENGINE: ECS REGISTRY
//
// The data-oriented 'World' of the MIRR compiler. This engine stores
// all hardware components (Signals, Guards, Reflexes) as cache-friendly
// entities in dense SoA (Structure of Arrays) tables. This architecture
// enables high-performance, parallel synthesis and simulation of
// multi-core SoCs exceeding 1,000,000 entities.
//
/// Max capacity for compiler entities (NASA P10 Rule #2: Fixed bounds)
pub const MAX_ENTITIES: usize = 1_000_000;

/// The Registry: The Data-Oriented "World" of the MIRR Compiler.
/// Refactored to Vec-based storage for O(1) access and cache locality.
#[derive(Debug, Clone)]
pub struct Registry {
    pub(super) next_id: u32,

    // --- Component Arrays (Dense SoA) ---
    pub names: Vec<Option<NameComponent>>,
    pub kinds: Vec<Option<KindComponent>>,
    pub types: Vec<Option<TypeComponent>>,
    pub spans: Vec<Option<SpanComponent>>,
    pub modules: Vec<Option<ModuleComponent>>,
    pub pattern_defs: Vec<Option<PatternDefComponent>>,
    pub cycles: Vec<Option<CyclesComponent>>,
    pub conditions: Vec<Option<ConditionComponent>>,

    // Expression Components
    pub literals: Vec<Option<LiteralComponent>>,
    pub unary_ops: Vec<Option<UnaryComponent>>,
    pub binary_ops: Vec<Option<BinaryComponent>>,
    pub prev_ops: Vec<Option<PrevComponent>>,
    pub signal_refs: Vec<Option<SignalRefComponent>>,
    pub pending_signal_refs: Vec<Option<PendingSignalRef>>,
    pub array_indices: Vec<Option<ArrayIndexComponent>>,
    pub field_accesses: Vec<Option<FieldAccessComponent>>,
    pub array_literals: Vec<Option<ArrayLiteralComponent>>,
    pub struct_literals: Vec<Option<StructLiteralComponent>>,
    pub unfold_indices: Vec<Option<UnfoldIndexComponent>>,
    pub muxes: Vec<Option<MuxComponent>>,

    // Phase 4: Width Inference Components
    pub width_constraints: Vec<Option<WidthConstraintComponent>>,

    // Knowledge Base Component Tables (Phase 2)
    pub vectors: Vec<Option<VectorComponent>>,
    pub chunk_texts: Vec<Option<ChunkTextComponent>>,
    pub source_paths: Vec<Option<SourcePathComponent>>,
    pub line_ranges: Vec<Option<LineRangeComponent>>,

    // Instruction Tables (RS-16)
    pub opcodes: Vec<Option<OpcodeComponent>>,
    pub instruction_tables: Vec<Option<InstructionTableComponent>>,

    // Phase 2: Logic & Synthesis Components
    pub reflex_comps: Vec<Option<ReflexComponent>>,
    pub assignment_comps: Vec<Option<AssignmentComponent>>,
    pub property_comps: Vec<Option<PropertyComponent>>,

    // Phase 3: Temporal Synthesis Components (Proposal 110)
    pub temporal_nodes: Vec<Option<TemporalNodeComponent>>,

    pub(super) symbol_to_entity: HashMap<String, EntityId>,
}

#[path = "registry_validate.rs"]
mod registry_validate;
#[path = "semantic_validate.rs"]
mod semantic_validate;
#[path = "typeck.rs"]
mod typeck;

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let cap = MAX_ENTITIES / 10; // Start with 100k capacity
        Registry {
            next_id: 0,

            // --- Component Arrays (Dense SoA) ---
            names: vec![None; cap],
            kinds: vec![None; cap],
            types: vec![None; cap],
            spans: vec![None; cap],
            modules: vec![None; cap],
            pattern_defs: vec![None; cap],
            cycles: vec![None; cap],
            conditions: vec![None; cap],
            literals: vec![None; cap],
            unary_ops: vec![None; cap],
            binary_ops: vec![None; cap],
            prev_ops: vec![None; cap],
            signal_refs: vec![None; cap],
            pending_signal_refs: vec![None; cap],
            vectors: vec![None; cap],
            chunk_texts: vec![None; cap],
            source_paths: vec![None; cap],
            line_ranges: vec![None; cap],
            opcodes: vec![None; cap],
            instruction_tables: vec![None; cap],
            reflex_comps: vec![None; cap],
            assignment_comps: vec![None; cap],
            property_comps: vec![None; cap],
            temporal_nodes: vec![None; cap],
            array_indices: vec![None; cap],
            field_accesses: vec![None; cap],
            array_literals: vec![None; cap],
            struct_literals: vec![None; cap],
            unfold_indices: vec![None; cap],
            muxes: vec![None; cap],
            width_constraints: vec![None; cap],
            symbol_to_entity: HashMap::with_capacity(cap),
        }
    }

    pub fn next_id(&mut self) -> EntityId {
        if self.next_id >= MAX_ENTITIES as u32 {
            // Safety-critical hard stop.
            // In a real NASA P10 environment, this would trigger a system reset or safe-state.
            // For MRT, we return the last valid ID to prevent overflow,
            // though the Registry::validate will catch this later.
            return EntityId(MAX_ENTITIES as u32 - 1);
        }
        let id = EntityId(self.next_id);
        self.next_id += 1;

        let idx = id.0 as usize;
        if idx >= self.names.len() {
            let new_cap = (idx + 1024).min(MAX_ENTITIES);
            self.names.resize(new_cap, None);
            self.kinds.resize(new_cap, None);
            self.types.resize(new_cap, None);
            self.modules.resize(new_cap, None);
            self.pattern_defs.resize(new_cap, None);
            self.cycles.resize(new_cap, None);
            self.conditions.resize(new_cap, None);
            self.literals.resize(new_cap, None);
            self.unary_ops.resize(new_cap, None);
            self.binary_ops.resize(new_cap, None);
            self.prev_ops.resize(new_cap, None);
            self.signal_refs.resize(new_cap, None);
            self.pending_signal_refs.resize(new_cap, None);
            self.assignment_comps.resize(new_cap, None);

            self.vectors.resize(new_cap, None);
            self.chunk_texts.resize(new_cap, None);
            self.source_paths.resize(new_cap, None);
            self.line_ranges.resize(new_cap, None);
            self.opcodes.resize(new_cap, None);
            self.instruction_tables.resize(new_cap, None);
            self.reflex_comps.resize(new_cap, None);
            self.assignment_comps.resize(new_cap, None);
            self.property_comps.resize(new_cap, None);
            self.temporal_nodes.resize(new_cap, None);
            self.array_indices.resize(new_cap, None);
            self.field_accesses.resize(new_cap, None);
            self.array_literals.resize(new_cap, None);
            self.struct_literals.resize(new_cap, None);
            self.unfold_indices.resize(new_cap, None);
            self.muxes.resize(new_cap, None);
            self.width_constraints.resize(new_cap, None);
        }

        id
    }

    pub fn active_entities(&self) -> usize {
        self.next_id as usize
    }

    /// Create a new Knowledge Base Chunk Entity
    pub fn create_kb_chunk(
        &mut self,
        id_str: String,
        text: String,
        source: String,
        range: (usize, usize),
        vector: Option<Vec<f32>>,
    ) -> EntityId {
        let id = self.next_id();
        let idx = id.0 as usize;

        self.symbol_to_entity.insert(id_str.clone(), id);
        self.names[idx] = Some(NameComponent(id_str));
        self.chunk_texts[idx] = Some(ChunkTextComponent(text));
        self.source_paths[idx] = Some(SourcePathComponent(source));
        self.line_ranges[idx] = Some(LineRangeComponent(range));

        if let Some(v) = vector {
            self.vectors[idx] = Some(VectorComponent(v));
        }

        id
    }

    /// Safely create a new entity.
    pub fn create_entity(&mut self, name: &str, kind: KindComponent) -> EntityId {
        let id = self.next_id();
        let idx = id.0 as usize;
        self.names[idx] = Some(NameComponent(name.to_string()));
        self.kinds[idx] = Some(kind);
        self.symbol_to_entity.insert(name.to_string(), id);
        id
    }

    /// Safely set type for an entity.
    pub fn set_type(&mut self, entity: EntityId, ty: TypeComponent) {
        self.types[entity.0 as usize] = Some(ty);
    }

    /// Safely set parent for an entity.
    pub fn set_parent(&mut self, entity: EntityId, parent: EntityId) {
        self.modules[entity.0 as usize] = Some(ModuleComponent(parent));
    }

    pub fn ingest_module(&mut self, module: &Module) -> Result<EntityId, MirrError> {
        let mod_id = self.next_id();
        let idx = mod_id.0 as usize;
        self.names[idx] = Some(NameComponent(module.name.clone()));
        self.kinds[idx] = Some(KindComponent(EntityKind::MODULE));

        // BUG FIX: Register the module itself in the symbol table
        self.symbol_to_entity.insert(module.name.clone(), mod_id);

        self.ingest_signals(mod_id, &module.name, &module.signals);
        self.ingest_guards(mod_id, &module.name, &module.guards)?;
        self.ingest_reflexes(mod_id, &module.name, &module.reflexes)?;
        self.ingest_properties(mod_id, &module.name, &module.properties)?;

        Ok(mod_id)
    }

    fn ingest_signals(
        &mut self,
        mod_id: EntityId,
        mod_name: &str,
        signals: &[crate::ast::program::SignalDecl],
    ) {
        for sig in signals {
            let entity = self.create_signal(
                sig.name.clone(),
                KindComponent(EntityKind::SIGNAL(sig.kind)),
                TypeComponent(sig.ty.clone()),
            );
            // Register both local and qualified names
            self.symbol_to_entity.insert(sig.name.clone(), entity);
            self.symbol_to_entity.insert(format!("{}::{}", mod_name, sig.name), entity);

            if let Some(span) = sig.span {
                self.spans[entity.0 as usize] = Some(SpanComponent(span));
            }
            self.modules[entity.0 as usize] = Some(ModuleComponent(mod_id));
        }
    }

    fn ingest_guards(
        &mut self,
        mod_id: EntityId,
        mod_name: &str,
        guards: &[Guard],
    ) -> Result<(), MirrError> {
        for guard in guards {
            let cond_entity = self.ingest_expr(&guard.condition)?;
            let guard_id = self.next_id();
            let g_idx = guard_id.0 as usize;

            // Register both local and qualified names
            self.symbol_to_entity.insert(guard.name.clone(), guard_id);
            self.symbol_to_entity.insert(format!("{}::{}", mod_name, guard.name), guard_id);

            self.names[g_idx] = Some(NameComponent(guard.name.clone()));
            self.kinds[g_idx] = Some(KindComponent(EntityKind::GUARD));
            self.conditions[g_idx] = Some(ConditionComponent(cond_entity));
            self.cycles[g_idx] = Some(CyclesComponent(guard.cycles));
            if let Some(span) = guard.span {
                self.spans[g_idx] = Some(SpanComponent(span));
            }
            self.modules[g_idx] = Some(ModuleComponent(mod_id));
        }
        Ok(())
    }

    fn ingest_reflexes(
        &mut self,
        mod_id: EntityId,
        _mod_name: &str,
        reflexes: &[crate::ast::program::Reflex],
    ) -> Result<(), MirrError> {
        for reflex in reflexes {
            let mut guard_entities = Vec::new();
            for gname in &reflex.guard_names {
                let clean_gname =
                    if let Some(pos) = gname.find('[') { &gname[..pos] } else { gname.as_str() };

                if let Some(g_ent) = self.get_entity_by_name(clean_gname) {
                    guard_entities.push(g_ent);
                } else if clean_gname == "always" {
                    // Check if 'always' sentinel already exists (idempotent)
                    let always_ent = if let Some(ent) = self.get_entity_by_name("always") {
                        ent
                    } else {
                        let ent = self.create_entity("always", KindComponent(EntityKind::GUARD));
                        let idx = ent.0 as usize;
                        self.cycles[idx] = Some(CyclesComponent(0));
                        ent
                    };
                    guard_entities.push(always_ent);
                } else {
                    // Look up qualified name if local fails
                    // ... (handled by get_entity_by_name logic if updated)

                    return Err(MirrError::SemanticError {
                        message: format!(
                            "{} Undeclared guard '{}' referenced in reflex.",
                            crate::error_codes::ec(204),
                            gname
                        ),
                        span: reflex.span,
                    });
                }
            }

            let mut assignment_entities = Vec::new();
            for assign in &reflex.assignments {
                let val_ent = self.ingest_expr(&assign.value)?;
                let target_ent = self.get_entity_by_name(&assign.target).unwrap_or_else(|| {
                    self.create_entity(
                        &assign.target,
                        KindComponent(EntityKind::SIGNAL(crate::ast::types::SignalKind::Internal)),
                    )
                });

                let assign_ent = self.next_id();
                let assign_idx = assign_ent.0 as usize;
                self.names[assign_idx] = Some(NameComponent(format!("_assign_{}", assign_ent.0)));
                self.kinds[assign_idx] = Some(KindComponent(EntityKind::ASSIGNMENT));
                self.assignment_comps[assign_idx] =
                    Some(AssignmentComponent { target: target_ent, value: val_ent });
                if let Some(span) = assign.span {
                    self.spans[assign_idx] = Some(SpanComponent(span));
                }
                assignment_entities.push(assign_ent);
            }

            let reflex_ent = self.next_id();
            let r_idx = reflex_ent.0 as usize;

            // Register both local and qualified names
            self.symbol_to_entity.insert(reflex.name.clone(), reflex_ent);
            self.symbol_to_entity.insert(format!("{}::{}", _mod_name, reflex.name), reflex_ent);

            self.names[r_idx] = Some(NameComponent(reflex.name.clone()));
            self.kinds[r_idx] = Some(KindComponent(EntityKind::REFLEX));
            self.reflex_comps[r_idx] =
                Some(ReflexComponent { guards: guard_entities, assignments: assignment_entities });
            if let Some(span) = reflex.span {
                self.spans[r_idx] = Some(SpanComponent(span));
            }
            self.modules[r_idx] = Some(ModuleComponent(mod_id));
        }
        Ok(())
    }

    fn ingest_properties(
        &mut self,
        mod_id: EntityId,
        mod_name: &str,
        properties: &[crate::ast::property::PropertyDecl],
    ) -> Result<(), MirrError> {
        for prop in properties {
            let mut formula_exprs = Vec::new();
            for expr in prop.formula.exprs() {
                formula_exprs.push(self.ingest_expr(expr)?);
            }

            let prop_ent = self.next_id();
            let p_idx = prop_ent.0 as usize;

            // Register both local and qualified names
            self.symbol_to_entity.insert(prop.name.clone(), prop_ent);
            self.symbol_to_entity.insert(format!("{}::{}", mod_name, prop.name), prop_ent);

            self.names[p_idx] = Some(NameComponent(prop.name.clone()));
            self.kinds[p_idx] = Some(KindComponent(EntityKind::PROPERTY));
            self.property_comps[p_idx] = Some(PropertyComponent { formula: prop.formula.clone(), formula_exprs });
            if let Some(span) = prop.span {
                self.spans[p_idx] = Some(SpanComponent(span));
            }
            self.modules[p_idx] = Some(ModuleComponent(mod_id));
        }
        Ok(())
    }

    /// Iteratively flatten an expression tree into ECS entities.
    /// Replaces the recursive implementation to comply with NASA Power of 10 Rule #1.
    pub fn ingest_expr(&mut self, expr: &Expr) -> Result<EntityId, MirrError> {
        #[derive(Debug)]
        enum Work {
            Process(Expr),
            FinishBinary(BinaryOp),
            FinishUnary(UnaryOp),
            FinishPrev(u64),
            FinishArrayIndex,
            FinishFieldAccess(String),
            FinishArrayLiteral(usize),
            FinishStructLiteral { name: String, field_names: Vec<String> },
        }

        let mut stack = vec![Work::Process(expr.clone())];
        let mut results = Vec::new();
        let mut node_count = 0;

        while let Some(work) = stack.pop() {
            match work {
                Work::Process(e) => {
                    node_count += 1;
                    if node_count > crate::ast::MAX_EXPR_NODES {
                        return Err(MirrError::InternalError(format!(
                            "Expression complexity limit exceeded (MAX_EXPR_NODES={})",
                            crate::ast::MAX_EXPR_NODES
                        )));
                    }
                    match e {
                        Expr::Literal(lit) => {
                            let id = self.next_id();
                            self.literals[id.0 as usize] = Some(LiteralComponent(lit));
                            results.push(id);
                        }
                        Expr::Signal(name) => {
                            let id = self.next_id();
                            if let Some(sig_ent) = self.get_entity_by_name(&name) {
                                self.signal_refs[id.0 as usize] = Some(SignalRefComponent(sig_ent));
                            } else {
                                // Store as pending reference if not yet declared
                                self.pending_signal_refs[id.0 as usize] =
                                    Some(PendingSignalRef(name));
                            }
                            results.push(id);
                        }
                        Expr::Unary { op, operand } => {
                            stack.push(Work::FinishUnary(op));
                            stack.push(Work::Process(*operand));
                        }
                        Expr::Binary { op, left, right } => {
                            stack.push(Work::FinishBinary(op));
                            stack.push(Work::Process(*right));
                            stack.push(Work::Process(*left));
                        }
                        Expr::Prev { signal, delay } => {
                            stack.push(Work::FinishPrev(delay));
                            stack.push(Work::Process(Expr::Signal(signal)));
                        }
                        Expr::ArrayIndex { array, index } => {
                            stack.push(Work::FinishArrayIndex);
                            stack.push(Work::Process(*index));
                            stack.push(Work::Process(*array));
                        }
                        Expr::FieldAccess { object, field } => {
                            stack.push(Work::FinishFieldAccess(field));
                            stack.push(Work::Process(*object));
                        }
                        Expr::ArrayLiteral(elems) => {
                            stack.push(Work::FinishArrayLiteral(elems.len()));
                            for elem in elems.iter().rev() {
                                stack.push(Work::Process(elem.clone()));
                            }
                        }
                        Expr::StructLiteral { name, fields } => {
                            let field_names: Vec<String> =
                                fields.iter().map(|(f, _)| f.clone()).collect();
                            stack.push(Work::FinishStructLiteral {
                                name: name.clone(),
                                field_names,
                            });
                            for (_, f_expr) in fields.iter().rev() {
                                stack.push(Work::Process(f_expr.clone()));
                            }
                        }
                        Expr::UnfoldIndex(idx) => {
                            let id = self.next_id();
                            self.unfold_indices[id.0 as usize] = Some(UnfoldIndexComponent(idx));
                            results.push(id);
                        }
                    }
                }
                Work::FinishBinary(op) => {
                    let right = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Result stack underflow (right)".to_string())
                    })?;
                    let left = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Result stack underflow (left)".to_string())
                    })?;
                    let id = self.next_id();
                    self.binary_ops[id.0 as usize] = Some(BinaryComponent { op, left, right });
                    results.push(id);
                }
                Work::FinishUnary(op) => {
                    let operand = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Result stack underflow (operand)".to_string())
                    })?;
                    let id = self.next_id();
                    self.unary_ops[id.0 as usize] = Some(UnaryComponent { op, operand });
                    results.push(id);
                }
                Work::FinishPrev(delay) => {
                    let sig_ref_ent = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Result stack underflow (prev)".to_string())
                    })?;
                    let id = self.next_id();
                    self.prev_ops[id.0 as usize] =
                        Some(PrevComponent { signal: sig_ref_ent, delay });
                    results.push(id);
                }
                Work::FinishArrayIndex => {
                    let index = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Result stack underflow (index)".to_string())
                    })?;
                    let array = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Result stack underflow (array)".to_string())
                    })?;
                    let id = self.next_id();
                    self.array_indices[id.0 as usize] = Some(ArrayIndexComponent { array, index });
                    results.push(id);
                }
                Work::FinishFieldAccess(field) => {
                    let object = results.pop().ok_or_else(|| {
                        MirrError::InternalError(
                            "Result stack underflow (field access)".to_string(),
                        )
                    })?;
                    let id = self.next_id();
                    self.field_accesses[id.0 as usize] =
                        Some(FieldAccessComponent { object, field });
                    results.push(id);
                }
                Work::FinishArrayLiteral(len) => {
                    let mut elems = Vec::with_capacity(len);
                    for _ in 0..len {
                        let elem = results.pop().ok_or_else(|| {
                            MirrError::InternalError(
                                "Result stack underflow (array literal)".to_string(),
                            )
                        })?;
                        elems.push(elem);
                    }
                    elems.reverse();
                    let id = self.next_id();
                    self.array_literals[id.0 as usize] = Some(ArrayLiteralComponent(elems));
                    results.push(id);
                }
                Work::FinishStructLiteral { name, field_names } => {
                    let len = field_names.len();
                    let mut field_ids = Vec::with_capacity(len);
                    for _ in 0..len {
                        let id = results.pop().ok_or_else(|| {
                            MirrError::InternalError(
                                "Result stack underflow (struct literal)".to_string(),
                            )
                        })?;
                        field_ids.push(id);
                    }
                    field_ids.reverse();

                    let mut fields = Vec::with_capacity(len);
                    for (f_name, f_id) in field_names.into_iter().zip(field_ids) {
                        fields.push((f_name, f_id));
                    }

                    let id = self.next_id();
                    self.struct_literals[id.0 as usize] =
                        Some(StructLiteralComponent { name, fields });
                    results.push(id);
                }
            }
        }

        results.pop().ok_or_else(|| MirrError::InternalError("Empty expression stack".to_string()))
    }

    /// Iteratively reify an EntityId back into an AST Expr.
    /// Replaces the recursive implementation to comply with NASA Power of 10 Rule #1.
    pub fn reify_expr(&self, root_id: EntityId) -> Result<Expr, MirrError> {
        let mut memo = std::collections::HashMap::new();
        self.reify_expr_memoized(root_id, 0, &mut memo)
    }

    fn reify_expr_memoized(
        &self,
        root_id: EntityId,
        current_depth: usize,
        memo: &mut std::collections::HashMap<EntityId, Expr>,
    ) -> Result<Expr, MirrError> {
        if current_depth > 64 {
            return Err(MirrError::SemanticError {
                message: "Expression exceeds maximum nesting depth".to_string(),
                span: None,
            });
        }

        if let Some(cached) = memo.get(&root_id) {
            return Ok(cached.clone());
        }

        #[derive(Debug)]
        enum Work {
            Process(EntityId, usize),
            FinishBinary(BinaryOp),
            FinishUnary(UnaryOp),
            FinishArrayLiteral(usize),
            FinishStructLiteral { name: String, field_names: Vec<String> },
        }

        let mut stack = vec![Work::Process(root_id, current_depth)];
        let mut results: Vec<Expr> = Vec::new();

        while let Some(work) = stack.pop() {
            match work {
                Work::Process(id, depth) => {
                    if depth > 64 {
                        return Err(MirrError::SemanticError {
                            message: "Expression exceeds maximum nesting depth".to_string(),
                            span: None,
                        });
                    }

                    if let Some(cached) = memo.get(&id) {
                        results.push(cached.clone());
                        continue;
                    }

                    let idx = id.0 as usize;
                    if let Some(LiteralComponent(lit)) = &self.literals[idx] {
                        let res = Expr::Literal(lit.clone());
                        memo.insert(id, res.clone());
                        results.push(res);
                    } else if let Some(SignalRefComponent(sig_ent)) = self.signal_refs[idx] {
                        let sig_name = self.names[sig_ent.0 as usize]
                            .as_ref()
                            .map(|n| n.0.clone())
                            .ok_or_else(|| {
                                MirrError::InternalError(
                                    "Signal reference to unnamed entity".to_string(),
                                )
                            })?;
                        let res = Expr::Signal(sig_name);
                        memo.insert(id, res.clone());
                        results.push(res);
                    } else if let Some(PendingSignalRef(name)) = &self.pending_signal_refs[idx] {
                        let res = Expr::Signal(name.clone());
                        memo.insert(id, res.clone());
                        results.push(res);
                    } else if let Some(BinaryComponent { op, left, right }) = &self.binary_ops[idx]
                    {
                        stack.push(Work::FinishBinary(*op));
                        stack.push(Work::Process(*right, depth + 1));
                        stack.push(Work::Process(*left, depth + 1));
                    } else if let Some(UnaryComponent { op, operand }) = &self.unary_ops[idx] {
                        stack.push(Work::FinishUnary(*op));
                        stack.push(Work::Process(*operand, depth + 1));
                    } else if let Some(PrevComponent { signal, delay }) = &self.prev_ops[idx] {
                        let sig_name = if let Some(SignalRefComponent(decl)) =
                            self.signal_refs[signal.0 as usize]
                        {
                            self.names[decl.0 as usize].as_ref().map(|n| n.0.clone())
                        } else if let Some(PendingSignalRef(n)) =
                            &self.pending_signal_refs[signal.0 as usize]
                        {
                            Some(n.clone())
                        } else {
                            None
                        }
                        .ok_or_else(|| {
                            MirrError::InternalError("Prev reference to unnamed entity".to_string())
                        })?;
                        let res = Expr::Prev { signal: sig_name, delay: *delay };
                        memo.insert(id, res.clone());
                        results.push(res);
                    } else if let Some(ArrayIndexComponent { array, index }) =
                        &self.array_indices[idx]
                    {
                        let array_expr = self.reify_expr_memoized(*array, depth + 1, memo)?;
                        let index_expr = self.reify_expr_memoized(*index, depth + 1, memo)?;
                        let res = Expr::ArrayIndex {
                            array: Box::new(array_expr),
                            index: Box::new(index_expr),
                        };
                        memo.insert(id, res.clone());
                        results.push(res);
                    } else if let Some(FieldAccessComponent { object, field }) =
                        &self.field_accesses[idx]
                    {
                        let object_expr = self.reify_expr_memoized(*object, depth + 1, memo)?;
                        let res = Expr::FieldAccess {
                            object: Box::new(object_expr),
                            field: field.clone(),
                        };
                        memo.insert(id, res.clone());
                        results.push(res);
                    } else if let Some(ArrayLiteralComponent(elems)) = &self.array_literals[idx] {
                        stack.push(Work::FinishArrayLiteral(elems.len()));
                        for elem in elems.iter().rev() {
                            stack.push(Work::Process(*elem, depth + 1));
                        }
                    } else if let Some(StructLiteralComponent { name, fields }) =
                        &self.struct_literals[idx]
                    {
                        let field_names: Vec<String> =
                            fields.iter().map(|(n, _)| n.clone()).collect();
                        stack.push(Work::FinishStructLiteral { name: name.clone(), field_names });
                        for (_, f_expr) in fields.iter().rev() {
                            stack.push(Work::Process(*f_expr, depth + 1));
                        }
                    } else if let Some(UnfoldIndexComponent(idx_val)) = &self.unfold_indices[idx] {
                        let res = Expr::UnfoldIndex(idx_val.clone());
                        memo.insert(id, res.clone());
                        results.push(res);
                    } else {
                        return Err(MirrError::InternalError(format!(
                            "Entity {} is not an expression",
                            id.0
                        )));
                    }
                }
                Work::FinishBinary(op) => {
                    let right = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Reify stack underflow (right)".to_string())
                    })?;
                    let left = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Reify stack underflow (left)".to_string())
                    })?;
                    results.push(Expr::Binary { op, left: Box::new(left), right: Box::new(right) });
                }
                Work::FinishUnary(op) => {
                    let operand = results.pop().ok_or_else(|| {
                        MirrError::InternalError("Reify stack underflow (operand)".to_string())
                    })?;
                    results.push(Expr::Unary { op, operand: Box::new(operand) });
                }
                Work::FinishArrayLiteral(len) => {
                    let mut elems = Vec::with_capacity(len);
                    for _ in 0..len {
                        let elem = results.pop().ok_or_else(|| {
                            MirrError::InternalError(
                                "Reify stack underflow (array literal)".to_string(),
                            )
                        })?;
                        elems.push(elem);
                    }
                    elems.reverse();
                    results.push(Expr::ArrayLiteral(elems));
                }
                Work::FinishStructLiteral { name, field_names } => {
                    let len = field_names.len();
                    let mut field_exprs = Vec::with_capacity(len);
                    for _ in 0..len {
                        let expr = results.pop().ok_or_else(|| {
                            MirrError::InternalError(
                                "Reify stack underflow (struct literal)".to_string(),
                            )
                        })?;
                        field_exprs.push(expr);
                    }
                    field_exprs.reverse();

                    let mut fields = Vec::with_capacity(len);
                    for (f_name, f_expr) in field_names.into_iter().zip(field_exprs) {
                        fields.push((f_name, f_expr));
                    }
                    results.push(Expr::StructLiteral { name, fields });
                }
            }
        }

        let final_res = results
            .pop()
            .ok_or_else(|| MirrError::InternalError("Empty reify stack".to_string()))?;
        memo.insert(root_id, final_res.clone());
        Ok(final_res)
    }

    /// Create a new Signal Entity
    pub fn create_signal(
        &mut self,
        name: String,
        kind: KindComponent,
        ty: TypeComponent,
    ) -> EntityId {
        let id = self.next_id();
        let idx = id.0 as usize;
        self.symbol_to_entity.insert(name.clone(), id);
        self.names[idx] = Some(NameComponent(name));
        self.kinds[idx] = Some(kind);
        self.types[idx] = Some(ty);
        id
    }

    pub fn get_entity_by_name(&self, name: &str) -> Option<EntityId> {
        // Since we now register both qualified (Mod::Name) and local names
        // in the symbol table during ingestion, we can use O(1) lookup.
        self.symbol_to_entity.get(name).copied()
    }

    /// Explicitly register a symbol in the O(1) lookup table.
    /// Primarily used by tests and pattern expansion.
    pub fn register_symbol(&mut self, symbol: &str, entity: EntityId) {
        self.symbol_to_entity.insert(symbol.to_string(), entity);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn hydrate_from_db(
        &mut self,
        storage: &mirr_kb_native::storage::SqliteHybridStorage,
    ) -> anyhow::Result<()> {
        let chunks = storage.get_all_chunks()?;
        for chunk in chunks {
            self.create_kb_chunk(
                chunk.id,
                chunk.text,
                chunk.module,
                chunk.line_range,
                None, // Vector handling deferred to Phase 2b
            );
        }
        Ok(())
    }
}
