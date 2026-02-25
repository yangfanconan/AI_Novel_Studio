use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub provider_type: ProviderType,
    pub credentials: HashMap<String, String>,
    pub sync_interval_seconds: u64,
    pub auto_sync: bool,
    pub conflict_resolution: ConflictResolutionStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub synced_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub file_path: String,
    pub conflict_type: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    PreferLocal,
    PreferRemote,
    AskUser,
    Merge,
    TimestampBased,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProviderType {
    Dropbox,
    GoogleDrive,
    OneDrive,
    iCloud,
    WebDAV,
    Custom,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            provider_type: ProviderType::Dropbox,
            credentials: HashMap::new(),
            sync_interval_seconds: 300,
            auto_sync: true,
            conflict_resolution: ConflictResolutionStrategy::AskUser,
        }
    }
}
