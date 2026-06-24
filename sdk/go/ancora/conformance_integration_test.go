package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

// TestConformanceFullRoundTripSingleAgent exercises the entire stack:
// Runtime -> CgoTransport -> StoringTransport -> TransportAgent -> SqliteStore.
func TestConformanceFullRoundTripSingleAgent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	spec := ancora.NewAgentSpec("full-rt", "mock", "")
	ag := ancora.NewTransportAgent(tr, spec)

	run, err := ag.Start(context.Background())
	if err != nil {
		t.Fatalf("Start: %v", err)
	}

	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}

	kinds := journalKinds(evs)
	if !equalSlices(kinds, []string{"started", "completed"}) {
		t.Fatalf("full round-trip journal mismatch: %v", kinds)
	}

	stored, err := store.EventsForRun(run.ID())
	if err != nil {
		t.Fatalf("EventsForRun: %v", err)
	}
	storedKinds := journalKinds(stored)
	if !equalSlices(storedKinds, kinds) {
		t.Fatalf("stored events differ from returned: stored=%v returned=%v", storedKinds, kinds)
	}
}

// TestConformanceFullRoundTripHumanInLoop exercises the HITL path end-to-end.
func TestConformanceFullRoundTripHumanInLoop(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	spec := ancora.NewAgentSpec("full-hil", "mock", "")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())

	pre, _ := run.DrainEvents(context.Background())
	run.Resume(context.Background(), []byte("approved"))
	post, _ := run.DrainEvents(context.Background())

	all := append(pre, post...)
	kinds := journalKinds(all)

	hasStarted, hasResumed, hasCompleted := false, false, false
	for _, k := range kinds {
		switch k {
		case "started":
			hasStarted = true
		case "resumed":
			hasResumed = true
		case "completed":
			hasCompleted = true
		}
	}
	if !hasStarted || !hasResumed || !hasCompleted {
		t.Fatalf("full HITL round-trip: missing events; kinds=%v", kinds)
	}
}

func TestConformanceSuiteGreenWithCgoTransport(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	results := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt)).RunAll(context.Background())
	var failed int
	for _, r := range results {
		if !r.Passed {
			t.Errorf("FAIL %q: %s", r.ScenarioID, r.Reason)
			failed++
		}
	}
	if failed > 0 {
		t.Fatalf("%d conformance scenario(s) failed", failed)
	}
}
