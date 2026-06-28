/// Zero-downtime migration strategy helpers.
/// Expand-contract pattern: add column (expand), backfill, switch reads/writes, drop old (contract).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZdtPhase {
    Idle,
    Expand,
    Backfill,
    Contract,
    Done,
}

pub struct ZdtMigration {
    pub name: String,
    pub phase: ZdtPhase,
    pub rows_backfilled: usize,
    pub total_rows: usize,
}

impl ZdtMigration {
    pub fn new(name: &str, total_rows: usize) -> Self {
        Self { name: name.to_string(), phase: ZdtPhase::Idle, rows_backfilled: 0, total_rows }
    }

    pub fn start_expand(&mut self) {
        self.phase = ZdtPhase::Expand;
    }

    pub fn start_backfill(&mut self) {
        self.phase = ZdtPhase::Backfill;
    }

    pub fn advance_backfill(&mut self, count: usize) {
        self.rows_backfilled = (self.rows_backfilled + count).min(self.total_rows);
    }

    pub fn backfill_complete(&self) -> bool {
        self.rows_backfilled >= self.total_rows
    }

    pub fn start_contract(&mut self) {
        if self.backfill_complete() {
            self.phase = ZdtPhase::Contract;
        }
    }

    pub fn finish(&mut self) {
        self.phase = ZdtPhase::Done;
    }

    pub fn progress_pct(&self) -> f64 {
        if self.total_rows == 0 {
            return 100.0;
        }
        (self.rows_backfilled as f64 / self.total_rows as f64) * 100.0
    }
}
