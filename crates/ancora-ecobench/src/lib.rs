//! ancora-ecobench: Ecosystem benchmarks and overhead measurement.
//!
//! Extension and packaging overhead is measured, reproducible, and gated
//! against regression. Each module isolates one benchmark domain; the
//! `harness` module provides the shared timing infrastructure.

pub mod adapter_overhead;
pub mod builder_export;
pub mod catalog_install;
pub mod harness;
pub mod plugin_load;
pub mod plugin_subprocess;
pub mod plugin_wasm;
pub mod recipe_instantiation;
pub mod registry_fetch;
pub mod result_schema;

#[cfg(test)]
mod tests {
    mod test_adapter_overhead;
    mod test_harness_reproducible;
    mod test_plugin_overhead;
    mod test_wasm_vs_subprocess;
}
