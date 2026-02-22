use ai_novel_studio::ai::{
    AIService, ModelRegistry, PromptManager, PromptTemplate,
    models::AIRequest,
};
use std::collections::HashMap;

#[tokio::test]
async fn test_prompt_manager_get_template() {
    let manager = PromptManager::new();
    
    let template = manager.get_template("novel-continuation").await;
    assert!(template.is_some());
    
    let template = template.unwrap();
    assert_eq!(template.id, "novel-continuation");
    assert_eq!(template.name, "小说续写");
    assert!(!template.system_prompt.is_empty());
    assert!(!template.user_prompt_template.is_empty());
}

#[tokio::test]
async fn test_prompt_manager_list_templates() {
    let manager = PromptManager::new();
    
    let templates = manager.list_templates(None).await;
    assert!(!templates.is_empty());
    assert!(templates.len() >= 5);
}

#[tokio::test]
async fn test_prompt_manager_list_by_category() {
    let manager = PromptManager::new();
    
    let writing_templates = manager.list_templates(Some("writing".to_string())).await;
    assert!(!writing_templates.is_empty());
    
    for template in writing_templates {
        assert_eq!(template.category, "writing");
    }
}

#[tokio::test]
async fn test_prompt_manager_build_prompt() {
    let manager = PromptManager::new();
    
    let mut variables = HashMap::new();
    variables.insert("context".to_string(), "这是一段测试内容".to_string());
    variables.insert("instruction".to_string(), "请继续续写".to_string());
    
    let result = manager.build_prompt("novel-continuation", &variables).await;
    assert!(result.is_ok());
    
    let (system_prompt, user_prompt) = result.unwrap();
    assert!(!system_prompt.is_empty());
    assert!(!user_prompt.is_empty());
    assert!(user_prompt.contains("这是一段测试内容"));
    assert!(user_prompt.contains("请继续续写"));
}

#[tokio::test]
async fn test_prompt_manager_add_remove_template() {
    let manager = PromptManager::new();
    
    let new_template = PromptTemplate {
        id: "test-template".to_string(),
        name: "测试模板".to_string(),
        category: "test".to_string(),
        system_prompt: "测试系统提示".to_string(),
        user_prompt_template: "测试 {var1}".to_string(),
        variables: vec!["var1".to_string()],
    };
    
    manager.add_template(new_template).await;
    
    let template = manager.get_template("test-template").await;
    assert!(template.is_some());
    
    let removed = manager.remove_template("test-template").await;
    assert!(removed);
    
    let template = manager.get_template("test-template").await;
    assert!(template.is_none());
}

#[tokio::test]
async fn test_model_registry() {
    let registry = ModelRegistry::new();
    
    let models = registry.list_models().await;
    assert_eq!(models.len(), 0);
    
    let model_id = "test-model".to_string();
    let model = create_mock_model();
    
    registry.register_model(model_id.clone(), model).await;
    
    let models = registry.list_models().await;
    assert_eq!(models.len(), 1);
    assert!(models.contains(&model_id));
    
    let retrieved = registry.get_model(&model_id).await;
    assert!(retrieved.is_some());
    
    let missing = registry.get_model("non-existent").await;
    assert!(missing.is_none());
}

#[tokio::test]
async fn test_ai_service_creation() {
    let service = AIService::new();
    
    assert!(service.get_registry().list_models().await.is_empty());
    
    let templates = service.get_prompt_manager().list_templates(None).await;
    assert!(!templates.is_empty());
}

fn create_mock_model() -> std::sync::Arc<dyn ai_novel_studio::ai::AIModel> {
    use ai_novel_studio::ai::AIModel;
    
    struct MockModel;
    
    #[async_trait::async_trait]
    impl AIModel for MockModel {
        fn get_name(&self) -> String {
            "mock-model".to_string()
        }

        fn get_provider(&self) -> String {
            "mock".to_string()
        }

        async fn complete(&self, _request: ai_novel_studio::ai::models::AIRequest) -> Result<ai_novel_studio::ai::models::AIResponse, String> {
            Ok(ai_novel_studio::ai::models::AIResponse {
                content: "Mock response".to_string(),
                finish_reason: Some("stop".to_string()),
                usage: None,
            })
        }

        async fn complete_stream(&self, _request: ai_novel_studio::ai::models::AIRequest) -> Result<ai_novel_studio::ai::ModelStream, String> {
            Err("Stream not implemented for mock".to_string())
        }
    }
    
    std::sync::Arc::new(MockModel)
}
