package ancora

import "sync"

// ToolFunc is a Go function that handles a tool invocation.
// Input and output are raw JSON bytes.
type ToolFunc func(input []byte) ([]byte, error)

// GoToolRegistry stores named Go-native tool implementations.
// Safe for concurrent use by multiple goroutines.
type GoToolRegistry struct {
	mu    sync.RWMutex
	tools map[string]ToolFunc
}

// NewGoToolRegistry returns an empty GoToolRegistry.
func NewGoToolRegistry() *GoToolRegistry {
	return &GoToolRegistry{tools: make(map[string]ToolFunc)}
}

// Register adds a named tool. Overwrites any existing registration with the same name.
func (r *GoToolRegistry) Register(name string, fn ToolFunc) {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.tools[name] = fn
}

// Invoke calls the named tool with the given input bytes.
// Returns ErrInternal if the tool is not registered.
func (r *GoToolRegistry) Invoke(name string, input []byte) ([]byte, error) {
	r.mu.RLock()
	fn, ok := r.tools[name]
	r.mu.RUnlock()
	if !ok {
		return nil, ErrInternal
	}
	return fn(input)
}

// Count returns the number of registered tools.
func (r *GoToolRegistry) Count() int {
	r.mu.RLock()
	defer r.mu.RUnlock()
	return len(r.tools)
}

// Has reports whether a tool with the given name is registered.
func (r *GoToolRegistry) Has(name string) bool {
	r.mu.RLock()
	defer r.mu.RUnlock()
	_, ok := r.tools[name]
	return ok
}

// Unregister removes a named tool. No-op if the tool is not registered.
func (r *GoToolRegistry) Unregister(name string) {
	r.mu.Lock()
	defer r.mu.Unlock()
	delete(r.tools, name)
}

// RuntimeToolkit pairs a Runtime with a GoToolRegistry for tool-aware runs.
type RuntimeToolkit struct {
	rt    *Runtime
	tools *GoToolRegistry
}

// NewRuntimeToolkit wraps a runtime with a new tool registry.
func NewRuntimeToolkit(rt *Runtime) *RuntimeToolkit {
	return &RuntimeToolkit{rt: rt, tools: NewGoToolRegistry()}
}

// RegisterTool adds a Go function as a named tool.
func (tk *RuntimeToolkit) RegisterTool(name string, fn ToolFunc) {
	tk.tools.Register(name, fn)
}

// InvokeTool calls a registered tool by name.
func (tk *RuntimeToolkit) InvokeTool(name string, input []byte) ([]byte, error) {
	return tk.tools.Invoke(name, input)
}

// Runtime returns the underlying Runtime.
func (tk *RuntimeToolkit) Runtime() *Runtime { return tk.rt }

// Tools returns the underlying GoToolRegistry.
func (tk *RuntimeToolkit) Tools() *GoToolRegistry { return tk.tools }
