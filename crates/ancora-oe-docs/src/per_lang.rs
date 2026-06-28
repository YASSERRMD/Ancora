//! Per-language observability SDK guidance and configuration snippets.

/// Supported SDK languages for observability integration.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SdkLanguage {
    Rust,
    Python,
    TypeScript,
    Go,
    Java,
}

impl std::fmt::Display for SdkLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SdkLanguage::Rust => write!(f, "Rust"),
            SdkLanguage::Python => write!(f, "Python"),
            SdkLanguage::TypeScript => write!(f, "TypeScript"),
            SdkLanguage::Go => write!(f, "Go"),
            SdkLanguage::Java => write!(f, "Java"),
        }
    }
}

/// Returns the recommended tracing crate/package for a language.
pub fn recommended_tracing_package(lang: &SdkLanguage) -> &'static str {
    match lang {
        SdkLanguage::Rust => "opentelemetry + tracing-opentelemetry",
        SdkLanguage::Python => "opentelemetry-sdk",
        SdkLanguage::TypeScript => "@opentelemetry/sdk-node",
        SdkLanguage::Go => "go.opentelemetry.io/otel",
        SdkLanguage::Java => "io.opentelemetry:opentelemetry-sdk",
    }
}

/// Returns a short setup snippet description for each language.
pub fn setup_guidance(lang: &SdkLanguage) -> &'static str {
    match lang {
        SdkLanguage::Rust => "Initialize a TracerProvider with an OTLP exporter in main.rs.",
        SdkLanguage::Python => "Call configure() with OTLPExporter before creating agents.",
        SdkLanguage::TypeScript => "Instantiate NodeSDK with OTLPTraceExporter before importing agents.",
        SdkLanguage::Go => "Use otel.SetTracerProvider with an OTLP gRPC exporter.",
        SdkLanguage::Java => "Use OpenTelemetrySdk.builder() with an OTLP exporter at startup.",
    }
}

/// Describes per-language SDK maturity.
#[derive(Debug, Clone)]
pub struct LangSdkInfo {
    pub language: SdkLanguage,
    pub tracing_package: &'static str,
    pub setup_guidance: &'static str,
}

impl LangSdkInfo {
    pub fn for_language(lang: SdkLanguage) -> Self {
        let tracing_package = recommended_tracing_package(&lang);
        let guidance = setup_guidance(&lang);
        Self {
            language: lang,
            tracing_package,
            setup_guidance: guidance,
        }
    }
}
