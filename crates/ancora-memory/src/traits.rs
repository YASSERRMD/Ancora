use crate::entry::MemoryEntry;
use crate::scope::Scope;
use crate::tier::MemoryTier;

/// Core memory interface scoped to a resource and thread.
pub trait Memory: Send + Sync {
    /// Write an entry into memory under the given scope.
    fn write(&self, scope: &Scope, entry: MemoryEntry);

    /// Read all entries for a scope, optionally filtered to a specific tier.
    fn read(&self, scope: &Scope, tier: Option<MemoryTier>) -> Vec<MemoryEntry>;
}
