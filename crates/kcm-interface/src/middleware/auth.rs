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
            if let Ok(map) =
                serde_json::from_str::<HashMap<String, serde_json::Value>>(&json_str)
            {
                for (token, value) in map {
                    let user_id = value["user_id"]
                        .as_str()
                        .unwrap_or("unknown")
                        .to_string();
                    let roles: Vec<String> = value["roles"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    store.register(&token, &user_id, roles);
                }
                log::info!(
                    "Loaded {} API token(s) from KCM_AUTH_TOKENS",
                    store.len()
                );
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
        Ok(AuthContext {
            user_id: entry.user_id.clone(),
            permission: Permission::Read,
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

        let user_id = value["user_id"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        let roles: Vec<String> = value["roles"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let permission = value["permission"]
            .as_str()
            .and_then(|s| Permission::from_name(s).ok())
            .unwrap_or(Permission::Read);

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
