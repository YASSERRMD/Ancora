use crate::context::TenantContext;

/// Scopes a journal table name to the current tenant.
pub fn journal_table(ctx: &TenantContext) -> String {
    ctx.scope_key("journal")
}

/// Scopes a journal entry key (run_id) to the current tenant.
pub fn journal_key(ctx: &TenantContext, run_id: &str) -> String {
    ctx.scope_key(&format!("journal:{}", run_id))
}

/// Cost record key scoped to tenant.
pub fn cost_key(ctx: &TenantContext, run_id: &str) -> String {
    ctx.scope_key(&format!("cost:{}", run_id))
}
