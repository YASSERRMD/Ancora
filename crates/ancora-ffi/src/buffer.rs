/// Owned byte buffer passed across the FFI boundary.
/// The caller is responsible for freeing with `ancora_buffer_free`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AncorBuffer {
    pub ptr: *mut u8,
    pub len: usize,
}

/// Allocate a buffer containing a copy of `bytes`.
/// Returns a zero-length buffer with null ptr if `bytes` is empty.
#[no_mangle]
pub extern "C" fn ancora_buffer_new(bytes: *const u8, len: usize) -> AncorBuffer {
    if len == 0 || bytes.is_null() {
        return AncorBuffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
    }
    let slice = unsafe { std::slice::from_raw_parts(bytes, len) };
    let mut vec = slice.to_vec();
    let ptr = vec.as_mut_ptr();
    let out_len = vec.len();
    std::mem::forget(vec);
    AncorBuffer { ptr, len: out_len }
}

/// Free a buffer previously created by `ancora_buffer_new` or `ancora_buffer_from_str`.
/// Passing a zero-length or null-ptr buffer is a no-op.
#[no_mangle]
pub extern "C" fn ancora_buffer_free(buf: AncorBuffer) {
    if buf.ptr.is_null() || buf.len == 0 {
        return;
    }
    unsafe {
        drop(Vec::from_raw_parts(buf.ptr, buf.len, buf.len));
    }
}

/// Build a buffer from a Rust string slice (UTF-8 bytes, no null terminator).
pub fn ancora_buffer_from_str(s: &str) -> AncorBuffer {
    ancora_buffer_new(s.as_ptr(), s.len())
}
