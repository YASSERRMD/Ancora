package ancora_test

import (
	"errors"
	"testing"

	"ancora.io/sdk/ancora"
)

var errToolFailed = errors.New("tool: upstream service unavailable")
var errToolInvalid = errors.New("tool: invalid input")

func failingTool(input []byte) ([]byte, error) {
	return nil, errToolFailed
}

func invalidInputTool(input []byte) ([]byte, error) {
	if len(input) == 0 {
		return nil, errToolInvalid
	}
	return input, nil
}

func TestToolErrorPropagatesFromInvoke(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("failing", failingTool)

	_, err := reg.Invoke("failing", []byte("{}"))
	if err == nil {
		t.Fatal("expected error from failing tool, got nil")
	}
}

func TestToolErrorMessageIsPreserved(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("failing", failingTool)

	_, err := reg.Invoke("failing", nil)
	if err == nil {
		t.Fatal("expected error")
	}
	if err.Error() == "" {
		t.Fatal("tool error message must not be empty")
	}
}

func TestToolErrorDoesNotAffectOtherTools(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("failing", failingTool)
	reg.Register("echo", echoTool)

	_, errFail := reg.Invoke("failing", []byte("{}"))
	if errFail == nil {
		t.Fatal("failing tool must return error")
	}

	out, errEcho := reg.Invoke("echo", []byte("hello"))
	if errEcho != nil {
		t.Fatalf("echo tool must succeed: %v", errEcho)
	}
	if string(out) != "hello" {
		t.Fatalf("echo output mismatch: %s", out)
	}
}

func TestToolErrorOnEmptyInput(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("invalid-input", invalidInputTool)

	_, err := reg.Invoke("invalid-input", []byte{})
	if err == nil {
		t.Fatal("expected error for empty input")
	}
}

func TestToolSuccessAfterErrorReset(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("invalid-input", invalidInputTool)

	// First call: fails.
	_, err := reg.Invoke("invalid-input", []byte{})
	if err == nil {
		t.Fatal("expected error on empty input")
	}

	// Second call with valid input: succeeds.
	out, err := reg.Invoke("invalid-input", []byte(`{"ok":true}`))
	if err != nil {
		t.Fatalf("expected success on valid input: %v", err)
	}
	if string(out) != `{"ok":true}` {
		t.Fatalf("expected round-trip output, got: %s", out)
	}
}

func TestUnregisteredToolReturnsError(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	_, err := reg.Invoke("nonexistent", []byte("input"))
	if err == nil {
		t.Fatal("invoking unregistered tool must return error")
	}
}

func TestToolkitInvokeToolErrorPropagates(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("fail-tool", failingTool)

	_, err := tk.InvokeTool("fail-tool", []byte("{}"))
	if err == nil {
		t.Fatal("expected error from failing tool in toolkit")
	}
}

func TestToolkitInvokeUnregisteredToolReturnsError(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)

	_, err := tk.InvokeTool("not-registered", []byte("{}"))
	if err == nil {
		t.Fatal("invoking unregistered tool via toolkit must return error")
	}
}

func TestMultipleToolErrorsAreIsolated(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	reg.Register("t1", failingTool)
	reg.Register("t2", failingTool)
	reg.Register("t3", echoTool)

	_, e1 := reg.Invoke("t1", nil)
	_, e2 := reg.Invoke("t2", nil)
	out3, e3 := reg.Invoke("t3", []byte("ping"))

	if e1 == nil { t.Fatal("t1 must fail") }
	if e2 == nil { t.Fatal("t2 must fail") }
	if e3 != nil { t.Fatalf("t3 must succeed: %v", e3) }
	if string(out3) != "ping" { t.Fatalf("t3 output wrong: %s", out3) }
}
