#![forbid(unsafe_code)]

use std::collections::HashMap;
use crate::ecs::components::*;

/// Max capacity for compiler entities (NASA P10 Rule #2: Fixed bounds)
pub const MAX_ENTITIES: usize = 1_000_000;

/// The Registry: The Data-Oriented "World" of the MIRR Compiler.
/// Uses Structure of Arrays (SoA) for cache-friendly system passes.
#[derive(Debug, Default)]
pub struct Registry {
    /// Incremental ID counter
    pub(super) next_id: u32,

    // --- Component Arrays (SoA) ---
    pub names: HashMap<EntityId, NameComponent>,
    pub kinds: HashMap<EntityId, KindComponent>,
    pub types: HashMap<EntityId, TypeComponent>,
    pub modules: HashMap<EntityId, ModuleComponent>,

    // Reverse lookup for speed (Symbol Table replacement)
    pub(super) symbol_to_entity: HashMap<String, EntityId>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            names: HashMap::with_capacity(MAX_ENTITIES / 10),
            kinds: HashMap::with_capacity(MAX_ENTITIES / 10),
            types: HashMap::with_capacity(MAX_ENTITIES / 10),
            modules: HashMap::with_capacity(MAX_ENTITIES / 100),
            symbol_to_entity: HashMap::with_capacity(MAX_ENTITIES / 10),
        }
    }

    /// Create a new Signal Entity
    pub fn create_signal(&mut self, name: String, kind: KindComponent, ty: TypeComponent) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;

        self.symbol_to_entity.insert(name.clone(), id);
        self.names.insert(id, NameComponent(name));
        self.kinds.insert(id, kind);
        self.types.insert(id, ty);

        id
    }

    pub fn get_entity_by_name(&self, name: &str) -> Option<EntityId> {
        self.symbol_to_entity.get(name).copied()
    }
}
