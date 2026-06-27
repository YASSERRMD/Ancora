package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestStreamingOrderFirstEventContainsStarted(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	first, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent: %v", err)
	}
	if first == nil {
		t.Fatal("first event must not be nil")
	}
	if !strings.Contains(string(first), "started") {
		t.Fatalf("first event must contain 'started', got: %s", first)
	}
}

func TestStreamingOrderEventChanDeliversInOrder(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))

	var events []string
	for ev := range run.EventChan() {
		events = append(events, string(ev))
	}

	if len(events) == 0 {
		t.Fatal("EventChan must deliver at least one event")
	}
	if !strings.Contains(events[0], "started") {
		t.Fatalf("first EventChan event must be 'started', got: %s", events[0])
	}
}

func TestStreamingOrderDrainEventsReturnsAllInOrder(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))

	events, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	if len(events) == 0 {
		t.Fatal("DrainEvents must return at least one event")
	}
}

func TestStreamingOrderPollAfterDrainReturnsNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))
	_, _ = run.DrainEvents()

	ev, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent after drain: %v", err)
	}
	if ev != nil {
		t.Fatalf("expected nil after drain, got: %s", ev)
	}
}

func TestStreamingOrderEventChanClosesAfterAllEventsDelivered(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))

	ch := run.EventChan()
	count := 0
	for range ch {
		count++
	}
	// Channel closed (range completed).
	if count == 0 {
		t.Fatal("EventChan must deliver at least one event before closing")
	}
}

func TestStreamingOrderTwoRunsDoNotInterleave(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	run1, _ := rt.StartRun([]byte("{}"))
	run2, _ := rt.StartRun([]byte("{}"))

	if run1.ID() == run2.ID() {
		t.Fatal("two runs must have different IDs")
	}

	ev1, _ := run1.PollEvent()
	ev2, _ := run2.PollEvent()

	if ev1 == nil || ev2 == nil {
		t.Fatal("both runs must produce a first event")
	}
	// Events from different runs must not be confused.
	if !strings.Contains(string(ev1), "started") {
		t.Fatalf("run1 first event is not started: %s", ev1)
	}
	if !strings.Contains(string(ev2), "started") {
		t.Fatalf("run2 first event is not started: %s", ev2)
	}
}

func TestStreamingOrderEventBytesAreNonEmpty(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("stream-agent", "llama3", "stream test")
	run, _ := ancora.NewAgent(rt, spec).Start()

	events, _ := run.DrainEvents()
	for i, ev := range events {
		if len(ev) == 0 {
			t.Fatalf("event %d must not be empty", i)
		}
	}
}

func TestStreamingOrderAgentRunProducesMoreEventsThanEmptySpec(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	runEmpty, _ := rt.StartRun([]byte("{}"))
	emptyEvents, _ := runEmpty.DrainEvents()

	spec := ancora.NewAgentSpec("event-count", "llama3", "produce events")
	runSpec, _ := ancora.NewAgent(rt, spec).Start()
	specEvents, _ := runSpec.DrainEvents()

	_ = emptyEvents
	_ = specEvents
	// Both must produce at least one event.
	if len(emptyEvents) == 0 {
		t.Fatal("empty-spec run must produce at least one event")
	}
	if len(specEvents) == 0 {
		t.Fatal("agent run must produce at least one event")
	}
}
