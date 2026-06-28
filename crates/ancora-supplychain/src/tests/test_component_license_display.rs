#[cfg(test)]
mod tests {
    use crate::component::License;

    #[test]
    fn test_mit_display() {
        assert_eq!(format!("{}", License::Mit), "MIT");
    }

    #[test]
    fn test_apache2_display() {
        assert_eq!(format!("{}", License::Apache2), "Apache-2.0");
    }

    #[test]
    fn test_gpl3_display() {
        assert_eq!(format!("{}", License::Gpl3), "GPL-3.0");
    }

    #[test]
    fn test_bsd2_display() {
        assert_eq!(format!("{}", License::Bsd2), "BSD-2-Clause");
    }

    #[test]
    fn test_bsd3_display() {
        assert_eq!(format!("{}", License::Bsd3), "BSD-3-Clause");
    }

    #[test]
    fn test_proprietary_display() {
        assert_eq!(format!("{}", License::Proprietary), "PROPRIETARY");
    }

    #[test]
    fn test_unknown_display() {
        assert_eq!(format!("{}", License::Unknown), "UNKNOWN");
    }
}
