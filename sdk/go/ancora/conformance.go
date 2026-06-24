package ancora

// ConformanceScenario describes a scenario every Ancora binding must pass.
type ConformanceScenario struct {
	ID          string
	Description string
	Tags        []string
}

// The four canonical conformance scenarios, matching ancora-core's definitions.
var (
	ScenarioSingleAgent = ConformanceScenario{
		ID:          "single-agent",
		Description: "A single agent node runs to completion without error",
		Tags:        []string{"agent", "basic"},
	}
	ScenarioMultiAgentVerifier = ConformanceScenario{
		ID:          "multi-agent-verifier",
		Description: "An agent node and a verifier node with an explicit dependency",
		Tags:        []string{"agent", "verifier", "graph"},
	}
	ScenarioHumanInLoop = ConformanceScenario{
		ID:          "human-in-loop",
		Description: "A run suspends awaiting human approval and then resumes correctly",
		Tags:        []string{"suspend", "resume", "human"},
	}
	ScenarioCrashAndRecover = ConformanceScenario{
		ID:          "crash-and-recover",
		Description: "A run journal persists across restart and replays deterministically",
		Tags:        []string{"journal", "replay", "recovery"},
	}
)

// AllConformanceScenarios returns the four canonical scenarios in stable order.
func AllConformanceScenarios() []ConformanceScenario {
	return []ConformanceScenario{
		ScenarioSingleAgent,
		ScenarioMultiAgentVerifier,
		ScenarioHumanInLoop,
		ScenarioCrashAndRecover,
	}
}

// ConformanceResult describes the outcome of running one conformance scenario.
type ConformanceResult struct {
	ScenarioID string
	Passed     bool
	Reason     string
}

// ConformancePassed returns a passing result for the given scenario.
func ConformancePassed(scenarioID string) ConformanceResult {
	return ConformanceResult{ScenarioID: scenarioID, Passed: true}
}

// ConformanceFailed returns a failing result with a reason.
func ConformanceFailed(scenarioID, reason string) ConformanceResult {
	return ConformanceResult{ScenarioID: scenarioID, Passed: false, Reason: reason}
}
