#[cfg(test)]
mod tests {
    use crate::{assert_compatible, Version};

    #[test]
    fn same_major_version_is_compatible() {
        let a = Version::new(1, 0, 0);
        let b = Version::new(1, 5, 3);
        assert!(assert_compatible(&a, &b).is_ok());
    }

    #[test]
    fn journal_incompatible_version_blocked() {
        let a = Version::new(1, 0, 0);
        let b = Version::new(2, 0, 0);
        let err = assert_compatible(&a, &b).unwrap_err();
        assert!(matches!(
            err,
            crate::DeployError::IncompatibleVersion { .. }
        ));
    }
}
