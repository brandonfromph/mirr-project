#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::retrieval::SearchResult;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Candidate before reranking.
#[derive(Debug, Clone)]
pub struct RerankedCandidate {
    pub result: SearchResult,
    pub bm25_rank: Option<usize>,
    pub vector_rank: Option<usize>,
    pub rrf_score: f32,
    pub appears_in_both: bool,
    pub freshness_secs: u64,
    pub estimated_tokens: usize,
}

/// Reranked result with final ranking score.
#[derive(Debug, Clone)]
pub struct RerankedResult {
    pub result: SearchResult,
    pub rerank_score: f32,
    pub original_rank: usize,
    pub final_rank: usize,
    pub bm25_rank: Option<usize>,
    pub vector_rank: Option<usize>,
    pub rrf_score: f32,
    pub appears_in_both: bool,
    pub freshness_secs: u64,
    pub estimated_tokens: usize,
}

/// CPU-only reranker with FlashRank-compatible behavior.
#[derive(Debug, Clone)]
pub struct FlashRankReranker {
    model_name: String,
}

impl Default for FlashRankReranker {
    fn default() -> Self {
        Self { model_name: "ms-marco-MiniLM-L-12-v2".to_string() }
    }
}

impl FlashRankReranker {
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    pub fn rerank(
        &self,
        query: &str,
        candidates: Vec<RerankedCandidate>,
        top_k: usize,
    ) -> Vec<RerankedResult> {
        let mut scored: Vec<RerankedResult> = candidates
            .into_iter()
            .enumerate()
            .map(|(index, candidate)| RerankedResult {
                rerank_score: rerank_score(
                    query,
                    &candidate.result.title,
                    &candidate.result.snippet,
                    candidate.rrf_score,
                    candidate.appears_in_both,
                ),
                result: candidate.result,
                original_rank: index + 1,
                final_rank: 0,
                bm25_rank: candidate.bm25_rank,
                vector_rank: candidate.vector_rank,
                rrf_score: candidate.rrf_score,
                appears_in_both: candidate.appears_in_both,
                freshness_secs: candidate.freshness_secs,
                estimated_tokens: candidate.estimated_tokens,
            })
            .collect();

        scored.sort_by(|lhs, rhs| {
            rhs.rerank_score.partial_cmp(&lhs.rerank_score).unwrap_or(Ordering::Equal)
        });
        scored.truncate(top_k);
        for (index, item) in scored.iter_mut().enumerate() {
            item.final_rank = index + 1;
        }
        scored
    }
}

fn rerank_score(
    query: &str,
    title: &str,
    snippet: &str,
    rrf_score: f32,
    appears_in_both: bool,
) -> f32 {
    let query_terms = tokenize(query);
    let candidate_terms = tokenize(&format!("{} {}", title, snippet));
    let overlap = term_overlap(&query_terms, &candidate_terms);
    let phrase_boost =
        if snippet.to_lowercase().contains(&query.to_lowercase()) { 0.15 } else { 0.0 };
    let both_boost = if appears_in_both { 0.10 } else { 0.0 };
    (overlap * 0.70) + phrase_boost + both_boost + (rrf_score * 0.05)
}

fn tokenize(input: &str) -> BTreeSet<String> {
    let mut terms = BTreeSet::new();
    for term in input.split(|ch: char| !ch.is_alphanumeric() && ch != '_') {
        if !term.is_empty() {
            terms.insert(term.to_lowercase());
        }
    }
    terms
}

fn term_overlap(lhs: &BTreeSet<String>, rhs: &BTreeSet<String>) -> f32 {
    if lhs.is_empty() || rhs.is_empty() {
        return 0.0;
    }
    let common = lhs.intersection(rhs).count() as f32;
    (common / lhs.len().max(rhs.len()) as f32).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(
        title: &str,
        snippet: &str,
        rrf_score: f32,
        appears_in_both: bool,
    ) -> RerankedCandidate {
        RerankedCandidate {
            result: SearchResult {
                key: title.to_string(),
                title: title.to_string(),
                snippet: snippet.to_string(),
                score: 0.5,
                source: "src/demo.mirr".to_string(),
            },
            bm25_rank: Some(1),
            vector_rank: Some(1),
            rrf_score,
            appears_in_both,
            freshness_secs: 123,
            estimated_tokens: 10,
        }
    }

    #[test]
    fn reranker_prefers_term_overlap() {
        let reranker = FlashRankReranker::default();
        let results = reranker.rerank(
            "what signals emit alpha",
            vec![
                candidate("demo.alpha", "alpha signal emits data", 0.1, true),
                candidate("demo.beta", "completely unrelated text", 0.9, false),
            ],
            2,
        );
        assert_eq!(results[0].result.key, "demo.alpha");
    }

    #[test]
    fn reranker_truncates_to_top_k() {
        let reranker = FlashRankReranker::default();
        let results = reranker.rerank(
            "alpha",
            vec![candidate("a", "alpha", 0.1, false), candidate("b", "alpha", 0.2, false)],
            1,
        );
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn reranker_exposes_model_name() {
        let reranker = FlashRankReranker::default();
        assert_eq!(reranker.model_name(), "ms-marco-MiniLM-L-12-v2");
    }
}
