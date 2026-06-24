/// Return the crate version as a null-terminated C string.
#[no_mangle]
pub extern "C" fn ancora_version() -> *const std::os::raw::c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr().cast()
}
