use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Role {
    Viewer,
    Developer,
    Operator,
    Admin,
}

impl Role {
    pub fn precedence(&self) -> u8 {
        match self {
            Role::Viewer => 0,
            Role::Developer => 1,
            Role::Operator => 2,
            Role::Admin => 3,
        }
    }

    pub fn dominates(&self, other: &Role) -> bool {
        self.precedence() >= other.precedence()
    }

    pub fn all() -> Vec<Role> {
        vec![Role::Viewer, Role::Developer, Role::Operator, Role::Admin]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Viewer => "viewer",
            Role::Developer => "developer",
            Role::Operator => "operator",
            Role::Admin => "admin",
        }
    }
}
