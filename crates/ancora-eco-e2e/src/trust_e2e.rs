//! Trust end-to-end: trust policy enforcement for plugin installation.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    Untrusted = 0,
    Community = 1,
    Verified = 2,
    Official = 3,
}

impl TrustLevel {
    pub fn parse_str(s: &str) -> Option<TrustLevel> {
        match s {
            "untrusted" => Some(TrustLevel::Untrusted),
            "community" => Some(TrustLevel::Community),
            "verified" => Some(TrustLevel::Verified),
            "official" => Some(TrustLevel::Official),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TrustLevel::Untrusted => "untrusted",
            TrustLevel::Community => "community",
            TrustLevel::Verified => "verified",
            TrustLevel::Official => "official",
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrustPolicy {
    pub minimum_level: TrustLevel,
    pub allow_unverified_publishers: bool,
    pub require_checksum: bool,
}

impl TrustPolicy {
    pub fn new(minimum_level: TrustLevel) -> Self {
        TrustPolicy {
            minimum_level,
            allow_unverified_publishers: false,
            require_checksum: true,
        }
    }

    pub fn strict() -> Self {
        TrustPolicy {
            minimum_level: TrustLevel::Official,
            allow_unverified_publishers: false,
            require_checksum: true,
        }
    }

    pub fn permissive() -> Self {
        TrustPolicy {
            minimum_level: TrustLevel::Community,
            allow_unverified_publishers: true,
            require_checksum: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub trust_level: TrustLevel,
    pub publisher_verified: bool,
    pub checksum: Option<String>,
}

impl PluginManifest {
    pub fn new(
        name: &str,
        trust_level: TrustLevel,
        publisher_verified: bool,
        checksum: Option<&str>,
    ) -> Self {
        PluginManifest {
            name: name.to_string(),
            trust_level,
            publisher_verified,
            checksum: checksum.map(|s| s.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct TrustGate {
    pub policy: TrustPolicy,
}

impl TrustGate {
    pub fn new(policy: TrustPolicy) -> Self {
        TrustGate { policy }
    }

    pub fn check(&self, manifest: &PluginManifest) -> Result<(), String> {
        if manifest.trust_level < self.policy.minimum_level {
            return Err(format!(
                "plugin '{}' trust level '{}' is below required '{}'",
                manifest.name,
                manifest.trust_level.as_str(),
                self.policy.minimum_level.as_str()
            ));
        }
        if !self.policy.allow_unverified_publishers && !manifest.publisher_verified {
            return Err(format!(
                "plugin '{}' publisher is not verified",
                manifest.name
            ));
        }
        if self.policy.require_checksum && manifest.checksum.is_none() {
            return Err(format!(
                "plugin '{}' missing required checksum",
                manifest.name
            ));
        }
        Ok(())
    }
}
