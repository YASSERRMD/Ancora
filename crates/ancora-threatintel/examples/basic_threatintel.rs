use ancora_threatintel::alert::AlertStore;
use ancora_threatintel::audit::{ThreatIntelAction, ThreatIntelAuditEntry, ThreatIntelAuditLog};
use ancora_threatintel::builder::IndicatorBuilder;
use ancora_threatintel::feed::FeedStore;
use ancora_threatintel::indicator::{IndicatorKind, ThreatLevel};
use ancora_threatintel::policy::ThreatPolicy;
use ancora_threatintel::presets::{internal_feed, known_bad_ip, known_malware_hash};
use ancora_threatintel::score::ThreatScorer;
use ancora_threatintel::store::IndicatorStore;
use ancora_threatintel::summary::ThreatIntelSummary;

fn main() {
    let mut store = IndicatorStore::new();
    let mut feeds = FeedStore::new();
    let alerts = AlertStore::new();
    let mut audit = ThreatIntelAuditLog::new();

    feeds.register_feed(internal_feed("acme", 1000));

    let bad_ip = known_bad_ip("acme", 1000);
    let malware = known_malware_hash("acme", 1000);
    let custom = IndicatorBuilder::new(
        "custom-1",
        "acme",
        IndicatorKind::Domain,
        "malware-c2.example",
    )
    .threat_level(ThreatLevel::Critical)
    .source("internal-honeypot")
    .tick(1000)
    .tag("apt29")
    .tag("c2")
    .build();

    for ind in [&bad_ip, &malware, &custom] {
        store.insert(ind.clone());
        audit.record(ThreatIntelAuditEntry::new(
            1000,
            "acme",
            ThreatIntelAction::IndicatorAdded,
            &ind.id,
            "ingested",
        ));
    }

    let policy = ThreatPolicy::new("acme");
    for ind in store.for_tenant("acme") {
        let score = ThreatScorer::score(ind, 0, 1000);
        let decision = policy.evaluate(&score);
        println!(
            "Indicator {} score={:.1} level={} decision={:?}",
            ind.id, score.raw_score, score.level, decision
        );
    }

    let all_inds: Vec<&_> = store.for_tenant("acme");
    let summary = ThreatIntelSummary::generate(&all_inds, &alerts, "acme");
    println!(
        "Summary: total={} active={} critical={} healthy={}",
        summary.total_indicators,
        summary.active_indicators,
        summary.critical_count,
        summary.is_healthy
    );
    println!("Audit entries: {}", audit.count());
}
