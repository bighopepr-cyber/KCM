use kcm_core::types::*;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use parking_lot::RwLock;

pub const KEY_SIZE: usize = 32;
pub const NONCE_SIZE: usize = 12;
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 1024;
pub const MAX_KEY_NAME_LENGTH: usize = 256;
pub const KEYSTORE_MAGIC: &[u8; 4] = b"KSK1";

pub struct EncryptionKey {
    key: [u8; KEY_SIZE],
}

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        for byte in self.key.iter_mut() {
            // SAFETY: key is a fixed-size [u8; 32] array. Volatile write prevents
            // compiler optimization from removing the zeroing.
            unsafe {
                std::ptr::write_volatile(byte, 0);
            }
        }
    }
}

impl EncryptionKey {
    pub fn from_password(password: &str, salt: &[u8; KEY_SIZE]) -> Result<Self, KcmError> {
        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "Password must be at least {} characters",
                MIN_PASSWORD_LENGTH
            )));
        }
        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "Password must not exceed {} characters",
                MAX_PASSWORD_LENGTH
            )));
        }

        let mut material = Vec::with_capacity(password.len() + KEY_SIZE);
        material.extend_from_slice(password.as_bytes());
        material.extend_from_slice(salt);
        
        let derived = blake3::derive_key("kcm-encryption-v1", &material);
        Ok(EncryptionKey { key: derived })
    }

    pub fn random() -> Result<Self, KcmError> {
        let mut key = [0u8; KEY_SIZE];
        getrandom::getrandom(&mut key)
            .map_err(|e| KcmError::Io(format!("CSPRNG failure: {}", e)))?;
        Ok(EncryptionKey { key })
    }

    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.key
    }

    pub fn constant_time_eq(&self, other: &EncryptionKey) -> bool {
        let mut result = 0u8;
        for (a, b) in self.key.iter().zip(other.key.iter()) {
            result |= a ^ b;
        }
        result == 0
    }
}

#[derive(Debug, Clone)]
pub struct KeyEntry {
    pub name: String,
    pub encrypted_key: Vec<u8>,
    pub created_at: i64,
    pub rotated_at: Option<i64>,
    pub version: u32,
}

pub struct EncryptedKeyStore {
    keys: Arc<RwLock<HashMap<String, KeyEntry>>>,
    master_key: EncryptionKey,
}

impl EncryptedKeyStore {
    pub fn new(master_key: EncryptionKey) -> Self {
        EncryptedKeyStore {
            keys: Arc::new(RwLock::new(HashMap::new())),
            master_key,
        }
    }

    pub fn generate_key(&self, name: &str) -> Result<(), KcmError> {
        if name.len() > MAX_KEY_NAME_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "Key name cannot exceed {} characters",
                MAX_KEY_NAME_LENGTH
            )));
        }
        let key = EncryptionKey::random()?;
        let encrypted = EncryptedStorage::encrypt(key.as_bytes(), &self.master_key)?;
        let entry = KeyEntry {
            name: name.to_string(),
            encrypted_key: encrypted,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
            rotated_at: None,
            version: 1,
        };
        self.keys.write().insert(name.to_string(), entry);
        Ok(())
    }

    pub fn get_key(&self, name: &str) -> Result<EncryptionKey, KcmError> {
        let keys = self.keys.read();
        let entry = keys
            .get(name)
            .ok_or_else(|| KcmError::NotFound(format!("Key not found: {}", name)))?;
        let decrypted = EncryptedStorage::decrypt(&entry.encrypted_key, &self.master_key)?;
        if decrypted.len() != KEY_SIZE {
            return Err(KcmError::Corrupted(
                "Decrypted key has invalid size".to_string(),
            ));
        }
        let mut key_bytes = [0u8; KEY_SIZE];
        key_bytes.copy_from_slice(&decrypted);
        Ok(EncryptionKey { key: key_bytes })
    }

    pub fn rotate_key(&self, name: &str) -> Result<(), KcmError> {
        let new_key = EncryptionKey::random()?;
        let encrypted = EncryptedStorage::encrypt(new_key.as_bytes(), &self.master_key)?;
        let mut keys = self.keys.write();
        let entry = keys
            .get_mut(name)
            .ok_or_else(|| KcmError::NotFound(format!("Key not found: {}", name)))?;
        entry.encrypted_key = encrypted;
        entry.rotated_at = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64,
        );
        entry.version += 1;
        Ok(())
    }

    pub fn delete_key(&self, name: &str) -> Result<(), KcmError> {
        self.keys
            .write()
            .remove(name)
            .ok_or_else(|| KcmError::NotFound(format!("Key not found: {}", name)))?;
        Ok(())
    }

    pub fn list_keys(&self) -> Vec<String> {
        self.keys.read().keys().cloned().collect()
    }

    pub fn key_exists(&self, name: &str) -> bool {
        self.keys.read().contains_key(name)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), KcmError> {
        let keys = self.keys.read();
        let mut data = Vec::new();
        data.extend_from_slice(KEYSTORE_MAGIC);
        data.extend_from_slice(&(keys.len() as u32).to_le_bytes());
        for (name, entry) in keys.iter() {
            let name_bytes = name.as_bytes();
            data.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
            data.extend_from_slice(name_bytes);
            data.extend_from_slice(&(entry.encrypted_key.len() as u32).to_le_bytes());
            data.extend_from_slice(&entry.encrypted_key);
            data.extend_from_slice(&entry.created_at.to_le_bytes());
            data.extend_from_slice(&entry.rotated_at.unwrap_or(0).to_le_bytes());
            data.extend_from_slice(&entry.version.to_le_bytes());
        }
        let mut file = File::create(path.as_ref())
            .map_err(|e| KcmError::Io(format!("Failed to create keystore file: {}", e)))?;
        file.write_all(&data)
            .map_err(|e| KcmError::Io(format!("Failed to write keystore: {}", e)))?;
        file.sync_all()
            .map_err(|e| KcmError::Io(format!("Failed to sync keystore: {}", e)))?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(path: P, master_key: EncryptionKey) -> Result<Self, KcmError> {
        let data = std::fs::read(path.as_ref())
            .map_err(|e| KcmError::Io(format!("Failed to read keystore: {}", e)))?;
        if data.len() < 8 {
            return Err(KcmError::Corrupted("Keystore file too small".to_string()));
        }
        if &data[..4] != KEYSTORE_MAGIC {
            return Err(KcmError::Corrupted("Invalid keystore magic".to_string()));
        }
        let count = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        let mut offset = 8;
        let mut keys = HashMap::new();
        for _ in 0..count {
            if offset + 4 > data.len() {
                return Err(KcmError::Corrupted("Truncated keystore entry".to_string()));
            }
            let name_len = u32::from_le_bytes([
                data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
            ]) as usize;
            offset += 4;
            if offset + name_len > data.len() {
                return Err(KcmError::Corrupted("Truncated key name".to_string()));
            }
            let name = String::from_utf8(data[offset..offset + name_len].to_vec())
                .map_err(|e| KcmError::Corrupted(format!("Invalid key name UTF-8: {}", e)))?;
            offset += name_len;
            if offset + 4 > data.len() {
                return Err(KcmError::Corrupted("Truncated key data length".to_string()));
            }
            let key_len = u32::from_le_bytes([
                data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
            ]) as usize;
            offset += 4;
            if offset + key_len > data.len() {
                return Err(KcmError::Corrupted("Truncated key data".to_string()));
            }
            let encrypted_key = data[offset..offset + key_len].to_vec();
            offset += key_len;
            if offset + 20 > data.len() {
                return Err(KcmError::Corrupted("Truncated key metadata".to_string()));
            }
            let created_at = i64::from_le_bytes([
                data[offset], data[offset+1], data[offset+2], data[offset+3],
                data[offset+4], data[offset+5], data[offset+6], data[offset+7],
            ]);
            offset += 8;
            let rotated_raw = i64::from_le_bytes([
                data[offset], data[offset+1], data[offset+2], data[offset+3],
                data[offset+4], data[offset+5], data[offset+6], data[offset+7],
            ]);
            let rotated_at = if rotated_raw == 0 { None } else { Some(rotated_raw) };
            offset += 8;
            let version = u32::from_le_bytes([
                data[offset], data[offset+1], data[offset+2], data[offset+3],
            ]);
            offset += 4;
            keys.insert(name, KeyEntry {
                name: name.clone(),
                encrypted_key,
                created_at,
                rotated_at,
                version,
            });
        }
        Ok(EncryptedKeyStore {
            keys: Arc::new(RwLock::new(keys)),
            master_key,
        })
    }
}

pub struct EncryptedStorage;

impl EncryptedStorage {
    pub fn encrypt(plaintext: &[u8], key: &EncryptionKey) -> Result<Vec<u8>, KcmError> {
        use aes_gcm::aead::rand_core::RngCore;
        use aes_gcm::{
            Aes256Gcm, Nonce,
            aead::{Aead, KeyInit, OsRng},
        };

        if plaintext.is_empty() {
            return Err(KcmError::InvalidArgument(
                "Plaintext cannot be empty".to_string(),
            ));
        }

        let cipher = Aes256Gcm::new_from_slice(&key.key)
            .map_err(|e| KcmError::Io(format!("Cipher init failed: {}", e)))?;

        let mut nonce_bytes = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| KcmError::Io(format!("Encryption failed: {}", e)))?;

        let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(encrypted: &[u8], key: &EncryptionKey) -> Result<Vec<u8>, KcmError> {
        use aes_gcm::{
            Aes256Gcm, Nonce,
            aead::{Aead, KeyInit},
        };

        if encrypted.len() < NONCE_SIZE {
            return Err(KcmError::Corrupted(format!(
                "Encrypted data too short: {} bytes (minimum {})",
                encrypted.len(),
                NONCE_SIZE
            )));
        }

        let cipher = Aes256Gcm::new_from_slice(&key.key)
            .map_err(|e| KcmError::Io(format!("Cipher init failed: {}", e)))?;

        let nonce = Nonce::from_slice(&encrypted[..NONCE_SIZE]);
        let ciphertext = &encrypted[NONCE_SIZE..];

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| KcmError::Corrupted(format!("Decryption failed: {}", e)))
    }

    pub fn encrypt_file<P: AsRef<Path>>(
        src: P,
        dst: P,
        key: &EncryptionKey,
    ) -> Result<(), KcmError> {
        let src_path = src.as_ref();
        let dst_path = dst.as_ref();

        if !src_path.exists() {
            return Err(KcmError::NotFound(format!(
                "Source file not found: {}",
                src_path.display()
            )));
        }

        let mut data = Vec::new();
        File::open(src_path)
            .and_then(|mut f| f.read_to_end(&mut data))
            .map_err(|e| KcmError::Io(format!("Failed to read source file: {}", e)))?;
        
        let encrypted = Self::encrypt(&data, key)?;
        
        let mut dst_file = File::create(dst_path)
            .map_err(|e| KcmError::Io(format!("Failed to create destination file: {}", e)))?;
        dst_file
            .write_all(&encrypted)
            .map_err(|e| KcmError::Io(format!("Failed to write encrypted data: {}", e)))?;
        
        Ok(())
    }

    pub fn decrypt_file<P: AsRef<Path>>(
        src: P,
        dst: P,
        key: &EncryptionKey,
    ) -> Result<(), KcmError> {
        let src_path = src.as_ref();
        let dst_path = dst.as_ref();

        if !src_path.exists() {
            return Err(KcmError::NotFound(format!(
                "Source file not found: {}",
                src_path.display()
            )));
        }

        let mut data = Vec::new();
        File::open(src_path)
            .and_then(|mut f| f.read_to_end(&mut data))
            .map_err(|e| KcmError::Io(format!("Failed to read encrypted file: {}", e)))?;
        
        let decrypted = Self::decrypt(&data, key)?;
        
        let mut dst_file = File::create(dst_path)
            .map_err(|e| KcmError::Io(format!("Failed to create destination file: {}", e)))?;
        dst_file
            .write_all(&decrypted)
            .map_err(|e| KcmError::Io(format!("Failed to write decrypted data: {}", e)))?;
        
        Ok(())
    }

    pub fn encrypt_in_place(buffer: &mut Vec<u8>, key: &EncryptionKey) -> Result<(), KcmError> {
        let plaintext = std::mem::take(buffer);
        let encrypted = Self::encrypt(&plaintext, key)?;
        *buffer = encrypted;
        Ok(())
    }

    pub fn decrypt_in_place(buffer: &mut Vec<u8>, key: &EncryptionKey) -> Result<(), KcmError> {
        let encrypted = std::mem::take(buffer);
        let decrypted = Self::decrypt(&encrypted, key)?;
        *buffer = decrypted;
        Ok(())
    }
}
