use crate::buffer::AncorBuffer;
use crate::error_code::AncorErrorCode;

/// Host-provided tool callback. `input` contains the tool invocation payload as bytes.
/// The callback writes its output into `out` and returns an error code.
pub type AncorToolCallback = unsafe extern "C" fn(
    input: *const u8,
    input_len: usize,
    out: *mut AncorBuffer,
) -> AncorErrorCode;
