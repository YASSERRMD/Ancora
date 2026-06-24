package ancora

import (
	"context"
	"fmt"
	"strings"
)

// ConformanceSuite runs all canonical conformance scenarios against a Transport
// and reports results.
type ConformanceSuite struct {
	tr Transport
}

// NewConformanceSuite returns a suite that runs scenarios via tr.
func NewConformanceSuite(tr Transport) *ConformanceSuite {
	return &ConformanceSuite{tr: tr}
}

// RunAll executes every scenario and returns one result per scenario.
func (s *ConformanceSuite) RunAll(ctx context.Context) []ConformanceResult {
	scenarios := AllConformanceScenarios()
	results := make([]ConformanceResult, 0, len(scenarios))
	for _, sc := range scenarios {
		results = append(results, s.run(ctx, sc))
	}
	return results
}

// run executes a single scenario and returns its result.
func (s *ConformanceSuite) run(ctx context.Context, sc ConformanceScenario) ConformanceResult {
	specJSON := fmt.Sprintf(`{"name":%q,"model_id":"mock","instructions":""}`, sc.ID)
	runID, err := s.tr.StartRun(ctx, []byte(specJSON))
	if err != nil {
		return ConformanceFailed(sc.ID, fmt.Sprintf("StartRun: %v", err))
	}
	var events []string
	for {
		ev, err := s.tr.PollRun(ctx, runID)
		if err != nil {
			return ConformanceFailed(sc.ID, fmt.Sprintf("PollRun: %v", err))
		}
		if ev == nil {
			break
		}
		events = append(events, string(ev))
	}
	if len(events) == 0 {
		return ConformanceFailed(sc.ID, "no events produced")
	}
	if !anyKind(events, "started") {
		return ConformanceFailed(sc.ID, fmt.Sprintf("missing 'started' event: %v", events))
	}
	return ConformancePassed(sc.ID)
}

func anyKind(evs []string, kind string) bool {
	for _, ev := range evs {
		if strings.Contains(ev, kind) {
			return true
		}
	}
	return false
}
