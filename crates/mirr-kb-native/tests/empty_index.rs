#![forbid(unsafe_code)]
#![deny(warnings)]

use mirr_kb_native::adapters::embedding::StubEmbeddingProvider;
use mirr_kb_native::query_handler::{run_query_pipeline, QueryPipelineRequest};
use mirr_kb_native::storage::SqliteHybridStorage;
use std::sync::Arc;

#[tokio::test]
async fn query_pipeline_empty_index_returns_empty_results_and_success() {
    let storage = Arc::new(
        SqliteHybridStorage::from_db_path(std::env::temp_dir().join("mirr-kb-empty-index-test.db"))
            .expect("storage"),
    );
    // No chunks inserted: index is empty.
    let request =
        QueryPipelineRequest { text: "alpha".to_string(), ..QueryPipelineRequest::default() };
    let response =
        run_query_pipeline(storage, &StubEmbeddingProvider, request).await.expect("query");
    assert!(response.results.is_empty(), "Expected empty results for empty index");
    // Should not error, and should not panic.
}
