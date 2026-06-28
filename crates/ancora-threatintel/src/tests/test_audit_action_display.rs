use crate::audit::ThreatIntelAction;

#[test]
fn action_display() {
    assert_eq!(format!("{}", ThreatIntelAction::IndicatorAdded), "INDICATOR_ADDED");
    assert_eq!(format!("{}", ThreatIntelAction::IndicatorExpired), "INDICATOR_EXPIRED");
    assert_eq!(format!("{}", ThreatIntelAction::IndicatorDeactivated), "INDICATOR_DEACTIVATED");
    assert_eq!(format!("{}", ThreatIntelAction::FeedIngested), "FEED_INGESTED");
    assert_eq!(format!("{}", ThreatIntelAction::FeedEnabled), "FEED_ENABLED");
    assert_eq!(format!("{}", ThreatIntelAction::FeedDisabled), "FEED_DISABLED");
    assert_eq!(format!("{}", ThreatIntelAction::ScoreComputed), "SCORE_COMPUTED");
    assert_eq!(format!("{}", ThreatIntelAction::AlertTriggered), "ALERT_TRIGGERED");
}
