package ancora

import "unsafe"

// #include "ancora.h"
// #include <stdlib.h>
import "C"

// Run is a handle to a live agent run, identified by its string ID.
type Run struct {
	rt *Runtime
	id string
}

// ID returns the unique run identifier.
func (r *Run) ID() string { return r.id }

// PollEvent pops the next event from the run's event queue.
// Returns nil, nil when no more events are available.
func (r *Run) PollEvent() ([]byte, error) {
	cid := C.CString(r.id)
	defer C.free(unsafe.Pointer(cid))
	var out C.AncorBuffer
	code := C.ancora_run_poll(r.rt.ptr, cid, &out)
	if err := asError(code); err != nil {
		return nil, err
	}
	if out.ptr == nil || out.len == 0 {
		return nil, nil
	}
	b := bufferToBytes(out)
	C.ancora_buffer_free(out)
	return b, nil
}

// Resume provides a decision to a suspended run.
func (r *Run) Resume(decision []byte) error {
	if len(decision) == 0 {
		decision = []byte{}
	}
	cid := C.CString(r.id)
	defer C.free(unsafe.Pointer(cid))
	var ptr *C.uint8_t
	if len(decision) > 0 {
		ptr = (*C.uint8_t)(&decision[0])
	}
	code := C.ancora_run_resume(r.rt.ptr, cid, ptr, C.uintptr_t(len(decision)))
	return asError(code)
}

// bufferToBytes copies an AncorBuffer into a Go byte slice.
// The caller must still free the original C buffer.
func bufferToBytes(buf C.AncorBuffer) []byte {
	if buf.ptr == nil || buf.len == 0 {
		return nil
	}
	return C.GoBytes(unsafe.Pointer(buf.ptr), C.int(buf.len))
}

// bufferToString copies an AncorBuffer into a Go string.
func bufferToString(buf C.AncorBuffer) string {
	if buf.ptr == nil || buf.len == 0 {
		return ""
	}
	return C.GoStringN((*C.char)(unsafe.Pointer(buf.ptr)), C.int(buf.len))
}
