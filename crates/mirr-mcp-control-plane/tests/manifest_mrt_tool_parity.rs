use std::collections::BTreeSet;

use mirror::tooling::MrtDispatchTool;
use serde_json::Value;

#[test]
fn manifest_snapshot_contains_all_mrt_dispatch_tools() {
    let manifest_text = include_str!("fixtures/mcp_manifest_snapshot.json");
    let manifest: Value = serde_json::from_str(manifest_text).expect("valid manifest fixture JSON");

    let tools =
        manifest.get("tools").and_then(Value::as_array).expect("manifest.tools must be an array");

    let manifest_names: BTreeSet<String> = tools
        .iter()
        .filter_map(|entry| entry.get("name"))
        .filter_map(Value::as_str)
        .map(ToOwned::to_owned)
        .collect();

    for tool in MrtDispatchTool::ALL {
        assert!(
            manifest_names.contains(tool.as_str()),
            "missing tool '{}' from manifest fixture",
            tool.as_str()
        );
    }
}

#[test]
fn manifest_snapshot_catalog_identity_matches_rust_constants() {
    let manifest_text = include_str!("fixtures/mcp_manifest_snapshot.json");
    let manifest: Value = serde_json::from_str(manifest_text).expect("valid manifest fixture JSON");

    let catalog = manifest
        .get("catalog")
        .and_then(Value::as_object)
        .expect("manifest.catalog must be an object");

    let id = catalog.get("id").and_then(Value::as_str).expect("catalog.id must be a string");
    let display = catalog
        .get("display_name")
        .and_then(Value::as_str)
        .expect("catalog.display_name must be a string");

    assert_eq!(id, mirror::catalog::CANONICAL_CATALOG_ID);
    assert_eq!(display, mirror::catalog::CANONICAL_DISPLAY_NAME);
}
