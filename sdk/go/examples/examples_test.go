package examples_test

import (
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

var exampleDirs = []string{
	"single-agent",
	"multi-agent-verifier",
	"human-in-loop",
	"sqlite-persistence",
}

func buildExample(t *testing.T, dir string) string {
	t.Helper()
	out := filepath.Join(t.TempDir(), dir)
	cmd := exec.Command("go", "build", "-o", out, "./"+dir)
	if b, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("build %s: %v\n%s", dir, err, b)
	}
	return out
}

func TestAllExamplesBuild(t *testing.T) {
	for _, dir := range exampleDirs {
		dir := dir
		t.Run(dir, func(t *testing.T) {
			t.Parallel()
			buildExample(t, dir)
		})
	}
}

func TestSingleAgentExampleRuns(t *testing.T) {
	bin := buildExample(t, "single-agent")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "started run:") {
		t.Fatalf("expected 'started run:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "done") {
		t.Fatalf("expected 'done' in output, got: %s", out)
	}
}

func TestMultiAgentVerifierExampleRuns(t *testing.T) {
	bin := buildExample(t, "multi-agent-verifier")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "main-agent") {
		t.Fatalf("expected 'main-agent' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "verifier-agent") {
		t.Fatalf("expected 'verifier-agent' in output, got: %s", out)
	}
}

func TestHumanInLoopExampleRunsNonInteractive(t *testing.T) {
	bin := buildExample(t, "human-in-loop")
	cmd := exec.Command(bin)
	cmd.Stdin = strings.NewReader("")
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "run started:") {
		t.Fatalf("expected 'run started:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "done") {
		t.Fatalf("expected 'done' in output, got: %s", out)
	}
}

func TestSqlitePersistenceExampleRuns(t *testing.T) {
	bin := buildExample(t, "sqlite-persistence")
	cmd := exec.Command(bin)
	cmd.Dir = t.TempDir()
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "run started:") {
		t.Fatalf("expected 'run started:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "stored") {
		t.Fatalf("expected 'stored' in output, got: %s", out)
	}
}
