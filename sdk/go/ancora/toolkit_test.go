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

func TestGoToolRegistryHasReturnsTrueAfterRegister(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("search", echoTool)
	if !reg.Has("search") {
		t.Fatal("Has should return true after Register")
	}
}

func TestGoToolRegistryHasReturnsFalseForUnknown(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	if reg.Has("missing") {
		t.Fatal("Has should return false for unregistered tool")
	}
}

func TestGoToolRegistryInvokeEchoReturnsInput(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("echo", echoTool)
	out, err := reg.Invoke("echo", []byte("hello"))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	if string(out) != "hello" {
		t.Fatalf("expected 'hello', got: %s", out)
	}
}

func TestGoToolRegistryInvokeUnknownReturnsError(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	_, err := reg.Invoke("nope", []byte("input"))
	if err == nil {
		t.Fatal("Invoke of unknown tool should return error")
	}
}

func TestGoToolRegistryRegisterTwoTools(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("a", echoTool)
	reg.Register("b", echoTool)
	if reg.Count() != 2 {
		t.Fatalf("expected count 2, got: %d", reg.Count())
	}
}

func TestGoToolRegistryOverwriteExistingTool(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("echo", func(in []byte) ([]byte, error) { return []byte("v1"), nil })
	reg.Register("echo", func(in []byte) ([]byte, error) { return []byte("v2"), nil })
	out, _ := reg.Invoke("echo", nil)
	if string(out) != "v2" {
		t.Fatalf("expected v2 after overwrite, got: %s", out)
	}
}
