/// Maintainer information for an extension.
#[derive(Debug, Clone)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
    pub github_handle: String,
}

impl Maintainer {
    pub fn new(
        name: impl Into<String>,
        email: impl Into<String>,
        github_handle: impl Into<String>,
    ) -> Self {
        Maintainer {
            name: name.into(),
            email: email.into(),
            github_handle: github_handle.into(),
        }
    }
}

/// Ownership record for an extension.
#[derive(Debug, Clone)]
pub struct ExtensionOwnership {
    pub extension_id: String,
    pub maintainers: Vec<Maintainer>,
    pub lead: Option<String>,
}

impl ExtensionOwnership {
    pub fn new(extension_id: impl Into<String>) -> Self {
        ExtensionOwnership {
            extension_id: extension_id.into(),
            maintainers: Vec::new(),
            lead: None,
        }
    }

    /// Add a maintainer.
    pub fn add_maintainer(&mut self, maintainer: Maintainer) {
        self.maintainers.push(maintainer);
    }

    /// Set the lead maintainer by GitHub handle.
    pub fn set_lead(&mut self, github_handle: impl Into<String>) {
        self.lead = Some(github_handle.into());
    }

    /// Returns true if the given GitHub handle is a maintainer.
    pub fn is_maintainer(&self, github_handle: &str) -> bool {
        self.maintainers
            .iter()
            .any(|m| m.github_handle == github_handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maintainer_lookup_works() {
        let mut ownership = ExtensionOwnership::new("my-ext");
        ownership.add_maintainer(Maintainer::new("Alice", "alice@example.com", "alice-gh"));
        assert!(ownership.is_maintainer("alice-gh"));
        assert!(!ownership.is_maintainer("bob-gh"));
    }
}
