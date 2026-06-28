/// Conflict resolution policy between agents competing for a resource.
#[derive(Debug, Clone)]
pub enum ConflictPolicy {
    HighestPriority,
    FirstCome,
    Random(u64),
}

#[derive(Debug, Clone)]
pub struct Claim {
    pub agent_id: String,
    pub priority: u32,
    pub arrived_at: u64,
}

pub struct ConflictResolver;

impl ConflictResolver {
    pub fn resolve<'a>(claims: &'a [Claim], policy: &ConflictPolicy) -> Option<&'a Claim> {
        if claims.is_empty() { return None; }
        match policy {
            ConflictPolicy::HighestPriority => {
                claims.iter().max_by_key(|c| c.priority)
            }
            ConflictPolicy::FirstCome => {
                claims.iter().min_by_key(|c| c.arrived_at)
            }
            ConflictPolicy::Random(seed) => {
                let idx = (*seed as usize) % claims.len();
                claims.get(idx)
            }
        }
    }
}
