use kcm_core::types::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct EncryptionKey {
    key: [u8; 32],
}

impl EncryptionKey {
    pub fn from_password(password: &str, salt: &[u8; 32]) -> Self {
        let hash = blake3::keyed_hash(salt, password.as_bytes());
        let mut key = [0u8; 32];
        key.copy_from_slice(hash.as_bytes());
        EncryptionKey { key }
    }

    pub fn random() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();
        let mut key = [0u8; 32];
        for i in 0..32 {
            key[i] = ((nanos >> (i % 8)) & 0xFF) as u8 ^ (i as u8).wrapping_mul(0x9E);
        }
        EncryptionKey { key }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.key
    }
}

pub struct EncryptedStorage;

impl EncryptedStorage {
    pub fn encrypt(plaintext: &[u8], key: &EncryptionKey) -> Vec<u8> {
        plaintext
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key.key[i % 32])
            .collect()
    }

    pub fn decrypt(encrypted: &[u8], key: &EncryptionKey) -> Vec<u8> {
        encrypted
            .iter()
            .enumerate()
            .map(|(i, &b)| b ^ key.key[i % 32])
            .collect()
    }

    pub fn encrypt_file<P: AsRef<Path>>(
        src: P,
        dst: P,
        key: &EncryptionKey,
    ) -> Result<(), KcmError> {
        let mut data = Vec::new();
        File::open(src)
            .and_then(|mut f| f.read_to_end(&mut data))
            .map_err(|e| KcmError::Io(e.to_string()))?;
        let encrypted = Self::encrypt(&data, key);
        File::create(dst)
            .and_then(|mut f| f.write_all(&encrypted))
            .map_err(|e| KcmError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn decrypt_file<P: AsRef<Path>>(
        src: P,
        dst: P,
        key: &EncryptionKey,
    ) -> Result<(), KcmError> {
        let mut data = Vec::new();
        File::open(src)
            .and_then(|mut f| f.read_to_end(&mut data))
            .map_err(|e| KcmError::Io(e.to_string()))?;
        let decrypted = Self::decrypt(&data, key);
        File::create(dst)
            .and_then(|mut f| f.write_all(&decrypted))
            .map_err(|e| KcmError::Io(e.to_string()))?;
        Ok(())
    }
}
