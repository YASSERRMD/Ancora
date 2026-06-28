# App Smoke Tests in CI

The `ancora-apps` crate includes a full test suite that runs in CI without
any network access.

## GitHub Actions Workflow

Add the following job to your workflow:

```yaml
apps-smoke-tests:
  name: App Gallery Smoke Tests
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - name: Build ancora-apps
      run: cargo build -p ancora-apps
    - name: Run ancora-apps tests
      run: cargo test -p ancora-apps
```

## Test Coverage

| Test file | App | Scenario |
|-----------|-----|----------|
| test_docqa_offline | document-qa | offline QA, empty store, unknown query |
| test_research_offline | research-assistant | by topic, by tag, unknown topic |
| test_coding_offline | coding-assistant | snippet lookup, cross-language, stub gen |
| test_dataanalysis_offline | data-analysis | stats, top-N, error cases |
| test_support_offline | customer-support | routing, default, lifecycle |
| test_compliance_airgapped | compliance-review | pass, missing class., secret, severity |
| test_apps_emit_traces | all apps | span emission, cost accumulation |
| test_apps_pass_guardrails | all apps | allow, block, redact |

All tests execute in milliseconds and require no external processes.
