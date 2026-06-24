package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceAllScenariosProduceAtLeastOneEvent(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	for _, sc := range ancora.AllConformanceScenarios() {
		sc := sc
		t.Run(sc.ID, func(t *testing.T) {
			spec := ancora.NewAgentSpec(sc.ID, "mock", "")
			run, err := ancora.NewAgent(rt, spec).Start()
			if err != nil {
				t.Fatalf("Start: %v", err)
			}
			evs, err := run.DrainEvents()
			if err != nil {
				t.Fatalf("DrainEvents: %v", err)
			}
			if len(evs) == 0 {
				t.Fatalf("scenario %q produced no events", sc.ID)
			}
		})
	}
}

func TestConformanceAllScenariosRunIDsAreNonEmpty(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	for _, sc := range ancora.AllConformanceScenarios() {
		sc := sc
		t.Run(sc.ID, func(t *testing.T) {
			b, _ := ancora.NewAgentSpecBuilder().
				WithName(sc.ID).WithModelID("mock").WithInstructions("").BuildBytes()
			run, err := rt.StartRun(b)
			if err != nil {
				t.Fatalf("StartRun: %v", err)
			}
			if run.ID() == "" {
				t.Fatalf("scenario %q: run ID must be non-empty", sc.ID)
			}
		})
	}
}

func TestConformanceAllScenariosViaTransportAgentSucceed(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)
	for _, sc := range ancora.AllConformanceScenarios() {
		sc := sc
		t.Run(sc.ID, func(t *testing.T) {
			spec := ancora.NewAgentSpec(sc.ID+"-ta", "mock", "")
			ag := ancora.NewTransportAgent(tr, spec)
			run, err := ag.Start(context.Background())
			if err != nil {
				t.Fatalf("Start: %v", err)
			}
			if run.ID() == "" {
				t.Fatalf("TransportAgent run ID must be non-empty for %q", sc.ID)
			}
		})
	}
}

func TestConformanceSingleAgentTagContainsAgent(t *testing.T) {
	for _, tag := range ancora.ScenarioSingleAgent.Tags {
		if tag == "agent" {
			return
		}
	}
	t.Fatal("ScenarioSingleAgent must have 'agent' tag")
}

func TestConformanceMultiAgentVerifierTagContainsGraph(t *testing.T) {
	for _, tag := range ancora.ScenarioMultiAgentVerifier.Tags {
		if tag == "graph" {
			return
		}
	}
	t.Fatal("ScenarioMultiAgentVerifier must have 'graph' tag")
}

func TestConformanceCrashAndRecoverTagContainsJournal(t *testing.T) {
	for _, tag := range ancora.ScenarioCrashAndRecover.Tags {
		if tag == "journal" {
			return
		}
	}
	t.Fatal("ScenarioCrashAndRecover must have 'journal' tag")
}

func TestConformanceAllScenariosUniqueIDsFromConformanceSuite(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())
	seen := make(map[string]bool)
	for _, r := range results {
		if seen[r.ScenarioID] {
			t.Fatalf("duplicate scenario ID in results: %q", r.ScenarioID)
		}
		seen[r.ScenarioID] = true
	}
}
