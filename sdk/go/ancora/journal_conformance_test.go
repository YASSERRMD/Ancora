package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

// journalKinds extracts the event-kind sequence from a journal event slice.
func journalKinds(evs []string) []string {
	kinds := make([]string, 0, len(evs))
	for _, ev := range evs {
		kinds = append(kinds, eventKind(ev))
	}
	return kinds
}

// equalSlices returns true when a and b have identical elements in order.
func equalSlices(a, b []string) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func TestJournalSingleAgentMatchesCoreFixture(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("journal-agent", "mock", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	evs, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	kinds := journalKinds(evs)
	expected := []string{"started", "completed"}
	if !equalSlices(kinds, expected) {
		t.Fatalf("journal mismatch: got %v, want %v", kinds, expected)
	}
}

func TestJournalSingleAgentStartedIsFirst(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("jf-agent", "mock", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	evs, _ := run.DrainEvents()
	if len(evs) == 0 {
		t.Fatal("journal must contain at least one event")
	}
	if eventKind(evs[0]) != "started" {
		t.Fatalf("journal: first event must be 'started', got: %q", evs[0])
	}
}

func TestJournalSingleAgentCompletedIsLast(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("jl-agent", "mock", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	evs, _ := run.DrainEvents()
	if len(evs) == 0 {
		t.Fatal("journal must not be empty")
	}
	last := evs[len(evs)-1]
	if eventKind(last) != "completed" {
		t.Fatalf("journal: last event must be 'completed', got: %q", last)
	}
}

func TestJournalHumanInLoopFirstPostResumeIsResumed(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("hil-j").WithModelID("mock").WithInstructions("").BuildBytes()
	run, _ := rt.StartRun(b)
	run.DrainEvents()
	run.Resume([]byte("ok"))
	postEvs, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	if len(postEvs) == 0 {
		t.Fatal("expected events after Resume")
	}
	if eventKind(postEvs[0]) != "resumed" {
		t.Fatalf("human-in-loop journal: first post-resume event must be 'resumed', got: %q", postEvs[0])
	}
}

func TestJournalHumanInLoopPostResumeEndsWithCompleted(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("hil-end").WithModelID("mock").WithInstructions("").BuildBytes()
	run, _ := rt.StartRun(b)
	run.DrainEvents()
	run.Resume([]byte("yes"))
	postEvs, _ := run.DrainEvents()
	if len(postEvs) == 0 {
		t.Fatal("expected events after Resume")
	}
	last := postEvs[len(postEvs)-1]
	if eventKind(last) != "completed" {
		t.Fatalf("human-in-loop journal: last event after Resume must be 'completed', got: %q", last)
	}
}

func TestJournalCrashRecoverKindsMatchAcrossRuntimes(t *testing.T) {
	rt1 := mustRuntime(t)
	b1, _ := ancora.NewAgentSpecBuilder().
		WithName("cr-1").WithModelID("mock").WithInstructions("").BuildBytes()
	run1, _ := rt1.StartRun(b1)
	evs1, _ := run1.DrainEvents()
	rt1.Free()

	rt2 := mustRuntime(t)
	b2, _ := ancora.NewAgentSpecBuilder().
		WithName("cr-2").WithModelID("mock").WithInstructions("").BuildBytes()
	run2, _ := rt2.StartRun(b2)
	evs2, _ := run2.DrainEvents()
	rt2.Free()

	kinds1, kinds2 := journalKinds(evs1), journalKinds(evs2)
	if !equalSlices(kinds1, kinds2) {
		t.Fatalf("crash-recover: journal kinds diverge: %v vs %v", kinds1, kinds2)
	}
}

func TestJournalConformanceScenariosHaveNonEmptyIDs(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID == "" {
			t.Fatalf("conformance scenario has empty ID: %+v", s)
		}
	}
}

func TestJournalConformanceScenariosCount(t *testing.T) {
	if n := len(ancora.AllConformanceScenarios()); n != 4 {
		t.Fatalf("expected 4 conformance scenarios, got: %d", n)
	}
}

func TestJournalConformanceScenariosUniqueIDs(t *testing.T) {
	seen := make(map[string]bool)
	for _, s := range ancora.AllConformanceScenarios() {
		if seen[s.ID] {
			t.Fatalf("duplicate scenario ID: %q", s.ID)
		}
		seen[s.ID] = true
	}
}

func TestJournalConformanceSingleAgentIDMatchesCore(t *testing.T) {
	if ancora.ScenarioSingleAgent.ID != "single-agent" {
		t.Fatalf("ScenarioSingleAgent.ID = %q, want 'single-agent'", ancora.ScenarioSingleAgent.ID)
	}
}

func TestJournalConformanceHumanInLoopTagContainsSuspend(t *testing.T) {
	for _, tag := range ancora.ScenarioHumanInLoop.Tags {
		if tag == "suspend" {
			return
		}
	}
	t.Fatal("ScenarioHumanInLoop must have 'suspend' tag")
}

func TestJournalAllScenariosHaveNonEmptyDescription(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if strings.TrimSpace(s.Description) == "" {
			t.Fatalf("scenario %q has empty description", s.ID)
		}
	}
}

func TestJournalAllScenariosHaveAtLeastOneTag(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if len(s.Tags) == 0 {
			t.Fatalf("scenario %q has no tags", s.ID)
		}
	}
}
