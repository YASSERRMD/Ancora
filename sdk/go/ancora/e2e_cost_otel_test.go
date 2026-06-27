package ancora_test

import (
	"encoding/json"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// otelSpan is a minimal representation of an OpenTelemetry span in event form.
type otelSpan struct {
	TraceID    string `json:"trace_id"`
	SpanID     string `json:"span_id"`
	Name       string `json:"name"`
	DurationNs int64  `json:"duration_ns"`
}

// costOtelEvent is a combined cost+otel event payload.
type costOtelEvent struct {
	Type         string   `json:"type"`
	InputTokens  int      `json:"input_tokens"`
	OutputTokens int      `json:"output_tokens"`
	CostUSD      float64  `json:"cost_usd"`
	Span         otelSpan `json:"span"`
}

func fixtureCostOtelTool(input []byte) ([]byte, error) {
	ev := costOtelEvent{
		Type:         "usage_with_otel",
		InputTokens:  150,
		OutputTokens: 80,
		CostUSD:      0.003,
		Span: otelSpan{
			TraceID:    "aabbccdd-0011-2233-4455-66778899aabb",
			SpanID:     "1122334455667788",
			Name:       "agent-run",
			DurationNs: 42000000,
		},
	}
	return json.Marshal(ev)
}

func TestE2ECostOtelEventRoundTrip(t *testing.T) {
	out, err := fixtureCostOtelTool(nil)
	if err != nil {
		t.Fatalf("fixtureCostOtelTool: %v", err)
	}
	var ev costOtelEvent
	if err := json.Unmarshal(out, &ev); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if ev.InputTokens != 150 {
		t.Fatalf("input_tokens mismatch: %d", ev.InputTokens)
	}
	if ev.Span.Name != "agent-run" {
		t.Fatalf("span.name mismatch: %q", ev.Span.Name)
	}
}

func TestE2ECostOtelEventStoredInJournal(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	out, _ := fixtureCostOtelTool(nil)
	_ = store.RecordRun("cost-otel-run")
	_ = store.AppendEvent("cost-otel-run", 0, string(out))

	events, _ := store.EventsForRun("cost-otel-run")
	if len(events) != 1 {
		t.Fatalf("expected 1 cost+otel event, got: %d", len(events))
	}
}

func TestE2ECostOtelEventContainsTraceID(t *testing.T) {
	out, _ := fixtureCostOtelTool(nil)
	if !strings.Contains(string(out), "trace_id") {
		t.Fatalf("cost+otel event must contain 'trace_id', got: %s", out)
	}
}

func TestE2ECostOtelEventContainsSpanID(t *testing.T) {
	out, _ := fixtureCostOtelTool(nil)
	if !strings.Contains(string(out), "span_id") {
		t.Fatalf("cost+otel event must contain 'span_id', got: %s", out)
	}
}

func TestE2ECostOtelInputTokensAreNonNegative(t *testing.T) {
	out, _ := fixtureCostOtelTool(nil)
	var ev costOtelEvent
	_ = json.Unmarshal(out, &ev)
	if ev.InputTokens < 0 {
		t.Fatalf("input_tokens must be non-negative, got: %d", ev.InputTokens)
	}
}

func TestE2ECostOtelOutputTokensAreNonNegative(t *testing.T) {
	out, _ := fixtureCostOtelTool(nil)
	var ev costOtelEvent
	_ = json.Unmarshal(out, &ev)
	if ev.OutputTokens < 0 {
		t.Fatalf("output_tokens must be non-negative, got: %d", ev.OutputTokens)
	}
}

func TestE2ECostOtelCostUSDIsNonNegative(t *testing.T) {
	out, _ := fixtureCostOtelTool(nil)
	var ev costOtelEvent
	_ = json.Unmarshal(out, &ev)
	if ev.CostUSD < 0 {
		t.Fatalf("cost_usd must be non-negative, got: %f", ev.CostUSD)
	}
}

func TestE2ECostOtelSpanDurationNsIsNonNegative(t *testing.T) {
	out, _ := fixtureCostOtelTool(nil)
	var ev costOtelEvent
	_ = json.Unmarshal(out, &ev)
	if ev.Span.DurationNs < 0 {
		t.Fatalf("span.duration_ns must be non-negative, got: %d", ev.Span.DurationNs)
	}
}

func TestE2ECostOtelToolRegisteredAndInvokable(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("cost-otel-emitter", fixtureCostOtelTool)

	out, err := reg.Invoke("cost-otel-emitter", []byte(`{}`))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	var ev costOtelEvent
	if err := json.Unmarshal(out, &ev); err != nil {
		t.Fatalf("Unmarshal from registry: %v", err)
	}
	if ev.Span.TraceID == "" {
		t.Fatal("trace_id must be non-empty")
	}
}
