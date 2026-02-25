use crate::plugin_system::types::*;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_bytes: usize,
    pub max_cpu_percent: u32,
    pub max_file_descriptors: u32,
    pub max_network_connections: u32,
    pub execution_timeout_seconds: u64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 256 * 1024 * 1024,
            max_cpu_percent: 80,
            max_file_descriptors: 100,
            max_network_connections: 10,
            execution_timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub allowed_paths: Vec<PathBuf>,
    pub denied_paths: Vec<PathBuf>,
    pub resource_limits: ResourceLimits,
    pub allow_network: bool,
    pub allowed_domains: Vec<String>,
    pub allow_filesystem: bool,
    pub allow_process: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            allowed_paths: vec![],
            denied_paths: vec![
                PathBuf::from("/etc"),
                PathBuf::from("/sys"),
                PathBuf::from("/proc"),
                PathBuf::from("/root"),
            ],
            resource_limits: ResourceLimits::default(),
            allow_network: false,
            allowed_domains: vec![],
            allow_filesystem: false,
            allow_process: false,
        }
    }
}

#[derive(Clone)]
pub struct PluginSandbox {
    plugin_id: String,
    config: SandboxConfig,
    semaphore: Arc<Semaphore>,
    resource_usage: Arc<RwLock<ResourceUsage>>,
}

#[derive(Debug, Clone, Default)]
struct ResourceUsage {
    memory_bytes: usize,
    file_descriptors: u32,
    network_connections: u32,
    execution_start: Option<std::time::Instant>,
}

impl PluginSandbox {
    pub fn new(plugin_id: String, config: SandboxConfig) -> Self {
        let max_concurrent = config.resource_limits.max_file_descriptors as usize;
        Self {
            plugin_id,
            config,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            resource_usage: Arc::new(RwLock::new(ResourceUsage::default())),
        }
    }

    pub async fn acquire(&self) -> Result<SandboxGuard<'_>> {
        let permit = self.semaphore.acquire().await.map_err(|_| {
            anyhow::anyhow!("Failed to acquire sandbox permit for plugin {}", self.plugin_id)
        })?;

        self.check_resource_limits()?;

        Ok(SandboxGuard::new(self, permit))
    }

    pub fn get_plugin_id(&self) -> &str {
        &self.plugin_id
    }

    pub fn get_config(&self) -> &SandboxConfig {
        &self.config
    }

    pub fn get_config_mut(&mut self) -> &mut SandboxConfig {
        &mut self.config
    }

    pub fn is_path_allowed(&self, path: &Path) -> bool {
        for denied in &self.config.denied_paths {
            if path.starts_with(denied) {
                return false;
            }
        }

        if self.config.allowed_paths.is_empty() {
            return true;
        }

        self.config.allowed_paths.iter().any(|allowed| path.starts_with(allowed))
    }

    pub fn is_domain_allowed(&self, domain: &str) -> bool {
        if !self.config.allow_network {
            return false;
        }

        if self.config.allowed_domains.is_empty() {
            return true;
        }

        self.config.allowed_domains.iter().any(|d| {
            domain.ends_with(d) || d == "*"
        })
    }

    fn check_resource_limits(&self) -> Result<()> {
        let limits = &self.config.resource_limits;
        let usage = self.resource_usage.blocking_read();

        if usage.memory_bytes > limits.max_memory_bytes {
            anyhow::bail!(
                "Plugin {} exceeded memory limit: {} > {} bytes",
                self.plugin_id,
                usage.memory_bytes,
                limits.max_memory_bytes
            );
        }

        if usage.network_connections > limits.max_network_connections {
            anyhow::bail!(
                "Plugin {} exceeded network connection limit: {} > {}",
                self.plugin_id,
                usage.network_connections,
                limits.max_network_connections
            );
        }

        if let Some(start) = usage.execution_start {
            let elapsed = start.elapsed().as_secs();
            if elapsed > limits.execution_timeout_seconds {
                anyhow::bail!(
                    "Plugin {} exceeded execution timeout: {}s > {}s",
                    self.plugin_id,
                    elapsed,
                    limits.execution_timeout_seconds
                );
            }
        }

        Ok(())
    }

    pub async fn get_resource_usage(&self) -> ResourceUsageStats {
        let usage = self.resource_usage.read().await;
        ResourceUsageStats {
            memory_bytes: usage.memory_bytes,
            memory_limit_bytes: self.config.resource_limits.max_memory_bytes,
            file_descriptors: usage.file_descriptors,
            file_descriptors_limit: self.config.resource_limits.max_file_descriptors,
            network_connections: usage.network_connections,
            network_connections_limit: self.config.resource_limits.max_network_connections,
            execution_duration_seconds: usage.execution_start
                .map(|s| s.elapsed().as_secs_f64())
                .unwrap_or(0.0),
            execution_timeout_seconds: self.config.resource_limits.execution_timeout_seconds,
        }
    }

    pub fn reset_resource_usage(&self) {
        let mut usage = self.resource_usage.blocking_write();
        *usage = ResourceUsage::default();
    }
}

pub struct SandboxGuard<'a> {
    sandbox: &'a PluginSandbox,
    _permit: tokio::sync::SemaphorePermit<'a>,
}

impl<'a> SandboxGuard<'a> {
    fn new(sandbox: &'a PluginSandbox, permit: tokio::sync::SemaphorePermit<'a>) -> Self {
        Self {
            sandbox,
            _permit: permit,
        }
    }

    pub fn validate_path(&self, path: &Path) -> Result<()> {
        if !self.sandbox.config.allow_filesystem {
            anyhow::bail!("Filesystem access is not allowed for plugin {}", self.sandbox.plugin_id);
        }

        if !self.sandbox.is_path_allowed(path) {
            anyhow::bail!("Access to path {:?} is not allowed for plugin {}", path, self.sandbox.plugin_id);
        }

        Ok(())
    }

    pub fn validate_domain(&self, domain: &str) -> Result<()> {
        if !self.sandbox.config.allow_network {
            anyhow::bail!("Network access is not allowed for plugin {}", self.sandbox.plugin_id);
        }

        if !self.sandbox.is_domain_allowed(domain) {
            anyhow::bail!("Access to domain {} is not allowed for plugin {}", domain, self.sandbox.plugin_id);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageStats {
    pub memory_bytes: usize,
    pub memory_limit_bytes: usize,
    pub file_descriptors: u32,
    pub file_descriptors_limit: u32,
    pub network_connections: u32,
    pub network_connections_limit: u32,
    pub execution_duration_seconds: f64,
    pub execution_timeout_seconds: u64,
}

pub struct SandboxManager {
    sandboxes: Arc<RwLock<HashMap<String, PluginSandbox>>>,
}

impl SandboxManager {
    pub fn new() -> Self {
        Self {
            sandboxes: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_sandbox(&self, plugin_id: String, config: SandboxConfig) -> Result<()> {
        let sandbox = PluginSandbox::new(plugin_id.clone(), config);
        let mut sandboxes = self.sandboxes.write().await;
        sandboxes.insert(plugin_id, sandbox);
        Ok(())
    }

    pub async fn get_sandbox(&self, plugin_id: &str) -> Option<PluginSandbox> {
        let sandboxes = self.sandboxes.read().await;
        sandboxes.get(plugin_id).cloned()
    }

    pub async fn remove_sandbox(&self, plugin_id: &str) -> Result<()> {
        let mut sandboxes = self.sandboxes.write().await;
        sandboxes.remove(plugin_id)
            .ok_or_else(|| anyhow::anyhow!("Sandbox not found for plugin {}", plugin_id))?;
        Ok(())
    }

    pub async fn get_resource_usage(&self, plugin_id: &str) -> Option<ResourceUsageStats> {
        let sandboxes = self.sandboxes.read().await;
        sandboxes.get(plugin_id)
            .map(|sandbox| {
                tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(sandbox.get_resource_usage())
                })
            })
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn create_sandbox_config_for_plugin(plugin: &Plugin, base_data_dir: &Path) -> SandboxConfig {
    let plugin_dir = PathBuf::from(&plugin.path);
    let plugin_data_dir = base_data_dir.join(&plugin.manifest.info.id);

    let allowed_paths = vec![
        plugin_dir.clone(),
        plugin_data_dir,
    ];

    let has_filesystem = plugin.manifest.capabilities.contains(&PluginCapability::FileSystem);
    let has_network = plugin.manifest.capabilities.contains(&PluginCapability::Network);

    SandboxConfig {
        allowed_paths,
        allow_filesystem: has_filesystem,
        allow_network: has_network,
        allowed_domains: vec![],
        denied_paths: vec![
            PathBuf::from("/etc"),
            PathBuf::from("/sys"),
            PathBuf::from("/proc"),
            PathBuf::from("/root"),
        ],
        resource_limits: ResourceLimits::default(),
        allow_process: false,
    }
}
