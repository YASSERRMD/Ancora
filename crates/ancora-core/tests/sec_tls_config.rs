// Security: TLS config validation -- enforce minimum TLS 1.2 and strong ciphers.

#[derive(Debug, PartialEq, Clone)]
enum TlsVersion {
    Tls10,
    Tls11,
    Tls12,
    Tls13,
}

impl TlsVersion {
    fn numeric(&self) -> u16 {
        match self {
            TlsVersion::Tls10 => 0x0301,
            TlsVersion::Tls11 => 0x0302,
            TlsVersion::Tls12 => 0x0303,
            TlsVersion::Tls13 => 0x0304,
        }
    }
}

const MIN_TLS: u16 = 0x0303; // TLS 1.2

fn validate_tls_version(version: &TlsVersion) -> Result<(), String> {
    if version.numeric() < MIN_TLS {
        Err(format!("TLS version {:?} is below minimum 1.2", version))
    } else {
        Ok(())
    }
}

const STRONG_CIPHERS: &[&str] = &[
    "TLS_AES_128_GCM_SHA256",
    "TLS_AES_256_GCM_SHA384",
    "TLS_CHACHA20_POLY1305_SHA256",
];

fn validate_cipher(cipher: &str) -> Result<(), String> {
    if STRONG_CIPHERS.contains(&cipher) {
        Ok(())
    } else {
        Err(format!("cipher '{}' not in strong list", cipher))
    }
}

#[test]
fn test_tls12_accepted() {
    assert!(validate_tls_version(&TlsVersion::Tls12).is_ok());
}

#[test]
fn test_tls13_accepted() {
    assert!(validate_tls_version(&TlsVersion::Tls13).is_ok());
}

#[test]
fn test_tls10_rejected() {
    assert!(validate_tls_version(&TlsVersion::Tls10).is_err());
}

#[test]
fn test_tls11_rejected() {
    let r = validate_tls_version(&TlsVersion::Tls11);
    assert!(r.unwrap_err().contains("below minimum"));
}

#[test]
fn test_strong_cipher_accepted() {
    assert!(validate_cipher("TLS_AES_256_GCM_SHA384").is_ok());
}

#[test]
fn test_weak_cipher_rejected() {
    let r = validate_cipher("TLS_RSA_WITH_RC4_128_MD5");
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("not in strong list"));
}

#[test]
fn test_all_strong_ciphers_accepted() {
    for c in STRONG_CIPHERS { assert!(validate_cipher(c).is_ok()); }
}
