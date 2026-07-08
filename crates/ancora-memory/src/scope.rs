/// Identifies which resource and conversation thread a memory entry belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Scope {
    pub resource_id: String,
    pub thread_id: String,
}

impl Scope {
    pub fn new(resource_id: impl Into<String>, thread_id: impl Into<String>) -> Self {
        Self {
            resource_id: resource_id.into(),
            thread_id: thread_id.into(),
        }
    }
}
