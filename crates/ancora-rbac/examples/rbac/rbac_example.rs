use ancora_rbac::{AssignmentStore, Permission, PermissionChecker, Role, RoleAssignment, RolePolicy};

fn main() {
    let mut assignments = AssignmentStore::new();
    assignments.assign(RoleAssignment::new("alice", "acme", Role::Developer, 0));
    assignments.assign(RoleAssignment::new("bob", "acme", Role::Admin, 0));
    assignments.assign(RoleAssignment::new("carol", "acme", Role::Viewer, 0));

    let policy = RolePolicy::new();
    let checker = PermissionChecker::new(&assignments, &policy);

    for (user, perm) in [
        ("alice", Permission::AgentExecute),
        ("alice", Permission::RoleAssign),
        ("bob", Permission::TenantAdmin),
        ("carol", Permission::AgentRead),
        ("carol", Permission::AgentWrite),
    ] {
        let decision = checker.check(user, "acme", &perm);
        println!("{user} + {}: {:?}", perm.as_str(), decision);
    }
}
