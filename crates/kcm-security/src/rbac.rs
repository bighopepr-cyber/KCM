use kcm_core::types::*;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Execute,
    Admin,
}

#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: HashSet<Permission>,
}

impl Role {
    pub fn new(name: &str) -> Self {
        Role {
            name: name.to_string(),
            permissions: HashSet::new(),
        }
    }
    pub fn with_permission(mut self, perm: Permission) -> Self {
        self.permissions.insert(perm);
        self
    }
    pub fn has_permission(&self, perm: Permission) -> bool {
        self.permissions.contains(&perm)
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub roles: HashSet<String>,
}

type AclEntry = Vec<(String, Permission)>;
type AclMap = HashMap<ContextID, AclEntry>;

pub struct ACLManager {
    users: Arc<RwLock<HashMap<String, User>>>,
    roles: Arc<RwLock<HashMap<String, Role>>>,
    context_acl: Arc<RwLock<AclMap>>,
}

impl ACLManager {
    pub fn new() -> Self {
        ACLManager {
            users: Arc::new(RwLock::new(HashMap::new())),
            roles: Arc::new(RwLock::new(HashMap::new())),
            context_acl: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn create_user(&self, user_id: &str) {
        self.users
            .write()
            .entry(user_id.to_string())
            .or_insert_with(|| User {
                user_id: user_id.to_string(),
                roles: HashSet::new(),
            });
    }

    pub fn create_role(&self, name: &str) {
        self.roles
            .write()
            .entry(name.to_string())
            .or_insert_with(|| Role::new(name));
    }

    pub fn add_permission_to_role(&self, role_name: &str, perm: Permission) {
        if let Some(role) = self.roles.write().get_mut(role_name) {
            role.permissions.insert(perm);
        }
    }

    pub fn assign_role(&self, user_id: &str, role_name: &str) {
        if let Some(user) = self.users.write().get_mut(user_id) {
            user.roles.insert(role_name.to_string());
        }
    }

    pub fn grant_context_permission(&self, user_id: &str, context: ContextID, perm: Permission) {
        self.context_acl
            .write()
            .entry(context)
            .or_default()
            .push((user_id.to_string(), perm));
    }

    pub fn check_permission(&self, user_id: &str, context: ContextID, perm: Permission) -> bool {
        if let Some(perms) = self.context_acl.read().get(&context) {
            if perms.iter().any(|(uid, p)| uid == user_id && *p == perm) {
                return true;
            }
        }
        if let Some(user) = self.users.read().get(user_id) {
            let roles = self.roles.read();
            for role_name in &user.roles {
                if let Some(role) = roles.get(role_name) {
                    if role.has_permission(perm) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl Default for ACLManager {
    fn default() -> Self {
        Self::new()
    }
}
