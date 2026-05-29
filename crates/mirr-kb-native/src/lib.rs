#![allow(unsafe_code)]
#![deny(warnings)]

pub mod adapters;
pub mod chunking;
pub mod config;
pub mod context;
pub mod evaluation;
pub mod expansion;
pub mod query_handler;
pub mod reranking;
pub mod resilience;
pub mod retrieval;
pub mod storage;
pub mod validation;

pub use chunking::{compute_hash, estimate_token_count, ChunkType, MirrChunk};
pub use config::{EmbeddingProvider, RagConfig, RagConfigError};
pub use context::{validate_query_size, ContextBudget};
pub use evaluation::{
    default_golden_qa_set, evaluate_pair, passes_quality_gate_for_set, Difficulty,
    EvaluationResult, GoldenQAPair,
};
pub use expansion::{expand_query_variants, ExpansionMode};
pub use query_handler::{run_query_pipeline, run_status_pipeline, QueryPipelineRequest};
pub use reranking::{FlashRankReranker, RerankedCandidate, RerankedResult};
pub use resilience::{ResiliencePolicy, RetryDecision};
pub use retrieval::{QueryRequest, QueryResponse, Retrieval, SearchMode};
pub use storage::{ChunkHit, IndexStats, SqliteHybridStorage};
pub use validation::{sanitize_text, validate_results, ValidationSummary};
