package ancora_test

import (
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// mcpToolCall is a local struct representing an MCP tool call fixture.
type mcpToolCall struct {
	ToolName string          `json:"tool_name"`
	Input    json.RawMessage `json:"input"`
}

// mcpToolResult is a local struct representing an MCP tool result fixture.
type mcpToolResult struct {
	ToolName string `json:"tool_name"`
	Output   string `json:"output"`
}

func TestMCPToolCallJSONRoundTrip(t *testing.T) {
	call := mcpToolCall{ToolName: "web_search", Input: json.RawMessage(`{"query":"encore agent"}`)}
	b, err := json.Marshal(call)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}
	var got mcpToolCall
	if err := json.Unmarshal(b, &got); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if got.ToolName != "web_search" {
		t.Fatalf("tool name mismatch: %q", got.ToolName)
	}
}

func TestMCPToolResultJSONRoundTrip(t *testing.T) {
	result := mcpToolResult{ToolName: "web_search", Output: "example.com result"}
	b, err := json.Marshal(result)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}
	var got mcpToolResult
	if err := json.Unmarshal(b, &got); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if got.Output != "example.com result" {
		t.Fatalf("output mismatch: %q", got.Output)
	}
}

func fixtureMCPWebSearch(input []byte) ([]byte, error) {
	result := mcpToolResult{ToolName: "web_search", Output: "fixture search result for: " + string(input)}
	return json.Marshal(result)
}

func TestMCPFixtureWebSearchReturnsOutput(t *testing.T) {
	out, err := fixtureMCPWebSearch([]byte(`{"query":"goroutines"}`))
	if err != nil {
		t.Fatalf("fixtureMCPWebSearch: %v", err)
	}
	if !strings.Contains(string(out), "fixture search result") {
		t.Fatalf("expected 'fixture search result' in output, got: %s", out)
	}
}

func TestMCPToolRegisteredInToolkit(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("web_search", fixtureMCPWebSearch)
	if !tk.Tools().Has("web_search") {
		t.Fatal("toolkit must have web_search after registration")
	}
}

func TestMCPToolInvokedViaRegistry(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("web_search", fixtureMCPWebSearch)
	out, err := reg.Invoke("web_search", []byte(`{"query":"test"}`))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	if len(out) == 0 {
		t.Fatal("MCP tool must return non-empty output")
	}
}

func TestMCPToolSpecCanBeAdded(t *testing.T) {
	searchTool := ancora.NewToolSpecBuilder().
		WithToolName("web_search").
		WithDescription("Searches the web for a query and returns results").
		WithInputSchema(`{"type":"object","properties":{"query":{"type":"string"}},"required":["query"]}`).
		Build()
	if searchTool.GetName() != "web_search" {
		t.Fatalf("tool name mismatch: %q", searchTool.GetName())
	}
}

func TestMCPAgentSpecWithWebSearchTool(t *testing.T) {
	searchTool := ancora.NewToolSpec("web_search", "Searches the web")
	spec := ancora.NewAgentSpecBuilder().
		WithName("mcp-agent").
		WithModelID("gpt-4o").
		WithTool(searchTool).
		Build()
	if len(spec.GetTools()) != 1 {
		t.Fatalf("expected 1 tool, got: %d", len(spec.GetTools()))
	}
	if spec.GetTools()[0].GetName() != "web_search" {
		t.Fatalf("tool name mismatch: %q", spec.GetTools()[0].GetName())
	}
}

func TestMCPRunWithToolStartsSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	searchTool := ancora.NewToolSpec("web_search", "Searches the web")
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

func TestMCPToolEventStoredInJournal(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("mcp-run-1")
	payload := `{"type":"tool_called","tool":"web_search","input":{"query":"test"}}`
	_ = store.AppendEvent("mcp-run-1", 0, payload)
	resultPayload := `{"type":"tool_result","tool":"web_search","output":"fixture result"}`
	_ = store.AppendEvent("mcp-run-1", 1, resultPayload)

	events, _ := store.EventsForRun("mcp-run-1")
	if len(events) != 2 {
		t.Fatalf("expected 2 MCP events, got: %d", len(events))
	}
}

func TestMCPToolUnregisteredReturnsError(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	_, err := reg.Invoke("not-registered", []byte(`{}`))
	if err == nil {
		t.Fatal("Invoke of unregistered tool must return error")
	}
}
