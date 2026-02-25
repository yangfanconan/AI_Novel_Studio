use crate::plugin_system::script::{ScriptEngine, ScriptContext};
use anyhow::Result;
use serde_json::Value;

pub struct LuaEngine;

impl LuaEngine {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl Default for LuaEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create Lua engine")
    }
}

unsafe impl Send for LuaEngine {}
unsafe impl Sync for LuaEngine {}

impl ScriptEngine for LuaEngine {
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
        "lua"
    }
}
