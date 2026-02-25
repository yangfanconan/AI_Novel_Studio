use crate::plugin_system::types::*;
use crate::plugin_system::registry::PluginRegistry;
use crate::plugin_system::permissions::PermissionManager;
use anyhow::{Context, Result};
use std::path::Path;
use tokio::sync::mpsc;
use log;

pub struct PluginLifecycleManager {
    registry: PluginRegistry,
    permission_manager: PermissionManager,
    event_sender: mpsc::UnboundedSender<PluginEvent>,
}

impl PluginLifecycleManager {
    pub fn new(registry: PluginRegistry, permission_manager: PermissionManager) -> Self {
        let (event_sender, _) = mpsc::unbounded_channel();
        Self {
            registry,
            permission_manager,
            event_sender,
        }
    }

    pub fn with_event_channel(registry: PluginRegistry, permission_manager: PermissionManager) -> (Self, mpsc::UnboundedReceiver<PluginEvent>) {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        (
            Self {
                registry,
                permission_manager,
                event_sender,
            },
            event_receiver,
        )
    }

    pub async fn load_plugin(&self, plugin_id: &str) -> Result<()> {
        if self.registry.plugin_exists(plugin_id).await {
            return Ok(());
        }

        let plugin_dir = self.registry.get_plugin_dir();
        let plugin_path = plugin_dir.join(plugin_id).join("plugin.json");

        if !plugin_path.exists() {
            anyhow::bail!("Plugin manifest not found: {:?}", plugin_path);
        }

        use crate::plugin_system::manifest::ManifestParser;
        let manifest = ManifestParser::parse_from_file(&plugin_path)?;

        let plugin = Plugin {
            manifest,
            path: plugin_dir.join(plugin_id).to_string_lossy().to_string(),
            state: PluginState::Loaded,
            error: None,
            settings: std::collections::HashMap::new(),
            installed_at: chrono::Utc::now(),
            last_activated: None,
        };

        self.registry.register_plugin(plugin).await?;
        self.emit_event(plugin_id, "plugin.loaded", serde_json::json!({})).await;

        Ok(())
    }

    pub async fn activate_plugin(&self, plugin_id: &str) -> Result<()> {
        let plugin = self.registry.get_plugin(plugin_id).await?;

        if plugin.state == PluginState::Activated {
            return Ok(());
        }

        let missing_permissions = self.check_missing_permissions(&plugin);

        if !missing_permissions.is_empty() {
            anyhow::bail!(
                "Plugin requires permissions that have not been granted: {:?}",
                missing_permissions
            );
        }

        self.execute_plugin_activation(&plugin).await?;

        self.registry
            .update_plugin_state(plugin_id, PluginState::Activated, None)
            .await?;

        self.emit_event(plugin_id, "plugin.activated", serde_json::json!({})).await;

        Ok(())
    }

    pub async fn activate_plugin_with_permissions(
        &self,
        plugin_id: &str,
        granted_permissions: Vec<String>,
    ) -> Result<()> {
        for perm in granted_permissions {
            self.permission_manager.grant_permission(plugin_id, &perm);
        }

        self.activate_plugin(plugin_id).await
    }

    pub async fn deactivate_plugin(&self, plugin_id: &str) -> Result<()> {
        let plugin = self.registry.get_plugin(plugin_id).await?;

        if plugin.state != PluginState::Activated {
            return Ok(());
        }

        self.execute_plugin_deactivation(&plugin).await?;

        self.registry
            .update_plugin_state(plugin_id, PluginState::Deactivated, None)
            .await?;

        self.emit_event(plugin_id, "plugin.deactivated", serde_json::json!({})).await;

        Ok(())
    }

    pub async fn unload_plugin(&self, plugin_id: &str) -> Result<()> {
        let plugin = self.registry.get_plugin(plugin_id).await?;

        if plugin.state == PluginState::Activated {
            self.deactivate_plugin(plugin_id).await?;
        }

        self.registry
            .update_plugin_state(plugin_id, PluginState::Unloaded, None)
            .await?;

        self.emit_event(plugin_id, "plugin.unloaded", serde_json::json!({})).await;

        self.registry.unregister_plugin(plugin_id).await?;
        self.permission_manager.clear_plugin_permissions(plugin_id);

        Ok(())
    }

    pub async fn reload_plugin(&self, plugin_id: &str) -> Result<()> {
        let was_active = {
            let plugin = self.registry.get_plugin(plugin_id).await?;
            plugin.state == PluginState::Activated
        };

        self.unload_plugin(plugin_id).await?;
        self.load_plugin(plugin_id).await?;

        if was_active {
            self.activate_plugin(plugin_id).await?;
        }

        Ok(())
    }

    fn check_missing_permissions(&self, plugin: &Plugin) -> Vec<String> {
        plugin
            .manifest
            .permissions
            .iter()
            .filter(|perm| !self.permission_manager.has_permission(&plugin.manifest.info.id, &perm.name))
            .map(|perm| perm.name.clone())
            .collect()
    }

    async fn execute_plugin_activation(&self, plugin: &Plugin) -> Result<()> {
        if let Some(script) = &plugin.manifest.script {
            match script.language.as_str() {
                "javascript" => {
                    self.execute_javascript_activation(plugin).await?;
                }
                "python" => {
                    self.execute_python_activation(plugin).await?;
                }
                "lua" => {
                    self.execute_lua_activation(plugin).await?;
                }
                _ => {
                    anyhow::bail!("Unsupported script language: {}", script.language);
                }
            }
        }
        Ok(())
    }

    async fn execute_plugin_deactivation(&self, plugin: &Plugin) -> Result<()> {
        if let Some(script) = &plugin.manifest.script {
            log::info!("Deactivating plugin {} (script: {})", plugin.manifest.info.id, script.language);
        }
        Ok(())
    }

    async fn execute_javascript_activation(&self, plugin: &Plugin) -> Result<()> {
        log::info!("Activating JavaScript plugin: {}", plugin.manifest.info.id);
        Ok(())
    }

    async fn execute_python_activation(&self, plugin: &Plugin) -> Result<()> {
        log::info!("Activating Python plugin: {}", plugin.manifest.info.id);
        Ok(())
    }

    async fn execute_lua_activation(&self, plugin: &Plugin) -> Result<()> {
        log::info!("Activating Lua plugin: {}", plugin.manifest.info.id);
        Ok(())
    }

    async fn emit_event(&self, plugin_id: &str, event_type: &str, payload: serde_json::Value) {
        let event = PluginEvent {
            id: uuid::Uuid::new_v4(),
            plugin_id: plugin_id.to_string(),
            event_type: event_type.to_string(),
            payload,
            timestamp: chrono::Utc::now(),
        };

        let _ = self.event_sender.send(event);
    }

    pub fn get_registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn get_permission_manager(&self) -> &PermissionManager {
        &self.permission_manager
    }
}
