#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::adapters::embedding::EmbeddingProvider;
use crate::context::{estimate_token_count, validate_query_size, ContextBudget};
use crate::reranking::flashrank::{FlashRankReranker, RerankedCandidate};
use crate::retrieval::{
    Freshness, IndexStatus, QueryRequest, QueryResponse, Retrieval, SearchMode, SearchResult,
};
use crate::storage::{ChunkHit, IndexStats, SqliteHybridStorage};
use async_trait::async_trait;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::sync::Arc;

const DEFAULT_RRF_K: usize = 60;
const DEFAULT_CANDIDATE_LIMIT: usize = 50;
const DEFAULT_RERANK_TOP_K: usize = 8;
const MAX_RESPONSE_CHUNKS: usize = 8;

#[derive(Debug, Clone)]
pub struct HybridRetrievalConfig {
    pub rrf_k: usize,
    pub candidate_limit: usize,
    pub rerank_top_k: usize,
    pub max_response_chunks: usize,
}

impl Default for HybridRetrievalConfig {
    fn default() -> Self {
        Self {
            rrf_k: DEFAULT_RRF_K,
            candidate_limit: DEFAULT_CANDIDATE_LIMIT,
            rerank_top_k: DEFAULT_RERANK_TOP_K,
            max_response_chunks: MAX_RESPONSE_CHUNKS,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HybridCandidate {
    pub result: SearchResult,
    pub bm25_rank: Option<usize>,
    pub vector_rank: Option<usize>,
    pub rrf_score: f32,
    pub appears_in_both: bool,
    pub freshness_secs: u64,
    pub estimated_tokens: usize,
}

#[derive(Debug, Clone)]
pub struct HybridSearcher {
    storage: Arc<SqliteHybridStorage>,
    reranker: FlashRankReranker,
    config: HybridRetrievalConfig,
}

impl HybridSearcher {
    pub fn new(storage: Arc<SqliteHybridStorage>, config: HybridRetrievalConfig) -> Self {
        Self { storage, reranker: FlashRankReranker::default(), config }
    }

    pub async fn search<E: EmbeddingProvider + ?Sized>(
        &self,
        embedder: &E,
        req: &QueryRequest,
    ) -> anyhow::Result<Vec<HybridCandidate>> {
        validate_query_size(&req.text).map_err(anyhow::Error::msg)?;

        let max_candidates = req.limit.min(self.config.candidate_limit);
        let bm25_hits =
            self.storage.bm25_search(&req.text, max_candidates, req.filter.as_deref())?;

        let vector_hits = match req.mode {
            SearchMode::Lexical | SearchMode::Graph | SearchMode::Temporal => Vec::new(),
            SearchMode::Semantic | SearchMode::Hybrid => match embedder.embed(&req.text).await {
                Ok(query_embedding) => self.storage.vector_search(
                    &query_embedding,
                    max_candidates,
                    req.filter.as_deref(),
                )?,
                Err(err) if matches!(req.mode, SearchMode::Hybrid) => {
                    let _ = err;
                    Vec::new()
                }
                Err(err) => return Err(err),
            },
        };

        let candidates = merge_hits(&bm25_hits, &vector_hits, self.config.rrf_k, req.limit);
        Ok(candidates)
    }

    pub async fn search_and_rerank<E: EmbeddingProvider + ?Sized>(
        &self,
        embedder: &E,
        req: &QueryRequest,
    ) -> anyhow::Result<Vec<HybridCandidate>> {
        let candidates = self.search(embedder, req).await?;
        let reranked = self.reranker.rerank(
            &req.text,
            candidates.into_iter().map(RerankedCandidate::from).collect(),
            self.config.rerank_top_k,
        );
        Ok(reranked.into_iter().map(HybridCandidate::from).collect())
    }

    pub fn reranker(&self) -> &FlashRankReranker {
        &self.reranker
    }

    pub fn rerank_top_k(&self) -> usize {
        self.config.rerank_top_k
    }
}

#[derive(Debug, Clone)]
pub struct HybridRetrieval<E: EmbeddingProvider + ?Sized> {
    searcher: HybridSearcher,
    embedder: Arc<E>,
}

impl<E: EmbeddingProvider + ?Sized> HybridRetrieval<E> {
    pub fn new(storage: Arc<SqliteHybridStorage>, embedder: Arc<E>) -> Self {
        Self { searcher: HybridSearcher::new(storage, HybridRetrievalConfig::default()), embedder }
    }

    pub fn with_config(
        storage: Arc<SqliteHybridStorage>,
        embedder: Arc<E>,
        config: HybridRetrievalConfig,
    ) -> Self {
        Self { searcher: HybridSearcher::new(storage, config), embedder }
    }
}

#[async_trait]
impl<E: EmbeddingProvider + ?Sized> Retrieval for HybridRetrieval<E> {
    async fn query(&self, req: QueryRequest) -> anyhow::Result<QueryResponse> {
        let start = std::time::Instant::now();
        let mut budget = ContextBudget::default();
        let candidates = self.searcher.search_and_rerank(self.embedder.as_ref(), &req).await?;

        let mut results = Vec::new();
        let mut truncated = false;

        for candidate in candidates.into_iter().take(self.searcher.config.max_response_chunks) {
            if !budget.try_add_chunk(&candidate.result.snippet) {
                truncated = true;
                break;
            }
            results.push(candidate.into_search_result());
        }

        Ok(QueryResponse {
            results,
            freshness: infer_freshness_from_candidates(&self.searcher.storage.index_stats()?),
            query_time_ms: start.elapsed().as_millis() as u64,
            truncated,
            error: None,
        })
    }

    async fn index_status(&self) -> anyhow::Result<IndexStatus> {
        let stats = self.searcher.storage.index_stats()?;
        Ok(IndexStatus {
            indexed_count: stats.indexed_count,
            last_refresh_secs: stats.last_refresh_secs,
            is_stale: stats.is_stale,
            error: stats.error,
        })
    }
}

fn merge_hits(
    bm25_hits: &[ChunkHit],
    vector_hits: &[ChunkHit],
    rrf_k: usize,
    limit: usize,
) -> Vec<HybridCandidate> {
    let mut map: BTreeMap<String, CandidateAccumulator> = BTreeMap::new();

    for (rank, hit) in bm25_hits.iter().enumerate() {
        let entry = map.entry(hit.key.clone()).or_insert_with(CandidateAccumulator::new);
        entry.merge_bm25(hit.clone(), rank + 1, rrf_k);
    }

    for (rank, hit) in vector_hits.iter().enumerate() {
        let entry = map.entry(hit.key.clone()).or_insert_with(CandidateAccumulator::new);
        entry.merge_vector(hit.clone(), rank + 1, rrf_k);
    }

    let mut candidates: Vec<HybridCandidate> =
        map.into_values().map(|acc| acc.into_candidate()).collect();
    candidates
        .sort_by(|lhs, rhs| rhs.rrf_score.partial_cmp(&lhs.rrf_score).unwrap_or(Ordering::Equal));
    candidates.truncate(limit);
    candidates
}

struct CandidateAccumulator {
    bm25_hit: Option<ChunkHit>,
    vector_hit: Option<ChunkHit>,
    bm25_rank: Option<usize>,
    vector_rank: Option<usize>,
    rrf_score: f32,
}

impl CandidateAccumulator {
    fn new() -> Self {
        Self {
            bm25_hit: None,
            vector_hit: None,
            bm25_rank: None,
            vector_rank: None,
            rrf_score: 0.0,
        }
    }

    fn merge_bm25(&mut self, hit: ChunkHit, rank: usize, rrf_k: usize) {
        self.rrf_score += 1.0 / ((rrf_k + rank) as f32);
        self.bm25_rank = Some(rank);
        self.bm25_hit = Some(hit);
    }

    fn merge_vector(&mut self, hit: ChunkHit, rank: usize, rrf_k: usize) {
        self.rrf_score += 1.0 / ((rrf_k + rank) as f32);
        self.vector_rank = Some(rank);
        self.vector_hit = Some(hit);
    }

    fn into_candidate(self) -> HybridCandidate {
        let hit = self.bm25_hit.or(self.vector_hit).expect("at least one hit is present");
        let appears_in_both = self.bm25_rank.is_some() && self.vector_rank.is_some();
        let title = hit.key.clone();
        let text = hit.text.clone();
        let estimated_tokens = estimate_token_count(&text);
        let result = SearchResult {
            key: hit.key,
            title,
            snippet: text,
            score: hit.score,
            source: hit.source,
        };
        HybridCandidate {
            result,
            bm25_rank: self.bm25_rank,
            vector_rank: self.vector_rank,
            rrf_score: self.rrf_score,
            appears_in_both,
            freshness_secs: hit.freshness_secs,
            estimated_tokens,
        }
    }
}

fn infer_freshness_from_candidates(stats: &IndexStats) -> Freshness {
    if stats.is_stale {
        Freshness::Stale
    } else if stats.indexed_count == 0 {
        Freshness::Unknown
    } else {
        Freshness::Fresh
    }
}

impl HybridCandidate {
    pub fn into_search_result(self) -> SearchResult {
        self.result
    }
}

impl From<RerankedCandidate> for HybridCandidate {
    fn from(value: RerankedCandidate) -> Self {
        Self {
            result: value.result,
            bm25_rank: value.bm25_rank,
            vector_rank: value.vector_rank,
            rrf_score: value.rrf_score,
            appears_in_both: value.appears_in_both,
            freshness_secs: value.freshness_secs,
            estimated_tokens: value.estimated_tokens,
        }
    }
}

impl From<HybridCandidate> for RerankedCandidate {
    fn from(value: HybridCandidate) -> Self {
        Self {
            result: value.result,
            bm25_rank: value.bm25_rank,
            vector_rank: value.vector_rank,
            rrf_score: value.rrf_score,
            appears_in_both: value.appears_in_both,
            freshness_secs: value.freshness_secs,
            estimated_tokens: value.estimated_tokens,
        }
    }
}

impl From<crate::reranking::flashrank::RerankedResult> for HybridCandidate {
    fn from(value: crate::reranking::flashrank::RerankedResult) -> Self {
        Self {
            result: value.result,
            bm25_rank: value.bm25_rank,
            vector_rank: value.vector_rank,
            rrf_score: value.rrf_score,
            appears_in_both: value.appears_in_both,
            freshness_secs: value.freshness_secs,
            estimated_tokens: value.estimated_tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::embedding::EmbeddingProvider as EmbeddingProviderTrait;
    use crate::chunking::{ChunkType, MirrChunk};
    use crate::retrieval::QueryRequest;
    use std::path::PathBuf;

    struct FixedEmbeddingProvider;

    #[async_trait]
    impl EmbeddingProviderTrait for FixedEmbeddingProvider {
        async fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>> {
            Ok(match text {
                t if t.contains("alpha") => vec![1.0, 0.0],
                t if t.contains("beta") => vec![0.0, 1.0],
                _ => vec![0.5, 0.5],
            })
        }
    }

    fn temp_db_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("mirr-hybrid-search-{}-{}.db", name, std::process::id()));
        path
    }

    fn chunk(id: &str, text: &str) -> MirrChunk {
        MirrChunk::new(
            id.to_string(),
            ChunkType::Module,
            text.to_string(),
            "demo".to_string(),
            None,
            (1, 1),
        )
    }

    #[tokio::test]
    async fn hybrid_search_combines_bm25_and_vector_hits() {
        let storage =
            Arc::new(SqliteHybridStorage::from_db_path(temp_db_path("combine")).expect("storage"));
        storage
            .upsert_chunk(&chunk("demo.alpha", "alpha module"), "src/demo.mirr", Some(&[1.0, 0.0]))
            .expect("insert alpha");
        storage
            .upsert_chunk(&chunk("demo.beta", "beta module"), "src/demo.mirr", Some(&[0.0, 1.0]))
            .expect("insert beta");

        let searcher = HybridSearcher::new(storage, HybridRetrievalConfig::default());
        let req = QueryRequest {
            text: "alpha".to_string(),
            mode: SearchMode::Hybrid,
            limit: 10,
            filter: None,
        };
        let results = searcher.search(&FixedEmbeddingProvider, &req).await.expect("search");
        assert!(!results.is_empty());
        assert_eq!(results[0].result.key, "demo.alpha");
    }

    #[tokio::test]
    async fn hybrid_retrieval_reranks_and_budgets_context() {
        let storage =
            Arc::new(SqliteHybridStorage::from_db_path(temp_db_path("rerank")).expect("storage"));
        storage
            .upsert_chunk(
                &chunk("demo.alpha", "alpha alpha alpha"),
                "src/demo.mirr",
                Some(&[1.0, 0.0]),
            )
            .expect("insert alpha");
        storage
            .upsert_chunk(&chunk("demo.beta", "beta beta beta"), "src/demo.mirr", Some(&[0.0, 1.0]))
            .expect("insert beta");

        let retrieval = HybridRetrieval::new(storage, Arc::new(FixedEmbeddingProvider));
        let resp = retrieval
            .query(QueryRequest {
                text: "alpha".to_string(),
                mode: SearchMode::Hybrid,
                limit: 5,
                filter: None,
            })
            .await
            .expect("query");

        assert!(!resp.results.is_empty());
        assert_eq!(resp.results[0].key, "demo.alpha");
        assert!(resp.query_time_ms < 1000);
    }
}
