/// Return codes used by all extern "C" functions.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AncorErrorCode {
    Ok = 0,
    NullPtr = 1,
    InvalidUtf8 = 2,
    Internal = 3,
}
