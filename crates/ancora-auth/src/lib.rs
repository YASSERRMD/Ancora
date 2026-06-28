pub mod audit;
pub mod idp;
pub mod introspect;
pub mod jwks;
pub mod jwt;
pub mod mfa;
pub mod oidc;
pub mod revocation;
pub mod saml;
pub mod service_account;
pub mod session;
pub mod token;

pub use audit::{AuthAuditLog, AuthEvent};
pub use idp::{IdpConfig, IdpKind, IdpRegistry};
pub use introspect::{IntrospectResult, IntrospectStatus, TokenIntrospector};
pub use jwks::{JwkKey, JwksStore};
pub use jwt::{JwtClaims, JwtError, JwtValidator};
pub use mfa::{MfaChallenge, MfaEnforcer, MfaMethod, MfaStatus};
pub use oidc::{MockOidcIdp, OidcAuthCode, OidcError};
pub use revocation::{revoke_all, RevocationStore};
pub use saml::{MockSamlIdp, SamlAssertion, SamlError};
pub use service_account::{ServiceAccount, ServiceAccountError, ServiceAccountRegistry};
pub use session::{Session, SessionState, SessionStore};
pub use token::{Token, TokenKind};

#[cfg(test)]
mod tests {
    mod test_oidc_flow;
    mod test_saml_flow;
    mod test_jwt_validation;
    mod test_jwks_rotation;
    mod test_token_introspect;
    mod test_session_mgmt;
    mod test_service_account;
    mod test_mfa_enforce;
    mod test_idp_config;
    mod test_revocation;
    mod test_token_expiry;
    mod test_mfa_required_oidc;
    mod test_mfa_required_saml;
    mod test_jwks_active_keys;
    mod test_multi_tenant_idp;
    mod test_service_account_disabled;
    mod test_service_account_wrong_key;
    mod test_saml_missing_signature;
    mod test_session_logout;
    mod test_revoked_token_rejected;
    mod test_oidc_mfa_required;
    mod test_bulk_revoke;
    mod test_audit_log;
}
