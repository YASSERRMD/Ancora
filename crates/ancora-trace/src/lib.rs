/// ancora-trace: Rich trace tree mirrors run structure with
/// redaction-aware, reproducible spans exported via OTLP.
///
/// Modules:
/// - `trace`       - Unified trace model assembled from journal events
/// - `span`        - Span, SpanId, TraceId and attribute types
/// - `propagation` - W3C traceparent encoding/decoding for a2a hops
/// - `redact`      - Policy-driven attribute redaction / truncation
/// - `genai_attrs` - GenAI semantic-convention attribute helpers
/// - `export`      - OTLP-compatible span export (mock and stub)
/// - `journal`     - Journal-to-span bridge
pub mod export;
pub mod genai_attrs;
pub mod journal;
pub mod propagation;
pub mod redact;
pub mod span;
pub mod trace;

#[cfg(test)]
mod tests;
