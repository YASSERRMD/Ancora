use crate::policy::ZeroTrustPolicy;
use crate::device::TrustLevel;

pub fn strict_policy(tenant_id: impl Into<String>) -> ZeroTrustPolicy {
    ZeroTrustPolicy::new(tenant_id)
        .require_device()
        .min_trust(TrustLevel::FullyTrusted)
        .mfa_for_group("admin")
        .mfa_for_group("finance")
}

pub fn standard_policy(tenant_id: impl Into<String>) -> ZeroTrustPolicy {
    ZeroTrustPolicy::new(tenant_id)
        .require_device()
        .min_trust(TrustLevel::Trusted)
        .mfa_for_group("admin")
}

pub fn permissive_policy(tenant_id: impl Into<String>) -> ZeroTrustPolicy {
    ZeroTrustPolicy::new(tenant_id)
}
