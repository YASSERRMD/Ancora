package ancora_test

import (
	"context"
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

type mcpE2EResult struct {
	Content string `json:"content"`
	Source  string `json:"source"`
}

func fixtureMCPSearch(input []byte) ([]byte, error) {
	result := mcpE2EResult{
		Content: "Ancora is an agent runtime framework.",
		Source:  "fixture-mcp-server",
	}
	return json.Marshal(result)
}

func TestE2EMCPToolCallReturnsContent(t *testing.T) {
	out, err := fixtureMCPSearch([]byte(`{"query":"ancora"}`))
	if err != nil {
		t.Fatalf("fixtureMCPSearch: %v", err)
	}
	if !strings.Contains(string(out), "Ancora") {
		t.Fatalf("MCP output must contain 'Ancora', got: %s", out)
	}
}

func TestE2EMCPToolRegisteredInToolkit(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("mcp-search", fixtureMCPSearch)
	if !tk.Tools().Has("mcp-search") {
		t.Fatal("toolkit must have mcp-search")
	}
}

func TestE2EMCPAgentSpecHasSearchTool(t *testing.T) {
	searchTool := ancora.NewToolSpec("mcp-search", "MCP-backed web search")
	spec := ancora.NewAgentSpecBuilder().
		WithName("mcp-agent").
		WithModelID("gpt-4o").
		WithTool(searchTool).
		Build()
	if len(spec.GetTools()) != 1 {
		t.Fatalf("expected 1 tool, got: %d", len(spec.GetTools()))
	}
	if spec.GetTools()[0].GetName() != "mcp-search" {
		t.Fatalf("tool name mismatch: %q", spec.GetTools()[0].GetName())
	}
}

func TestE2EMCPRunWithToolStartsSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	searchTool := ancora.NewToolSpec("mcp-search", "MCP-backed web search")
	spec := ancora.NewAgentSpecBuilder().
		WithName("mcp-agent").
		WithModelID("llama3").
		WithTool(searchTool).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestE2EMCPToolInvokedViaRegistryReturnsJSON(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("mcp-search", fixtureMCPSearch)

	out, err := reg.Invoke("mcp-search", []byte(`{"query":"test"}`))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	var result mcpE2EResult
	if err := json.Unmarshal(out, &result); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if result.Source != "fixture-mcp-server" {
		t.Fatalf("source mismatch: %q", result.Source)
	}
}

func TestE2EMCPToolCallAndResultStoredInJournal(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("mcp-e2e-run")
	_ = store.AppendEvent("mcp-e2e-run", 0, `{"type":"tool_called","tool":"mcp-search"}`)
	_ = store.AppendEvent("mcp-e2e-run", 1, `{"type":"tool_result","tool":"mcp-search","content":"fixture"}`)

	events, _ := store.EventsForRun("mcp-e2e-run")
	if len(events) != 2 {
		t.Fatalf("expected 2 MCP events, got: %d", len(events))
	}
}

func TestE2EMCPStoringTransportRecordsMCPRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	for {
		ev, _ := tr.PollRun(context.Background(), runID)
		if ev == nil {
			break
		}
	}

	count, err := store.EventCount(runID)
	if err != nil {
		t.Fatalf("EventCount: %v", err)
	}
	if count == 0 {
		t.Fatal("MCP run must store at least one event")
	}
}

func TestE2EMCPGRPCTransportStartsRun(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun via gRPC: %v", err)
	}
	if runID == "" {
		t.Fatal("gRPC run ID must be non-empty")
	}
}

func TestE2EMCPUnregisteredToolReturnsError(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	_, err := reg.Invoke("mcp-not-registered", []byte(`{}`))
	if err == nil {
		t.Fatal("unregistered MCP tool must return error")
	}
}
