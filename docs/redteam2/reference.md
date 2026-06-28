# ancora-redteam2 Reference

Extended adversarial red team simulation for enterprise security testing. Models attack chains, detection events, objectives, and comprehensive audit trails.

## Modules

### scenario
`RedTeamScenario` tracks a full red team scenario from creation through completion.

- `ScenarioKind`: PrivilegeEscalation, LateralMovement, DataExfiltration, CredentialHarvesting, PersistenceMechanism, DefenseEvasion, CommandAndControl, InitialAccess, CollectionAndRecon, ImpactAndDisruption
- `ScenarioStatus`: Pending, Running, Completed, Failed, Aborted
- Lifecycle: `start()`, `complete(tick)`, `fail()`, `abort()`
- MITRE ATT&CK tactic tagging via `with_mitre()`

### attack
`AttackStep` records a discrete action within a scenario.

- `AttackVector`: Network, Local, Physical, Adjacent
- `AttackOutcome`: Success, PartialSuccess, Failure, Detected, Blocked
- `AttackLog`: stores and filters steps by scenario, outcome, vector

### objective
`RedTeamObjective` tracks a goal within a scenario.

- `ObjectiveStatus`: Pending, InProgress, Achieved, Failed
- `ObjectiveTracker`: aggregate tracking, progress ratio, per-scenario filtering

### detection
`DetectionEvent` models a security control firing during a scenario.

- `DetectionSource`: Siem, Edr, Ids, NetworkMonitor, ManualReview, HoneyToken
- `DetectionLog`: true/false positive filtering, per-source and per-scenario queries, detection rate

### store
`ScenarioStore`: in-memory store with tenant, kind, and status filtering.

### audit
`RedTeamAuditLog` records every significant red team action.

- `RedTeamAction`: ScenarioCreated, ScenarioStarted, ScenarioCompleted, ScenarioAborted, AttackStepExecuted, ObjectiveAchieved, DetectionLogged, ReportGenerated

### stats
`RedTeamStats::compute(scenarios, attacks, detections)` produces:
- `success_rate`, `detection_rate`, `evasion_rate()`
- Per-category counts

### report
`RedTeamReport::generate(store, attacks, detections, objectives, audit, tick)` produces a point-in-time snapshot including `objective_progress()`.

### builder
`ScenarioBuilder`, `AttackStepBuilder`, `ObjectiveBuilder` provide fluent construction.

### presets
Ready-made scenarios, attack logs, detection logs, and objective trackers:
- `priv_esc_scenario`, `lateral_movement_scenario`, `exfil_scenario`
- `standard_objectives`, `network_attack_steps`, `siem_detections`

## Display strings

| Type | Variant | Display |
|------|---------|---------|
| ScenarioKind | PrivilegeEscalation | PRIVILEGE_ESCALATION |
| ScenarioKind | LateralMovement | LATERAL_MOVEMENT |
| ScenarioKind | DataExfiltration | DATA_EXFILTRATION |
| AttackVector | Network | NETWORK |
| AttackOutcome | PartialSuccess | PARTIAL_SUCCESS |
| DetectionSource | NetworkMonitor | NETWORK_MONITOR |
| ObjectiveStatus | InProgress | IN_PROGRESS |
| RedTeamAction | AttackStepExecuted | ATTACK_STEP_EXECUTED |
