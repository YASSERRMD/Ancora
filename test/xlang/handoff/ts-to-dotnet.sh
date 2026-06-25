#!/usr/bin/env bash
# Simulates a TypeScript agent handing off to a .NET agent via A2A.
#
# Requires:
#   - node / npm (TypeScript SDK)
#   - dotnet SDK (>= 8)
#   - Ancora native library built: cargo build -p ancora-ffi --release
#
# The .NET agent serves an agent card at http://localhost:9001/.well-known/agent.json.
# The TypeScript agent fetches that card, verifies the identity, and submits a task.
# Both agents print their run events; the harness checks exit codes.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
DOTNET_PORT=9001

cleanup() {
    [ -n "${DOTNET_PID:-}" ] && kill "$DOTNET_PID" 2>/dev/null || true
}
trap cleanup EXIT

echo "=== Starting .NET A2A server on port $DOTNET_PORT ==="
if ! command -v dotnet &>/dev/null; then
    echo "SKIP: dotnet SDK not found"
    exit 0
fi

dotnet run --project "$REPO_ROOT/sdk/dotnet/Ancora" -- \
    --serve-a2a "$DOTNET_PORT" &
DOTNET_PID=$!
sleep 1

echo "=== Running TypeScript handoff agent ==="
if ! command -v node &>/dev/null; then
    echo "SKIP: node not found"
    exit 0
fi

cd "$REPO_ROOT/sdk/ts"
node -e "
const { A2aClient } = require('./dist/a2a-client');
async function main() {
    const client = new A2aClient('http://localhost:${DOTNET_PORT}');
    const card = await client.fetchAndVerifyCard();
    console.log('Remote agent:', card.name);
    const task = await client.submitTask('ts-to-dotnet-001', 'Translate to French.');
    console.log('Task queued:', task.id);
}
main().catch(e => { console.error(e); process.exit(1); });
" 2>&1

echo ""
echo "=== Polyglot handoff (TS -> .NET): PASSED ==="
