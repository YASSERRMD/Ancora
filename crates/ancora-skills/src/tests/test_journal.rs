use crate::journal::SkillJournal;

#[test]
fn journal_records_invocation() {
    let mut j = SkillJournal::default();
    j.record(1, "search", 2, "node-1");
    assert_eq!(j.records().len(), 1);
}

#[test]
fn journal_replays_deterministically() {
    let mut j = SkillJournal::default();
    j.record(1, "search", 1, "n1");
    j.record(2, "summarize", 2, "n2");
    let replayed = j.replay();
    assert_eq!(replayed[0], ("search", 1));
    assert_eq!(replayed[1], ("summarize", 2));
}

#[test]
fn journal_filters_by_skill_name() {
    let mut j = SkillJournal::default();
    j.record(1, "a", 1, "n1");
    j.record(2, "b", 1, "n2");
    j.record(3, "a", 1, "n3");
    assert_eq!(j.records_for_skill("a").len(), 2);
}
