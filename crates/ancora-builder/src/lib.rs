pub mod edges;
pub mod export;
pub mod import;
pub mod palette;
pub mod panels;
pub mod placement;
pub mod runner;
/// ancora-builder - Visual graph builder that produces and reads valid graph
/// specs and runs them locally with a trace overlay.
pub mod scaffold;
pub mod templates;
pub mod trace_overlay;
pub mod validation;

#[cfg(test)]
mod tests;
