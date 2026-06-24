package ancora

import "unsafe"

// #include "ancora.h"
// #include <stdlib.h>
import "C"

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
