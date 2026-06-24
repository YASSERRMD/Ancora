package ancora_test

import (
	"strings"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceScenarioDescriptionsAreNonEmpty(t *testing.T) {
	for _, sc := range ancora.AllConformanceScenarios() {
		if strings.TrimSpace(sc.Description) == "" {
			t.Fatalf("scenario %q has empty description", sc.ID)
		}
	}
}

func TestConformanceScenarioCrashAndRecoverHasRecoveryTag(t *testing.T) {
	for _, tag := range ancora.ScenarioCrashAndRecover.Tags {
		if tag == "recovery" {
			return
		}
	}
	t.Fatal("ScenarioCrashAndRecover must have 'recovery' tag")
}

func TestConformanceScenarioMultiAgentVerifierHasVerifierTag(t *testing.T) {
	for _, tag := range ancora.ScenarioMultiAgentVerifier.Tags {
		if tag == "verifier" {
			return
		}
	}
	t.Fatal("ScenarioMultiAgentVerifier must have 'verifier' tag")
}

func TestConformanceScenarioIDsMatchCoreExpected(t *testing.T) {
	expected := []string{
		"single-agent",
		"multi-agent-verifier",
		"human-in-loop",
		"crash-and-recover",
	}
	scenarios := ancora.AllConformanceScenarios()
	for i, sc := range scenarios {
		if sc.ID != expected[i] {
			t.Errorf("scenarios[%d].ID = %q, want %q", i, sc.ID, expected[i])
		}
	}
}

func TestConformanceScenarioDescriptionsContainKeywords(t *testing.T) {
	tests := []struct {
		sc      ancora.ConformanceScenario
		keyword string
	}{
		{ancora.ScenarioSingleAgent, "agent"},
		{ancora.ScenarioHumanInLoop, "approval"},
		{ancora.ScenarioCrashAndRecover, "journal"},
	}
	for _, tc := range tests {
		if !strings.Contains(strings.ToLower(tc.sc.Description), tc.keyword) {
			t.Errorf("scenario %q description must contain %q, got: %q",
				tc.sc.ID, tc.keyword, tc.sc.Description)
		}
	}
}

func TestConformanceSuiteRunAllScenarioOrderIsStable(t *testing.T) {
	rt1 := mustRuntime(t)
	defer rt1.Free()
	rt2 := mustRuntime(t)
	defer rt2.Free()

	r1 := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt1)).RunAll(nil)
	r2 := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt2)).RunAll(nil)

	if len(r1) != len(r2) {
		t.Fatalf("different result counts: %d vs %d", len(r1), len(r2))
	}
	for i := range r1 {
		if r1[i].ScenarioID != r2[i].ScenarioID {
			t.Fatalf("order differs at %d: %q vs %q", i, r1[i].ScenarioID, r2[i].ScenarioID)
		}
	}
}
