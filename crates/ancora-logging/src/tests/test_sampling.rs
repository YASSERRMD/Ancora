#[cfg(test)]
mod tests {
    use crate::sampling::Sampler;

    #[test]
    fn sampling_reduces_volume() {
        let mut s = Sampler::new(10);
        let total = 100;
        let sampled = (0..total).filter(|_| s.should_sample()).count();
        assert_eq!(sampled, 10, "expected 10% of 100 = 10 samples");
    }

    #[test]
    fn rate_1_samples_all() {
        let mut s = Sampler::new(1);
        for _ in 0..5 {
            assert!(s.should_sample());
        }
    }

    #[test]
    fn sampler_reset_restarts_counter() {
        let mut s = Sampler::new(2);
        s.should_sample(); // counter=1
        s.reset();
        // after reset, next call is counter=1 again, not sampled for rate=2
        assert!(!s.should_sample()); // counter=1, 1%2!=0
        assert!(s.should_sample()); // counter=2, 2%2==0
    }
}
