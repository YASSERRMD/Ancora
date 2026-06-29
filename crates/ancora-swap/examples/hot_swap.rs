/// Example: model hot-swap without stopping in-flight runs.
///
/// Run with:
///   cargo run --example hot_swap -p ancora-swap

use ancora_swap::model::{ModelMeta, ModelVersion};
use ancora_swap::model::ModelHandle;
use ancora_swap::runtime::{RunId, SwapRuntime, WarmupStatus};

fn main() {
    // --- Set up initial model ---
    let v1 = ModelVersion::next();
    let m1 = ModelHandle::new(
        ModelMeta {
            name: "my-llm".to_string(),
            version: "0.1.0".to_string(),
            memory_bytes: 1024 * 1024 * 512,
        },
        v1,
    );
    println!("Starting runtime with model {}", m1.version());
    let rt = SwapRuntime::new(m1);

    // --- Start an in-flight run that pins model v1 ---
    let run_a = RunId(1);
    rt.start_run(run_a).expect("run_a started");
    println!("run_a pinned to model {}", rt.run_model_version(run_a).unwrap());

    // --- Prepare a new model (simulate warmup) ---
    let v2 = ModelVersion::next();
    let m2 = ModelHandle::new(
        ModelMeta {
            name: "my-llm".to_string(),
            version: "0.2.0".to_string(),
            memory_bytes: 1024 * 1024 * 600,
        },
        v2,
    );

    println!("Warming up candidate model {}...", m2.version());
    match rt.warmup(&m2, 0) {
        WarmupStatus::Complete(d) => println!("Warmup done in {:?}", d),
        WarmupStatus::Failed(e) => {
            eprintln!("Warmup failed: {e}");
            return;
        }
        WarmupStatus::InProgress => unreachable!(),
    }

    // --- Hot-swap to the new model ---
    let result = rt.swap(m2);
    println!(
        "Swapped {} -> {} in {} ns",
        result.old_version, result.new_version, result.elapsed_ns
    );

    // run_a is still alive on the old model.
    println!(
        "run_a still pinned to {} (graceful drain)",
        rt.run_model_version(run_a).unwrap()
    );

    // New runs use the new model.
    let run_b = RunId(2);
    rt.start_run(run_b).expect("run_b started");
    println!("run_b pinned to new model {}", rt.run_model_version(run_b).unwrap());
    rt.finish_run(run_b);

    // Finish old run and reclaim memory.
    rt.finish_run(run_a);
    let freed = rt.reclaim_unloaded();
    println!("Memory reclaimed for {freed} model(s)");

    // Print journal.
    println!("\nSwap journal:");
    for (i, entry) in rt.journal().entries().iter().enumerate() {
        println!("  [{}] {:?}", i, entry.event);
    }

    // Demonstrate rollback.
    println!("\nAttempting rollback (no drain present after reclaim)...");
    match rt.rollback() {
        Ok(rb) => println!("Rolled back to {}", rb.restored_version),
        Err(e) => println!("Rollback not available: {e}"),
    }
}
