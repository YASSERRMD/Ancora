//! Step decomposition: break a goal into ordered reasoning steps.

#[derive(Debug, Clone, PartialEq)]
pub enum StepStatus {
    Pending,
    Verified,
    Refuted,
    Abstained,
}

#[derive(Debug, Clone)]
pub struct ReasoningStep {
    pub index: usize,
    pub claim: String,
    pub status: StepStatus,
}

pub struct StepDecomposer;

impl StepDecomposer {
    pub fn decompose(claims: Vec<String>) -> Vec<ReasoningStep> {
        claims
            .into_iter()
            .enumerate()
            .map(|(i, claim)| ReasoningStep {
                index: i,
                claim,
                status: StepStatus::Pending,
            })
            .collect()
    }
}
