//! Export fine-tuned adapters to GGUF and ONNX pointer records.

use crate::model::{AdapterDescriptor, AdapterFormat};
use crate::runtime::{FtError, FtResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A pointer record for an exported adapter in GGUF format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GgufPointer {
    /// Source adapter id.
    pub adapter_id: String,
    /// Target GGUF file path (may not exist on disk in simulation).
    pub gguf_path: PathBuf,
    /// Quantization level (e.g., "Q4_K_M").
    pub quantization: String,
    /// Merged model id this GGUF was produced from.
    pub merged_model_id: String,
}

/// A pointer record for an exported adapter in ONNX format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnnxPointer {
    /// Source adapter id.
    pub adapter_id: String,
    /// Target ONNX model path.
    pub onnx_path: PathBuf,
    /// ONNX opset version.
    pub opset_version: u32,
}

/// Export options controlling the conversion process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// Target format.
    pub format: AdapterFormat,
    /// Output directory.
    pub output_dir: PathBuf,
    /// For GGUF: quantization level.
    pub quantization: Option<String>,
    /// For ONNX: opset version.
    pub onnx_opset: Option<u32>,
}

impl ExportOptions {
    pub fn gguf(output_dir: PathBuf, quantization: impl Into<String>) -> Self {
        ExportOptions {
            format: AdapterFormat::Gguf,
            output_dir,
            quantization: Some(quantization.into()),
            onnx_opset: None,
        }
    }

    pub fn onnx(output_dir: PathBuf, opset: u32) -> Self {
        ExportOptions {
            format: AdapterFormat::Onnx,
            output_dir,
            quantization: None,
            onnx_opset: Some(opset),
        }
    }
}

/// The result of an export operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportResult {
    Gguf(GgufPointer),
    Onnx(OnnxPointer),
}

impl ExportResult {
    pub fn path(&self) -> &PathBuf {
        match self {
            ExportResult::Gguf(p) => &p.gguf_path,
            ExportResult::Onnx(p) => &p.onnx_path,
        }
    }
}

/// Export an adapter to the target format, returning a pointer record.
///
/// Does not perform actual file I/O; returns a record describing the
/// expected output path that a real converter would produce.
pub fn export_adapter(
    descriptor: &AdapterDescriptor,
    base_model_id: &str,
    options: &ExportOptions,
) -> FtResult<ExportResult> {
    match &options.format {
        AdapterFormat::Gguf => {
            let quant = options
                .quantization
                .clone()
                .unwrap_or_else(|| "Q8_0".into());
            let filename = format!("{}-{}.gguf", descriptor.id.as_str(), quant);
            let gguf_path = options.output_dir.join(filename);
            Ok(ExportResult::Gguf(GgufPointer {
                adapter_id: descriptor.id.to_string(),
                gguf_path,
                quantization: quant,
                merged_model_id: base_model_id.to_string(),
            }))
        }
        AdapterFormat::Onnx => {
            let opset = options.onnx_opset.unwrap_or(17);
            let filename = format!("{}.onnx", descriptor.id.as_str());
            let onnx_path = options.output_dir.join(filename);
            Ok(ExportResult::Onnx(OnnxPointer {
                adapter_id: descriptor.id.to_string(),
                onnx_path,
                opset_version: opset,
            }))
        }
        other => Err(FtError::ExportError(format!(
            "export to format '{}' not supported",
            other
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AdapterDescriptor;
    use std::path::PathBuf;

    fn make_desc(id: &str) -> AdapterDescriptor {
        AdapterDescriptor::new(
            id,
            "Export Test",
            "llama-3.1-8b",
            PathBuf::from(format!("/tmp/{}.safetensors", id)),
        )
    }

    #[test]
    fn export_to_gguf_pointer() {
        let desc = make_desc("a1");
        let opts = ExportOptions::gguf(PathBuf::from("/exports"), "Q4_K_M");
        let result = export_adapter(&desc, "llama-3.1-8b", &opts).unwrap();
        match &result {
            ExportResult::Gguf(p) => {
                assert_eq!(p.quantization, "Q4_K_M");
                assert!(p.gguf_path.to_str().unwrap().ends_with(".gguf"));
            }
            _ => panic!("expected gguf"),
        }
    }

    #[test]
    fn export_to_onnx_pointer() {
        let desc = make_desc("a1");
        let opts = ExportOptions::onnx(PathBuf::from("/exports"), 17);
        let result = export_adapter(&desc, "llama-3.1-8b", &opts).unwrap();
        match &result {
            ExportResult::Onnx(p) => {
                assert_eq!(p.opset_version, 17);
                assert!(p.onnx_path.to_str().unwrap().ends_with(".onnx"));
            }
            _ => panic!("expected onnx"),
        }
    }

    #[test]
    fn export_unsupported_format_fails() {
        let desc = make_desc("a1");
        let opts = ExportOptions {
            format: AdapterFormat::Raw,
            output_dir: PathBuf::from("/exports"),
            quantization: None,
            onnx_opset: None,
        };
        let err = export_adapter(&desc, "llama-3.1-8b", &opts).unwrap_err();
        assert!(matches!(err, FtError::ExportError(_)));
    }
}
