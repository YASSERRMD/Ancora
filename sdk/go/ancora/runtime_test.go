package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func contains(s, sub string) bool { return strings.Contains(s, sub) }

func mustRuntime(t *testing.T) *ancora.Runtime {
	t.Helper()
	rt, err := ancora.NewRuntime()
	if err != nil {
		t.Fatalf("NewRuntime: %v", err)
	}
	return rt
}

func TestNewRuntimeReturnsNonNil(t *testing.T) {
	rt := mustRuntime(t)
	if rt == nil {
		t.Fatal("NewRuntime returned nil")
	}
	rt.Free()
}

func TestFreeRuntimeIsIdempotent(t *testing.T) {
	rt := mustRuntime(t)
	rt.Free()
	rt.Free()
}

func TestStartRunReturnsNonEmptyID(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("StartRun returned empty run ID")
	}
}

func TestPollEventReturnsStartedEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	ev, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent: %v", err)
	}
	if ev == nil {
		t.Fatal("expected first event to be non-nil")
	}
	if !contains(string(ev), "started") {
		t.Fatalf("expected started event, got: %s", ev)
	}
}

func TestPollReturnsNilWhenQueueDrained(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))
	drainEvents(t, run)
	ev, err := run.PollEvent()
	if err != nil {
		t.Fatalf("PollEvent after drain: %v", err)
	}
	if ev != nil {
		t.Fatalf("expected nil after drain, got: %s", ev)
	}
}

func drainEvents(t *testing.T, run *ancora.Run) []string {
	t.Helper()
	var events []string
	for {
		ev, err := run.PollEvent()
		if err != nil {
			t.Fatalf("PollEvent: %v", err)
		}
		if ev == nil {
			break
		}
		events = append(events, string(ev))
	}
	return events
}
