use crate::semver::SemVer;

/// Marks an API point as deprecated.
#[derive(Debug, Clone)]
pub struct DeprecationMarker {
    /// The version in which the deprecation was first announced.
    pub since: SemVer,
    /// The version in which the item will be removed.
    pub removed_in: SemVer,
    /// A human-readable reason and migration hint.
    pub message: String,
}

impl DeprecationMarker {
    pub fn new(since: SemVer, removed_in: SemVer, message: impl Into<String>) -> Self {
        DeprecationMarker {
            since,
            removed_in,
            message: message.into(),
        }
    }

    /// Returns true if the item is currently deprecated under the given version.
    pub fn is_active_at(&self, current: &SemVer) -> bool {
        current >= &self.since && current < &self.removed_in
    }

    /// Returns true if the item has been removed at the given version.
    pub fn is_removed_at(&self, current: &SemVer) -> bool {
        current >= &self.removed_in
    }
}

/// A warning produced when a deprecated API is used.
#[derive(Debug, Clone)]
pub struct DeprecationWarning {
    pub api_name: String,
    pub marker: DeprecationMarker,
}

impl DeprecationWarning {
    pub fn format(&self) -> String {
        format!(
            "DEPRECATED: `{}` is deprecated since {} and will be removed in {}. {}",
            self.api_name,
            self.marker.since,
            self.marker.removed_in,
            self.marker.message
        )
    }
}

/// Check an extension usage against the deprecation registry.
/// Returns a warning if the API is deprecated at the current version.
pub fn check_deprecation(
    api_name: &str,
    marker: &DeprecationMarker,
    current_version: &SemVer,
) -> Option<DeprecationWarning> {
    if marker.is_active_at(current_version) {
        Some(DeprecationWarning {
            api_name: api_name.to_string(),
            marker: marker.clone(),
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warning_emitted_during_deprecation_window() {
        let marker = DeprecationMarker::new(
            SemVer::new(1, 2, 0),
            SemVer::new(2, 0, 0),
            "Use new_api instead",
        );
        let current = SemVer::new(1, 4, 0);
        let warning = check_deprecation("old_hook", &marker, &current);
        assert!(warning.is_some());
        let w = warning.unwrap();
        assert!(w.format().contains("old_hook"));
        assert!(w.format().contains("deprecated"));
    }

    #[test]
    fn no_warning_before_deprecation() {
        let marker = DeprecationMarker::new(
            SemVer::new(1, 5, 0),
            SemVer::new(2, 0, 0),
            "Use new_api instead",
        );
        let current = SemVer::new(1, 3, 0);
        assert!(check_deprecation("old_hook", &marker, &current).is_none());
    }
}
