use crate::error::CostError;
use serde::{Deserialize, Serialize};

/// Budget period in seconds.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BudgetPeriod {
    Daily,
    Weekly,
    Monthly,
    Custom { secs: u64 },
}

impl BudgetPeriod {
    pub fn secs(&self) -> u64 {
        match self {
            Self::Daily => 86400,
            Self::Weekly => 604800,
            Self::Monthly => 2592000,
            Self::Custom { secs } => *secs,
        }
    }
}

/// Per-tenant budget definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantBudget {
    pub tenant_id: String,
    pub hard_limit_usd: f64,
    pub soft_limit_fraction: f64,
    pub period: BudgetPeriod,
    pub period_start_secs: u64,
    pub spent_usd: f64,
}

impl TenantBudget {
    pub fn new(
        tenant_id: impl Into<String>,
        hard_limit_usd: f64,
        soft_limit_fraction: f64,
        period: BudgetPeriod,
        period_start_secs: u64,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            hard_limit_usd,
            soft_limit_fraction,
            period,
            period_start_secs,
            spent_usd: 0.0,
        }
    }

    /// Check and record a spend. Returns Err if hard cap or soft cap is triggered.
    pub fn record_spend(&mut self, amount: f64) -> Result<(), CostError> {
        let new_total = self.spent_usd + amount;
        if new_total > self.hard_limit_usd {
            return Err(CostError::HardCapExceeded {
                budget: self.hard_limit_usd,
                spent: new_total,
            });
        }
        self.spent_usd = new_total;
        let pct = self.spent_usd / self.hard_limit_usd;
        if pct >= self.soft_limit_fraction {
            return Err(CostError::SoftCapWarning { pct: pct * 100.0 });
        }
        Ok(())
    }

    pub fn remaining_usd(&self) -> f64 {
        (self.hard_limit_usd - self.spent_usd).max(0.0)
    }

    /// Reset spent_usd at period rollover.
    pub fn rollover(&mut self, now: u64) {
        if now >= self.period_start_secs + self.period.secs() {
            self.spent_usd = 0.0;
            self.period_start_secs = now;
        }
    }
}

/// Per-project budget (aggregates multiple tenants under a project).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectBudget {
    pub project_id: String,
    pub hard_limit_usd: f64,
    pub soft_limit_fraction: f64,
    pub spent_usd: f64,
}

impl ProjectBudget {
    pub fn new(
        project_id: impl Into<String>,
        hard_limit_usd: f64,
        soft_limit_fraction: f64,
    ) -> Self {
        Self {
            project_id: project_id.into(),
            hard_limit_usd,
            soft_limit_fraction,
            spent_usd: 0.0,
        }
    }

    pub fn record_spend(&mut self, amount: f64) -> Result<(), CostError> {
        let new_total = self.spent_usd + amount;
        if new_total > self.hard_limit_usd {
            return Err(CostError::HardCapExceeded {
                budget: self.hard_limit_usd,
                spent: new_total,
            });
        }
        self.spent_usd = new_total;
        let pct = self.spent_usd / self.hard_limit_usd;
        if pct >= self.soft_limit_fraction {
            return Err(CostError::SoftCapWarning { pct: pct * 100.0 });
        }
        Ok(())
    }
}
