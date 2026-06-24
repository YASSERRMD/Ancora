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

func TestGoToolExecutesWithinRun(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("echo", echoTool)

	spec := ancora.NewAgentSpec("tool-agent", "llama3", "use tools")
	ag := ancora.NewAgent(rt, spec)
	run, err := ag.Start()
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	run.DrainEvents()

	out, err := tk.InvokeTool("echo", []byte(`{"msg":"hello"}`))
	if err != nil {
		t.Fatalf("InvokeTool: %v", err)
	}
	if string(out) != `{"msg":"hello"}` {
		t.Fatalf("expected echo output, got: %s", out)
	}
	if run.ID() == "" {
		t.Fatal("run ID must remain valid after tool invocation")
	}
}

func TestRuntimeToolkitRegisterTool(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("calc", echoTool)
	if !tk.Tools().Has("calc") {
		t.Fatal("toolkit must have tool after RegisterTool")
	}
}

func TestRuntimeToolkitRuntimeAccessor(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	if tk.Runtime() != rt {
		t.Fatal("RuntimeToolkit.Runtime() must return original runtime")
	}
}

func TestToolWithSchemaRoundTrips(t *testing.T) {
	type calcInput struct {
		A int `json:"a" schema:"first operand"`
		B int `json:"b" schema:"second operand"`
	}
	schema, err := ancora.SchemaFromStruct(calcInput{})
	if err != nil {
		t.Fatalf("SchemaFromStruct: %v", err)
	}
	tool := ancora.NewToolSpecBuilder().
		WithToolName("calc").
		WithDescription("adds two numbers").
		WithInputSchema(schema).
		Build()
	if tool.GetInputSchemaJson() == "" {
		t.Fatal("ToolSpec must carry input schema JSON")
	}
	if !contains(tool.GetInputSchemaJson(), `"a"`) {
		t.Fatalf("schema missing field 'a', got: %s", tool.GetInputSchemaJson())
	}
}

func TestGoToolWithAgentSpecContainsToolName(t *testing.T) {
	rt := mustRuntime(t)
	defer rt.Free()
	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("search", echoTool)

	tool := ancora.NewToolSpec("search", "searches the web")
	spec := ancora.NewAgentSpecBuilder().
		WithName("searcher").
		WithModelID("gpt-4o").
		WithTool(tool).
		Build()
	if spec.GetTools()[0].GetName() != "search" {
		t.Fatalf("AgentSpec tool name mismatch: %q", spec.GetTools()[0].GetName())
	}
	if !tk.Tools().Has("search") {
		t.Fatal("toolkit must have search tool")
	}
}

func TestGoToolRegistryCountAfterMultipleRegistrations(t *testing.T) {
	reg := ancora.NewGoToolRegistry()
	for _, name := range []string{"a", "b", "c", "d"} {
		reg.Register(name, echoTool)
	}
	if reg.Count() != 4 {
		t.Fatalf("expected 4 tools, got: %d", reg.Count())
	}
}

func TestToolFuncWithJsonInputAndOutput(t *testing.T) {
	uppercaseTool := ancora.ToolFunc(func(input []byte) ([]byte, error) {
		return []byte(string(input) + "-processed"), nil
	})
	reg := ancora.NewGoToolRegistry()
	reg.Register("process", uppercaseTool)
	out, err := reg.Invoke("process", []byte("data"))
	if err != nil {
		t.Fatalf("Invoke: %v", err)
	}
	if string(out) != "data-processed" {
		t.Fatalf("expected 'data-processed', got: %s", out)
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
