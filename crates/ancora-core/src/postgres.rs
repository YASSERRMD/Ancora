// Postgres-backed journal and checkpoint store. Compiled only with the `postgres` feature.
use std::sync::Mutex;

use prost::Message as _;
use postgres::{Client, NoTls};

use ancora_proto::ancora::JournalEvent;

use crate::error::AncoraError;
use crate::journal::{CheckpointStore, JournalStore};

fn storage(e: impl std::fmt::Display) -> AncoraError {
    AncoraError::Storage(e.to_string())
}

const MIGRATION_V1: &str = "
CREATE TABLE IF NOT EXISTS journal_events (
    id              BIGSERIAL PRIMARY KEY,
    run_id          TEXT    NOT NULL,
    seq             BIGINT  NOT NULL,
    event_id        TEXT    NOT NULL,
    recorded_at_ns  BIGINT  NOT NULL DEFAULT 0,
    activity_key    TEXT    UNIQUE,
    proto_bytes     BYTEA   NOT NULL,
    UNIQUE (run_id, seq)
);

CREATE INDEX IF NOT EXISTS idx_journal_events_run_seq
    ON journal_events (run_id, seq);

CREATE TABLE IF NOT EXISTS checkpoints (
    run_id  TEXT    PRIMARY KEY,
    at_seq  BIGINT  NOT NULL,
    data    BYTEA   NOT NULL
);
";

/// Postgres-backed journal and checkpoint store.
pub struct PostgresStore {
    client: Mutex<Client>,
}

fn extract_activity_key(event: &JournalEvent) -> Option<String> {
    use ancora_proto::ancora::journal_event::Event;
    match event.event.as_ref()? {
        Event::ActivityRecorded(a) if !a.activity_key.is_empty() => {
            Some(a.activity_key.clone())
        }
        _ => None,
    }
}

impl JournalStore for PostgresStore {
    fn append(&self, run_id: &str, mut event: JournalEvent) -> Result<u64, AncoraError> {
        let mut client = self.client.lock().map_err(|_| storage("mutex poisoned"))?;

        Self::lock_run(&mut client, run_id)?;

        let row = client
            .query_one(
                "SELECT COUNT(*) FROM journal_events WHERE run_id = $1",
                &[&run_id],
            )
            .map_err(storage)?;
        let seq = row.get::<_, i64>(0) as u64;

        event.seq = seq;
        event.run_id = run_id.to_string();

        let activity_key = extract_activity_key(&event);
        let proto_bytes = event.encode_to_vec();

        client
            .execute(
                "INSERT INTO journal_events
                    (run_id, seq, event_id, recorded_at_ns, activity_key, proto_bytes)
                 VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &run_id,
                    &(seq as i64),
                    &event.event_id,
                    &event.recorded_at_ns,
                    &activity_key,
                    &proto_bytes,
                ],
            )
            .map_err(|e| {
                let msg = e.to_string();
                if msg.contains("unique") || msg.contains("duplicate") {
                    AncoraError::JournalWrite(format!("duplicate activity_key: {e}"))
                } else {
                    storage(e)
                }
            })?;

        Ok(seq)
    }

    fn read(&self, _run_id: &str) -> Result<Vec<JournalEvent>, AncoraError> {
        Ok(vec![])
    }

    fn load(&self, _run_id: &str, _seq: u64) -> Result<Option<JournalEvent>, AncoraError> {
        Ok(None)
    }
}

impl PostgresStore {
    fn lock_run(client: &mut Client, run_id: &str) -> Result<(), AncoraError> {
        client
            .execute("SELECT pg_advisory_xact_lock(hashtext($1))", &[&run_id])
            .map(|_| ())
            .map_err(storage)
    }

    /// Connect using a Postgres connection string and run schema migrations.
    pub fn connect(connection_str: &str) -> Result<Self, AncoraError> {
        let client = Client::connect(connection_str, NoTls).map_err(storage)?;
        let store = Self { client: Mutex::new(client) };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<(), AncoraError> {
        let mut client = self.client.lock().map_err(|_| storage("mutex poisoned"))?;
        client.batch_execute(MIGRATION_V1).map_err(storage)
    }
}
