#[cfg(test)]
mod tests {
    use crate::bounds::ScaleBounds;

    #[test]
    fn clamp_below_min_returns_min() {
        let b = ScaleBounds::new(2, 8);
        assert_eq!(b.clamp(0), 2);
    }

    #[test]
    fn clamp_above_max_returns_max() {
        let b = ScaleBounds::new(2, 8);
        assert_eq!(b.clamp(100), 8);
    }

    #[test]
    fn clamp_within_range_returns_same() {
        let b = ScaleBounds::new(2, 8);
        assert_eq!(b.clamp(5), 5);
    }

    #[test]
    fn at_max_detection() {
        let b = ScaleBounds::new(1, 5);
        assert!(b.at_max(5));
        assert!(!b.at_max(4));
    }

    #[test]
    fn at_min_detection() {
        let b = ScaleBounds::new(1, 5);
        assert!(b.at_min(1));
        assert!(!b.at_min(2));
    }
}
