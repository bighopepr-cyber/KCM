use std::collections::HashMap;

pub use kcm_security::rbac::Permission;

#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    InvalidToken,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingCredentials => write!(f, "Missing authentication credentials"),
            AuthError::InvalidToken => write!(f, "Invalid or unknown API key"),
        }
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug, Clone)]
pub struct CredentialEntry {
    pub user_id: String,
    pub roles: Vec<String>,
}

impl CredentialEntry {
    fn from_json_value(value: &serde_json::Value) -> Result<Self, AuthError> {
        let object = value.as_object().ok_or(AuthError::InvalidToken)?;
        let user_id = object
            .get("user_id")
            .and_then(|value| value.as_str())
            .filter(|value| !value.trim().is_empty())
            .ok_or(AuthError::InvalidToken)?
            .to_string();

        let roles = match object.get("roles") {
            Some(roles_value) => roles_value
                .as_array()
                .ok_or(AuthError::InvalidToken)?
                .iter()
                .map(|role| {
                    role.as_str()
                        .filter(|value| !value.trim().is_empty())
                        .map(String::from)
                        .ok_or(AuthError::InvalidToken)
                })
                .collect::<Result<Vec<String>, AuthError>>()?,
            None => Vec::new(),
        };

        Ok(Self { user_id, roles })
    }
}

pub struct CredentialStore {
    entries: HashMap<String, CredentialEntry>,
}

impl CredentialStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register(&mut self, token: &str, user_id: &str, roles: Vec<String>) {
        self.entries.insert(
            token.to_string(),
            CredentialEntry {
                user_id: user_id.to_string(),
                roles,
            },
        );
    }

    pub fn lookup(&self, token: &str) -> Option<&CredentialEntry> {
        self.entries.get(token)
    }

    pub fn entries(&self) -> &HashMap<String, CredentialEntry> {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Load credentials from `KCM_AUTH_TOKENS` environment variable.
    ///
    /// Format: `{"token_value": {"user_id": "name", "roles": ["role1"]}}`
    /// Roles should correspond to ACLManager role names: "reader", "writer", "admin".
    pub fn from_env() -> Self {
        let mut store = Self::new();
        if let Ok(json_str) = std::env::var("KCM_AUTH_TOKENS") {
            if let Ok(map) = serde_json::from_str::<HashMap<String, serde_json::Value>>(&json_str) {
                for (token, value) in map {
                    match CredentialEntry::from_json_value(&value) {
                        Ok(entry) => {
                            store.register(&token, &entry.user_id, entry.roles);
                        }
                        Err(err) => {
                            log::warn!(
                                "Skipping invalid credential entry for token {}: {}",
                                token,
                                err
                            );
                        }
                    }
                }
                log::info!("Loaded {} API token(s) from KCM_AUTH_TOKENS", store.len());
            } else {
                log::error!("Failed to parse KCM_AUTH_TOKENS: invalid JSON");
            }
        }
        store
    }
}

impl Default for CredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AuthContext {
    pub user_id: String,
    pub permission: Permission,
    pub roles: Vec<String>,
}

impl AuthContext {
    fn permission_from_roles(roles: &[String], fallback: Permission) -> Permission {
        roles.iter().fold(fallback, |current, role| {
            let normalized = role.trim().to_lowercase();
            let mapped = match normalized.as_str() {
                "admin" => Permission::Admin,
                "writer" | "write" => Permission::Write,
                "reader" | "read" => Permission::Read,
                "delete" => Permission::Delete,
                "execute" => Permission::Execute,
                _ => return current,
            };

            if mapped.level() > current.level() {
                mapped
            } else {
                current
            }
        })
    }

    pub fn anonymous() -> Self {
        AuthContext {
            user_id: "anonymous".to_string(),
            permission: Permission::Read,
            roles: vec!["reader".to_string()],
        }
    }

    pub fn admin() -> Self {
        AuthContext {
            user_id: "admin".to_string(),
            permission: Permission::Admin,
            roles: vec!["admin".to_string()],
        }
    }

    pub fn new(user_id: String, permission: Permission, roles: Vec<String>) -> Self {
        Self {
            user_id,
            permission,
            roles,
        }
    }

    pub fn validate_token(token: &str, store: &CredentialStore) -> Result<Self, AuthError> {
        let entry = store.lookup(token).ok_or(AuthError::InvalidToken)?;
        let permission = Self::permission_from_roles(&entry.roles, Permission::Read);
        Ok(AuthContext {
            user_id: entry.user_id.clone(),
            permission,
            roles: entry.roles.clone(),
        })
    }

    /// Validate an API token by looking it up via a `SecretProvider`.
    ///
    /// The provider should map key `kcm_api_token_<token>` to a JSON value:
    /// `{"user_id": "...", "roles": ["..."], "permission": "read"}`.
    pub fn validate_token_from_secrets(
        token: &str,
        provider: &dyn kcm_security::secrets::SecretProvider,
    ) -> Result<Self, AuthError> {
        let secret_key = format!("kcm_api_token_{}", token);
        let json_str = provider
            .get_secret(&secret_key)
            .map_err(|_| AuthError::InvalidToken)?;

        let value: serde_json::Value =
            serde_json::from_str(&json_str).map_err(|_| AuthError::InvalidToken)?;

        let object = value.as_object().ok_or(AuthError::InvalidToken)?;
        let user_id = object
            .get("user_id")
            .and_then(|value| value.as_str())
            .filter(|value| !value.trim().is_empty())
            .ok_or(AuthError::InvalidToken)?
            .to_string();

        let roles = match object.get("roles") {
            Some(roles_value) => roles_value
                .as_array()
                .ok_or(AuthError::InvalidToken)?
                .iter()
                .map(|role| {
                    role.as_str()
                        .filter(|value| !value.trim().is_empty())
                        .map(String::from)
                        .ok_or(AuthError::InvalidToken)
                })
                .collect::<Result<Vec<String>, AuthError>>()?,
            None => Vec::new(),
        };

        let permission = match object.get("permission") {
            Some(permission_value) => {
                let name = permission_value
                    .as_str()
                    .ok_or(AuthError::InvalidToken)?
                    .trim();
                Permission::from_name(name).map_err(|_| AuthError::InvalidToken)?
            }
            None => Permission::Read,
        };

        Ok(AuthContext {
            user_id,
            permission,
            roles,
        })
    }

    pub fn has_permission(&self, required: Permission) -> bool {
        self.permission.level() >= required.level()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kcm_security::secrets::SecretProvider;

    struct StubProvider {
        secret: String,
    }

    impl SecretProvider for StubProvider {
        fn get_secret(&self, _key: &str) -> Result<String, kcm_core::types::KcmError> {
            Ok(self.secret.clone())
        }

        fn set_secret(&self, _key: &str, _value: &str) -> Result<(), kcm_core::types::KcmError> {
            Ok(())
        }

        fn delete_secret(&self, _key: &str) -> Result<(), kcm_core::types::KcmError> {
            Ok(())
        }

        fn list_secrets(&self) -> Result<Vec<String>, kcm_core::types::KcmError> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn validate_token_from_secrets_rejects_missing_user_id() {
        let provider = StubProvider {
            secret: r#"{"roles": ["reader"]}"#.to_string(),
        };

        let result = AuthContext::validate_token_from_secrets("abc", &provider);

        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[test]
    fn validate_token_from_secrets_rejects_invalid_permission() {
        let provider = StubProvider {
            secret: r#"{"user_id": "alice", "permission": "superuser"}"#.to_string(),
        };

        let result = AuthContext::validate_token_from_secrets("abc", &provider);

        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[test]
    fn validate_token_assigns_admin_permission_for_admin_roles() {
        let mut store = CredentialStore::new();
        store.register("admin-token", "alice", vec!["admin".to_string()]);

        let context = AuthContext::validate_token("admin-token", &store).unwrap();

        assert_eq!(context.permission, Permission::Admin);
        assert!(context.has_permission(Permission::Read));
        assert!(context.has_permission(Permission::Admin));
    }

    #[test]
    fn validate_token_assigns_write_permission_for_writer_roles() {
        let mut store = CredentialStore::new();
        store.register("writer-token", "bob", vec!["writer".to_string()]);

        let context = AuthContext::validate_token("writer-token", &store).unwrap();

        assert_eq!(context.permission, Permission::Write);
        assert!(context.has_permission(Permission::Read));
        assert!(context.has_permission(Permission::Write));
    }

    #[test]
    fn credential_entry_from_json_rejects_missing_user_id() {
        let value = serde_json::json!({ "roles": ["reader"] });

        let result = CredentialEntry::from_json_value(&value);

        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }

    #[test]
    fn credential_entry_from_json_parses_valid_payload() {
        let value = serde_json::json!({ "user_id": "alice", "roles": ["reader", "writer"] });

        let entry = CredentialEntry::from_json_value(&value).unwrap();

        assert_eq!(entry.user_id, "alice");
        assert_eq!(entry.roles, vec!["reader", "writer"]);
    }
}
