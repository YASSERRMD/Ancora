use crate::quant_level::QuantTier;
use crate::tradeoff::{recommended_tier, standard_tradeoffs, DeploymentScenario};

#[test]
fn standard_tradeoffs_covers_all_tiers() {
    let tradeoffs = standard_tradeoffs();
    let tiers: Vec<QuantTier> = tradeoffs.iter().map(|t| t.tier).collect();
    assert!(tiers.contains(&QuantTier::Full));
    assert!(tiers.contains(&QuantTier::Half));
    assert!(tiers.contains(&QuantTier::Int8));
    assert!(tiers.contains(&QuantTier::Medium));
    assert!(tiers.contains(&QuantTier::Int4));
    assert!(tiers.contains(&QuantTier::Aggressive));
}

#[test]
fn model_selection_offline() {
    // With plenty of RAM and high-end server, should recommend something good.
    let tier = recommended_tier(DeploymentScenario::HighEndServer, 80.0, 7.0);
    assert!(tier == QuantTier::Full || tier == QuantTier::Half || tier == QuantTier::Int8);
}

#[test]
fn embedded_scenario_picks_aggressive() {
    // 2 GB RAM, embedded -- should pick aggressive tier.
    let tier = recommended_tier(DeploymentScenario::Embedded, 2.0, 7.0);
    assert!(tier == QuantTier::Aggressive || tier == QuantTier::Int4);
}

#[test]
fn scenario_score_is_positive() {
    for t in standard_tradeoffs() {
        assert!(t.scenario_score() > 0.0);
    }
}

#[test]
fn compression_increases_with_aggressiveness() {
    let tradeoffs = standard_tradeoffs();
    // Higher tiers should have higher compression ratios.
    let full = tradeoffs.iter().find(|t| t.tier == QuantTier::Full).unwrap();
    let aggressive = tradeoffs
        .iter()
        .find(|t| t.tier == QuantTier::Aggressive)
        .unwrap();
    assert!(aggressive.compression_ratio > full.compression_ratio);
}
