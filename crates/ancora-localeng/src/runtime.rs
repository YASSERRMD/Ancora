/// Engine selection by hardware profile.
///
/// Inspects the available hardware and returns the most suitable
/// engine kind and configuration.  In production this would probe
/// real hardware; here it uses a declarative `HardwareProfile` so
/// the logic is fully unit-testable offline.

use crate::capability::Capabilities;
use crate::model::{EngineConfig, EngineKind};

/// Describes the hardware available on the host.
#[derive(Debug, Clone)]
pub struct HardwareProfile {
    pub ram_gb: f64,
    pub vram_gb: f64,
    pub cpu_cores: usize,
    pub has_cuda: bool,
    pub has_metal: bool,
    pub has_directml: bool,
    pub has_rocm: bool,
}

impl HardwareProfile {
    pub fn cpu_only(ram_gb: f64, cpu_cores: usize) -> Self {
        HardwareProfile {
            ram_gb,
            vram_gb: 0.0,
            cpu_cores,
            has_cuda: false,
            has_metal: false,
            has_directml: false,
            has_rocm: false,
        }
    }

    pub fn with_cuda(mut self, vram_gb: f64) -> Self {
        self.has_cuda = true;
        self.vram_gb = vram_gb;
        self
    }

    pub fn with_metal(mut self, vram_gb: f64) -> Self {
        self.has_metal = true;
        self.vram_gb = vram_gb;
        self
    }

    pub fn has_gpu(&self) -> bool {
        self.has_cuda || self.has_metal || self.has_directml || self.has_rocm
    }
}

/// Selection criteria combining hardware and capability requirements.
#[derive(Debug, Clone)]
pub struct SelectionCriteria {
    pub hardware: HardwareProfile,
    pub required_capabilities: Capabilities,
    pub prefer_throughput: bool,
}

impl SelectionCriteria {
    pub fn new(hardware: HardwareProfile) -> Self {
        SelectionCriteria {
            hardware,
            required_capabilities: Capabilities::default(),
            prefer_throughput: false,
        }
    }

    pub fn requiring(mut self, caps: Capabilities) -> Self {
        self.required_capabilities = caps;
        self
    }

    pub fn prefer_throughput(mut self) -> Self {
        self.prefer_throughput = true;
        self
    }
}

/// Result of engine selection.
#[derive(Debug, Clone)]
pub struct SelectionResult {
    pub engine: EngineKind,
    pub config: EngineConfig,
    pub reason: String,
}

/// Select the best engine for the given criteria.
///
/// Priority rules:
/// 1. If throughput is preferred and GPU is available -> vLLM / SGLang.
/// 2. If GPU is available but throughput not critical -> Ollama.
/// 3. CPU only, high RAM -> llama.cpp server.
/// 4. CPU only, low RAM -> llama.cpp embedded (smallest footprint).
/// 5. ONNX Runtime is preferred when embeddings are the primary need.
pub fn select_engine(criteria: &SelectionCriteria) -> SelectionResult {
    let hw = &criteria.hardware;
    let caps = &criteria.required_capabilities;

    // Embeddings-only on ONNX Runtime
    if caps.embeddings
        && !caps.streaming
        && !caps.function_calling
        && !caps.batch_inference
    {
        let config = EngineConfig::new(EngineKind::OnnxRuntime);
        return SelectionResult {
            engine: EngineKind::OnnxRuntime,
            config,
            reason: "embeddings workload maps well to onnx runtime".to_string(),
        };
    }

    // High-throughput with GPU
    if criteria.prefer_throughput && hw.has_gpu() {
        let config = EngineConfig::new(EngineKind::Vllm)
            .with_endpoint("http://127.0.0.1:8000")
            .with_gpu_layers(-1);
        return SelectionResult {
            engine: EngineKind::Vllm,
            config,
            reason: format!(
                "throughput-preferred with {}GB vram -> vllm",
                hw.vram_gb
            ),
        };
    }

    // GPU available, latency / ease preferred
    if hw.has_gpu() && hw.vram_gb >= 4.0 {
        let config = EngineConfig::new(EngineKind::Ollama)
            .with_endpoint("http://127.0.0.1:11434")
            .with_gpu_layers(-1);
        return SelectionResult {
            engine: EngineKind::Ollama,
            config,
            reason: format!(
                "gpu available ({}GB vram) -> ollama for ease",
                hw.vram_gb
            ),
        };
    }

    // CPU with adequate RAM
    if hw.ram_gb >= 16.0 {
        let config = EngineConfig::new(EngineKind::LlamaCppServer)
            .with_endpoint("http://127.0.0.1:8080")
            .with_threads(hw.cpu_cores.min(8));
        return SelectionResult {
            engine: EngineKind::LlamaCppServer,
            config,
            reason: format!(
                "cpu-only with {}GB ram -> llama.cpp server",
                hw.ram_gb
            ),
        };
    }

    // Minimal footprint
    let config = EngineConfig::new(EngineKind::LlamaCppEmbedded)
        .with_threads(hw.cpu_cores.min(4));
    SelectionResult {
        engine: EngineKind::LlamaCppEmbedded,
        config,
        reason: format!(
            "low-resource host ({}GB ram) -> llama.cpp embedded",
            hw.ram_gb
        ),
    }
}

/// List all engines whose capabilities satisfy the requirements.
pub fn compatible_engines(caps: &Capabilities) -> Vec<EngineKind> {
    Capabilities::matching_engines(caps)
}
