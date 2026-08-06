use kcm_core::types::*;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplicationStatus {
    Active,
    Lagging,
    Disconnected,
    Syncing,
}

#[derive(Debug, Clone)]
pub struct RegionNode {
    pub region_id: String,
    pub endpoint: String,
    pub status: ReplicationStatus,
    pub lag_ms: u64,
    pub last_sync: i64,
}

pub struct ReplicationManager {
    regions: Arc<RwLock<HashMap<String, RegionNode>>>,
    primary_region: Arc<RwLock<String>>,
}

impl ReplicationManager {
    pub fn new(primary_region: &str) -> Self {
        ReplicationManager {
            regions: Arc::new(RwLock::new(HashMap::new())),
            primary_region: Arc::new(RwLock::new(primary_region.to_string())),
        }
    }

    pub fn register_region(&self, node: RegionNode) {
        self.regions.write().insert(node.region_id.clone(), node);
    }

    pub fn remove_region(&self, region_id: &str) -> Result<(), KcmError> {
        self.regions
            .write()
            .remove(region_id)
            .ok_or_else(|| KcmError::NotFound(format!("Region not found: {}", region_id)))?;
        Ok(())
    }

    pub fn get_region(&self, region_id: &str) -> Option<RegionNode> {
        self.regions.read().get(region_id).cloned()
    }

    pub fn primary_region(&self) -> String {
        self.primary_region.read().clone()
    }

    pub fn set_primary(&self, region_id: &str) -> Result<(), KcmError> {
        if !self.regions.read().contains_key(region_id) {
            return Err(KcmError::NotFound(format!(
                "Region not found: {}",
                region_id
            )));
        }
        *self.primary_region.write() = region_id.to_string();
        Ok(())
    }

    pub fn all_regions(&self) -> Vec<RegionNode> {
        self.regions.read().values().cloned().collect()
    }

    pub fn healthy_regions(&self) -> Vec<RegionNode> {
        self.regions
            .read()
            .values()
            .filter(|r| r.status == ReplicationStatus::Active)
            .cloned()
            .collect()
    }

    pub fn region_count(&self) -> usize {
        self.regions.read().len()
    }

    pub fn update_status(
        &self,
        region_id: &str,
        status: ReplicationStatus,
    ) -> Result<(), KcmError> {
        let mut regions = self.regions.write();
        let node = regions
            .get_mut(region_id)
            .ok_or_else(|| KcmError::NotFound(format!("Region not found: {}", region_id)))?;
        node.status = status;
        Ok(())
    }

    pub fn update_lag(&self, region_id: &str, lag_ms: u64) -> Result<(), KcmError> {
        let mut regions = self.regions.write();
        let node = regions
            .get_mut(region_id)
            .ok_or_else(|| KcmError::NotFound(format!("Region not found: {}", region_id)))?;
        node.lag_ms = lag_ms;
        Ok(())
    }
}

impl Default for ReplicationManager {
    fn default() -> Self {
        Self::new("us-east-1")
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn test_register_region() {
        let mgr = ReplicationManager::new("us-east-1");
        mgr.register_region(RegionNode {
            region_id: "us-west-2".to_string(),
            endpoint: "https://us-west-2.kcm.example.com".to_string(),
            status: ReplicationStatus::Active,
            lag_ms: 0,
            last_sync: 0,
        });
        assert_eq!(mgr.region_count(), 1);
    }

    #[test]
    fn test_primary_region() {
        let mgr = ReplicationManager::new("us-east-1");
        assert_eq!(mgr.primary_region(), "us-east-1");
        mgr.register_region(RegionNode {
            region_id: "eu-west-1".to_string(),
            endpoint: "https://eu-west-1.kcm.example.com".to_string(),
            status: ReplicationStatus::Active,
            lag_ms: 50,
            last_sync: 0,
        });
        mgr.set_primary("eu-west-1").unwrap();
        assert_eq!(mgr.primary_region(), "eu-west-1");
    }

    #[test]
    fn test_healthy_regions() {
        let mgr = ReplicationManager::new("us-east-1");
        mgr.register_region(RegionNode {
            region_id: "us-west-2".to_string(),
            endpoint: "https://us-west-2.kcm.example.com".to_string(),
            status: ReplicationStatus::Active,
            lag_ms: 0,
            last_sync: 0,
        });
        mgr.register_region(RegionNode {
            region_id: "eu-west-1".to_string(),
            endpoint: "https://eu-west-1.kcm.example.com".to_string(),
            status: ReplicationStatus::Lagging,
            lag_ms: 500,
            last_sync: 0,
        });
        assert_eq!(mgr.healthy_regions().len(), 1);
    }

    #[test]
    fn test_remove_region() {
        let mgr = ReplicationManager::new("us-east-1");
        mgr.register_region(RegionNode {
            region_id: "us-west-2".to_string(),
            endpoint: "https://us-west-2.kcm.example.com".to_string(),
            status: ReplicationStatus::Active,
            lag_ms: 0,
            last_sync: 0,
        });
        assert_eq!(mgr.region_count(), 1);
        mgr.remove_region("us-west-2").unwrap();
        assert_eq!(mgr.region_count(), 0);
        assert!(mgr.remove_region("nonexistent").is_err());
    }
}
