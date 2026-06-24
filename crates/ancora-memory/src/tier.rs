/// The four memory tiers, from most to least immediate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryTier {
    /// Short-term context active in the current reasoning step.
    Working,
    /// Past interactions and experiences within a session.
    Episodic,
    /// Facts and knowledge that persist across sessions.
    Semantic,
    /// Long-term cold storage for rarely accessed information.
    Archival,
}
