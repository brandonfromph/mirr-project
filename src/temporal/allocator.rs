//! SPU Resource Allocator: Maps logical ECS signals to physical R-SPU registers.

#![forbid(unsafe_code)]

use crate::ecs::{EntityId, Registry};
use crate::error::MirrError;
use crate::error_codes::{mirrcode, ErrorCode};
use std::collections::HashMap;

#[derive(Default)]
pub struct RspuAllocator {
    // Current mapping from EntityID to Physical Register ID
    allocation_map: HashMap<EntityId, u32>,
    next_register: u32,
}

impl RspuAllocator {
    pub fn new() -> Self {
        Self { allocation_map: HashMap::new(), next_register: 0 }
    }

    pub fn allocate(&mut self, _registry: &Registry, entity: EntityId) -> Result<u32, MirrError> {
        if self.next_register >= 256 {
            return Err(mirrcode(
                ErrorCode::RspuRegisterAlloc,
                "physical register limit exceeded (256)",
            ));
        }

        let reg = self.next_register;
        self.allocation_map.insert(entity, reg);
        self.next_register += 1;
        Ok(reg)
    }
}
