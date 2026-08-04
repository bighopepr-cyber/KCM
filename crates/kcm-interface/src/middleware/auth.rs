/// Permission levels for RBAC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Permission {
    Reader = 0,
    Writer = 1,
    Delete = 2,
    Execute = 3,
    Admin = 4,
}

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
            permission: Permission::Reader,
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
        self.permission >= required
    }
}
