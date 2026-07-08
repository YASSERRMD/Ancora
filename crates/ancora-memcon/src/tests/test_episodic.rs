use crate::episodic::{EpisodicEntry, EpisodicToSemanticPromoter};

#[test]
fn promotion_moves_recurring_facts() {
    let promoter = EpisodicToSemanticPromoter::new(3);
    let entries = vec![
        EpisodicEntry {
            key: "a".into(),
            content: "fact-a".into(),
            occurrences: 5,
        },
        EpisodicEntry {
            key: "b".into(),
            content: "fact-b".into(),
            occurrences: 1,
        },
    ];
    let promoted = promoter.promote(&entries);
    assert_eq!(promoted.len(), 1);
    assert_eq!(promoted[0].key, "a");
}

#[test]
fn below_threshold_not_promoted() {
    let promoter = EpisodicToSemanticPromoter::new(10);
    let entries = vec![EpisodicEntry {
        key: "x".into(),
        content: "c".into(),
        occurrences: 5,
    }];
    assert!(promoter.promote(&entries).is_empty());
}

#[test]
fn exact_threshold_is_promoted() {
    let promoter = EpisodicToSemanticPromoter::new(3);
    let entries = vec![EpisodicEntry {
        key: "z".into(),
        content: "c".into(),
        occurrences: 3,
    }];
    assert_eq!(promoter.promote(&entries).len(), 1);
}
