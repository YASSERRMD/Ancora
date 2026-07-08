pub mod dotnet_interfaces;
pub mod examples_go;
pub mod examples_py;
pub mod examples_rs;
pub mod go_interfaces;
pub mod java_interfaces;
pub mod parity;
pub mod py_classes;
pub mod registration;
/// ancora-sdkext: SDK extension ergonomics for all supported languages.
///
/// This crate provides the core traits, language adapters, registration
/// helpers, and interop parity kit that enable authors to write Ancora
/// tool extensions in Rust, Go, Python, TypeScript, .NET, and Java.
pub mod rs_traits;
pub mod ts_interfaces;

#[cfg(test)]
mod tests {
    mod test_dotnet_tool_ext;
    mod test_examples_run;
    mod test_go_tool_ext;
    mod test_interop_kit;
    mod test_java_tool_ext;
    mod test_py_tool_ext;
    mod test_rs_tool_ext;
    mod test_ts_tool_ext;
}
