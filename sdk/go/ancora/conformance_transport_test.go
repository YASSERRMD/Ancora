package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestCgoTransportConformanceSingleAgentProducesStartedEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-conf", "mock", "")
	ag := ancora.NewTransportAgent(tr, spec)
	run, _ := ag.Start(context.Background())
	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	for _, ev := range evs {
		if eventKind(ev) == "started" {
			return
		}
	}
	t.Fatalf("cgo transport single-agent: missing 'started' event, got: %v", evs)
}

func TestCgoTransportConformanceSingleAgentProducesCompletedEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-done", "mock", "")
	ag := ancora.NewTransportAgent(tr, spec)
	run, _ := ag.Start(context.Background())
	evs, err := run.DrainEvents(context.Background())
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	for _, ev := range evs {
		if eventKind(ev) == "completed" {
			return
		}
	}
	t.Fatalf("cgo transport single-agent: missing 'completed' event, got: %v", evs)
}

func TestCgoTransportConformanceSingleAgentJournalMatchesFixture(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-j", "mock", "")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	evs, _ := run.DrainEvents(context.Background())
	kinds := journalKinds(evs)
	expected := []string{"started", "completed"}
	if !equalSlices(kinds, expected) {
		t.Fatalf("cgo transport journal mismatch: got %v, want %v", kinds, expected)
	}
}

func TestCgoTransportConformanceTwoRunsHaveUniqueIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-uniq", "mock", "")
	ag := ancora.NewTransportAgent(tr, spec)
	r1, _ := ag.Start(context.Background())
	r2, _ := ag.Start(context.Background())
	if r1.ID() == r2.ID() {
		t.Fatalf("cgo transport: run IDs must be unique, both=%q", r1.ID())
	}
}

func TestCgoTransportConformanceHumanInLoopProducesResumedEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	spec := ancora.NewAgentSpec("cgo-hil", "mock", "")
	run, _ := ancora.NewTransportAgent(tr, spec).Start(context.Background())
	run.DrainEvents(context.Background())
	run.Resume(context.Background(), []byte("approved"))
	postEvs, err := run.DrainEvents(context.Background())
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	for _, ev := range postEvs {
		if eventKind(ev) == "resumed" {
			return
		}
	}
	t.Fatalf("cgo transport human-in-loop: missing 'resumed' event, got: %v", postEvs)
}

func TestCgoTransportConformanceCrashRecoverDeterministic(t *testing.T) {
	var kinds1, kinds2 []string

	rt1 := mustRuntime(t)
	tr1 := ancora.NewCgoTransport(rt1)
	spec := ancora.NewAgentSpec("cr-tr1", "mock", "")
	r1, _ := ancora.NewTransportAgent(tr1, spec).Start(context.Background())
	evs1, _ := r1.DrainEvents(context.Background())
	rt1.Free()
	kinds1 = journalKinds(evs1)

	rt2 := mustRuntime(t)
	tr2 := ancora.NewCgoTransport(rt2)
	spec2 := ancora.NewAgentSpec("cr-tr2", "mock", "")
	r2, _ := ancora.NewTransportAgent(tr2, spec2).Start(context.Background())
	evs2, _ := r2.DrainEvents(context.Background())
	rt2.Free()
	kinds2 = journalKinds(evs2)

	if !equalSlices(kinds1, kinds2) {
		t.Fatalf("cgo transport crash-recover journal diverged: %v vs %v", kinds1, kinds2)
	}
}
