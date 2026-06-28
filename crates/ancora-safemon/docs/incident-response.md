# Incident Response for Safety

This guide describes how to respond to safety incidents detected by ancora-safemon.

## Severity Levels

| Level    | Description                              | Response Time |
|----------|------------------------------------------|---------------|
| Info     | Informational, no immediate action       | Best effort   |
| Low      | Minor issue, monitor for recurrence      | 24 hours      |
| Medium   | Moderate concern, investigate            | 4 hours       |
| High     | Serious issue, immediate investigation   | 1 hour        |
| Critical | Critical breach, immediate escalation    | Immediate     |

## Incident Lifecycle

1. Detection - classifier identifies a safety issue
2. Logging - incident recorded in IncidentLog with full context
3. Alerting - AlertManager fires to all registered channels above threshold
4. Investigation - operator reviews redacted excerpt and agent context
5. Resolution - root cause identified, agent retrained or blocked
6. Closure - incident marked resolved in audit trail

## Response Procedures

### PII Exposure (HIGH/CRITICAL)

1. Retrieve incident from IncidentLog using incident ID
2. Identify the agent session responsible (agent_id field)
3. Quarantine agent output - do not propagate to downstream systems
4. Notify data protection officer if personal data was exposed externally
5. Review agent training data for PII leakage source
6. Apply output redaction rules to future outputs from that agent

### Toxic Content (MEDIUM/HIGH)

1. Review matched terms in toxicity report
2. Determine if context is technical (e.g., "kill process") or genuine toxicity
3. If genuine, retrain agent with cleaned training data
4. Add terms to monitoring watchlist for ongoing alerting

### Policy Violation (HIGH/CRITICAL)

1. Identify which policy rule was triggered (rule_id field)
2. Review the full output excerpt to assess severity
3. Block agent from further output if violation is confirmed
4. File compliance incident report per organizational policy

## Audit Requirements

All incidents are retained in the IncidentLog for the configured retention period.
Export incident logs as JSON for compliance audits:

```rust
let json = log.to_json();
// Write to audit store
```
