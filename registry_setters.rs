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

// --- Component Setters ---
    #[inline]
    pub fn set_name(&mut self, entity: EntityId, comp: NameComponent) {
        let idx = entity.0 as usize;
        self.names[idx] = Some(comp);
        self.component_masks[idx] |= COMP_NAME;
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
    pub fn set_binary_op(&mut self, entity: EntityId, comp: BinaryComponent) {
        let idx = entity.0 as usize;
        self.binary_ops[idx] = Some(comp);
        self.component_masks[idx] |= COMP_BINARY_OP;
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
