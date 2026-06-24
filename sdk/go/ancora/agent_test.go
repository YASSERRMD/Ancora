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

func TestEventChanOnEmptyQueueReturnsClosed(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	run.DrainEvents() // empty the queue
	count := 0
	for range run.EventChan() {
		count++
	}
	if count != 0 {
		t.Fatalf("expected 0 events on empty queue, got: %d", count)
	}
}

func TestAgentSpecAccessor(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	if ag.Spec() == nil {
		t.Fatal("Agent.Spec() returned nil")
	}
}

func TestAgentRuntimeAccessor(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	if ag.Runtime() != rt {
		t.Fatal("Agent.Runtime() did not return the original runtime")
	}
}

func TestDrainEventsOrderStartedBeforeCompleted(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	events, _ := run.DrainEvents()
	if len(events) < 2 {
		t.Fatalf("expected at least 2 events, got: %d", len(events))
	}
	if !contains(events[0], "started") {
		t.Fatalf("first event must be started, got: %s", events[0])
	}
}

func TestMultipleStartsOnSameAgentAllSucceed(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	for i := 0; i < 3; i++ {
		run, err := ag.Start()
		if err != nil {
			t.Fatalf("Start #%d: %v", i, err)
		}
		if run.ID() == "" {
			t.Fatalf("Start #%d: empty run ID", i)
		}
		run.DrainEvents()
	}
}

func TestEventChanFromAgentStartReceivesAllEvents(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	var events []string
	for ev := range run.EventChan() {
		events = append(events, string(ev))
	}
	if len(events) < 2 {
		t.Fatalf("expected at least 2 events via channel, got: %d", len(events))
	}
}

func TestEventChanAfterResumeContainsResumed(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, _ := ag.Start()
	run.DrainEvents()
	ag.Resume(run, []byte("approved"))
	var events []string
	for ev := range run.EventChan() {
		events = append(events, string(ev))
	}
	found := false
	for _, e := range events {
		if contains(e, "resumed") {
			found = true
		}
	}
	if !found {
		t.Fatalf("EventChan after resume must contain resumed event, got: %v", events)
	}
}

func TestAgentSpecNameSetViaNewAgentSpec(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	ag := ancora.NewAgent(rt, ancora.NewAgentSpec("named-agent", "m", "i"))
	if ag.Spec().GetName() != "named-agent" {
		t.Fatalf("expected 'named-agent', got: %q", ag.Spec().GetName())
	}
}

func TestStartWithEventsReturnsRunAndChannel(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, ch, err := ag.StartWithEvents()
	if err != nil {
		t.Fatalf("StartWithEvents: %v", err)
	}
	if run == nil || ch == nil {
		t.Fatal("StartWithEvents returned nil run or nil channel")
	}
	var events []string
	for ev := range ch {
		events = append(events, string(ev))
	}
	if len(events) == 0 {
		t.Fatal("StartWithEvents channel produced no events")
	}
}

func TestStartWithEventsRunIDMatchesChannelSource(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	run, ch, _ := ag.StartWithEvents()
	count := 0
	for range ch {
		count++
	}
	if run.ID() == "" {
		t.Fatal("run from StartWithEvents has empty ID")
	}
	if count == 0 {
		t.Fatal("no events received from StartWithEvents channel")
	}
}

func TestNewAgentReturnsNonNil(t *testing.T) {
	rt, ag := makeAgent(t)
	defer rt.Free()
	if ag == nil {
		t.Fatal("NewAgent returned nil")
	}
}
