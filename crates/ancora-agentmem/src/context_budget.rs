/// Context window budget tracker for multi-turn conversations.
pub struct ContextBudget {
    pub max_tokens: u32,
    pub system_tokens: u32,
    used_tokens: u32,
}

impl ContextBudget {
    pub fn new(max_tokens: u32, system_tokens: u32) -> Self {
        Self {
            max_tokens,
            system_tokens,
            used_tokens: system_tokens,
        }
    }

    pub fn remaining(&self) -> u32 {
        self.max_tokens.saturating_sub(self.used_tokens)
    }

    pub fn add_message(&mut self, tokens: u32) -> bool {
        if self.used_tokens + tokens > self.max_tokens {
            return false;
        }
        self.used_tokens += tokens;
        true
    }

    pub fn reset_to_system(&mut self) {
        self.used_tokens = self.system_tokens;
    }

    pub fn used(&self) -> u32 {
        self.used_tokens
    }

    pub fn utilization_pct(&self) -> f64 {
        self.used_tokens as f64 / self.max_tokens as f64 * 100.0
    }
}
