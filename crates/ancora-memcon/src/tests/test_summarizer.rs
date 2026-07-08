use crate::summarizer::{ConversationSummarizer, SummarizationPolicy, Turn};

fn make_turns(n: usize) -> Vec<Turn> {
    (0..n)
        .map(|i| Turn {
            index: i,
            role: "user".into(),
            content: format!("fact-{i}"),
        })
        .collect()
}

#[test]
fn policy_triggers_at_threshold() {
    let pol = SummarizationPolicy::new(5, 2);
    assert!(!pol.should_summarize(4));
    assert!(pol.should_summarize(5));
}

#[test]
fn summarize_keeps_last_n() {
    let pol = SummarizationPolicy::new(5, 2);
    let s = ConversationSummarizer::new(pol);
    let turns = make_turns(5);
    let res = s.summarize(&turns);
    assert_eq!(res.kept.len(), 2);
    assert_eq!(res.dropped_count, 3);
}

#[test]
fn summary_preserves_key_facts() {
    let pol = SummarizationPolicy::new(3, 1);
    let s = ConversationSummarizer::new(pol);
    let turns = vec![
        Turn {
            index: 0,
            role: "user".into(),
            content: "important-fact".into(),
        },
        Turn {
            index: 1,
            role: "user".into(),
            content: "noise".into(),
        },
        Turn {
            index: 2,
            role: "user".into(),
            content: "keep-this".into(),
        },
    ];
    let res = s.summarize(&turns);
    assert!(res.summary.contains("important-fact"));
    assert_eq!(res.kept[0].content, "keep-this");
}

#[test]
fn no_summary_when_turns_lt_keep_n() {
    let pol = SummarizationPolicy::new(10, 5);
    let s = ConversationSummarizer::new(pol);
    let turns = make_turns(3);
    let res = s.summarize(&turns);
    assert!(res.summary.is_empty());
    assert_eq!(res.kept.len(), 3);
}
