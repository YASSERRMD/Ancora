use crate::significance::{is_significant, SampleStats};

#[test]
fn tiny_delta_with_high_variance_is_not_significant() {
    // baseline mean=0.90, candidate mean=0.89 with very high std dev and small n
    let baseline = SampleStats::new(3, 0.90, 0.15).unwrap();
    let candidate = SampleStats::new(3, 0.89, 0.15).unwrap();
    assert!(
        !is_significant(&baseline, &candidate, 0.05),
        "tiny delta with high variance should not be significant"
    );
}

#[test]
fn zero_delta_never_significant() {
    let baseline = SampleStats::new(10, 0.80, 0.05).unwrap();
    let candidate = SampleStats::new(10, 0.80, 0.05).unwrap();
    assert!(!is_significant(&baseline, &candidate, 0.05));
}
