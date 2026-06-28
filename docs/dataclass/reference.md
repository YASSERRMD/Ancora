# ancora-dataclass: Data Classification

`ancora-dataclass` provides offline-first data classification with sensitivity labels, write/read enforcement, and audit logging for Ancora agents.

## Sensitivity Levels

Ordered from lowest to highest:

| Level | Numeric | Display |
|-------|---------|---------|
| `Public` | 0 | `PUBLIC` |
| `Internal` | 1 | `INTERNAL` |
| `Confidential` | 2 | `CONFIDENTIAL` |
| `Restricted` | 3 | `RESTRICTED` |
| `TopSecret` | 4 | `TOP_SECRET` |

Levels implement `PartialOrd`/`Ord` so you can compare directly with `<`, `>`, `>=`.

## Data Categories

`DataCategory` variants: `Pii`, `Financial`, `Health`, `Credentials`, `Intellectual`, `Operational`, `Generic`.

## DataRecord

A classified data item. Build with `DataRecordBuilder`:

```rust
let record = DataRecordBuilder::new("user-ssn-001", "acme", "Employee SSN")
    .level(SensitivityLevel::Restricted)
    .category(DataCategory::Pii)
    .tick(100)
    .tag("gdpr")
    .tag("hipaa")
    .build();
```

## Classification Policy

`ClassificationPolicy` sets the maximum allowed sensitivity level and write controls per tenant:

```rust
let policy = ClassificationPolicy::strict("acme");
// max_allowed_level: Confidential
// require_category_tag: true
// deny_public_write: true
```

Presets: `strict()`, `permissive()`, or `new(tenant, max_level)` for custom.

## ClassificationEnforcer

```rust
let write_decision = ClassificationEnforcer::check_write(&policy, &record);
let read_decision  = ClassificationEnforcer::check_read(&policy, &record, &clearance);
```

`check_write` validates level against policy ceiling and tag requirements.
`check_read` validates the subject's clearance against the record's level.

## DataRegistry

Multi-tenant in-memory store. Rejects duplicate `id` values. Supports `by_tenant()` and `at_or_above(level)` queries.

## DataQuery

Chainable filter builder:

```rust
let results = DataQuery::new()
    .min_level(SensitivityLevel::Confidential)
    .category(DataCategory::Pii)
    .tag("gdpr")
    .run(registry.all());
```

## Downgrade Policy

`DowngradePolicy` enforces a minimum level floor when downgrading records:

```rust
let pol = DowngradePolicy::new(SensitivityLevel::Internal);
pol.apply(&mut record, SensitivityLevel::Public);
// DowngradeResult::Denied -- below minimum
```

## Redaction

`RedactionConfig` masks sensitive values at export time:

```rust
let config = RedactionConfig::new(SensitivityLevel::Confidential).with_mask("[REDACTED]");
let safe_value = config.apply(&raw_value, &record.level);
```

## Export

`to_csv` and `to_json` serialize `&[&DataRecord]` for reporting.
