use std::path::Path;
use std::sync::Mutex;

use prost::Message as _;
use rusqlite::{params, Connection, OptionalExtension};

use ancora_proto::ancora::JournalEvent;

use crate::error::AncoraError;
use crate::journal::{CheckpointStore, JournalStore};

/// SQLite-backed journal and checkpoint store.
///
/// The connection is wrapped in a Mutex so `SqliteStore` is Send+Sync and can
/// be placed behind an Arc for multi-threaded use.
pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    /// Open (or create) a store at the given path and run migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AncoraError> {
        let conn = Connection::open(path)
            .map_err(|e| AncoraError::Storage(format!("open: {e}")))?;
        let store = Self { conn: Mutex::new(conn) };
        store.migrate()?;
        Ok(store)
    }

    /// Open an in-memory store (useful for tests).
    pub fn open_in_memory() -> Result<Self, AncoraError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| AncoraError::Storage(format!("open_in_memory: {e}")))?;
        let store = Self { conn: Mutex::new(conn) };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<(), AncoraError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;
        conn.execute_batch(MIGRATION_V1)
            .map_err(|e| AncoraError::Storage(format!("migrate: {e}")))
    }
}

/// Schema version 1. Applied once at open time via execute_batch.
/// New columns must be added in a MIGRATION_V2 constant appended here;
/// never modify MIGRATION_V1 after it ships.
const MIGRATION_V1: &str = "
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS journal_events (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id          TEXT    NOT NULL,
    seq             INTEGER NOT NULL,
    event_id        TEXT    NOT NULL,
    recorded_at_ns  INTEGER NOT NULL DEFAULT 0,
    activity_key    TEXT,
    proto_bytes     BLOB    NOT NULL,
    UNIQUE (run_id, seq)
);

CREATE INDEX IF NOT EXISTS idx_journal_events_run_seq
    ON journal_events (run_id, seq);

CREATE TABLE IF NOT EXISTS checkpoints (
    run_id  TEXT    PRIMARY KEY,
    at_seq  INTEGER NOT NULL,
    data    BLOB    NOT NULL
);
";

fn extract_activity_key(event: &JournalEvent) -> Option<String> {
    use ancora_proto::ancora::journal_event::Event;
    match event.event.as_ref()? {
        Event::ActivityRecorded(a) => {
            if a.activity_key.is_empty() {
                None
            } else {
                Some(a.activity_key.clone())
            }
        }
        _ => None,
    }
}

impl JournalStore for SqliteStore {
    fn append(&self, run_id: &str, mut event: JournalEvent) -> Result<u64, AncoraError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;

        let current_count: u64 = conn
            .query_row(
                "SELECT COUNT(*) FROM journal_events WHERE run_id = ?1",
                params![run_id],
                |row| row.get(0),
            )
            .map_err(|e| AncoraError::Storage(format!("count: {e}")))?;

        let seq = current_count;
        event.seq = seq;
        event.run_id = run_id.to_string();

        let activity_key = extract_activity_key(&event);
        let proto_bytes = event.encode_to_vec();

        conn.execute(
            "INSERT INTO journal_events
                (run_id, seq, event_id, recorded_at_ns, activity_key, proto_bytes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                run_id,
                seq as i64,
                event.event_id,
                event.recorded_at_ns,
                activity_key,
                proto_bytes,
            ],
        )
        .map_err(|e| AncoraError::Storage(format!("insert: {e}")))?;

        Ok(seq)
    }

    fn read(&self, run_id: &str) -> Result<Vec<JournalEvent>, AncoraError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT proto_bytes FROM journal_events
                  WHERE run_id = ?1
                  ORDER BY seq ASC",
            )
            .map_err(|e| AncoraError::Storage(format!("prepare: {e}")))?;

        let events: Result<Vec<JournalEvent>, AncoraError> = stmt
            .query_map(params![run_id], |row| row.get::<_, Vec<u8>>(0))
            .map_err(|e| AncoraError::Storage(format!("query: {e}")))?
            .map(|bytes| {
                let b = bytes.map_err(|e| AncoraError::Storage(format!("row: {e}")))?;
                JournalEvent::decode(b.as_slice())
                    .map_err(|e| AncoraError::Storage(format!("decode: {e}")))
            })
            .collect();

        events
    }

    fn load(&self, run_id: &str, seq: u64) -> Result<Option<JournalEvent>, AncoraError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;

        let result = conn
            .query_row(
                "SELECT proto_bytes FROM journal_events
                  WHERE run_id = ?1 AND seq = ?2",
                params![run_id, seq as i64],
                |row| row.get::<_, Vec<u8>>(0),
            )
            .optional()
            .map_err(|e| AncoraError::Storage(format!("load: {e}")))?;

        match result {
            None => Ok(None),
            Some(bytes) => {
                let ev = JournalEvent::decode(bytes.as_slice())
                    .map_err(|e| AncoraError::Storage(format!("decode: {e}")))?;
                Ok(Some(ev))
            }
        }
    }
}

impl CheckpointStore for SqliteStore {
    fn save(&self, run_id: &str, at_seq: u64, data: &[u8]) -> Result<(), AncoraError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;

        conn.execute(
            "INSERT INTO checkpoints (run_id, at_seq, data)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(run_id) DO UPDATE SET at_seq = excluded.at_seq, data = excluded.data",
            params![run_id, at_seq as i64, data],
        )
        .map_err(|e| AncoraError::Storage(format!("checkpoint save: {e}")))?;
        Ok(())
    }

    fn load_checkpoint(&self, run_id: &str) -> Result<Option<(u64, Vec<u8>)>, AncoraError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AncoraError::Storage("mutex poisoned".to_string()))?;

        let result = conn
            .query_row(
                "SELECT at_seq, data FROM checkpoints WHERE run_id = ?1",
                params![run_id],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, Vec<u8>>(1)?)),
            )
            .optional()
            .map_err(|e| AncoraError::Storage(format!("checkpoint load: {e}")))?;

        Ok(result.map(|(seq, data)| (seq as u64, data)))
    }
}
