use std::collections::{BTreeMap, BTreeSet};

use mirror::tooling::{discovery_method_by_name, DiscoveryParameter, MrtDispatchTool};
use serde_json::Value;

fn expected_json_type(parameter_type: &str) -> &str {
    match parameter_type {
        "string" => "string",
        "number" => "number",
        "array" => "array",
        "boolean" => "boolean",
        _ => "string",
    }
}

fn expected_required_set(parameters: &[DiscoveryParameter]) -> BTreeSet<&'static str> {
    parameters
        .iter()
        .filter(|parameter| parameter.required)
        .map(|parameter| parameter.name)
        .collect()
}

#[test]
fn manifest_snapshot_input_schema_matches_canonical_parameter_types() {
    let manifest_text = include_str!("fixtures/mcp_manifest_snapshot.json");
    let manifest: Value = serde_json::from_str(manifest_text).expect("valid manifest fixture JSON");

    let tools =
        manifest.get("tools").and_then(Value::as_array).expect("manifest.tools must be an array");

    let tools_by_name: BTreeMap<&str, &Value> = tools
        .iter()
        .filter_map(|entry| {
            let name = entry.get("name").and_then(Value::as_str)?;
            Some((name, entry))
        })
        .collect();

    for tool in MrtDispatchTool::ALL {
        let method = discovery_method_by_name(tool.as_str())
            .expect("every dispatch tool must have discovery metadata");

        let entry = tools_by_name
            .get(method.name)
            .copied()
            .expect("manifest fixture must include every canonical tool");

        let input_schema = entry
            .get("inputSchema")
            .and_then(Value::as_object)
            .expect("tool.inputSchema must be an object");

        let properties = input_schema
            .get("properties")
            .and_then(Value::as_object)
            .expect("tool.inputSchema.properties must be an object");

        assert_eq!(
            properties.len(),
            method.parameters.len(),
            "tool '{}' property count drift",
            method.name
        );

        for parameter in method.parameters {
            let property = properties
                .get(parameter.name)
                .and_then(Value::as_object)
                .expect("tool inputSchema property must exist and be an object");

            let actual_type = property
                .get("type")
                .and_then(Value::as_str)
                .expect("tool inputSchema property.type must be a string");

            assert_eq!(
                actual_type,
                expected_json_type(parameter.ty),
                "tool '{}' parameter '{}' type drift",
                method.name,
                parameter.name
            );

            if parameter.ty == "array" {
                assert!(
                    property.contains_key("items"),
                    "tool '{}' parameter '{}' array schema missing items",
                    method.name,
                    parameter.name
                );
            }
        }

        let expected_required = expected_required_set(method.parameters);
        let actual_required: BTreeSet<&str> = input_schema
            .get("required")
            .and_then(Value::as_array)
            .map(|required| required.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();

        assert_eq!(
            actual_required, expected_required,
            "tool '{}' required list drift",
            method.name
        );
    }
}
