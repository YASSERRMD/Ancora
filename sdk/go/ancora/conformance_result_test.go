package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformancePassedHasPassedTrue(t *testing.T) {
	r := ancora.ConformancePassed("single-agent")
	if !r.Passed {
		t.Fatal("ConformancePassed must set Passed=true")
	}
}

func TestConformancePassedHasCorrectScenarioID(t *testing.T) {
	r := ancora.ConformancePassed("human-in-loop")
	if r.ScenarioID != "human-in-loop" {
		t.Fatalf("ScenarioID = %q, want 'human-in-loop'", r.ScenarioID)
	}
}

func TestConformanceFailedHasPassedFalse(t *testing.T) {
	r := ancora.ConformanceFailed("crash-and-recover", "journal diverged")
	if r.Passed {
		t.Fatal("ConformanceFailed must set Passed=false")
	}
}

func TestConformanceFailedHasReason(t *testing.T) {
	r := ancora.ConformanceFailed("multi-agent-verifier", "ids matched")
	if r.Reason != "ids matched" {
		t.Fatalf("Reason = %q, want 'ids matched'", r.Reason)
	}
}

func TestConformancePassedReasonIsEmpty(t *testing.T) {
	r := ancora.ConformancePassed("single-agent")
	if r.Reason != "" {
		t.Fatalf("ConformancePassed.Reason must be empty, got: %q", r.Reason)
	}
}

func TestConformanceScenariosAllPassSingleAgentRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("pass-agent", "mock", "")
	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	evs, _ := run.DrainEvents()
	result := ancora.ConformancePassed(ancora.ScenarioSingleAgent.ID)
	for _, ev := range evs {
		if eventKind(ev) == "completed" {
			result = ancora.ConformancePassed(ancora.ScenarioSingleAgent.ID)
			break
		}
	}
	if !result.Passed {
		r := ancora.ConformanceFailed(ancora.ScenarioSingleAgent.ID, "no completed event")
		t.Fatalf("single-agent conformance failed: %s", r.Reason)
	}
}
