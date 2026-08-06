use kcm_core::types::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub trait SecretProvider: Send + Sync {
    fn get_secret(&self, key: &str) -> Result<String, KcmError>;
    fn set_secret(&self, key: &str, value: &str) -> Result<(), KcmError>;
    fn delete_secret(&self, key: &str) -> Result<(), KcmError>;
    fn list_secrets(&self) -> Result<Vec<String>, KcmError>;
}

pub struct VaultConfig {
    pub addr: String,
    pub token: String,
    pub mount_path: String,
}

pub struct HashiCorpVaultProvider {
    config: VaultConfig,
}

impl HashiCorpVaultProvider {
    pub fn new(config: VaultConfig) -> Self {
        HashiCorpVaultProvider { config }
    }
}

impl SecretProvider for HashiCorpVaultProvider {
    fn get_secret(&self, key: &str) -> Result<String, KcmError> {
        let url = format!(
            "{}/v1/{}/data/{}",
            self.config.addr, self.config.mount_path, key
        );
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(&url)
            .header("X-Vault-Token", &self.config.token)
            .send()
            .map_err(|e| KcmError::Io(format!("Vault request failed: {}", e)))?;
        let body: serde_json::Value = resp
            .json()
            .map_err(|e| KcmError::Io(format!("Vault response parse error: {}", e)))?;
        body["data"]["data"][key]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| KcmError::NotFound(format!("Secret not found: {}", key)))
    }

    fn set_secret(&self, key: &str, value: &str) -> Result<(), KcmError> {
        let url = format!(
            "{}/v1/{}/data/{}",
            self.config.addr, self.config.mount_path, key
        );
        let client = reqwest::blocking::Client::new();
        let mut data = HashMap::new();
        data.insert(key, value);
        let body = serde_json::json!({ "data": data });
        client
            .post(&url)
            .header("X-Vault-Token", &self.config.token)
            .json(&body)
            .send()
            .map_err(|e| KcmError::Io(format!("Vault write failed: {}", e)))?;
        Ok(())
    }

    fn delete_secret(&self, key: &str) -> Result<(), KcmError> {
        let url = format!(
            "{}/v1/{}/metadata/{}",
            self.config.addr, self.config.mount_path, key
        );
        let client = reqwest::blocking::Client::new();
        client
            .delete(&url)
            .header("X-Vault-Token", &self.config.token)
            .send()
            .map_err(|e| KcmError::Io(format!("Vault delete failed: {}", e)))?;
        Ok(())
    }

    fn list_secrets(&self) -> Result<Vec<String>, KcmError> {
        let url = format!(
            "{}/v1/{}/metadata/?list=true",
            self.config.addr, self.config.mount_path
        );
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(&url)
            .header("X-Vault-Token", &self.config.token)
            .send()
            .map_err(|e| KcmError::Io(format!("Vault list failed: {}", e)))?;
        let body: serde_json::Value = resp
            .json()
            .map_err(|e| KcmError::Io(format!("Vault list parse error: {}", e)))?;
        let keys = body["data"]["keys"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        Ok(keys)
    }
}

pub struct EnvSecretProvider;

impl SecretProvider for EnvSecretProvider {
    fn get_secret(&self, key: &str) -> Result<String, KcmError> {
        std::env::var(key).map_err(|_| KcmError::NotFound(format!("Env secret not found: {}", key)))
    }

    /// WARNING: `std::env::set_var` is not thread-safe per Rust documentation.
    /// This provider should only be used in single-threaded contexts or during
    /// initialization before spawning threads.
    fn set_secret(&self, key: &str, value: &str) -> Result<(), KcmError> {
        std::env::set_var(key, value);
        Ok(())
    }

    fn delete_secret(&self, key: &str) -> Result<(), KcmError> {
        std::env::remove_var(key);
        Ok(())
    }

    fn list_secrets(&self) -> Result<Vec<String>, KcmError> {
        Ok(Vec::new())
    }
}

pub struct SecretsManager {
    provider: Arc<dyn SecretProvider>,
    cache: RwLock<HashMap<String, String>>,
    #[allow(dead_code)]
    cache_ttl_secs: u64,
}

impl SecretsManager {
    pub fn new(provider: Arc<dyn SecretProvider>) -> Self {
        SecretsManager {
            provider,
            cache: RwLock::new(HashMap::new()),
            cache_ttl_secs: 300,
        }
    }

    pub fn with_cache_ttl(provider: Arc<dyn SecretProvider>, ttl_secs: u64) -> Self {
        SecretsManager {
            provider,
            cache: RwLock::new(HashMap::new()),
            cache_ttl_secs: ttl_secs,
        }
    }

    pub fn get_secret(&self, key: &str) -> Result<String, KcmError> {
        {
            let cache = self.cache.read();
            if let Some(val) = cache.get(key) {
                return Ok(val.clone());
            }
        }
        let value = self.provider.get_secret(key)?;
        self.cache.write().insert(key.to_string(), value.clone());
        Ok(value)
    }

    pub fn set_secret(&self, key: &str, value: &str) -> Result<(), KcmError> {
        self.provider.set_secret(key, value)?;
        self.cache
            .write()
            .insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn invalidate_cache(&self) {
        self.cache.write().clear();
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    struct MockSecretProvider {
        secrets: RwLock<HashMap<String, String>>,
    }

    impl MockSecretProvider {
        fn new() -> Self {
            MockSecretProvider {
                secrets: RwLock::new(HashMap::new()),
            }
        }
    }

    impl SecretProvider for MockSecretProvider {
        fn get_secret(&self, key: &str) -> Result<String, KcmError> {
            self.secrets
                .read()
                .get(key)
                .cloned()
                .ok_or_else(|| KcmError::NotFound(format!("Secret not found: {}", key)))
        }

        fn set_secret(&self, key: &str, value: &str) -> Result<(), KcmError> {
            self.secrets
                .write()
                .insert(key.to_string(), value.to_string());
            Ok(())
        }

        fn delete_secret(&self, key: &str) -> Result<(), KcmError> {
            self.secrets.write().remove(key);
            Ok(())
        }

        fn list_secrets(&self) -> Result<Vec<String>, KcmError> {
            Ok(self.secrets.read().keys().cloned().collect())
        }
    }

    #[test]
    fn test_secrets_manager_set_get() {
        let provider = Arc::new(MockSecretProvider::new());
        let manager = SecretsManager::new(provider);
        manager.set_secret("db-password", "secret123").unwrap();
        assert_eq!(manager.get_secret("db-password").unwrap(), "secret123");
    }

    #[test]
    fn test_secrets_manager_not_found() {
        let provider = Arc::new(MockSecretProvider::new());
        let manager = SecretsManager::new(provider);
        assert!(manager.get_secret("nonexistent").is_err());
    }

    #[test]
    fn test_secrets_manager_cache() {
        let provider = Arc::new(MockSecretProvider::new());
        let manager = SecretsManager::new(provider.clone());
        manager.set_secret("key1", "val1").unwrap();
        assert_eq!(manager.get_secret("key1").unwrap(), "val1");
        manager.invalidate_cache();
        assert_eq!(manager.get_secret("key1").unwrap(), "val1");
    }
}
