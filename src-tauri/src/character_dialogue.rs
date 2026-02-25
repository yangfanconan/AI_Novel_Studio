use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDialogue {
    pub id: String,
    pub character_id: String,
    pub user_message: String,
    pub ai_response: String,
    pub context: DialogueContext,
    pub metadata: DialogueMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContext {
    pub character: CharacterInfo,
    pub conversation_history: Vec<DialogueMessage>,
    pub current_emotion: Option<String>,
    pub scene_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInfo {
    pub id: String,
    pub name: String,
    pub role_type: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueMessage {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub message_type: String,
    pub character_state: Option<HashMap<String, String>>,
    pub emotional_context: Option<String>,
    pub scene_context: Option<String>,
    pub tokens_used: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueMetadata {
    pub timestamp: i64,
    pub model: String,
    pub tokens_used: i32,
    pub generation_time: f64,
    pub quality_score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSettings {
    pub ai_model: String,
    pub temperature: f64,
    pub max_tokens: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSession {
    pub id: String,
    pub character_id: String,
    pub chapter_id: Option<String>,
    pub session_name: String,
    pub system_prompt: Option<String>,
    pub context_summary: Option<String>,
    pub messages: Vec<DialogueMessage>,
    pub settings: DialogueSettings,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialoguePrompt {
    pub system_prompt: String,
    pub user_prompt: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub top_p: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub content: String,
    pub finish_reason: String,
    pub usage: TokenUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

pub struct CharacterDialogueManager;

impl CharacterDialogueManager {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_ai_response(
        character: &CharacterInfo,
        user_message: &str,
        context: &DialogueContext,
        _metadata: &DialogueMetadata,
    ) -> String {
        let simulated_responses = HashMap::from([
            ("高兴", vec![
                "太好了！我觉得今天特别开心！",
                "哈哈哈，这真是太有趣了！",
                "嗯，我觉得这件事很有意思。",
            ]),
            ("冷静", vec![
                "嗯，让我想想这个问题。",
                "这个情况需要仔细考虑。",
                "我不确定，但我们可以尝试。",
            ]),
            ("热情", vec![
                "这太棒了！我非常激动！",
                "我迫不及待想要开始！",
                "这个主意真是太好了！",
            ]),
            ("忧郁", vec![
                "唉...有时候事情就是这样的。",
                "我有点担心...",
                "希望一切都会好起来。",
            ]),
            ("狡诈", vec![
                "哼，这件事没那么简单...",
                "让我想想有什么可以利用的。",
                "我会找到最好的方法。",
            ]),
        ]);

        let personality = context.character.personality.as_deref().unwrap_or("高兴");
        let possible_responses = simulated_responses
            .get(personality)
            .unwrap_or(simulated_responses.get("高兴").unwrap());

        if possible_responses.is_empty() {
            format!("（{}听到你的话，思考了一下）嗯，这确实是个值得考虑的问题。", character.name)
        } else {
            let index = (user_message.len() + context.conversation_history.len()) % possible_responses.len();
            possible_responses[index].to_string()
        }
    }

    pub fn build_system_prompt(context: &DialogueContext) -> String {
        let history_len = context.conversation_history.len();
        let take_count = if history_len > 10 { 10 } else { history_len };

        let history_prompt = if context.conversation_history.is_empty() {
            String::new()
        } else {
            let history: Vec<String> = context.conversation_history
                .iter()
                .rev()
                .take(take_count)
                .rev()
                .map(|msg| {
                    format!("{}: {}", msg.role, msg.content)
                })
                .collect();

            format!("\n\n对话历史:\n{}", history.join("\n"))
        };

        let scene_prompt = context.scene_context
            .as_ref()
            .map(|s| format!("\n\n场景背景: {}", s))
            .unwrap_or_default();

        let character_info = &context.character;

        let personality = character_info.personality.as_ref().map(|s| s.as_str()).unwrap_or("");
        let background = character_info.background.as_ref().map(|s| s.as_str()).unwrap_or("");
        let role = character_info.role_type.as_ref().map(|s| s.as_str()).unwrap_or("");

        format!(
            "你是一个角色扮演助手。你现在扮演角色'{}'。

角色信息:
- 角色类型: {}
- 描述: {}
- 性格: {}

你的任务是根据角色的设定和性格特点，以角色的口吻和思维方式回应用户的消息。

{}{}",
            character_info.name,
            role,
            background,
            personality,
            history_prompt,
            scene_prompt
        )
    }
}
