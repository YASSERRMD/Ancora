use ancora_skills::{SkillDescriptor, SkillJournal, SkillRegistry, SkillScope};

fn make_descriptor(name: &str, version: u32) -> SkillDescriptor {
    SkillDescriptor::new(name, version, "desc", vec!["test"], SkillScope::ReadOnly)
}

fn load_skills(registry: &mut SkillRegistry) {
    registry.load(make_descriptor("summarize", 1));
    registry.load(make_descriptor("translate", 1));
}

fn record_invocations(journal: &mut SkillJournal) {
    journal.record(1, "summarize", 1, "node-1");
    journal.record(2, "translate", 1, "node-1");
    journal.record(3, "summarize", 1, "node-2");
}

#[test]
fn skills_journal_replay_order_stable() {
    let mut j1 = SkillJournal::default();
    let mut j2 = SkillJournal::default();
    record_invocations(&mut j1);
    record_invocations(&mut j2);

    let r1 = j1.replay();
    let r2 = j2.replay();
    assert_eq!(r1, r2);
}

#[test]
fn skills_journal_records_for_skill_stable() {
    let mut j1 = SkillJournal::default();
    let mut j2 = SkillJournal::default();
    record_invocations(&mut j1);
    record_invocations(&mut j2);

    let sum1: Vec<u64> = j1
        .records_for_skill("summarize")
        .iter()
        .map(|r| r.tick)
        .collect();
    let sum2: Vec<u64> = j2
        .records_for_skill("summarize")
        .iter()
        .map(|r| r.tick)
        .collect();
    assert_eq!(sum1, sum2);
}

#[test]
fn skills_registry_lookup_stable() {
    let mut r1 = SkillRegistry::default();
    let mut r2 = SkillRegistry::default();
    load_skills(&mut r1);
    load_skills(&mut r2);

    let v1 = r1.find("summarize").map(|s| s.version);
    let v2 = r2.find("summarize").map(|s| s.version);
    assert_eq!(v1, v2);
}
