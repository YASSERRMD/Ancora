/// Trust and governance summary for the ecosystem milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct TrustDimension {
    pub name: &'static str,
    pub level: TrustLevel,
    pub evidence: &'static str,
}

impl TrustDimension {
    pub const fn new(name: &'static str, level: TrustLevel, evidence: &'static str) -> Self {
        Self { name, level, evidence }
    }

    pub fn is_high(&self) -> bool {
        self.level == TrustLevel::High
    }
}

pub fn trust_dimensions() -> Vec<TrustDimension> {
    vec![
        TrustDimension::new(
            "Plugin Sandboxing",
            TrustLevel::High,
            "WASM sandbox with capability-based permissions, no ambient authority",
        ),
        TrustDimension::new(
            "Supply Chain",
            TrustLevel::High,
            "All artifacts signed with ECDSA P-256; provenance tracked via SLSA level 2",
        ),
        TrustDimension::new(
            "Audit Logging",
            TrustLevel::High,
            "Tamper-evident append-only audit log with HMAC chain",
        ),
        TrustDimension::new(
            "Access Control",
            TrustLevel::High,
            "RBAC + ABAC enforced at every API boundary",
        ),
        TrustDimension::new(
            "Secret Management",
            TrustLevel::High,
            "Secrets stored in HSM-backed vault; never logged or serialized",
        ),
    ]
}

pub fn governance_score(dimensions: &[TrustDimension]) -> u8 {
    let high_count = dimensions.iter().filter(|d| d.is_high()).count();
    let pct = (high_count * 100) / dimensions.len().max(1);
    pct as u8
}
