package ancora_test

import (
	"testing"

	"ancora.io/sdk/ancora"
)

func echoTool(input []byte) ([]byte, error) { return input, nil }

func TestGoToolRegistryRegisterIncreasesCount(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("echo", echoTool)
	if reg.Count() != 1 {
		t.Fatalf("expected count 1, got: %d", reg.Count())
	}
}
