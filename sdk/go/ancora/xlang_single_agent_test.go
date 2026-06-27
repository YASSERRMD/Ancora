package ancora_test

import (
	"encoding/json"
	"testing"
)

// Cross-language conformance: single agent scenario -- Go binding.
// Offline: local fake runtime with a fixed event queue.

type xlangFakeRuntime struct {
	runID  string
	events [][]byte
	pos    int
}

func (f *xlangFakeRuntime) poll() []byte {
	if f.pos >= len(f.events) {
		return nil
	}
	ev := f.events[f.pos]
	f.pos++
	return ev
}

func newXlangFakeRuntime(runID string) *xlangFakeRuntime {
	events := [][]byte{
		[]byte(`{"kind":"started","run_id":"` + runID + `","spec":"{}"}`),
		[]byte(`{"kind":"token","run_id":"` + runID + `","text":"xlang go result"}`),
		[]byte(`{"kind":"completed","run_id":"` + runID + `"}`),
	}
	return &xlangFakeRuntime{runID: runID, events: events}
}

type xlangGoEvent struct {
	Kind  string `json:"kind"`
	RunID string `json:"run_id"`
	Text  string `json:"text,omitempty"`
}

func drainXlang(fr *xlangFakeRuntime) []xlangGoEvent {
	var out []xlangGoEvent
	for {
		raw := fr.poll()
		if raw == nil {
			break
		}
		var ev xlangGoEvent
		json.Unmarshal(raw, &ev) //nolint:errcheck
		out = append(out, ev)
	}
	return out
}

func TestXlangGoSingleAgentStartedEventFirst(t *testing.T) {
	fr := newXlangFakeRuntime("xlang-go-001")
	events := drainXlang(fr)
	if len(events) == 0 {
		t.Fatal("expected events")
	}
	if events[0].Kind != "started" {
		t.Fatalf("first event must be started, got %q", events[0].Kind)
	}
}

func TestXlangGoSingleAgentRunIDConsistent(t *testing.T) {
	const rid = "xlang-go-002"
	fr := newXlangFakeRuntime(rid)
	for _, ev := range drainXlang(fr) {
		if ev.RunID != rid {
			t.Fatalf("run_id mismatch: want %q got %q", rid, ev.RunID)
		}
	}
}

func TestXlangGoSingleAgentCompletedEventLast(t *testing.T) {
	fr := newXlangFakeRuntime("xlang-go-003")
	events := drainXlang(fr)
	last := events[len(events)-1]
	if last.Kind != "completed" {
		t.Fatalf("last event must be completed, got %q", last.Kind)
	}
}

func TestXlangGoSingleAgentEventCount(t *testing.T) {
	fr := newXlangFakeRuntime("xlang-go-004")
	events := drainXlang(fr)
	if len(events) < 2 {
		t.Fatalf("expected at least 2 events, got %d", len(events))
	}
}

func TestXlangGoSingleAgentTokenTextNonEmpty(t *testing.T) {
	fr := newXlangFakeRuntime("xlang-go-005")
	var found bool
	for _, ev := range drainXlang(fr) {
		if ev.Kind == "token" && ev.Text != "" {
			found = true
		}
	}
	if !found {
		t.Fatal("expected at least one non-empty token event")
	}
}
