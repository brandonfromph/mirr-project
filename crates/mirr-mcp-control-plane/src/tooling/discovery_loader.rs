use std::process::Command;
use serde_json::Value;
use super::canonical_discovery_method_metadata::{DiscoveryMethodMetadata, DiscoveryParameter};

pub fn load_metadata_from_binary(bin_name: &str) -> Vec<DiscoveryMethodMetadata> {
    let output = Command::new("cargo")
        .args(&["run", "--bin", bin_name, "--", "--help-json"])
        .output()
        .ok();

    let Some(output) = output else { return vec![]; };
    if !output.status.success() { return vec![]; }

    let json: Value = serde_json::from_slice(&output.stdout).ok().unwrap_or(Value::Null);
    if json.is_null() { return vec![]; }

    let mut results = Vec::new();
    flatten_command(&json, "", &mut results);
    results
}

fn flatten_command(json: &Value, prefix: &str, results: &mut Vec<DiscoveryMethodMetadata>) {
    let raw_name = json["name"].as_str().unwrap_or("unknown");
    let name = if prefix.is_empty() {
        raw_name.to_string()
    } else {
        format!("{}_{}", prefix, raw_name)
    };

    let description = json["about"].as_str().unwrap_or("").to_string();
    
    let mut params = Vec::new();
    if let Some(args) = json["args"].as_array() {
        for arg in args {
            let p_name = arg["id"].as_str().unwrap_or("").to_string();
            let p_required = arg["required"].as_bool().unwrap_or(false);
            
            params.push(DiscoveryParameter::new(
                Box::leak(p_name.into_boxed_str()),
                p_required,
                "string"
            ));
        }
    }
    
    results.push(DiscoveryMethodMetadata::new(
        Box::leak(name.into_boxed_str()),
        false,
        Box::leak(description.into_boxed_str()),
        Box::leak(params.into_boxed_slice())
    ));

    if let Some(subs) = json["subcommands"].as_array() {
        for sub in subs {
            flatten_command(sub, &raw_name.replace('-', "_"), results);
        }
    }
}
