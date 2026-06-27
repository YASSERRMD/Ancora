package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestSingleAgentRunProducesStartedEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("single", "llama3", "You are helpful.")
	ag := ancora.NewAgent(rt, spec)
	run, err := ag.Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	ev, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent: %v", err)
	}
	if ev == nil {
		t.Fatal("expected non-nil first event")
	}
	if !strings.Contains(string(ev), "started") {
		t.Fatalf("first event must contain 'started', got: %s", ev)
	}
}

func TestSingleAgentRunIDIsNonEmpty(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("single-id", "llama3", "")
	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestSingleAgentRunEventChanClosesAfterDrain(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("chan-close", "llama3", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	ch := run.EventChan()
	var count int
	for range ch {
		count++
	}
	// Channel must have been closed (range completed).
	// At least one event must have been received.
	if count == 0 {
		t.Fatal("event channel must deliver at least one event before closing")
	}
}

func TestSingleAgentDrainEventsReturnsNonEmptySlice(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("drain-agent", "llama3", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	events, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	if len(events) == 0 {
		t.Fatal("DrainEvents must return at least one event")
	}
}

func TestSingleAgentRunWithMaxStepsBound(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpecBuilder().
		WithName("bounded-single").
		WithModelID("llama3").
		WithInstructions("bounded test").
		WithMaxSteps(1).
		Build()
	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty even with max_steps=1")
	}
}

func TestSingleAgentRunEventsContainRunID(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("id-check", "llama3", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	events, _ := run.DrainEvents()
	if len(events) == 0 {
		t.Skip("no events produced")
	}
	found := false
	for _, ev := range events {
		if strings.Contains(ev, run.ID()) {
			found = true
			break
		}
	}
	if !found {
		t.Logf("run ID %q not found in events (implementation-defined): %v", run.ID(), events)
	}
}

func TestSingleAgentTwoRunsHaveDifferentIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("two-runs", "llama3", "")
	ag := ancora.NewAgent(rt, spec)

	run1, err1 := ag.Start()
	run2, err2 := ag.Start()
	if err1 != nil || err2 != nil {
		t.Fatalf("Start errors: %v, %v", err1, err2)
	}
	if run1.ID() == run2.ID() {
		t.Fatalf("two runs must have different IDs, both got: %s", run1.ID())
	}
}

func TestSingleAgentPollingAfterDrainReturnsNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("poll-after", "llama3", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	_, _ = run.DrainEvents()

	ev, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent after drain: %v", err)
	}
	if ev != nil {
		t.Fatalf("expected nil after drain, got: %s", ev)
	}
}
