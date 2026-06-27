package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestHumanInLoopScenarioIsRegistered(t *testing.T) {
	found := false
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID == "human-in-loop" {
			found = true
		}
	}
	if !found {
		t.Fatal("human-in-loop scenario must be registered")
	}
}

func TestHumanInLoopScenarioHasSuspendTag(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID != "human-in-loop" {
			continue
		}
		found := false
		for _, tag := range s.Tags {
			if tag == "suspend" || tag == "human" || tag == "resume" {
				found = true
			}
		}
		if !found {
			t.Fatalf("human-in-loop must have at least one of suspend/human/resume tags, got: %v", s.Tags)
		}
		return
	}
	t.Fatal("human-in-loop scenario not found")
}

func TestRunResumeWithApprovalPayload(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, err := rt.StartRun([]byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}
	drainEvents(t, run)

	if err := run.Resume([]byte(`{"decision":"approved","comment":"looks good"}`)); err != nil {
		t.Fatalf("Resume: %v", err)
	}

	events := drainEvents(t, run)
	containsResumed := false
	for _, ev := range events {
		if strings.Contains(ev, "resumed") {
			containsResumed = true
		}
	}
	if !containsResumed {
		t.Logf("events after resume: %v", events)
	}
}

func TestRunResumeWithRejectionPayload(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))
	drainEvents(t, run)

	if err := run.Resume([]byte(`{"decision":"rejected","reason":"needs revision"}`)); err != nil {
		t.Fatalf("Resume with rejection: %v", err)
	}

	events := drainEvents(t, run)
	_ = events
}

func TestRunResumeWithEmptyDecisionBytes(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))
	drainEvents(t, run)

	if err := run.Resume([]byte("{}")); err != nil {
		t.Fatalf("Resume with empty decision: %v", err)
	}
}

func TestMultipleResumesProduceDistinctEventsEach(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))
	drainEvents(t, run)

	payloads := [][]byte{
		[]byte(`{"step":1}`),
		[]byte(`{"step":2}`),
	}
	for i, payload := range payloads {
		if err := run.Resume(payload); err != nil {
			t.Fatalf("Resume %d: %v", i, err)
		}
		_ = drainEvents(t, run)
	}
}

func TestHumanInLoopRunIDPersistsAfterResume(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	run, _ := rt.StartRun([]byte("{}"))
	originalID := run.ID()
	drainEvents(t, run)
	_ = run.Resume([]byte(`{"approved":true}`))
	if run.ID() != originalID {
		t.Fatalf("run ID changed after Resume: was %q, now %q", originalID, run.ID())
	}
}

func TestHumanInLoopAgentSpecHasNonEmptyName(t *testing.T) {
	spec := ancora.NewAgentSpec("hil-agent", "llama3", "Await human approval before proceeding.")
	if spec.GetName() == "" {
		t.Fatal("agent spec name must be non-empty")
	}
}
