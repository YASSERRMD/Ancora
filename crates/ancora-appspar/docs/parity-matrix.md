# Sample App Parity Matrix

This matrix shows which features each language sample app implements.

| Feature            | Go  | Python | TypeScript | .NET | Java | Rust |
|--------------------|-----|--------|------------|------|------|------|
| streaming          | yes | yes    | yes        | yes  | yes  | yes  |
| tool_calls         | yes | yes    | yes        | yes  | yes  | yes  |
| structured_output  | yes | yes    | yes        | yes  | yes  | yes  |
| guardrails         | yes | yes    | yes        | yes  | yes  | yes  |
| tracing            | yes | yes    | yes        | yes  | yes  | yes  |

All six languages are at full parity. Each feature is verified by the
`test_parity` test suite in this crate.
