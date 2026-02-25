use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginType {
    #[serde(rename = "editor_extension")]
    EditorExtension,
    #[serde(rename = "feature_module")]
    FeatureModule,
    #[serde(rename = "theme")]
    Theme,
    #[serde(rename = "language_pack")]
    LanguagePack,
    #[serde(rename = "ai_adapter")]
    AIAdapter,
    #[serde(rename = "import_export")]
    ImportExport,
    #[serde(rename = "utility")]
    Utility,
    #[serde(rename = "integration")]
    Integration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginCapability {
    Editor,
    Project,
    AI,
    FileSystem,
    Network,
    UI,
    Storage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PermissionRisk {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginState {
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "activated")]
    Activated,
    #[serde(rename = "deactivated")]
    Deactivated,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "unloaded")]
    Unloaded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPermission {
    pub name: String,
    pub description: String,
    pub risk: PermissionRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContribution {
    #[serde(rename = "type")]
    pub contribution_type: String,
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub enabled_by_default: bool,
    #[serde(flatten)]
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginScript {
    pub language: String,
    pub entry_point: String,
    #[serde(default)]
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginAuthor {
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub version: String,
    pub name: String,
    pub description: String,
    pub author: PluginAuthor,
    #[serde(rename = "pluginType")]
    pub plugin_type: PluginType,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(rename = "minAppVersion")]
    pub min_app_version: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub info: PluginInfo,
    #[serde(default)]
    pub permissions: Vec<PluginPermission>,
    #[serde(default)]
    pub capabilities: Vec<PluginCapability>,
    #[serde(default)]
    pub contributes: Vec<PluginContribution>,
    #[serde(default)]
    pub script: Option<PluginScript>,
    #[serde(default)]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "dependencies", default)]
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub manifest: PluginManifest,
    pub path: String,
    pub state: PluginState,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub last_activated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCommand {
    pub plugin_id: String,
    pub command_id: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub keybinding: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStorageItem {
    pub plugin_id: String,
    pub key: String,
    pub value: serde_json::Value,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEvent {
    pub id: Uuid,
    pub plugin_id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginError {
    pub plugin_id: String,
    pub error_type: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub stack_trace: Option<String>,
}
