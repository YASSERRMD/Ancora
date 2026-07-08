use crate::error::OrchestrateError;

/// Enforces a maximum spawn depth to prevent infinite agent recursion.
pub struct DepthLimiter {
    pub max_depth: usize,
    current: usize,
}

impl DepthLimiter {
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            current: 0,
        }
    }

    pub fn enter(&mut self) -> Result<(), OrchestrateError> {
        if self.current >= self.max_depth {
            return Err(OrchestrateError::MaxDepthExceeded {
                depth: self.current,
            });
        }
        self.current += 1;
        Ok(())
    }

    pub fn exit(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn depth(&self) -> usize {
        self.current
    }
}
