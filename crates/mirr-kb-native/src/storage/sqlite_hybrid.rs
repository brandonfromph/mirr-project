#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::chunking::MirrChunk;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_RESULTS_DEFAULT: usize = 1000;
const FRESHNESS_WINDOW_SECS: u64 = 3600;
const EMBEDDING_PRECISION: usize = 6;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkHit {
    pub key: String,
    pub module: String,
    pub chunk_type: String,
    pub text: String,
    pub source: String,
    pub score: f32,
    pub freshness_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub indexed_count: usize,
    pub last_refresh_secs: u64,
    pub is_stale: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SqliteHybridStorage {
    db_path: PathBuf,
}

impl SqliteHybridStorage {
    pub fn new(kb_root: impl AsRef<Path>) -> anyhow::Result<Self> {
        let db_path = kb_root.as_ref().join("graph.db");
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let storage = Self { db_path };
        storage.initialize_schema()?;
        Ok(storage)
    }

    pub fn from_db_path(db_path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let db_path = db_path.as_ref().to_path_buf();
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let storage = Self { db_path };
        storage.initialize_schema()?;
        Ok(storage)
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    fn connection(&self) -> anyhow::Result<rusqlite::Connection> {
        Ok(rusqlite::Connection::open(&self.db_path)?)
    }

    fn initialize_schema(&self) -> anyhow::Result<()> {
        let conn = self.connection()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS kb_chunks (
                key TEXT PRIMARY KEY,
                module TEXT NOT NULL,
                chunk_type TEXT NOT NULL,
                text TEXT NOT NULL,
                source TEXT NOT NULL,
                parent_id TEXT,
                line_start INTEGER NOT NULL,
                line_end INTEGER NOT NULL,
                hash TEXT NOT NULL,
                estimated_tokens INTEGER NOT NULL,
                freshness_secs INTEGER NOT NULL,
                embedding_json TEXT
            );
            CREATE VIRTUAL TABLE IF NOT EXISTS kb_chunks_fts USING fts5(
                key,
                module,
                chunk_type,
                text,
                source,
                tokenize = 'unicode61'
            );
            CREATE TABLE IF NOT EXISTS kb_vectors (
                key TEXT PRIMARY KEY,
                vector_json TEXT NOT NULL,
                updated_secs INTEGER NOT NULL
            );",
        )?;
        Ok(())
    }

    pub fn upsert_chunk(
        &self,
        chunk: &MirrChunk,
        source: &str,
        embedding: Option<&[f32]>,
    ) -> anyhow::Result<()> {
        let conn = self.connection()?;
        let freshness_secs = unix_now_secs();
        let embedding_json = embedding.map(serialize_embedding).transpose()?;
        let embedding_json_for_db = embedding_json.clone();

        conn.execute(
            "INSERT INTO kb_chunks (
                key, module, chunk_type, text, source, parent_id, line_start, line_end, hash, estimated_tokens, freshness_secs, embedding_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ON CONFLICT(key) DO UPDATE SET
                module = excluded.module,
                chunk_type = excluded.chunk_type,
                text = excluded.text,
                source = excluded.source,
                parent_id = excluded.parent_id,
                line_start = excluded.line_start,
                line_end = excluded.line_end,
                hash = excluded.hash,
                estimated_tokens = excluded.estimated_tokens,
                freshness_secs = excluded.freshness_secs,
                embedding_json = excluded.embedding_json",
            rusqlite::params![
                chunk.id,
                chunk.module,
                format!("{:?}", chunk.chunk_type),
                chunk.text,
                source,
                chunk.parent_id,
                chunk.line_range.0 as i64,
                chunk.line_range.1 as i64,
                chunk.hash,
                chunk.estimated_tokens as i64,
                freshness_secs as i64,
                embedding_json_for_db,
            ],
        )?;

        conn.execute("DELETE FROM kb_chunks_fts WHERE key = ?1", rusqlite::params![chunk.id])?;
        conn.execute(
            "INSERT INTO kb_chunks_fts (key, module, chunk_type, text, source) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![
                chunk.id,
                chunk.module,
                format!("{:?}", chunk.chunk_type),
                chunk.text,
                source,
            ],
        )?;

        if let Some(vector_json) = embedding_json {
            conn.execute(
                "INSERT INTO kb_vectors (key, vector_json, updated_secs) VALUES (?1, ?2, ?3)
                 ON CONFLICT(key) DO UPDATE SET vector_json = excluded.vector_json, updated_secs = excluded.updated_secs",
                rusqlite::params![chunk.id, vector_json, freshness_secs as i64],
            )?;
        }

        Ok(())
    }

    pub fn bm25_search(
        &self,
        query: &str,
        limit: usize,
        filter: Option<&str>,
    ) -> anyhow::Result<Vec<ChunkHit>> {
        let conn = self.connection()?;
        let fts_query = build_fts_query(query, filter);
        if fts_query.is_empty() {
            return self.fallback_lexical_scan(query, limit, filter);
        }

        let mut hits = Vec::new();
        let limit = limit.min(MAX_RESULTS_DEFAULT) as i64;
        let sql = "SELECT c.key, c.module, c.chunk_type, c.text, c.source, c.freshness_secs, bm25(kb_chunks_fts) AS rank
               FROM kb_chunks_fts
               INNER JOIN kb_chunks c ON c.key = kb_chunks_fts.key
               WHERE kb_chunks_fts MATCH ?1
               ORDER BY rank ASC
               LIMIT ?2";
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(rusqlite::params![fts_query, limit], |row| {
            Ok(ChunkHit {
                key: row.get(0)?,
                module: row.get(1)?,
                chunk_type: row.get(2)?,
                text: row.get(3)?,
                source: row.get(4)?,
                freshness_secs: row.get::<_, i64>(5)? as u64,
                score: normalize_bm25(row.get::<_, f32>(6)?),
            })
        })?;

        for row in rows {
            let hit = row?;
            if let Some(filter_value) = filter {
                if !matches_filter(
                    &hit.key,
                    &hit.module,
                    &hit.source,
                    &hit.chunk_type,
                    &hit.text,
                    filter_value,
                ) {
                    continue;
                }
            }
            hits.push(hit);
        }

        if hits.is_empty() {
            return self.fallback_lexical_scan(query, limit as usize, filter);
        }

        Ok(hits)
    }

    pub fn vector_search(
        &self,
        query_embedding: &[f32],
        limit: usize,
        filter: Option<&str>,
    ) -> anyhow::Result<Vec<ChunkHit>> {
        let conn = self.connection()?;
        let limit = limit.min(MAX_RESULTS_DEFAULT);

        let mut hits = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT c.key, c.module, c.chunk_type, c.text, c.source, c.freshness_secs, c.embedding_json
             FROM kb_chunks c
             INNER JOIN kb_vectors v ON v.key = c.key",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)? as u64,
                row.get::<_, String>(6)?,
            ))
        })?;

        for row in rows {
            let (key, module, chunk_type, text, source, freshness_secs, vector_json) = row?;
            if let Some(filter_value) = filter {
                if !matches_filter(&key, &module, &source, &chunk_type, &text, filter_value) {
                    continue;
                }
            }
            let candidate = deserialize_embedding(&vector_json)?;
            let score = cosine_similarity(query_embedding, &candidate);
            let distance = 1.0 - score;
            hits.push(ChunkHit {
                key,
                module,
                chunk_type,
                text,
                source,
                freshness_secs,
                score: normalize_similarity(score),
            });
            // distance kept as an internal derived value to make the ordering explicit in code below.
            let _ = distance;
        }

        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
        hits.truncate(limit);
        Ok(hits)
    }

    pub fn index_stats(&self) -> anyhow::Result<IndexStats> {
        let conn = self.connection()?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM kb_chunks", [], |row| row.get(0))?;
        let last_refresh_secs: i64 =
            conn.query_row("SELECT COALESCE(MAX(freshness_secs), 0) FROM kb_chunks", [], |row| {
                row.get(0)
            })?;
        let is_stale = if last_refresh_secs <= 0 {
            true
        } else {
            unix_now_secs().saturating_sub(last_refresh_secs as u64) > FRESHNESS_WINDOW_SECS
        };

        Ok(IndexStats {
            indexed_count: count as usize,
            last_refresh_secs: last_refresh_secs as u64,
            is_stale,
            error: None,
        })
    }

    pub fn filter_by_chunk_type(
        &self,
        chunk_type: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<ChunkHit>> {
        let conn = self.connection()?;
        let mut hits = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT key, module, chunk_type, text, source, freshness_secs
             FROM kb_chunks
             WHERE LOWER(chunk_type) = LOWER(?1)
             ORDER BY freshness_secs DESC
             LIMIT ?2",
        )?;
        let rows = stmt.query_map(
            rusqlite::params![chunk_type, limit.min(MAX_RESULTS_DEFAULT) as i64],
            |row| {
                Ok(ChunkHit {
                    key: row.get(0)?,
                    module: row.get(1)?,
                    chunk_type: row.get(2)?,
                    text: row.get(3)?,
                    source: row.get(4)?,
                    freshness_secs: row.get::<_, i64>(5)? as u64,
                    score: 0.5,
                })
            },
        )?;

        for row in rows {
            hits.push(row?);
        }
        Ok(hits)
    }

    pub fn search_with_temporal_range(
        &self,
        query: &str,
        limit: usize,
        start_secs: u64,
        end_secs: u64,
        filter: Option<&str>,
    ) -> anyhow::Result<Vec<ChunkHit>> {
        let mut hits = self.bm25_search(query, limit.min(MAX_RESULTS_DEFAULT), filter)?;
        hits.retain(|hit| hit.freshness_secs >= start_secs && hit.freshness_secs <= end_secs);
        Ok(hits)
    }

    pub fn graph_search_module_deps(
        &self,
        module_id: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<ChunkHit>> {
        let conn = self.connection()?;
        let mut hits = Vec::new();
        let module_pattern = format!("%{}%", module_id);
        let mut stmt = conn.prepare(
            "SELECT key, module, chunk_type, text, source, freshness_secs
             FROM kb_chunks
             WHERE module = ?1 OR text LIKE ?2
             ORDER BY freshness_secs DESC
             LIMIT ?3",
        )?;
        let rows = stmt.query_map(
            rusqlite::params![module_id, module_pattern, limit.min(MAX_RESULTS_DEFAULT) as i64],
            |row| {
                Ok(ChunkHit {
                    key: row.get(0)?,
                    module: row.get(1)?,
                    chunk_type: row.get(2)?,
                    text: row.get(3)?,
                    source: row.get(4)?,
                    freshness_secs: row.get::<_, i64>(5)? as u64,
                    score: 0.5,
                })
            },
        )?;

        for row in rows {
            hits.push(row?);
        }

        Ok(hits)
    }

    pub fn chunk_count(&self) -> anyhow::Result<usize> {
        let conn = self.connection()?;
        let count: i64 = conn.query_row("SELECT count(*) FROM kb_chunks", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn get_all_chunks(&self) -> anyhow::Result<Vec<crate::chunking::MirrChunk>> {
        let conn = self.connection()?;
        let mut stmt =
            conn.prepare("SELECT key, chunk_type, text, source, embedding_json FROM kb_chunks")?;
        let rows = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let _chunk_type: String = row.get(1)?;
            let text: String = row.get(2)?;
            let source: String = row.get(3)?;
            let emb_json: Option<String> = row.get(4)?;
            let vector = emb_json.and_then(|j| serde_json::from_str(&j).ok());
            Ok(crate::chunking::MirrChunk::new(
                key,
                crate::chunking::ChunkType::Module,
                text,
                source,
                vector,
                (1, 1),
            ))
        })?;

        let mut chunks = Vec::new();
        for row in rows {
            chunks.push(row?);
        }
        Ok(chunks)
    }

    fn fallback_lexical_scan(
        &self,
        query: &str,
        limit: usize,
        filter: Option<&str>,
    ) -> anyhow::Result<Vec<ChunkHit>> {
        let conn = self.connection()?;
        let needle = query.to_lowercase();
        let filter = filter.map(|value| value.to_lowercase());
        let mut hits = Vec::new();
        let mut stmt = conn.prepare(
            "SELECT key, module, chunk_type, text, source, freshness_secs
             FROM kb_chunks",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)? as u64,
            ))
        })?;

        for row in rows {
            let (key, module, chunk_type, text, source, freshness_secs) = row?;
            let haystack = format!("{} {} {} {}", key, module, chunk_type, text).to_lowercase();
            if !haystack.contains(&needle) {
                continue;
            }
            if let Some(filter_value) = &filter {
                if !matches_filter(&key, &module, &source, &chunk_type, &text, filter_value) {
                    continue;
                }
            }
            hits.push(ChunkHit {
                key,
                module,
                chunk_type,
                text,
                source,
                freshness_secs,
                score: 0.25,
            });
        }

        hits.truncate(limit);
        Ok(hits)
    }
}

fn unix_now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

fn build_fts_query(query: &str, filter: Option<&str>) -> String {
    let normalized_query = normalize_terms(query);
    let normalized_filter = filter.map(normalize_terms);

    match (normalized_query.is_empty(), normalized_filter) {
        (true, Some(filter_query)) if !filter_query.is_empty() => filter_query,
        (false, Some(filter_query)) if !filter_query.is_empty() => {
            format!("({}) AND ({})", filter_query, normalized_query)
        }
        (false, _) => normalized_query,
        _ => String::from(""),
    }
}

fn normalize_terms(input: &str) -> String {
    let mut terms = BTreeSet::new();
    for term in
        input.split(|ch: char| !ch.is_alphanumeric() && ch != '_').filter(|term| !term.is_empty())
    {
        terms.insert(term.to_lowercase());
    }
    terms.into_iter().collect::<Vec<_>>().join(" AND ")
}

fn matches_filter(
    key: &str,
    module: &str,
    source: &str,
    chunk_type: &str,
    text: &str,
    filter: &str,
) -> bool {
    let mut generic_terms = Vec::new();
    let mut required_module: Option<String> = None;
    let mut required_chunk_type: Option<String> = None;
    let mut required_source: Option<String> = None;

    for token in filter.split_whitespace() {
        if let Some(value) = token.strip_prefix("module:") {
            required_module = Some(value.to_lowercase());
            continue;
        }
        if let Some(value) = token.strip_prefix("chunk_type:") {
            required_chunk_type = Some(value.to_lowercase());
            continue;
        }
        if let Some(value) = token.strip_prefix("source:") {
            required_source = Some(value.to_lowercase());
            continue;
        }
        generic_terms.push(token.to_lowercase());
    }

    let key_lc = key.to_lowercase();
    let module_lc = module.to_lowercase();
    let source_lc = source.to_lowercase();
    let chunk_type_lc = chunk_type.to_lowercase();
    let text_lc = text.to_lowercase();

    if let Some(expected_module) = required_module {
        if module_lc != expected_module {
            return false;
        }
    }
    if let Some(expected_chunk_type) = required_chunk_type {
        if chunk_type_lc != expected_chunk_type {
            return false;
        }
    }
    if let Some(expected_source) = required_source {
        if !source_lc.contains(&expected_source) {
            return false;
        }
    }

    if generic_terms.is_empty() {
        return true;
    }

    generic_terms.into_iter().all(|term| {
        key_lc.contains(&term)
            || module_lc.contains(&term)
            || source_lc.contains(&term)
            || chunk_type_lc.contains(&term)
            || text_lc.contains(&term)
    })
}

fn normalize_bm25(rank: f32) -> f32 {
    if rank.is_nan() {
        0.0
    } else {
        1.0 / (1.0 + rank.abs())
    }
}

fn normalize_similarity(score: f32) -> f32 {
    score.clamp(0.0, 1.0)
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || b.is_empty() || a.len() != b.len() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for (lhs, rhs) in a.iter().zip(b.iter()) {
        dot += lhs * rhs;
        norm_a += lhs * lhs;
        norm_b += rhs * rhs;
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    (dot / (norm_a.sqrt() * norm_b.sqrt())).clamp(0.0, 1.0)
}

fn serialize_embedding(embedding: &[f32]) -> anyhow::Result<String> {
    let rounded: Vec<String> =
        embedding.iter().map(|value| format!("{:.*}", EMBEDDING_PRECISION, value)).collect();
    Ok(format!("[{}]", rounded.join(",")))
}

fn deserialize_embedding(value: &str) -> anyhow::Result<Vec<f32>> {
    let parsed: Vec<f32> = serde_json::from_str(value)?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunking::{ChunkType, MirrChunk};
    use std::path::PathBuf;

    fn temp_db_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("mirr-hybrid-{}-{}-{}.db", name, std::process::id(), unix_now_secs()));
        path
    }

    fn sample_chunk(id: &str, text: &str) -> MirrChunk {
        MirrChunk::new(
            id.to_string(),
            ChunkType::Module,
            text.to_string(),
            "demo".to_string(),
            None,
            (1, 1),
        )
    }

    #[test]
    fn storage_upserts_and_counts_chunks() {
        let storage = SqliteHybridStorage::from_db_path(temp_db_path("count")).expect("storage");
        storage
            .upsert_chunk(&sample_chunk("demo.module", "signal alpha: u8;"), "src/demo.mirr", None)
            .expect("insert");
        assert_eq!(storage.chunk_count().expect("count"), 1);
    }

    #[test]
    fn bm25_falls_back_to_lexical_scan_if_fts_returns_empty() {
        let storage = SqliteHybridStorage::from_db_path(temp_db_path("fallback")).expect("storage");
        storage
            .upsert_chunk(&sample_chunk("demo.module", "signal alpha: u8;"), "src/demo.mirr", None)
            .expect("insert");
        let hits = storage.bm25_search("alpha", 10, None).expect("search");
        assert!(!hits.is_empty());
        assert_eq!(hits[0].key, "demo.module");
    }

    #[test]
    fn vector_search_orders_by_similarity() {
        let storage = SqliteHybridStorage::from_db_path(temp_db_path("vector")).expect("storage");
        storage
            .upsert_chunk(&sample_chunk("demo.a", "alpha beta"), "src/demo.mirr", Some(&[1.0, 0.0]))
            .expect("insert a");
        storage
            .upsert_chunk(
                &sample_chunk("demo.b", "gamma delta"),
                "src/demo.mirr",
                Some(&[0.0, 1.0]),
            )
            .expect("insert b");
        let hits = storage.vector_search(&[1.0, 0.0], 10, None).expect("search");
        assert_eq!(hits[0].key, "demo.a");
    }

    #[test]
    fn filter_by_chunk_type_returns_only_matching_type() {
        let storage =
            SqliteHybridStorage::from_db_path(temp_db_path("chunk_type")).expect("storage");
        let mut signal_chunk = sample_chunk("demo.signal", "signal alpha: u8;");
        signal_chunk.chunk_type = ChunkType::Signal;
        storage.upsert_chunk(&signal_chunk, "src/demo.mirr", None).expect("insert signal");

        let mut guard_chunk = sample_chunk("demo.guard", "on alpha > 0");
        guard_chunk.chunk_type = ChunkType::Guard;
        storage.upsert_chunk(&guard_chunk, "src/demo.mirr", None).expect("insert guard");

        let hits = storage.filter_by_chunk_type("Signal", 10).expect("filter");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].chunk_type, "Signal");
    }

    #[test]
    fn temporal_search_respects_freshness_window() {
        let storage = SqliteHybridStorage::from_db_path(temp_db_path("temporal")).expect("storage");
        storage
            .upsert_chunk(&sample_chunk("demo.module", "signal alpha: u8;"), "src/demo.mirr", None)
            .expect("insert");

        let now = unix_now_secs();
        let hits = storage
            .search_with_temporal_range(
                "alpha",
                10,
                now.saturating_sub(5),
                now.saturating_add(5),
                None,
            )
            .expect("temporal search");
        assert_eq!(hits.len(), 1);
    }

    #[test]
    fn graph_search_uses_module_or_text_match() {
        let storage = SqliteHybridStorage::from_db_path(temp_db_path("graph")).expect("storage");
        storage
            .upsert_chunk(&sample_chunk("demo.core", "depends on demo.io"), "src/demo.mirr", None)
            .expect("insert core");
        storage
            .upsert_chunk(&sample_chunk("demo.io", "signal data: u8;"), "src/demo.mirr", None)
            .expect("insert io");

        let hits = storage.graph_search_module_deps("demo.io", 10).expect("graph");
        assert!(!hits.is_empty());
    }
}
