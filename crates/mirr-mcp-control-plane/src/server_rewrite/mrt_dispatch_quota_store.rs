#![forbid(unsafe_code)]

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

pub const MAX_QUOTA_TOKEN_BYTES: usize = 512;
pub const MAX_QUOTA_QUERY_ROWS: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PersistedTokenQuotaState {
    pub token: String,
    pub window_start_ms: u64,
    pub count: u32,
}

pub trait MrtDispatchQuotaEventSink: Send + Sync {
    fn persist_token_quota(
        &self,
        token: &str,
        window_start_ms: u64,
        count: u32,
    ) -> Result<(), String>;

    fn load_recent_token_quota_rows(
        &self,
        _limit: usize,
    ) -> Result<Vec<PersistedTokenQuotaState>, String> {
        Ok(Vec::new())
    }
}

#[derive(Default)]
pub struct NoopMrtDispatchQuotaEventSink;

impl MrtDispatchQuotaEventSink for NoopMrtDispatchQuotaEventSink {
    fn persist_token_quota(
        &self,
        _token: &str,
        _window_start_ms: u64,
        _count: u32,
    ) -> Result<(), String> {
        Ok(())
    }
}

pub fn bounded_recent_quota_rows(
    sink: &dyn MrtDispatchQuotaEventSink,
    limit: usize,
) -> Result<Vec<PersistedTokenQuotaState>, String> {
    let bounded_limit = limit.clamp(1, MAX_QUOTA_QUERY_ROWS);
    let rows = sink.load_recent_token_quota_rows(bounded_limit)?;

    let mut sanitized = Vec::<PersistedTokenQuotaState>::new();
    for row in rows {
        if row.token.is_empty() {
            continue;
        }

        sanitized.push(row);
        if sanitized.len() >= bounded_limit {
            break;
        }
    }

    Ok(sanitized)
}

pub struct SqliteMrtDispatchQuotaStore {
    connection: Mutex<Connection>,
}

pub struct SqliteMrtDispatchQuotaEventSink {
    store: SqliteMrtDispatchQuotaStore,
}

fn now_unix_millis() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or(0)
}

fn sanitize_token(token: &str) -> Result<String, String> {
    if token.is_empty() {
        return Err("quota_token_empty".to_owned());
    }
    if token.len() > MAX_QUOTA_TOKEN_BYTES {
        return Err("quota_token_too_large".to_owned());
    }

    Ok(token.to_owned())
}

fn to_i64_u64(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

fn to_i64_u32(value: u32) -> i64 {
    i64::from(value)
}

fn non_negative_i64_to_u64(value: i64) -> u64 {
    if value < 0 {
        0
    } else {
        value as u64
    }
}

fn non_negative_i64_to_u32(value: i64) -> u32 {
    if value <= 0 {
        0
    } else if value > i64::from(u32::MAX) {
        u32::MAX
    } else {
        value as u32
    }
}

impl SqliteMrtDispatchQuotaStore {
    pub fn open(path: &str) -> Result<Self, String> {
        let connection = Connection::open(path)
            .map_err(|err| format!("sqlite_open_quota_store_failed:{}", err))?;
        Self::initialize_schema(&connection)?;

        Ok(Self { connection: Mutex::new(connection) })
    }

    pub fn in_memory() -> Result<Self, String> {
        let connection = Connection::open_in_memory()
            .map_err(|err| format!("sqlite_open_quota_store_in_memory_failed:{}", err))?;
        Self::initialize_schema(&connection)?;

        Ok(Self { connection: Mutex::new(connection) })
    }

    pub fn upsert_token_quota(
        &self,
        token: &str,
        window_start_ms: u64,
        count: u32,
    ) -> Result<(), String> {
        let token = sanitize_token(token)?;
        let window_start_ms = to_i64_u64(window_start_ms);
        let count = to_i64_u32(count);
        let updated_at_ms = to_i64_u64(now_unix_millis());

        let guard = self.connection.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        guard
            .execute(
                "INSERT INTO mrt_dispatch_token_quota_state (
                    token,
                    window_start_ms,
                    request_count,
                    updated_at_ms
                 ) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(token) DO UPDATE SET
                    window_start_ms = excluded.window_start_ms,
                    request_count = excluded.request_count,
                    updated_at_ms = excluded.updated_at_ms",
                params![token, window_start_ms, count, updated_at_ms],
            )
            .map_err(|err| format!("sqlite_upsert_token_quota_failed:{}", err))?;

        Ok(())
    }

    pub fn read_token_quota(
        &self,
        token: &str,
    ) -> Result<Option<PersistedTokenQuotaState>, String> {
        let token = sanitize_token(token)?;
        let guard = self.connection.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        let mut statement = guard
            .prepare(
                "SELECT token, window_start_ms, request_count
                 FROM mrt_dispatch_token_quota_state
                 WHERE token = ?1",
            )
            .map_err(|err| format!("sqlite_prepare_read_token_quota_failed:{}", err))?;

        let mut rows = statement
            .query(params![token])
            .map_err(|err| format!("sqlite_query_read_token_quota_failed:{}", err))?;

        let Some(row) =
            rows.next().map_err(|err| format!("sqlite_next_read_token_quota_failed:{}", err))?
        else {
            return Ok(None);
        };

        let token: String =
            row.get(0).map_err(|err| format!("sqlite_decode_quota_token_failed:{}", err))?;
        let window_start_ms: i64 =
            row.get(1).map_err(|err| format!("sqlite_decode_quota_window_start_failed:{}", err))?;
        let count: i64 =
            row.get(2).map_err(|err| format!("sqlite_decode_quota_count_failed:{}", err))?;

        Ok(Some(PersistedTokenQuotaState {
            token,
            window_start_ms: non_negative_i64_to_u64(window_start_ms),
            count: non_negative_i64_to_u32(count),
        }))
    }

    pub fn recent_rows(&self, limit: usize) -> Result<Vec<PersistedTokenQuotaState>, String> {
        let bounded_limit = limit.clamp(1, MAX_QUOTA_QUERY_ROWS);
        let limit_i64 = i64::try_from(bounded_limit).unwrap_or(i64::MAX);

        let guard = self.connection.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        let mut statement = guard
            .prepare(
                "SELECT token, window_start_ms, request_count
                 FROM mrt_dispatch_token_quota_state
                 ORDER BY updated_at_ms DESC
                 LIMIT ?1",
            )
            .map_err(|err| format!("sqlite_prepare_recent_quota_rows_failed:{}", err))?;

        let mut rows = statement
            .query(params![limit_i64])
            .map_err(|err| format!("sqlite_query_recent_quota_rows_failed:{}", err))?;

        let mut result = Vec::<PersistedTokenQuotaState>::new();
        for _ in 0..bounded_limit {
            let maybe_row = rows
                .next()
                .map_err(|err| format!("sqlite_next_recent_quota_row_failed:{}", err))?;
            let Some(row) = maybe_row else {
                break;
            };

            let token: String = row
                .get(0)
                .map_err(|err| format!("sqlite_decode_recent_quota_token_failed:{}", err))?;
            let window_start_ms: i64 = row
                .get(1)
                .map_err(|err| format!("sqlite_decode_recent_quota_window_start_failed:{}", err))?;
            let count: i64 = row
                .get(2)
                .map_err(|err| format!("sqlite_decode_recent_quota_count_failed:{}", err))?;

            result.push(PersistedTokenQuotaState {
                token,
                window_start_ms: non_negative_i64_to_u64(window_start_ms),
                count: non_negative_i64_to_u32(count),
            });
        }

        Ok(result)
    }

    fn initialize_schema(connection: &Connection) -> Result<(), String> {
        connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS mrt_dispatch_token_quota_state (
                    token TEXT PRIMARY KEY,
                    window_start_ms INTEGER NOT NULL,
                    request_count INTEGER NOT NULL,
                    updated_at_ms INTEGER NOT NULL
                );
                CREATE INDEX IF NOT EXISTS idx_mrt_dispatch_token_quota_updated
                ON mrt_dispatch_token_quota_state(updated_at_ms DESC);",
            )
            .map_err(|err| format!("sqlite_init_quota_schema_failed:{}", err))
    }
}

impl SqliteMrtDispatchQuotaEventSink {
    pub fn open(path: &str) -> Result<Self, String> {
        Ok(Self { store: SqliteMrtDispatchQuotaStore::open(path)? })
    }

    pub fn in_memory() -> Result<Self, String> {
        Ok(Self { store: SqliteMrtDispatchQuotaStore::in_memory()? })
    }

    pub fn read_token_quota(
        &self,
        token: &str,
    ) -> Result<Option<PersistedTokenQuotaState>, String> {
        self.store.read_token_quota(token)
    }
}

impl MrtDispatchQuotaEventSink for SqliteMrtDispatchQuotaEventSink {
    fn persist_token_quota(
        &self,
        token: &str,
        window_start_ms: u64,
        count: u32,
    ) -> Result<(), String> {
        self.store.upsert_token_quota(token, window_start_ms, count)
    }

    fn load_recent_token_quota_rows(
        &self,
        limit: usize,
    ) -> Result<Vec<PersistedTokenQuotaState>, String> {
        self.store.recent_rows(limit)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        bounded_recent_quota_rows, MrtDispatchQuotaEventSink, NoopMrtDispatchQuotaEventSink,
        PersistedTokenQuotaState, SqliteMrtDispatchQuotaEventSink, SqliteMrtDispatchQuotaStore,
        MAX_QUOTA_TOKEN_BYTES,
    };

    #[test]
    fn sqlite_quota_store_upsert_and_read_roundtrip() {
        let store = SqliteMrtDispatchQuotaStore::in_memory()
            .expect("in-memory quota store should initialize");

        store.upsert_token_quota("builder-token", 100, 1).expect("first quota row should append");
        store
            .upsert_token_quota("builder-token", 100, 2)
            .expect("quota row should upsert by token");

        let row = store
            .read_token_quota("builder-token")
            .expect("quota row should read")
            .expect("quota row should exist");
        assert_eq!(row.token, "builder-token");
        assert_eq!(row.window_start_ms, 100);
        assert_eq!(row.count, 2);
    }

    #[test]
    fn sqlite_quota_store_rejects_oversized_token() {
        let store = SqliteMrtDispatchQuotaStore::in_memory()
            .expect("in-memory quota store should initialize");
        let oversized = "x".repeat(MAX_QUOTA_TOKEN_BYTES + 1);

        let err = store
            .upsert_token_quota(&oversized, 0, 1)
            .expect_err("oversized token should fail closed");
        assert!(err.contains("quota_token_too_large"));
    }

    #[test]
    fn sqlite_quota_sink_persists_rows_without_panicking() {
        let sink = SqliteMrtDispatchQuotaEventSink::in_memory()
            .expect("in-memory quota sink should initialize");

        sink.persist_token_quota("committer-token", 200, 3)
            .expect("quota sink persistence should succeed");

        let row = sink
            .read_token_quota("committer-token")
            .expect("quota row should be readable")
            .expect("quota row should exist");
        assert_eq!(row.count, 3);
        assert_eq!(row.window_start_ms, 200);
    }

    #[test]
    fn bounded_recent_rows_filters_empty_tokens_and_clamps_limit() {
        struct FakeSink;

        impl MrtDispatchQuotaEventSink for FakeSink {
            fn persist_token_quota(
                &self,
                _token: &str,
                _window_start_ms: u64,
                _count: u32,
            ) -> Result<(), String> {
                Ok(())
            }

            fn load_recent_token_quota_rows(
                &self,
                _limit: usize,
            ) -> Result<Vec<PersistedTokenQuotaState>, String> {
                Ok(vec![
                    PersistedTokenQuotaState { token: String::new(), window_start_ms: 1, count: 1 },
                    PersistedTokenQuotaState {
                        token: "builder-token".to_owned(),
                        window_start_ms: 2,
                        count: 2,
                    },
                    PersistedTokenQuotaState {
                        token: "committer-token".to_owned(),
                        window_start_ms: 3,
                        count: 3,
                    },
                ])
            }
        }

        let sink = FakeSink;
        let rows = bounded_recent_quota_rows(&sink, 1)
            .expect("quota rows should be readable for fake sink");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].token, "builder-token");

        let noop_rows = bounded_recent_quota_rows(&NoopMrtDispatchQuotaEventSink, 32)
            .expect("noop sink rows should succeed");
        assert!(noop_rows.is_empty());
    }
}
