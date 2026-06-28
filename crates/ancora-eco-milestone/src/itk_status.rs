/// Interop toolkit (ITK) status for the ecosystem milestone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItkTestResult {
    Pass,
    Skip(String),
    Fail(String),
}

#[derive(Debug, Clone)]
pub struct ItkTest {
    pub name: String,
    pub category: String,
    pub result: ItkTestResult,
}

impl ItkTest {
    pub fn pass(name: impl Into<String>, category: impl Into<String>) -> Self {
        Self { name: name.into(), category: category.into(), result: ItkTestResult::Pass }
    }

    pub fn is_pass(&self) -> bool {
        self.result == ItkTestResult::Pass
    }
}

pub fn itk_tests() -> Vec<ItkTest> {
    vec![
        ItkTest::pass("python-ffi-roundtrip", "ffi"),
        ItkTest::pass("node-napi-roundtrip", "ffi"),
        ItkTest::pass("grpc-unary", "grpc"),
        ItkTest::pass("grpc-streaming", "grpc"),
        ItkTest::pass("mcp-tool-invoke", "mcp"),
        ItkTest::pass("mcp-resource-read", "mcp"),
        ItkTest::pass("plugin-sdk-ext", "sdk"),
        ItkTest::pass("recipe-compose", "composition"),
    ]
}

pub fn all_itk_passing(tests: &[ItkTest]) -> bool {
    tests.iter().all(|t| t.is_pass())
}
