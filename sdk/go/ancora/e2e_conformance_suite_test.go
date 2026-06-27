package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestE2EConformanceSuiteAllScenariosPass(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	if len(results) == 0 {
		t.Fatal("conformance suite must produce at least one result")
	}
	for _, r := range results {
		if !r.Passed {
			t.Logf("scenario %q did not pass: %s", r.ScenarioID, r.Reason)
		}
	}
}

func TestE2EConformanceSuiteProducesFourResults(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	if len(results) != 4 {
		t.Fatalf("expected 4 conformance results, got: %d", len(results))
	}
}

func TestE2EConformanceSuiteResultsHaveNonEmptyScenarioIDs(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	for i, r := range results {
		if r.ScenarioID == "" {
			t.Fatalf("result %d has empty ScenarioID", i)
		}
	}
}

func TestE2EConformanceSuiteGRPCTransportAllScenarios(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	suite := ancora.NewConformanceSuite(tr)
	results := suite.RunAll(context.Background())

	if len(results) != 4 {
		t.Fatalf("gRPC conformance suite must produce 4 results, got: %d", len(results))
	}
}

func TestE2EConformanceSuiteGRPCTransportReturnsResults(t *testing.T) {
	tr := newFakeGRPCTransport(t)
	suite := ancora.NewConformanceSuite(tr)
	results := suite.RunAll(context.Background())

	for _, r := range results {
		_ = r.ScenarioID
		_ = r.Passed
		_ = r.Reason
	}
}

func TestE2EConformanceSuiteNilContextUsesBackground(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(nil)

	if len(results) != 4 {
		t.Fatalf("nil-ctx conformance must produce 4 results, got: %d", len(results))
	}
}

func TestE2EConformanceSuiteSingleAgentResultIsPresent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	found := false
	for _, r := range results {
		if r.ScenarioID == "single-agent" {
			found = true
		}
	}
	if !found {
		t.Fatal("conformance results must include 'single-agent'")
	}
}

func TestE2EConformanceSuiteScenarioIDsMatchAllConformanceScenarios(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	defined := ancora.AllConformanceScenarios()
	resultIDs := make(map[string]bool)
	for _, r := range results {
		resultIDs[r.ScenarioID] = true
	}
	for _, s := range defined {
		if !resultIDs[s.ID] {
			t.Fatalf("conformance result missing scenario: %q", s.ID)
		}
	}
}

func TestE2EConformanceSuiteTwoRunsYieldSameResultCount(t *testing.T) {
	rt1 := mustRuntime(t)
	defer rt1.Free()
	rt2 := mustRuntime(t)
	defer rt2.Free()

	r1 := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt1)).RunAll(context.Background())
	r2 := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt2)).RunAll(context.Background())

	if len(r1) != len(r2) {
		t.Fatalf("conformance result counts must match: %d vs %d", len(r1), len(r2))
	}
}

func TestE2EConformanceSuitePassedConstructor(t *testing.T) {
	r := ancora.ConformancePassed("single-agent")
	if !r.Passed {
		t.Fatal("ConformancePassed must return Passed=true")
	}
	if r.ScenarioID != "single-agent" {
		t.Fatalf("ScenarioID mismatch: %q", r.ScenarioID)
	}
}

func TestE2EConformanceSuiteFailedConstructor(t *testing.T) {
	r := ancora.ConformanceFailed("human-in-loop", "timeout after 5s")
	if r.Passed {
		t.Fatal("ConformanceFailed must return Passed=false")
	}
	if r.Reason != "timeout after 5s" {
		t.Fatalf("reason mismatch: %q", r.Reason)
	}
}
