/// Represents the lifecycle state of an extension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleState {
    /// Extension is registered but not yet loaded.
    Registered,
    /// Extension has been loaded and is initializing.
    Loading,
    /// Extension is active and fully operational.
    Active,
    /// Extension has been deprecated but still runs.
    Deprecated,
    /// Extension is being unloaded gracefully.
    Unloading,
    /// Extension has been unloaded and is no longer operational.
    Unloaded,
    /// Extension encountered an error and is in a failed state.
    Failed(String),
}

impl LifecycleState {
    /// Returns true if this state permits a transition to `next`.
    pub fn can_transition_to(&self, next: &LifecycleState) -> bool {
        use LifecycleState::*;
        matches!(
            (self, next),
            (Registered, Loading)
                | (Loading, Active)
                | (Loading, Failed(_))
                | (Active, Deprecated)
                | (Active, Unloading)
                | (Deprecated, Unloading)
                | (Unloading, Unloaded)
                | (Active, Failed(_))
        )
    }
}

/// Extension lifecycle controller.
pub struct ExtensionLifecycle {
    pub id: String,
    pub state: LifecycleState,
}

impl ExtensionLifecycle {
    pub fn new(id: impl Into<String>) -> Self {
        ExtensionLifecycle {
            id: id.into(),
            state: LifecycleState::Registered,
        }
    }

    /// Attempt to transition to the given state.
    /// Returns Ok(()) on success, Err with the rejected transition on failure.
    pub fn transition(&mut self, next: LifecycleState) -> Result<(), String> {
        if self.state.can_transition_to(&next) {
            self.state = next;
            Ok(())
        } else {
            Err(format!(
                "invalid transition from {:?} to {:?}",
                self.state, next
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_load_sequence() {
        let mut ext = ExtensionLifecycle::new("my-ext");
        ext.transition(LifecycleState::Loading).unwrap();
        ext.transition(LifecycleState::Active).unwrap();
        assert_eq!(ext.state, LifecycleState::Active);
    }

    #[test]
    fn invalid_transition_rejected() {
        let mut ext = ExtensionLifecycle::new("my-ext");
        // Cannot go directly from Registered to Active
        let result = ext.transition(LifecycleState::Active);
        assert!(result.is_err());
    }
}
