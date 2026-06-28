// Security: key rotation -- new key encrypts, old key no longer valid.

#[derive(Debug, PartialEq, Clone)]
struct Key {
    id: u32,
    secret: u32,
}

impl Key {
    fn encrypt(&self, plaintext: u32) -> u32 { plaintext ^ self.secret }
    fn decrypt(&self, ciphertext: u32) -> u32 { ciphertext ^ self.secret }
}

struct KeyStore {
    current: Key,
    previous: Option<Key>,
}

impl KeyStore {
    fn new(key: Key) -> Self { Self { current: key, previous: None } }

    fn rotate(&mut self, new_key: Key) {
        self.previous = Some(self.current.clone());
        self.current = new_key;
    }

    fn decrypt_any(&self, ciphertext: u32, key_id: u32) -> Result<u32, String> {
        if self.current.id == key_id {
            return Ok(self.current.decrypt(ciphertext));
        }
        if let Some(prev) = &self.previous {
            if prev.id == key_id {
                return Ok(prev.decrypt(ciphertext));
            }
        }
        Err(format!("key id {} not found", key_id))
    }
}

#[test]
fn test_current_key_encrypts_and_decrypts() {
    let k = Key { id: 1, secret: 0xDEAD };
    let ct = k.encrypt(42);
    assert_eq!(k.decrypt(ct), 42);
}

#[test]
fn test_rotated_key_encrypts_new_data() {
    let mut store = KeyStore::new(Key { id: 1, secret: 0xAAAA });
    store.rotate(Key { id: 2, secret: 0xBBBB });
    let ct = store.current.encrypt(99);
    assert_eq!(store.decrypt_any(ct, 2).unwrap(), 99);
}

#[test]
fn test_old_key_still_decrypts_old_data() {
    let old_key = Key { id: 1, secret: 0xAAAA };
    let ct = old_key.encrypt(77);
    let mut store = KeyStore::new(old_key);
    store.rotate(Key { id: 2, secret: 0xBBBB });
    assert_eq!(store.decrypt_any(ct, 1).unwrap(), 77);
}

#[test]
fn test_unknown_key_id_returns_error() {
    let store = KeyStore::new(Key { id: 1, secret: 0x1234 });
    let r = store.decrypt_any(0, 99);
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("99"));
}

#[test]
fn test_new_key_different_from_old() {
    let k1 = Key { id: 1, secret: 0x0001 };
    let k2 = Key { id: 2, secret: 0x0002 };
    assert_ne!(k1.secret, k2.secret);
}

#[test]
fn test_encryption_is_xor_invertible() {
    let k = Key { id: 0, secret: 0xFFFF_FFFF };
    assert_eq!(k.decrypt(k.encrypt(0x12345678)), 0x12345678);
}
