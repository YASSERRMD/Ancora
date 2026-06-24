use crate::error::AncoraError;

/// A single non-deterministic unit of work that must be recorded in the
/// journal on first execution and replayed from the journal on subsequent
/// executions of the same run.
///
/// All implementations must be idempotent with respect to their key: two
/// activities with the same key must produce the same result or the second
/// must be satisfied by the journaled result of the first.
pub trait Activity: Send + Sync {
    /// Execute the activity and return a JSON-encoded result string.
    fn execute(&self) -> Result<String, AncoraError>;

    /// A unique, stable key for this activity within a run.
    ///
    /// The key is used as the idempotency key in the journal. It must:
    /// - Be the same every time this activity would be executed in the same
    ///   position during a fresh run.
    /// - Be different from any other activity in the same run.
    fn key(&self) -> String;
}
