package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceStoreRecordsEventsForAllScenarios(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	suite := ancora.NewConformanceSuite(tr)
	suite.RunAll(context.Background())

	ids, err := store.ListRuns()
	if err != nil {
		t.Fatalf("ListRuns: %v", err)
	}
	if len(ids) != 4 {
		t.Fatalf("expected 4 stored run IDs, got: %d", len(ids))
	}
	for _, id := range ids {
		n, err := store.EventCount(id)
		if err != nil {
			t.Fatalf("EventCount(%q): %v", id, err)
		}
		if n == 0 {
			t.Fatalf("run %q must have at least one stored event", id)
		}
	}
}

func TestConformanceStoreSingleAgentEventsMatchFixture(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)

	spec := ancora.NewAgentSpec("store-conf", "mock", "")
	ag := ancora.NewTransportAgent(tr, spec)
	run, _ := ag.Start(context.Background())
	run.DrainEvents(context.Background())

	evs, err := store.EventsForRun(run.ID())
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	kinds := journalKinds(evs)
	expected := []string{"started", "completed"}
	if !equalSlices(kinds, expected) {
		t.Fatalf("stored journal mismatch: got %v, want %v", kinds, expected)
	}
}

func TestConformanceStoreHumanInLoopPostResumeStoredEvents(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)

	spec := ancora.NewAgentSpec("hil-store", "mock", "")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	run.DrainEvents(context.Background())
	run.Resume(context.Background(), []byte("yes"))
	run.DrainEvents(context.Background())

	evs, _ := store.EventsForRun(run.ID())
	if len(evs) < 2 {
		t.Fatalf("expected at least 2 stored events after resume, got: %d", len(evs))
	}
}
