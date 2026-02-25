use crate::plugin_system::types::*;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::sync::RwLock as AsyncRwLock;

#[derive(Clone)]
pub struct PluginRegistry {
    plugins: Arc<AsyncRwLock<HashMap<String, Plugin>>>,
    plugin_dir: PathBuf,
}

impl PluginRegistry {
    pub fn new<P: AsRef<Path>>(plugin_dir: P) -> Self {
        Self {
            plugins: Arc::new(AsyncRwLock::new(HashMap::new())),
            plugin_dir: plugin_dir.as_ref().to_path_buf(),
        }
    }

    pub async fn discover_plugins(&self) -> Result<Vec<String>> {
        let mut discovered = Vec::new();

        if !self.plugin_dir.exists() {
            fs::create_dir_all(&self.plugin_dir)
                .context("Failed to create plugin directory")?;
            return Ok(discovered);
        }

        let entries = fs::read_dir(&self.plugin_dir)
            .context("Failed to read plugin directory")?;

        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join("plugin.json");
                if manifest_path.exists() {
                    if let Some(plugin_id) = self.load_plugin_from_path(&manifest_path).await? {
                        discovered.push(plugin_id);
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(plugin_id) = self.load_plugin_from_path(&path).await? {
                    discovered.push(plugin_id);
                }
            }
        }

        Ok(discovered)
    }

    async fn load_plugin_from_path(&self, manifest_path: &Path) -> Result<Option<String>> {
        use crate::plugin_system::manifest::ManifestParser;

        let manifest = ManifestParser::parse_from_file(manifest_path)
            .with_context(|| format!("Failed to parse manifest from {:?}", manifest_path))?;

        let plugin_dir = manifest_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();

        let plugin = Plugin {
            state: PluginState::Loaded,
            error: None,
            settings: HashMap::new(),
            installed_at: chrono::Utc::now(),
            last_activated: None,
            path: plugin_dir.to_string_lossy().to_string(),
            manifest,
        };

        let plugin_id = plugin.manifest.info.id.clone();
        self.register_plugin(plugin).await?;

        Ok(Some(plugin_id))
    }

    pub async fn register_plugin(&self, plugin: Plugin) -> Result<()> {
        let plugin_id = plugin.manifest.info.id.clone();
        let mut plugins = self.plugins.write().await;
        plugins.insert(plugin_id, plugin);
        Ok(())
    }

    pub async fn unregister_plugin(&self, plugin_id: &str) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        plugins.remove(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", plugin_id))?;
        Ok(())
    }

    pub async fn get_plugin(&self, plugin_id: &str) -> Result<Plugin> {
        let plugins = self.plugins.read().await;
        plugins
            .get(plugin_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", plugin_id))
    }

    pub async fn get_all_plugins(&self) -> Vec<Plugin> {
        let plugins = self.plugins.read().await;
        plugins.values().cloned().collect()
    }

    pub async fn get_plugins_by_type(&self, plugin_type: &PluginType) -> Vec<Plugin> {
        let plugins = self.plugins.read().await;
        plugins
            .values()
            .filter(|p| &p.manifest.info.plugin_type == plugin_type)
            .cloned()
            .collect()
    }

    pub async fn get_plugins_by_state(&self, state: &PluginState) -> Vec<Plugin> {
        let plugins = self.plugins.read().await;
        plugins
            .values()
            .filter(|p| &p.state == state)
            .cloned()
            .collect()
    }

    pub async fn update_plugin_state(&self, plugin_id: &str, new_state: PluginState, error: Option<String>) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.state = new_state.clone();
            plugin.error = error;

            if new_state == PluginState::Activated {
                plugin.last_activated = Some(chrono::Utc::now());
            }

            Ok(())
        } else {
            anyhow::bail!("Plugin not found: {}", plugin_id)
        }
    }

    pub async fn update_plugin_settings(&self, plugin_id: &str, settings: HashMap<String, serde_json::Value>) -> Result<()> {
        let mut plugins = self.plugins.write().await;
        if let Some(plugin) = plugins.get_mut(plugin_id) {
            plugin.settings = settings;
            Ok(())
        } else {
            anyhow::bail!("Plugin not found: {}", plugin_id)
        }
    }

    pub async fn plugin_exists(&self, plugin_id: &str) -> bool {
        let plugins = self.plugins.read().await;
        plugins.contains_key(plugin_id)
    }

    pub async fn get_plugin_count(&self) -> usize {
        let plugins = self.plugins.read().await;
        plugins.len()
    }

    pub async fn search_plugins(&self, query: &str) -> Vec<Plugin> {
        let query_lower = query.to_lowercase();
        let plugins = self.plugins.read().await;

        plugins
            .values()
            .filter(|p| {
                p.manifest.info.name.to_lowercase().contains(&query_lower)
                    || p.manifest.info.description.to_lowercase().contains(&query_lower)
                    || p.manifest.info.id.to_lowercase().contains(&query_lower)
                    || p.manifest.info.keywords.as_ref().map_or(false, |keywords| {
                        keywords.iter().any(|k| k.to_lowercase().contains(&query_lower))
                    })
            })
            .cloned()
            .collect()
    }

    pub async fn get_commands(&self) -> Vec<PluginCommand> {
        let plugins = self.plugins.read().await;

        plugins
            .values()
            .filter(|p| p.state == PluginState::Activated)
            .flat_map(|p| {
                p.manifest
                    .contributes
                    .iter()
                    .filter(|c| c.contribution_type == "command")
                    .map(|c| PluginCommand {
                        plugin_id: p.manifest.info.id.clone(),
                        command_id: c.id.clone(),
                        title: c.label.clone(),
                        description: c.description.clone(),
                        category: c.config.get("category").and_then(|v| v.as_str().map(String::from)),
                        icon: c.icon.clone(),
                        keybinding: c.config.get("keybinding").and_then(|v| v.as_str().map(String::from)),
                    })
            })
            .collect()
    }

    pub fn get_plugin_dir(&self) -> &Path {
        &self.plugin_dir
    }
}
