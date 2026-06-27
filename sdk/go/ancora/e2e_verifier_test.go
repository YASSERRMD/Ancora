package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestE2EVerifierConformancePasses(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	for _, r := range results {
		if r.ScenarioID == "multi-agent-verifier" && !r.Passed {
			t.Fatalf("conformance 'multi-agent-verifier' failed: %s", r.Reason)
		}
	}
}

func TestE2EVerifierScenarioIsRegistered(t *testing.T) {
	found := false
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID == "multi-agent-verifier" {
			found = true
		}
	}
	if !found {
		t.Fatal("multi-agent-verifier must be registered in AllConformanceScenarios")
	}
}

func TestE2EVerifierScenarioHasGraphTag(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID != "multi-agent-verifier" {
			continue
		}
		hasTag := false
		for _, tag := range s.Tags {
			if tag == "graph" || tag == "verifier" || tag == "agent" {
				hasTag = true
			}
		}
		if !hasTag {
			t.Fatalf("multi-agent-verifier must have graph/verifier/agent tag, got: %v", s.Tags)
		}
		return
	}
	t.Fatal("multi-agent-verifier scenario not found")
}

func TestE2EVerifierTwoToolSpecsCanBeBuilt(t *testing.T) {
	agentTool := ancora.NewToolSpec("search", "searches the web")
	verifierTool := ancora.NewToolSpec("verify", "verifies a claim")
	spec := ancora.NewAgentSpecBuilder().
		WithName("agent-node").
		WithModelID("llama3").
		WithTool(agentTool).
		WithTool(verifierTool).
		Build()
	if len(spec.GetTools()) != 2 {
		t.Fatalf("expected 2 tools, got: %d", len(spec.GetTools()))
	}
}

func TestE2EVerifierRunStartsSuccessfully(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	verifierTool := ancora.NewToolSpec("verify", "verifies a claim")
	spec := ancora.NewAgentSpecBuilder().
		WithName("verifier-agent").
		WithModelID("llama3").
		WithTool(verifierTool).
		Build()

	run, err := ancora.NewAgent(rt, spec).Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	if run.ID() == "" {
		t.Fatal("run ID must be non-empty")
	}
}

func TestE2EVerifierToolkitHasBothTools(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("agent-search", func(input []byte) ([]byte, error) {
		return []byte(`{"results":[]}`), nil
	})
	tk.RegisterTool("verifier-verify", func(input []byte) ([]byte, error) {
		return []byte(`{"verdict":"pass"}`), nil
	})

	if !tk.Tools().Has("agent-search") {
		t.Fatal("toolkit must have agent-search")
	}
	if !tk.Tools().Has("verifier-verify") {
		t.Fatal("toolkit must have verifier-verify")
	}
}

func TestE2EVerifierStoringTransportRecordsRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	store, err := ancora.OpenSqliteStore(":memory:")
	if err != nil {
		t.Fatalf("OpenSqliteStore: %v", err)
	}
	defer store.Close()

	tr := ancora.NewStoringTransport(ancora.NewCgoTransport(rt), store)
	runID, err := tr.StartRun(context.Background(), []byte("{}"))
	if err != nil {
		t.Fatalf("StartRun: %v", err)
	}

	has, err := store.HasRun(runID)
	if err != nil {
		t.Fatalf("HasRun: %v", err)
	}
	if !has {
		t.Fatalf("store must record verifier run %q", runID)
	}
}

func TestE2EVerifierConformanceResultScenarioIDsMatch(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	expected := map[string]bool{
		"single-agent":        false,
		"multi-agent-verifier": false,
		"human-in-loop":       false,
		"crash-and-recover":   false,
	}
	for _, r := range results {
		expected[r.ScenarioID] = true
	}
	for id, seen := range expected {
		if !seen {
			t.Fatalf("conformance result missing scenario: %q", id)
		}
	}
}

func TestE2EVerifierRegistryInvokesVerifyTool(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("verify", func(input []byte) ([]byte, error) {
		return []byte(`{"verdict":"pass"}`), nil
	})

	out, err := reg.Invoke("verify", []byte(`{"claim":"sky is blue"}`))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	if len(out) == 0 {
		t.Fatal("verify tool must return non-empty output")
	}
}

func TestE2EVerifierSpecHasVerifierTag(t *testing.T) {
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID != "multi-agent-verifier" {
			continue
		}
		for _, tag := range s.Tags {
			if tag == "verifier" {
				return
			}
		}
		t.Fatalf("multi-agent-verifier scenario must have 'verifier' tag, got: %v", s.Tags)
	}
}
