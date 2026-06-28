use crate::permission::Permission;
use crate::role::Role;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum RbacEvent {
    RoleAssigned { subject: String, tenant_id: String, role: Role, tick: u64 },
    RoleRevoked { subject: String, tenant_id: String, tick: u64 },
    PermissionGranted { subject: String, tenant_id: String, perm: String, tick: u64 },
    PermissionDenied { subject: String, tenant_id: String, perm: String, tick: u64 },
}

#[derive(Debug, Default)]
pub struct RbacAuditLog {
    events: VecDeque<RbacEvent>,
    max_size: usize,
}

impl RbacAuditLog {
    pub fn new(max_size: usize) -> Self {
        Self { events: VecDeque::new(), max_size }
    }

    pub fn record(&mut self, event: RbacEvent) {
        if self.events.len() >= self.max_size { self.events.pop_front(); }
        self.events.push_back(event);
    }

    pub fn record_check(&mut self, subject: &str, tenant_id: &str, perm: &Permission, allowed: bool, tick: u64) {
        if allowed {
            self.record(RbacEvent::PermissionGranted {
                subject: subject.into(), tenant_id: tenant_id.into(), perm: perm.as_str().into(), tick,
            });
        } else {
            self.record(RbacEvent::PermissionDenied {
                subject: subject.into(), tenant_id: tenant_id.into(), perm: perm.as_str().into(), tick,
            });
        }
    }

    pub fn count(&self) -> usize { self.events.len() }

    pub fn denied_count(&self) -> usize {
        self.events.iter().filter(|e| matches!(e, RbacEvent::PermissionDenied { .. })).count()
    }
}
