use crate::detection::DetectionSource;

#[test]
fn siem() { assert_eq!(DetectionSource::Siem.to_string(), "SIEM"); }
#[test]
fn edr() { assert_eq!(DetectionSource::Edr.to_string(), "EDR"); }
#[test]
fn ids() { assert_eq!(DetectionSource::Ids.to_string(), "IDS"); }
#[test]
fn network_monitor() { assert_eq!(DetectionSource::NetworkMonitor.to_string(), "NETWORK_MONITOR"); }
#[test]
fn manual_review() { assert_eq!(DetectionSource::ManualReview.to_string(), "MANUAL_REVIEW"); }
#[test]
fn honey_token() { assert_eq!(DetectionSource::HoneyToken.to_string(), "HONEY_TOKEN"); }
