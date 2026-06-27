package ancora_test

import (
	"context"
	"testing"

	"ancora.io/sdk/ancora"
)

func TestMultiAgentVerifierScenarioExistsInSuite(t *testing.T) {
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(mustRuntime(t)))
	found := false
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID == "multi-agent-verifier" {
			found = true
		}
	}
	if !found {
		t.Fatal("multi-agent-verifier scenario must be registered in the conformance suite")
	}
	_ = suite
}

func TestMultiAgentVerifierScenarioHasVerifierTag(t *testing.T) {
	var verifierScenario *ancora.ConformanceScenario
	for _, s := range ancora.AllConformanceScenarios() {
		if s.ID == "multi-agent-verifier" {
			sc := s
			verifierScenario = &sc
		}
	}
	if verifierScenario == nil {
		t.Fatal("multi-agent-verifier scenario must be registered")
	}
	found := false
	for _, tag := range verifierScenario.Tags {
		if tag == "verifier" {
			found = true
		}
	}
	if !found {
		t.Fatalf("multi-agent-verifier scenario must have 'verifier' tag, got: %v", verifierScenario.Tags)
	}
}

func TestMultiAgentVerifierSpecHasTwoTools(t *testing.T) {
	agent := ancora.NewToolSpec("agent-output", "produces a draft output")
	verifier := ancora.NewToolSpec("verifier-check", "approves or rejects the draft")

	spec := ancora.NewAgentSpecBuilder().
		WithName("multi-agent").
		WithModelID("llama3").
		WithTool(agent).
		WithTool(verifier).
		Build()

	if len(spec.GetTools()) != 2 {
		t.Fatalf("expected 2 tools, got: %d", len(spec.GetTools()))
	}
}

func TestMultiAgentVerifierSpecToolNamesAreCorrect(t *testing.T) {
	agentTool := ancora.NewToolSpec("draft", "generates the draft")
	verifierTool := ancora.NewToolSpec("approve", "approves or rejects")

	spec := ancora.NewAgentSpecBuilder().
		WithName("two-tool-agent").
		WithModelID("gpt-4o").
		WithTool(agentTool).
		WithTool(verifierTool).
		Build()

	if spec.GetTools()[0].GetName() != "draft" {
		t.Fatalf("first tool name mismatch: %q", spec.GetTools()[0].GetName())
	}
	if spec.GetTools()[1].GetName() != "approve" {
		t.Fatalf("second tool name mismatch: %q", spec.GetTools()[1].GetName())
	}
}

func TestMultiAgentVerifierRunCanStart(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()

	agentTool := ancora.NewToolSpec("produce-output", "generates draft")
	verifierTool := ancora.NewToolSpec("verify-output", "checks draft")

	spec := ancora.NewAgentSpecBuilder().
		WithName("verifier-agent").
		WithModelID("llama3").
		WithTool(agentTool).
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

func TestVerifierToolRegisteredBeforeAgentStart(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)

	approved := false
	tk.RegisterTool("verify", func(input []byte) ([]byte, error) {
		approved = true
		return []byte(`{"verdict":"approved"}`), nil
	})

	if !tk.Tools().Has("verify") {
		t.Fatal("toolkit must have verify tool before Start")
	}
	_ = approved
}

func TestConformanceSuiteRunsMultiAgentVerifierScenario(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	suite := ancora.NewConformanceSuite(ancora.NewCgoTransport(rt))
	results := suite.RunAll(context.Background())

	var verifierResult *ancora.ConformanceResult
	for _, r := range results {
		if r.ScenarioID == "multi-agent-verifier" {
			rc := r
			verifierResult = &rc
		}
	}
	if verifierResult == nil {
		t.Fatal("conformance suite must include multi-agent-verifier result")
	}
}
