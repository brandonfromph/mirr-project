#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::chunking::{ChunkType, MirrChunk};
use crate::storage::SqliteHybridStorage;
use crate::storage::{ChunkHit, IndexStats};
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

/// Trait for vector store backends.
/// Local-first implementations are preferred for MIRR.
#[async_trait]
pub trait VectorStore: Send + Sync {
    /// Insert a vector with associated metadata.
    async fn insert(
        &self,
        key: &str,
        vector: Vec<f32>,
        metadata: serde_json::Value,
    ) -> anyhow::Result<()>;

    /// Search for vectors similar to the query vector.
    async fn search(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> anyhow::Result<Vec<crate::retrieval::SearchResult>>;
}

/// Local SQLite-backed vector store wrapper.
/// Uses the same storage file as the hybrid retrieval pipeline.
#[derive(Clone)]
pub struct SqliteVectorStore {
    storage: Arc<SqliteHybridStorage>,
}

impl SqliteVectorStore {
    pub fn new(kb_root: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self { storage: Arc::new(SqliteHybridStorage::new(kb_root)?) })
    }

    pub fn from_storage(storage: Arc<SqliteHybridStorage>) -> Self {
        Self { storage }
    }

    pub fn storage(&self) -> &Arc<SqliteHybridStorage> {
        &self.storage
    }

    pub fn index_stats(&self) -> anyhow::Result<IndexStats> {
        self.storage.index_stats()
    }
}

#[async_trait]
impl VectorStore for SqliteVectorStore {
    async fn insert(
        &self,
        key: &str,
        vector: Vec<f32>,
        metadata: serde_json::Value,
    ) -> anyhow::Result<()> {
        let text = metadata.get("text").and_then(|value| value.as_str()).unwrap_or(key).to_string();
        let source = metadata
            .get("source")
            .and_then(|value| value.as_str())
            .unwrap_or("vector_store")
            .to_string();
        let module = metadata
            .get("module")
            .and_then(|value| value.as_str())
            .unwrap_or("vector_store")
            .to_string();
        let chunk = MirrChunk::new(key.to_string(), ChunkType::Module, text, module, None, (0, 0));
        self.storage.upsert_chunk(&chunk, &source, Some(vector.as_slice()))
    }

    async fn search(
        &self,
        query_vector: Vec<f32>,
        limit: usize,
    ) -> anyhow::Result<Vec<crate::retrieval::SearchResult>> {
        let hits: Vec<ChunkHit> = self.storage.vector_search(&query_vector, limit, None)?;
        Ok(hits
            .into_iter()
            .map(|hit| crate::retrieval::SearchResult {
                key: hit.key,
                title: hit.module,
                snippet: hit.text,
                score: hit.score,
                source: hit.source,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_db_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("mirr-vector-{}-{}.db", name, std::process::id()));
        path
    }

    #[tokio::test]
    async fn sqlite_vector_store_inserts_and_searches() {
        let store = SqliteVectorStore::new(temp_db_path("search")).expect("store");
        store
            .insert(
                "demo.vector",
                vec![1.0, 0.0],
                serde_json::json!({"text": "alpha signal", "source": "src/demo.mirr", "module": "demo"}),
            )
            .await
            .expect("insert");

        let results = store.search(vec![1.0, 0.0], 10).await.expect("search");
        assert_eq!(results[0].key, "demo.vector");
    }
}
