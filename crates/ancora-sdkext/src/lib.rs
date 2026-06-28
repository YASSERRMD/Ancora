/// ancora-sdkext: SDK extension ergonomics for all supported languages.
///
/// This crate provides the core traits, language adapters, registration
/// helpers, and interop parity kit that enable authors to write Ancora
/// tool extensions in Rust, Go, Python, TypeScript, .NET, and Java.

pub mod rs_traits;
pub mod go_interfaces;
pub mod py_classes;
pub mod ts_interfaces;
pub mod dotnet_interfaces;
pub mod java_interfaces;
pub mod registration;
pub mod examples_rs;
pub mod examples_go;
pub mod examples_py;
pub mod parity;

#[cfg(test)]
mod tests {
    mod test_rs_tool_ext;
    mod test_go_tool_ext;
    mod test_py_tool_ext;
    mod test_ts_tool_ext;
    mod test_dotnet_tool_ext;
    mod test_java_tool_ext;
    mod test_interop_kit;
    mod test_examples_run;
}
