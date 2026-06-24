package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceJournalAllScenariosStoredEventCountPositive(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	suite := ancora.NewConformanceSuite(tr)
	suite.RunAll(context.Background())

	ids, _ := store.ListRuns()
	for _, id := range ids {
		n, err := store.EventCount(id)
		if err != nil {
			t.Fatalf("EventCount(%q): %v", id, err)
		}
		if n < 1 {
			t.Fatalf("stored run %q must have at least one event, got: %d", id, n)
		}
	}
}

func TestConformanceJournalSingleAgentStoredKindsMatchFixture(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	spec := ancora.NewAgentSpec("jstore-agent", "mock", "")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	run.DrainEvents(context.Background())

	evs, _ := store.EventsForRun(run.ID())
	kinds := journalKinds(evs)
	expected := []string{"started", "completed"}
	if !equalSlices(kinds, expected) {
		t.Fatalf("stored journal mismatch: %v vs %v", kinds, expected)
	}
}

func TestConformanceJournalHumanInLoopStoredStartedBeforeResumed(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	spec := ancora.NewAgentSpec("jstore-hil", "mock", "")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	run.DrainEvents(context.Background())
	run.Resume(context.Background(), []byte("ok"))
	run.DrainEvents(context.Background())

	evs, _ := store.EventsForRun(run.ID())
	kinds := journalKinds(evs)
	if len(kinds) < 3 {
		t.Fatalf("expected at least 3 stored events, got: %d (%v)", len(kinds), kinds)
	}
	if kinds[0] != "started" {
		t.Fatalf("first stored event must be 'started', got: %q", kinds[0])
	}
	hasResumed := false
	for _, k := range kinds {
		if k == "resumed" {
			hasResumed = true
		}
	}
	if !hasResumed {
		t.Fatalf("stored events must contain 'resumed', got: %v", kinds)
	}
}

func TestConformanceJournalCrashRecoverStoredKindsAreDeterministic(t *testing.T) {
	var store1evs, store2evs []string

	rt1 := mustRuntime(t)
	s1 := mustInMemoryStore(t)
	tr1 := ancora.NewStoringTransport(ancora.NewCgoTransport(rt1), s1)
	spec1 := ancora.NewAgentSpec("cr-s1", "mock", "")
	r1, _ := ancora.NewTransportAgent(tr1, spec1).Start(context.Background())
	r1.DrainEvents(context.Background())
	store1evs, _ = s1.EventsForRun(r1.ID())
	rt1.Free()

	rt2 := mustRuntime(t)
	s2 := mustInMemoryStore(t)
	tr2 := ancora.NewStoringTransport(ancora.NewCgoTransport(rt2), s2)
	spec2 := ancora.NewAgentSpec("cr-s2", "mock", "")
	r2, _ := ancora.NewTransportAgent(tr2, spec2).Start(context.Background())
	r2.DrainEvents(context.Background())
	store2evs, _ = s2.EventsForRun(r2.ID())
	rt2.Free()

	k1, k2 := journalKinds(store1evs), journalKinds(store2evs)
	if !equalSlices(k1, k2) {
		t.Fatalf("stored journal diverged: %v vs %v", k1, k2)
	}
}
