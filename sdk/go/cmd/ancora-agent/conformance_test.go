package main_test

import (
	"os/exec"
	"strings"
	"testing"
)

func execBinary(bin string, args ...string) (string, error) {
	out, err := exec.Command(bin, args...).CombinedOutput()
	return string(out), err
}

func extractRunID(output string) string {
	for _, line := range strings.SplitN(output, "\n", 2) {
		if strings.HasPrefix(line, "run_id=") {
			return strings.TrimPrefix(strings.TrimSpace(line), "run_id=")
		}
	}
	return ""
}

func TestBinaryConformanceSingleAgentHasStartedEvent(t *testing.T) {
	bin := buildBinary(t)
	out, err := execBinary(bin, "-name=single-agent", "-model=mock", "-instructions=")
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(out, "started") {
		t.Fatalf("expected 'started' in binary output, got: %s", out)
	}
}

func TestBinaryConformanceSingleAgentHasCompletedEvent(t *testing.T) {
	bin := buildBinary(t)
	out, err := execBinary(bin, "-name=completed-check", "-model=mock", "-instructions=")
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(out, "completed") {
		t.Fatalf("expected 'completed' in binary output, got: %s", out)
	}
}

func TestBinaryConformanceTwoRunsHaveDistinctIDs(t *testing.T) {
	bin := buildBinary(t)
	out1, _ := execBinary(bin, "-name=id-a", "-model=mock", "-instructions=")
	out2, _ := execBinary(bin, "-name=id-b", "-model=mock", "-instructions=")
	id1 := extractRunID(out1)
	id2 := extractRunID(out2)
	if id1 == "" || id2 == "" {
		t.Fatalf("both IDs must be non-empty: id1=%q id2=%q", id1, id2)
	}
	if id1 == id2 {
		t.Fatalf("binary invocations must produce unique run IDs, both=%q", id1)
	}
}

func TestBinaryConformanceSingleAgentEventOrderStartedThenCompleted(t *testing.T) {
	bin := buildBinary(t)
	out, err := execBinary(bin, "-name=order-check", "-model=mock", "-instructions=")
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	startedIdx := strings.Index(out, "started")
	completedIdx := strings.Index(out, "completed")
	if startedIdx < 0 || completedIdx < 0 {
		t.Fatalf("output must contain both 'started' and 'completed', got: %s", out)
	}
	if startedIdx > completedIdx {
		t.Fatalf("'started' must appear before 'completed' in output, got: %s", out)
	}
}
