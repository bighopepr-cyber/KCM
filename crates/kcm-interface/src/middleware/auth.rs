pub use kcm_security::rbac::Permission;

/// Auth context for a request.
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

    pub fn has_permission(&self, required: Permission) -> bool {
        self.permission.level() >= required.level()
    }
}
