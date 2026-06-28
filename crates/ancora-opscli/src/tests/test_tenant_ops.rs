#[cfg(test)]
mod tests {
    use crate::tenant_ops::{TenantOps, TenantState};

    #[test]
    fn tenant_create_and_suspend() {
        let mut ops = TenantOps::default();
        ops.create("tenant-a");
        assert_eq!(ops.get("tenant-a").unwrap().state, TenantState::Active);
        ops.suspend("tenant-a");
        assert_eq!(ops.get("tenant-a").unwrap().state, TenantState::Suspended);
    }

    #[test]
    fn suspend_unknown_tenant_returns_false() {
        let mut ops = TenantOps::default();
        assert!(!ops.suspend("unknown"));
    }

    #[test]
    fn list_tenants_sorted() {
        let mut ops = TenantOps::default();
        ops.create("zzz");
        ops.create("aaa");
        let list = ops.list();
        assert_eq!(list[0].tenant_id, "aaa");
        assert_eq!(list[1].tenant_id, "zzz");
    }
}
