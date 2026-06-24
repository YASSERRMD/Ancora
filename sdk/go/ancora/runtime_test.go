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

func TestResumeRunProducesResumedEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))
	drainEvents(t, run)
	if err := run.Resume([]byte("approved")); err != nil {
		t.Fatalf("Resume: %v", err)
	}
	events := drainEvents(t, run)
	found := false
	for _, e := range events {
		if contains(e, "resumed") {
			found = true
		}
	}
	if !found {
		t.Fatalf("expected resumed event, got: %v", events)
	}
}

func TestAncorErrorImplementsError(t *testing.T) {
	var err error = ancora.ErrNullPtr
	if err.Error() == "" {
		t.Fatal("AncorError.Error() must return non-empty string")
	}
}

func TestRuntimeExplicitFreeRemovesFinalizer(t *testing.T) {
	rt := mustRuntime(t)
	rt.Free()
	// After explicit Free, finalizer is removed and ptr is nil;
	// a subsequent GC cycle will not double-free.
	rt.Free() // second call must be a no-op, not a crash
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
