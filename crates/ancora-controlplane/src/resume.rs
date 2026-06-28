use crate::model::ResumeDecision;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResumeError {
    #[error("decision required")]
    DecisionRequired,
    #[error("reason required when rejecting")]
    ReasonRequiredOnReject,
}

pub fn validate_decision(decision: &ResumeDecision) -> Result<(), ResumeError> {
    if !decision.approved && decision.reason.is_none() {
        return Err(ResumeError::ReasonRequiredOnReject);
    }
    Ok(())
}
