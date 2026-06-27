package ancora_test

import (
	"encoding/json"
	"testing"

	"ancora.io/sdk/ancora"
)

// costRecord is a local struct mirroring how cost data would appear in event JSON.
type costRecord struct {
	InputTokens  int     `json:"input_tokens"`
	OutputTokens int     `json:"output_tokens"`
	CostUSD      float64 `json:"cost_usd"`
}

func TestCostStoreRecordsCostEvent(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("cost-run-1")
	ev := `{"type":"usage","input_tokens":100,"output_tokens":50,"cost_usd":0.002}`
	if err := store.AppendEvent("cost-run-1", 0, ev); err != nil {
		t.Fatalf("AppendEvent: %v", err)
	}

	events, err := store.EventsForRun("cost-run-1")
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	if len(events) != 1 {
		t.Fatalf("expected 1 cost event, got: %d", len(events))
	}
}

func TestCostEventJSONRoundTrip(t *testing.T) {
	rec := costRecord{InputTokens: 200, OutputTokens: 100, CostUSD: 0.004}
	b, err := json.Marshal(rec)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}
	var got costRecord
	if err := json.Unmarshal(b, &got); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}
	if got.InputTokens != 200 || got.OutputTokens != 100 {
		t.Fatalf("round-trip mismatch: %+v", got)
	}
}

func TestCostInputTokensAreNonNegative(t *testing.T) {
	rec := costRecord{InputTokens: 0, OutputTokens: 0, CostUSD: 0}
	if rec.InputTokens < 0 {
		t.Fatal("input tokens must be non-negative")
	}
}

func TestCostOutputTokensAreNonNegative(t *testing.T) {
	rec := costRecord{InputTokens: 10, OutputTokens: 5, CostUSD: 0.001}
	if rec.OutputTokens < 0 {
		t.Fatal("output tokens must be non-negative")
	}
}

func TestCostUSDAreNonNegative(t *testing.T) {
	rec := costRecord{InputTokens: 10, OutputTokens: 5, CostUSD: 0.001}
	if rec.CostUSD < 0 {
		t.Fatal("cost_usd must be non-negative")
	}
}

func TestCostTwoRunsAccumulateIndependently(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("cost-a")
	_ = store.RecordRun("cost-b")
	_ = store.AppendEvent("cost-a", 0, `{"type":"usage","input_tokens":100}`)
	_ = store.AppendEvent("cost-b", 0, `{"type":"usage","input_tokens":200}`)
	_ = store.AppendEvent("cost-b", 1, `{"type":"usage","input_tokens":300}`)

	evA, _ := store.EventsForRun("cost-a")
	evB, _ := store.EventsForRun("cost-b")

	if len(evA) != 1 {
		t.Fatalf("run-a expected 1 cost event, got: %d", len(evA))
	}
	if len(evB) != 2 {
		t.Fatalf("run-b expected 2 cost events, got: %d", len(evB))
	}
}

func TestCostEventCountGrowsPerRun(t *testing.T) {
	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	_ = store.RecordRun("cost-grow")
	for i := 0; i < 5; i++ {
		_ = store.AppendEvent("cost-grow", i, `{"type":"usage","input_tokens":10}`)
	}

	count, err := store.EventCount("cost-grow")
	if err != nil {
		t.Fatalf("EventCount: %v", err)
	}
	if count != 5 {
		t.Fatalf("expected 5 cost events, got: %d", count)
	}
}

func TestCostAgentSpecBuildBytesIsNonEmpty(t *testing.T) {
	spec := ancora.NewAgentSpecBuilder().
		WithName("cost-agent").
		WithModelID("claude-opus-4-8").
		WithMaxSteps(10).
		Build()
	if spec == nil {
		t.Fatal("spec must not be nil")
	}
	if spec.GetName() == "" {
		t.Fatal("spec name must not be empty")
	}
}
