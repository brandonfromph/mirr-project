#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use super::rpc_method_aliases::resolve_method_alias;

fn camel_to_snake(source: &str) -> String {
    let mut output = String::with_capacity(source.len() * 2);
    for ch in source.chars() {
        if ch.is_ascii_uppercase() {
            output.push('_');
            output.push(ch.to_ascii_lowercase());
        } else {
            output.push(ch.to_ascii_lowercase());
        }
    }
    output
}

fn normalize_space_dash_separators(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut in_separator = false;

    for ch in source.chars() {
        if ch.is_ascii_whitespace() || ch == '-' {
            if !in_separator {
                output.push('_');
            }
            in_separator = true;
            continue;
        }

        in_separator = false;
        output.push(ch.to_ascii_lowercase());
    }

    output
}

pub fn normalize_rpc_method_name(
    raw_method: Option<&str>,
    call_tool_name: Option<&str>,
    known_methods: &BTreeSet<String>,
) -> String {
    let Some(method_name) = raw_method else {
        return String::new();
    };

    if !method_name.is_empty() && known_methods.contains(method_name) {
        return method_name.to_owned();
    }

    if !method_name.is_empty() {
        let camel = camel_to_snake(method_name);
        if known_methods.contains(&camel) {
            return camel;
        }

        let spaced = normalize_space_dash_separators(method_name);
        if known_methods.contains(&spaced) {
            return spaced;
        }

        if let Some(alias) = resolve_method_alias(method_name, call_tool_name) {
            return alias;
        }
    }

    method_name.to_owned()
}
