//! # ancora-sync
//!
//! Offline sync and reconciliation for edge devices in the Ancora agent
//! framework.
//!
//! Edge devices operate autonomously when connectivity is unavailable.  When
//! connectivity is restored, ancora-sync synchronises local state to a central
//! hub using an idempotent, determinism-preserving protocol that respects data
//! residency requirements.
//!
//! ## Modules
//!
//! | Module | Responsibility |
//! |--------|---------------|
//! | [`model`] | Core types: `JournalEntry`, `SyncMarker`, `Conflict`, `ResidencyZone` |
//! | [`journal`] | Local-first append-only journal with sync markers |
//! | [`changelog`] | Change log recording offline mutations |
//! | [`protocol`] | Sync request / response protocol and in-memory Hub |
//! | [`conflict`] | Conflict detection and resolution policies |
//! | [`partial`] | Partial sync and resume across multiple round-trips |
//! | [`transport`] | Encrypted sync transport (envelope seal / open) |
//! | [`residency`] | Residency-aware filtering before data leaves the device |

pub mod model;
pub mod journal;
pub mod changelog;
pub mod protocol;
pub mod conflict;
pub mod partial;
pub mod transport;
pub mod residency;

#[cfg(test)]
mod tests;
