use crate::research_assistant::{KnowledgeBase, KnowledgeEntry, ResearchAssistant};

#[test]
fn research_assistant_works_offline() {
    let mut kb = KnowledgeBase::new();
    kb.add(KnowledgeEntry::new(
        "Zero Trust",
        "Zero Trust is a security model requiring verification for every access request.",
        vec!["security".to_string(), "network".to_string()],
    ));
    kb.add(KnowledgeEntry::new(
        "mTLS",
        "Mutual TLS authenticates both client and server during the TLS handshake.",
        vec!["security".to_string(), "protocol".to_string()],
    ));

    let ra = ResearchAssistant::new(kb);

    let summary = ra.research("Zero Trust");
    assert!(
        !summary.bullets.is_empty(),
        "should return bullets for known topic"
    );
}

#[test]
fn research_by_tag_returns_multiple_entries() {
    let mut kb = KnowledgeBase::new();
    kb.add(KnowledgeEntry::new(
        "Firewall",
        "Firewalls filter traffic based on rules.",
        vec!["security".to_string()],
    ));
    kb.add(KnowledgeEntry::new(
        "IDS",
        "Intrusion detection systems monitor for anomalies.",
        vec!["security".to_string()],
    ));

    let ra = ResearchAssistant::new(kb);
    let summary = ra.research("security");
    assert!(summary.bullets.len() >= 2);
}

#[test]
fn research_unknown_topic_returns_no_bullets() {
    let kb = KnowledgeBase::new();
    let ra = ResearchAssistant::new(kb);
    let summary = ra.research("quantum entanglement");
    assert!(summary.bullets.is_empty());
}
