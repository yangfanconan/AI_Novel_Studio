use crate::plugin_system::types::*;
use crate::plugin_system::registry::PluginRegistry;
use crate::plugin_system::lifecycle::PluginLifecycleManager;
use crate::plugin_system::permissions::PermissionManager;
use crate::plugin_system::sandbox::{SandboxManager, create_sandbox_config_for_plugin};
use crate::plugin_system::script::{ScriptEngine, ScriptEngineManager, NoOpScriptEngine, ScriptContext};
use crate::plugin_system::javascript_engine::JavaScriptEngine;
use crate::plugin_system::python_engine::PythonEngine;
use crate::plugin_system::lua_engine::LuaEngine;
use crate::plugin_system::PermissionStatus;
use anyhow::{Context, Result};
use log;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

#[derive(Clone)]
pub struct PluginManager {
    registry: PluginRegistry,
    lifecycle_manager: Arc<PluginLifecycleManager>,
    permission_manager: PermissionManager,
    sandbox_manager: Arc<SandboxManager>,
    script_engine_manager: Arc<std::sync::Mutex<ScriptEngineManager>>,
    app_handle: Arc<AppHandle>,
}

impl PluginManager {
    pub fn new(plugin_dir: PathBuf, app_handle: AppHandle) -> Self {
        let registry = PluginRegistry::new(plugin_dir.clone());
        let permission_manager = PermissionManager::new();
        let lifecycle_manager = Arc::new(PluginLifecycleManager::new(
            registry.clone(),
            permission_manager.clone(),
        ));
        let sandbox_manager = Arc::new(SandboxManager::new());

        let js_engine: Arc<dyn ScriptEngine> = match JavaScriptEngine::new() {
            Ok(engine) => Arc::new(engine),
            Err(e) => {
                log::error!("Failed to create JavaScript engine: {}", e);
                Arc::new(NoOpScriptEngine)
            }
        };
        let python_engine: Arc<dyn ScriptEngine> = match PythonEngine::new() {
            Ok(engine) => Arc::new(engine),
            Err(e) => {
                log::error!("Failed to create Python engine: {}", e);
                Arc::new(NoOpScriptEngine)
            }
        };
        let lua_engine: Arc<dyn ScriptEngine> = match LuaEngine::new() {
            Ok(engine) => Arc::new(engine),
            Err(e) => {
                log::error!("Failed to create Lua engine: {}", e);
                Arc::new(NoOpScriptEngine)
            }
        };

        let script_engine_manager = Arc::new(std::sync::Mutex::new(ScriptEngineManager::new()));

        let mut manager = script_engine_manager.lock().unwrap();
        manager.register_engine(js_engine);
        manager.register_engine(python_engine);
        manager.register_engine(lua_engine);
        drop(manager);

        Self {
            registry,
            lifecycle_manager,
            permission_manager,
            sandbox_manager,
            script_engine_manager,
            app_handle: Arc::new(app_handle),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        let discovered = self.registry.discover_plugins().await
            .context("Failed to discover plugins")?;

        log::info!("Discovered {} plugins", discovered.len());

        for plugin_id in discovered {
            log::info!("Discovered plugin: {}", plugin_id);
        }

        Ok(())
    }

    pub async fn install_plugin(&self, plugin_path: PathBuf) -> Result<String> {
        use std::fs;

        let manifest_path = if plugin_path.is_dir() {
            plugin_path.join("plugin.json")
        } else if plugin_path.extension().map_or(false, |ext| ext == "json") {
            plugin_path.clone()
        } else {
            anyhow::bail!("Invalid plugin path. Expected a directory with plugin.json or a .json file");
        };

        if !manifest_path.exists() {
            anyhow::bail!("Plugin manifest not found at {:?}", manifest_path);
        }

        use crate::plugin_system::manifest::ManifestParser;
        let manifest = ManifestParser::parse_from_file(&manifest_path)?;
        let plugin_id = manifest.info.id.clone();

        let target_dir = self.registry.get_plugin_dir().join(&plugin_id);

        if target_dir.exists() {
            anyhow::bail!("Plugin {} is already installed", plugin_id);
        }

        fs::create_dir_all(&target_dir)
            .with_context(|| format!("Failed to create plugin directory {:?}", target_dir))?;

        if plugin_path.is_dir() {
            PluginManager::copy_dir_recursive(&plugin_path, &target_dir)?;
        } else {
            fs::copy(&manifest_path, target_dir.join("plugin.json"))
                .with_context(|| format!("Failed to copy manifest to {:?}", target_dir))?;
        }

        self.lifecycle_manager.load_plugin(&plugin_id).await?;

        Ok(plugin_id)
    }

    pub async fn uninstall_plugin(&self, plugin_id: &str) -> Result<()> {
        if self.registry.get_plugin(plugin_id).await?.state == PluginState::Activated {
            self.lifecycle_manager.deactivate_plugin(plugin_id).await?;
        }

        self.lifecycle_manager.unload_plugin(plugin_id).await?;

        let plugin_dir = self.registry.get_plugin_dir().join(plugin_id);
        if plugin_dir.exists() {
            std::fs::remove_dir_all(&plugin_dir)
                .with_context(|| format!("Failed to remove plugin directory {:?}", plugin_dir))?;
        }

        self.sandbox_manager.remove_sandbox(plugin_id).await?;
        self.permission_manager.clear_plugin_permissions(plugin_id);

        Ok(())
    }

    pub async fn activate_plugin(&self, plugin_id: &str) -> Result<()> {
        let plugin = self.registry.get_plugin(plugin_id).await?;

        if !self.sandbox_manager.get_sandbox(plugin_id).await.is_some() {
            let base_data_dir = self.get_plugin_data_dir();
            let sandbox_config = create_sandbox_config_for_plugin(&plugin, &base_data_dir);
            self.sandbox_manager.create_sandbox(plugin_id.to_string(), sandbox_config).await?;
        }

        self.lifecycle_manager.activate_plugin(plugin_id).await?;

        Ok(())
    }

    pub async fn deactivate_plugin(&self, plugin_id: &str) -> Result<()> {
        self.lifecycle_manager.deactivate_plugin(plugin_id).await?;
        Ok(())
    }

    pub async fn get_plugin(&self, plugin_id: &str) -> Result<Plugin> {
        self.registry.get_plugin(plugin_id).await
    }

    pub async fn get_all_plugins(&self) -> Vec<Plugin> {
        self.registry.get_all_plugins().await
    }

    pub async fn get_plugin_permissions(&self, plugin_id: &str) -> Result<Vec<PermissionStatus>> {
        let plugin = self.registry.get_plugin(plugin_id).await?;
        Ok(self.permission_manager.get_plugin_permissions(plugin_id, &plugin.manifest.permissions))
    }

    pub async fn grant_permission(&self, plugin_id: &str, permission_name: &str) -> Result<()> {
        self.permission_manager.grant_permission(plugin_id, permission_name);
        Ok(())
    }

    pub async fn revoke_permission(&self, plugin_id: &str, permission_name: &str) -> Result<()> {
        self.permission_manager.revoke_permission(plugin_id, permission_name);
        Ok(())
    }

    pub async fn get_plugin_settings(&self, plugin_id: &str) -> Result<serde_json::Value> {
        let plugin = self.registry.get_plugin(plugin_id).await?;
        Ok(serde_json::to_value(&plugin.settings)?)
    }

    pub async fn update_plugin_settings(&self, plugin_id: &str, settings: serde_json::Value) -> Result<()> {
        let settings_map: HashMap<String, serde_json::Value> = serde_json::from_value(settings)?;
        self.registry.update_plugin_settings(plugin_id, settings_map).await
    }

    pub async fn get_plugin_commands(&self) -> Vec<PluginCommand> {
        self.registry.get_commands().await
    }

    pub async fn search_plugins(&self, query: &str) -> Vec<Plugin> {
        self.registry.search_plugins(query).await
    }

    pub async fn get_plugin_resource_usage(&self, plugin_id: &str) -> Option<serde_json::Value> {
        self.sandbox_manager.get_resource_usage(plugin_id).await
            .map(|stats| serde_json::to_value(stats).ok())
            .flatten()
    }

    pub fn get_registry(&self) -> &PluginRegistry {
        self.lifecycle_manager.get_registry()
    }

    pub fn get_permission_manager(&self) -> &PermissionManager {
        self.lifecycle_manager.get_permission_manager()
    }

    pub fn get_app_handle(&self) -> &AppHandle {
        &self.app_handle
    }

    fn get_plugin_data_dir(&self) -> PathBuf {
        let handle = self.app_handle.as_ref();
        let app_data_dir = handle.path().app_data_dir()
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
        app_data_dir.join("plugins")
    }

    fn copy_dir_recursive(source: &PathBuf, destination: &PathBuf) -> Result<()> {
        use std::fs;

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let dest_path = destination.join(entry.file_name());

            if source_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
                PluginManager::copy_dir_recursive(&source_path, &dest_path)?;
            } else {
                fs::copy(&source_path, &dest_path)?;
            }
        }

        Ok(())
    }
}
