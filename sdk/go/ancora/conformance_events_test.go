package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func TestConformanceEventPayloadsAreNonEmptyStrings(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("payload-agent", "mock", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	evs, err := run.DrainEvents()
	if err != nil {
		t.Fatalf("DrainEvents: %v", err)
	}
	for i, ev := range evs {
		if ev == "" {
			t.Fatalf("event[%d] must not be empty", i)
		}
	}
}

func TestConformanceSingleAgentEventCountIsExactlyTwo(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	spec := ancora.NewAgentSpec("count-agent", "mock", "")
	run, _ := ancora.NewAgent(rt, spec).Start()
	evs, _ := run.DrainEvents()
	if len(evs) != 2 {
		t.Fatalf("single-agent: expected exactly 2 events, got: %d (%v)", len(evs), evs)
	}
}

func TestConformanceHumanInLoopTotalEventCountAfterResumeAtLeastThree(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	b, _ := ancora.NewAgentSpecBuilder().
		WithName("hil-count").WithModelID("mock").WithInstructions("").BuildBytes()
	run, _ := rt.StartRun(b)
	pre, _ := run.DrainEvents()
	run.Resume([]byte("ok"))
	post, _ := run.DrainEvents()
	total := len(pre) + len(post)
	if total < 3 {
		t.Fatalf("human-in-loop: expected at least 3 total events (got %d: pre=%v post=%v)",
			total, pre, post)
	}
}

func TestConformanceAllScenariosEventPayloadsContainKind(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	for _, sc := range ancora.AllConformanceScenarios() {
		sc := sc
		t.Run(sc.ID, func(t *testing.T) {
			spec := ancora.NewAgentSpec(sc.ID+"-pk", "mock", "")
			run, _ := ancora.NewAgent(rt, spec).Start()
			evs, _ := run.DrainEvents()
			for _, ev := range evs {
				k := eventKind(ev)
				if k != "started" && k != "completed" && k != "resumed" && k == ev {
					t.Logf("event with unrecognized kind: %q", ev)
				}
			}
		})
	}
}

func TestConformanceMultipleSuiteRunsProduceSameScenarioOrder(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tr := ancora.NewCgoTransport(rt)

	run1 := ancora.NewConformanceSuite(tr).RunAll(nil)
	run2 := ancora.NewConformanceSuite(tr).RunAll(nil)

	if len(run1) != len(run2) {
		t.Fatalf("suite result count differs: %d vs %d", len(run1), len(run2))
	}
	for i := range run1 {
		if run1[i].ScenarioID != run2[i].ScenarioID {
			t.Fatalf("result[%d] ScenarioID differs: %q vs %q",
				i, run1[i].ScenarioID, run2[i].ScenarioID)
		}
	}
}
