use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub role: String,
    pub content: String,
    pub token_count: u32,
}

impl ConversationTurn {
    pub fn new(role: &str, content: &str, token_count: u32) -> Self {
        Self { role: role.to_string(), content: content.to_string(), token_count }
    }
}

pub struct ConversationCompressor {
    pub summary_token_budget: u32,
}

impl ConversationCompressor {
    pub fn new(summary_token_budget: u32) -> Self {
        Self { summary_token_budget }
    }

    /// Compress old turns by dropping middle turns until total fits the budget.
    /// Always keeps the first and last N turns.
    pub fn compress(&self, turns: Vec<ConversationTurn>, budget: u32, keep_last: usize) -> Vec<ConversationTurn> {
        let total: u32 = turns.iter().map(|t| t.token_count).sum();
        if total <= budget {
            return turns;
        }

        let keep_last = keep_last.min(turns.len());
        let (head, tail) = turns.split_at(turns.len() - keep_last);

        let mut compressed = vec![ConversationTurn::new(
            "system",
            "[earlier context summarized]",
            self.summary_token_budget,
        )];
        compressed.extend_from_slice(tail);
        compressed
    }

    pub fn total_tokens(turns: &[ConversationTurn]) -> u32 {
        turns.iter().map(|t| t.token_count).sum()
    }
}
