/// Trust policy for extension install.
///
/// Operators configure an `InstallPolicy` that gates extension installs based
/// on the computed trust score, required badges, and other signals. The policy
/// is evaluated before install proceeds.

use crate::badge::{BadgeKind, BadgeSet};
use crate::residency::{Region, ResidencyDeclaration, ResidencyError, enforce_residency};
use crate::trust_score::TrustScore;

/// The operating mode of the install policy.
#[derive(Debug, Clone, PartialEq)]
pub enum PolicyMode {
    /// Any extension may be installed; trust signals are informational only.
    Permissive,
    /// Extensions below the threshold trigger a warning but are allowed.
    Warn,
    /// Extensions below the threshold or missing required signals are blocked.
    Strict,
}

/// Install policy configuration.
#[derive(Debug, Clone)]
pub struct InstallPolicy {
    pub mode: PolicyMode,
    /// Minimum trust score required to install (used in Warn and Strict modes).
    pub min_trust_score: u32,
    /// Badges that must be present on the extension (used in Strict mode).
    pub required_badges: Vec<BadgeKind>,
    /// Allowed data residency regions; empty means all regions are allowed.
    pub allowed_regions: Vec<Region>,
}

impl Default for InstallPolicy {
    fn default() -> Self {
        InstallPolicy {
            mode: PolicyMode::Warn,
            min_trust_score: 50,
            required_badges: Vec::new(),
            allowed_regions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolicyVerdict {
    /// Install is permitted.
    Allow,
    /// Install is permitted but operator should be warned.
    Warn(Vec<String>),
    /// Install is blocked.
    Block(Vec<String>),
}

impl PolicyVerdict {
    pub fn is_allowed(&self) -> bool {
        matches!(self, PolicyVerdict::Allow | PolicyVerdict::Warn(_))
    }

    pub fn is_blocked(&self) -> bool {
        matches!(self, PolicyVerdict::Block(_))
    }
}

/// Evaluate the install policy against computed signals.
pub fn evaluate_policy(
    policy: &InstallPolicy,
    trust_score: &TrustScore,
    badges: &BadgeSet,
    residency: Option<&ResidencyDeclaration>,
) -> PolicyVerdict {
    let mut warnings: Vec<String> = Vec::new();
    let mut blocks: Vec<String> = Vec::new();

    // Check trust score threshold
    if trust_score.score < policy.min_trust_score {
        let msg = format!(
            "trust score {} is below minimum {}",
            trust_score.score, policy.min_trust_score
        );
        match policy.mode {
            PolicyMode::Permissive => {}
            PolicyMode::Warn => warnings.push(msg),
            PolicyMode::Strict => blocks.push(msg),
        }
    }

    // Check required badges (Strict mode only)
    if policy.mode == PolicyMode::Strict {
        for required in &policy.required_badges {
            if !badges.has(required) {
                blocks.push(format!("required badge '{}' is missing", required));
            }
        }
    }

    // Check residency
    if !policy.allowed_regions.is_empty() {
        match enforce_residency(residency, &policy.allowed_regions) {
            Ok(()) => {}
            Err(ResidencyError::DeclarationMissing) => {
                let msg = "residency declaration is missing".to_string();
                match policy.mode {
                    PolicyMode::Permissive => {}
                    PolicyMode::Warn => warnings.push(msg),
                    PolicyMode::Strict => blocks.push(msg),
                }
            }
            Err(e) => {
                let msg = e.to_string();
                match policy.mode {
                    PolicyMode::Permissive => {}
                    PolicyMode::Warn => warnings.push(msg),
                    PolicyMode::Strict => blocks.push(msg),
                }
            }
        }
    }

    if !blocks.is_empty() {
        PolicyVerdict::Block(blocks)
    } else if !warnings.is_empty() {
        PolicyVerdict::Warn(warnings)
    } else {
        PolicyVerdict::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust_score::{TrustScore, TrustBreakdown};

    fn low_score() -> TrustScore {
        TrustScore {
            score: 20,
            breakdown: TrustBreakdown {
                identity: 5,
                security: 0,
                license: 0,
                residency: 0,
                badges: 0,
                history: 0,
            },
        }
    }

    fn high_score() -> TrustScore {
        TrustScore {
            score: 90,
            breakdown: TrustBreakdown {
                identity: 20,
                security: 30,
                license: 15,
                residency: 15,
                badges: 10,
                history: 10,
            },
        }
    }

    #[test]
    fn strict_blocks_low_trust() {
        let policy = InstallPolicy {
            mode: PolicyMode::Strict,
            min_trust_score: 50,
            required_badges: Vec::new(),
            allowed_regions: Vec::new(),
        };
        let badges = BadgeSet::new();
        let verdict = evaluate_policy(&policy, &low_score(), &badges, None);
        assert!(verdict.is_blocked());
    }

    #[test]
    fn permissive_allows_low_trust() {
        let policy = InstallPolicy {
            mode: PolicyMode::Permissive,
            min_trust_score: 50,
            required_badges: Vec::new(),
            allowed_regions: Vec::new(),
        };
        let badges = BadgeSet::new();
        let verdict = evaluate_policy(&policy, &low_score(), &badges, None);
        assert!(verdict.is_allowed());
    }

    #[test]
    fn strict_allows_high_trust() {
        let policy = InstallPolicy {
            mode: PolicyMode::Strict,
            min_trust_score: 50,
            required_badges: Vec::new(),
            allowed_regions: Vec::new(),
        };
        let badges = BadgeSet::new();
        let verdict = evaluate_policy(&policy, &high_score(), &badges, None);
        assert!(verdict.is_allowed());
    }
}
