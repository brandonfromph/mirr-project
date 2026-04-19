#![forbid(unsafe_code)]
#![deny(warnings)]

use std::env;

/// Maximum number of query results returned in a single response.
const MAX_QUERY_RESULTS: usize = 1000;

/// Maximum number of symbols indexed in the knowledge base.
const MAX_INDEXED_SYMBOLS: usize = 100_000;

/// Maximum query timeout in milliseconds.
const MAX_QUERY_TIMEOUT_MS: u64 = 30_000;

/// Maximum context budget in tokens.
const MAX_CONTEXT_BUDGET_TOKENS: usize = 8_000;

/// Maximum number of chunks to keep after reranking.
const MAX_CONTEXT_CHUNKS: usize = 8;

/// Default embedding dimensions for cloud providers.
const DEFAULT_EMBEDDING_DIMENSIONS: usize = 768;

/// Supported embedding providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbeddingProvider {
    /// Gemini or equivalent cloud embedding service.
    Gemini,
    /// Deterministic local fallback (no cloud required).
    Local,
}

/// Configuration for the RAG system, sourced from environment variables.
#[derive(Debug, Clone)]
pub struct RagConfig {
    /// Root directory for KB storage (e.g., `.kb-data`).
    pub kb_root: String,

    /// API key for cloud embedding providers.
    pub embedding_api_key: Option<String>,

    /// Embedding dimensionality (384 or 768 preferred).
    pub embedding_dimensions: usize,

    /// Whether hybrid search (BM25 + vector + RRF) is enabled.
    pub hybrid_search_enabled: bool,

    /// Whether reranking is enabled.
    pub reranking_enabled: bool,

    /// Whether dual-run telemetry is enabled.
    pub dual_run_enabled: bool,

    /// Maximum assembled context budget in tokens.
    pub context_budget_tokens: usize,

    /// Maximum number of chunks after reranking.
    pub max_context_chunks: usize,

    /// Optional Qdrant cluster URL. If set, semantic retrieval is enabled.
    pub qdrant_url: Option<String>,

    /// Optional Qdrant API key. Required if qdrant_url is set.
    pub qdrant_api_key: Option<String>,

    /// Embedding provider strategy (Gemini or Local).
    pub embedding_provider: EmbeddingProvider,

    /// Maximum number of results per query.
    pub max_query_results: usize,

    /// Maximum number of symbols to index.
    pub max_indexed_symbols: usize,

    /// Query timeout in milliseconds.
    pub query_timeout_ms: u64,
}

/// Configuration errors.
#[derive(Debug, Clone)]
pub enum RagConfigError {
    /// Qdrant URL is set but API key is missing.
    MissingQdrantApiKey,
    /// Invalid bounds for configuration parameter.
    InvalidBounds(String),
    /// Parse error for integer environment variable.
    ParseError(String),
}

impl std::fmt::Display for RagConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RagConfigError::MissingQdrantApiKey => {
                write!(f, "MIRR_QDRANT_URL is set but MIRR_QDRANT_API_KEY is missing")
            }
            RagConfigError::InvalidBounds(param) => {
                write!(f, "Invalid bounds for configuration parameter: {}", param)
            }
            RagConfigError::ParseError(msg) => {
                write!(f, "Failed to parse environment variable: {}", msg)
            }
        }
    }
}

impl std::error::Error for RagConfigError {}

impl RagConfig {
    /// Create configuration from environment variables with defaults.
    ///
    /// Environment variables (all optional):
    /// - `MIRR_KB_ROOT`: Root directory for KB storage (default: `.kb-data`)
    /// - `MIRR_QDRANT_URL`: Qdrant cluster URL (no default; if set, enables semantic retrieval)
    /// - `MIRR_QDRANT_API_KEY`: Qdrant API key (required if MIRR_QDRANT_URL is set)
    /// - `MIRR_EMBEDDING_API_KEY`: Cloud embedding API key (optional; fallback is lexical-only)
    /// - `MIRR_EMBEDDING_PROVIDER`: Provider selection, "gemini" or "local" (default: "local")
    /// - `MIRR_EMBEDDING_DIMENSIONS`: Embedding dimensions (default: 768)
    /// - `MIRR_KB_HYBRID_SEARCH`: Enable hybrid BM25 + vector search (default: true)
    /// - `MIRR_KB_RERANKING`: Enable reranking (default: true)
    /// - `MIRR_KB_DUALRUN`: Enable dual-run telemetry (default: false)
    /// - `MIRR_CONTEXT_BUDGET_TOKENS`: Context budget tokens (default: 8000)
    /// - `MIRR_MAX_CONTEXT_CHUNKS`: Max reranked chunks in context (default: 8)
    /// - `MIRR_MAX_QUERY_RESULTS`: Max results per query (default: 16, max: 1000)
    /// - `MIRR_MAX_INDEXED_SYMBOLS`: Max symbols in index (default: 10000, max: 100000)
    /// - `MIRR_QUERY_TIMEOUT_MS`: Query timeout in ms (default: 5000, max: 30000)
    pub fn from_env() -> Result<Self, RagConfigError> {
        let kb_root = env::var("MIRR_KB_ROOT").unwrap_or_else(|_| ".kb-data".to_string());
        let embedding_api_key = env::var("MIRR_EMBEDDING_API_KEY").ok();

        let embedding_dimensions = env::var("MIRR_EMBEDDING_DIMENSIONS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(DEFAULT_EMBEDDING_DIMENSIONS);

        let hybrid_search_enabled = env_flag("MIRR_KB_HYBRID_SEARCH", true)?;
        let reranking_enabled = env_flag("MIRR_KB_RERANKING", true)?;
        let dual_run_enabled = env_flag("MIRR_KB_DUALRUN", false)?;

        let context_budget_tokens = env::var("MIRR_CONTEXT_BUDGET_TOKENS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(MAX_CONTEXT_BUDGET_TOKENS);

        let max_context_chunks = env::var("MIRR_MAX_CONTEXT_CHUNKS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(MAX_CONTEXT_CHUNKS);

        let qdrant_url = env::var("MIRR_QDRANT_URL").ok();
        let qdrant_api_key = env::var("MIRR_QDRANT_API_KEY").ok();

        // Both required if semantic retrieval is enabled
        if let Some(url) = &qdrant_url {
            if !url.is_empty() && qdrant_api_key.is_none() {
                return Err(RagConfigError::MissingQdrantApiKey);
            }
        }

        let provider_str = env::var("MIRR_EMBEDDING_PROVIDER")
            .unwrap_or_else(|_| "local".to_string())
            .to_lowercase();

        let embedding_provider = match provider_str.as_str() {
            "gemini" => EmbeddingProvider::Gemini,
            "local" => EmbeddingProvider::Local,
            other => {
                return Err(RagConfigError::ParseError(format!(
                    "Unknown embedding provider: {}",
                    other
                )))
            }
        };

        let max_query_results = env::var("MIRR_MAX_QUERY_RESULTS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(16);

        let max_indexed_symbols = env::var("MIRR_MAX_INDEXED_SYMBOLS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(10_000);

        let query_timeout_ms = env::var("MIRR_QUERY_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5_000);

        let cfg = RagConfig {
            kb_root,
            embedding_api_key,
            embedding_dimensions,
            hybrid_search_enabled,
            reranking_enabled,
            dual_run_enabled,
            context_budget_tokens,
            max_context_chunks,
            qdrant_url,
            qdrant_api_key,
            embedding_provider,
            max_query_results,
            max_indexed_symbols,
            query_timeout_ms,
        };

        cfg.validate()?;
        Ok(cfg)
    }

    /// Validate configuration bounds. Called automatically by `from_env()`.
    fn validate(&self) -> Result<(), RagConfigError> {
        if self.max_query_results == 0 || self.max_query_results > MAX_QUERY_RESULTS {
            return Err(RagConfigError::InvalidBounds(format!(
                "max_query_results must be 1–{}",
                MAX_QUERY_RESULTS
            )));
        }

        if self.max_indexed_symbols == 0 || self.max_indexed_symbols > MAX_INDEXED_SYMBOLS {
            return Err(RagConfigError::InvalidBounds(format!(
                "max_indexed_symbols must be 1–{}",
                MAX_INDEXED_SYMBOLS
            )));
        }

        if self.query_timeout_ms == 0 || self.query_timeout_ms > MAX_QUERY_TIMEOUT_MS {
            return Err(RagConfigError::InvalidBounds(format!(
                "query_timeout_ms must be 1–{}",
                MAX_QUERY_TIMEOUT_MS
            )));
        }

        if self.context_budget_tokens == 0 || self.context_budget_tokens > MAX_CONTEXT_BUDGET_TOKENS
        {
            return Err(RagConfigError::InvalidBounds(format!(
                "context_budget_tokens must be 1–{}",
                MAX_CONTEXT_BUDGET_TOKENS
            )));
        }

        if self.max_context_chunks == 0 || self.max_context_chunks > MAX_CONTEXT_CHUNKS {
            return Err(RagConfigError::InvalidBounds(format!(
                "max_context_chunks must be 1–{}",
                MAX_CONTEXT_CHUNKS
            )));
        }

        if self.embedding_dimensions != 384
            && self.embedding_dimensions != 768
            && self.embedding_dimensions != 1536
        {
            return Err(RagConfigError::InvalidBounds(
                "embedding_dimensions must be 384, 768, or 1536".to_string(),
            ));
        }

        Ok(())
    }
}

fn env_flag(name: &str, default: bool) -> Result<bool, RagConfigError> {
    match env::var(name) {
        Ok(value) => match value.to_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            other => Err(RagConfigError::ParseError(format!(
                "Unknown boolean value for {}: {}",
                name, other
            ))),
        },
        Err(_) => Ok(default),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        // When no env vars are set, should use defaults
        let cfg = RagConfig::from_env().expect("defaults should always be valid");
        assert_eq!(cfg.kb_root, ".kb-data");
        assert!(cfg.qdrant_url.is_none());
        assert_eq!(cfg.embedding_provider, EmbeddingProvider::Local);
        assert_eq!(cfg.max_query_results, 16);
        assert_eq!(cfg.max_indexed_symbols, 10_000);
        assert_eq!(cfg.query_timeout_ms, 5_000);
    }

    #[test]
    fn test_config_bounds() {
        // Validation should reject out-of-bounds values
        // Note: In real testing with env vars, this would be set externally
        // For now, we defer to integration tests that actually set env vars
    }
}
