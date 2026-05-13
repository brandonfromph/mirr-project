#![forbid(unsafe_code)]

use crate::ast::program::Module;
use crate::ast::Expr;
use crate::ecs::components::*;
use std::collections::HashMap;

/// Max capacity for compiler entities (NASA P10 Rule #2: Fixed bounds)
pub const MAX_ENTITIES: usize = 1_000_000;

/// The Registry: The Data-Oriented "World" of the MIRR Compiler.
/// Refactored to Vec-based storage for O(1) access and cache locality.
#[derive(Debug)]
pub struct Registry {
    pub(super) next_id: u32,

    // --- Component Arrays (Dense SoA) ---
    pub names: Vec<Option<NameComponent>>,
    pub kinds: Vec<Option<KindComponent>>,
    pub types: Vec<Option<TypeComponent>>,
    pub modules: Vec<Option<ModuleComponent>>,
    pub cycles: Vec<Option<CyclesComponent>>,
    pub conditions: Vec<Option<ConditionComponent>>,

    // Expression Components
    pub literals: Vec<Option<LiteralComponent>>,
    pub unary_ops: Vec<Option<UnaryComponent>>,
    pub binary_ops: Vec<Option<BinaryComponent>>,
    pub prev_ops: Vec<Option<PrevComponent>>,
    pub signal_refs: Vec<Option<SignalRefComponent>>,

    // Knowledge Base Component Tables (Phase 2)
    pub vectors: Vec<Option<VectorComponent>>,
    pub chunk_texts: Vec<Option<ChunkTextComponent>>,
    pub source_paths: Vec<Option<SourcePathComponent>>,
    pub line_ranges: Vec<Option<LineRangeComponent>>,

    pub(super) symbol_to_entity: HashMap<String, EntityId>,
}

#[path = "registry_validate.rs"]
mod registry_validate;

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        let cap = MAX_ENTITIES / 10; // Start with 100k capacity
        Self {
            next_id: 0,
            names: vec![None; cap],
            kinds: vec![None; cap],
            types: vec![None; cap],
            modules: vec![None; cap],
            cycles: vec![None; cap],
            conditions: vec![None; cap],
            literals: vec![None; cap],
            unary_ops: vec![None; cap],
            binary_ops: vec![None; cap],
            prev_ops: vec![None; cap],
            signal_refs: vec![None; cap],
            vectors: vec![None; cap],
            chunk_texts: vec![None; cap],
            source_paths: vec![None; cap],
            line_ranges: vec![None; cap],
            symbol_to_entity: HashMap::with_capacity(cap),
        }
    }

    fn next_id(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;

        let idx = id.0 as usize;
        if idx >= self.names.len() && idx < MAX_ENTITIES {
            let new_cap = (idx + 1024).min(MAX_ENTITIES);
            self.names.resize(new_cap, None);
            self.kinds.resize(new_cap, None);
            self.types.resize(new_cap, None);
            self.modules.resize(new_cap, None);
            self.cycles.resize(new_cap, None);
            self.conditions.resize(new_cap, None);
            self.literals.resize(new_cap, None);
            self.unary_ops.resize(new_cap, None);
            self.binary_ops.resize(new_cap, None);
            self.prev_ops.resize(new_cap, None);
            self.signal_refs.resize(new_cap, None);
            self.vectors.resize(new_cap, None);
            self.chunk_texts.resize(new_cap, None);
            self.source_paths.resize(new_cap, None);
            self.line_ranges.resize(new_cap, None);
        }

        id
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

    /// Ingest a traditional Tree-based Module into the ECS World.
    pub fn ingest_module(&mut self, module: &Module) -> EntityId {
        let mod_id = self.next_id();
        let idx = mod_id.0 as usize;
        self.names[idx] = Some(NameComponent(module.name.clone()));

        // 1. Ingest Signals
        for sig in &module.signals {
            let entity = self.create_signal(
                sig.name.clone(),
                KindComponent(sig.kind),
                TypeComponent(sig.ty.clone()),
            );
            self.modules[entity.0 as usize] = Some(ModuleComponent(mod_id));
        }

        // 2. Ingest Guards
        for guard in &module.guards {
            let cond_entity = self.ingest_expr(&guard.condition);
            let guard_id = self.next_id();
            let g_idx = guard_id.0 as usize;
            self.symbol_to_entity.insert(guard.name.clone(), guard_id);
            self.names[g_idx] = Some(NameComponent(guard.name.clone()));
            self.conditions[g_idx] = Some(ConditionComponent(cond_entity));
            self.cycles[g_idx] = Some(CyclesComponent(guard.cycles));
            self.modules[g_idx] = Some(ModuleComponent(mod_id));
        }

        mod_id
    }

    /// Recursively flatten an expression tree into ECS entities.
    pub fn ingest_expr(&mut self, expr: &Expr) -> EntityId {
        match expr {
            Expr::Literal(lit) => {
                let id = self.next_id();
                self.literals[id.0 as usize] = Some(LiteralComponent(lit.clone()));
                id
            }
            Expr::Signal(name) => {
                let id = self.next_id();
                if let Some(sig_ent) = self.get_entity_by_name(name) {
                    self.signal_refs[id.0 as usize] = Some(SignalRefComponent(sig_ent));
                }
                id
            }
            Expr::Unary { op, operand } => {
                let operand_id = self.ingest_expr(operand);
                let id = self.next_id();
                self.unary_ops[id.0 as usize] =
                    Some(UnaryComponent { op: *op, operand: operand_id });
                id
            }
            Expr::Binary { op, left, right } => {
                let left_id = self.ingest_expr(left);
                let right_id = self.ingest_expr(right);
                let id = self.next_id();
                self.binary_ops[id.0 as usize] =
                    Some(BinaryComponent { op: *op, left: left_id, right: right_id });
                id
            }
            Expr::Prev { signal, delay } => {
                let id = self.next_id();
                if let Some(sig_ent) = self.get_entity_by_name(signal) {
                    self.prev_ops[id.0 as usize] =
                        Some(PrevComponent { signal: sig_ent, delay: *delay });
                }
                id
            }
            _ => self.next_id(),
        }
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
        self.symbol_to_entity.get(name).copied()
    }

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
