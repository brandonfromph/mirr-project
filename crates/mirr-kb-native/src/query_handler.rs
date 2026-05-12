#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::adapters::embedding::EmbeddingProvider;
use crate::expansion::{expand_query_variants, ExpansionMode};
use crate::reranking::RerankedCandidate;
use crate::resilience::{run_with_resilience, ResiliencePolicy};
use crate::retrieval::hybrid_search::{HybridRetrievalConfig, HybridSearcher};
use crate::retrieval::{Freshness, QueryRequest, QueryResponse, SearchMode, SearchResult};
use crate::storage::{IndexStats, SqliteHybridStorage};
use crate::validation::validate_results;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::sync::Arc;

const MAX_LIMIT: usize = 1_000;
const DEFAULT_LIMIT: usize = 16;

#[derive(Debug, Clone)]
pub struct QueryPipelineRequest {
    pub text: String,
    pub mode: SearchMode,
    pub limit: usize,
    pub filter: Option<String>,
    pub expansion_mode: ExpansionMode,
    pub retry_count: u8,
    pub timeout_ms: u64,
    pub temporal_start_secs: Option<u64>,
    pub temporal_end_secs: Option<u64>,
}

impl Default for QueryPipelineRequest {
    fn default() -> Self {
        Self {
            text: String::new(),
            mode: SearchMode::Hybrid,
            limit: DEFAULT_LIMIT,
            filter: None,
            expansion_mode: ExpansionMode::None,
            retry_count: 1,
            timeout_ms: 30_000,
            temporal_start_secs: None,
            temporal_end_secs: None,
        }
    }
}

pub async fn run_query_pipeline<E: EmbeddingProvider + ?Sized>(
    storage: Arc<SqliteHybridStorage>,
    embedder: &E,
    request: QueryPipelineRequest,
) -> anyhow::Result<QueryResponse> {
    let start = std::time::Instant::now();
    let limit = request.limit.clamp(1, MAX_LIMIT);
    let variants = expand_query_variants(&request.text, request.expansion_mode);

    if variants.is_empty() {
        return Ok(QueryResponse {
            results: Vec::new(),
            freshness: Freshness::Unknown,
            query_time_ms: start.elapsed().as_millis() as u64,
            truncated: false,
            error: Some("query text cannot be empty".to_string()),
        });
    }

    let searcher = HybridSearcher::new(storage.clone(), HybridRetrievalConfig::default());
    let policy = ResiliencePolicy {
        max_retries: request.retry_count,
        timeout_ms: request.timeout_ms,
        fallback_to_lexical: true,
    };

    let mut merged: BTreeMap<String, SearchResult> = BTreeMap::new();
    let mut last_error: Option<String> = None;

    for (variant, weight) in variants {
        let result = match request.mode {
            SearchMode::Graph => {
                run_graph_search(storage.clone(), &variant, limit, request.filter.as_deref())
            }
            SearchMode::Temporal => run_temporal_search(
                storage.clone(),
                &variant,
                limit,
                request.temporal_start_secs,
                request.temporal_end_secs,
                request.filter.as_deref(),
            ),
            _ => {
                let query = QueryRequest {
                    text: variant.clone(),
                    mode: request.mode,
                    limit,
                    filter: request.filter.clone(),
                };
                let search_result = run_with_resilience(policy, || async {
                    search_with_original_rerank_query(&searcher, embedder, &query, &request.text)
                        .await
                })
                .await;
                match search_result {
                    Ok(candidates) => Ok(candidates
                        .into_iter()
                        .map(|item| item.into_search_result())
                        .collect::<Vec<_>>()),
                    Err(_err)
                        if policy.fallback_to_lexical
                            && matches!(
                                request.mode,
                                SearchMode::Semantic | SearchMode::Hybrid
                            ) =>
                    {
                        let lexical_query = QueryRequest {
                            text: variant.clone(),
                            mode: SearchMode::Lexical,
                            limit,
                            filter: request.filter.clone(),
                        };
                        let lexical_candidates = search_with_original_rerank_query(
                            &searcher,
                            embedder,
                            &lexical_query,
                            &request.text,
                        )
                        .await?;
                        Ok(lexical_candidates
                            .into_iter()
                            .map(|item| item.into_search_result())
                            .collect::<Vec<_>>())
                    }
                    Err(err) => Err(err),
                }
            }
        };

        match result {
            Ok(results) => {
                for mut item in results {
                    item.score = (item.score * weight).clamp(0.0, 1.0);
                    merge_result(&mut merged, item);
                }
            }
            Err(err) => {
                last_error = Some(err.to_string());
            }
        }
    }

    let mut results: Vec<SearchResult> = merged.into_values().collect();
    results.sort_by(|lhs, rhs| rhs.score.partial_cmp(&lhs.score).unwrap_or(Ordering::Equal));
    let truncated = results.len() > limit;
    results.truncate(limit);

    let (results, validation_summary) = validate_results(results);
    let mut error = last_error;
    if validation_summary.dropped_empty > 0 {
        error = Some(format!("dropped {} invalid result(s)", validation_summary.dropped_empty));
    }

    Ok(QueryResponse {
        results,
        freshness: infer_freshness(&storage.index_stats()?),
        query_time_ms: start.elapsed().as_millis() as u64,
        truncated,
        error,
    })
}

async fn search_with_original_rerank_query<E: EmbeddingProvider + ?Sized>(
    searcher: &HybridSearcher,
    embedder: &E,
    retrieval_query: &QueryRequest,
    original_query: &str,
) -> anyhow::Result<Vec<crate::retrieval::hybrid_search::HybridCandidate>> {
    let candidates = searcher.search(embedder, retrieval_query).await?;
    let reranked = searcher.reranker().rerank(
        original_query,
        candidates.into_iter().map(RerankedCandidate::from).collect(),
        searcher.rerank_top_k(),
    );
    Ok(reranked.into_iter().map(crate::retrieval::hybrid_search::HybridCandidate::from).collect())
}

pub fn run_status_pipeline(storage: &SqliteHybridStorage) -> anyhow::Result<IndexStats> {
    storage.index_stats()
}

fn run_graph_search(
    storage: Arc<SqliteHybridStorage>,
    query: &str,
    limit: usize,
    filter: Option<&str>,
) -> anyhow::Result<Vec<SearchResult>> {
    let seed = filter.unwrap_or(query);
    let hits = storage.graph_search_module_deps(seed, limit)?;
    Ok(hits.into_iter().map(chunk_to_result).collect())
}

fn run_temporal_search(
    storage: Arc<SqliteHybridStorage>,
    query: &str,
    limit: usize,
    start_secs: Option<u64>,
    end_secs: Option<u64>,
    filter: Option<&str>,
) -> anyhow::Result<Vec<SearchResult>> {
    let from = start_secs.unwrap_or(0);
    let to = end_secs.unwrap_or(u64::MAX);
    let hits = storage.search_with_temporal_range(query, limit, from, to, filter)?;
    Ok(hits.into_iter().map(chunk_to_result).collect())
}

fn chunk_to_result(hit: crate::storage::ChunkHit) -> SearchResult {
    SearchResult {
        key: hit.key,
        title: hit.module,
        snippet: hit.text,
        score: hit.score,
        source: hit.source,
    }
}

fn merge_result(merged: &mut BTreeMap<String, SearchResult>, candidate: SearchResult) {
    match merged.get(&candidate.key) {
        Some(existing) if existing.score >= candidate.score => {}
        _ => {
            merged.insert(candidate.key.clone(), candidate);
        }
    }
}

fn infer_freshness(stats: &IndexStats) -> Freshness {
    if stats.is_stale {
        Freshness::Stale
    } else if stats.indexed_count == 0 {
        Freshness::Unknown
    } else {
        Freshness::Fresh
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::embedding::EmbeddingProvider as EmbeddingProviderTrait;
    use crate::chunking::{ChunkType, MirrChunk};
    use async_trait::async_trait;

    struct FixedEmbedder;

    #[async_trait]
    impl EmbeddingProviderTrait for FixedEmbedder {
        async fn embed(&self, _text: &str) -> anyhow::Result<Vec<f32>> {
            Ok(vec![1.0, 0.0])
        }
    }

    fn temp_db(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("mirr-query-handler-{}-{}.db", name, std::process::id()));
        path
    }

    fn sample_chunk(id: &str, text: &str, module: &str) -> MirrChunk {
        MirrChunk::new(
            id.to_string(),
            ChunkType::Module,
            text.to_string(),
            module.to_string(),
            None,
            (1, 2),
        )
    }

    #[tokio::test]
    async fn query_pipeline_applies_expansion_and_merges() {
        let storage =
            Arc::new(SqliteHybridStorage::from_db_path(temp_db("merge")).expect("storage"));
        storage
            .upsert_chunk(
                &sample_chunk("demo.alpha", "signal alpha emits value", "demo"),
                "src/demo.mirr",
                Some(&[1.0, 0.0]),
            )
            .expect("insert");

        let request = QueryPipelineRequest {
            text: "signal alpha".to_string(),
            expansion_mode: ExpansionMode::Synonym,
            ..QueryPipelineRequest::default()
        };

        let response = run_query_pipeline(storage, &FixedEmbedder, request).await.expect("query");
        assert!(!response.results.is_empty());
    }
}
