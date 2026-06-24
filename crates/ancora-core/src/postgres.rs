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

    fn read(&self, run_id: &str) -> Result<Vec<JournalEvent>, AncoraError> {
        let mut client = self.client.lock().map_err(|_| storage("mutex poisoned"))?;

        let rows = client
            .query(
                "SELECT proto_bytes FROM journal_events
                  WHERE run_id = $1
                  ORDER BY seq ASC",
                &[&run_id],
            )
            .map_err(storage)?;

        rows.iter()
            .map(|row| {
                let bytes: Vec<u8> = row.get(0);
                JournalEvent::decode(bytes.as_slice())
                    .map_err(|e| storage(format!("decode: {e}")))
            })
            .collect()
    }

    fn load(&self, run_id: &str, seq: u64) -> Result<Option<JournalEvent>, AncoraError> {
        let mut client = self.client.lock().map_err(|_| storage("mutex poisoned"))?;

        let rows = client
            .query(
                "SELECT proto_bytes FROM journal_events
                  WHERE run_id = $1 AND seq = $2",
                &[&run_id, &(seq as i64)],
            )
            .map_err(storage)?;

        match rows.into_iter().next() {
            None => Ok(None),
            Some(row) => {
                let bytes: Vec<u8> = row.get(0);
                JournalEvent::decode(bytes.as_slice())
                    .map_err(|e| storage(format!("decode: {e}")))
                    .map(Some)
            }
        }
    }
}

impl CheckpointStore for PostgresStore {
    fn save(&self, run_id: &str, at_seq: u64, data: &[u8]) -> Result<(), AncoraError> {
        let mut client = self.client.lock().map_err(|_| storage("mutex poisoned"))?;

        client
            .execute(
                "INSERT INTO checkpoints (run_id, at_seq, data)
                 VALUES ($1, $2, $3)
                 ON CONFLICT (run_id) DO UPDATE SET at_seq = EXCLUDED.at_seq, data = EXCLUDED.data",
                &[&run_id, &(at_seq as i64), &data],
            )
            .map(|_| ())
            .map_err(storage)
    }

    fn load_checkpoint(&self, run_id: &str) -> Result<Option<(u64, Vec<u8>)>, AncoraError> {
        let mut client = self.client.lock().map_err(|_| storage("mutex poisoned"))?;

        let rows = client
            .query(
                "SELECT at_seq, data FROM checkpoints WHERE run_id = $1",
                &[&run_id],
            )
            .map_err(storage)?;

        Ok(rows.into_iter().next().map(|row| {
            let at_seq: i64 = row.get(0);
            let data: Vec<u8> = row.get(1);
            (at_seq as u64, data)
        }))
    }
}

impl PostgresStore {
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

    fn lock_run(client: &mut Client, run_id: &str) -> Result<(), AncoraError> {
        client
            .execute("SELECT pg_advisory_xact_lock(hashtext($1))", &[&run_id])
            .map(|_| ())
            .map_err(storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ancora_proto::ancora::{journal_event::Event, ActivityRecordedEvent, RunStartedEvent};

    fn postgres_url() -> Option<String> {
        std::env::var("POSTGRES_URL").ok()
    }

    fn unique_run() -> String {
        format!("pg-run-{}", uuid::Uuid::new_v4())
    }

    fn run_started(label: &str) -> JournalEvent {
        JournalEvent {
            event_id: label.to_string(),
            run_id: String::new(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::RunStarted(RunStartedEvent {
                run_id: label.to_string(),
                spec_bytes: vec![],
                spec_type: "AgentSpec".to_string(),
            })),
        }
    }

    fn activity(key: &str) -> JournalEvent {
        JournalEvent {
            event_id: key.to_string(),
            run_id: String::new(),
            seq: 0,
            recorded_at_ns: 0,
            event: Some(Event::ActivityRecorded(ActivityRecordedEvent {
                activity_key: key.to_string(),
                activity_kind: "model_call".to_string(),
                input_json: "{}".to_string(),
                result_json: "{}".to_string(),
                replayed: false,
            })),
        }
    }
}
