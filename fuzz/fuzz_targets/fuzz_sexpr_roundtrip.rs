#![no_main]
use libfuzzer_sys::fuzz_target;
use mirrc::{
    ecs::{components::EntityKind, EntityId, Registry},
    run_pipeline, PipelineConfig,
    sexpr::{parser::parse_sexpr, printer::print_sexpr, registry_to_sexpr, sexpr_to_registry},
};

/// Find the first MODULE entity in the registry.
fn find_module_entity(registry: &Registry) -> Option<EntityId> {
    for i in 0..registry.active_entities() {
        if let Some(kind) = registry.kinds[i].as_ref() {
            if std::mem::discriminant(&kind.0)
                == std::mem::discriminant(&EntityKind::MODULE)
            {
                return Some(EntityId(i as u32));
            }
        }
    }
    None
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let config = PipelineConfig::default();
        if let Ok(result) = run_pipeline(s, &config) {
            if let Some(registry) = result.ecs_registry {
                if let Some(mod_id) = find_module_entity(&registry) {
                    let sexpr = registry_to_sexpr(&registry, mod_id);
                    let printed = print_sexpr(&sexpr);
                    if let Ok(reparsed) = parse_sexpr(&printed) {
                        let mut new_registry = Registry::new();
                        let _ = sexpr_to_registry(&mut new_registry, &reparsed);
                    }
                }
            }
        }
    }
});
