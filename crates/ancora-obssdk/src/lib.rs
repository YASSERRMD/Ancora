/// ancora-obssdk: Observability SDK helpers for all Ancora language integrations.
///
/// Exposes trace, cost, and eval helpers with a notebook render path,
/// tested across Go, Python, TypeScript, .NET, Java, and Rust.

pub mod context;
pub mod go_helpers;
pub mod py_helpers;
pub mod ts_helpers;
pub mod dotnet_helpers;
pub mod java_helpers;
pub mod rs_helpers;
pub mod notebook;
pub mod eval_helpers;

#[cfg(test)]
mod tests {
    mod test_go_trace_accessor;
    mod test_py_trace_accessor;
    mod test_ts_trace_accessor;
    mod test_dotnet_trace_accessor;
    mod test_java_trace_accessor;
    mod test_rs_trace_accessor;
    mod test_notebook_render;
    mod test_eval_helper;
}
