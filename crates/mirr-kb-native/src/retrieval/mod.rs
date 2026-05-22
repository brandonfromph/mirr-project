#![forbid(unsafe_code)]
#![deny(warnings)]

pub mod hybrid_search;
pub mod lexical;

pub use hybrid_search::{HybridCandidate, HybridRetrieval, HybridRetrievalConfig, HybridSearcher};
pub use lexical::LexicalRetrieval;

use async_trait::async_trait;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Search mode for retrieval queries.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    /// Lexical (local deterministic) search only.
    Lexical,
    /// Semantic (cloud-based embedding) search only.
    Semantic,
    /// Hybrid: try semantic first, fall back to lexical on error or unavailability.
    Hybrid,
    /// Graph-aware traversal seeded from module references.
    Graph,
    /// Temporal windowed search over freshness timestamps.
    Temporal,
}

/// Query request for the retrieval system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    /// Search query text (max 4096 bytes).
    pub text: String,

    /// Search mode (lexical, semantic, or hybrid).
    pub mode: SearchMode,

    /// Maximum results to return (1–1000, bounded by config).
    pub limit: usize,

    /// Optional file or module filter (for scoped search).
    pub filter: Option<String>,
}

/// Individual search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document or symbol identifier.
    pub key: String,

    /// Document title or symbol name.
    pub title: String,

    /// Snippet of matching content.
    pub snippet: String,

    /// Relevance score (0.0–1.0).
    pub score: f32,

    /// Source file or module.
    pub source: String,
}

/// Response from a retrieval query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    /// List of search results.
    pub results: Vec<SearchResult>,

    /// Whether this result set is fresh (recently indexed) or potentially stale.
    pub freshness: Freshness,

    /// Total milliseconds spent on the query.
    pub query_time_ms: u64,

    /// Whether results were truncated due to limit.
    pub truncated: bool,

    /// Optional error message if query partially failed but still returned results.
    pub error: Option<String>,
}

/// Index freshness indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Freshness {
    /// Index was recently refreshed (within last hour).
    Fresh,
    /// Index may be stale (older than configured threshold).
    Stale,
    /// Freshness status unknown.
    Unknown,
}

/// Trait for pluggable retrieval backends.
/// Implementations provide lexical, semantic, or hybrid search capabilities.
#[async_trait]
pub trait Retrieval: Send + Sync {
    /// Execute a search query.
    /// Returns a QueryResponse with results bounded by config limits.
    async fn query(&self, req: QueryRequest) -> anyhow::Result<QueryResponse>;

    /// Get the current index status and freshness information.
    async fn index_status(&self) -> anyhow::Result<IndexStatus>;
}

/// Index status snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStatus {
    /// Number of indexed symbols.
    pub indexed_count: usize,

    /// Last index refresh timestamp (Unix seconds).
    pub last_refresh_secs: u64,

    /// Whether the index is currently stale.
    pub is_stale: bool,

    /// Optional error message if index build failed recently.
    pub error: Option<String>,
}
