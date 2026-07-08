use crate::feed::{FeedFormat, ThreatFeed};
use crate::indicator::{Indicator, IndicatorKind, ThreatLevel};

pub fn known_bad_ip(tenant_id: impl Into<String>, tick: u64) -> Indicator {
    Indicator::new(
        "preset-badip-1",
        tenant_id,
        IndicatorKind::IpAddress,
        "192.0.2.1",
        ThreatLevel::High,
        "preset",
        tick,
    )
}

pub fn known_malware_hash(tenant_id: impl Into<String>, tick: u64) -> Indicator {
    Indicator::new(
        "preset-hash-1",
        tenant_id,
        IndicatorKind::FileHash,
        "e3b0c44298fc1c149afb",
        ThreatLevel::Critical,
        "preset",
        tick,
    )
}

pub fn phishing_domain(tenant_id: impl Into<String>, tick: u64) -> Indicator {
    Indicator::new(
        "preset-phish-1",
        tenant_id,
        IndicatorKind::Domain,
        "evil-phish.example",
        ThreatLevel::High,
        "preset",
        tick,
    )
}

pub fn internal_feed(tenant_id: impl Into<String>, tick: u64) -> ThreatFeed {
    ThreatFeed::new(
        "feed-internal",
        tenant_id,
        "Internal Threat Feed",
        FeedFormat::Internal,
        "internal://feeds/main",
        tick,
    )
}
