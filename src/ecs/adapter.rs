use crate::ast::program::MirrProgram;
use crate::ecs::components::*;
use crate::ecs::Registry;
use crate::error::MirrError;

/// Hydrates a parsed AST into a validated ECS Registry.
pub fn ingest_program(
    registry: &mut Registry,
    program: MirrProgram,
    base_dir: Option<&std::path::Path>,
) -> Result<(), MirrError> {
    // 1. Ingest patterns (pre-bake them into the registry)
    for pat in program.patterns {
        let entity = registry.create_entity(&pat.name, KindComponent::PATTERN);
        registry.set_type(entity, TypeComponent::pattern(pat.clone()));
        registry.pattern_defs[entity.0 as usize] = Some(PatternDefComponent(pat));
    }

    // 1.5. Ingest imports
    if let Some(dir) = base_dir {
        let mut loaded = std::collections::HashSet::new();
        for import in program.imports {
            ingest_import_recursive(registry, import, dir, &mut loaded)?;
        }
    }

    // 2. Ingest module structure
    registry.target_config = program.target.clone();
    registry.ingest_module(&program.module)?;

    // 3. Final structural validation (The Gate)
    registry.validate().map_err(|e| MirrError::SemanticError {
        message: format!("{} Registry hydration failure: {}", crate::error_codes::ec(901), e),
        span: None,
    })
}

fn ingest_import_recursive(
    registry: &mut Registry,
    import: crate::ast::program::ImportDecl,
    current_dir: &std::path::Path,
    loaded_paths: &mut std::collections::HashSet<std::path::PathBuf>,
) -> Result<(), MirrError> {
    let import_path = current_dir.join(&import.path);
    let canonical_path = import_path.canonicalize().unwrap_or_else(|_| import_path.clone());
    if loaded_paths.contains(&canonical_path) {
        return Ok(());
    }
    loaded_paths.insert(canonical_path.clone());

    let source = std::fs::read_to_string(&import_path).map_err(|e| MirrError::SemanticError {
        message: format!(
            "{} Failed to read imported file '{}': {}",
            crate::error_codes::ec(200),
            import.path,
            e
        ),
        span: import.span,
    })?;
    let imported_prog = crate::parser::module_parser::parse_mirr(&source)?;

    for pat in imported_prog.patterns {
        let entity = registry.create_entity(&pat.name, KindComponent::PATTERN);
        registry.set_type(entity, TypeComponent::pattern(pat.clone()));
        registry.pattern_defs[entity.0 as usize] = Some(PatternDefComponent(pat.clone()));

        // Register the qualified name (Alias::Name) in the symbol table
        let qualified_name = format!("{}::{}", import.alias, pat.name);
        registry.symbol_to_entity.insert(qualified_name, entity);
    }

    registry.ingest_module(&imported_prog.module)?;

    if let Some(parent_dir) = import_path.parent() {
        for nested_import in imported_prog.imports {
            ingest_import_recursive(registry, nested_import, parent_dir, loaded_paths)?;
        }
    }

    Ok(())
}

/// Direct registration of a parsed signal into the ECS Registry.
pub fn register_signal_to_ecs(
    registry: &mut Registry,
    module_entity: EntityId,
    sig: crate::ast::program::SignalDecl,
) -> EntityId {
    let entity = registry.create_entity(&sig.name, KindComponent(EntityKind::SIGNAL(sig.kind)));
    registry.set_type(entity, TypeComponent::signal(sig.ty));
    if let Some(span) = sig.span {
        registry.spans[entity.0 as usize] = Some(SpanComponent(span));
    }
    registry.set_parent(entity, module_entity);
    entity
}
