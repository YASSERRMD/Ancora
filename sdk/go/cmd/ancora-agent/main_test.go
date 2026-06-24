package main_test

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"
)

// buildBinary compiles ancora-agent and returns its path.
func buildBinary(t *testing.T) string {
	t.Helper()
	dir := t.TempDir()
	out := filepath.Join(dir, "ancora-agent")
	cmd := exec.Command("go", "build", "-o", out, ".")
	cmd.Dir = "."
	if b, err := cmd.CombinedOutput(); err != nil {
		t.Fatalf("build: %v\n%s", err, b)
	}
	return out
}

func TestSingleBinaryBuilds(t *testing.T) {
	bin := buildBinary(t)
	if _, err := os.Stat(bin); err != nil {
		t.Fatalf("binary not found: %v", err)
	}
}

func TestSingleBinaryRunsOffline(t *testing.T) {
	bin := buildBinary(t)
	cmd := exec.Command(bin, "-name=offline-agent", "-model=llama3", "-instructions=hello")
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	if !strings.Contains(string(out), "run_id=") {
		t.Fatalf("expected run_id in output, got: %s", out)
	}
}

func TestSingleBinaryWithDatabasePersists(t *testing.T) {
	bin := buildBinary(t)
	dir := t.TempDir()
	dbPath := filepath.Join(dir, "runs.db")
	cmd := exec.Command(bin,
		"-name=db-agent",
		"-model=llama3",
		"-instructions=test",
		"-db="+dbPath,
	)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("run with db: %v\noutput: %s", err, out)
	}
	if _, err := os.Stat(dbPath); err != nil {
		t.Fatalf("db file not created: %v", err)
	}
}

func TestSingleBinaryOutputsRunID(t *testing.T) {
	bin := buildBinary(t)
	cmd := exec.Command(bin, "-name=id-check", "-model=llama3", "-instructions=hi")
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("run: %v\noutput: %s", err, out)
	}
	lines := strings.Split(strings.TrimSpace(string(out)), "\n")
	if len(lines) == 0 || !strings.HasPrefix(lines[0], "run_id=") {
		t.Fatalf("first line must be run_id=..., got: %q", lines[0])
	}
	id := strings.TrimPrefix(lines[0], "run_id=")
	if id == "" {
		t.Fatal("run ID must not be empty")
	}
}
