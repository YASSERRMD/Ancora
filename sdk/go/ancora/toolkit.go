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
