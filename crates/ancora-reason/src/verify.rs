//! Intermediate verification: check each reasoning step against a checker function.

use crate::decompose::{ReasoningStep, StepStatus};

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub step_index: usize,
    pub passed: bool,
    pub reason: String,
}

pub struct StepVerifier;

impl StepVerifier {
    pub fn verify<F: Fn(&str) -> bool>(step: &mut ReasoningStep, checker: F) -> VerificationResult {
        let passed = checker(&step.claim);
        step.status = if passed {
            StepStatus::Verified
        } else {
            StepStatus::Refuted
        };
        VerificationResult {
            step_index: step.index,
            passed,
            reason: if passed {
                "verified".into()
            } else {
                "refuted by checker".into()
            },
        }
    }
}
