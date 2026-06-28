# ancora-supplychain Reference

Supply chain security with dependency signing, SBOM generation, provenance tracking, and compliance reporting for the Ancora platform.

## Core Types

### `Component`
Represents a single software dependency.

| Field | Type | Description |
|---|---|---|
| `id` | `String` | Component identifier |
| `name` | `String` | Package name |
| `version` | `String` | Package version |
| `kind` | `ComponentKind` | Component type |
| `license` | `License` | SPDX license |
| `supplier` | `String` | Originating organization |
| `digest` | `String` | Content hash |

### `ComponentKind`
`Library` | `Binary` | `Container` | `OsPackage` | `Framework` | `Service`

### `License`
`Mit` | `Apache2` | `Gpl3` | `Bsd2` | `Bsd3` | `Proprietary` | `Unknown`

## SBOM

`Sbom` collects components into a software bill of materials.

```rust
let mut sbom = Sbom::new("sbom-1", "tenant", SbomFormat::CycloneDx, tick);
sbom.add_component(component);
let found = sbom.find_by_name("openssl");
let prop_count = sbom.proprietary_count();
```

### `SbomFormat`
`CycloneDx` | `Spdx` | `Internal`

## Signatures

```rust
let mut store = SignatureStore::new();
store.register(ComponentSignature::new("comp-id", SignatureAlgorithm::Ed25519, "signer", "sig-bytes", tick));
let result = store.verify("comp-id", "expected-sig");
// VerificationResult: Valid | Invalid(String) | Missing
```

## Provenance

```rust
let mut store = ProvenanceStore::new();
store.record(ProvenanceRecord::new("comp-id", ProvenanceKind::BuildSystem, "ci-url", "build-123", tick));
assert!(store.has_provenance("comp-id"));
let by_vcs = store.by_kind(&ProvenanceKind::Vcs);
```

### `ProvenanceKind`
`BuildSystem` | `Vcs` | `Registry` | `ArtifactStore`

## Policy

```rust
let policy = SupplyChainPolicy::new("tenant")
    .deny_license(License::Gpl3)
    .require_signature()
    .require_provenance()
    .allow_supplier("trusted-vendor");

let decision = policy.check_component(&component, has_sig, has_prov);
// PolicyDecision: Allow | Deny(String)
```

## Compliance Report

```rust
let report = SupplyChainReport::generate(&sbom, &sigs, &provenance, &policy, tick);
assert!(report.is_compliant());
println!("Sign rate: {:.0}%", report.sign_rate() * 100.0);
```

## Stats

```rust
let stats = SbomStats::from(&sbom);
println!("OSS rate: {:.0}%", stats.oss_rate() * 100.0);
println!("Licenses: {:?}", stats.by_license);
```

## Builder

```rust
let c = ComponentBuilder::new("id", "libssl", "3.0.0")
    .kind(ComponentKind::Library)
    .license(License::Apache2)
    .supplier("openssl-project")
    .digest("sha256:deadbeef")
    .build();
```

## Query

```rust
let results = ComponentQuery::new()
    .kind(ComponentKind::Library)
    .open_source_only()
    .supplier("trusted-vendor")
    .run(sbom.components.iter());
```

## Audit Log

```rust
log.record(SupplyChainAuditEntry::new(tick, "tenant", "comp-id", SupplyChainEvent::ComponentSigned, "ci-bot", true));
log.for_tenant("tenant");
log.failures();
```

### `SupplyChainEvent`
`ComponentAdded` | `ComponentSigned` | `ComponentVerified` | `ProvenanceRecorded` | `SbomGenerated` | `PolicyChecked`

## Offline-First Design

All types use `u64` monotonic ticks. No network calls. No third-party dependencies.
