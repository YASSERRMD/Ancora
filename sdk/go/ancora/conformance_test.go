package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// eventKind classifies an event string to its semantic kind.
func eventKind(ev string) string {
	switch {
	case strings.Contains(ev, "started"):
		return "started"
	case strings.Contains(ev, "completed"):
		return "completed"
	case strings.Contains(ev, "resumed"):
		return "resumed"
	default:
		return ev
	}
}

// startConformanceRun creates a runtime, starts a run with an empty spec, and
// returns the runtime, run ID, and initial events. Caller must call rt.Free().
func startConformanceRun(t *testing.T) (*ancora.Runtime, string, []string) {
	t.Helper()
	rt := mustRuntime(t)
	spec := ancora.NewAgentSpec("conformance-agent", "mock", "")
	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		rt.Free()
		t.Fatalf("Start: %v", err)
	}
	evs, err := run.DrainEvents()
	if err != nil {
		rt.Free()
		t.Fatalf("DrainEvents: %v", err)
	}
	return rt, run.ID(), evs
}

// --- single-agent scenario ---

func TestConformanceSingleAgentStartReturnsOK(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("c-agent", "mock", "")
	_, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("single-agent: Start must return nil error, got: %v", err)
	}
}

func TestConformanceSingleAgentRunIDIsNonEmpty(t *testing.T) {
	rt, id, _ := startConformanceRun(t)
	defer rt.Free()
	if id == "" {
		t.Fatal("single-agent: run ID must be non-empty")
	}
}

func TestConformanceSingleAgentProducesStartedEvent(t *testing.T) {
	rt, _, evs := startConformanceRun(t)
	defer rt.Free()
	for _, ev := range evs {
		if eventKind(ev) == "started" {
			return
		}
	}
	t.Fatalf("single-agent: missing 'started' event, got: %v", evs)
}

func TestConformanceSingleAgentProducesCompletedEvent(t *testing.T) {
	rt, _, evs := startConformanceRun(t)
	defer rt.Free()
	for _, ev := range evs {
		if eventKind(ev) == "completed" {
			return
		}
	}
	t.Fatalf("single-agent: missing 'completed' event, got: %v", evs)
}

// --- multi-agent-verifier scenario ---

func TestConformanceMultiAgentVerifierTwoRunsHaveDifferentIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("v-agent", "mock", "")
	ag := ancora.NewAgent(rt, spec)
	r1, _ := ag.Start()
	r2, _ := ag.Start()
	if r1.ID() == r2.ID() {
		t.Fatalf("multi-agent-verifier: each run must have a unique ID, both=%q", r1.ID())
	}
}

// --- human-in-loop scenario ---

func TestConformanceHumanInLoopResumeProducesResumedEvent(t *testing.T) {
	rt, _, _ := startConformanceRun(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("hil").WithModelID("mock").WithInstructions("").BuildBytes()
	rawRun, err := rt.StartRun(b)
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	rawRun.DrainEvents()
	if err := rawRun.Resume([]byte("approved")); err != nil {
		t.Fatalf("Resume: %v", err)
	}
	postEvs, err := rawRun.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents post-resume: %v", err)
	}
	for _, ev := range postEvs {
		if eventKind(ev) == "resumed" {
			return
		}
	}
	t.Fatalf("human-in-loop: missing 'resumed' event after Resume, got: %v", postEvs)
}

// --- crash-and-recover scenario ---

func TestConformanceCrashAndRecoverEventKindsAreDeterministic(t *testing.T) {
	rt1, _, evs1 := startConformanceRun(t)
	rt1.Free()
	rt2, _, evs2 := startConformanceRun(t)
	rt2.Free()

	kinds1 := make([]string, len(evs1))
	kinds2 := make([]string, len(evs2))
	for i, e := range evs1 {
		kinds1[i] = eventKind(e)
	}
	for i, e := range evs2 {
		kinds2[i] = eventKind(e)
	}
	if len(kinds1) != len(kinds2) {
		t.Fatalf("crash-and-recover: event count differs: %d vs %d", len(kinds1), len(kinds2))
	}
	for i := range kinds1 {
		if kinds1[i] != kinds2[i] {
			t.Fatalf("crash-and-recover: event[%d] differs: %q vs %q", i, kinds1[i], kinds2[i])
		}
	}
}
