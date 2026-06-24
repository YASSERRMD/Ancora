package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

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
