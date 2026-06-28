# App Gallery Overview

The `ancora-apps` crate ships a gallery of six full sample applications.
Every app runs offline; the compliance-review app supports fully air-gapped
government environments.

## Apps

| ID | Name | Category | Offline | Air-gapped |
|----|------|----------|---------|------------|
| document-qa | Document QA | Document Processing | Yes | Yes |
| research-assistant | Research Assistant | Research | Yes | Yes |
| coding-assistant | Coding Assistant | Coding | Yes | Yes |
| data-analysis | Data Analysis | Data Analysis | Yes | Yes |
| customer-support | Customer Support | Support | Yes | No |
| compliance-review | Compliance Review (Government) | Compliance | Yes | Yes |

## Design Principles

- Local-first: all apps resolve models through the `local_models` registry.
- Safety: all outputs pass through `safety::SafetyGuardrail` before returning.
- Observability: all apps emit `traces::Span` records for cost accounting.
- Zero network calls: the test suite verifies offline operation.
