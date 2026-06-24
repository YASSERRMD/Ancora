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

func TestNewAgentReturnsNonNil(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	if ag == nil {
		t.Fatal("NewAgent returned nil")
	}
}
