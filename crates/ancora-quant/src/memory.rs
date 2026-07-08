/// Memory-aware model selection.
///
/// Selects the best model from a registry given a RAM budget and optional
/// quality preferences.
use std::cmp::Ordering;

use crate::registry::{ModelEntry, ModelRegistry};

/// Policy for choosing among models that fit the RAM budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionPolicy {
    /// Prefer the largest model (most parameters) that fits.
    LargestFit,
    /// Prefer the smallest model to conserve RAM.
    SmallestFit,
    /// Prefer the model with fewest bits-per-weight (most compressed).
    MostCompressed,
}

/// Result of a selection query.
#[derive(Debug, Clone)]
pub struct SelectionResult<'a> {
    pub model_id: &'a str,
    pub entry: &'a ModelEntry,
    pub estimated_ram_bytes: u64,
}

/// Select the best model from a registry given a RAM budget.
///
/// Returns `None` if no model fits.
pub fn select_model<'a>(
    registry: &'a ModelRegistry,
    ram_budget_bytes: u64,
    policy: SelectionPolicy,
) -> Option<SelectionResult<'a>> {
    let candidates: Vec<(&str, &ModelEntry)> = registry.models_fitting_ram(ram_budget_bytes);

    if candidates.is_empty() {
        return None;
    }

    let best = match policy {
        SelectionPolicy::LargestFit => candidates.into_iter().max_by(|(_, a), (_, b)| {
            a.param_count_billions()
                .partial_cmp(&b.param_count_billions())
                .unwrap_or(Ordering::Equal)
        }),
        SelectionPolicy::SmallestFit => candidates
            .into_iter()
            .min_by(|(_, a), (_, b)| a.estimated_ram_bytes().cmp(&b.estimated_ram_bytes())),
        SelectionPolicy::MostCompressed => candidates.into_iter().min_by(|(_, a), (_, b)| {
            // Smaller estimated RAM per billion params = more compressed.
            let ratio_a = if a.param_count_billions() > 0.0 {
                a.estimated_ram_bytes() as f64 / a.param_count_billions() as f64
            } else {
                f64::MAX
            };
            let ratio_b = if b.param_count_billions() > 0.0 {
                b.estimated_ram_bytes() as f64 / b.param_count_billions() as f64
            } else {
                f64::MAX
            };
            ratio_a.partial_cmp(&ratio_b).unwrap_or(Ordering::Equal)
        }),
    };

    best.map(|(id, entry)| SelectionResult {
        model_id: id,
        entry,
        estimated_ram_bytes: entry.estimated_ram_bytes(),
    })
}

/// Select the best model for a given system memory state.
///
/// Uses a conservative headroom: only considers models that consume at most
/// `headroom_fraction` of the available RAM.
pub fn select_model_with_headroom<'a>(
    registry: &'a ModelRegistry,
    available_ram_bytes: u64,
    headroom_fraction: f64,
    policy: SelectionPolicy,
) -> Option<SelectionResult<'a>> {
    let budget = (available_ram_bytes as f64 * headroom_fraction) as u64;
    select_model(registry, budget, policy)
}

/// Memory report for a registry.
#[derive(Debug)]
pub struct MemoryReport {
    /// Total number of models in registry.
    pub total_models: usize,
    /// Models that fit within budget (sorted ascending by RAM).
    pub fitting_models: Vec<(String, u64)>,
    /// Budget in bytes.
    pub budget_bytes: u64,
}

/// Generate a memory report for the registry given a budget.
pub fn memory_report(registry: &ModelRegistry, budget_bytes: u64) -> MemoryReport {
    let mut fitting_models: Vec<(String, u64)> = registry
        .models_fitting_ram(budget_bytes)
        .into_iter()
        .map(|(id, e)| (id.to_string(), e.estimated_ram_bytes()))
        .collect();
    fitting_models.sort_by_key(|(_, ram)| *ram);

    MemoryReport {
        total_models: registry.len(),
        fitting_models,
        budget_bytes,
    }
}
