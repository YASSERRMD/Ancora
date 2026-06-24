/// Owned byte buffer passed across the FFI boundary.
/// The caller is responsible for freeing with `ancora_buffer_free`.
#[repr(C)]
pub struct AncorBuffer {
    pub ptr: *mut u8,
    pub len: usize,
}
