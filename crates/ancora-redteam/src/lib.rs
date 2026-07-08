//! ancora-redteam: adversarial red-team harness for scoring guardrail effectiveness.
//!
//! Provides: canonical attack scenario datasets (injection, tool-misuse, exfiltration,
//! privilege-escalation, jailbreak), effectiveness scoring, regression baseline,
//! and custom scenario authoring.

pub mod custom;
pub mod exfiltration;
pub mod injection;
pub mod jailbreak;
pub mod privilege;
pub mod regression;
pub mod scenario;
pub mod scorer;
pub mod tool_misuse;

pub use custom::ScenarioBuilder;
pub use exfiltration::exfiltration_scenarios;
pub use injection::injection_scenarios;
pub use jailbreak::jailbreak_scenarios;
pub use privilege::privilege_scenarios;
pub use regression::known_attack_regression_set;
pub use scenario::{AdversarialScenario, AttackCategory, ScenarioDataset};
pub use scorer::{EffectivenessReport, GuardrailScorer, ScenarioResult};
pub use tool_misuse::tool_misuse_scenarios;

#[cfg(test)]
mod tests;
