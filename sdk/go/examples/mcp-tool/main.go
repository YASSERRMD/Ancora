// mcp-tool registers Go-native tool functions with a GoToolRegistry and
// wires them into a RuntimeToolkit, then shows how to invoke tools
// before or alongside an agent run.
package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"

	"ancora.io/sdk/ancora"
)

// weatherInput is the expected tool input shape.
type weatherInput struct {
	City string `json:"city"`
}

// weatherTool returns a fake weather report for any city.
func weatherTool(input []byte) ([]byte, error) {
	var req weatherInput
	if err := json.Unmarshal(input, &req); err != nil {
		return nil, err
	}
	city := strings.TrimSpace(req.City)
	if city == "" {
		city = "unknown"
	}
	result := map[string]string{
		"city":        city,
		"temperature": "22C",
		"condition":   "partly cloudy",
	}
	return json.Marshal(result)
}

// echoTool returns the input bytes verbatim.
func echoTool(input []byte) ([]byte, error) {
	return input, nil
}

func main() {
	rt, err := ancora.NewRuntime()
	if err != nil {
		fmt.Fprintf(os.Stderr, "runtime: %v\n", err)
		os.Exit(1)
	}
	defer rt.Free()

	tk := ancora.NewRuntimeToolkit(rt)
	tk.RegisterTool("get_weather", weatherTool)
	tk.RegisterTool("echo", echoTool)

	fmt.Printf("registered tools: %d\n", tk.Tools().Count())
	fmt.Printf("has 'get_weather': %v\n", tk.Tools().Has("get_weather"))

	// invoke the weather tool directly
	out, err := tk.InvokeTool("get_weather", []byte(`{"city":"Cairo"}`))
	if err != nil {
		fmt.Fprintf(os.Stderr, "invoke: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("tool result: %s\n", out)

	// invoke echo tool
	echo, err := tk.InvokeTool("echo", []byte(`{"msg":"hello ancora"}`))
	if err != nil {
		fmt.Fprintf(os.Stderr, "echo: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("echo result: %s\n", echo)

	fmt.Println("mcp-tool done")
}
