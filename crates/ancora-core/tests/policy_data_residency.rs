// Policy: data residency -- run stays within allowed regions.

#[derive(Debug, PartialEq, Clone)]
enum Region {
    EuWest1,
    UsEast1,
    ApSoutheast1,
}

struct ResidencyPolicy {
    allowed: Vec<Region>,
}

impl ResidencyPolicy {
    fn new(allowed: Vec<Region>) -> Self { Self { allowed } }
    fn is_allowed(&self, region: &Region) -> bool { self.allowed.contains(region) }
    fn check(&self, region: &Region) -> Result<(), String> {
        if self.is_allowed(region) {
            Ok(())
        } else {
            Err(format!("region {:?} not in residency policy", region))
        }
    }
}

#[test]
fn test_allowed_region_passes() {
    let p = ResidencyPolicy::new(vec![Region::EuWest1]);
    assert!(p.check(&Region::EuWest1).is_ok());
}

#[test]
fn test_disallowed_region_fails() {
    let p = ResidencyPolicy::new(vec![Region::EuWest1]);
    let r = p.check(&Region::UsEast1);
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("UsEast1"));
}

#[test]
fn test_multi_region_policy_allows_any() {
    let p = ResidencyPolicy::new(vec![Region::EuWest1, Region::UsEast1]);
    assert!(p.check(&Region::EuWest1).is_ok());
    assert!(p.check(&Region::UsEast1).is_ok());
    assert!(p.check(&Region::ApSoutheast1).is_err());
}

#[test]
fn test_empty_policy_rejects_all() {
    let p = ResidencyPolicy::new(vec![]);
    assert!(p.check(&Region::EuWest1).is_err());
}

#[test]
fn test_single_region_allows_exact_match_only() {
    let p = ResidencyPolicy::new(vec![Region::ApSoutheast1]);
    assert!(p.is_allowed(&Region::ApSoutheast1));
    assert!(!p.is_allowed(&Region::EuWest1));
}

#[test]
fn test_error_message_names_region() {
    let p = ResidencyPolicy::new(vec![Region::EuWest1]);
    let err = p.check(&Region::ApSoutheast1).unwrap_err();
    assert!(err.contains("ApSoutheast1"));
}
