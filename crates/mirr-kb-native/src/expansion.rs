#![forbid(unsafe_code)]
#![deny(warnings)]

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

const MAX_QUERY_VARIANTS: usize = 3;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpansionMode {
    None,
    Synonym,
    Hyde,
}

pub fn expand_query_variants(query: &str, mode: ExpansionMode) -> Vec<(String, f32)> {
    let base = query.trim();
    if base.is_empty() {
        return Vec::new();
    }

    let mut variants = vec![(base.to_string(), 1.0_f32)];
    match mode {
        ExpansionMode::None => {}
        ExpansionMode::Synonym => {
            if let Some(synonym_variant) = apply_synonym_expansion(base) {
                variants.push((synonym_variant, 0.8));
            }
            if let Some(structural_variant) = apply_structural_expansion(base) {
                variants.push((structural_variant, 0.6));
            }
        }
        ExpansionMode::Hyde => {
            variants.push((format!("{} with implementation context", base), 0.8));
            variants
                .push((format!("{} including guard reflex and signal dependencies", base), 0.6));
        }
    }

    variants.truncate(MAX_QUERY_VARIANTS);
    variants
}

fn apply_synonym_expansion(query: &str) -> Option<String> {
    let mut rewritten = query.to_string();
    let mappings = [
        ("signal", "wire"),
        ("signals", "wires"),
        ("guard", "condition"),
        ("reflex", "action"),
        ("module", "component"),
        ("emit", "output"),
    ];

    let mut changed = false;
    for (from, to) in mappings {
        let candidate = rewritten.replace(from, to);
        if candidate != rewritten {
            rewritten = candidate;
            changed = true;
        }
    }

    if changed {
        Some(rewritten)
    } else {
        None
    }
}

fn apply_structural_expansion(query: &str) -> Option<String> {
    if query.contains("depends") || query.contains("dependency") {
        return Some(format!("{} and transitive module references", query));
    }
    if query.contains("temporal") || query.contains("delay") {
        return Some(format!("{} with time-window constraints", query));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synonym_expansion_is_deterministic() {
        let first =
            expand_query_variants("what guard controls signal alpha", ExpansionMode::Synonym);
        let second =
            expand_query_variants("what guard controls signal alpha", ExpansionMode::Synonym);
        assert_eq!(first, second);
    }

    #[test]
    fn none_expansion_keeps_single_variant() {
        let variants = expand_query_variants("alpha", ExpansionMode::None);
        assert_eq!(variants.len(), 1);
        assert_eq!(variants[0].0, "alpha");
        assert_eq!(variants[0].1, 1.0);
    }

    #[test]
    fn expansion_is_bounded_to_three_variants() {
        let variants = expand_query_variants("temporal dependency", ExpansionMode::Hyde);
        assert!(variants.len() <= 3);
    }
}
