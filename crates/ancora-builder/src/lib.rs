/// ancora-builder - Visual graph builder that produces and reads valid graph
/// specs and runs them locally with a trace overlay.

pub mod scaffold;
pub mod palette;
pub mod placement;
pub mod edges;
pub mod panels;
pub mod import;
pub mod export;
pub mod validation;
pub mod runner;
pub mod trace_overlay;
pub mod templates;

#[cfg(test)]
mod tests;
