use crate::forecast::CostForecaster;

#[test]
fn linear_forecast_within_tolerance() {
    let mut f = CostForecaster::new();
    // Perfect linear series: cost = 2.0 + 1.0 * period
    for i in 0u32..8 {
        f.add_observation(i, 2.0 + i as f64);
    }
    let predicted = f.forecast_linear(10).unwrap();
    // Expected: 2.0 + 10 = 12.0
    let tolerance = 0.05;
    assert!(
        (predicted - 12.0).abs() < tolerance,
        "predicted {} is not within {} of 12.0",
        predicted,
        tolerance
    );
}

#[test]
fn ema_forecast_converges() {
    let mut f = CostForecaster::new();
    // All same value: ema should equal that value.
    for _ in 0..10 {
        f.add_observation(0, 5.0);
    }
    let ema = f.forecast_ema(0.3).unwrap();
    assert!((ema - 5.0).abs() < 1e-6);
}

#[test]
fn forecast_next_n_returns_n_points() {
    let mut f = CostForecaster::new();
    for i in 0u32..5 {
        f.add_observation(i, i as f64 * 2.0);
    }
    let points = f.forecast_next_n(5, 4);
    assert_eq!(points.len(), 4);
    // periods should be 5, 6, 7, 8
    assert_eq!(points[0].period, 5);
    assert_eq!(points[3].period, 8);
}

#[test]
fn insufficient_observations_returns_none() {
    let mut f = CostForecaster::new();
    f.add_observation(0, 1.0);
    let pred = f.forecast_linear(5);
    assert!(
        pred.is_none(),
        "need >= 2 observations for linear regression"
    );
}
