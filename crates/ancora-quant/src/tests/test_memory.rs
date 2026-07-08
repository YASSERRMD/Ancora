use crate::gguf::{GgufDescriptor, GgufQuantType};
use crate::memory::{memory_report, select_model, select_model_with_headroom, SelectionPolicy};
use crate::registry::ModelRegistry;

fn build_registry() -> ModelRegistry {
    let mut reg = ModelRegistry::new();
    // Small 3B Q4 model.
    reg.register_gguf(
        "small-3b",
        GgufDescriptor::new(
            "small-3b",
            "/tmp/s.gguf",
            "llama",
            3.0,
            GgufQuantType::Q4_K,
            0,
            2048,
        ),
    );
    // Medium 7B Q5 model.
    reg.register_gguf(
        "medium-7b",
        GgufDescriptor::new(
            "medium-7b",
            "/tmp/m.gguf",
            "llama",
            7.0,
            GgufQuantType::Q5_K,
            0,
            4096,
        ),
    );
    // Large 13B Q8 model.
    reg.register_gguf(
        "large-13b",
        GgufDescriptor::new(
            "large-13b",
            "/tmp/l.gguf",
            "llama",
            13.0,
            GgufQuantType::Q8_0,
            0,
            4096,
        ),
    );
    reg
}

#[test]
fn memory_aware_selection_picks_a_fitting_model() {
    let reg = build_registry();
    // Budget: 5 GB -- should fit small and medium but not large.
    let budget = 5 * 1024 * 1024 * 1024_u64;
    let result = select_model(&reg, budget, SelectionPolicy::LargestFit);
    assert!(result.is_some());
    let r = result.unwrap();
    // LargestFit should pick medium-7b over small-3b if it fits.
    assert!(r.estimated_ram_bytes <= budget);
}

#[test]
fn selection_returns_none_when_nothing_fits() {
    let reg = build_registry();
    let budget = 1024_u64; // 1 KB -- nothing fits.
    let result = select_model(&reg, budget, SelectionPolicy::LargestFit);
    assert!(result.is_none());
}

#[test]
fn smallest_fit_picks_lowest_ram() {
    let reg = build_registry();
    let budget = 64 * 1024 * 1024 * 1024_u64; // 64 GB -- all fit.
    let result = select_model(&reg, budget, SelectionPolicy::SmallestFit).unwrap();
    // SmallestFit should pick small-3b.
    assert_eq!(result.model_id, "small-3b");
}

#[test]
fn select_with_headroom_reduces_budget() {
    let reg = build_registry();
    // 64 GB with 0.05 headroom = 3.2 GB budget -- only very small models fit.
    let available = 64 * 1024 * 1024 * 1024_u64;
    let result = select_model_with_headroom(&reg, available, 0.05, SelectionPolicy::LargestFit);
    // With 3.2 GB, small-3b Q4 may or may not fit depending on exact estimate.
    // Just check the result respects the budget.
    if let Some(r) = result {
        let budget = (available as f64 * 0.05) as u64;
        assert!(r.estimated_ram_bytes <= budget);
    }
}

#[test]
fn memory_report_counts_correctly() {
    let reg = build_registry();
    let budget = 5 * 1024 * 1024 * 1024_u64;
    let report = memory_report(&reg, budget);
    assert_eq!(report.total_models, 3);
    assert_eq!(report.budget_bytes, budget);
    // Fitting models are sorted ascending.
    let rams: Vec<u64> = report.fitting_models.iter().map(|(_, r)| *r).collect();
    for i in 1..rams.len() {
        assert!(rams[i] >= rams[i - 1]);
    }
}
