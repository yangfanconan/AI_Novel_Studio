use crate::plugin_system::types::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandOptions {
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorPosition {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorRange {
    pub start: EditorPosition,
    pub end: EditorPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSelection {
    pub text: String,
    pub range: EditorRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorDocument {
    pub id: String,
    pub uri: String,
    pub language: String,
    pub content: String,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterInfo {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub order: i32,
    pub word_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInfo {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub traits: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerationOptions {
    pub model: String,
    #[serde(default)]
    pub temperature: Option<f32>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub top_p: Option<f32>,
    #[serde(default)]
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerationResult {
    pub content: String,
    pub model: String,
    pub tokens_used: Option<u32>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub accelerator: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub submenu: Option<Vec<MenuItem>>,
    #[serde(default)]
    pub checked: Option<bool>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolbarItem {
    pub id: String,
    pub icon: String,
    pub tooltip: String,
    #[serde(default)]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelConfig {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub size: Option<f32>,
    #[serde(default)]
    pub closable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationOptions {
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptions {
    #[serde(default)]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReadOptions {
    #[serde(default)]
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWriteOptions {
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub create_parents: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRequestOptions {
    pub method: String,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub timeout: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    pub plugin_id: String,
    pub app_version: String,
    pub data_dir: String,
    pub config_dir: String,
}

pub trait CommandAPI {
    fn register_command(&self, id: String, handler: String) -> Result<()>;
    fn execute_command(&self, id: String, options: CommandOptions) -> Result<serde_json::Value>;
    fn unregister_command(&self, id: String) -> Result<()>;
}

pub trait EditorAPI {
    fn get_document(&self, document_id: String) -> Result<EditorDocument>;
    fn set_document_content(&self, document_id: String, content: String) -> Result<()>;
    fn get_selection(&self, document_id: String) -> Result<Option<EditorSelection>>;
    fn set_selection(&self, document_id: String, range: EditorRange) -> Result<()>;
    fn insert_text(&self, document_id: String, text: String, position: Option<EditorPosition>) -> Result<()>;
    fn replace_text(&self, document_id: String, range: EditorRange, text: String) -> Result<()>;
    fn get_language(&self, document_id: String) -> Result<String>;
    fn set_language(&self, document_id: String, language: String) -> Result<()>;
}

pub trait ProjectAPI {
    fn get_projects(&self) -> Result<Vec<ProjectInfo>>;
    fn get_project(&self, project_id: String) -> Result<ProjectInfo>;
    fn create_project(&self, name: String, description: Option<String>) -> Result<ProjectInfo>;
    fn update_project(&self, project_id: String, name: Option<String>, description: Option<String>) -> Result<ProjectInfo>;
    fn delete_project(&self, project_id: String) -> Result<()>;

    fn get_chapters(&self, project_id: String) -> Result<Vec<ChapterInfo>>;
    fn get_chapter(&self, chapter_id: String) -> Result<ChapterInfo>;
    fn create_chapter(&self, project_id: String, title: String, content: String, order: i32) -> Result<ChapterInfo>;
    fn update_chapter(&self, chapter_id: String, title: Option<String>, content: Option<String>) -> Result<ChapterInfo>;
    fn delete_chapter(&self, chapter_id: String) -> Result<()>;

    fn get_characters(&self, project_id: String) -> Result<Vec<CharacterInfo>>;
    fn create_character(&self, project_id: String, name: String, description: Option<String>) -> Result<CharacterInfo>;
    fn update_character(&self, character_id: String, name: Option<String>, description: Option<String>) -> Result<CharacterInfo>;
    fn delete_character(&self, character_id: String) -> Result<()>;
}

pub trait AIAPI {
    fn get_models(&self) -> Result<Vec<AIModelInfo>>;
    fn generate_text(&self, prompt: String, options: AIGenerationOptions) -> Result<AIGenerationResult>;
    fn generate_stream(&self, prompt: String, options: AIGenerationOptions) -> Result<String>;
    fn cancel_generation(&self, generation_id: String) -> Result<()>;
}

pub trait UIAPI {
    fn show_notification(&self, options: NotificationOptions) -> Result<()>;
    fn show_dialog(&self, title: String, message: String, options: Option<serde_json::Value>) -> Result<Option<String>>;
    fn show_confirm_dialog(&self, title: String, message: String) -> Result<bool>;
    fn show_input_dialog(&self, title: String, message: String, default: Option<String>) -> Result<Option<String>>;
    fn show_select_dialog(&self, title: String, message: String, options: Vec<String>) -> Result<Option<String>>;

    fn add_menu_item(&self, item: MenuItem) -> Result<()>;
    fn remove_menu_item(&self, id: String) -> Result<()>;
    fn add_toolbar_item(&self, item: ToolbarItem) -> Result<()>;
    fn remove_toolbar_item(&self, id: String) -> Result<()>;
    fn create_panel(&self, config: PanelConfig) -> Result<()>;
    fn close_panel(&self, panel_id: String) -> Result<()>;
    fn update_panel(&self, panel_id: String, content: String) -> Result<()>;

    fn open_url(&self, url: String) -> Result<()>;
    fn get_theme(&self) -> Result<String>;
    fn set_theme(&self, theme: String) -> Result<()>;
}

pub trait StorageAPI {
    fn get(&self, key: String, options: Option<StorageOptions>) -> Result<Option<serde_json::Value>>;
    fn set(&self, key: String, value: serde_json::Value, options: Option<StorageOptions>) -> Result<()>;
    fn delete(&self, key: String, options: Option<StorageOptions>) -> Result<()>;
    fn list(&self, prefix: Option<String>, options: Option<StorageOptions>) -> Result<Vec<String>>;
    fn clear(&self, options: Option<StorageOptions>) -> Result<()>;
}

pub trait FileSystemAPI {
    fn read_file(&self, path: String, options: Option<FileReadOptions>) -> Result<String>;
    fn write_file(&self, path: String, content: String, options: Option<FileWriteOptions>) -> Result<()>;
    fn delete_file(&self, path: String) -> Result<()>;
    fn copy_file(&self, source: String, destination: String) -> Result<()>;
    fn move_file(&self, source: String, destination: String) -> Result<()>;
    fn read_dir(&self, path: String) -> Result<Vec<String>>;
    fn create_dir(&self, path: String, recursive: bool) -> Result<()>;
    fn delete_dir(&self, path: String, recursive: bool) -> Result<()>;
    fn exists(&self, path: String) -> Result<bool>;
    fn get_file_info(&self, path: String) -> Result<FileInfo>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

pub trait NetworkAPI {
    fn request(&self, url: String, options: NetworkRequestOptions) -> Result<NetworkResponse>;
    fn get(&self, url: String, headers: Option<HashMap<String, String>>) -> Result<NetworkResponse>;
    fn post(&self, url: String, body: Option<serde_json::Value>, headers: Option<HashMap<String, String>>) -> Result<NetworkResponse>;
    fn put(&self, url: String, body: Option<serde_json::Value>, headers: Option<HashMap<String, String>>) -> Result<NetworkResponse>;
    fn delete(&self, url: String, headers: Option<HashMap<String, String>>) -> Result<NetworkResponse>;
    fn download_file(&self, url: String, destination: String) -> Result<String>;
}

pub struct PluginAPI {
    pub command: Box<dyn CommandAPI + Send + Sync>,
    pub editor: Box<dyn EditorAPI + Send + Sync>,
    pub project: Box<dyn ProjectAPI + Send + Sync>,
    pub ai: Box<dyn AIAPI + Send + Sync>,
    pub ui: Box<dyn UIAPI + Send + Sync>,
    pub storage: Box<dyn StorageAPI + Send + Sync>,
    pub filesystem: Box<dyn FileSystemAPI + Send + Sync>,
    pub network: Box<dyn NetworkAPI + Send + Sync>,
}

impl PluginAPI {
    pub fn new(
        command: Box<dyn CommandAPI + Send + Sync>,
        editor: Box<dyn EditorAPI + Send + Sync>,
        project: Box<dyn ProjectAPI + Send + Sync>,
        ai: Box<dyn AIAPI + Send + Sync>,
        ui: Box<dyn UIAPI + Send + Sync>,
        storage: Box<dyn StorageAPI + Send + Sync>,
        filesystem: Box<dyn FileSystemAPI + Send + Sync>,
        network: Box<dyn NetworkAPI + Send + Sync>,
    ) -> Self {
        Self {
            command,
            editor,
            project,
            ai,
            ui,
            storage,
            filesystem,
            network,
        }
    }
}
