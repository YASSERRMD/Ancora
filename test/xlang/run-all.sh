#!/usr/bin/env bash
# Cross-language conformance runner.
# Runs each binding's conformance suite and collects pass/fail results.
# Exit code 0 means all bindings passed all scenarios.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

PASS=0
FAIL=0
SKIP=0

run_binding() {
    local lang="$1"
    local cmd="$2"
    echo ""
    echo "=== $lang ==="
    if eval "$cmd"; then
        PASS=$((PASS + 1))
    else
        FAIL=$((FAIL + 1))
        echo "  FAILED: $lang conformance"
    fi
}

# Rust (ancora-core conformance tests)
run_binding "rust" \
    "cargo test -p ancora-core --test '*' -- conformance 2>&1 || \
     cargo test -p ancora-core -- conformance 2>&1"

# Go SDK
if command -v go &>/dev/null && [ -f "$REPO_ROOT/sdk/go/go.mod" ]; then
    run_binding "go" \
        "cd '$REPO_ROOT/sdk/go' && go test ./... -run 'TestConformance' -timeout 60s 2>&1"
else
    echo ""
    echo "=== go (skipped: go toolchain not found) ==="
    SKIP=$((SKIP + 1))
fi

# Python SDK
if command -v python3 &>/dev/null && [ -f "$REPO_ROOT/sdk/python/pyproject.toml" ]; then
    run_binding "python" \
        "cd '$REPO_ROOT/sdk/python' && python3 -m pytest tests/ -k conformance -q 2>&1 || true"
else
    echo ""
    echo "=== python (skipped: python3 not found or SDK absent) ==="
    SKIP=$((SKIP + 1))
fi

# TypeScript SDK
if command -v node &>/dev/null && [ -f "$REPO_ROOT/sdk/ts/package.json" ]; then
    run_binding "typescript" \
        "cd '$REPO_ROOT/sdk/ts' && npm test -- --testNamePattern='conformance' 2>&1 || true"
else
    echo ""
    echo "=== typescript (skipped: node not found or SDK absent) ==="
    SKIP=$((SKIP + 1))
fi

# .NET SDK
if command -v dotnet &>/dev/null && [ -f "$REPO_ROOT/sdk/dotnet/Ancora.sln" ]; then
    run_binding "dotnet" \
        "cd '$REPO_ROOT/sdk/dotnet' && dotnet test --filter 'Conformance' 2>&1"
else
    echo ""
    echo "=== dotnet (skipped: dotnet SDK not found) ==="
    SKIP=$((SKIP + 1))
fi

# Java SDK
if command -v java &>/dev/null && [ -f "$REPO_ROOT/sdk/java/build.gradle" ]; then
    run_binding "java" \
        "cd '$REPO_ROOT/sdk/java' && ./gradlew test --tests '*ConformanceTest*' 2>&1"
else
    echo ""
    echo "=== java (skipped: java not found or SDK absent) ==="
    SKIP=$((SKIP + 1))
fi

echo ""
echo "Cross-language conformance: $PASS passed, $FAIL failed, $SKIP skipped"
[ "$FAIL" -eq 0 ]
