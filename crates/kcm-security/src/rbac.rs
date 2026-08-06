use kcm_core::types::*;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub const MAX_USER_ID_LENGTH: usize = 256;
pub const MAX_ROLE_NAME_LENGTH: usize = 256;
pub const MAX_ROLES_PER_USER: usize = 32;
pub const MAX_PERMISSIONS_PER_ROLE: usize = 64;
pub const MAX_ACL_ENTRIES: usize = 10_000;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Permission {
    Read,
    Write,
    Delete,
    Execute,
    Admin,
}

impl Permission {
    pub fn name(&self) -> &'static str {
        match self {
            Permission::Read => "read",
            Permission::Write => "write",
            Permission::Delete => "delete",
            Permission::Execute => "execute",
            Permission::Admin => "admin",
        }
    }

    pub fn level(&self) -> u8 {
        match self {
            Permission::Read => 0,
            Permission::Write => 1,
            Permission::Delete => 2,
            Permission::Execute => 3,
            Permission::Admin => 4,
        }
    }

    pub fn from_name(name: &str) -> Result<Self, KcmError> {
        match name.to_lowercase().as_str() {
            "read" => Ok(Permission::Read),
            "write" => Ok(Permission::Write),
            "delete" => Ok(Permission::Delete),
            "execute" => Ok(Permission::Execute),
            "admin" => Ok(Permission::Admin),
            _ => Err(KcmError::InvalidArgument(format!(
                "Unknown permission: {}",
                name
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Role {
    pub name: String,
    pub permissions: HashSet<Permission>,
}

impl Role {
    pub fn new(name: &str) -> Result<Self, KcmError> {
        Self::validate_name(name)?;
        Ok(Role {
            name: name.to_string(),
            permissions: HashSet::new(),
        })
    }

    pub fn with_permission(mut self, perm: Permission) -> Result<Self, KcmError> {
        if self.permissions.len() >= MAX_PERMISSIONS_PER_ROLE {
            return Err(KcmError::InvalidArgument(format!(
                "Role cannot have more than {} permissions",
                MAX_PERMISSIONS_PER_ROLE
            )));
        }
        self.permissions.insert(perm);
        Ok(self)
    }

    pub fn has_permission(&self, perm: Permission) -> bool {
        self.permissions.contains(&perm)
    }

    pub fn has_permission_level(&self, perm: Permission) -> bool {
        let required_level = perm.level();
        self.permissions.iter().any(|p| p.level() >= required_level)
    }

    fn validate_name(name: &str) -> Result<(), KcmError> {
        if name.is_empty() {
            return Err(KcmError::InvalidArgument(
                "Role name cannot be empty".to_string(),
            ));
        }
        if name.len() > MAX_ROLE_NAME_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "Role name cannot exceed {} characters",
                MAX_ROLE_NAME_LENGTH
            )));
        }
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(KcmError::InvalidArgument(
                "Role name can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub roles: HashSet<String>,
}

impl User {
    pub fn new(user_id: &str) -> Result<Self, KcmError> {
        Self::validate_user_id(user_id)?;
        Ok(User {
            user_id: user_id.to_string(),
            roles: HashSet::new(),
        })
    }

    pub fn add_role(&mut self, role_name: &str) -> Result<(), KcmError> {
        if self.roles.len() >= MAX_ROLES_PER_USER {
            return Err(KcmError::InvalidArgument(format!(
                "User cannot have more than {} roles",
                MAX_ROLES_PER_USER
            )));
        }
        self.roles.insert(role_name.to_string());
        Ok(())
    }

    pub fn remove_role(&mut self, role_name: &str) -> bool {
        self.roles.remove(role_name)
    }

    pub fn has_role(&self, role_name: &str) -> bool {
        self.roles.contains(role_name)
    }

    fn validate_user_id(user_id: &str) -> Result<(), KcmError> {
        if user_id.is_empty() {
            return Err(KcmError::InvalidArgument(
                "User ID cannot be empty".to_string(),
            ));
        }
        if user_id.len() > MAX_USER_ID_LENGTH {
            return Err(KcmError::InvalidArgument(format!(
                "User ID cannot exceed {} characters",
                MAX_USER_ID_LENGTH
            )));
        }
        Ok(())
    }
}

type AclEntry = HashSet<(String, Permission)>;
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

    pub fn create_user(&self, user_id: &str) -> Result<(), KcmError> {
        User::validate_user_id(user_id)?;
        self.users
            .write()
            .entry(user_id.to_string())
            .or_insert_with(|| User {
                user_id: user_id.to_string(),
                roles: HashSet::new(),
            });
        Ok(())
    }

    pub fn create_role(&self, name: &str) -> Result<(), KcmError> {
        Role::validate_name(name)?;
        self.roles
            .write()
            .entry(name.to_string())
            .or_insert_with(|| Role {
                name: name.to_string(),
                permissions: HashSet::new(),
            });
        Ok(())
    }

    pub fn add_permission_to_role(
        &self,
        role_name: &str,
        perm: Permission,
    ) -> Result<(), KcmError> {
        let mut roles = self.roles.write();
        let role = roles
            .get_mut(role_name)
            .ok_or_else(|| KcmError::NotFound(format!("Role not found: {}", role_name)))?;

        if role.permissions.len() >= MAX_PERMISSIONS_PER_ROLE {
            return Err(KcmError::InvalidArgument(format!(
                "Role cannot have more than {} permissions",
                MAX_PERMISSIONS_PER_ROLE
            )));
        }

        role.permissions.insert(perm);
        Ok(())
    }

    pub fn remove_permission_from_role(
        &self,
        role_name: &str,
        perm: Permission,
    ) -> Result<(), KcmError> {
        let mut roles = self.roles.write();
        let role = roles
            .get_mut(role_name)
            .ok_or_else(|| KcmError::NotFound(format!("Role not found: {}", role_name)))?;
        role.permissions.remove(&perm);
        Ok(())
    }

    pub fn assign_role(&self, user_id: &str, role_name: &str) -> Result<(), KcmError> {
        User::validate_user_id(user_id)?;
        Role::validate_name(role_name)?;

        {
            let roles = self.roles.read();
            if !roles.contains_key(role_name) {
                return Err(KcmError::NotFound(format!("Role not found: {}", role_name)));
            }
        }

        let mut users = self.users.write();
        let user = users
            .get_mut(user_id)
            .ok_or_else(|| KcmError::NotFound(format!("User not found: {}", user_id)))?;

        if user.roles.len() >= MAX_ROLES_PER_USER {
            return Err(KcmError::InvalidArgument(format!(
                "User cannot have more than {} roles",
                MAX_ROLES_PER_USER
            )));
        }

        user.roles.insert(role_name.to_string());
        Ok(())
    }

    pub fn remove_role(&self, user_id: &str, role_name: &str) -> Result<(), KcmError> {
        let mut users = self.users.write();
        let user = users
            .get_mut(user_id)
            .ok_or_else(|| KcmError::NotFound(format!("User not found: {}", user_id)))?;
        user.roles.remove(role_name);
        Ok(())
    }

    pub fn grant_context_permission(
        &self,
        user_id: &str,
        context: ContextID,
        perm: Permission,
    ) -> Result<(), KcmError> {
        User::validate_user_id(user_id)?;

        let mut acl = self.context_acl.write();
        let entries = acl.entry(context).or_default();

        if entries.len() >= MAX_ACL_ENTRIES {
            return Err(KcmError::InvalidArgument(format!(
                "ACL for context cannot have more than {} entries",
                MAX_ACL_ENTRIES
            )));
        }

        entries.insert((user_id.to_string(), perm));
        Ok(())
    }

    pub fn revoke_context_permission(
        &self,
        user_id: &str,
        context: ContextID,
        perm: Permission,
    ) -> Result<(), KcmError> {
        let mut acl = self.context_acl.write();
        if let Some(entries) = acl.get_mut(&context) {
            entries.retain(|(uid, p)| !(uid == user_id && *p == perm));
        }
        Ok(())
    }

    pub fn check_permission(&self, user_id: &str, context: ContextID, perm: Permission) -> bool {
        if let Some(perms) = self.context_acl.read().get(&context) {
            if perms.contains(&(user_id.to_string(), perm)) {
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

    pub fn check_permission_level(
        &self,
        user_id: &str,
        context: ContextID,
        perm: Permission,
    ) -> bool {
        if let Some(perms) = self.context_acl.read().get(&context) {
            for (uid, p) in perms {
                if uid == user_id && p.level() >= perm.level() {
                    return true;
                }
            }
        }
        if let Some(user) = self.users.read().get(user_id) {
            let roles = self.roles.read();
            for role_name in &user.roles {
                if let Some(role) = roles.get(role_name) {
                    if role.has_permission_level(perm) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn user_exists(&self, user_id: &str) -> bool {
        self.users.read().contains_key(user_id)
    }

    pub fn role_exists(&self, role_name: &str) -> bool {
        self.roles.read().contains_key(role_name)
    }

    pub fn get_user_roles(&self, user_id: &str) -> Result<Vec<String>, KcmError> {
        let users = self.users.read();
        let user = users
            .get(user_id)
            .ok_or_else(|| KcmError::NotFound(format!("User not found: {}", user_id)))?;
        Ok(user.roles.iter().cloned().collect())
    }

    pub fn get_role_permissions(&self, role_name: &str) -> Result<Vec<Permission>, KcmError> {
        let roles = self.roles.read();
        let role = roles
            .get(role_name)
            .ok_or_else(|| KcmError::NotFound(format!("Role not found: {}", role_name)))?;
        Ok(role.permissions.iter().cloned().collect())
    }

    pub fn delete_user(&self, user_id: &str) -> Result<(), KcmError> {
        let mut users = self.users.write();
        users
            .remove(user_id)
            .ok_or_else(|| KcmError::NotFound(format!("User not found: {}", user_id)))?;
        drop(users);

        let mut context_acl = self.context_acl.write();
        for perms in context_acl.values_mut() {
            perms.retain(|(uid, _)| uid != user_id);
        }
        Ok(())
    }

    pub fn delete_role(&self, role_name: &str) -> Result<(), KcmError> {
        let mut roles = self.roles.write();
        roles
            .remove(role_name)
            .ok_or_else(|| KcmError::NotFound(format!("Role not found: {}", role_name)))?;
        drop(roles);

        let mut users = self.users.write();
        for user in users.values_mut() {
            user.roles.remove(role_name);
        }
        Ok(())
    }
}

impl Default for ACLManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_and_role() {
        let acl = ACLManager::new();
        acl.create_user("alice").unwrap();
        acl.create_role("admin").unwrap();
        acl.add_permission_to_role("admin", Permission::Admin)
            .unwrap();
        acl.assign_role("alice", "admin").unwrap();
        assert!(acl.check_permission("alice", ContextID(1), Permission::Admin));
    }

    #[test]
    fn test_deny_unauthorized() {
        let acl = ACLManager::new();
        acl.create_user("bob").unwrap();
        assert!(!acl.check_permission("bob", ContextID(1), Permission::Write));
    }

    #[test]
    fn test_context_permission() {
        let acl = ACLManager::new();
        acl.create_user("carol").unwrap();
        acl.grant_context_permission("carol", ContextID(5), Permission::Read)
            .unwrap();
        assert!(acl.check_permission("carol", ContextID(5), Permission::Read));
        assert!(!acl.check_permission("carol", ContextID(6), Permission::Read));
    }

    #[test]
    fn test_invalid_user_id() {
        let acl = ACLManager::new();
        assert!(acl.create_user("").is_err());
        assert!(acl.create_user(&"a".repeat(257)).is_err());
    }

    #[test]
    fn test_invalid_role_name() {
        let acl = ACLManager::new();
        assert!(acl.create_role("").is_err());
        assert!(acl.create_role(&"a".repeat(257)).is_err());
        assert!(acl.create_role("invalid role!").is_err());
    }

    #[test]
    fn test_permission_level() {
        assert!(Permission::Admin.level() > Permission::Read.level());
        assert!(Permission::Write.level() > Permission::Read.level());
    }

    #[test]
    fn test_permission_from_name() {
        assert!(Permission::from_name("read").is_ok());
        assert!(Permission::from_name("invalid").is_err());
    }

    #[test]
    fn test_user_role_limit() {
        let acl = ACLManager::new();
        acl.create_user("user1").unwrap();
        for i in 0..32 {
            let role_name = format!("role_{}", i);
            acl.create_role(&role_name).unwrap();
            acl.assign_role("user1", &role_name).unwrap();
        }
        assert!(acl.assign_role("user1", "role_32").is_err());
    }

    #[test]
    fn test_role_permission_limit() {
        let acl = ACLManager::new();
        acl.create_role("limited").unwrap();
        acl.add_permission_to_role("limited", Permission::Read)
            .unwrap();
        acl.add_permission_to_role("limited", Permission::Write)
            .unwrap();
        acl.add_permission_to_role("limited", Permission::Delete)
            .unwrap();
        acl.add_permission_to_role("limited", Permission::Execute)
            .unwrap();
        acl.add_permission_to_role("limited", Permission::Admin)
            .unwrap();
        let perms = acl.get_role_permissions("limited").unwrap();
        assert_eq!(perms.len(), 5);
    }

    #[test]
    fn test_revoke_context_permission() {
        let acl = ACLManager::new();
        acl.create_user("dave").unwrap();
        acl.grant_context_permission("dave", ContextID(1), Permission::Read)
            .unwrap();
        assert!(acl.check_permission("dave", ContextID(1), Permission::Read));
        acl.revoke_context_permission("dave", ContextID(1), Permission::Read)
            .unwrap();
        assert!(!acl.check_permission("dave", ContextID(1), Permission::Read));
    }

    #[test]
    fn test_delete_user() {
        let acl = ACLManager::new();
        acl.create_user("eve").unwrap();
        assert!(acl.user_exists("eve"));
        acl.delete_user("eve").unwrap();
        assert!(!acl.user_exists("eve"));
    }

    #[test]
    fn test_delete_role() {
        let acl = ACLManager::new();
        acl.create_role("temp_role").unwrap();
        assert!(acl.role_exists("temp_role"));
        acl.delete_role("temp_role").unwrap();
        assert!(!acl.role_exists("temp_role"));
    }
}
