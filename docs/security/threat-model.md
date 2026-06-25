# Ancora Security: Threat Model and Posture

## Scope

This document describes the security properties of the Ancora core framework,
covering the Rust engine, FFI layer, A2A protocol implementation, MCP server,
and policy engine. It does not cover model inference infrastructure or
host-OS hardening.

## Trust Boundaries

```
 User code         |  Ancora boundary          |  External
 (agent graph,     |  (engine, journal,        |  (model APIs,
  tool impls)      |   MCP server, policy)     |   remote agents)
                   |                           |
 trusted           |  semi-trusted             |  untrusted
```

Tools and agents supplied by the operator are treated as trusted. Remote
agents contacted via A2A, external model endpoints, and all network input
are treated as untrusted.

## Threat Catalogue

### T1 - Prompt / Tool Poisoning

**Description.** A malicious user or a compromised upstream model injects
instructions or SQL-/shell-injection payloads into tool input fields, trying
to subvert the tool or the framework.

**Mitigation.**
- Tool inputs are treated as data, not as code. Ancora never `eval`s or
  shells out using raw input strings.
- The `ToolRegistry::call` pipeline validates every input against the tool's
  JSON Schema before dispatch. Inputs missing required fields are rejected
  with `ToolError::ValidationFailed` before the tool function is entered.
- Test coverage: `crates/ancora-tools/tests/tool_poisoning_tests.rs`
  (7 tests: injection strings, oversized input, null values, name injection).

**Residual risk.** Individual tool implementations may still be vulnerable
to injection if they pass user input to a subprocess or external API. This
is the tool author's responsibility; Ancora provides the input-validation
hook but cannot audit tool internals.

### T2 - Unauthenticated MCP Access

**Description.** An attacker on the local network or on a shared host sends
JSON-RPC requests to the MCP server and calls or lists tools without
authentication.

**Mitigation.**
- The MCP server accepts an optional static bearer token via
  `McpServer::with_token(token)`. When configured, every HTTP request is
  checked for `Authorization: Bearer <token>` before any JSON-RPC dispatch.
  Non-matching or missing tokens return `HTTP 401` with an empty body,
  leaking no schema or error details.
- The bearer comparison is case-insensitive on the scheme name and exact
  on the token value (no timing sidechannel in the current implementation;
  a constant-time comparison should be substituted for production use).
- Test coverage: `crates/ancora-tools/tests/auth_rejection_tests.rs`
  (7 tests covering missing header, wrong token, empty value, padded value,
  Basic scheme, and correct token for both `tools/list` and `tools/call`).

**Residual risk.** Static bearer tokens do not rotate. For high-security
deployments, front the MCP server with mTLS or a sidecar that handles
token issuance and rotation.

### T3 - Unauthorized Egress (Data Exfiltration)

**Description.** A compromised tool or agent sends sensitive data to an
external endpoint outside the operator's approved list.

**Mitigation.**
- The `ancora-policy` crate provides `check_endpoint(policy, url)`.
- `Policy::air_gapped()` blocks all outbound calls unconditionally,
  returning `PolicyError::EgressBlocked`. This overrides any
  `allow_endpoint` entries, making air-gapped mode tamper-resistant.
- `Policy::allow_endpoint(prefix)` implements allowlist-based egress:
  only URLs that start with an approved prefix are permitted.
- Test coverage: `crates/ancora-policy/tests/air_gapped_tests.rs`
  (8 tests: unconditional block, localhost block, allow-list override,
  residency violation, open policy).

**Residual risk.** `check_endpoint` is advisory: it must be called at every
egress site. If a tool makes HTTP calls without going through the policy
engine, the check is bypassed. A network-level control (firewall, eBPF
policy) should be added for high-assurance deployments.

### T4 - Replay Tampering

**Description.** An attacker modifies the journal to alter the replay of a
prior run, causing the agent to re-execute activities with false history.

**Mitigation.**
- The journal is an append-only log. Events carry monotonically increasing
  `seq` numbers and are keyed by `run_id`. The replay engine (`replay_events`)
  reads the journal in seq order and rejects events whose `run_id` does not
  match the active run.
- For durable stores (SQLite, PostgreSQL) the underlying store should enforce
  append-only constraints at the DB layer.
- Journal masking (`mask_events`, `assert_structurally_equal`) strips model
  content before cross-language comparison, preventing content-based attacks
  on the comparison pipeline.

**Residual risk.** The in-memory store has no persistence. Tampering with a
`MemoryStore` at the Rust level is equivalent to compromising the process.

### T5 - A2A Identity Spoofing

**Description.** A malicious agent presents a forged `AgentCard` claiming to
be a trusted agent.

**Mitigation.**
- `AgentIdentity::generate()` creates a fresh Ed25519 keypair.
- `sign_card(identity, card)` attaches a base64-encoded signature of the
  serialised card JSON.
- `verify_card(card)` rejects cards whose signature does not verify against
  the embedded public key.
- `A2aClient::fetch_and_verify_card()` verifies before trusting a remote
  card; `HandoffRequest::require_signed_identity = true` enforces this for
  agent handoffs.
- Test coverage: `crates/ancora-grpc/tests/a2a_tests.rs` and
  `a2a_client_tests.rs`.

**Residual risk.** Ed25519 public keys in agent cards are self-signed; there
is no PKI or certificate authority. A TOFU (trust on first use) approach or
an out-of-band key registry is needed for high-security deployments.

### T6 - PII Leakage in Journal

**Description.** Sensitive fields (email addresses, API keys, credit card
numbers) flow through the journal unredacted and are stored or transmitted
in plaintext.

**Mitigation.**
- `Policy::require_pii_redaction = true` triggers `pii::redact_journal`
  before events are committed.
- PII detection is pattern-based (regex on known field shapes).

**Residual risk.** Pattern-based detection has false negatives. For regulated
data, a DLP service should scan the journal before archival.

## Security Controls Summary

| Control | Mechanism | Test file |
|---------|-----------|-----------|
| Input validation | `schema::validate_input`, JSON Schema | `tool_poisoning_tests.rs` |
| MCP authentication | bearer token, HTTP 401 | `auth_rejection_tests.rs` |
| Egress restriction | `policy::air_gapped`, allowlist | `air_gapped_tests.rs` |
| Agent identity | Ed25519 sign/verify | `a2a_tests.rs`, `a2a_client_tests.rs` |
| Replay integrity | append-only journal, seq ordering | `chaos_kill_resume.rs` |
| PII redaction | `pii::redact_journal`, policy flag | unit tests in `pii.rs` |

## Out of Scope

- Model inference security (prompt injection at the LLM layer).
- OS-level isolation (containers, seccomp, AppArmor).
- Cryptographic key management and rotation.
- Distributed denial-of-service protection.

## Reporting

Report security issues by filing a private advisory in the repository or
emailing the maintainers directly. Do not open public GitHub issues for
vulnerability reports.
