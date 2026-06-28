use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    AgentRead,
    AgentWrite,
    AgentDelete,
    AgentExecute,
    TaskRead,
    TaskWrite,
    TaskDelete,
    LogRead,
    LogWrite,
    SecretRead,
    SecretWrite,
    SecretDelete,
    PolicyRead,
    PolicyWrite,
    UserRead,
    UserWrite,
    UserDelete,
    RoleAssign,
    TenantAdmin,
    AuditRead,
}

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::AgentRead => "agent:read",
            Permission::AgentWrite => "agent:write",
            Permission::AgentDelete => "agent:delete",
            Permission::AgentExecute => "agent:execute",
            Permission::TaskRead => "task:read",
            Permission::TaskWrite => "task:write",
            Permission::TaskDelete => "task:delete",
            Permission::LogRead => "log:read",
            Permission::LogWrite => "log:write",
            Permission::SecretRead => "secret:read",
            Permission::SecretWrite => "secret:write",
            Permission::SecretDelete => "secret:delete",
            Permission::PolicyRead => "policy:read",
            Permission::PolicyWrite => "policy:write",
            Permission::UserRead => "user:read",
            Permission::UserWrite => "user:write",
            Permission::UserDelete => "user:delete",
            Permission::RoleAssign => "role:assign",
            Permission::TenantAdmin => "tenant:admin",
            Permission::AuditRead => "audit:read",
        }
    }
}
