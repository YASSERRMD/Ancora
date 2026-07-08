/// Trace context propagation for agent-to-agent (a2a) calls.
///
/// Encodes and decodes the W3C traceparent header format so that
/// trace context can cross agent boundaries without external crates.
use crate::span::{SpanId, TraceId};

/// The propagated context passed in / out of an a2a boundary.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceContext {
    pub trace_id: TraceId,
    pub parent_span_id: SpanId,
    /// W3C flags byte (bit 0 = sampled).
    pub flags: u8,
}

impl TraceContext {
    /// Create a sampled context.
    pub fn sampled(trace_id: TraceId, parent_span_id: SpanId) -> Self {
        TraceContext {
            trace_id,
            parent_span_id,
            flags: 0x01,
        }
    }

    /// Create an unsampled context.
    pub fn unsampled(trace_id: TraceId, parent_span_id: SpanId) -> Self {
        TraceContext {
            trace_id,
            parent_span_id,
            flags: 0x00,
        }
    }

    /// Returns true if the sampled flag is set.
    pub fn is_sampled(&self) -> bool {
        self.flags & 0x01 != 0
    }

    /// Encode as a traceparent header value.
    ///
    /// Format: `00|<trace_id>|<parent_span_id>|<flags>`
    ///
    /// We use `|` as the field separator (rather than `-`) so that trace and
    /// span ids may themselves contain hyphens without ambiguity.
    pub fn to_traceparent(&self) -> String {
        format!(
            "00|{}|{}|{:02x}",
            self.trace_id.0, self.parent_span_id.0, self.flags
        )
    }

    /// Parse a traceparent header value.
    ///
    /// Returns `None` if the format is not recognised.
    pub fn from_traceparent(value: &str) -> Option<Self> {
        let parts: Vec<&str> = value.splitn(4, '|').collect();
        if parts.len() != 4 {
            return None;
        }
        let _version = parts[0]; // "00"
        let trace_id = TraceId(parts[1].to_owned());
        let parent_span_id = SpanId(parts[2].to_owned());
        let flags = u8::from_str_radix(parts[3], 16).ok()?;
        Some(TraceContext {
            trace_id,
            parent_span_id,
            flags,
        })
    }
}

/// A carrier that transports trace context across a2a boundaries via
/// string headers (HTTP-like map).
pub struct HeaderCarrier {
    headers: std::collections::HashMap<String, String>,
}

impl HeaderCarrier {
    pub fn new() -> Self {
        HeaderCarrier {
            headers: std::collections::HashMap::new(),
        }
    }

    /// Inject a context into the carrier.
    pub fn inject(&mut self, ctx: &TraceContext) {
        self.headers
            .insert("traceparent".to_owned(), ctx.to_traceparent());
    }

    /// Extract a context from the carrier.
    pub fn extract(&self) -> Option<TraceContext> {
        self.headers
            .get("traceparent")
            .and_then(|v| TraceContext::from_traceparent(v))
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }
}

impl Default for HeaderCarrier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{SpanId, TraceId};

    #[test]
    fn round_trip_traceparent() {
        let ctx = TraceContext::sampled(TraceId("abc123".into()), SpanId("def456".into()));
        let header = ctx.to_traceparent();
        let parsed = TraceContext::from_traceparent(&header).unwrap();
        assert_eq!(parsed, ctx);
    }

    #[test]
    fn unsampled_flag() {
        let ctx = TraceContext::unsampled(TraceId("t1".into()), SpanId("s1".into()));
        assert!(!ctx.is_sampled());
    }

    #[test]
    fn carrier_inject_extract() {
        let ctx = TraceContext::sampled(TraceId("t2".into()), SpanId("s2".into()));
        let mut carrier = HeaderCarrier::new();
        carrier.inject(&ctx);
        let extracted = carrier.extract().unwrap();
        assert_eq!(extracted.trace_id, ctx.trace_id);
        assert_eq!(extracted.parent_span_id, ctx.parent_span_id);
    }
}
