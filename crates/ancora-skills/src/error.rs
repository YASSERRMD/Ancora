#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    NotFound(String),
    VersionNotFound(String, u32),
    PermissionDenied(String),
}

impl std::fmt::Display for SkillError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkillError::NotFound(n) => write!(f, "skill not found: {n}"),
            SkillError::VersionNotFound(n, v) => write!(f, "skill {n} v{v} not found"),
            SkillError::PermissionDenied(m) => write!(f, "permission denied: {m}"),
        }
    }
}
