use std::os::raw::c_char;

use crate::buffer::{ancora_buffer_from_str, AncorBuffer};
use crate::error_code::AncorErrorCode;
use crate::handles::AncorRuntime;
use crate::memory_backend::{
    decode_collection_spec, decode_point_ids, decode_points, decode_query_request,
    encode_scored_points,
};
use crate::runtime::InnerRuntime;

fn c_str_to_str<'a>(ptr: *const c_char) -> &'a str {
    unsafe { std::ffi::CStr::from_ptr(ptr) }
        .to_str()
        .unwrap_or("")
}

/// Create a vector collection. `spec_bytes` is JSON:
/// `{"name":"docs","dimensions":768,"distance":"cosine"}` (`distance` is one
/// of `"cosine"`, `"dot"`, `"l2"`, defaulting to `"cosine"`).
/// Returns `NullPtr` if `rt`/`spec_bytes` is null, `InvalidUtf8` if the JSON
/// is malformed or missing `name`/`dimensions`, `Internal` if the backend
/// rejects the request (e.g. collection already exists).
///
/// # Safety
/// `rt` must be a live runtime pointer. `spec_bytes` must point to at least
/// `spec_len` valid bytes.
#[no_mangle]
pub unsafe extern "C" fn ancora_memory_create_collection(
    rt: *mut AncorRuntime,
    spec_bytes: *const u8,
    spec_len: usize,
) -> AncorErrorCode {
    if rt.is_null() || spec_bytes.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let bytes = unsafe { std::slice::from_raw_parts(spec_bytes, spec_len) };
    let Some(spec) = decode_collection_spec(bytes) else {
        return AncorErrorCode::InvalidUtf8;
    };
    let inner = unsafe { &*rt.cast::<InnerRuntime>() };
    match inner.memory.create_collection(spec) {
        Ok(()) => AncorErrorCode::Ok,
        Err(_) => AncorErrorCode::Internal,
    }
}

/// Drop a vector collection by name. Returns `NullPtr` if `rt`/`name` is
/// null, `Internal` if the backend rejects the request (e.g. collection
/// does not exist).
///
/// # Safety
/// `rt` must be a live runtime pointer. `name` must be a valid
/// null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn ancora_memory_drop_collection(
    rt: *mut AncorRuntime,
    name: *const c_char,
) -> AncorErrorCode {
    if rt.is_null() || name.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let name = c_str_to_str(name);
    let inner = unsafe { &*rt.cast::<InnerRuntime>() };
    match inner.memory.drop_collection(name) {
        Ok(()) => AncorErrorCode::Ok,
        Err(_) => AncorErrorCode::Internal,
    }
}

/// Upsert points into a collection. `points_bytes` is a JSON array:
/// `[{"id":1,"vector":[0.1,0.2],"payload":{"text":"..."}}]`. Point ids are
/// non-negative integers (required by the pgvector-backed store; kept
/// consistent across backends so the same request works against either).
/// Returns `NullPtr` if any pointer is null, `InvalidUtf8` if `points_bytes`
/// is not a valid points array, `Internal` if the backend rejects the
/// request (e.g. dimension mismatch, unknown collection).
///
/// # Safety
/// `rt` must be a live runtime pointer. `collection` must be a valid
/// null-terminated C string. `points_bytes` must point to at least
/// `points_len` valid bytes.
#[no_mangle]
pub unsafe extern "C" fn ancora_memory_upsert(
    rt: *mut AncorRuntime,
    collection: *const c_char,
    points_bytes: *const u8,
    points_len: usize,
) -> AncorErrorCode {
    if rt.is_null() || collection.is_null() || points_bytes.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let collection = c_str_to_str(collection);
    let bytes = unsafe { std::slice::from_raw_parts(points_bytes, points_len) };
    let Some(points) = decode_points(bytes) else {
        return AncorErrorCode::InvalidUtf8;
    };
    let inner = unsafe { &*rt.cast::<InnerRuntime>() };
    match inner.memory.upsert(collection, points) {
        Ok(()) => AncorErrorCode::Ok,
        Err(_) => AncorErrorCode::Internal,
    }
}

/// Run a similarity query against a collection. `query_bytes` is JSON:
/// `{"vector":[0.1,0.2],"top_k":5,"score_threshold":0.0}` (`top_k` defaults
/// to 10, `score_threshold` is optional). Writes a JSON array of
/// `{"id":..,"score":..,"payload":{..}}` into `out`.
/// Returns `NullPtr` if any pointer is null, `InvalidUtf8` if `query_bytes`
/// is malformed, `Internal` if the backend rejects the request.
///
/// # Safety
/// `rt` must be a live runtime pointer. `collection` must be a valid
/// null-terminated C string. `query_bytes` must point to at least
/// `query_len` valid bytes. `out` must point to valid, writable memory for
/// an `AncorBuffer`.
#[no_mangle]
pub unsafe extern "C" fn ancora_memory_query(
    rt: *mut AncorRuntime,
    collection: *const c_char,
    query_bytes: *const u8,
    query_len: usize,
    out: *mut AncorBuffer,
) -> AncorErrorCode {
    if rt.is_null() || collection.is_null() || query_bytes.is_null() || out.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let collection = c_str_to_str(collection);
    let bytes = unsafe { std::slice::from_raw_parts(query_bytes, query_len) };
    let Some(req) = decode_query_request(bytes) else {
        return AncorErrorCode::InvalidUtf8;
    };
    let inner = unsafe { &*rt.cast::<InnerRuntime>() };
    match inner.memory.query(collection, req) {
        Ok(results) => {
            unsafe { *out = ancora_buffer_from_str(&encode_scored_points(&results)) };
            AncorErrorCode::Ok
        }
        Err(_) => AncorErrorCode::Internal,
    }
}

/// Delete points from a collection by id. `ids_bytes` is a JSON array of
/// non-negative integers: `[1,2,3]`. Returns `NullPtr` if any pointer is
/// null, `InvalidUtf8` if `ids_bytes` is not a valid id array, `Internal` if
/// the backend rejects the request.
///
/// # Safety
/// `rt` must be a live runtime pointer. `collection` must be a valid
/// null-terminated C string. `ids_bytes` must point to at least `ids_len`
/// valid bytes.
#[no_mangle]
pub unsafe extern "C" fn ancora_memory_delete(
    rt: *mut AncorRuntime,
    collection: *const c_char,
    ids_bytes: *const u8,
    ids_len: usize,
) -> AncorErrorCode {
    if rt.is_null() || collection.is_null() || ids_bytes.is_null() {
        return AncorErrorCode::NullPtr;
    }
    let collection = c_str_to_str(collection);
    let bytes = unsafe { std::slice::from_raw_parts(ids_bytes, ids_len) };
    let Some(ids) = decode_point_ids(bytes) else {
        return AncorErrorCode::InvalidUtf8;
    };
    let inner = unsafe { &*rt.cast::<InnerRuntime>() };
    match inner.memory.delete(collection, ids) {
        Ok(()) => AncorErrorCode::Ok,
        Err(_) => AncorErrorCode::Internal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{ancora_free_runtime, ancora_runtime_new};

    fn make_rt() -> *mut AncorRuntime {
        let mut rt = std::ptr::null_mut();
        unsafe { ancora_runtime_new(&mut rt) };
        rt
    }

    fn cstr(s: &str) -> std::ffi::CString {
        std::ffi::CString::new(s).unwrap()
    }

    #[test]
    fn create_collection_with_null_runtime_returns_null_ptr() {
        let spec = br#"{"name":"docs","dimensions":2}"#;
        let code = unsafe {
            ancora_memory_create_collection(std::ptr::null_mut(), spec.as_ptr(), spec.len())
        };
        assert_eq!(code, AncorErrorCode::NullPtr);
    }

    #[test]
    fn create_collection_with_malformed_spec_returns_invalid_utf8() {
        let rt = make_rt();
        let spec = b"not json";
        let code = unsafe { ancora_memory_create_collection(rt, spec.as_ptr(), spec.len()) };
        assert_eq!(code, AncorErrorCode::InvalidUtf8);
        unsafe { ancora_free_runtime(rt) };
    }

    #[test]
    fn full_create_upsert_query_delete_round_trip() {
        let rt = make_rt();
        let name = cstr("docs");

        let spec = br#"{"name":"docs","dimensions":2,"distance":"cosine"}"#;
        let code = unsafe { ancora_memory_create_collection(rt, spec.as_ptr(), spec.len()) };
        assert_eq!(code, AncorErrorCode::Ok);

        let points = br#"[{"id":1,"vector":[1.0,0.0],"payload":{"text":"alpha"}},
                           {"id":2,"vector":[0.0,1.0],"payload":{"text":"beta"}}]"#;
        let code =
            unsafe { ancora_memory_upsert(rt, name.as_ptr(), points.as_ptr(), points.len()) };
        assert_eq!(code, AncorErrorCode::Ok);

        let query = br#"{"vector":[1.0,0.0],"top_k":1}"#;
        let mut out = AncorBuffer {
            ptr: std::ptr::null_mut(),
            len: 0,
        };
        let code = unsafe {
            ancora_memory_query(rt, name.as_ptr(), query.as_ptr(), query.len(), &mut out)
        };
        assert_eq!(code, AncorErrorCode::Ok);
        let slice = unsafe { std::slice::from_raw_parts(out.ptr, out.len) };
        let json = String::from_utf8_lossy(slice).into_owned();
        assert!(
            json.contains("\"alpha\""),
            "closest point to [1,0] should be id 1 (alpha), got: {json}"
        );
        unsafe { crate::buffer::ancora_buffer_free(out) };

        let ids = b"[1,2]";
        let code = unsafe { ancora_memory_delete(rt, name.as_ptr(), ids.as_ptr(), ids.len()) };
        assert_eq!(code, AncorErrorCode::Ok);

        let code = unsafe { ancora_memory_drop_collection(rt, name.as_ptr()) };
        assert_eq!(code, AncorErrorCode::Ok);

        unsafe { ancora_free_runtime(rt) };
    }

    #[test]
    fn drop_unknown_collection_returns_internal() {
        let rt = make_rt();
        let name = cstr("nope");
        let code = unsafe { ancora_memory_drop_collection(rt, name.as_ptr()) };
        assert_eq!(code, AncorErrorCode::Internal);
        unsafe { ancora_free_runtime(rt) };
    }
}
