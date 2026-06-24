package ancora

import "runtime"

// #include "ancora.h"
import "C"

// Runtime is an opaque handle to an Ancora runtime instance.
type Runtime struct {
	ptr *C.AncorRuntime
}

// NewRuntime allocates a new runtime. Returns an error if allocation fails.
func NewRuntime() (*Runtime, error) {
	var ptr *C.AncorRuntime
	code := C.ancora_runtime_new(&ptr)
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
	var out C.AncorBuffer
	code := C.ancora_run_start(
		r.ptr,
		(*C.uint8_t)(&spec[0]),
		C.uintptr_t(len(spec)),
		&out,
	)
	if err := asError(code); err != nil {
		return nil, err
	}
	id := bufferToString(out)
	C.ancora_buffer_free(out)
	return &Run{rt: r, id: id}, nil
}

// Free releases the underlying runtime. Idempotent; subsequent calls are no-ops.
func (r *Runtime) Free() {
	if r.ptr != nil {
		C.ancora_free_runtime(r.ptr)
		r.ptr = nil
	}
	runtime.SetFinalizer(r, nil)
}
