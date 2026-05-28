#![forbid(unsafe_code)]
#![deny(warnings)]

//! Integration test suite implementing Phase B (RAG Semantic Proof Integration Suite)
//! of the massive test suite expansion plan.
//!
//! Validates:
//! 1. Semantic query matching and vector search retrieval precision for guard precedents.
//! 2. Exact metadata filtering for target modules and chunk categories.
//! 3. Database integrity checks (upsert overrides, empty queries, synonym expansion).

use async_trait::async_trait;
use mirr_kb_native::adapters::embedding::EmbeddingProvider;
use mirr_kb_native::chunking::{ChunkType, MirrChunk};
use mirr_kb_native::expansion::ExpansionMode;
use mirr_kb_native::query_handler::{run_query_pipeline, QueryPipelineRequest};
use mirr_kb_native::retrieval::SearchMode;
use mirr_kb_native::storage::SqliteHybridStorage;
use std::path::PathBuf;
use std::sync::Arc;

struct ProofFixedEmbedder;

#[async_trait]
impl EmbeddingProvider for ProofFixedEmbedder {
    async fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        // High similarity vector assignment based on vocabulary
        Ok(if text.contains("alu") || text.contains("arithmetic") {
            vec![1.0, 0.0, 0.0]
        } else if text.contains("ram") || text.contains("memory") {
            vec![0.0, 1.0, 0.0]
        } else if text.contains("noc") || text.contains("bus") {
            vec![0.0, 0.0, 1.0]
        } else {
            vec![0.5, 0.5, 0.5]
        })
    }
}

fn temp_db_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("mirr-rag-proof-{}-{}.db", name, std::process::id()));
    path
}

fn make_proof_chunk(id: &str, module: &str, kind: ChunkType, text: &str) -> MirrChunk {
    MirrChunk::new(id.to_string(), kind, text.to_string(), module.to_string(), None, (1, 5))
}

/// Test retrieving a guard precedent based on vector semantic similarity.
#[tokio::test]
async fn test_proof_retrieval_by_guard_semantic_similarity() {
    let storage = Arc::new(
        SqliteHybridStorage::from_db_path(temp_db_path("similarity"))
            .expect("storage initialization failed"),
    );

    // Upsert different hardware proof precedents
    storage
        .upsert_chunk(
            &make_proof_chunk(
                "alu.proof",
                "alu",
                ChunkType::Guard,
                "ALU addition overflow guard proof precedent",
            ),
            "src/alu.mirr",
            Some(&[1.0, 0.0, 0.0]),
        )
        .expect("insert alu");
    storage
        .upsert_chunk(
            &make_proof_chunk(
                "ram.proof",
                "ram",
                ChunkType::Guard,
                "RAM memory boundary boundary read proof precedent",
            ),
            "src/ram.mirr",
            Some(&[0.0, 1.0, 0.0]),
        )
        .expect("insert ram");

    let request = QueryPipelineRequest {
        text: "memory read boundary".to_string(),
        mode: SearchMode::Semantic,
        limit: 2,
        ..QueryPipelineRequest::default()
    };

    let response =
        run_query_pipeline(storage, &ProofFixedEmbedder, request).await.expect("query failed");

    assert!(!response.results.is_empty());
    // Most similar should be RAM because "memory" embedding maps to [0, 1, 0]
    assert_eq!(response.results[0].key, "ram.proof");
}

/// Test filtering vector search results strictly to a specific Module or ChunkType.
#[tokio::test]
async fn test_proof_filtering_by_chunk_type_isolation() {
    let storage = Arc::new(
        SqliteHybridStorage::from_db_path(temp_db_path("filter"))
            .expect("storage initialization failed"),
    );

    storage
        .upsert_chunk(
            &make_proof_chunk(
                "noc.guard",
                "noc",
                ChunkType::Guard,
                "NOC priority bus collision guard precedent",
            ),
            "src/noc.mirr",
            Some(&[0.0, 0.0, 1.0]),
        )
        .expect("insert guard");
    storage
        .upsert_chunk(
            &make_proof_chunk(
                "noc.reflex",
                "noc",
                ChunkType::Reflex,
                "NOC priority bus routing reflex logic",
            ),
            "src/noc.mirr",
            Some(&[0.0, 0.0, 1.0]),
        )
        .expect("insert reflex");

    // Query for NOC, but filter strictly for ChunkType:Guard
    let request = QueryPipelineRequest {
        text: "bus priority".to_string(),
        mode: SearchMode::Hybrid,
        filter: Some("chunk_type:Guard module:noc".to_string()),
        limit: 10,
        ..QueryPipelineRequest::default()
    };

    let response =
        run_query_pipeline(storage, &ProofFixedEmbedder, request).await.expect("query failed");

    assert_eq!(response.results.len(), 1);
    assert_eq!(response.results[0].key, "noc.guard");
}

/// Test synonym query expansion routing rules.
#[tokio::test]
async fn test_proof_retrieval_synonym_routing() {
    let storage = Arc::new(
        SqliteHybridStorage::from_db_path(temp_db_path("synonym"))
            .expect("storage initialization failed"),
    );

    storage
        .upsert_chunk(
            &make_proof_chunk(
                "noc.router",
                "noc",
                ChunkType::Module,
                "noc_router topology interconnection bus mapping",
            ),
            "src/noc.mirr",
            Some(&[0.0, 0.0, 1.0]),
        )
        .expect("insert router");

    let request = QueryPipelineRequest {
        text: "interconnection bus".to_string(),
        mode: SearchMode::Hybrid,
        expansion_mode: ExpansionMode::Synonym,
        limit: 4,
        ..QueryPipelineRequest::default()
    };

    let response =
        run_query_pipeline(storage, &ProofFixedEmbedder, request).await.expect("query failed");

    assert!(!response.results.is_empty());
    assert_eq!(response.results[0].key, "noc.router");
}

/// Test duplicate ID upserts overwrite rather than introducing duplicate rows.
#[tokio::test]
async fn test_proof_duplicate_upsert_overwrite() {
    let storage = Arc::new(
        SqliteHybridStorage::from_db_path(temp_db_path("upsert-overwrite"))
            .expect("storage initialization failed"),
    );

    storage
        .upsert_chunk(
            &make_proof_chunk(
                "temp.id",
                "temp",
                ChunkType::Signal,
                "Original signal text description",
            ),
            "src/temp.mirr",
            None,
        )
        .expect("insert original");

    // Overwrite the same chunk ID
    storage
        .upsert_chunk(
            &make_proof_chunk(
                "temp.id",
                "temp",
                ChunkType::Signal,
                "Updated signal text description",
            ),
            "src/temp.mirr",
            None,
        )
        .expect("insert overwrite");

    let request = QueryPipelineRequest {
        text: "description".to_string(),
        mode: SearchMode::Lexical,
        ..QueryPipelineRequest::default()
    };

    let response =
        run_query_pipeline(storage, &ProofFixedEmbedder, request).await.expect("query failed");

    assert_eq!(response.results.len(), 1);
    assert!(response.results[0].snippet.contains("Updated"));
}

/// Test that empty query requests are handled safely.
#[tokio::test]
async fn test_proof_empty_query_fallback() {
    let storage = Arc::new(
        SqliteHybridStorage::from_db_path(temp_db_path("empty_query"))
            .expect("storage initialization failed"),
    );

    storage
        .upsert_chunk(
            &make_proof_chunk(
                "alu.proof",
                "alu",
                ChunkType::Guard,
                "ALU addition overflow guard proof precedent",
            ),
            "src/alu.mirr",
            None,
        )
        .expect("insert alu");

    let request = QueryPipelineRequest {
        text: "".to_string(),
        mode: SearchMode::Semantic,
        ..QueryPipelineRequest::default()
    };

    let response =
        run_query_pipeline(storage, &ProofFixedEmbedder, request).await.expect("query failed");

    // Empty query matches nothing or returns default lexical output without erroring
    assert!(response.results.is_empty());
}
