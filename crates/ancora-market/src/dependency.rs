/// Dependency declaration for marketplace extensions.
///
/// Extensions may depend on other extensions or well-known runtime capabilities.
/// The dependency graph is validated before publishing and on install.

use crate::versioning::SemVer;

#[derive(Debug, Clone, PartialEq)]
pub struct DependencySpec {
    /// Extension ID of the dependency.
    pub id: String,
    /// Minimum required version (inclusive).
    pub min_version: SemVer,
    /// Maximum allowed version (exclusive), if any.
    pub max_version: Option<SemVer>,
    /// Whether this dependency is optional (feature-gated).
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DependencyError {
    EmptyId,
    InvalidVersionRange { min: SemVer, max: SemVer },
    CircularDependency(String),
    DuplicateDependency(String),
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyError::EmptyId => write!(f, "dependency id must not be empty"),
            DependencyError::InvalidVersionRange { min, max } => {
                write!(f, "min version {} >= max version {}", min, max)
            }
            DependencyError::CircularDependency(id) => {
                write!(f, "circular dependency detected for '{}'", id)
            }
            DependencyError::DuplicateDependency(id) => {
                write!(f, "dependency '{}' declared more than once", id)
            }
        }
    }
}

impl DependencySpec {
    pub fn new(id: impl Into<String>, min_version: SemVer) -> Result<Self, DependencyError> {
        let id = id.into();
        if id.is_empty() {
            return Err(DependencyError::EmptyId);
        }
        Ok(DependencySpec {
            id,
            min_version,
            max_version: None,
            optional: false,
        })
    }

    pub fn with_max(mut self, max_version: SemVer) -> Result<Self, DependencyError> {
        if max_version <= self.min_version {
            return Err(DependencyError::InvalidVersionRange {
                min: self.min_version.clone(),
                max: max_version,
            });
        }
        self.max_version = Some(max_version);
        Ok(self)
    }

    /// Check whether a given version satisfies this dependency spec.
    pub fn is_satisfied_by(&self, version: &SemVer) -> bool {
        if version < &self.min_version {
            return false;
        }
        if let Some(max) = &self.max_version {
            if version >= max {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct DependencyList {
    deps: Vec<DependencySpec>,
}

impl DependencyList {
    pub fn new() -> Self {
        DependencyList { deps: Vec::new() }
    }

    pub fn add(&mut self, dep: DependencySpec) -> Result<(), DependencyError> {
        if self.deps.iter().any(|d| d.id == dep.id) {
            return Err(DependencyError::DuplicateDependency(dep.id));
        }
        self.deps.push(dep);
        Ok(())
    }

    pub fn all(&self) -> &[DependencySpec] {
        &self.deps
    }

    pub fn len(&self) -> usize {
        self.deps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.deps.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_satisfies_range() {
        let dep = DependencySpec::new("com.example.dep", SemVer::parse("1.0.0").unwrap())
            .unwrap()
            .with_max(SemVer::parse("2.0.0").unwrap())
            .unwrap();
        assert!(dep.is_satisfied_by(&SemVer::parse("1.5.0").unwrap()));
        assert!(!dep.is_satisfied_by(&SemVer::parse("2.0.0").unwrap()));
        assert!(!dep.is_satisfied_by(&SemVer::parse("0.9.0").unwrap()));
    }

    #[test]
    fn duplicate_dep_rejected() {
        let mut list = DependencyList::new();
        let dep1 = DependencySpec::new("com.example.dep", SemVer::parse("1.0.0").unwrap()).unwrap();
        let dep2 = DependencySpec::new("com.example.dep", SemVer::parse("1.0.0").unwrap()).unwrap();
        list.add(dep1).unwrap();
        assert!(matches!(list.add(dep2), Err(DependencyError::DuplicateDependency(_))));
    }
}
