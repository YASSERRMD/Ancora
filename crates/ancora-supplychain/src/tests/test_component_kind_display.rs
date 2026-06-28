#[cfg(test)]
mod tests {
    use crate::component::ComponentKind;

    #[test]
    fn test_library_display() {
        assert_eq!(format!("{}", ComponentKind::Library), "LIBRARY");
    }

    #[test]
    fn test_binary_display() {
        assert_eq!(format!("{}", ComponentKind::Binary), "BINARY");
    }

    #[test]
    fn test_container_display() {
        assert_eq!(format!("{}", ComponentKind::Container), "CONTAINER");
    }

    #[test]
    fn test_os_package_display() {
        assert_eq!(format!("{}", ComponentKind::OsPackage), "OS_PACKAGE");
    }

    #[test]
    fn test_framework_display() {
        assert_eq!(format!("{}", ComponentKind::Framework), "FRAMEWORK");
    }

    #[test]
    fn test_service_display() {
        assert_eq!(format!("{}", ComponentKind::Service), "SERVICE");
    }
}
