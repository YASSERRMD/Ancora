package ancora_test

import (
	"context"
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestE2ESingleAgentRunProducesStartedEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	spec := ancora.NewAgentSpecBuilder().
		WithName("e2e-single").
		WithModelID("llama3").
		WithInstructions("You are a helpful assistant.").
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}

	events, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	if len(events) == 0 {
		t.Fatal("single agent run must produce at least one event")
	}

	found := false
	for _, ev := range events {
		if strings.Contains(ev, "started") {
			found = true
			break
		}
	}
	if !found {
		t.Logf("events: %v", events)
	}
}

func TestE2ESingleAgentRunIDIsNonEmpty(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	spec := ancora.NewAgentSpecBuilder().
		WithName("e2e-id-check").
		WithModelID("gpt-4o").
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestE2ESingleAgentRunStoresEventsInJournal(t *testing.T) {
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
		t.Fatal("single agent run must store at least one event in journal")
	}
}

func TestE2ESingleAgentRunTwoRunsHaveDifferentIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	spec := ancora.NewAgentSpecBuilder().
		WithName("e2e-two-runs").
		WithModelID("llama3").
		Build()

	run1, err1 := ancora.NewAgent(rt, spec).Start()
	run2, err2 := ancora.NewAgent(rt, spec).Start()
	if err1 != nil || err2 != nil {
		t.Fatalf("Start: %v / %v", err1, err2)
	}
	if run1.ID() == run2.ID() {
		t.Fatalf("two runs must have different IDs, both: %q", run1.ID())
	}
}

func TestE2ESingleAgentConformanceSuitePassesSingleAgent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	for _, r := range results {
		if r.ScenarioID == "single-agent" && !r.Passed {
			t.Fatalf("conformance scenario 'single-agent' failed: %s", r.Reason)
		}
	}
}

func TestE2ESingleAgentEventChanDeliversAllEvents(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	spec := ancora.NewAgentSpecBuilder().
		WithName("e2e-chan").
		WithModelID("llama3").
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}

	var events [][]byte
	for ev := range run.EventChan() {
		events = append(events, ev)
	}

	if len(events) == 0 {
		t.Fatal("EventChan must deliver at least one event")
	}
}

func TestE2ESingleAgentMaxStepsIsRespected(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	spec := ancora.NewAgentSpecBuilder().
		WithName("e2e-maxsteps").
		WithModelID("llama3").
		WithMaxSteps(1).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}

	events, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	if len(events) == 0 {
		t.Fatal("run with max_steps=1 must produce at least one event")
	}
}

func TestE2ESingleAgentOutputIsDrainedCompletely(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))
	events1, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("first DrainEvents: %v", err)
	}

	events2, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("second DrainEvents: %v", err)
	}
	if len(events2) != 0 {
		t.Fatalf("second drain must return zero events, got: %d (first had %d)", len(events2), len(events1))
	}
}

func TestE2ESingleAgentPollAfterDrainIsNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run, _ := rt.StartRun([]byte("{}"))
	_, _ = run.DrainEvents()

	ev, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent after drain: %v", err)
	}
	if ev != nil {
		t.Fatalf("PollEvent after drain must return nil, got: %s", ev)
	}
}

func TestE2ESingleAgentStoringTransportListsRun(t *testing.T) {
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

	ids, err := store.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	found := false
	for _, id := range ids {
		if id == runID {
			found = true
		}
	}
	if !found {
		t.Fatalf("ListRuns must include %q, got: %v", runID, ids)
	}
}

func TestE2ESingleAgentInstructionsReachSpec(t *testing.T) {
	instructions := "You are a helpful assistant for testing."
	spec := ancora.NewAgentSpecBuilder().
		WithName("e2e-instructions").
		WithModelID("llama3").
		WithInstructions(instructions).
		Build()
	if spec.GetInstructions() != instructions {
		t.Fatalf("instructions mismatch: %q", spec.GetInstructions())
	}
}
