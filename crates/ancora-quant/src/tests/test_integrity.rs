use crate::integrity::{
    adler32_hex, adler32_of, verify_exists_nonzero, verify_file, ChecksumAlgorithm,
    ExpectedChecksum, IntegrityError,
};

fn temp_path(tag: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(format!("/tmp/ancora_quant_test_{}.bin", tag))
}

#[test]
fn adler32_known_value() {
    // "Wikipedia" should produce 0x11E60398
    let data = b"Wikipedia";
    let checksum = adler32_of(data);
    assert_eq!(checksum, 0x11E60398);
}

#[test]
fn adler32_empty_is_one() {
    assert_eq!(adler32_of(b""), 1);
}

#[test]
fn adler32_hex_is_eight_chars() {
    let h = adler32_hex(0xDEADBEEF);
    assert_eq!(h.len(), 8);
    assert_eq!(h, "deadbeef");
}

#[test]
fn integrity_check_passes_for_valid_file() {
    let path = temp_path("valid");
    std::fs::write(&path, b"hello world").unwrap();
    let checksum_val = adler32_hex(adler32_of(b"hello world"));
    let expected = ExpectedChecksum::new(ChecksumAlgorithm::Adler32, &checksum_val);
    let result = verify_file(&path, &expected, Some(11));
    let _ = std::fs::remove_file(&path);
    assert!(result.is_ok());
}

#[test]
fn integrity_check_rejects_corruption() {
    let path = temp_path("corrupt");
    // Compute checksum of original content.
    let good_checksum = adler32_hex(adler32_of(b"original content"));
    // Write corrupted content to file.
    std::fs::write(&path, b"corrupted content").unwrap();
    let expected = ExpectedChecksum::new(ChecksumAlgorithm::Adler32, &good_checksum);
    let result = verify_file(&path, &expected, None);
    let _ = std::fs::remove_file(&path);
    assert!(matches!(
        result,
        Err(IntegrityError::ChecksumMismatch { .. })
    ));
}

#[test]
fn integrity_size_mismatch_detected() {
    let path = temp_path("size");
    std::fs::write(&path, b"hello").unwrap();
    let checksum = adler32_hex(adler32_of(b"hello"));
    let expected = ExpectedChecksum::new(ChecksumAlgorithm::Adler32, &checksum);
    let result = verify_file(&path, &expected, Some(99)); // wrong size
    let _ = std::fs::remove_file(&path);
    assert!(matches!(result, Err(IntegrityError::SizeMismatch { .. })));
}

#[test]
fn verify_exists_nonzero_ok() {
    let path = temp_path("nonzero");
    std::fs::write(&path, b"data").unwrap();
    let result = verify_exists_nonzero(&path);
    let _ = std::fs::remove_file(&path);
    assert!(result.is_ok());
}

#[test]
fn verify_exists_nonzero_empty_fails() {
    let path = temp_path("empty");
    std::fs::write(&path, b"").unwrap();
    let result = verify_exists_nonzero(&path);
    let _ = std::fs::remove_file(&path);
    assert!(result.is_err());
}
