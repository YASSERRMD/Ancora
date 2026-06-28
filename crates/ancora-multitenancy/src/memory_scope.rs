use crate::context::TenantContext;

/// Scopes an in-memory store namespace to the current tenant.
pub fn memory_namespace(ctx: &TenantContext) -> String {
    ctx.scope_key("memory")
}

/// Scopes a specific memory key to the current tenant.
pub fn memory_key(ctx: &TenantContext, key: &str) -> String {
    ctx.scope_key(&format!("memory:{}", key))
}
