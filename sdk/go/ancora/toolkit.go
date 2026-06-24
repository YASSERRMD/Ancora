package ancora

// ToolFunc is a Go function that handles a tool invocation.
// Input and output are raw JSON bytes.
type ToolFunc func(input []byte) ([]byte, error)
