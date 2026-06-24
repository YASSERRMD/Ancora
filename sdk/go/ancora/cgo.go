package ancora

import "unsafe"

// #cgo CFLAGS: -I${SRCDIR}/../../../crates/ancora-ffi/include
// #cgo LDFLAGS: -L${SRCDIR}/../../../target/debug -lancora_ffi
// #cgo linux LDFLAGS: -lpthread -ldl
// #include "ancora.h"
// #include <stdlib.h>
import "C"

func cRuntimeNew() (unsafe.Pointer, uint32) {
	var ptr *C.AncorRuntime
	code := C.ancora_runtime_new(&ptr)
	return unsafe.Pointer(ptr), uint32(code)
}

func cRuntimeFree(p unsafe.Pointer) {
	C.ancora_free_runtime((*C.AncorRuntime)(p))
}

func cRunStart(rt unsafe.Pointer, spec []byte) (string, uint32) {
	var out C.AncorBuffer
	code := C.ancora_run_start(
		(*C.AncorRuntime)(rt),
		(*C.uint8_t)(&spec[0]),
		C.uintptr_t(len(spec)),
		&out,
	)
	if uint32(code) != 0 {
		return "", uint32(code)
	}
	var id string
	if out.ptr != nil && out.len > 0 {
		id = C.GoStringN((*C.char)(unsafe.Pointer(out.ptr)), C.int(out.len))
	}
	C.ancora_buffer_free(out)
	return id, 0
}

func cRunPoll(rt unsafe.Pointer, id string) ([]byte, uint32) {
	cid := C.CString(id)
	defer C.free(unsafe.Pointer(cid))
	var out C.AncorBuffer
	code := C.ancora_run_poll((*C.AncorRuntime)(rt), cid, &out)
	if uint32(code) != 0 || out.ptr == nil || out.len == 0 {
		return nil, uint32(code)
	}
	b := C.GoBytes(unsafe.Pointer(out.ptr), C.int(out.len))
	C.ancora_buffer_free(out)
	return b, 0
}

func cRunResume(rt unsafe.Pointer, id string, decision []byte) uint32 {
	cid := C.CString(id)
	defer C.free(unsafe.Pointer(cid))
	if len(decision) == 0 {
		return uint32(C.ancora_run_resume((*C.AncorRuntime)(rt), cid, nil, 0))
	}
	return uint32(C.ancora_run_resume(
		(*C.AncorRuntime)(rt),
		cid,
		(*C.uint8_t)(&decision[0]),
		C.uintptr_t(len(decision)),
	))
}
