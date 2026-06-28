use crate::workload::WorkloadSpec;

pub fn baseline() -> WorkloadSpec {
    WorkloadSpec::new("baseline", 100.0, 1800, 10)
}

pub fn spike() -> WorkloadSpec {
    WorkloadSpec::new("spike", 500.0, 60, 50)
}

pub fn soak_long() -> WorkloadSpec {
    WorkloadSpec::new("soak-long", 50.0, 7200, 5).with_payload(4096)
}
