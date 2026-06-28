# Post-Mortem Template

**Incident ID:** INC-XXXX
**Severity:** P1 / P2
**Duration:** HH:MM
**Date:** YYYY-MM-DD
**Commander:** @name
**Author:** @name

## Impact

< 2-3 sentences: which tenants affected, what % of requests failed, what users experienced >

## Timeline

| Time (UTC) | Event |
|-----------|-------|
| HH:MM | Alert fired: <alert name> |
| HH:MM | On-call acknowledged |
| HH:MM | Root cause identified |
| HH:MM | Mitigation applied |
| HH:MM | Service restored |

## Root cause

< One clear sentence. What was the underlying technical cause? >

## Contributing factors

- < Factor 1: e.g. missing metric coverage for this code path >
- < Factor 2: e.g. circuit breaker threshold too aggressive >

## What went well

- < e.g. Automatic failover kicked in within 30 seconds >

## What could be improved

- < e.g. Queue backlog alert fired 4 minutes before user impact was visible >

## Action items

| Item | Owner | Due |
|------|-------|-----|
| Add test for X | @name | YYYY-MM-DD |
| Tune circuit breaker threshold | @name | YYYY-MM-DD |
| Add runbook for Y | @name | YYYY-MM-DD |
