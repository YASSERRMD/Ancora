//! ancora-slm: Small-language-model first orchestration patterns.
//!
//! Provides replayable orchestration utilities tuned for small / local models:
//! - SLM-friendly prompt formatting
//! - Tool-call repair for weaker models
//! - Constrained decoding integration
//! - Schema-guided generation
//! - Step decomposition
//! - Verifier-heavy patterns
//! - Escalation to a larger model
//! - Few-shot injection
//! - Deterministic replay support

pub mod model;
pub mod prompt;
pub mod repair;
pub mod constrained;
pub mod schema;
pub mod decompose;
pub mod verifier;
pub mod escalate;
pub mod fewshot;
pub mod replay;

#[cfg(test)]
mod tests;

pub use model::{CompletionRequest, CompletionResponse, Message, ModelSpec, ModelTier, Role};
pub use prompt::{format_prompt, slm_system_prompt, PromptOptions, PromptStyle};
pub use repair::{repair_tool_call, RepairError, ToolCall};
pub use constrained::{run_constrained, validate_json_output, ConstrainedConfig, ConstrainedError};
pub use schema::{augment_prompt_with_schema, validate as validate_schema, Schema, ValidationError};
pub use decompose::{execute_plan, DecompositionPlan, OutputFormat, Step, StepResult};
pub use verifier::{
    run_verifiers, ContainsKeywordsVerifier, FnVerifier, LengthVerifier, NonEmptyVerifier,
    RequiredKeysVerifier, ValidJsonVerifier, Verdict, VerificationReport, Verifier,
};
pub use escalate::{can_escalate_to, run_with_escalation, EscalationPolicy, EscalationReason};
pub use fewshot::{inject_few_shots, FewShotExample, FewShotLibrary};
pub use replay::{make_replay_fn, ReplayModelFn, ReplayStore};
