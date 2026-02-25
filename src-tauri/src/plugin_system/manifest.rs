use crate::plugin_system::types::*;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ManifestParser;

impl ManifestParser {
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> Result<PluginManifest> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read plugin manifest file")?;

        Self::parse_from_str(&content)
    }

    pub fn parse_from_str(content: &str) -> Result<PluginManifest> {
        let manifest: PluginManifest = serde_json::from_str(content)
            .context("Failed to parse plugin manifest JSON")?;

        Self::validate(&manifest)?;
        Ok(manifest)
    }

    fn validate(manifest: &PluginManifest) -> Result<()> {
        Self::validate_info(&manifest.info)?;
        Self::validate_permissions(&manifest.permissions)?;
        Self::validate_contributes(&manifest.contributes)?;
        Self::validate_script(&manifest.script)?;
        Ok(())
    }

    fn validate_info(info: &PluginInfo) -> Result<()> {
        if info.id.is_empty() {
            anyhow::bail!("Plugin ID cannot be empty");
        }

        if !info.id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
            anyhow::bail!("Plugin ID contains invalid characters");
        }

        if info.name.is_empty() {
            anyhow::bail!("Plugin name cannot be empty");
        }

        if info.description.is_empty() {
            anyhow::bail!("Plugin description cannot be empty");
        }

        if info.author.name.is_empty() {
            anyhow::bail!("Plugin author name cannot be empty");
        }

        Self::validate_version(&info.version)?;
        Self::validate_version(&info.min_app_version)?;

        Ok(())
    }

    fn validate_version(version: &str) -> Result<()> {
        if version.is_empty() {
            anyhow::bail!("Version cannot be empty");
        }

        if !version.parse::<semver::Version>().is_ok() {
            anyhow::bail!("Invalid version format: {}. Expected semver format (e.g., 1.0.0)", version);
        }

        Ok(())
    }

    fn validate_permissions(permissions: &[PluginPermission]) -> Result<()> {
        for perm in permissions {
            if perm.name.is_empty() {
                anyhow::bail!("Permission name cannot be empty");
            }

            if perm.description.is_empty() {
                anyhow::bail!("Permission description cannot be empty");
            }

            match perm.risk {
                PermissionRisk::Low | PermissionRisk::Medium | PermissionRisk::High => {}
            }
        }
        Ok(())
    }

    fn validate_contributes(contributes: &[PluginContribution]) -> Result<()> {
        for contribute in contributes {
            if contribute.contribution_type.is_empty() {
                anyhow::bail!("Contribution type cannot be empty");
            }

            if contribute.id.is_empty() {
                anyhow::bail!("Contribution ID cannot be empty");
            }

            if contribute.label.is_empty() {
                anyhow::bail!("Contribution label cannot be empty");
            }
        }
        Ok(())
    }

    fn validate_script(script: &Option<PluginScript>) -> Result<()> {
        if let Some(script) = script {
            if script.language.is_empty() {
                anyhow::bail!("Script language cannot be empty");
            }

            if script.entry_point.is_empty() {
                anyhow::bail!("Script entry point cannot be empty");
            }

            match script.language.as_str() {
                "javascript" | "python" | "lua" => {}
                _ => anyhow::bail!("Unsupported script language: {}. Supported: javascript, python, lua", script.language),
            }
        }
        Ok(())
    }

    pub fn create_template(info: PluginInfo) -> PluginManifest {
        PluginManifest {
            info,
            permissions: vec![],
            capabilities: vec![],
            contributes: vec![],
            script: None,
            settings: None,
            dependencies: HashMap::new(),
        }
    }

    pub fn to_json(manifest: &PluginManifest) -> Result<String> {
        serde_json::to_string_pretty(manifest)
            .context("Failed to serialize plugin manifest")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_manifest() {
        let json = r#"{
            "info": {
                "id": "test-plugin",
                "version": "1.0.0",
                "name": "Test Plugin",
                "description": "A test plugin",
                "author": {
                    "name": "Test Author"
                },
                "pluginType": "editor_extension",
                "minAppVersion": "1.0.0"
            }
        }"#;

        let manifest = ManifestParser::parse_from_str(json).unwrap();
        assert_eq!(manifest.info.id, "test-plugin");
        assert_eq!(manifest.info.version, "1.0.0");
    }

    #[test]
    fn test_validate_invalid_id() {
        let json = r#"{
            "info": {
                "id": "",
                "version": "1.0.0",
                "name": "Test Plugin",
                "description": "A test plugin",
                "author": {
                    "name": "Test Author"
                },
                "pluginType": "editor_extension",
                "minAppVersion": "1.0.0"
            }
        }"#;

        assert!(ManifestParser::parse_from_str(json).is_err());
    }

    #[test]
    fn test_validate_invalid_version() {
        let json = r#"{
            "info": {
                "id": "test-plugin",
                "version": "invalid",
                "name": "Test Plugin",
                "description": "A test plugin",
                "author": {
                    "name": "Test Author"
                },
                "pluginType": "editor_extension",
                "minAppVersion": "1.0.0"
            }
        }"#;

        assert!(ManifestParser::parse_from_str(json).is_err());
    }
}
