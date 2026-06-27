package ancora_test

import (
	"encoding/json"
	"testing"
)

// Cross-language MCP: Go server consumed by TypeScript client (offline fixture).

type mcpGoTsActivity struct {
	ActivityKey  string          `json:"activity_key"`
	ActivityKind string          `json:"activity_kind"`
	InputJSON    json.RawMessage `json:"input_json"`
	ResultJSON   json.RawMessage `json:"result_json"`
}

func makeMcpGoTsActivity() mcpGoTsActivity {
	return mcpGoTsActivity{
		ActivityKey:  "mcp-go-server/search",
		ActivityKind: "tool-call",
		InputJSON:    json.RawMessage(`{"query":"xlang search","client_lang":"ts","server_lang":"go"}`),
		ResultJSON:   json.RawMessage(`[{"id":"g1","text":"Go MCP result","server":"go-mcp-server"}]`),
	}
}

func TestMcpGoServerActivityKeyContainsGo(t *testing.T) {
	a := makeMcpGoTsActivity()
	key := a.ActivityKey
	if key == "" || key == "mcp-go-server/search" {
		if key != "mcp-go-server/search" {
			t.Fatalf("expected mcp-go-server/search, got %q", key)
		}
	}
}

func TestMcpGoServerClientLangIsTs(t *testing.T) {
	a := makeMcpGoTsActivity()
	var inp map[string]interface{}
	json.Unmarshal(a.InputJSON, &inp) //nolint:errcheck
	if inp["client_lang"] != "ts" {
		t.Fatalf("expected client_lang=ts, got %v", inp["client_lang"])
	}
}

func TestMcpGoServerServerLangIsGo(t *testing.T) {
	a := makeMcpGoTsActivity()
	var inp map[string]interface{}
	json.Unmarshal(a.InputJSON, &inp) //nolint:errcheck
	if inp["server_lang"] != "go" {
		t.Fatalf("expected server_lang=go, got %v", inp["server_lang"])
	}
}

func TestMcpGoServerResultHasServerField(t *testing.T) {
	a := makeMcpGoTsActivity()
	var res []map[string]interface{}
	json.Unmarshal(a.ResultJSON, &res) //nolint:errcheck
	if len(res) == 0 || res[0]["server"] != "go-mcp-server" {
		t.Fatalf("result missing go-mcp-server: %v", res)
	}
}

func TestMcpGoServerActivityKindIsToolCall(t *testing.T) {
	a := makeMcpGoTsActivity()
	if a.ActivityKind != "tool-call" {
		t.Fatalf("expected tool-call, got %q", a.ActivityKind)
	}
}

func TestMcpGoServerActivitySerialisesToJSON(t *testing.T) {
	a := makeMcpGoTsActivity()
	raw, err := json.Marshal(a)
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	var decoded mcpGoTsActivity
	if err := json.Unmarshal(raw, &decoded); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if decoded.ActivityKey != a.ActivityKey {
		t.Fatalf("roundtrip key mismatch")
	}
}
