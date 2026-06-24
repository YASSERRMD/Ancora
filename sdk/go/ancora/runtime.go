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
