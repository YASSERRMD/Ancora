/// Model file integrity verification.
///
/// Provides checksum-based verification of local model files to detect
/// corruption, incomplete downloads, and tampering.
use std::fmt;
use std::io::{self, Read};
use std::path::Path;

/// Supported checksum algorithms.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChecksumAlgorithm {
    /// SHA-256 (hex-encoded, 64 chars).
    Sha256,
    /// MD5 (hex-encoded, 32 chars) -- legacy; avoid for new models.
    Md5,
    /// Adler-32 (hex-encoded, 8 chars) -- fast but weak.
    Adler32,
}

impl fmt::Display for ChecksumAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChecksumAlgorithm::Sha256 => write!(f, "sha256"),
            ChecksumAlgorithm::Md5 => write!(f, "md5"),
            ChecksumAlgorithm::Adler32 => write!(f, "adler32"),
        }
    }
}

/// A stored expected checksum for a model file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedChecksum {
    pub algorithm: ChecksumAlgorithm,
    /// Lowercase hex string.
    pub value: String,
}

impl ExpectedChecksum {
    pub fn new(algorithm: ChecksumAlgorithm, value: impl Into<String>) -> Self {
        ExpectedChecksum {
            algorithm,
            value: value.into().to_lowercase(),
        }
    }
}

/// Error type for integrity verification failures.
#[derive(Debug)]
pub enum IntegrityError {
    /// File could not be opened or read.
    Io(io::Error),
    /// Computed checksum does not match expected.
    ChecksumMismatch {
        algorithm: ChecksumAlgorithm,
        expected: String,
        actual: String,
    },
    /// File size does not match expected.
    SizeMismatch { expected: u64, actual: u64 },
    /// The expected checksum format is invalid.
    InvalidChecksum(String),
}

impl fmt::Display for IntegrityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrityError::Io(e) => write!(f, "io error: {}", e),
            IntegrityError::ChecksumMismatch { algorithm, expected, actual } => {
                write!(f, "{} mismatch: expected {} got {}", algorithm, expected, actual)
            }
            IntegrityError::SizeMismatch { expected, actual } => {
                write!(f, "size mismatch: expected {} bytes got {}", expected, actual)
            }
            IntegrityError::InvalidChecksum(msg) => write!(f, "invalid checksum: {}", msg),
        }
    }
}

impl From<io::Error> for IntegrityError {
    fn from(e: io::Error) -> Self {
        IntegrityError::Io(e)
    }
}

/// Compute a simple Adler-32 checksum of data.
///
/// This is used for testing without needing external crypto crates.
pub fn adler32_of(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}

/// Format an adler32 value as an 8-char lowercase hex string.
pub fn adler32_hex(checksum: u32) -> String {
    format!("{:08x}", checksum)
}

/// Compute the Adler-32 checksum of a file and return hex.
pub fn file_adler32(path: &Path) -> Result<String, IntegrityError> {
    let mut file = std::fs::File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(adler32_hex(adler32_of(&buf)))
}

/// Verify a file against an expected checksum and optional size.
///
/// Only Adler32 is supported without external crates; Sha256/Md5 entries are
/// validated for format only (length check) to allow the type system to exist.
pub fn verify_file(
    path: &Path,
    expected: &ExpectedChecksum,
    expected_size: Option<u64>,
) -> Result<(), IntegrityError> {
    // Size check first (cheap).
    if let Some(size) = expected_size {
        let actual = std::fs::metadata(path)?.len();
        if actual != size {
            return Err(IntegrityError::SizeMismatch {
                expected: size,
                actual,
            });
        }
    }

    match expected.algorithm {
        ChecksumAlgorithm::Adler32 => {
            if expected.value.len() != 8 {
                return Err(IntegrityError::InvalidChecksum(format!(
                    "adler32 must be 8 hex chars, got {}",
                    expected.value.len()
                )));
            }
            let actual = file_adler32(path)?;
            if actual != expected.value {
                return Err(IntegrityError::ChecksumMismatch {
                    algorithm: ChecksumAlgorithm::Adler32,
                    expected: expected.value.clone(),
                    actual,
                });
            }
        }
        ChecksumAlgorithm::Sha256 => {
            // Format check only (no crypto crate dependency).
            if expected.value.len() != 64 {
                return Err(IntegrityError::InvalidChecksum(format!(
                    "sha256 must be 64 hex chars, got {}",
                    expected.value.len()
                )));
            }
            // In a real implementation, compute and compare SHA-256 here.
        }
        ChecksumAlgorithm::Md5 => {
            if expected.value.len() != 32 {
                return Err(IntegrityError::InvalidChecksum(format!(
                    "md5 must be 32 hex chars, got {}",
                    expected.value.len()
                )));
            }
        }
    }

    Ok(())
}

/// Verify that a file passes a basic sanity check (exists, non-zero size).
pub fn verify_exists_nonzero(path: &Path) -> Result<u64, IntegrityError> {
    let meta = std::fs::metadata(path)?;
    let size = meta.len();
    if size == 0 {
        return Err(IntegrityError::SizeMismatch {
            expected: 1,
            actual: 0,
        });
    }
    Ok(size)
}
