#[cfg(test)]
mod tests {
    use crate::forecast::CostForecaster;

    #[test]
    fn forecast_within_tolerance() {
        let f = CostForecaster::new(vec![1.0, 2.0, 3.0]);
        let predicted = f.forecast(7);
        // avg=2.0 * 7 = 14.0
        assert!((predicted - 14.0).abs() < 1e-9);
    }

    #[test]
    fn forecast_empty_returns_zero() {
        let f = CostForecaster::new(vec![]);
        assert_eq!(f.forecast(30), 0.0);
    }

    #[test]
    fn cheaper_model_suggested_when_over_budget() {
        let f = CostForecaster::new(vec![10.0]);
        let model = f.suggest_cheaper_model(150.0, 100.0);
        assert_eq!(model, Some("gpt-4o-mini"));
    }

    #[test]
    fn no_suggestion_when_within_budget() {
        let f = CostForecaster::new(vec![1.0]);
        let model = f.suggest_cheaper_model(50.0, 100.0);
        assert!(model.is_none());
    }
}
