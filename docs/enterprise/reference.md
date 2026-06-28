# ancora-enterprise Reference

Enterprise checkpoint module for Ancora. Provides license management, feature gates, security posture assessment, incident coordination, health checkpoints, and consolidated enterprise reporting.

## Modules

### license
`EnterpriseLicense` manages tier, capabilities, and expiry.

- `LicenseTier`: Community, Professional, Enterprise, GovCloud
- `EnterpriseCap` (10 capabilities): Hsm, AirGap, RedTeamSim, PentestAutomation, AdvancedCompliance, SsoIntegration, AuditExport, MultiRegion, CustomRoles, ThreatIntelFeed
- `is_valid(tick)`, `is_expired(tick)`, `has_cap(cap)`, `is_enterprise_or_above()`

### feature
`FeatureRegistry` holds `FeatureFlag` entries with `FeatureState` (Enabled/Disabled/BetaOnly/Deprecated).

- `is_enabled(name)`, `enable(name)`, `disable(name)`, `enabled_count()`

### posture
`SecurityPosture` aggregates `DomainScore` entries into an overall score and `PostureLevel`.

- `PostureLevel`: Critical (0-29), Poor (30-49), Fair (50-69), Good (70-84), Excellent (85+)
- `overall_score()`, `posture_level()`, `total_critical_findings()`

### incident
`EnterpriseIncident` tracks security incidents across domains.

- `IncidentSeverity`: Low, Medium, High, Critical
- `IncidentStatus`: Open, Investigating, Contained, Resolved, Closed
- Lifecycle: `investigate()`, `contain()`, `resolve(tick)`, `close()`
- `IncidentLog`: filtering by tenant, severity, resolution status

### checkpoint
`EnterpriseCheckpoint` aggregates `HealthCheck` entries.

- `CheckStatus`: Pass, Warn, Fail, NotApplicable
- `all_healthy()`, `pass_rate()`, `failing()`, `warnings()`, `for_domain()`

### audit
`EnterpriseAuditLog` with 9 `EnterpriseAction` variants.

### stats
`EnterpriseStats::compute(licenses, incidents, checkpoint, posture, tick)`:
- `health_score()`: combined metric (posture + check rate, penalized by critical incidents)

### report
`EnterpriseReport::generate(tenant_id, ...)`: per-tenant point-in-time view with `is_healthy()`.

### builder
`LicenseBuilder`, `IncidentBuilder`, `HealthCheckBuilder`, `DomainScoreBuilder`.

### presets
- `enterprise_license()`: full 10-cap enterprise license
- `community_license()`: no-cap community tier
- `default_feature_registry()`: 7 features (5 enabled, 1 beta, 1 disabled)
- `standard_checkpoint()`: 5 checks (4 pass, 1 warn)
- `healthy_posture()`: 4 domains averaging 82.5 (Good)

## Display strings

| Type | Variant | Display |
|------|---------|---------|
| LicenseTier | GovCloud | GOV_CLOUD |
| EnterpriseCap | AirGap | AIR_GAP |
| EnterpriseCap | RedTeamSim | RED_TEAM_SIM |
| FeatureState | BetaOnly | BETA_ONLY |
| PostureLevel | Critical | CRITICAL |
| IncidentStatus | Investigating | INVESTIGATING |
| CheckStatus | NotApplicable | NOT_APPLICABLE |
| EnterpriseAction | CheckpointRun | CHECKPOINT_RUN |
| EnterpriseAction | PostureAssessed | POSTURE_ASSESSED |
