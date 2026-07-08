use crate::audit::NetpolAuditLog;

#[derive(Debug, Clone)]
pub struct NetpolStats {
    pub total: usize,
    pub allowed: usize,
    pub denied: usize,
}

impl NetpolStats {
    pub fn from_log(log: &NetpolAuditLog, tenant_id: &str) -> Self {
        let all: Vec<_> = log.all().filter(|r| r.tenant_id == tenant_id).collect();
        let total = all.len();
        let denied = all.iter().filter(|r| !r.allowed).count();
        let allowed = total - denied;
        Self {
            total,
            allowed,
            denied,
        }
    }

    pub fn global(log: &NetpolAuditLog) -> Self {
        let all: Vec<_> = log.all().collect();
        let total = all.len();
        let denied = all.iter().filter(|r| !r.allowed).count();
        let allowed = total - denied;
        Self {
            total,
            allowed,
            denied,
        }
    }

    pub fn deny_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.denied as f64 / self.total as f64
        }
    }

    pub fn allow_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.allowed as f64 / self.total as f64
        }
    }
}
