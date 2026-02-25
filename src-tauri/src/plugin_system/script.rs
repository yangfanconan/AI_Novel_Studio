use crate::plugin_system::types::*;
use crate::plugin_system::api::*;
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub trait ScriptEngine: Send + Sync {
    fn execute(&self, script: &str, context: &ScriptContext) -> Result<Value>;
    fn evaluate(&self, expression: &str, context: &ScriptContext) -> Result<Value>;
    fn call_function(&self, function_name: &str, args: Vec<Value>, context: &ScriptContext) -> Result<Value>;
    fn get_language(&self) -> &'static str;
}

pub struct ScriptContext {
    pub plugin_id: String,
    pub app_version: String,
    pub data_dir: String,
    pub config_dir: String,
    pub api: Arc<PluginAPI>,
}

#[derive(Clone)]
pub struct PluginContext {
    pub plugin_id: String,
    pub app_version: String,
    pub data_dir: String,
    pub config_dir: String,
}

pub struct ScriptEngineManager {
    javascript_engine: Option<Arc<dyn ScriptEngine>>,
    python_engine: Option<Arc<dyn ScriptEngine>>,
    lua_engine: Option<Arc<dyn ScriptEngine>>,
}

impl ScriptEngineManager {
    pub fn new() -> Self {
        Self {
            javascript_engine: None,
            python_engine: None,
            lua_engine: None,
        }
    }

    pub fn register_engine(&mut self, engine: Arc<dyn ScriptEngine>) {
        let lang = engine.get_language();
        match lang {
            "javascript" => {
                self.javascript_engine = Some(engine);
            }
            "python" => {
                self.python_engine = Some(engine);
            }
            "lua" => {
                self.lua_engine = Some(engine);
            }
            _ => {
                log::warn!("Unsupported script language: {}", lang);
            }
        }
    }

    pub fn get_engine(&self, language: &str) -> Result<Arc<dyn ScriptEngine>> {
        match language {
            "javascript" => {
                self.javascript_engine.clone()
                    .ok_or_else(|| anyhow::anyhow!("JavaScript engine not registered"))
            }
            "python" => {
                self.python_engine.clone()
                    .ok_or_else(|| anyhow::anyhow!("Python engine not registered"))
            }
            "lua" => {
                self.lua_engine.clone()
                    .ok_or_else(|| anyhow::anyhow!("Lua engine not registered"))
            }
            _ => {
                anyhow::bail!("Unsupported script language: {}", language)
            }
        }
    }

    pub fn is_language_supported(&self, language: &str) -> bool {
        match language {
            "javascript" => self.javascript_engine.is_some(),
            "python" => self.python_engine.is_some(),
            "lua" => self.lua_engine.is_some(),
            _ => false,
        }
    }

    pub fn supported_languages(&self) -> Vec<&'static str> {
        let mut langs = Vec::new();
        if self.javascript_engine.is_some() {
            langs.push("javascript");
        }
        if self.python_engine.is_some() {
            langs.push("python");
        }
        if self.lua_engine.is_some() {
            langs.push("lua");
        }
        langs
    }
}

impl Default for ScriptEngineManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ScriptExecutor {
    engine_manager: Arc<ScriptEngineManager>,
}

impl ScriptExecutor {
    pub fn new(engine_manager: Arc<ScriptEngineManager>) -> Self {
        Self {
            engine_manager,
        }
    }

    pub async fn execute_plugin_script(
        &self,
        plugin: &Plugin,
        script: &str,
        api: Arc<PluginAPI>,
    ) -> Result<Value> {
        let plugin_script = plugin.manifest.script.as_ref()
            .context("Plugin does not have a script configuration")?;

        let engine = self.engine_manager.get_engine(&plugin_script.language)?;

        let context = ScriptContext {
            plugin_id: plugin.manifest.info.id.clone(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            data_dir: std::env::var("APP_DATA_DIR").unwrap_or_else(|_| ".".to_string()),
            config_dir: std::env::var("APP_CONFIG_DIR").unwrap_or_else(|_| ".".to_string()),
            api,
        };

        engine.execute(script, &context)
    }

    pub async fn call_plugin_function(
        &self,
        plugin: &Plugin,
        function_name: &str,
        args: Vec<Value>,
        api: Arc<PluginAPI>,
    ) -> Result<Value> {
        let plugin_script = plugin.manifest.script.as_ref()
            .context("Plugin does not have a script configuration")?;

        let engine = self.engine_manager.get_engine(&plugin_script.language)?;

        let context = ScriptContext {
            plugin_id: plugin.manifest.info.id.clone(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            data_dir: std::env::var("APP_DATA_DIR").unwrap_or_else(|_| ".".to_string()),
            config_dir: std::env::var("APP_CONFIG_DIR").unwrap_or_else(|_| ".".to_string()),
            api,
        };

        engine.call_function(function_name, args, &context)
    }

    pub fn execute_hook(&self, plugin: &Plugin, hook_name: &str, payload: Value, api: Arc<PluginAPI>) -> Result<Value> {
        let plugin_script = plugin.manifest.script.as_ref()
            .context("Plugin does not have a script configuration")?;

        let engine = self.engine_manager.get_engine(&plugin_script.language)?;

        let context = ScriptContext {
            plugin_id: plugin.manifest.info.id.clone(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            data_dir: std::env::var("APP_DATA_DIR").unwrap_or_else(|_| ".".to_string()),
            config_dir: std::env::var("APP_CONFIG_DIR").unwrap_or_else(|_| ".".to_string()),
            api,
        };

        let hook_function = format!("on_{}", hook_name);
        engine.call_function(&hook_function, vec![payload], &context)
    }
}

pub struct NoOpScriptEngine;

impl ScriptEngine for NoOpScriptEngine {
    fn execute(&self, _script: &str, _context: &ScriptContext) -> Result<Value> {
        Ok(Value::Null)
    }

    fn evaluate(&self, _expression: &str, _context: &ScriptContext) -> Result<Value> {
        Ok(Value::Null)
    }

    fn call_function(&self, _function_name: &str, _args: Vec<Value>, _context: &ScriptContext) -> Result<Value> {
        Ok(Value::Null)
    }

    fn get_language(&self) -> &'static str {
        "noop"
    }
}
