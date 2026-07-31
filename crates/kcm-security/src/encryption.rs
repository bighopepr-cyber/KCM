use kcm_core::types::*;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct EncryptionKey {
    key: [u8; 32],
}

impl EncryptionKey {
    pub fn from_password(password: &str, salt: &[u8; 32]) -> Self {
        let material = [password.as_bytes(), salt.as_slice()].concat();
        let derived = blake3::derive_key("kcm-encryption", &material);
        EncryptionKey { key: derived }
    }

    pub fn random() -> Self {
        let mut key = [0u8; 32];
        getrandom::getrandom(&mut key).expect("Failed to generate random key");
        EncryptionKey { key }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.key
    }
}

pub struct EncryptedStorage;

impl EncryptedStorage {
    pub fn encrypt(plaintext: &[u8], key: &EncryptionKey) -> Result<Vec<u8>, KcmError> {
        use aes_gcm::aead::rand_core::RngCore;
        use aes_gcm::{
            aead::{Aead, KeyInit, OsRng},
            Aes256Gcm, Nonce,
        };

        let cipher = Aes256Gcm::new_from_slice(&key.key)
            .map_err(|e| KcmError::Io(format!("Cipher init failed: {}", e)))?;

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| KcmError::Io(format!("Encryption failed: {}", e)))?;

        let mut result = Vec::with_capacity(12 + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(encrypted: &[u8], key: &EncryptionKey) -> Result<Vec<u8>, KcmError> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };

        if encrypted.len() < 12 {
            return Err(KcmError::Corrupted("Encrypted data too short".to_string()));
        }

        let cipher = Aes256Gcm::new_from_slice(&key.key)
            .map_err(|e| KcmError::Io(format!("Cipher init failed: {}", e)))?;

        let nonce = Nonce::from_slice(&encrypted[..12]);
        let ciphertext = &encrypted[12..];

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| KcmError::Corrupted(format!("Decryption failed: {}", e)))
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
        let encrypted = Self::encrypt(&data, key)?;
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
        let decrypted = Self::decrypt(&data, key)?;
        File::create(dst)
            .and_then(|mut f| f.write_all(&decrypted))
            .map_err(|e| KcmError::Io(e.to_string()))?;
        Ok(())
    }
}
