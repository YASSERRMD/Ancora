use crate::policy::NetworkPolicy;
use crate::rule::{Effect, NetworkRule, Protocol};

pub fn allow_https_only(tenant_id: impl Into<String>) -> NetworkPolicy {
    let mut policy = NetworkPolicy::deny_by_default(tenant_id);
    policy.add_rule(
        NetworkRule::new(
            "allow-https",
            "*",
            Some(443),
            Protocol::Tcp,
            Effect::Allow,
            100,
        )
        .with_description("allow all outbound HTTPS"),
    );
    policy
}

pub fn allow_internal_only(
    tenant_id: impl Into<String>,
    internal_domain: impl Into<String>,
) -> NetworkPolicy {
    let mut policy = NetworkPolicy::deny_by_default(tenant_id);
    let domain = format!("*.{}", internal_domain.into());
    policy.add_rule(
        NetworkRule::new(
            "allow-internal",
            domain,
            None,
            Protocol::Any,
            Effect::Allow,
            100,
        )
        .with_description("allow internal domain traffic"),
    );
    policy
}

pub fn block_known_bad(policy: &mut NetworkPolicy, bad_host: impl Into<String>) {
    let host = bad_host.into();
    let id = format!("deny-{}", host.replace('.', "-"));
    policy.add_rule(
        NetworkRule::new(id, host, None, Protocol::Any, Effect::Deny, 10)
            .with_description("blocked host"),
    );
}
