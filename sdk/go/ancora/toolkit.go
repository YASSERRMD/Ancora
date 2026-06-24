package ancora

// ToolFunc is a Go function that handles a tool invocation.
// Input and output are raw JSON bytes.
type ToolFunc func(input []byte) ([]byte, error)

// GoToolRegistry stores named Go-native tool implementations.
type GoToolRegistry struct {
	tools map[string]ToolFunc
}

// NewGoToolRegistry returns an empty GoToolRegistry.
func NewGoToolRegistry() *GoToolRegistry {
	return &GoToolRegistry{tools: make(map[string]ToolFunc)}
}

// Register adds a named tool. Overwrites any existing registration with the same name.
func (r *GoToolRegistry) Register(name string, fn ToolFunc) {
	r.tools[name] = fn
}

// Invoke calls the named tool with the given input bytes.
// Returns ErrInternal if the tool is not registered.
func (r *GoToolRegistry) Invoke(name string, input []byte) ([]byte, error) {
	fn, ok := r.tools[name]
	if !ok {
		return nil, ErrInternal
	}
	return fn(input)
}

// Count returns the number of registered tools.
func (r *GoToolRegistry) Count() int { return len(r.tools) }

// Has reports whether a tool with the given name is registered.
func (r *GoToolRegistry) Has(name string) bool {
	_, ok := r.tools[name]
	return ok
}
