pub mod dotnet_app;
/// ancora-appspar: multi-language sample app parity verification.
///
/// Each language ships a full sample app. This crate models them in Rust,
/// checks feature parity, verifies trace equivalence, enforces guardrails,
/// and demonstrates polyglot A2A composition.
pub mod go_app;
pub mod java_app;
pub mod parity;
pub mod polyglot;
pub mod python_app;
pub mod rust_app;
pub mod ts_app;

#[cfg(test)]
mod tests {
    mod test_dotnet_offline;
    mod test_equal_traces;
    mod test_go_offline;
    mod test_guardrails;
    mod test_java_offline;
    mod test_parity;
    mod test_polyglot_a2a;
    mod test_py_offline;
    mod test_rust_offline;
    mod test_ts_offline;
}
