use crate::device::{DevicePosture, DeviceStore};
use crate::identity::Identity;
use crate::policy::{AuthzDecision, ZeroTrustPolicy};
use crate::request::AccessRequest;

pub struct ZeroTrustEvaluator;

impl ZeroTrustEvaluator {
    pub fn evaluate(
        policy: &ZeroTrustPolicy,
        request: &AccessRequest,
        identity: &Identity,
        devices: &DeviceStore,
    ) -> AuthzDecision {
        if !identity.is_active() {
            return AuthzDecision::Deny("identity is not active".into());
        }

        if policy.resource_denied(&request.resource) {
            return AuthzDecision::Deny(format!(
                "resource {} is denied by policy",
                request.resource
            ));
        }

        if policy.require_device_trust {
            match request.device_id.as_deref().and_then(|id| devices.get(id)) {
                None => return AuthzDecision::Deny("no trusted device presented".into()),
                Some(d) if d.trust_level < policy.min_device_trust => {
                    return AuthzDecision::Deny(format!(
                        "device trust {} below minimum {:?}",
                        d.trust_level, policy.min_device_trust
                    ));
                }
                _ => {}
            }
        }

        if policy.needs_mfa_for(&identity.groups) {
            return AuthzDecision::RequireMfa;
        }

        AuthzDecision::Allow
    }
}
