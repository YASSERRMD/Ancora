/// XOR-based stream cipher for offline test coverage of the encrypt/decrypt path.
/// In production, replace with AES-256-GCM via the `aes-gcm` crate.
pub fn xor_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        return data.to_vec();
    }
    data.iter()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key.len()])
        .collect()
}

pub fn xor_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // XOR is symmetric
    xor_encrypt(data, key)
}
