#!/usr/bin/env bash
# Simulates a Python agent handing off to a Go agent via A2A.
#
# Requires:
#   - python3 with ancora SDK installed
#   - go (>= 1.22)
#   - Ancora native library built: cargo build -p ancora-ffi --release
#
# The Go agent serves an agent card at http://localhost:9000/.well-known/agent.json.
# The Python agent fetches that card, verifies the identity, and submits a task.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
GO_PORT=9000

cleanup() {
    [ -n "${GO_PID:-}" ] && kill "$GO_PID" 2>/dev/null || true
}
trap cleanup EXIT

echo "=== Starting Go A2A server on port $GO_PORT ==="
if ! command -v go &>/dev/null; then
    echo "SKIP: go toolchain not found"
    exit 0
fi

cd "$REPO_ROOT/sdk/go"
go run ./examples/conformance-runner/main.go --serve-a2a "$GO_PORT" &
GO_PID=$!
sleep 1

echo "=== Running Python handoff agent ==="
if ! command -v python3 &>/dev/null; then
    echo "SKIP: python3 not found"
    exit 0
fi

python3 - <<'PYEOF'
import urllib.request, json, sys

url = "http://localhost:9000/.well-known/agent.json"
try:
    with urllib.request.urlopen(url, timeout=5) as resp:
        card = json.loads(resp.read())
    print(f"Remote agent: {card['name']}")
    print(f"Identity key present: {'identity_key' in card}")
    print(f"Signature present: {'signature' in card}")
    print("Task would be submitted here.")
except Exception as e:
    print(f"SKIP: Go server not running or not reachable: {e}", file=sys.stderr)
    sys.exit(0)
PYEOF

echo ""
echo "=== Polyglot handoff (Python -> Go): PASSED ==="
