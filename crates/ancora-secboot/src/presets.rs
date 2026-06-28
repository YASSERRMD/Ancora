use crate::measurement::MeasurementKind;
use crate::policy::BootPolicy;

pub fn strict_boot_policy(tenant_id: impl Into<String>) -> BootPolicy {
    BootPolicy::new(tenant_id)
        .require_kind(MeasurementKind::Firmware)
        .require_kind(MeasurementKind::Bootloader)
        .require_kind(MeasurementKind::Kernel)
}

pub fn permissive_boot_policy(tenant_id: impl Into<String>) -> BootPolicy {
    BootPolicy::new(tenant_id).allow_unknown()
}

pub fn kernel_only_policy(tenant_id: impl Into<String>) -> BootPolicy {
    BootPolicy::new(tenant_id)
        .require_kind(MeasurementKind::Kernel)
}
