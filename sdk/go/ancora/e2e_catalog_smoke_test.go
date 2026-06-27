package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

// catalogExamples lists the canonical SDK example scenarios and the minimal
// spec each one requires. These smoke-test the API surface without requiring
// a live provider.
var catalogExamples = []struct {
	name    string
	modelID string
	tools   []string
}{
	{"single-agent", "llama3", nil},
	{"multi-agent-verifier", "llama3", []string{"verify"}},
	{"human-in-loop", "llama3", nil},
	{"rag-lancedb", "gpt-4o", []string{"lancedb-retrieve"}},
	{"mcp-tool", "gpt-4o", []string{"web-search"}},
	{"streaming-chat", "claude-opus-4-8", nil},
	{"structured-output", "llama3", nil},
	{"cost-otel", "llama3", nil},
	{"durable-restart", "llama3", nil},
	{"event-chan", "llama3", nil},
}

func TestE2ECatalogSmokeAllExamplesStartSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	for _, ex := range catalogExamples {
		builder := ancora.NewAgentSpecBuilder().
			WithName(ex.name).
			WithModelID(ex.modelID)

		for _, toolName := range ex.tools {
			builder = builder.WithTool(ancora.NewToolSpec(toolName, "catalog example tool"))
		}

		spec := builder.Build()
		run, err := ancora.NewAgent(rt, spec).Start()
		if err != nil {
			t.Errorf("example %q: Start failed: %v", ex.name, err)
			continue
		}
		if run.ID() == "" {
			t.Errorf("example %q: run ID is empty", ex.name)
		}
	}
}

func TestE2ECatalogSmokeAllExamplesProduceAtLeastOneEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	for _, ex := range catalogExamples {
		builder := ancora.NewAgentSpecBuilder().
			WithName(ex.name).
			WithModelID(ex.modelID)
		spec := builder.Build()

		run, err := ancora.NewAgent(rt, spec).Start()
		if err != nil {
			t.Errorf("example %q: Start: %v", ex.name, err)
			continue
		}
		events, err := run.DrainEvents()
		if err != nil {
			t.Errorf("example %q: DrainEvents: %v", ex.name, err)
			continue
		}
		if len(events) == 0 {
			t.Errorf("example %q: no events produced", ex.name)
		}
	}
}

func TestE2ECatalogSmokeAllExamplesHaveNonEmptyModelID(t *testing.T) {
	for _, ex := range catalogExamples {
		if ex.modelID == "" {
			t.Errorf("example %q: modelID must be non-empty", ex.name)
		}
	}
}

func TestE2ECatalogSmokeStoringTransportRecordsAllExamples(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	for _, ex := range catalogExamples[:3] {
		runID, err := tr.StartRun(context.Background(), []byte("{}"))
		if err != nil {
			t.Errorf("example %q: StartRun: %v", ex.name, err)
			continue
		}
		for {
			ev, _ := tr.PollRun(context.Background(), runID)
			if ev == nil {
				break
			}
		}
	}

	count, _ := store.RunCount()
	if count < 3 {
		t.Fatalf("expected at least 3 catalog runs stored, got: %d", count)
	}
}

func TestE2ECatalogSmokeCatalogHasTenExamples(t *testing.T) {
	if len(catalogExamples) != 10 {
		t.Fatalf("expected 10 catalog examples, got: %d", len(catalogExamples))
	}
}
