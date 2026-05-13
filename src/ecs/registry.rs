#![forbid(unsafe_code)]

use std::collections::HashMap;
use crate::ecs::components::*;
use crate::ast::program::Module;
use crate::ast::Expr;

/// Max capacity for compiler entities (NASA P10 Rule #2: Fixed bounds)
pub const MAX_ENTITIES: usize = 1_000_000;

/// The Registry: The Data-Oriented "World" of the MIRR Compiler.
#[derive(Debug, Default)]
pub struct Registry {
    pub(super) next_id: u32,

    // --- Component Arrays (SoA) ---
    pub names: HashMap<EntityId, NameComponent>,
    pub kinds: HashMap<EntityId, KindComponent>,
    pub types: HashMap<EntityId, TypeComponent>,
    pub modules: HashMap<EntityId, ModuleComponent>,
    pub cycles: HashMap<EntityId, CyclesComponent>,
    pub conditions: HashMap<EntityId, ConditionComponent>,

    // Expression Components
    pub literals: HashMap<EntityId, LiteralComponent>,
    pub unary_ops: HashMap<EntityId, UnaryComponent>,
    pub binary_ops: HashMap<EntityId, BinaryComponent>,
    pub prev_ops: HashMap<EntityId, PrevComponent>,
    pub signal_refs: HashMap<EntityId, SignalRefComponent>,

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
            cycles: HashMap::with_capacity(MAX_ENTITIES / 100),
            conditions: HashMap::with_capacity(MAX_ENTITIES / 100),
            literals: HashMap::with_capacity(MAX_ENTITIES / 10),
            unary_ops: HashMap::with_capacity(MAX_ENTITIES / 10),
            binary_ops: HashMap::with_capacity(MAX_ENTITIES / 10),
            prev_ops: HashMap::with_capacity(MAX_ENTITIES / 10),
            signal_refs: HashMap::with_capacity(MAX_ENTITIES / 10),
            symbol_to_entity: HashMap::with_capacity(MAX_ENTITIES / 10),
        }
    }

    fn next_id(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Ingest a traditional Tree-based Module into the ECS World.
    pub fn ingest_module(&mut self, module: &Module) -> EntityId {
        let mod_id = self.next_id();
        self.names.insert(mod_id, NameComponent(module.name.clone()));
        
        // 1. Ingest Signals
        for sig in &module.signals {
            let entity = self.create_signal(
                sig.name.clone(),
                KindComponent(sig.kind),
                TypeComponent(sig.ty.clone())
            );
            self.modules.insert(entity, ModuleComponent(mod_id));
        }

        // 2. Ingest Guards
        for guard in &module.guards {
            let cond_entity = self.ingest_expr(&guard.condition);
            let guard_id = self.next_id();
            self.symbol_to_entity.insert(guard.name.clone(), guard_id);
            self.names.insert(guard_id, NameComponent(guard.name.clone()));
            self.conditions.insert(guard_id, ConditionComponent(cond_entity));
            self.cycles.insert(guard_id, CyclesComponent(guard.cycles));
            self.modules.insert(guard_id, ModuleComponent(mod_id));
        }

        mod_id
    }

    /// Recursively flatten an expression tree into ECS entities.
    pub fn ingest_expr(&mut self, expr: &Expr) -> EntityId {
        match expr {
            Expr::Literal(lit) => {
                let id = self.next_id();
                self.literals.insert(id, LiteralComponent(lit.clone()));
                id
            }
            Expr::Signal(name) => {
                let id = self.next_id();
                // Link to the signal entity if it exists
                if let Some(sig_ent) = self.get_entity_by_name(name) {
                    self.signal_refs.insert(id, SignalRefComponent(sig_ent));
                }
                id
            }
            Expr::Unary { op, operand } => {
                let operand_id = self.ingest_expr(operand);
                let id = self.next_id();
                self.unary_ops.insert(id, UnaryComponent { op: *op, operand: operand_id });
                id
            }
            Expr::Binary { op, left, right } => {
                let left_id = self.ingest_expr(left);
                let right_id = self.ingest_expr(right);
                let id = self.next_id();
                self.binary_ops.insert(id, BinaryComponent { op: *op, left: left_id, right: right_id });
                id
            }
            Expr::Prev { signal, delay } => {
                let id = self.next_id();
                if let Some(sig_ent) = self.get_entity_by_name(signal) {
                    self.prev_ops.insert(id, PrevComponent { signal: sig_ent, delay: *delay });
                }
                id
            }
            // Add other Expr variants as needed
            _ => self.next_id(),
        }
    }

    /// Create a new Signal Entity
    pub fn create_signal(&mut self, name: String, kind: KindComponent, ty: TypeComponent) -> EntityId {
        let id = self.next_id();
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
