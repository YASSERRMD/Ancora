package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceSuiteFailedTransportReturnsFailedResult(t *testing.T) {
	tr := newErrorGRPCTransport(t)
	suite := ancora.NewConformanceSuite(tr)
	results := suite.RunAll(context.Background())
	for _, r := range results {
		if r.Passed {
			t.Errorf("scenario %q should have failed with error transport, but passed", r.ScenarioID)
		}
		if r.Reason == "" {
			t.Errorf("scenario %q: Reason must be non-empty when Passed=false", r.ScenarioID)
		}
	}
}

func TestConformanceSuiteFailedTransportReturnsFourResults(t *testing.T) {
	tr := newErrorGRPCTransport(t)
	suite := ancora.NewConformanceSuite(tr)
	results := suite.RunAll(context.Background())
	if len(results) != 4 {
		t.Fatalf("even on failure, must return 4 results, got: %d", len(results))
	}
}

func TestConformanceSuiteFailedTransportScenarioIDsAreCorrect(t *testing.T) {
	tr := newErrorGRPCTransport(t)
	suite := ancora.NewConformanceSuite(tr)
	results := suite.RunAll(context.Background())
	expected := []string{
		"single-agent",
		"multi-agent-verifier",
		"human-in-loop",
		"crash-and-recover",
	}
	for i, r := range results {
		if r.ScenarioID != expected[i] {
			t.Errorf("result[%d].ScenarioID = %q, want %q", i, r.ScenarioID, expected[i])
		}
	}
}

func TestConformanceSuiteEmptyEventTransportAllFail(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	suite := ancora.NewConformanceSuite(tr)
	results := suite.RunAll(context.Background())
	for _, r := range results {
		if r.Passed {
			t.Errorf("scenario %q must not pass with empty-event transport", r.ScenarioID)
		}
	}
}
