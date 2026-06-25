use crate::error::PolicyError;
use crate::policy::Policy;

/// Check that `endpoint` is permitted by the policy.
///
/// When `air_gapped` is set, every egress call is blocked regardless of
/// the `allowed_endpoints` list.
pub fn check_endpoint(policy: &Policy, endpoint: &str) -> Result<(), PolicyError> {
    if policy.air_gapped {
        return Err(PolicyError::EgressBlocked(endpoint.to_owned()));
    }
    if policy.allowed_endpoints.is_empty() {
        return Ok(());
    }
    let allowed = policy.allowed_endpoints.iter()
        .any(|allowed| endpoint.starts_with(allowed.as_str()));
    if !allowed {
        return Err(PolicyError::ResidencyViolation(endpoint.to_owned()));
    }
    Ok(())
}

/// Check that `tool_name` is permitted by the policy.
pub fn check_tool(policy: &Policy, tool_name: &str) -> Result<(), PolicyError> {
    if policy.allowed_tools.is_empty() {
        return Ok(());
    }
    if !policy.allowed_tools.contains(tool_name) {
        return Err(PolicyError::PermissionDenied(
            format!("tool '{tool_name}' is not in the allowed list")
        ));
    }
    Ok(())
}

/// Check whether an audit is required for the action.
pub fn check_audit_required(policy: &Policy, action: &str) -> Result<(), PolicyError> {
    if policy.require_audit {
        return Err(PolicyError::AuditRequired(action.to_owned()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn residency_violation_blocks_disallowed_endpoint() {
        let policy = Policy::new().allow_endpoint("https://eu.api.example.com");
        let err = check_endpoint(&policy, "https://us.api.example.com/v1/chat").unwrap_err();
        assert!(matches!(err, PolicyError::ResidencyViolation(_)));
    }

    #[test]
    fn allowed_endpoint_passes_check() {
        let policy = Policy::new().allow_endpoint("https://eu.api.example.com");
        assert!(check_endpoint(&policy, "https://eu.api.example.com/v1/chat").is_ok());
    }
}
