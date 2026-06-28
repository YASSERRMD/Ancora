use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleDecision {
    ScaleUp { by: usize },
    ScaleDown { by: usize },
    NoOp { reason: String },
}

impl ScaleDecision {
    pub fn is_scale_up(&self) -> bool {
        matches!(self, ScaleDecision::ScaleUp { .. })
    }

    pub fn is_scale_down(&self) -> bool {
        matches!(self, ScaleDecision::ScaleDown { .. })
    }

    pub fn is_noop(&self) -> bool {
        matches!(self, ScaleDecision::NoOp { .. })
    }
}
