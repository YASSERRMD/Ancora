// Remote policy update for edge devices

use crate::registration::DeviceId;
use std::collections::HashMap;

/// A policy rule — key/value pairs representing enforcement settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyRule {
    pub key: String,
    pub value: String,
}

/// A policy document applied to devices
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Policy {
    pub policy_id: String,
    pub version: u64,
    pub rules: Vec<PolicyRule>,
}

impl Policy {
    pub fn new(policy_id: impl Into<String>, version: u64) -> Self {
        Self {
            policy_id: policy_id.into(),
            version,
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.rules.push(PolicyRule {
            key: key.into(),
            value: value.into(),
        });
    }

    pub fn get_rule(&self, key: &str) -> Option<&str> {
        self.rules
            .iter()
            .find(|r| r.key == key)
            .map(|r| r.value.as_str())
    }
}

/// Status of a policy update on a device
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PolicyUpdateStatus {
    Pending,
    Applied,
    Rejected(String),
}

/// Record of a policy update push
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyUpdateRecord {
    pub device_id: DeviceId,
    pub policy_id: String,
    pub version: u64,
    pub status: PolicyUpdateStatus,
}

/// Remote policy service — manages policy distribution and acknowledgement
#[derive(Debug, Default)]
pub struct RemotePolicyService {
    /// (device_id, policy_id) -> PolicyUpdateRecord
    records: HashMap<(String, String), PolicyUpdateRecord>,
}

impl RemotePolicyService {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a policy to a device
    pub fn push_policy(&mut self, device_id: &DeviceId, policy: &Policy) -> PolicyUpdateRecord {
        let record = PolicyUpdateRecord {
            device_id: device_id.clone(),
            policy_id: policy.policy_id.clone(),
            version: policy.version,
            status: PolicyUpdateStatus::Applied,
        };
        self.records.insert(
            (device_id.0.clone(), policy.policy_id.clone()),
            record.clone(),
        );
        record
    }

    /// Push a policy to all devices in the fleet
    pub fn push_to_fleet(
        &mut self,
        device_ids: &[DeviceId],
        policy: &Policy,
    ) -> Vec<PolicyUpdateRecord> {
        device_ids
            .iter()
            .map(|id| self.push_policy(id, policy))
            .collect()
    }

    pub fn status(&self, device_id: &DeviceId, policy_id: &str) -> Option<&PolicyUpdateStatus> {
        let key = (device_id.0.clone(), policy_id.to_string());
        self.records.get(&key).map(|r| &r.status)
    }

    pub fn is_applied(&self, device_id: &DeviceId, policy_id: &str) -> bool {
        self.status(device_id, policy_id)
            .map(|s| *s == PolicyUpdateStatus::Applied)
            .unwrap_or(false)
    }

    /// Devices that have applied a given policy
    pub fn applied_devices(&self, policy_id: &str) -> Vec<&DeviceId> {
        self.records
            .values()
            .filter(|r| r.policy_id == policy_id && r.status == PolicyUpdateStatus::Applied)
            .map(|r| &r.device_id)
            .collect()
    }
}
