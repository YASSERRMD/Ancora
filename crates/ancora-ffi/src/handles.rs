/// Opaque handle to a live Ancora runtime.
#[repr(C)]
pub struct AncorRuntime {
    _private: [u8; 0],
}

/// Opaque handle to a single run identifier.
#[repr(C)]
pub struct AncorRunId {
    _private: [u8; 0],
}
