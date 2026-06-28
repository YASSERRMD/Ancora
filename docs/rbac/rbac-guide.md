# RBAC Guide

## Roles

Four roles in ascending order of privilege:

| Role | Key permissions |
|---|---|
| `Viewer` | read-only across all resources |
| `Developer` | write agents and tasks, execute agents |
| `Operator` | all developer permissions plus secret management and policy write |
| `Admin` | full control including role assignment and tenant administration |

## Quick start

```rust
use ancora_rbac::{AssignmentStore, Permission, PermissionChecker, Role, RoleAssignment, RolePolicy};

let mut assignments = AssignmentStore::new();
assignments.assign(RoleAssignment::new("alice", "acme-corp", Role::Developer, current_tick));

let policy = RolePolicy::new();
let checker = PermissionChecker::new(&assignments, &policy);

if checker.is_allowed("alice", "acme-corp", &Permission::AgentExecute) {
    // proceed
}
```

## Role inheritance

Each role inherits all permissions of lower roles: Admin > Operator > Developer > Viewer.

## Custom policy overrides

```rust
let mut policy = RolePolicy::new();
policy.grant(Role::Viewer, Permission::SecretRead);
```

Overrides add extra permissions; they do not remove base permissions.

## Multi-tenant isolation

Role assignments are scoped to a `(subject, tenant_id)` pair. An admin in
`tenant-a` has no privileges in `tenant-b`.
