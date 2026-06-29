//! Quantization tradeoff tests.

use crate::quant::{QuantFormat, QuantMeasurement, QuantTradeoffEval};

#[test]
fn test_quant_degradation_is_non_negative() {
    let baseline = QuantMeasurement::new(QuantFormat::Fp32, 0.95, 5.0, 8000.0);
    let mut eval = QuantTradeoffEval::new(baseline);
    let v = QuantMeasurement::new(QuantFormat::Int8, 0.90, 5.5, 2000.0);
    eval.add_variant(v.clone());
    let deg = eval.quality_degradation(&v);
    assert!(deg >= 0.0);
    assert!((deg - 0.05).abs() < 1e-9);
}

#[test]
fn test_quant_memory_savings_ratio() {
    let baseline = QuantMeasurement::new(QuantFormat::Fp32, 1.0, 5.0, 8000.0);
    let mut eval = QuantTradeoffEval::new(baseline);
    let v = QuantMeasurement::new(QuantFormat::Int8, 0.95, 5.5, 2000.0);
    eval.add_variant(v.clone());
    let ratio = eval.memory_savings_ratio(&v);
    assert!((ratio - 0.75).abs() < 1e-9);
}

#[test]
fn test_quant_fp16_compression() {
    assert!((QuantFormat::Fp16.compression_ratio() - 2.0).abs() < 1e-9);
}
