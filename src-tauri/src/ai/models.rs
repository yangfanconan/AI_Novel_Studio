use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub model: String,
    pub messages: Vec<AIMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub finish_reason: Option<String>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIStreamChunk {
    pub content: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub api_endpoint: String,
    pub api_key: Option<String>,
    pub supports_streaming: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub name: String,
    pub category: String,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub variables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AICompletionRequest {
    pub model_id: String,
    pub context: String,
    pub instruction: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: Option<bool>,
    pub character_context: Option<String>,
    pub worldview_context: Option<String>,
    pub project_id: Option<String>,
    pub chapter_mission_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRewriteRequest {
    pub model_id: String,
    pub content: String,
    pub instruction: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

/// AI生成角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerateCharacterRequest {
    pub model_id: Option<String>,
    pub project_id: String,
    pub genre: Option<String>,
    pub character_type: Option<String>,
    pub description: Option<String>,
}

/// AI生成角色关系请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerateCharacterRelationsRequest {
    pub model_id: Option<String>,
    pub project_id: String,
}

/// AI生成世界观请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerateWorldViewRequest {
    pub model_id: Option<String>,
    pub project_id: String,
    pub category: String,
    pub description: Option<String>,
}

/// AI生成情节点请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGeneratePlotPointsRequest {
    pub model_id: Option<String>,
    pub project_id: String,
    pub context: Option<String>,
    pub direction: Option<String>,
}

/// AI生成分镜提示词请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerateStoryboardRequest {
    pub model_id: Option<String>,
    pub chapter_id: Option<String>,
    pub plot_point_id: Option<String>,
    pub content: Option<String>,
    pub style_preference: Option<String>,
}

/// AI一键排版请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIFormatContentRequest {
    pub model_id: Option<String>,
    pub content: String,
    pub paragraph_style: Option<String>,
    pub dialogue_style: Option<String>,
    pub scene_separator: Option<String>,
    pub special_requirements: Option<String>,
}
