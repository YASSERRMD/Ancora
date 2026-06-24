package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func makeAgent(t *testing.T) (*ancora.Runtime, *ancora.Agent) {
	t.Helper()
	rt := mustRuntime(t)
	spec := ancora.NewAgentSpec("test", "llama3", "You are a test agent.")
	return rt, ancora.NewAgent(rt, spec)
}

func TestAgentStartReturnsNonNilRun(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, err := ag.Start()
	if err != nil {
		t.Fatalf("Agent.Start: %v", err)
	}
	if run == nil {
		t.Fatal("Agent.Start returned nil Run")
	}
}

func TestAgentStartRunHasNonEmptyID(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	if run.ID() == "" {
		t.Fatal("Agent.Start returned Run with empty ID")
	}
}

func TestEventChanReceivesAtLeastOneEvent(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	ch := run.EventChan()
	ev, ok := <-ch
	if !ok || ev == nil {
		t.Fatal("EventChan should deliver at least one event")
	}
}

func TestDrainEventsReturnsStartedAndCompleted(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	events, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	var hasStarted, hasCompleted bool
	for _, e := range events {
		if contains(e, "started") {
			hasStarted = true
		}
		if contains(e, "completed") {
			hasCompleted = true
		}
	}
	if !hasStarted {
		t.Fatalf("missing started event, got: %v", events)
	}
	if !hasCompleted {
		t.Fatalf("missing completed event, got: %v", events)
	}
}

func TestSingleAgentRunCompletesEndToEnd(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, err := ag.Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	var events []string
	for ev := range run.EventChan() {
		events = append(events, string(ev))
	}
	if len(events) == 0 {
		t.Fatal("run produced no events")
	}
	last := events[len(events)-1]
	if !contains(last, "completed") {
		t.Fatalf("last event must be completed, got: %s", last)
	}
}

func TestEventChanClosesAfterEventsExhausted(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	// range over the channel; it must close eventually
	count := 0
	for range run.EventChan() {
		count++
	}
	if count == 0 {
		t.Fatal("expected at least one event before channel close")
	}
}

func TestResumePropagatesDecisionViaAgent(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	run.DrainEvents()
	if err := ag.Resume(run, []byte("approved")); err != nil {
		t.Fatalf("Agent.Resume: %v", err)
	}
	events, _ := run.DrainEvents()
	found := false
	for _, e := range events {
		if contains(e, "resumed") {
			found = true
		}
	}
	if !found {
		t.Fatalf("expected resumed event after Resume, got: %v", events)
	}
}

func TestTwoConcurrentAgentsHaveDifferentRunIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("a", "m", "i")
	ag := ancora.NewAgent(rt, spec)
	run1, _ := ag.Start()
	run2, _ := ag.Start()
	if run1.ID() == run2.ID() {
		t.Fatalf("expected distinct run IDs, both were: %s", run1.ID())
	}
}

func TestAgentWithBuilderWorksEndToEnd(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpecBuilder().
		WithName("builder-agent").
		WithModelID("gpt-4o").
		WithInstructions("respond briefly").
		WithMaxSteps(3).
		Build()
	ag := ancora.NewAgent(rt, spec)
	run, err := ag.Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	events, _ := run.DrainEvents()
	if len(events) == 0 {
		t.Fatal("builder agent run produced no events")
	}
}

func TestDrainEventsAfterResumeIncludesCompleted(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	run.DrainEvents()
	ag.Resume(run, []byte("ok"))
	events, _ := run.DrainEvents()
	var hasCompleted bool
	for _, e := range events {
		if contains(e, "completed") {
			hasCompleted = true
		}
	}
	if !hasCompleted {
		t.Fatalf("post-resume events missing completed: %v", events)
	}
}

func TestNewAgentReturnsNonNil(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	if ag == nil {
		t.Fatal("NewAgent returned nil")
	}
}
