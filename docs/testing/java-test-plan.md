# Java SDK Test Plan

## Overview

This document describes how the Ancora Java SDK test suite is structured, how to run it locally, and what the CI enforces.

## Test Layers

| Layer | Package pattern | Requires native lib |
|-------|----------------|---------------------|
| Unit | `Phase148*Test` | No (offline assertions) |
| Integration | `Phase148*Test` (live paths) | Yes (skipped via `Assumptions`) |
| E2E | `Phase149*Test` | Yes (skipped via `Assumptions`) |
| Conformance | `Phase149Conf*Test` | Yes |
| Reliability | `Phase149Rel*Test` | Yes |
| Security | `Phase149Sec*Test` | Partial |
| Performance | `Phase149Perf*Test` | Yes |

## Running Tests Locally

```bash
cd sdk/java

# Unit tests only (no native lib needed)
gradle test

# With native lib present
ANCORA_NATIVE_LIB_PATH=/path/to/target/release gradle test

# Coverage report
gradle jacocoTestReport
open build/reports/jacoco/test/html/index.html
```

## CI Pipeline

The `java-ci.yml` workflow runs on every push or PR touching `sdk/java/**`:

1. Builds `ancora-ffi` cdylib via `cargo build -p ancora-ffi --release`
2. Runs `gradle test jacocoTestReport` with `ANCORA_NATIVE_LIB_PATH` set
3. Parses `build/reports/jacoco/test/jacocoTestReport.xml` and enforces 60% line coverage

The `java-e2e-ci.yml` workflow runs the full suite including e2e and reliability tests.

## Native Library Skip Pattern

All tests that invoke FFI code use:

```java
private static void skipIfAbsent() {
    Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
}
```

Call `skipIfAbsent()` at the top of any test that needs the native library. Integration test try-catch blocks also guard `UnsatisfiedLinkError`.

## Coverage Gate

The minimum required line coverage is **60%**. The gate is enforced by parsing the JaCoCo XML report in CI.

## Offline Guarantee

All tests run offline. No live HTTP calls are made. External service interactions use:
- In-memory fixtures (`Phase148RagRetrievalMilvusTest`, `Phase149E2eRagMilvusTest`)
- Map-based mock implementations (`Phase149E2eMcpTest`)
- Java records for data modeling (no I/O)
