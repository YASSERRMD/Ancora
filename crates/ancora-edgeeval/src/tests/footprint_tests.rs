//! Additional footprint tests.

use crate::memory::{smallest_fitting, MemoryBudget};
use crate::runtime::MemoryFootprint;

#[test]
fn test_footprint_over_budget() {
    let fp = MemoryFootprint::new("big-model", 8 * 1024 * 1024 * 1024, 0, 0); // 8 GiB
    let budget = MemoryBudget::new("phone", 4096.0, 0.25); // 3 GiB available
    assert!(!budget.fits(&fp));
    assert!(budget.headroom_mib(&fp) < 0.0);
}

#[test]
fn test_smallest_fitting_selects_correctly() {
    let fp1 = MemoryFootprint::new("a", 256 * 1024 * 1024, 0, 0); // 256 MiB
    let fp2 = MemoryFootprint::new("b", 128 * 1024 * 1024, 0, 0); // 128 MiB
    let footprints = vec![("model-a".to_string(), fp1), ("model-b".to_string(), fp2)];
    let budget = MemoryBudget::new("device", 512.0, 0.0);
    let selected = smallest_fitting(&footprints, &budget);
    assert_eq!(selected, Some("model-b"));
}
