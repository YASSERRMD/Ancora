//! ancora-redteam: adversarial red-team harness for scoring guardrail effectiveness.
//!
//! Provides: canonical attack scenario datasets (injection, tool-misuse, exfiltration,
//! privilege-escalation, jailbreak), effectiveness scoring, regression baseline,
//! and custom scenario authoring.

pub mod scenario;
pub mod injection;
pub mod tool_misuse;
pub mod exfiltration;
pub mod privilege;
pub mod jailbreak;
pub mod scorer;
pub mod regression;
pub mod custom;

pub use scenario::{AdversarialScenario, AttackCategory, ScenarioDataset};
pub use injection::injection_scenarios;
pub use tool_misuse::tool_misuse_scenarios;
pub use exfiltration::exfiltration_scenarios;
pub use privilege::privilege_scenarios;
pub use jailbreak::jailbreak_scenarios;
pub use scorer::{EffectivenessReport, GuardrailScorer, ScenarioResult};
pub use regression::known_attack_regression_set;
pub use custom::ScenarioBuilder;

#[cfg(test)]
mod tests;
