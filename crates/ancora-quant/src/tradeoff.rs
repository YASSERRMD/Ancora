/// Quantization trade-off notes per model.
///
/// Provides structured trade-off analysis comparing quantization levels,
/// helping operators choose the right model for a deployment scenario.
use crate::quant_level::QuantTier;

/// Scenario classification for a deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeploymentScenario {
    /// Server with ample GPU VRAM (>=24 GB).
    HighEndServer,
    /// Consumer GPU (4-16 GB VRAM).
    ConsumerGpu,
    /// CPU-only server with lots of RAM (>=32 GB).
    CpuServer,
    /// Laptop / edge device (<= 16 GB RAM).
    Edge,
    /// Embedded device (<= 4 GB RAM).
    Embedded,
}

/// A trade-off record for one quantization level applied to a model.
#[derive(Debug, Clone)]
pub struct QuantTradeoff {
    /// Which quantization tier.
    pub tier: QuantTier,
    /// Approximate compression ratio vs FP32.
    pub compression_ratio: f32,
    /// Relative inference speed vs FP32 on CPU (1.0 = same, 2.0 = 2x faster).
    pub cpu_speed_ratio: f32,
    /// Estimated perplexity increase (PPL delta; lower is better).
    pub perplexity_delta: f32,
    /// Minimum recommended RAM in GB for a 7B parameter model.
    pub min_ram_gb_7b: f32,
    /// Recommended deployment scenarios.
    pub recommended_for: Vec<DeploymentScenario>,
    /// Human-readable trade-off summary.
    pub summary: String,
}

impl QuantTradeoff {
    /// Create a trade-off record.
    pub fn new(
        tier: QuantTier,
        compression_ratio: f32,
        cpu_speed_ratio: f32,
        perplexity_delta: f32,
        min_ram_gb_7b: f32,
        recommended_for: Vec<DeploymentScenario>,
        summary: impl Into<String>,
    ) -> Self {
        QuantTradeoff {
            tier,
            compression_ratio,
            cpu_speed_ratio,
            perplexity_delta,
            min_ram_gb_7b,
            recommended_for,
            summary: summary.into(),
        }
    }

    /// Is this tier recommended for the given scenario?
    pub fn recommended_for_scenario(&self, scenario: DeploymentScenario) -> bool {
        self.recommended_for.contains(&scenario)
    }

    /// Score combining quality and speed for a scenario (higher is better).
    ///
    /// Uses a simple heuristic: speed_ratio / (1 + perplexity_delta).
    pub fn scenario_score(&self) -> f32 {
        self.cpu_speed_ratio / (1.0 + self.perplexity_delta)
    }
}

/// Return a catalogue of standard trade-off records covering all tiers.
pub fn standard_tradeoffs() -> Vec<QuantTradeoff> {
    vec![
        QuantTradeoff::new(
            QuantTier::Full,
            1.0,
            1.0,
            0.0,
            28.0,
            vec![DeploymentScenario::HighEndServer],
            "FP32: reference quality, very large, slow on CPU. GPU-only in practice.",
        ),
        QuantTradeoff::new(
            QuantTier::Half,
            2.0,
            1.5,
            0.01,
            14.0,
            vec![DeploymentScenario::HighEndServer, DeploymentScenario::ConsumerGpu],
            "FP16/BF16: excellent quality with 2x compression. Requires GPU for speed.",
        ),
        QuantTradeoff::new(
            QuantTier::Int8,
            4.0,
            2.5,
            0.05,
            7.0,
            vec![
                DeploymentScenario::ConsumerGpu,
                DeploymentScenario::CpuServer,
            ],
            "INT8/Q8: near-lossless, 4x compression. Good CPU performance.",
        ),
        QuantTradeoff::new(
            QuantTier::Medium,
            6.0,
            3.0,
            0.15,
            5.0,
            vec![
                DeploymentScenario::CpuServer,
                DeploymentScenario::ConsumerGpu,
                DeploymentScenario::Edge,
            ],
            "Q5/Q6: good quality-size trade-off. Recommended default for most use cases.",
        ),
        QuantTradeoff::new(
            QuantTier::Int4,
            8.0,
            3.5,
            0.35,
            4.0,
            vec![DeploymentScenario::Edge, DeploymentScenario::ConsumerGpu],
            "Q4: noticeable quality loss but very small. Popular for consumer hardware.",
        ),
        QuantTradeoff::new(
            QuantTier::Aggressive,
            12.0,
            4.0,
            1.2,
            2.0,
            vec![DeploymentScenario::Embedded],
            "Q2/Q3: significant quality degradation. Only for very constrained devices.",
        ),
    ]
}

/// Find the best trade-off for a given scenario by score.
/// Returns the index into `standard_tradeoffs()` of the best match.
pub fn best_tradeoff_index_for(
    scenario: DeploymentScenario,
    available_ram_gb: f32,
    param_billions: f32,
) -> Option<usize> {
    let tradeoffs = standard_tradeoffs();
    let mut best: Option<(usize, f32)> = None;
    for (i, t) in tradeoffs.iter().enumerate() {
        let scaled = t.min_ram_gb_7b * (param_billions / 7.0).max(0.01);
        if scaled <= available_ram_gb && t.recommended_for_scenario(scenario) {
            let score = t.scenario_score();
            if best.as_ref().map(|(_, s)| score > *s).unwrap_or(true) {
                best = Some((i, score));
            }
        }
    }
    best.map(|(i, _)| i)
}

/// Return the recommended tier for a scenario and RAM budget.
pub fn recommended_tier(
    scenario: DeploymentScenario,
    available_ram_gb: f32,
    param_billions: f32,
) -> QuantTier {
    let tradeoffs = standard_tradeoffs();
    // Filter by RAM viability and scenario recommendation, pick best score.
    let viable: Vec<&QuantTradeoff> = tradeoffs
        .iter()
        .filter(|t| {
            // Scale min_ram_gb_7b proportionally to param count.
            let scaled_ram = t.min_ram_gb_7b * (param_billions / 7.0).max(0.01);
            scaled_ram <= available_ram_gb && t.recommended_for_scenario(scenario)
        })
        .collect();

    viable
        .into_iter()
        .max_by(|a, b| {
            a.scenario_score()
                .partial_cmp(&b.scenario_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|t| t.tier)
        .unwrap_or(QuantTier::Aggressive)
}
