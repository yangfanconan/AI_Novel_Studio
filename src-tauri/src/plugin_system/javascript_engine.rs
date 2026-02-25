use crate::plugin_system::script::{ScriptEngine, ScriptContext};
use anyhow::Result;
use serde_json::Value;

pub struct JavaScriptEngine;

impl JavaScriptEngine {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl Default for JavaScriptEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create JavaScript engine")
    }
}

unsafe impl Send for JavaScriptEngine {}
unsafe impl Sync for JavaScriptEngine {}

impl ScriptEngine for JavaScriptEngine {
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
        "javascript"
    }
}
