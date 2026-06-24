# conformance-runner

Runs all canonical Ancora conformance scenarios against the local CGO transport
and prints a pass/fail summary.

## Run

```bash
go run ./examples/conformance-runner
```

## What it shows

- Creating a `ConformanceSuite` with any `Transport`
- Calling `RunAll` to execute every built-in scenario
- Checking `ConformanceResult.Passed` and printing a summary
- Exiting non-zero when any scenario fails
