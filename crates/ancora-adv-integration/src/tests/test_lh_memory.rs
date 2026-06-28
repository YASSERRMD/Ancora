use ancora_lh::{BackgroundRun, Checkpoint, RunState, ScheduledWakeup};
use ancora_memcon::{
    ConsolidationJob, ConsolidationJournal, EpisodicToSemanticPromoter,
    ForgettingPolicy, SalienceItem, SalienceScorer, SummarizationPolicy, ConversationSummarizer,
    Turn,
};
use ancora_ageval::MemoryMetric;

#[test]
fn long_horizon_plus_memory_consolidation() {
    let mut run = BackgroundRun::new("run-1", 1);
    run.start();
    run.sleep_until(5);
    assert!(run.wake(10));
    assert_eq!(run.state, RunState::Woken);

    let mut ck = Checkpoint::new("run-1", 10);
    ck.set("step", "after-wake");
    assert_eq!(ck.get("step"), Some("after-wake"));

    let policy = SummarizationPolicy::new(2, 1);
    let summarizer = ConversationSummarizer::new(policy);
    let scorer = SalienceScorer::default_weights();
    let promoter = EpisodicToSemanticPromoter::new(2);
    let forgetting = ForgettingPolicy::new(0.0, 10000);
    let job = ConsolidationJob { summarizer, scorer, promoter, forgetting };

    let turns = vec![
        Turn { index: 0, role: "user".into(), content: "msg1".into() },
        Turn { index: 1, role: "agent".into(), content: "reply1".into() },
        Turn { index: 2, role: "user".into(), content: "msg2".into() },
    ];
    let items = vec![
        SalienceItem { key: "k1".into(), content: "c1".into(), importance: 1, access_count: 1, age_secs: 10 },
        SalienceItem { key: "k2".into(), content: "c2".into(), importance: 1, access_count: 2, age_secs: 5 },
    ];
    let mut journal = ConsolidationJournal::default();
    let output = job.run(&turns, items, &[], 10, &mut journal);

    assert!(!output.retained.is_empty());
    let score = MemoryMetric::score(output.retained.len(), 2);
    assert!(score > 0.0);
}

#[test]
fn scheduled_wakeup_combines_with_checkpoint() {
    let wakeup = ScheduledWakeup { run_id: "run-2".into(), wake_at_tick: 20 };
    assert!(!wakeup.should_fire(15));
    assert!(wakeup.should_fire(20));

    let mut ck = Checkpoint::new("run-2", 20);
    ck.set("status", "woken");
    assert_eq!(ck.get("status"), Some("woken"));
}
