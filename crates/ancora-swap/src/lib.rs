/// ancora-swap -- model hot-swapping and lifecycle for the Ancora agent framework.
///
/// Hot-swap models without restarting runs:
/// * model handle abstraction (`model` module)
/// * runtime binding and hot-swap (`runtime` module)
/// * graceful drain (old runs keep their pin until they finish)
/// * version pinning per run (`pin` module)
/// * swap journaled for replay (`journal` module)
/// * rollback to previous model
/// * warmup new model before swap
/// * memory reclaim on unload

pub mod drain;
pub mod journal;
pub mod model;
pub mod pin;
pub mod runtime;

#[cfg(test)]
mod tests;
