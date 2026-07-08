//! Boot-time agent service logic for headless OS integration.
//!
//! Handles the ordered boot sequence: config load, cgroup setup,
//! model preload, socket bind, and readiness signal.

use std::time::{Duration, Instant};

/// Ordered boot phases for the headless agent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BootPhase {
    Init,
    ConfigLoad,
    CgroupSetup,
    ModelPreload,
    SocketBind,
    Ready,
}

impl std::fmt::Display for BootPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootPhase::Init => write!(f, "init"),
            BootPhase::ConfigLoad => write!(f, "config-load"),
            BootPhase::CgroupSetup => write!(f, "cgroup-setup"),
            BootPhase::ModelPreload => write!(f, "model-preload"),
            BootPhase::SocketBind => write!(f, "socket-bind"),
            BootPhase::Ready => write!(f, "ready"),
        }
    }
}

/// The result of a single boot phase.
#[derive(Debug, Clone)]
pub struct PhaseResult {
    pub phase: BootPhase,
    pub success: bool,
    pub duration: Duration,
    pub message: String,
}

impl PhaseResult {
    pub fn ok(phase: BootPhase, duration: Duration) -> Self {
        PhaseResult {
            phase: phase.clone(),
            success: true,
            duration,
            message: format!("{} completed", phase),
        }
    }

    pub fn err(phase: BootPhase, duration: Duration, msg: impl Into<String>) -> Self {
        PhaseResult {
            phase,
            success: false,
            duration,
            message: msg.into(),
        }
    }
}

/// Ordered sequence of boot phases.
pub const BOOT_SEQUENCE: &[BootPhase] = &[
    BootPhase::Init,
    BootPhase::ConfigLoad,
    BootPhase::CgroupSetup,
    BootPhase::ModelPreload,
    BootPhase::SocketBind,
    BootPhase::Ready,
];

/// Boot record capturing all phase results and total elapsed time.
pub struct BootRecord {
    pub phases: Vec<PhaseResult>,
    pub total_duration: Duration,
    pub boot_to_ready_ms: u64,
}

impl BootRecord {
    pub fn new(phases: Vec<PhaseResult>, total_duration: Duration) -> Self {
        let boot_to_ready_ms = total_duration.as_millis() as u64;
        BootRecord {
            phases,
            total_duration,
            boot_to_ready_ms,
        }
    }

    pub fn all_succeeded(&self) -> bool {
        self.phases.iter().all(|p| p.success)
    }

    pub fn failed_phase(&self) -> Option<&PhaseResult> {
        self.phases.iter().find(|p| !p.success)
    }

    pub fn phase_duration(&self, phase: &BootPhase) -> Option<Duration> {
        self.phases
            .iter()
            .find(|p| &p.phase == phase)
            .map(|p| p.duration)
    }
}

/// Simulates a boot sequence and records the results.
/// In production, each phase would perform real OS-level operations.
pub struct BootSequencer {
    phase_simulators: Vec<(BootPhase, bool, &'static str)>,
}

impl BootSequencer {
    /// Creates a sequencer where all phases succeed.
    pub fn all_pass() -> Self {
        BootSequencer {
            phase_simulators: BOOT_SEQUENCE
                .iter()
                .map(|p| (p.clone(), true, ""))
                .collect(),
        }
    }

    /// Creates a sequencer with a specific phase set to fail.
    pub fn with_failure(fail_phase: BootPhase, msg: &'static str) -> Self {
        BootSequencer {
            phase_simulators: BOOT_SEQUENCE
                .iter()
                .map(|p| {
                    if p == &fail_phase {
                        (p.clone(), false, msg)
                    } else {
                        (p.clone(), true, "")
                    }
                })
                .collect(),
        }
    }

    /// Runs the boot sequence and returns a BootRecord.
    pub fn run(&self) -> BootRecord {
        let start = Instant::now();
        let mut results = Vec::new();
        for (phase, ok, msg) in &self.phase_simulators {
            let phase_start = Instant::now();
            let dur = phase_start.elapsed();
            if *ok {
                results.push(PhaseResult::ok(phase.clone(), dur));
            } else {
                results.push(PhaseResult::err(phase.clone(), dur, *msg));
                break;
            }
        }
        let total = start.elapsed();
        BootRecord::new(results, total)
    }
}

/// Returns the next phase after the given one, or None if at Ready.
pub fn next_phase(current: &BootPhase) -> Option<BootPhase> {
    match current {
        BootPhase::Init => Some(BootPhase::ConfigLoad),
        BootPhase::ConfigLoad => Some(BootPhase::CgroupSetup),
        BootPhase::CgroupSetup => Some(BootPhase::ModelPreload),
        BootPhase::ModelPreload => Some(BootPhase::SocketBind),
        BootPhase::SocketBind => Some(BootPhase::Ready),
        BootPhase::Ready => None,
    }
}
