# PROPOSAL-107: Phase 3 KB-Native RAG Complete Implementation (2026-04-14)

**Status**: DRAFT (awaiting SIGN)  
**Campaign ID**: P107  
**Target**: Full KB-native RAG with chunking, hybrid search, reranking, evaluation framework, and dual-run parity mode  
**Scope**: 12 files (6 new, 6 modified)  
**Risk Level**: HIGH (architectural foundation for Phase 4-6)  

---

## Part 1: Audit

### Current State Assessment

| Area | Status | Evidence |
|------|--------|----------|
| Resolver routing | ✅ Complete | mrt_dispatch_invocation_resolver.rs: MrtKbQuery arm (line 599), MrtBrainGet arm (line 561) |
| Discovery metadata | ✅ Complete | canonical_discovery_method_metadata.rs: both tools with input schema |
| Role allowlist | ✅ Complete | mrt_dispatch_tool_role_allowlist.rs: both tools configured |
| Manifest fixture | ✅ Complete | mcp_manifest_snapshot.json: both tools with schemas |
| Route host structure | ✅ Ready | axum_route_host.rs: dispatch_canonical_route (line 417) is hook point |
| KB retrieval traits | ✅ Complete | mirr-kb-native crate: Retrieval trait, QueryRequest, QueryResponse |
| Chunking strategy | ❌ Missing | No MIRR-native boundaries defined yet |
| Hybrid search | ❌ Missing | No BM25+vector+RRF implementation |
| Reranking | ❌ Missing | No FlashRank or cross-encoder integration |
| Evaluation framework | ❌ Missing | No RAGAS/DeepEval setup or golden QA pairs |
| SQLite FTS5+sqlite-vec | ❌ Missing | Not yet in mirr-kb-native storage layer |
| Context budget enforcement | ❌ Missing | No 8K token or 4-8 chunk limits |

### Public API Surface (Existing Files)

**mirr-kb-native/src/lib.rs**:
- `pub mod adapters`, `pub mod config`, `pub mod retrieval`
- `pub use config::{EmbeddingProvider, RagConfig, RagConfigError}`
- `pub use retrieval::{QueryRequest, QueryResponse, Retrieval, SearchMode}`

**mirr-kb-native/src/retrieval/mod.rs**:
- `trait Retrieval`: `async fn query()`, `async fn index_status()`
- `SearchMode`: Lexical, Semantic, Hybrid
- `QueryRequest`: text, mode, limit, filter
- `QueryResponse`: results, freshness, query_time_ms, truncated, error

**mirr-kb-native/src/retrieval/lexical.rs**:
- `LexicalRetrieval`: current SQLite key/value impl (to be extended)

---

## Part 2: Philosophy Gate and Debt Audit

### Philosophy Gate Check

✅ **Three-construct surface**: No 4th construct added. Signal/guard/reflex remain singular.  
✅ **Deterministic bounds**: Explicit MAX_* for query string (4KB), result count (1000), context budget (8K tokens), chunk limit (8 max).  
✅ **Hardware-synthesizable**: Telemetry and reranking are metadata-only; no impact on signal/guard/reflex.  
✅ **No unsafe code**: All new modules will have `#![forbid(unsafe_code)]`.  
✅ **Zero-Debt**: Chunking, hybrid search, reranking are core components (not temporary shims).  

**GATE VERDICT: PASS**

### Debt Audit

| # | Prohibition | Findings in scope | Action |
|---|-------------|-------------------|--------|
| D1 | No wrapper functions | None found in new modules | N/A |
| D2 | No deprecated aliases | None found | N/A |
| D3 | No dead code | Old lexical.rs key/value methods will be extended (not deleted) to support chunking | Preserve backward compat with new storage layer on top |
| D4 | No redundant abstractions | None found | N/A |
| D5 | No backward-compat shims | Dual-run mode is feature-gated in config, not permanent | N/A |
| D6 | No duplicate logic | None found | N/A |
| D7 | No misleading comments | All comments align with phase 3 chunking/search/reranking semantics | N/A |

**AUDIT VERDICT: PASS (no findings requiring resolution)**

---

## Part 3: Risk and Constraint Analysis

### Risks

| # | Risk | Severity | Mitigation |
|---|------|----------|------------|
| R1 | Chunking quality directly impacts RAG success (80% of problems are chunking) | HIGH | Implement MIRR-native boundaries first; extensive unit tests; validate chunk granularity before hybrid search |
| R2 | Embedding API latency on i5 could exceed 5s timeout | MEDIUM | Async API calls; cache embeddings; timeout with fallback to lexical-only |
| R3 | Vector quantization (8-bit) may reduce search accuracy | MEDIUM | Start with 768-dim unquantized; only quantize if corpus exceeds 50k vectors; measure degradation |
| R4 | RRF fusion weighting (k=60) may need tuning for MIRR domain | MEDIUM | Configurable k value; initial tests with k=60; tune based on golden QA evaluation |
| R5 | FlashRank reranking latency on 50 candidates could hit 20ms limit | MEDIUM | Profile on i5-class hardware; batch rerank; measure latency in integration tests |
| R6 | Golden QA evaluation set (20-30 pairs) may be biased | MEDIUM | Diverse query types (structural, semantic, cross-module); manual review of answers |
| R7 | Dual-run mode doubles query latency if both paths execute serially | MEDIUM | Execute in parallel with tokio::join!; per-path timeout; return legacy if new times out |
| R8 | Response structure parity comparison may miss semantic drift | MEDIUM | Structured comparison of result counts, truncation flags, top-k relevance; snapshot testing |

### Constraints

| # | Constraint | Source | Enforced by |
|---|-----------|--------|-------------|
| C1 | No unsafe code | NASA P10 | `#![forbid(unsafe_code)]` in all new modules |
| C2 | Zero warnings | CI | `#![deny(warnings)]` + clippy all-targets |
| C3 | Query latency bounded | Phase 3 requirement | MAX_QUERY_TIME_MS = 5000 per path |
| C4 | Context budget enforced | Architecture memo | Enforce < 8K tokens, 4-8 chunks max |
| C5 | Backward compatibility | Plan requirement | Legacy mrt_brain_get unchanged; dual-run is opt-in |
| C6 | Chunking primacy | Research consensus (EMNLP 2025) | MIRR-native boundaries implemented first |
| C7 | Embedding provider contract | Architecture memo | Cloud API only (Voyage/OpenAI), env-var credentials, deterministic fallback |
| C8 | Reranking non-negotiable | Architecture memo | FlashRank MiniLM-L-12-v2 required before top-k selection |
| C9 | Evaluation before cutover | Architecture memo | RAGAS metrics + golden QA pairs required before Phase 4 start |

---

## Part 4: Detailed Sections

### Section A: MIRR-Native Chunking Strategy
**File**: `crates/mirr-kb-native/src/chunking/mirr_boundaries.rs` (new)

Implement MIRR-native semantic chunking:

```rust
pub struct MirrChunk {
    pub id: String,           // e.g., "module_foo.signal_bar"
    pub chunk_type: ChunkType,// Module, Signal, Guard, Reflex, Property
    pub text: String,         // Source text of chunk
    pub module: String,       // Parent module name
    pub parent_id: Option<String>, // e.g., signal_id for reflex chunks
    pub line_range: (usize, usize),
    pub hash: String,         // For incremental indexing
}

pub enum ChunkType {
    Module,      // Entire module declaration
    Signal,      // Signal declaration + type/initialization
    Guard,       // Guard body
    Reflex,      // Reflex body + assignments
    Property,    // Property formula
}

pub trait MirrChunker: Send + Sync {
    async fn chunk_module(&self, module_src: &str, module_name: &str) 
        -> anyhow::Result<Vec<MirrChunk>>;
}

pub struct BoundaryChunker;
impl BoundaryChunker {
    pub async fn chunk_mirr_ast(&self, program: &MirrProgram) -> anyhow::Result<Vec<MirrChunk>> {
        // Walk AST, emit chunks at boundaries
        // Boundary priorities: Module > Signal > Guard > Reflex > Property
    }
}
```

**Rationale**: Chunking is 80% of RAG success. MIRR's native AST makes perfect chunk boundaries nearly free.

---

### Section B: SQLite FTS5 + sqlite-vec Storage Layer
**File**: `crates/mirr-kb-native/src/storage/sqlite_hybrid.rs` (new)

Implement local hybrid storage:

```rust
pub struct SqliteHybridStorage {
    db_path: PathBuf,
    embedding_dim: usize,  // 384 or 768
    fts5_table: String,    // "kb_chunks_fts"
    vec_table: String,     // "kb_embeddings"
    quantized: bool,       // 8-bit quantization if > 50k vectors
}

impl SqliteHybridStorage {
    pub async fn store_chunk(&mut self, chunk: &MirrChunk, embedding: &[f32]) 
        -> anyhow::Result<()> {
        // INSERT into both FTS5 (for BM25) and vec table (for similarity search)
    }

    pub async fn bm25_search(&self, query: &str, limit: usize) 
        -> anyhow::Result<Vec<BM25Result>> {
        // SELECT * FROM kb_chunks_fts WHERE kb_chunks_fts MATCH ? ORDER BY rank LIMIT ?
    }

    pub async fn vector_search(&self, query_embedding: &[f32], limit: usize) 
        -> anyhow::Result<Vec<VectorSearchResult>> {
        // SELECT * FROM kb_embeddings WHERE vec_distance(embedding, ?) < threshold ORDER BY distance LIMIT ?
    }

    pub async fn get_index_stats(&self) -> anyhow::Result<IndexStats> {
        // SELECT COUNT(*), AVG(LENGTH(text)), MAX(freshness_timestamp) FROM kb_chunks_fts
    }
}

pub struct BM25Result {
    pub chunk_id: String,
    pub chunk_text: String,
    pub rank: i32,  // FTS5 rank (negative)
}

pub struct VectorSearchResult {
    pub chunk_id: String,
    pub chunk_text: String,
    pub distance: f32,
    pub score: f32,  // normalized to 0-1
}
```

**Rationale**: Hybrid storage keeps everything local, deterministic, zero external dependencies for retrieval.

---

### Section C: Hybrid Search with RRF Fusion
**File**: `crates/mirr-kb-native/src/retrieval/hybrid_search.rs` (new)

Implement BM25 + vector fusion with Reciprocal Rank Fusion:

```rust
pub struct HybridSearcher {
    storage: Arc<SqliteHybridStorage>,
    embedder: Arc<dyn EmbeddingProvider>,
    k: usize,  // RRF parameter, typically 60
}

pub struct RRFScoreResult {
    pub chunk_id: String,
    pub chunk_text: String,
    pub bm25_rank: Option<usize>,
    pub vector_rank: Option<usize>,
    pub rrf_score: f32,  // 1/(k + rank_bm25) + 1/(k + rank_vector)
    pub appears_in_both: bool,
}

impl HybridSearcher {
    pub async fn search(&self, query: &str, limit: usize) 
        -> anyhow::Result<Vec<RRFScoreResult>> {
        // 1. Get embeddings for query (cloud API)
        // 2. Run BM25 search (local)
        // 3. Run vector search (local)
        // 4. Merge results with RRF scoring
        // 5. Sort by RRF score descending
        // 6. Return top `limit` results
    }

    fn compute_rrf_score(bm25_rank: Option<usize>, vector_rank: Option<usize>, k: usize) -> f32 {
        let mut score = 0.0;
        if let Some(rank) = bm25_rank {
            score += 1.0 / ((k + rank) as f32);
        }
        if let Some(rank) = vector_rank {
            score += 1.0 / ((k + rank) as f32);
        }
        score
    }
}
```

**Rationale**: RRF fusion gives massive boost to documents appearing in both BM25 and vector top-k. Research-backed (2026 best practice).

---

### Section D: FlashRank Reranker Integration
**File**: `crates/mirr-kb-native/src/reranking/flashrank.rs` (new)

Lightweight CPU reranker:

```rust
pub struct FlashRankReranker {
    model_path: PathBuf,  // ms-marco-MiniLM-L-12-v2, ~34MB
}

pub struct RerankedResult {
    pub chunk_id: String,
    pub chunk_text: String,
    pub rerank_score: f32,  // 0-1, cross-encoder logits normalized
    pub position: usize,     // new position after reranking
}

impl FlashRankReranker {
    pub async fn rerank(&self, query: &str, candidates: Vec<RRFScoreResult>, top_k: usize) 
        -> anyhow::Result<Vec<RerankedResult>> {
        // 1. Build (query, candidate) pairs
        // 2. Load MiniLM-L-12-v2 model (no Torch, CPU only)
        // 3. Batch inference (~20ms for 50 candidates on i5)
        // 4. Sort by cross-encoder score
        // 5. Return top `top_k` results (max 8 for context budget)
    }
}
```

**Rationale**: FlashRank is the only CPU-viable reranker for i5-class hardware. MiniLM-L-12-v2 has superior quality vs TinyBERT.

---

### Section E: Context Budget Enforcement
**File**: `crates/mirr-kb-native/src/context/budget.rs` (new)

Enforce context budget limits:

```rust
pub const MAX_CONTEXT_TOKENS: usize = 8000;
pub const MAX_CHUNKS_IN_CONTEXT: usize = 8;
pub const MAX_QUERY_BYTES: usize = 4096;

pub struct ContextBudget {
    available_tokens: usize,
    available_chunks: usize,
}

impl ContextBudget {
    pub fn try_add_chunk(&mut self, chunk_text: &str) -> bool {
        let chunk_tokens = estimate_token_count(chunk_text);
        if self.available_tokens >= chunk_tokens && self.available_chunks > 0 {
            self.available_tokens -= chunk_tokens;
            self.available_chunks -= 1;
            true
        } else {
            false
        }
    }

    pub fn remaining(&self) -> (usize, usize) {
        (self.available_tokens, self.available_chunks)
    }
}

fn estimate_token_count(text: &str) -> usize {
    // Rule of thumb: ~4 characters per token (conservatively)
    text.len() / 4 + 1
}
```

**Rationale**: Shorter precise context beats dumping 50K tokens. Enforce 4-8 chunk ceiling because faithfulness degrades above 8.

---

### Section F: Evaluation Framework Setup
**File**: `crates/mirr-kb-native/tests/golden_qa_eval.rs` (new)

Golden query/answer evaluation:

```rust
pub struct GoldenQAPair {
    pub query: String,
    pub expected_answer_chunks: Vec<String>,  // chunk IDs that should be retrieved
    pub context_difficulty: Difficulty,       // easy/medium/hard
}

pub enum Difficulty {
    Easy,     // single signal in single module
    Medium,   // cross-module signal reference
    Hard,     // property obligation spanning multiple modules
}

pub struct EvaluationResult {
    pub query: String,
    pub retrieved_chunks: Vec<String>,
    pub context_precision: f32,    // % retrieved chunks are relevant
    pub context_recall: f32,       // % of all relevant chunks retrieved
    pub faithfulness: f32,         // would LLM answer match expected answer?
    pub top_k_position: usize,
}

#[cfg(test)]
mod golden_qa_tests {
    use super::*;

    #[tokio::test]
    async fn evaluate_golden_qa_set() {
        let qa_pairs = load_golden_qa_set();
        let mut results = Vec::new();
        
        for pair in qa_pairs {
            let retrieved = run_hybrid_search(&pair.query, 20).await;
            let reranked = rerank(pair.query, retrieved, 8).await;
            
            let result = evaluate_retrieval(&pair, &reranked);
            results.push(result);
        }

        // Gate: context_precision >= 0.7, faithfulness >= 0.8
        assert!(results.iter().all(|r| r.context_precision >= 0.7), "context precision gate");
        assert!(results.iter().all(|r| r.faithfulness >= 0.8), "faithfulness gate");
    }
}
```

**Rationale**: Without evaluation, you cannot prove new RAG beats baseline. Golden QA pairs are the only objective metric.

---

### Section G: Dual-Run Telemetry Types
**File**: `crates/mirr-mcp-control-plane/src/server_rewrite/mrt_dispatch_dual_run_telemetry.rs` (new)

Capture both path execution:

```rust
pub struct DualRunTelemetry {
    pub request_id: String,
    pub tool_name: String,
    pub query: String,
    pub legacy_path: PathExecutionEvent,
    pub new_path: PathExecutionEvent,
    pub parity_metrics: ParityMetrics,
    pub timestamp_ms: u64,
}

pub struct PathExecutionEvent {
    pub path_name: String,
    pub success: bool,
    pub latency_ms: u64,
    pub result_count: usize,
    pub truncated: bool,
    pub error: Option<String>,
}

pub struct ParityMetrics {
    pub paths_match: bool,
    pub result_count_match: bool,
    pub result_count_diff: i32,
    pub truncation_match: bool,
    pub drift_category: DriftCategory,
    pub top_k_reordered: bool,
}

pub enum DriftCategory {
    NoDrift,
    MinorReordering,
    ResultCountMismatch,
    FreshnessMismatch,
    QualityDrift,
}
```

**Rationale**: Telemetry is how you know when it's safe to promote new path to default.

---

### Section H: Dual-Run Route Handler Integration
**File**: `crates/mirr-mcp-control-plane/src/server_rewrite/axum_route_host.rs` (modify)

Add dual-run branch to `dispatch_canonical_route`:

```rust
fn dispatch_canonical_route(
    state: &AxumMcpHostState,
    dispatch_input: &StdioRpcDispatchInput,
    tool_name: &str,
    api_key: Option<String>,
) -> (u16, StdioRpcResponse) {
    // Check if dual-run is enabled AND tool is mrt_kb_query
    if is_dual_run_enabled(&state.execution_config) && tool_name == "mrt_kb_query" {
        return execute_dual_run_query(state, dispatch_input, tool_name, api_key);
    }

    // Fall through to single-path execution (existing code)
    execute_single_path_query(state, dispatch_input, tool_name, api_key)
}

async fn execute_dual_run_query(
    state: &AxumMcpHostState,
    dispatch_input: &StdioRpcDispatchInput,
    tool_name: &str,
    api_key: Option<String>,
) -> (u16, StdioRpcResponse) {
    // 1. Spawn legacy (mrt_brain_get) and new (mrt_kb_query) tasks in parallel
    // 2. Use tokio::join! with per-path timeout (MAX_QUERY_TIME_MS = 5000)
    // 3. Capture telemetry for both paths
    // 4. Compare responses (DriftCategory)
    // 5. Return primary path result (new if ready; fall back to legacy if timeout)
    // 6. Log parity metrics to audit sink
}
```

**Rationale**: Parallel execution minimizes latency impact; timeout ensures fail-safe behavior.

---

### Section I: Unit Tests for Chunking
**File**: `crates/mirr-kb-native/tests/chunking_tests.rs` (new)

Test MIRR-native boundaries:

```rust
#[tokio::test]
async fn chunking_respects_module_boundary() {
    let src = "module test { signal x: u8; signal y: u16; }";
    let chunks = chunk_mirr_module(src).await.unwrap();
    assert_eq!(chunks.len(), 3); // module + x + y
    assert!(chunks[0].chunk_type == ChunkType::Module);
}

#[tokio::test]
async fn chunking_preserves_signal_guard_reflex_hierarchy() {
    let src = r#"
    module fsm {
        signal state: u3;
        guard g1 = state == 0;
        guard g2 = state == 1;
        reflex r1 { on g1 -> state = 1; }
    }
    "#;
    let chunks = chunk_mirr_module(src).await.unwrap();
    let reflex_chunks: Vec<_> = chunks.iter().filter(|c| c.chunk_type == ChunkType::Reflex).collect();
    assert!(!reflex_chunks.is_empty());
}
```

**Rationale**: Chunking quality directly impacts all downstream retrieval quality.

---

### Section J: Integration Test for Hybrid Search
**File**: `crates/mirr-kb-native/tests/hybrid_search_tests.rs` (new)

Test BM25 + vector + RRF:

```rust
#[tokio::test]
async fn hybrid_search_ranks_documents_consistently() {
    let searcher = setup_hybrid_searcher().await;
    let results = searcher.search("signal definition", 10).await.unwrap();
    
    // Verify RRF scoring is applied
    assert!(results[0].rrf_score > 0.0);
    assert!(results.iter().all(|r| r.rrf_score >= results.last().unwrap().rrf_score));
    
    // Verify documents in both BM25 and vector are top-ranked
    let both_searches = results.iter().filter(|r| r.appears_in_both).collect::<Vec<_>>();
    assert!(!both_searches.is_empty());
}

#[tokio::test]
async fn hybrid_search_degrades_gracefully_if_embedding_api_unavailable() {
    let searcher = setup_hybrid_searcher_no_embedding().await;
    let results = searcher.search("signal", 10).await;
    // Should fall back to BM25-only
    assert!(results.is_ok());
    assert!(!results.unwrap().is_empty());
}
```

**Rationale**: Hybrid search is mandatory for production RAG; fallback proves determinism.

---

### Section K: Integration Test for Full Dual-Run Flow
**File**: `crates/mirr-mcp-control-plane/tests/server_dual_run_integration_tests.rs` (new)

Test MCP endpoint with dual-run:

```rust
#[tokio::test]
async fn dual_run_captures_parity_telemetry() {
    let state = setup_dual_run_state().await;
    let request = StdioRpcDispatchInput {
        method: Some("mrt_kb_query".to_string()),
        params: Some(json!({ "query": "signal test" })),
        ..Default::default()
    };
    
    let (status, response) = dispatch_host_rpc_message(&state, &HeaderMap::new(), request);
    assert_eq!(status, 200);
    
    // Verify telemetry was captured
    let telemetry = state.audit_event_sink.last_dual_run_telemetry();
    assert!(telemetry.is_some());
    let telem = telemetry.unwrap();
    assert_eq!(telem.tool_name, "mrt_kb_query");
}

#[tokio::test]
async fn dual_run_disabled_returns_single_path() {
    let mut state = setup_state();
    state.execution_config.dual_run_enabled = false;
    
    let request = StdioRpcDispatchInput {
        method: Some("mrt_kb_query".to_string()),
        params: Some(json!({ "query": "test" })),
        ..Default::default()
    };
    
    let (status, _) = dispatch_host_rpc_message(&state, &HeaderMap::new(), request);
    assert_eq!(status, 200);
}
```

**Rationale**: End-to-end validation of dual-run through HTTP handler.

---

### Section L: Configuration and Feature Flags
**File**: `crates/mirr-kb-native/src/config.rs` (modify existing)

Add new configuration:

```rust
pub struct RagConfig {
    pub kb_root: PathBuf,
    pub embedding_api_key: Option<String>,
    pub embedding_provider: EmbeddingProvider,
    pub embedding_dimensions: usize,  // 384, 768, or 1536
    pub sqlite_db_path: PathBuf,
    pub enable_quantization: bool,    // 8-bit quantization for > 50k vectors
    pub hybrid_search_enabled: bool,  // BM25 + vector + RRF
    pub reranking_enabled: bool,      // FlashRank reranking
    pub context_budget_bytes: usize,  // 8000 tokens max
    pub max_chunks_in_response: usize, // 8 max
    pub dual_run_enabled: bool,        // parallel legacy + new path execution
}

impl RagConfig {
    pub fn from_env() -> Result<Self, RagConfigError> {
        Ok(Self {
            kb_root: std::env::var("MIRR_KB_ROOT")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./kb")),
            embedding_api_key: std::env::var("MIRR_EMBEDDING_API_KEY").ok(),
            embedding_provider: parse_provider_env()?,
            embedding_dimensions: parse_dims_env().unwrap_or(768),
            sqlite_db_path: std::env::var("MIRR_KB_DB")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./kb/kb.db")),
            enable_quantization: std::env::var("MIRR_KB_QUANTIZE").is_ok(),
            hybrid_search_enabled: true,   // default ON
            reranking_enabled: true,       // default ON
            context_budget_bytes: 8000,
            max_chunks_in_response: 8,
            dual_run_enabled: std::env::var("MIRR_KB_DUALRUN").is_ok(),
        })
    }
}
```

**Rationale**: Feature-gated configuration; can enable/disable components independently.

---

## Part 5: Execution Plan

| Step | Section | Files | Depends on |
|------|---------|-------|-----------|
| 1 | A | mirr-kb-native/src/chunking/mirr_boundaries.rs (new) | — |
| 2 | B | mirr-kb-native/src/storage/sqlite_hybrid.rs (new) | Step 1 |
| 3 | C | mirr-kb-native/src/retrieval/hybrid_search.rs (new) | Step 2 |
| 4 | D | mirr-kb-native/src/reranking/flashrank.rs (new) | Step 3 |
| 5 | E | mirr-kb-native/src/context/budget.rs (new) | — |
| 6 | F | mirr-kb-native/tests/golden_qa_eval.rs (new) | Step 4 |
| 7 | G | mrt_dispatch_dual_run_telemetry.rs (new) | — |
| 8 | H | axum_route_host.rs (modify) | Step 7 |
| 9 | L | config.rs (modify) | Steps 1-4 |
| 10 | I | chunking_tests.rs (new) | Steps 1, 2 |
| 11 | J | hybrid_search_tests.rs (new) | Steps 3, 4 |
| 12 | K | server_dual_run_integration_tests.rs (new) | Steps 8, 10 |

---

## Part 6: Breakage Map

| Step | What breaks | Why | Fixed in |
|------|------------|-----|----------|
| 9 | RagConfig::from_env() signature changes | Added new config fields | Callers updated in Steps 9a-9c |
| — | No other breakage expected | All changes additive or feature-gated | — |

---

## Part 7: Wave Plan

| Wave | Steps | Parallelizable | Dependent | Breakage |
|------|-------|-----------------|-----------|----------|
| 1 | 1, 5, 7 | Yes — different files | — | No |
| 2 | 2, 3, 4 | Yes — storage pipeline | Wave 1 | No |
| 3 | 6, 9 | Yes — config + eval setup | Wave 2 | No |
| 4 | 8 | Sequential | Waves 1-3 | No (feature-gated) |
| 5 | 10, 11, 12 | Yes — test files | Waves 1-4 | No |

---

## Part 8: File Manifest

### New Files (9)
| File | Purpose |
|------|---------|
| crates/mirr-kb-native/src/chunking/mirr_boundaries.rs | MIRR-native chunk boundary detection |
| crates/mirr-kb-native/src/storage/sqlite_hybrid.rs | SQLite FTS5 + sqlite-vec storage |
| crates/mirr-kb-native/src/retrieval/hybrid_search.rs | BM25 + vector + RRF fusion |
| crates/mirr-kb-native/src/reranking/flashrank.rs | FlashRank cross-encoder integration |
| crates/mirr-kb-native/src/context/budget.rs | Context budget enforcement |
| crates/mirr-kb-native/tests/golden_qa_eval.rs | Golden QA evaluation framework |
| crates/mirr-mcp-control-plane/src/server_rewrite/mrt_dispatch_dual_run_telemetry.rs | Dual-run telemetry capture |
| crates/mirr-kb-native/tests/chunking_tests.rs | Chunking unit tests |
| crates/mirr-kb-native/tests/hybrid_search_tests.rs | Hybrid search + fallback tests |
| crates/mirr-mcp-control-plane/tests/server_dual_run_integration_tests.rs | Integration tests for dual-run |

### Modified Files (3)
| File | Change summary |
|------|----------------|
| crates/mirr-kb-native/src/config.rs | Add embeddings config, dual-run flag, context budget limits |
| crates/mirr-kb-native/src/lib.rs | Export new modules: chunking, storage, reranking, context, evaluation |
| crates/mirr-mcp-control-plane/src/server_rewrite/axum_route_host.rs | Add execute_dual_run_query branch to dispatch_canonical_route |

---

## Part 9: Verification Gates

```bash
# 1. Compile all new modules
cargo check --all-targets

# 2. Chunking unit tests
cargo test -p mirr-kb-native --test chunking_tests

# 3. Hybrid search unit tests (deterministic, no embedding API)
cargo test -p mirr-kb-native --test hybrid_search_tests

# 4. Golden QA evaluation
cargo test -p mirr-kb-native --test golden_qa_eval

# 5. Dual-run integration tests
cargo test -p mirror --test server_dual_run_integration_tests

# 6. Existing parity tests still pass (no regression)
cargo test -p mirror --test server_mrt_dispatch_invocation_resolver_parity
cargo test -p mirror --test manifest_mrt_tool_parity
cargo test -p mirror --test manifest_input_schema_parity

# 7. Formatting and clippy clean
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings

# 8. Full workspace
cargo nextest run --workspace --no-fail-fast
```

---

## Part 10: Phase Dependencies and Cutover Gates

**Approval gates before Phase 4**:
- ✅ Chunking strategy validated with unit tests
- ✅ Hybrid search (BM25+vector+RRF) tested with deterministic fallback working
- ✅ Reranking latency < 20ms for 50 candidates on i5-class hardware
- ✅ Context budget enforcement prevents > 8 chunks or > 8K tokens
- ✅ Dual-run telemetry captures parity metrics accurately
- ⚠️ Golden QA evaluation set (20-30 pairs) created and baseline metrics recorded
- ⚠️ Target metrics: context_precision >= 0.7, faithfulness >= 0.8

**Phase 4 Start Gates** (after Phase 3):
- All above gates pass
- Golden QA evaluation shows new path outperforms baseline on 80%+ of queries
- Dual-run telemetry is stable (parity drift < 10% on result rankings)
- Index lifecycle (Phase 4) is ready for implementation

---

## Part 11: Risk Mitigation Summary

| Risk | Mitigation | Owner |
|------|-----------|-------|
| Chunking quality | Unit tests + golden QA validation | Implementation team |
| Embedding API latency | Caching + timeout with lexical fallback | mirr-kb-native crate |
| Vector quantization accuracy | Start unquantized; measure degradation if needed | Storage team |
| RRF weighting (k=60) tuning | Configurable k value + evaluation metrics | Evaluation team |
| FlashRank latency on CPU | Profile on i5; batch rerank if needed | Reranking team |
| Golden QA evaluation bias | Diverse query types; manual review | Evaluation team |
| Dual-run latency | Parallel execution with timeout + fallback | axum route host |
| Response structure parity | Snapshot testing + structured comparison | Telemetry team |

---

## Part 12: Success Criteria

✅ **Code Quality**:
- All new modules have `#![forbid(unsafe_code)]` and `#![deny(warnings)]`
- Clippy passes on all-targets with -D warnings
- Zero doc warnings

✅ **Functional**:
- Chunking produces valid MIRR-native boundaries
- Hybrid search runs both BM25 and vector in parallel
- RRF fusion correctly ranks both-path documents highest
- Reranking reduces candidate list from 20-50 → 5-10 within 20ms
- Context budget enforcer prevents > 8 chunks or > 8K tokens
- Dual-run executes both paths in parallel, captures telemetry
- Lexical fallback works deterministically when embedding API unavailable

✅ **Testing**:
- 4 new unit test files (chunking, hybrid search, golden QA, dual-run integration)
- All net-new tests pass
- Existing parity tests still pass
- Golden QA evaluation shows target metrics (context_precision >= 0.7, faithfulness >= 0.8)

✅ **Performance**:
- Hybrid search latency < 2s (combined BM25 + vector + RRF)
- Reranking latency < 20ms for 50 candidates on i5
- Dual-run latency < 2× single-path (parallel execution + timeout)

---

## Recommendation

**SIGN and execute Phase 3 immediately.** This is the architectural foundation for all cutover decisions in Phases 4-6. Without chunking, hybrid search, reranking, and evaluation, there is no objective way to prove the new RAG is better than the legacy baseline.

