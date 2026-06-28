# Multi-Tenant RBAC

Role assignments are scoped to `(subject, tenant_id)`. The same user can hold
different roles in different tenants.

## Example

```rust
assignments.assign(RoleAssignment::new("alice", "tenant-a", Role::Admin, tick));
assignments.assign(RoleAssignment::new("alice", "tenant-b", Role::Viewer, tick));
```

Alice is an admin in `tenant-a` but a viewer in `tenant-b`. A permission check
for `alice` in `tenant-b` uses the Viewer role.

## Cross-tenant access prevention

`PermissionChecker::check` always requires both `subject` and `tenant_id`. If
`alice` has no assignment in `tenant-c`, all checks return `Deny`.

## Tenant administration

Only subjects with `Permission::TenantAdmin` (Admin role) may:
- Create or remove IdP configurations for a tenant
- Assign roles within a tenant
- Modify per-tenant quota limits
