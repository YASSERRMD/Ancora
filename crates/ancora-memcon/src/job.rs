use crate::dedup::Deduplicator;
use crate::episodic::{EpisodicEntry, EpisodicToSemanticPromoter, SemanticEntry};
use crate::forgetting::ForgettingPolicy;
use crate::journal::{ConsolidationEvent, ConsolidationJournal};
use crate::salience::{SalienceItem, SalienceScorer};
use crate::summarizer::{ConversationSummarizer, SummaryResult, Turn};

/// Runs a full consolidation pass and journals every step.
pub struct ConsolidationJob {
    pub summarizer: ConversationSummarizer,
    pub scorer: SalienceScorer,
    pub promoter: EpisodicToSemanticPromoter,
    pub forgetting: ForgettingPolicy,
}

pub struct ConsolidationOutput {
    pub summary: SummaryResult,
    pub promoted: Vec<SemanticEntry>,
    pub retained: Vec<SalienceItem>,
}

impl ConsolidationJob {
    pub fn run(
        &self,
        turns: &[Turn],
        salience_items: Vec<SalienceItem>,
        episodic: &[EpisodicEntry],
        tick: u64,
        journal: &mut ConsolidationJournal,
    ) -> ConsolidationOutput {
        let summary = self.summarizer.summarize(turns);
        journal.record(
            tick,
            ConsolidationEvent::Summarized {
                dropped_count: summary.dropped_count,
                summary_len: summary.summary.len(),
            },
        );

        let promoted = self.promoter.promote(episodic);
        for p in &promoted {
            journal.record(tick, ConsolidationEvent::Promoted { key: p.key.clone() });
        }

        let before_dedup_len = salience_items.len();
        let deduped_items: Vec<SalienceItem> =
            Deduplicator::dedup_by_key(salience_items, |i| i.key.clone());
        let removed_count = before_dedup_len.saturating_sub(deduped_items.len());
        if removed_count > 0 {
            journal.record(tick, ConsolidationEvent::Deduped { removed_count });
        }

        let retained = self.forgetting.prune(deduped_items, &self.scorer);
        ConsolidationOutput {
            summary,
            promoted,
            retained,
        }
    }
}
