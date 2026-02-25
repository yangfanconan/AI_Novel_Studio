use crate::multimedia_generation::types::*;
use crate::ai::traits::AIModel;
use crate::ai::models::{AIRequest, AIResponse, AIMessage};
use std::sync::Arc;

pub struct SceneExtractor {
    ai_model: Arc<dyn AIModel>,
}

impl SceneExtractor {
    pub fn new(ai_model: Arc<dyn AIModel>) -> Self {
        Self { ai_model }
    }

    pub async fn extract_scenes(&self, text: &str) -> Result<Vec<Scene>, String> {
        let prompt = self.build_extraction_prompt(text);
        
        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.3),
            max_tokens: None,
            stream: Some(false),
        };
        
        let response = self
            .ai_model
            .complete(request)
            .await
            .map_err(|e| e.to_string())?;

        self.parse_scenes(&response.content)
    }

    fn build_extraction_prompt(&self, text: &str) -> String {
        format!(
            "请分析以下小说文本，提取出所有可以转化为视觉场景的片段。

文本：
{}

请为每个场景提供以下信息，以JSON数组格式输出：
1. id - 场景ID（唯一标识符）
2. number - 场景编号
3. title - 场景标题
4. location - 地点
5. time_of_day - 时间（Dawn/Morning/Noon/Afternoon/Dusk/Evening/Night/Unknown）
6. characters - 出场角色列表（包含id, name, appearance, expression, action, dialogue）
7. description - 场景描述
8. action - 主要动作
9. emotional_tone - 情感基调（Happy/Sad/Tense/Romantic/Mysterious/Action/Peaceful/Dramatic/Horror/Comedy）
10. suggested_shots - 建议镜头类型列表
11. original_text - 原文片段
12. duration - 预计时长（秒，可选）
13. notes - 备注（可选）

输出格式示例：
[
  {{
    \"id\": \"scene_1\",
    \"number\": 1,
    \"title\": \"开场\",
    \"location\": \"森林边缘\",
    \"time_of_day\": \"Morning\",
    \"characters\": [],
    \"description\": \"清晨的阳光透过树叶洒在地面上\",
    \"action\": \"主角走出森林\",
    \"emotional_tone\": \"Peaceful\",
    \"suggested_shots\": [\"LongShot\", \"MediumShot\"],
    \"original_text\": \"...\"
  }}
]",
            text
        )
    }

    fn parse_scenes(&self, json_text: &str) -> Result<Vec<Scene>, String> {
        serde_json::from_str(json_text).map_err(|e| format!("解析场景失败: {}", e))
    }

    pub async fn identify_characters(&self, text: &str) -> Result<Vec<CharacterInScene>, String> {
        let prompt = format!(
            "请从以下文本中识别所有角色，并为每个角色提供详细信息：

文本：
{}

请提取以下信息，以JSON数组格式输出：
1. id - 角色ID
2. name - 角色名称
3. appearance - 外貌描述（可选）
4. expression - 表情（可选）
5. action - 动作（可选）
6. dialogue - 对白列表（可选）",
            text
        );

        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.3),
            max_tokens: None,
            stream: Some(false),
        };

        let response = self
            .ai_model
            .complete(request)
            .await
            .map_err(|e| e.to_string())?;

        serde_json::from_str(&response.content).map_err(|e| format!("解析角色失败: {}", e))
    }
}
