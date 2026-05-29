#![forbid(unsafe_code)]
#![deny(warnings)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Difficulty tier for a golden MIRR query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

/// A golden query/answer pair used for offline RAG evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenQAPair {
    pub query: String,
    pub expected_chunks: Vec<String>,
    pub expected_answer: String,
    pub difficulty: Difficulty,
}

/// Metrics for a single evaluated query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub query: String,
    pub retrieved_chunks: Vec<String>,
    pub context_precision: f32,
    pub context_recall: f32,
    pub faithfulness: f32,
    pub top_k_position: usize,
}

/// Quality gate thresholds used for promotion decisions.
pub const MIN_CONTEXT_PRECISION: f32 = 0.7;
pub const MIN_FAITHFULNESS: f32 = 0.8;

/// Build the default MIRR-specific golden QA set.
pub fn default_golden_qa_set() -> Vec<GoldenQAPair> {
    vec![
        GoldenQAPair {
            query: "what signals does module alpha emit".to_string(),
            expected_chunks: vec!["module.alpha.signal.emit".to_string()],
            expected_answer: "module alpha emits signal emit".to_string(),
            difficulty: Difficulty::Easy,
        },
        GoldenQAPair {
            query: "which guards reference width Y".to_string(),
            expected_chunks: vec![
                "module.beta.guard.width_y".to_string(),
                "module.beta.reflex.r1".to_string(),
            ],
            expected_answer: "guards and reflexes referencing width Y".to_string(),
            difficulty: Difficulty::Medium,
        },
        GoldenQAPair {
            query: "show properties that depend on temporal guard g1".to_string(),
            expected_chunks: vec![
                "module.gamma.property.p1".to_string(),
                "module.gamma.guard.g1".to_string(),
            ],
            expected_answer: "property p1 depends on guard g1".to_string(),
            difficulty: Difficulty::Hard,
        },
    ]
}

/// Evaluate retrieval outputs against a golden pair.
pub fn evaluate_pair(
    pair: &GoldenQAPair,
    retrieved_chunks: &[String],
    generated_answer: &str,
) -> EvaluationResult {
    let expected: BTreeSet<&String> = pair.expected_chunks.iter().collect();
    let retrieved: BTreeSet<&String> = retrieved_chunks.iter().collect();

    let relevant_retrieved = retrieved.intersection(&expected).count() as f32;
    let context_precision =
        if retrieved.is_empty() { 0.0 } else { relevant_retrieved / retrieved.len() as f32 };
    let context_recall =
        if expected.is_empty() { 0.0 } else { relevant_retrieved / expected.len() as f32 };

    let answer_overlap = token_overlap(&pair.expected_answer, generated_answer);
    let faithfulness =
        ((answer_overlap * 0.7) + (context_precision * 0.2) + (context_recall * 0.1))
            .clamp(0.0, 1.0);

    let top_k_position = if let Some(first_expected) = pair.expected_chunks.first() {
        retrieved_chunks
            .iter()
            .position(|chunk| chunk == first_expected)
            .map(|index| index + 1)
            .unwrap_or(0)
    } else {
        0
    };

    EvaluationResult {
        query: pair.query.clone(),
        retrieved_chunks: retrieved_chunks.to_vec(),
        context_precision,
        context_recall,
        faithfulness,
        top_k_position,
    }
}

/// Check whether a single evaluation result passes the promotion gate.
pub fn passes_quality_gate(result: &EvaluationResult) -> bool {
    result.context_precision >= MIN_CONTEXT_PRECISION && result.faithfulness >= MIN_FAITHFULNESS
}

/// Check whether all evaluation results pass the promotion gate.
pub fn passes_quality_gate_for_set(results: &[EvaluationResult]) -> bool {
    !results.is_empty() && results.iter().all(passes_quality_gate)
}

fn token_overlap(expected: &str, actual: &str) -> f32 {
    let expected_tokens = tokenize(expected);
    let actual_tokens = tokenize(actual);
    if expected_tokens.is_empty() || actual_tokens.is_empty() {
        return 0.0;
    }
    let common = expected_tokens.intersection(&actual_tokens).count() as f32;
    common / expected_tokens.len().max(actual_tokens.len()) as f32
}

fn tokenize(text: &str) -> BTreeSet<String> {
    let mut tokens = BTreeSet::new();
    for token in text.split(|ch: char| !ch.is_alphanumeric() && ch != '_') {
        if !token.is_empty() {
            tokens.insert(token.to_lowercase());
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_set_has_three_queries() {
        let set = default_golden_qa_set();
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn evaluation_scores_precise_retrieval_higher() {
        let pair = &default_golden_qa_set()[0];
        let result = evaluate_pair(
            pair,
            &["module.alpha.signal.emit".to_string(), "unrelated.chunk".to_string()],
            "module alpha emits signal emit",
        );
        assert!(result.context_precision > 0.4);
        assert!(result.context_recall > 0.9);
        assert!(result.faithfulness > 0.6);
    }

    #[test]
    fn evaluation_scores_low_for_mismatch() {
        let pair = &default_golden_qa_set()[1];
        let result = evaluate_pair(pair, &["other.chunk".to_string()], "irrelevant answer");
        assert_eq!(result.context_precision, 0.0);
        assert_eq!(result.context_recall, 0.0);
        assert!(result.faithfulness <= 0.2);
    }

    #[test]
    fn quality_gate_passes_only_for_thresholds() {
        let passing = EvaluationResult {
            query: "q".to_string(),
            retrieved_chunks: vec!["a".to_string()],
            context_precision: 0.75,
            context_recall: 0.80,
            faithfulness: 0.85,
            top_k_position: 1,
        };
        let failing = EvaluationResult {
            query: "q".to_string(),
            retrieved_chunks: vec!["a".to_string()],
            context_precision: 0.65,
            context_recall: 0.80,
            faithfulness: 0.85,
            top_k_position: 1,
        };

        assert!(passes_quality_gate(&passing));
        assert!(!passes_quality_gate(&failing));
        assert!(passes_quality_gate_for_set(&[passing]));
        assert!(!passes_quality_gate_for_set(&[]));
    }
}
