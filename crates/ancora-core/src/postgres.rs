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
