#![forbid(unsafe_code)]

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

use super::mrt_dispatch_route_handler::MrtDispatchAuditEvent;

pub const MAX_AUDIT_KIND_BYTES: usize = 64;
pub const MAX_AUDIT_SUBJECT_BYTES: usize = 256;
pub const MAX_AUDIT_MESSAGE_BYTES: usize = 2_048;
pub const MAX_AUDIT_QUERY_ROWS: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchAuditRow {
    pub event_id: i64,
    pub timestamp_ms: u64,
    pub kind: String,
    pub subject: String,
    pub message: Option<String>,
}

pub trait MrtDispatchAuditEventSink: Send + Sync {
    fn append(&self, event: &MrtDispatchAuditEvent);
}

#[derive(Default)]
pub struct NoopMrtDispatchAuditEventSink;

impl MrtDispatchAuditEventSink for NoopMrtDispatchAuditEventSink {
    fn append(&self, _event: &MrtDispatchAuditEvent) {}
}

pub struct SqliteMrtDispatchAuditStore {
    connection: Mutex<Connection>,
}

pub struct SqliteMrtDispatchAuditEventSink {
    store: SqliteMrtDispatchAuditStore,
}

fn now_unix_millis() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => match u64::try_from(duration.as_millis()) {
            Ok(value) => value,
            Err(_) => u64::MAX,
        },
        Err(_) => 0,
    }
}

fn sanitize_required_text(
    value: &str,
    max_bytes: usize,
    field_name: &str,
) -> Result<String, String> {
    if value.is_empty() {
        return Err(format!("{}_empty", field_name));
    }
    if value.len() > max_bytes {
        return Err(format!("{}_too_large", field_name));
    }

    Ok(value.to_owned())
}

fn sanitize_optional_text(
    value: Option<&str>,
    max_bytes: usize,
    field_name: &str,
) -> Result<Option<String>, String> {
    let Some(raw) = value else {
        return Ok(None);
    };

    if raw.is_empty() {
        return Ok(None);
    }
    if raw.len() > max_bytes {
        return Err(format!("{}_too_large", field_name));
    }

    Ok(Some(raw.to_owned()))
}

fn timestamp_to_i64(timestamp_ms: u64) -> i64 {
    match i64::try_from(timestamp_ms) {
        Ok(value) => value,
        Err(_) => i64::MAX,
    }
}

impl SqliteMrtDispatchAuditStore {
    pub fn open(path: &str) -> Result<Self, String> {
        let connection =
            Connection::open(path).map_err(|err| format!("sqlite_open_failed:{}", err))?;
        Self::initialize_schema(&connection)?;

        Ok(Self { connection: Mutex::new(connection) })
    }

    pub fn in_memory() -> Result<Self, String> {
        let connection = Connection::open_in_memory()
            .map_err(|err| format!("sqlite_open_in_memory_failed:{}", err))?;
        Self::initialize_schema(&connection)?;

        Ok(Self { connection: Mutex::new(connection) })
    }

    pub fn append_event(&self, event: &MrtDispatchAuditEvent) -> Result<(), String> {
        let kind = sanitize_required_text(event.kind, MAX_AUDIT_KIND_BYTES, "audit_kind")?;
        let subject =
            sanitize_required_text(&event.subject, MAX_AUDIT_SUBJECT_BYTES, "audit_subject")?;
        let message = sanitize_optional_text(
            event.message.as_deref(),
            MAX_AUDIT_MESSAGE_BYTES,
            "audit_message",
        )?;
        let timestamp_ms = timestamp_to_i64(now_unix_millis());

        let guard = self.connection.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        guard
            .execute(
                "INSERT INTO mrt_dispatch_audit_events (timestamp_ms, kind, subject, message) VALUES (?1, ?2, ?3, ?4)",
                params![timestamp_ms, kind, subject, message],
            )
            .map_err(|err| format!("sqlite_insert_audit_event_failed:{}", err))?;

        Ok(())
    }

    pub fn recent_rows(&self, limit: usize) -> Result<Vec<MrtDispatchAuditRow>, String> {
        let bounded_limit = limit.clamp(1, MAX_AUDIT_QUERY_ROWS);
        let limit_i64 = match i64::try_from(bounded_limit) {
            Ok(value) => value,
            Err(_) => i64::MAX,
        };

        let guard = self.connection.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

        let mut statement = guard
            .prepare(
                "SELECT event_id, timestamp_ms, kind, subject, message
                 FROM mrt_dispatch_audit_events
                 ORDER BY event_id DESC
                 LIMIT ?1",
            )
            .map_err(|err| format!("sqlite_prepare_recent_audit_rows_failed:{}", err))?;

        let mut rows = statement
            .query(params![limit_i64])
            .map_err(|err| format!("sqlite_query_recent_audit_rows_failed:{}", err))?;

        let mut result = Vec::<MrtDispatchAuditRow>::new();
        for _ in 0..bounded_limit {
            let maybe_row = rows
                .next()
                .map_err(|err| format!("sqlite_next_recent_audit_row_failed:{}", err))?;
            let Some(row) = maybe_row else {
                break;
            };

            let timestamp_raw: i64 = row
                .get(1)
                .map_err(|err| format!("sqlite_decode_recent_audit_timestamp_failed:{}", err))?;
            let timestamp_ms = if timestamp_raw < 0 { 0 } else { timestamp_raw as u64 };

            result.push(MrtDispatchAuditRow {
                event_id: row
                    .get(0)
                    .map_err(|err| format!("sqlite_decode_recent_audit_event_id_failed:{}", err))?,
                timestamp_ms,
                kind: row
                    .get(2)
                    .map_err(|err| format!("sqlite_decode_recent_audit_kind_failed:{}", err))?,
                subject: row
                    .get(3)
                    .map_err(|err| format!("sqlite_decode_recent_audit_subject_failed:{}", err))?,
                message: row
                    .get(4)
                    .map_err(|err| format!("sqlite_decode_recent_audit_message_failed:{}", err))?,
            });
        }

        Ok(result)
    }

    fn initialize_schema(connection: &Connection) -> Result<(), String> {
        connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS mrt_dispatch_audit_events (
                    event_id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp_ms INTEGER NOT NULL,
                    kind TEXT NOT NULL,
                    subject TEXT NOT NULL,
                    message TEXT
                );
                CREATE INDEX IF NOT EXISTS idx_mrt_dispatch_audit_events_event_id
                ON mrt_dispatch_audit_events(event_id DESC);",
            )
            .map_err(|err| format!("sqlite_init_audit_schema_failed:{}", err))
    }
}

impl SqliteMrtDispatchAuditEventSink {
    pub fn open(path: &str) -> Result<Self, String> {
        Ok(Self { store: SqliteMrtDispatchAuditStore::open(path)? })
    }

    pub fn in_memory() -> Result<Self, String> {
        Ok(Self { store: SqliteMrtDispatchAuditStore::in_memory()? })
    }

    pub fn recent_rows(&self, limit: usize) -> Result<Vec<MrtDispatchAuditRow>, String> {
        self.store.recent_rows(limit)
    }
}

impl MrtDispatchAuditEventSink for SqliteMrtDispatchAuditEventSink {
    fn append(&self, event: &MrtDispatchAuditEvent) {
        let _ = self.store.append_event(event);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        MrtDispatchAuditEventSink, SqliteMrtDispatchAuditEventSink, SqliteMrtDispatchAuditStore,
        MAX_AUDIT_SUBJECT_BYTES,
    };
    use crate::server_rewrite::mrt_dispatch_route_handler::MrtDispatchAuditEvent;

    #[test]
    fn sqlite_store_appends_and_reads_recent_rows() {
        let store = SqliteMrtDispatchAuditStore::in_memory()
            .expect("in-memory sqlite store should initialize");

        store
            .append_event(&MrtDispatchAuditEvent {
                kind: "mrt_dispatch_start",
                subject: "mrt_audit".to_owned(),
                message: Some("role=builder".to_owned()),
            })
            .expect("audit start row should append");
        store
            .append_event(&MrtDispatchAuditEvent {
                kind: "mrt_dispatch_complete",
                subject: "mrt_audit".to_owned(),
                message: Some("exit_code=0".to_owned()),
            })
            .expect("audit completion row should append");

        let rows = store.recent_rows(8).expect("recent rows query should succeed");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].kind, "mrt_dispatch_complete");
        assert_eq!(rows[1].kind, "mrt_dispatch_start");
    }

    #[test]
    fn sqlite_store_rejects_oversized_subject() {
        let store = SqliteMrtDispatchAuditStore::in_memory()
            .expect("in-memory sqlite store should initialize");

        let oversized_subject = "x".repeat(MAX_AUDIT_SUBJECT_BYTES + 1);
        let err = store
            .append_event(&MrtDispatchAuditEvent {
                kind: "mrt_dispatch_error",
                subject: oversized_subject,
                message: Some("payload".to_owned()),
            })
            .expect_err("oversized subject must fail closed");

        assert!(err.contains("audit_subject_too_large"));
    }

    #[test]
    fn sqlite_sink_appends_rows_without_panicking() {
        let sink = SqliteMrtDispatchAuditEventSink::in_memory()
            .expect("in-memory sqlite sink should initialize");

        sink.append(&MrtDispatchAuditEvent {
            kind: "mrt_dispatch_start",
            subject: "mrt_compile".to_owned(),
            message: Some("role=builder".to_owned()),
        });

        let rows = sink.recent_rows(4).expect("sink recent rows should succeed");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].kind, "mrt_dispatch_start");
        assert_eq!(rows[0].subject, "mrt_compile");
    }
}
