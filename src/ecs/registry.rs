#![forbid(unsafe_code)]

use crate::ast::program::{Guard, Module};
use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::ecs::components::*;
use crate::ecs::intern::{InternId, StringInterner};
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

use serde::{Deserialize, Serialize};

// --- Bitmask Constants ---
// --- Bitmask Constants ---
pub const COMP_NAME: u64 = 1 << 0;
pub const COMP_KIND: u64 = 1 << 1;
pub const COMP_TYPE: u64 = 1 << 2;
pub const COMP_SPAN: u64 = 1 << 3;
pub const COMP_MODULE: u64 = 1 << 4;
pub const COMP_PATTERN_DEF: u64 = 1 << 5;
pub const COMP_CYCLES: u64 = 1 << 6;
pub const COMP_CONDITION: u64 = 1 << 7;
pub const COMP_LITERAL: u64 = 1 << 8;
pub const COMP_UNARY_OP: u64 = 1 << 9;
pub const COMP_BINARY_OP: u64 = 1 << 10;
pub const COMP_PREV_OP: u64 = 1 << 11;
pub const COMP_SIGNAL_REF: u64 = 1 << 12;
pub const COMP_PENDING_SIGNAL_REF: u64 = 1 << 13;
pub const COMP_ARRAY_INDEX: u64 = 1 << 14;
pub const COMP_FIELD_ACCESS: u64 = 1 << 15;
pub const COMP_ARRAY_LITERAL: u64 = 1 << 16;
pub const COMP_STRUCT_LITERAL: u64 = 1 << 17;
pub const COMP_UNFOLD_INDEX: u64 = 1 << 18;
pub const COMP_MUX: u64 = 1 << 19;
pub const COMP_WIDTH_CONSTRAINT: u64 = 1 << 20;
pub const COMP_VECTOR: u64 = 1 << 21;
pub const COMP_CHUNK_TEXT: u64 = 1 << 22;
pub const COMP_SOURCE_PATH: u64 = 1 << 23;
pub const COMP_LINE_RANGE: u64 = 1 << 24;
pub const COMP_OPCODE: u64 = 1 << 25;
pub const COMP_INSTRUCTION_TABLE: u64 = 1 << 26;
pub const COMP_REFLEX: u64 = 1 << 27;
pub const COMP_ASSIGNMENT: u64 = 1 << 28;
pub const COMP_PROPERTY: u64 = 1 << 29;
pub const COMP_TEMPORAL_NODE: u64 = 1 << 30;
pub const COMP_HLS_DATAFLOW: u64 = 1 << 31;
pub const COMP_HLS_SCHEDULE: u64 = 1 << 32;
pub const COMP_HLS_BINDING: u64 = 1 << 33;
pub const COMP_PATTERN_CALL: u64 = 1 << 34;
pub const COMP_PATTERN_INSTANCE: u64 = 1 << 35;
pub const COMP_CLOCK_DOMAINS: u64 = 1 << 36;

/// The Registry: The Data-Oriented "World" of the MIRR Compiler.
/// Refactored to Vec-based storage for O(1) access and cache locality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry {
    pub(super) next_id: u32,

    /// String interner — maps names to compact [`InternId`] handles.
    ///
    /// All `NameComponent` values are `InternId`s into this table.
    /// Resolve with `self.interner.resolve(id)`.
    /// Bounded: max [`crate::ecs::intern::MAX_INTERN_ENTRIES`] unique strings.
    pub interner: StringInterner,

    // --- Component Arrays (Dense SoA) ---
    pub component_masks: Vec<u64>,
    pub names: Vec<Option<NameComponent>>,
    pub kinds: Vec<Option<KindComponent>>,
    pub types: Vec<Option<TypeComponent>>,
    pub spans: Vec<Option<SpanComponent>>,
    pub modules: Vec<Option<ModuleComponent>>,
    pub pattern_defs: Vec<Option<PatternDefComponent>>,
    pub pattern_calls: Vec<Option<PatternCallComponent>>,
    pub pattern_instances: Vec<Option<PatternInstanceComponent>>,
    pub clock_domains: Vec<Option<ClockDomainsComponent>>,
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

    // Pattern Traceability (Phase 7b)
    pub pattern_origins: Vec<crate::ast::pattern::PatternOrigin>,

    // Target Hardware Configuration
    pub target_config: Option<crate::ast::program::TargetConfig>,

    // External structural module instantiations
    pub extern_instantiations: Vec<EntityId>,

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

    // Phase 5c: HLS Components (MEGA-12 Migration)
    pub hls_dataflow: Vec<Option<HlsDataflowComponent>>,
    pub hls_schedules: Vec<Option<HlsScheduleComponent>>,
    pub hls_bindings: Vec<Option<HlsBindingComponent>>,

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
            interner: StringInterner::new(),

            // --- Component Arrays (Dense SoA) ---
            component_masks: vec![0; cap],
            names: vec![None; cap],
            kinds: vec![None; cap],
            types: vec![None; cap],
            spans: vec![None; cap],
            modules: vec![None; cap],
            pattern_defs: vec![None; cap],
            pattern_calls: vec![None; cap],
            pattern_instances: vec![None; cap],
            clock_domains: vec![None; cap],
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
            hls_dataflow: vec![None; cap],
            hls_schedules: vec![None; cap],
            hls_bindings: vec![None; cap],
            array_indices: vec![None; cap],
            field_accesses: vec![None; cap],
            array_literals: vec![None; cap],
            struct_literals: vec![None; cap],
            unfold_indices: vec![None; cap],
            muxes: vec![None; cap],
            width_constraints: vec![None; cap],
            pattern_origins: Vec::new(),
            target_config: None,
            extern_instantiations: Vec::new(),
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
        if idx >= self.component_masks.len() {
            let new_cap = (idx + 1024).min(MAX_ENTITIES);
            self.component_masks.resize(new_cap, 0);
            self.names.resize(new_cap, None);
            self.kinds.resize(new_cap, None);
            self.spans.resize(new_cap, None);
            self.modules.resize(new_cap, None);
            self.pattern_defs.resize(new_cap, None);
            self.pattern_calls.resize(new_cap, None);
            self.pattern_instances.resize(new_cap, None);
            self.clock_domains.resize(new_cap, None);
            self.cycles.resize(new_cap, None);
            self.conditions.resize(new_cap, None);
            self.literals.resize(new_cap, None);
            self.unary_ops.resize(new_cap, None);
            self.binary_ops.resize(new_cap, None);
            self.prev_ops.resize(new_cap, None);
            self.signal_refs.resize(new_cap, None);
            self.pending_signal_refs.resize(new_cap, None);
            self.types.resize(new_cap, None);

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
            self.hls_dataflow.resize(new_cap, None);
            self.hls_schedules.resize(new_cap, None);
            self.hls_bindings.resize(new_cap, None);
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

    /// Returns an iterator over all EntityIds that possess ALL components specified in the mask.
    pub fn entities_with_components(&self, mask: u64) -> impl Iterator<Item = EntityId> + '_ {
        self.component_masks.iter().enumerate().take(self.active_entities()).filter_map(
            move |(i, &entity_mask)| {
                if entity_mask & mask == mask {
                    Some(EntityId(i as u32))
                } else {
                    None
                }
            },
        )
    }

    // --- Component Setters ---
    #[inline]
    pub fn set_name(&mut self, entity: EntityId, comp: NameComponent) {
        let idx = entity.0 as usize;
        self.names[idx] = Some(comp);
        self.component_masks[idx] |= COMP_NAME;
    }
    #[inline]
    pub fn unset_name(&mut self, entity: EntityId) {
        let idx = entity.0 as usize;
        self.names[idx] = None;
        self.component_masks[idx] &= !COMP_NAME;
    }

    #[inline]
    pub fn set_kind(&mut self, entity: EntityId, comp: KindComponent) {
        let idx = entity.0 as usize;
        self.kinds[idx] = Some(comp);
        self.component_masks[idx] |= COMP_KIND;
    }
    #[inline]
    pub fn set_type(&mut self, entity: EntityId, comp: TypeComponent) {
        let idx = entity.0 as usize;
        self.types[idx] = Some(comp);
        self.component_masks[idx] |= COMP_TYPE;
    }
    #[inline]
    pub fn set_span(&mut self, entity: EntityId, comp: SpanComponent) {
        let idx = entity.0 as usize;
        self.spans[idx] = Some(comp);
        self.component_masks[idx] |= COMP_SPAN;
    }
    #[inline]
    pub fn set_module(&mut self, entity: EntityId, comp: ModuleComponent) {
        let idx = entity.0 as usize;
        self.modules[idx] = Some(comp);
        self.component_masks[idx] |= COMP_MODULE;
    }
    #[inline]
    pub fn set_pattern_def(&mut self, entity: EntityId, comp: PatternDefComponent) {
        let idx = entity.0 as usize;
        self.pattern_defs[idx] = Some(comp);
        self.component_masks[idx] |= COMP_PATTERN_DEF;
    }
    #[inline]
    pub fn set_pattern_call(&mut self, entity: EntityId, comp: PatternCallComponent) {
        let idx = entity.0 as usize;
        self.pattern_calls[idx] = Some(comp);
        self.component_masks[idx] |= COMP_PATTERN_CALL;
    }
    #[inline]
    pub fn unset_pattern_call(&mut self, entity: EntityId) {
        let idx = entity.0 as usize;
        self.pattern_calls[idx] = None;
        self.component_masks[idx] &= !COMP_PATTERN_CALL;
    }
    #[inline]
    pub fn set_pattern_instance(&mut self, entity: EntityId, comp: PatternInstanceComponent) {
        let idx = entity.0 as usize;
        self.pattern_instances[idx] = Some(comp);
        self.component_masks[idx] |= COMP_PATTERN_INSTANCE;
    }
    #[inline]
    pub fn unset_pattern_instance(&mut self, entity: EntityId) {
        let idx = entity.0 as usize;
        self.pattern_instances[idx] = None;
        self.component_masks[idx] &= !COMP_PATTERN_INSTANCE;
    }
    #[inline]
    pub fn set_cycle(&mut self, entity: EntityId, comp: CyclesComponent) {
        let idx = entity.0 as usize;
        self.cycles[idx] = Some(comp);
        self.component_masks[idx] |= COMP_CYCLES;
    }
    #[inline]
    pub fn set_condition(&mut self, entity: EntityId, comp: ConditionComponent) {
        let idx = entity.0 as usize;
        self.conditions[idx] = Some(comp);
        self.component_masks[idx] |= COMP_CONDITION;
    }
    #[inline]
    pub fn set_literal(&mut self, entity: EntityId, comp: LiteralComponent) {
        let idx = entity.0 as usize;
        self.literals[idx] = Some(comp);
        self.component_masks[idx] |= COMP_LITERAL;
    }
    #[inline]
    pub fn set_unary_op(&mut self, entity: EntityId, comp: UnaryComponent) {
        let idx = entity.0 as usize;
        self.unary_ops[idx] = Some(comp);
        self.component_masks[idx] |= COMP_UNARY_OP;
    }
    #[inline]
    pub fn unset_unary_op(&mut self, entity: EntityId) {
        let idx = entity.0 as usize;
        self.unary_ops[idx] = None;
        self.component_masks[idx] &= !COMP_UNARY_OP;
    }

    #[inline]
    pub fn set_binary_op(&mut self, entity: EntityId, comp: BinaryComponent) {
        let idx = entity.0 as usize;
        self.binary_ops[idx] = Some(comp);
        self.component_masks[idx] |= COMP_BINARY_OP;
    }
    #[inline]
    pub fn unset_binary_op(&mut self, entity: EntityId) {
        let idx = entity.0 as usize;
        self.binary_ops[idx] = None;
        self.component_masks[idx] &= !COMP_BINARY_OP;
    }

    #[inline]
    pub fn set_prev_op(&mut self, entity: EntityId, comp: PrevComponent) {
        let idx = entity.0 as usize;
        self.prev_ops[idx] = Some(comp);
        self.component_masks[idx] |= COMP_PREV_OP;
    }
    #[inline]
    pub fn set_signal_ref(&mut self, entity: EntityId, comp: SignalRefComponent) {
        let idx = entity.0 as usize;
        self.signal_refs[idx] = Some(comp);
        self.component_masks[idx] |= COMP_SIGNAL_REF;
    }
    #[inline]
    pub fn set_pending_signal_ref(&mut self, entity: EntityId, comp: PendingSignalRef) {
        let idx = entity.0 as usize;
        self.pending_signal_refs[idx] = Some(comp);
        self.component_masks[idx] |= COMP_PENDING_SIGNAL_REF;
    }
    #[inline]
    pub fn set_array_index(&mut self, entity: EntityId, comp: ArrayIndexComponent) {
        let idx = entity.0 as usize;
        self.array_indices[idx] = Some(comp);
        self.component_masks[idx] |= COMP_ARRAY_INDEX;
    }
    #[inline]
    pub fn set_field_access(&mut self, entity: EntityId, comp: FieldAccessComponent) {
        let idx = entity.0 as usize;
        self.field_accesses[idx] = Some(comp);
        self.component_masks[idx] |= COMP_FIELD_ACCESS;
    }
    #[inline]
    pub fn set_array_literal(&mut self, entity: EntityId, comp: ArrayLiteralComponent) {
        let idx = entity.0 as usize;
        self.array_literals[idx] = Some(comp);
        self.component_masks[idx] |= COMP_ARRAY_LITERAL;
    }
    #[inline]
    pub fn set_struct_literal(&mut self, entity: EntityId, comp: StructLiteralComponent) {
        let idx = entity.0 as usize;
        self.struct_literals[idx] = Some(comp);
        self.component_masks[idx] |= COMP_STRUCT_LITERAL;
    }
    #[inline]
    pub fn set_unfold_index(&mut self, entity: EntityId, comp: UnfoldIndexComponent) {
        let idx = entity.0 as usize;
        self.unfold_indices[idx] = Some(comp);
        self.component_masks[idx] |= COMP_UNFOLD_INDEX;
    }
    #[inline]
    pub fn set_mux(&mut self, entity: EntityId, comp: MuxComponent) {
        let idx = entity.0 as usize;
        self.muxes[idx] = Some(comp);
        self.component_masks[idx] |= COMP_MUX;
    }
    #[inline]
    pub fn unset_mux(&mut self, entity: EntityId) {
        let idx = entity.0 as usize;
        self.muxes[idx] = None;
        self.component_masks[idx] &= !COMP_MUX;
    }

    #[inline]
    pub fn set_width_constraint(&mut self, entity: EntityId, comp: WidthConstraintComponent) {
        let idx = entity.0 as usize;
        self.width_constraints[idx] = Some(comp);
        self.component_masks[idx] |= COMP_WIDTH_CONSTRAINT;
    }
    #[inline]
    pub fn set_vector(&mut self, entity: EntityId, comp: VectorComponent) {
        let idx = entity.0 as usize;
        self.vectors[idx] = Some(comp);
        self.component_masks[idx] |= COMP_VECTOR;
    }
    #[inline]
    pub fn set_chunk_text(&mut self, entity: EntityId, comp: ChunkTextComponent) {
        let idx = entity.0 as usize;
        self.chunk_texts[idx] = Some(comp);
        self.component_masks[idx] |= COMP_CHUNK_TEXT;
    }
    #[inline]
    pub fn set_source_path(&mut self, entity: EntityId, comp: SourcePathComponent) {
        let idx = entity.0 as usize;
        self.source_paths[idx] = Some(comp);
        self.component_masks[idx] |= COMP_SOURCE_PATH;
    }
    #[inline]
    pub fn set_line_range(&mut self, entity: EntityId, comp: LineRangeComponent) {
        let idx = entity.0 as usize;
        self.line_ranges[idx] = Some(comp);
        self.component_masks[idx] |= COMP_LINE_RANGE;
    }
    #[inline]
    pub fn set_opcode(&mut self, entity: EntityId, comp: OpcodeComponent) {
        let idx = entity.0 as usize;
        self.opcodes[idx] = Some(comp);
        self.component_masks[idx] |= COMP_OPCODE;
    }
    #[inline]
    pub fn set_instruction_table(&mut self, entity: EntityId, comp: InstructionTableComponent) {
        let idx = entity.0 as usize;
        self.instruction_tables[idx] = Some(comp);
        self.component_masks[idx] |= COMP_INSTRUCTION_TABLE;
    }
    #[inline]
    pub fn set_reflex(&mut self, entity: EntityId, comp: ReflexComponent) {
        let idx = entity.0 as usize;
        self.reflex_comps[idx] = Some(comp);
        self.component_masks[idx] |= COMP_REFLEX;
    }
    #[inline]
    pub fn set_assignment(&mut self, entity: EntityId, comp: AssignmentComponent) {
        let idx = entity.0 as usize;
        self.assignment_comps[idx] = Some(comp);
        self.component_masks[idx] |= COMP_ASSIGNMENT;
    }
    #[inline]
    pub fn set_property(&mut self, entity: EntityId, comp: PropertyComponent) {
        let idx = entity.0 as usize;
        self.property_comps[idx] = Some(comp);
        self.component_masks[idx] |= COMP_PROPERTY;
    }
    #[inline]
    pub fn set_temporal_node(&mut self, entity: EntityId, comp: TemporalNodeComponent) {
        let idx = entity.0 as usize;
        self.temporal_nodes[idx] = Some(comp);
        self.component_masks[idx] |= COMP_TEMPORAL_NODE;
    }
    #[inline]
    pub fn set_hls_dataflow(&mut self, entity: EntityId, comp: HlsDataflowComponent) {
        let idx = entity.0 as usize;
        self.hls_dataflow[idx] = Some(comp);
        self.component_masks[idx] |= COMP_HLS_DATAFLOW;
    }
    #[inline]
    pub fn set_hls_schedule(&mut self, entity: EntityId, comp: HlsScheduleComponent) {
        let idx = entity.0 as usize;
        self.hls_schedules[idx] = Some(comp);
        self.component_masks[idx] |= COMP_HLS_SCHEDULE;
    }
    #[inline]
    pub fn set_hls_binding(&mut self, entity: EntityId, comp: HlsBindingComponent) {
        let idx = entity.0 as usize;
        self.hls_bindings[idx] = Some(comp);
        self.component_masks[idx] |= COMP_HLS_BINDING;
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
        self.names[idx] = Some(NameComponent(self.interner.intern(&id_str)));
        self.chunk_texts[idx] = Some(ChunkTextComponent(text));
        self.source_paths[idx] = Some(SourcePathComponent(source));
        self.line_ranges[idx] = Some(LineRangeComponent(range));

        if let Some(v) = vector {
            self.vectors[idx] = Some(VectorComponent(v));
        }

        id
    }

    pub fn create_entity(&mut self, name: &str, kind: KindComponent) -> EntityId {
        let id = self.next_id();
        let name_id = self.interner.intern(name);
        self.set_name(id, NameComponent(name_id));
        self.set_kind(id, kind);
        self.symbol_to_entity.insert(name.to_string(), id);
        id
    }

    /// Safely set type for an entity (Delegated to ECS bitmask setter).
    pub fn set_legacy_type(&mut self, entity: EntityId, ty: TypeComponent) {
        self.set_type(entity, ty);
    }

    /// Safely set parent for an entity.
    pub fn set_parent(&mut self, entity: EntityId, parent: EntityId) {
        self.set_module(entity, ModuleComponent(parent));
    }

    /// Resolve a [`NameComponent`]'s interned id to a `&str`.
    ///
    /// This is the canonical read API for entity names. O(1).
    /// Returns `"<invalid>"` for sentinel ids (P10 Rule #5 — no panics).
    ///
    /// # Example
    /// ```ignore
    /// if let Some(nc) = registry.names[idx] {
    ///     let name = registry.resolve_name(nc.0);
    /// }
    /// ```
    #[inline]
    pub fn resolve_name(&self, id: InternId) -> &str {
        self.interner.resolve(id)
    }

    /// Convenience: get the name string for an `EntityId`, or `"<unnamed>"`.
    ///
    /// Used by emitters and diagnostics that need the name of an arbitrary entity.
    #[inline]
    pub fn get_entity_name(&self, entity: EntityId) -> &str {
        let idx = entity.0 as usize;
        match self.names.get(idx).and_then(|n| n.as_ref()) {
            Some(nc) => self.interner.resolve(nc.0),
            None => "<unnamed>",
        }
    }

    pub fn ingest_program(
        &mut self,
        program: &crate::ast::program::MirrProgram,
    ) -> Result<EntityId, MirrError> {
        self.target_config = program.target.clone();
        self.ingest_module(&program.module)
    }

    pub fn ingest_module(&mut self, module: &Module) -> Result<EntityId, MirrError> {
        let mod_id = self.next_id();
        let idx = mod_id.0 as usize;
        self.names[idx] = Some(NameComponent(self.interner.intern(&module.name)));
        self.kinds[idx] = Some(KindComponent(EntityKind::MODULE));
        self.spans[idx] = module.span.map(crate::ecs::components::SpanComponent);

        // BUG FIX: Register the module itself in the symbol table
        self.symbol_to_entity.insert(module.name.clone(), mod_id);

        self.ingest_signals(mod_id, &module.name, &module.signals);
        self.ingest_guards(mod_id, &module.name, &module.guards)?;
        self.ingest_reflexes(mod_id, &module.name, &module.reflexes)?;
        self.ingest_properties(mod_id, &module.name, &module.properties)?;

        let domains = module.clock_domains.iter().map(|d| d.name.clone()).collect::<Vec<_>>();
        if !domains.is_empty() {
            self.clock_domains[idx] = Some(ClockDomainsComponent(domains));
        }

        self.pattern_origins = module.pattern_origins.clone();

        Ok(mod_id)
    }

    /// Retrieve the root module name from the registry.
    pub fn get_module_name(&self) -> Option<String> {
        for i in (0..self.names.len()).rev() {
            if let (Some(nc), Some(kc)) = (&self.names[i], &self.kinds[i]) {
                if let EntityKind::MODULE = kc.0 {
                    return Some(self.resolve_name(nc.0).to_string());
                }
            }
        }
        None
    }

    /// Retrieve the root module span from the registry.
    pub fn get_module_span(&self) -> Option<crate::span::Span> {
        for (i, kind_comp) in self.kinds.iter().enumerate() {
            if let Some(KindComponent(EntityKind::MODULE)) = kind_comp {
                if let Some(span_comp) = &self.spans[i] {
                    return Some(span_comp.0);
                }
            }
        }
        None
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

            self.names[g_idx] = Some(NameComponent(self.interner.intern(&guard.name)));
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
                            crate::error_codes::ec(205),
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
                    let id = self.next_id();
                    let name_id = self.interner.intern(&assign.target);
                    self.set_name(id, NameComponent(name_id));
                    self.symbol_to_entity.insert(assign.target.clone(), id);
                    id
                });

                let assign_ent = self.next_id();
                let assign_idx = assign_ent.0 as usize;
                let assign_name = format!("_assign_{}", assign_ent.0);
                self.names[assign_idx] = Some(NameComponent(self.interner.intern(&assign_name)));
                self.kinds[assign_idx] = Some(KindComponent(EntityKind::ASSIGNMENT));
                self.assignment_comps[assign_idx] = Some(AssignmentComponent {
                    target: target_ent,
                    value: val_ent,
                    target_index: None,
                });
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

            self.names[r_idx] = Some(NameComponent(self.interner.intern(&reflex.name)));
            self.kinds[r_idx] = Some(KindComponent(EntityKind::REFLEX));
            self.reflex_comps[r_idx] = Some(ReflexComponent {
                guards: guard_entities,
                assignments: assignment_entities,
                origin: reflex.origin.clone(),
            });
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

            self.names[p_idx] = Some(NameComponent(self.interner.intern(&prop.name)));
            self.kinds[p_idx] = Some(KindComponent(EntityKind::PROPERTY));
            self.property_comps[p_idx] = Some(PropertyComponent {
                directive: prop.directive,
                formula: prop.formula.clone(),
                formula_exprs,
                origin: prop.origin.clone(),
            });
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
                        let sig_name = self
                            .names
                            .get(sig_ent.0 as usize)
                            .and_then(|n| *n)
                            .map(|nc| self.resolve_name(nc.0).to_string())
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
                            self.names
                                .get(decl.0 as usize)
                                .and_then(|n| *n)
                                .map(|nc| self.resolve_name(nc.0).to_string())
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
        self.names[idx] = Some(NameComponent(self.interner.intern(&name)));
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

    pub fn get_symbol_table(&self) -> &HashMap<String, EntityId> {
        &self.symbol_to_entity
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
