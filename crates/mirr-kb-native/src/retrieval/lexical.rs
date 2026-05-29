#![forbid(unsafe_code)]
#![deny(warnings)]

use crate::retrieval::{
    Freshness, IndexStatus, QueryRequest, QueryResponse, Retrieval, SearchResult,
};
use async_trait::async_trait;
use rusqlite::{params, Connection};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Deterministic lexical search backend using SQLite.
/// No cloud dependencies; works offline and in CI environments.
pub struct LexicalRetrieval {
    db_path: String,
    max_results: usize,
}

impl LexicalRetrieval {
    /// Create a new lexical retrieval instance.
    pub fn new(kb_root: &str, max_results: usize) -> anyhow::Result<Self> {
        let db_path = format!("{}/graph.db", kb_root);

        // Create parent directory if needed
        if let Some(parent) = Path::new(&db_path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Initialize database schema if needed
        let conn = Connection::open(&db_path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS kb_entries (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                source TEXT NOT NULL
            )",
        )?;

        Ok(LexicalRetrieval { db_path, max_results })
    }

    /// Insert or update an entry in the knowledge base.
    pub fn insert(&self, key: &str, value: &str, source: &str) -> anyhow::Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO kb_entries (key, value, source) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value=excluded.value, source=excluded.source",
            params![key, value, source],
        )?;
        Ok(())
    }

    /// Get a single entry by exact key match.
    pub fn get(&self, key: &str) -> anyhow::Result<Option<(String, String)>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT value, source FROM kb_entries WHERE key = ?1")?;

        let result = stmt.query_row(params![key], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        });

        match result {
            Ok((value, source)) => Ok(Some((value, source))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }

    /// Lexical search: case-insensitive substring matching.
    fn lexical_search(&self, query_text: &str, limit: usize) -> anyhow::Result<Vec<SearchResult>> {
        let conn = Connection::open(&self.db_path)?;

        // Case-insensitive LIKE search
        let search_pattern = format!("%{}%", query_text.to_lowercase());
        let mut stmt = conn.prepare(
            "SELECT key, value, source FROM kb_entries 
             WHERE LOWER(key) LIKE ?1 OR LOWER(value) LIKE ?1
             ORDER BY LENGTH(key) ASC
             LIMIT ?2",
        )?;

        let mut results = Vec::new();
        let rows = stmt.query_map(params![search_pattern, limit as i64], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
        })?;

        for row_result in rows {
            let (key, value, source) = row_result?;

            // Create snippet: truncate value to first 200 chars
            let snippet =
                if value.len() > 200 { format!("{}…", &value[..200]) } else { value.clone() };

            results.push(SearchResult {
                key: key.clone(),
                title: key,
                snippet,
                score: 0.75, // Default confidence for lexical match
                source,
            });
        }

        Ok(results)
    }

    /// Get freshness by checking last database modification time.
    fn check_freshness(&self) -> anyhow::Result<Freshness> {
        let path = Path::new(&self.db_path);

        if !path.exists() {
            return Ok(Freshness::Unknown);
        }

        let metadata = std::fs::metadata(path)?;
        let modified = metadata.modified()?;

        let elapsed = SystemTime::now().duration_since(modified).unwrap_or_default();

        // Consider fresh if modified within last hour
        if elapsed.as_secs() < 3600 {
            Ok(Freshness::Fresh)
        } else {
            Ok(Freshness::Stale)
        }
    }

    /// Count indexed entries.
    fn count_entries(&self) -> anyhow::Result<usize> {
        let conn = Connection::open(&self.db_path)?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM kb_entries", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}

#[async_trait]
impl Retrieval for LexicalRetrieval {
    async fn query(&self, req: QueryRequest) -> anyhow::Result<QueryResponse> {
        let start = std::time::Instant::now();

        if req.text.is_empty() {
            return Ok(QueryResponse {
                results: vec![],
                freshness: Freshness::Unknown,
                query_time_ms: start.elapsed().as_millis() as u64,
                truncated: false,
                error: Some("Query text cannot be empty".to_string()),
            });
        }

        let limit = std::cmp::min(req.limit, self.max_results);
        let results = self.lexical_search(&req.text, limit)?;
        let truncated = results.len() >= limit;
        let freshness = self.check_freshness()?;

        Ok(QueryResponse {
            results,
            freshness,
            query_time_ms: start.elapsed().as_millis() as u64,
            truncated,
            error: None,
        })
    }

    async fn index_status(&self) -> anyhow::Result<IndexStatus> {
        let count = self.count_entries()?;
        let freshness = self.check_freshness()?;

        let path = Path::new(&self.db_path);
        let last_refresh_secs = if path.exists() {
            let metadata = std::fs::metadata(path)?;
            let modified = metadata.modified()?;
            modified.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
        } else {
            0
        };

        Ok(IndexStatus {
            indexed_count: count,
            last_refresh_secs,
            is_stale: freshness == Freshness::Stale,
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_db_path(test_name: &str) -> String {
        let dir = PathBuf::from(format!(".kb-test-{}-{}", test_name, std::process::id()));
        format!("{}", dir.display())
    }

    #[tokio::test]
    async fn test_lexical_insert_and_search() {
        let db_path = test_db_path("search");

        let lexical = LexicalRetrieval::new(&db_path, 100).expect("init");

        // Insert test entries
        lexical
            .insert("parser.rs", "pub fn parse_module(...) { ... }", "src/parser/")
            .expect("insert 1");
        lexical
            .insert("typeck.rs", "pub fn check_types(...) { ... }", "src/typeck/")
            .expect("insert 2");

        // Search for "parse"
        let req = QueryRequest {
            text: "parse".to_string(),
            mode: crate::retrieval::SearchMode::Lexical,
            limit: 10,
            filter: None,
        };

        let resp = lexical.query(req).await.expect("query");
        assert_eq!(resp.results.len(), 1);
        assert_eq!(resp.results[0].key, "parser.rs");
        assert!(!resp.truncated);
    }

    #[tokio::test]
    async fn test_lexical_empty_query() {
        let db_path = test_db_path("empty_query");

        let lexical = LexicalRetrieval::new(&db_path, 100).expect("init");

        let req = QueryRequest {
            text: "".to_string(),
            mode: crate::retrieval::SearchMode::Lexical,
            limit: 10,
            filter: None,
        };

        let resp = lexical.query(req).await.expect("query");
        assert_eq!(resp.results.len(), 0);
        assert!(resp.error.is_some());
    }

    #[tokio::test]
    async fn test_index_status() {
        let db_path = test_db_path("status");

        let lexical = LexicalRetrieval::new(&db_path, 100).expect("init");
        lexical.insert("test.rs", "fn test() {}", "src/").expect("insert");

        let status = lexical.index_status().await.expect("status");
        assert_eq!(status.indexed_count, 1);
        assert!(!status.is_stale);
    }
}
