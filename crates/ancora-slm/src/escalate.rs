//! Escalation to a larger local model.
//!
//! When the SLM fails to produce valid output within the configured retry
//! budget, the orchestrator can escalate to a larger (but slower / more
//! expensive) model.  This module provides the escalation policy and the
//! routing logic that decides when escalation is warranted.

use crate::model::{ModelSpec, ModelTier};
use crate::verifier::{run_verifiers, Verifier};

/// Policy controlling when the orchestrator escalates.
#[derive(Debug, Clone)]
pub struct EscalationPolicy {
    /// Maximum attempts with the primary (small) model before escalating.
    pub max_slm_attempts: usize,
    /// Minimum tier required for the escalation model.
    pub escalation_tier: ModelTier,
}

impl Default for EscalationPolicy {
    fn default() -> Self {
        Self {
            max_slm_attempts: 2,
            escalation_tier: ModelTier::Large,
        }
    }
}

/// The reason an escalation was triggered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscalationReason {
    /// Primary model failed all retry attempts.
    RetryBudgetExceeded,
    /// Verifier rejected the primary model's output.
    VerifierRejected(String),
}

/// The result of an orchestration run that may have involved escalation.
#[derive(Debug, Clone)]
pub struct OrchestrationResult {
    /// The final raw response text.
    pub text: String,
    /// Which model ultimately produced the response.
    pub model_id: String,
    /// Whether escalation was triggered.
    pub escalated: bool,
    /// If escalated, the reason.
    pub escalation_reason: Option<EscalationReason>,
    /// Number of attempts consumed on the primary model.
    pub slm_attempts: usize,
}

/// Run a request with escalation support.
///
/// `slm_fn` is called with the prompt and returns a raw response string for
/// the primary (small) model.  `llm_fn` is called only if escalation occurs.
/// Both functions are deterministic when given the same inputs, ensuring
/// replay.
pub fn run_with_escalation<S, L>(
    prompt: &str,
    policy: &EscalationPolicy,
    verifiers: &[Box<dyn Verifier>],
    slm_fn: S,
    llm_fn: L,
    slm_model_id: &str,
    llm_model_id: &str,
) -> OrchestrationResult
where
    S: Fn(&str) -> String,
    L: Fn(&str) -> String,
{
    let mut slm_attempts = 0;

    for _ in 0..policy.max_slm_attempts {
        let raw = slm_fn(prompt);
        slm_attempts += 1;
        let report = run_verifiers(&raw, verifiers);
        if report.passed() {
            return OrchestrationResult {
                text: raw,
                model_id: slm_model_id.to_string(),
                escalated: false,
                escalation_reason: None,
                slm_attempts,
            };
        }

        // Capture first failure reason for the escalation record.
        if let Some((name, _)) = report.checks.iter().find(|(_, v)| v.is_fail()) {
            let _ = name; // used below
        }
    }

    // All SLM attempts exhausted — escalate.
    let last_slm_raw = slm_fn(prompt);
    let report = run_verifiers(&last_slm_raw, verifiers);
    let fail_reason = report
        .checks
        .iter()
        .find(|(_, v)| v.is_fail())
        .map(|(name, _)| name.clone())
        .unwrap_or_else(|| "unknown".into());

    let llm_raw = llm_fn(prompt);

    OrchestrationResult {
        text: llm_raw,
        model_id: llm_model_id.to_string(),
        escalated: true,
        escalation_reason: Some(EscalationReason::VerifierRejected(fail_reason)),
        slm_attempts,
    }
}

/// Decide whether a given [`ModelSpec`] meets the escalation tier requirement.
pub fn can_escalate_to(candidate: &ModelSpec, policy: &EscalationPolicy) -> bool {
    candidate.tier >= policy.escalation_tier
}
