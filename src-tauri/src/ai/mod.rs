pub mod models;
pub mod traits;
pub mod openai_adapter;
pub mod ollama_adapter;
pub mod bigmodel_adapter;
pub mod prompt_manager;
pub mod service;
pub mod generators;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use models::*;
pub use traits::{AIModel, ModelStream};
pub use openai_adapter::OpenAIAdapter;
pub use ollama_adapter::OllamaAdapter;
pub use bigmodel_adapter::BigModelAdapter;
pub use prompt_manager::PromptManager;
pub use service::{AIService, create_ai_service};
pub use generators::{
    GeneratorPrompts, FormatOptions,
    GeneratedCharacter, GeneratedCharacterRelation,
    GeneratedWorldView, GeneratedPlotPoint, GeneratedStoryboard,
};

#[derive(Clone)]
pub struct ModelRegistry {
    models: Arc<RwLock<HashMap<String, Arc<dyn AIModel>>>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_model(&self, id: String, model: Arc<dyn AIModel>) {
        let mut models = self.models.write().await;
        models.insert(id, model);
    }

    pub async fn get_model(&self, id: &str) -> Option<Arc<dyn AIModel>> {
        let models = self.models.read().await;
        models.get(id).cloned()
    }

    pub async fn list_models(&self) -> Vec<String> {
        let models = self.models.read().await;
        models.keys().cloned().collect()
    }

    pub async fn initialize_default_bigmodel_models(&self) {
        let default_api_key = std::env::var("BIGMODEL_API_KEY")
            .unwrap_or_else(|_| "45913d02a609452b916a1706b8dc9702".to_string());

        let glm4 = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4".to_string()));
        let glm4_plus = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-plus".to_string()));
        let glm4_air = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-air".to_string()));
        let glm4_flash = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-flash".to_string()));
        let glm4_flashx = Arc::new(BigModelAdapter::new(default_api_key.clone(), "glm-4-flashx".to_string()));

        self.register_model("glm-4".to_string(), glm4).await;
        self.register_model("glm-4-plus".to_string(), glm4_plus).await;
        self.register_model("glm-4-air".to_string(), glm4_air).await;
        self.register_model("glm-4-flash".to_string(), glm4_flash).await;
        self.register_model("glm-4-flashx".to_string(), glm4_flashx).await;
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}
