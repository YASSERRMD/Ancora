package ancora

import (
	"runtime"
	"unsafe"
)

// Runtime is an opaque handle to an Ancora runtime instance.
type Runtime struct {
	ptr unsafe.Pointer
}

// NewRuntime allocates a new runtime. The GC calls Free when the handle
// becomes unreachable.
func NewRuntime() (*Runtime, error) {
	ptr, code := cRuntimeNew()
	if err := asError(code); err != nil {
		return nil, err
	}
	r := &Runtime{ptr: ptr}
	runtime.SetFinalizer(r, (*Runtime).Free)
	return r, nil
}

// StartRun starts a new agent run from spec bytes and returns a Run handle.
func (r *Runtime) StartRun(spec []byte) (*Run, error) {
	if len(spec) == 0 {
		spec = []byte("{}")
	}
	id, code := cRunStart(r.ptr, spec)
	if err := asError(code); err != nil {
		return nil, err
	}
	return &Run{rt: r, id: id}, nil
}

// Free releases the underlying runtime. Idempotent; subsequent calls are no-ops.
func (r *Runtime) Free() {
	if r.ptr != nil {
		cRuntimeFree(r.ptr)
		r.ptr = nil
	}
	runtime.SetFinalizer(r, nil)
}
