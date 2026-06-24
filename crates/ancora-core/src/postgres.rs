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
