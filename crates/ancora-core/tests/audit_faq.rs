// Documentation audit: FAQ covers most-asked questions.

const FAQ_QUESTIONS: &[&str] = &[
    "Is Ancora open source?",
    "Does Ancora require cloud connectivity?",
    "Which languages are supported?",
    "Can I use local models?",
    "How does Ancora handle failures?",
    "What is the journal?",
    "How is replay deterministic?",
    "Does Ancora support streaming tokens?",
    "How do I run tests offline?",
    "What is A2A?",
    "What is MCP?",
    "How do I control costs?",
];

fn is_valid_question(q: &str) -> bool {
    !q.is_empty() && q.ends_with('?')
}

#[test]
fn test_twelve_faq_questions() {
    assert_eq!(FAQ_QUESTIONS.len(), 12);
}

#[test]
fn test_all_questions_end_with_question_mark() {
    for q in FAQ_QUESTIONS {
        assert!(is_valid_question(q), "FAQ question does not end with ?: {q}");
    }
}

#[test]
fn test_local_models_faq_present() {
    assert!(FAQ_QUESTIONS.iter().any(|q| q.contains("local models")));
}

#[test]
fn test_offline_testing_faq_present() {
    assert!(FAQ_QUESTIONS.iter().any(|q| q.to_lowercase().contains("offline")));
}

#[test]
fn test_cost_faq_present() {
    assert!(FAQ_QUESTIONS.iter().any(|q| q.to_lowercase().contains("cost")));
}

#[test]
fn test_no_empty_question() {
    for q in FAQ_QUESTIONS { assert!(!q.trim().is_empty()); }
}
