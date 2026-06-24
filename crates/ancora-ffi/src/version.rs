/// Return the crate version as a null-terminated C string.
#[no_mangle]
pub extern "C" fn ancora_version() -> *const std::os::raw::c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}

/// ABI version. Incremented on every breaking ABI change.
pub const ANCORA_ABI_VERSION: u32 = 1;
