package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceSuiteNewReturnsNonNil(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	if suite == nil {
		t.Fatal("NewConformanceSuite returned nil")
	}
}

func TestConformanceSuiteRunAllReturnsFourResults(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())
	if len(results) != 4 {
		t.Fatalf("expected 4 results, got: %d", len(results))
	}
}

func TestConformanceSuiteAllResultsHaveScenarioID(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	for _, r := range suite.RunAll(context.Background()) {
		if r.ScenarioID == "" {
			t.Fatal("result must have non-empty ScenarioID")
		}
	}
}

func TestConformanceSuiteCgoTransportAllPass(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	for _, r := range suite.RunAll(context.Background()) {
		if !r.Passed {
			t.Errorf("scenario %q failed: %s", r.ScenarioID, r.Reason)
		}
	}
}

func TestConformanceSuiteGRPCTransportRunAllReturnsFourResults(t *testing.T) {
	suite := ancora.NewConformanceSuite(newFakeGRPCTransport(t))
	results := suite.RunAll(context.Background())
	if len(results) != 4 {
		t.Fatalf("expected 4 results, got: %d", len(results))
	}
}

func TestConformanceSuiteStoringTransportRecordsAllRuns(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	store := mustInMemoryStore(t)
	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	suite := ancora.NewConformanceSuite(tr)
	suite.RunAll(context.Background())
	n, err := store.RunCount()
	if err != nil {
		t.Fatalf("RunCount: %v", err)
	}
	if n != 4 {
		t.Fatalf("expected 4 stored runs, got: %d", n)
	}
}

func TestConformanceSuiteRunAllScenarioIDsMatchExpected(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())
	expected := []string{
		"single-agent",
		"multi-agent-verifier",
		"human-in-loop",
		"crash-and-recover",
	}
	for i, r := range results {
		if r.ScenarioID != expected[i] {
			t.Errorf("results[%d].ScenarioID = %q, want %q", i, r.ScenarioID, expected[i])
		}
	}
}
