use crate::context::TenantContext;

/// Returns the vector collection name scoped to the current tenant.
/// Each tenant gets an isolated collection so cross-tenant similarity search is impossible.
pub fn vector_collection(ctx: &TenantContext, base_collection: &str) -> String {
    ctx.scope_key(&format!("vec:{}", base_collection))
}
