//! Contract: ECS-Native Synthesis Integration
//! This test enforces that signals defined in ergonomic 'signals { ... }' blocks
//! are correctly ingested into the ECS Registry as first-class entities.

use mirrc::ecs::{components::*, Registry};
use mirrc::parser::parse_mirr;

#[test]
fn test_signals_block_to_ecs_entities() {
    let input = r#"
module test_mod {
    signals {
        lidar_range: in u32;
    }
}"#;

    // 2. Parse into AST
    let _program = parse_mirr(input).expect("Parser should accept expanded signals");

    // 3. Ingest into ECS Registry
    let mut registry = Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut registry, input, None)
        .expect("ECS ingestion failed");

    // 4. Verify entity exists (use public method get_entity_by_name)
    let lidar_entity =
        registry.get_entity_by_name("lidar_range").expect("Signal should be registered in ECS");

    let kind =
        registry.kinds[lidar_entity.0 as usize].expect("Entity should have a Kind component");

    assert_eq!(kind, KindComponent(EntityKind::SIGNAL(mirrc::ast::types::SignalKind::Input)));
}
