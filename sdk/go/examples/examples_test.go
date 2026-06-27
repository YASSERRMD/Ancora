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
	"event-chan",
	"conformance-runner",
	"grpc-transport",
	"structured-output",
	"streaming-chat",
	"rag-lancedb",
	"mcp-tool",
	"glm-provider",
	"durable-restart",
	"cost-otel",
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

func TestEventChanExampleRuns(t *testing.T) {
	bin := buildExample(t, "event-chan")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "run started:") {
		t.Fatalf("expected 'run started:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "received") {
		t.Fatalf("expected 'received' in output, got: %s", out)
	}
}

func TestConformanceRunnerExampleRuns(t *testing.T) {
	bin := buildExample(t, "conformance-runner")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "conformance:") {
		t.Fatalf("expected 'conformance:' summary in output, got: %s", out)
	}
}

func TestAllExamplesOutputRunID(t *testing.T) {
	for _, dir := range []string{"single-agent", "multi-agent-verifier", "event-chan"} {
		dir := dir
		t.Run(dir, func(t *testing.T) {
			bin := buildExample(t, dir)
			out, err := exec.Command(bin).CombinedOutput()
			if err != nil {
				t.Fatalf("run %s: %v\n%s", dir, err, out)
			}
			if !strings.Contains(string(out), "run") {
				t.Fatalf("%s: expected run info in output, got: %s", dir, out)
			}
		})
	}
}

func TestStructuredOutputExampleRuns(t *testing.T) {
	bin := buildExample(t, "structured-output")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "schema:") {
		t.Fatalf("expected 'schema:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "done") {
		t.Fatalf("expected 'done' in output, got: %s", out)
	}
}

func TestStreamingChatExampleRuns(t *testing.T) {
	bin := buildExample(t, "streaming-chat")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "stream started:") {
		t.Fatalf("expected 'stream started:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "stream done:") {
		t.Fatalf("expected 'stream done:' in output, got: %s", out)
	}
}

func TestRagLancedbExampleRuns(t *testing.T) {
	bin := buildExample(t, "rag-lancedb")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "retrieved") {
		t.Fatalf("expected 'retrieved' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "run started:") {
		t.Fatalf("expected 'run started:' in output, got: %s", out)
	}
}

func TestMcpToolExampleRuns(t *testing.T) {
	bin := buildExample(t, "mcp-tool")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "registered tools:") {
		t.Fatalf("expected 'registered tools:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "tool result:") {
		t.Fatalf("expected 'tool result:' in output, got: %s", out)
	}
}

func TestGlmProviderExampleRuns(t *testing.T) {
	bin := buildExample(t, "glm-provider")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "model=glm-4") {
		t.Fatalf("expected 'model=glm-4' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "glm-provider done") {
		t.Fatalf("expected 'glm-provider done' in output, got: %s", out)
	}
}

func TestDurableRestartExampleRuns(t *testing.T) {
	bin := buildExample(t, "durable-restart")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "first run:") {
		t.Fatalf("expected 'first run:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "replayed") {
		t.Fatalf("expected 'replayed' in output, got: %s", out)
	}
}

func TestCostOtelExampleRuns(t *testing.T) {
	bin := buildExample(t, "cost-otel")
	out, err := exec.Command(bin).CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "run started:") {
		t.Fatalf("expected 'run started:' in output, got: %s", out)
	}
	if !strings.Contains(string(out), "span:") {
		t.Fatalf("expected 'span:' in output, got: %s", out)
	}
}
