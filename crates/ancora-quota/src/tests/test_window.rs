#[cfg(test)]
mod tests {
    use crate::SlidingWindow;

    #[test]
    fn window_resets_after_elapsed() {
        let mut w = SlidingWindow::new(60, 0);
        w.increment(0, 10);
        assert_eq!(w.value(0), 10);
        // Advance past the window
        assert_eq!(w.value(60), 0);
    }

    #[test]
    fn window_accumulates_within_period() {
        let mut w = SlidingWindow::new(60, 0);
        w.increment(0, 5);
        w.increment(30, 3);
        assert_eq!(w.value(30), 8);
    }

    #[test]
    fn seconds_until_reset_correct() {
        let w = SlidingWindow::new(60, 0);
        assert_eq!(w.seconds_until_reset(40), 20);
        assert_eq!(w.seconds_until_reset(60), 0);
    }
}
