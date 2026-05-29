use async_trait::async_trait;
use mirr_kb_native::adapters::embedding::EmbeddingProvider;
use mirr_kb_native::chunking::{ChunkType, MirrChunk};
use mirr_kb_native::expansion::ExpansionMode;
use mirr_kb_native::query_handler::{run_query_pipeline, QueryPipelineRequest};
use mirr_kb_native::retrieval::SearchMode;
use mirr_kb_native::storage::SqliteHybridStorage;
use std::path::PathBuf;
use std::sync::Arc;

struct FixedEmbedder;
struct FailingEmbedder;

#[async_trait]
impl EmbeddingProvider for FixedEmbedder {
    async fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        Ok(if text.contains("beta") { vec![0.0, 1.0] } else { vec![1.0, 0.0] })
    }
}

#[async_trait]
impl EmbeddingProvider for FailingEmbedder {
    async fn embed(&self, _text: &str) -> anyhow::Result<Vec<f32>> {
        Err(anyhow::anyhow!("embedding unavailable"))
    }
}

fn temp_db_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("mirr-phase4-search-{}-{}.db", name, std::process::id()));
    path
}

fn chunk(id: &str, module: &str, kind: ChunkType, text: &str) -> MirrChunk {
    MirrChunk::new(id.to_string(), kind, text.to_string(), module.to_string(), None, (1, 2))
}

#[tokio::test]
async fn graph_mode_returns_dependency_related_chunks() {
    let storage =
        Arc::new(SqliteHybridStorage::from_db_path(temp_db_path("graph")).expect("storage"));
    storage
        .upsert_chunk(
            &chunk("alpha.main", "alpha", ChunkType::Module, "depends on beta.module"),
            "src/alpha.mirr",
            None,
        )
        .expect("insert alpha");
    storage
        .upsert_chunk(
            &chunk("beta.module", "beta", ChunkType::Module, "signal beta_out: u8;"),
            "src/beta.mirr",
            None,
        )
        .expect("insert beta");

    let request = QueryPipelineRequest {
        text: "beta".to_string(),
        mode: SearchMode::Graph,
        limit: 8,
        filter: Some("beta".to_string()),
        ..QueryPipelineRequest::default()
    };

    let response = run_query_pipeline(storage, &FixedEmbedder, request).await.expect("query");
    assert!(!response.results.is_empty());
}

#[tokio::test]
async fn temporal_mode_honors_time_window() {
    let storage =
        Arc::new(SqliteHybridStorage::from_db_path(temp_db_path("temporal")).expect("storage"));
    storage
        .upsert_chunk(
            &chunk("demo.signal", "demo", ChunkType::Signal, "signal x: u8;"),
            "src/demo.mirr",
            None,
        )
        .expect("insert");

    let now =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("time").as_secs();

    let request = QueryPipelineRequest {
        text: "signal".to_string(),
        mode: SearchMode::Temporal,
        temporal_start_secs: Some(now.saturating_sub(5)),
        temporal_end_secs: Some(now.saturating_add(5)),
        ..QueryPipelineRequest::default()
    };

    let response = run_query_pipeline(storage, &FixedEmbedder, request).await.expect("query");
    assert_eq!(response.results.len(), 1);
}

#[tokio::test]
async fn hybrid_mode_supports_synonym_expansion_and_filtering() {
    let storage =
        Arc::new(SqliteHybridStorage::from_db_path(temp_db_path("hybrid")).expect("storage"));
    storage
        .upsert_chunk(
            &chunk("demo.signal", "demo", ChunkType::Signal, "signal alpha: u8;"),
            "src/demo.mirr",
            Some(&[1.0, 0.0]),
        )
        .expect("insert signal");
    storage
        .upsert_chunk(
            &chunk("demo.guard", "demo", ChunkType::Guard, "on alpha > 0"),
            "src/demo.mirr",
            Some(&[1.0, 0.0]),
        )
        .expect("insert guard");

    let request = QueryPipelineRequest {
        text: "signal alpha".to_string(),
        mode: SearchMode::Hybrid,
        limit: 16,
        filter: Some("chunk_type:Signal module:demo".to_string()),
        expansion_mode: ExpansionMode::Synonym,
        retry_count: 2,
        timeout_ms: 10_000,
        ..QueryPipelineRequest::default()
    };

    let response = run_query_pipeline(storage, &FixedEmbedder, request).await.expect("query");
    assert_eq!(response.results.len(), 1);
    assert!(response.results[0].key.contains("signal"));
}

#[tokio::test]
async fn semantic_mode_falls_back_to_lexical_on_embedding_failure() {
    let storage = Arc::new(
        SqliteHybridStorage::from_db_path(temp_db_path("semantic-fallback")).expect("storage"),
    );
    storage
        .upsert_chunk(
            &chunk("demo.signal", "demo", ChunkType::Signal, "signal alpha: u8;"),
            "src/demo.mirr",
            None,
        )
        .expect("insert signal");

    let request = QueryPipelineRequest {
        text: "signal alpha".to_string(),
        mode: SearchMode::Semantic,
        ..QueryPipelineRequest::default()
    };

    let response = run_query_pipeline(storage, &FailingEmbedder, request).await.expect("query");
    assert!(!response.results.is_empty());
}
