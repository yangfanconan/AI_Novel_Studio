use crate::plugin_system::types::*;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct PermissionManager {
    granted_permissions: Arc<RwLock<HashSet<String>>>,
    denied_permissions: Arc<RwLock<HashSet<String>>>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            granted_permissions: Arc::new(RwLock::new(HashSet::new())),
            denied_permissions: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    pub fn has_permission(&self, plugin_id: &str, permission_name: &str) -> bool {
        let key = format!("{}:{}", plugin_id, permission_name);

        let denied = self.denied_permissions.read().unwrap();
        if denied.contains(&key) {
            return false;
        }

        let granted = self.granted_permissions.read().unwrap();
        granted.contains(&key)
    }

    pub fn grant_permission(&self, plugin_id: &str, permission_name: &str) {
        let key = format!("{}:{}", plugin_id, permission_name);
        let mut granted = self.granted_permissions.write().unwrap();
        granted.insert(key);
    }

    pub fn deny_permission(&self, plugin_id: &str, permission_name: &str) {
        let key = format!("{}:{}", plugin_id, permission_name);
        let mut denied = self.denied_permissions.write().unwrap();
        denied.insert(key);
    }

    pub fn revoke_permission(&self, plugin_id: &str, permission_name: &str) {
        let key = format!("{}:{}", plugin_id, permission_name);
        {
            let mut granted = self.granted_permissions.write().unwrap();
            granted.remove(&key);
        }
        {
            let mut denied = self.denied_permissions.write().unwrap();
            denied.remove(&key);
        }
    }

    pub fn get_plugin_permissions(&self, plugin_id: &str, permissions: &[PluginPermission]) -> Vec<PermissionStatus> {
        permissions
            .iter()
            .map(|perm| PermissionStatus {
                name: perm.name.clone(),
                granted: self.has_permission(plugin_id, &perm.name),
                risk: perm.risk.clone(),
            })
            .collect()
    }

    pub fn grant_all_permissions(&self, plugin_id: &str, permissions: &[PluginPermission]) {
        for perm in permissions {
            self.grant_permission(plugin_id, &perm.name);
        }
    }

    pub fn clear_plugin_permissions(&self, plugin_id: &str) {
        let prefix = format!("{}:", plugin_id);

        {
            let mut granted = self.granted_permissions.write().unwrap();
            granted.retain(|k| !k.starts_with(&prefix));
        }

        {
            let mut denied = self.denied_permissions.write().unwrap();
            denied.retain(|k| !k.starts_with(&prefix));
        }
    }

    pub fn check_capability(&self, plugin_id: &str, capability: &PluginCapability) -> bool {
        let perm_name = match capability {
            PluginCapability::Editor => "editor",
            PluginCapability::Project => "project",
            PluginCapability::AI => "ai",
            PluginCapability::FileSystem => "filesystem",
            PluginCapability::Network => "network",
            PluginCapability::UI => "ui",
            PluginCapability::Storage => "storage",
        };

        self.has_permission(plugin_id, perm_name)
    }

    pub fn get_required_permission_dialog_info(&self, plugin: &Plugin) -> Vec<PermissionDialogInfo> {
        plugin
            .manifest
            .permissions
            .iter()
            .filter(|perm| !self.has_permission(&plugin.manifest.info.id, &perm.name))
            .map(|perm| PermissionDialogInfo {
                plugin_id: plugin.manifest.info.id.clone(),
                plugin_name: plugin.manifest.info.name.clone(),
                permission_name: perm.name.clone(),
                permission_description: perm.description.clone(),
                risk: perm.risk.clone(),
            })
            .collect()
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct PermissionStatus {
    pub name: String,
    pub granted: bool,
    pub risk: PermissionRisk,
}

#[derive(Debug, Clone)]
pub struct PermissionDialogInfo {
    pub plugin_id: String,
    pub plugin_name: String,
    pub permission_name: String,
    pub permission_description: String,
    pub risk: PermissionRisk,
}

pub fn get_permission_risk_level(risk: &PermissionRisk) -> &'static str {
    match risk {
        PermissionRisk::Low => "低",
        PermissionRisk::Medium => "中",
        PermissionRisk::High => "高",
    }
}

pub fn get_permission_risk_color(risk: &PermissionRisk) -> &'static str {
    match risk {
        PermissionRisk::Low => "text-green-600",
        PermissionRisk::Medium => "text-yellow-600",
        PermissionRisk::High => "text-red-600",
    }
}

pub fn should_show_permission_dialog(risk: &PermissionRisk) -> bool {
    matches!(risk, PermissionRisk::Medium | PermissionRisk::High)
}
