use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CostError {
    #[error("hard cap exceeded: budget={budget:.4} usd, spent={spent:.4} usd")]
    HardCapExceeded { budget: f64, spent: f64 },

    #[error("soft cap warning: spent {pct:.1}% of budget")]
    SoftCapWarning { pct: f64 },
}
