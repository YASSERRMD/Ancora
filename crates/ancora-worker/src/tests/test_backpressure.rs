#[cfg(test)]
mod tests {
    use crate::scheduler::{backpressure, Backpressure};

    #[test]
    fn backpressure_none_below_capacity() {
        assert_eq!(backpressure(3, 4, 2), Backpressure::None);
    }

    #[test]
    fn backpressure_soft_at_capacity() {
        assert_eq!(backpressure(8, 2, 4), Backpressure::Soft);
    }

    #[test]
    fn backpressure_hard_at_double_capacity() {
        assert_eq!(backpressure(16, 2, 4), Backpressure::Hard);
    }

    #[test]
    fn backpressure_hard_with_zero_workers() {
        assert_eq!(backpressure(5, 0, 4), Backpressure::Hard);
    }

    #[test]
    fn backpressure_hard_with_zero_concurrency() {
        assert_eq!(backpressure(1, 4, 0), Backpressure::Hard);
    }
}
