// ... imports
use crate::ecs::Registry;
use crate::ecs::components::*;
use crate::ast::program::{MirrProgram, Module};
use crate::error::MirrError;

/// Hydrates a parsed AST into a validated ECS Registry.
pub fn ingest_program(registry: &mut Registry, program: MirrProgram) -> Result<(), MirrError> {
    // 1. Ingest patterns (pre-bake them into the registry)
    for pat in program.patterns {
        let entity = registry.create_entity(&pat.name, KindComponent::PATTERN);
        registry.set_type(entity, TypeComponent::pattern(pat));
    }

    // 2. Ingest module structure
    ingest_module(registry, program.module)?;

    // 3. Final structural validation (The Gate)
    registry.validate().map_err(|e| MirrError::SemanticError {
        message: format!("[E901] Registry hydration failure: {}", e),
        span: None,
    })
}

fn ingest_module(registry: &mut Registry, module: Module) -> Result<(), MirrError> {
    let module_entity = registry.create_entity(&module.name, KindComponent::MODULE);
    
    // Ingest signals
    for sig in module.signals {
        let entity = registry.create_entity(&sig.name, KindComponent::SIGNAL);
        registry.set_type(entity, TypeComponent::signal(sig.ty));
        registry.set_parent(entity, module_entity);
    }
    
    Ok(())
}
