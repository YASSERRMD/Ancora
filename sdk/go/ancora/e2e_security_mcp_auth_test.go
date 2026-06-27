package ancora_test

import (
	"errors"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

var errUnauthorized = errors.New("unauthorized: missing API key")

func fixtureAuthenticatedMCPTool(input []byte) ([]byte, error) {
	return []byte(`{"result":"authenticated response"}`), nil
}

func fixtureUnauthenticatedMCPTool(input []byte) ([]byte, error) {
	return nil, errUnauthorized
}

func TestE2ESecurityMCPAuthenticatedToolReturnsResult(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("authed-mcp", fixtureAuthenticatedMCPTool)

	out, err := reg.Invoke("authed-mcp", []byte(`{"query":"test"}`))
	if err != nil {
		t.Fatalf("authed-mcp Invoke: %v", err)
	}
	if !strings.Contains(string(out), "authenticated response") {
		t.Fatalf("expected 'authenticated response', got: %s", out)
	}
}

func TestE2ESecurityMCPUnauthenticatedToolReturnsError(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("unauthed-mcp", fixtureUnauthenticatedMCPTool)

	_, err := reg.Invoke("unauthed-mcp", []byte(`{}`))
	if err == nil {
		t.Fatal("unauthenticated MCP tool must return error")
	}
	if !errors.Is(err, errUnauthorized) {
		t.Fatalf("expected errUnauthorized, got: %v", err)
	}
}

func TestE2ESecurityMCPUnregisteredToolIsRefused(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	_, err := reg.Invoke("unregistered-mcp", []byte(`{}`))
	if err == nil {
		t.Fatal("unregistered MCP tool must return error")
	}
}

func TestE2ESecurityMCPAuthErrorIsStoredInJournal(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("mcp-auth-run")
	_ = store.AppendEvent("mcp-auth-run", 0,
		`{"type":"error","code":"UNAUTHORIZED","message":"missing API key","detail":""}`)

	events, _ := store.EventsForRun("mcp-auth-run")
	if len(events) != 1 {
		t.Fatalf("expected 1 auth error event, got: %d", len(events))
	}
	if !strings.Contains(events[0], "UNAUTHORIZED") {
		t.Fatalf("event must contain UNAUTHORIZED, got: %s", events[0])
	}
}

func TestE2ESecurityMCPAuthErrorCodeIsDistinctFromRateLimit(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("auth-vs-rl")
	_ = store.AppendEvent("auth-vs-rl", 0, `{"type":"error","code":"UNAUTHORIZED"}`)
	_ = store.AppendEvent("auth-vs-rl", 1, `{"type":"error","code":"RATE_LIMIT"}`)

	events, _ := store.EventsForRun("auth-vs-rl")
	if !strings.Contains(events[0], "UNAUTHORIZED") {
		t.Fatalf("event 0 must be UNAUTHORIZED, got: %s", events[0])
	}
	if !strings.Contains(events[1], "RATE_LIMIT") {
		t.Fatalf("event 1 must be RATE_LIMIT, got: %s", events[1])
	}
}

func TestE2ESecurityMCPToolkitRefusesCallToUnregisteredTool(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)

	_, err := tk.InvokeTool("not-registered", []byte(`{}`))
	if err == nil {
		t.Fatal("invoking unregistered tool via toolkit must return error")
	}
}

func TestE2ESecurityMCPRegistryCountZeroBeforeRegistration(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	if reg.Count() != 0 {
		t.Fatalf("expected 0 tools before any registration, got: %d", reg.Count())
	}
}

func TestE2ESecurityMCPUnregisterRemovesAccess(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("temp-tool", fixtureAuthenticatedMCPTool)
	reg.Unregister("temp-tool")

	_, err := reg.Invoke("temp-tool", []byte(`{}`))
	if err == nil {
		t.Fatal("unregistered tool must not be invocable")
	}
}
