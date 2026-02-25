use crate::multimedia_generation::types::*;
use crate::ai::traits::AIModel;
use crate::ai::models::{AIRequest, AIResponse, AIMessage};
use std::sync::Arc;

pub struct ScriptGenerator {
    ai_model: Arc<dyn AIModel>,
}

impl ScriptGenerator {
    pub fn new(ai_model: Arc<dyn AIModel>) -> Self {
        Self { ai_model }
    }

    pub async fn convert_to_script(
        &self,
        text: &str,
        format: ScriptFormat,
    ) -> Result<Script, String> {
        let prompt = self.build_conversion_prompt(text, &format);

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

        self.parse_script(&response.content, &format)
    }

    fn build_conversion_prompt(&self, text: &str, format: &ScriptFormat) -> String {
        let format_name = match format {
            ScriptFormat::Hollywood => "好莱坞标准",
            ScriptFormat::Bbc => "BBC标准",
            ScriptFormat::Chinese => "中国标准",
            ScriptFormat::StagePlay => "舞台剧",
        };

        format!(
            "请将以下小说文本转换为专业的{}格式剧本：

原文：
{}

转换要求：
1. 场景标题（INT./EXT. + 地点 + 时间）
2. 动作描述
3. 角色名
4. 对白
5. 舞台指示（括号内）
6. 转场指示

请以JSON格式输出，包含以下字段：
- title: 剧本标题
- format: 格式
- scenes: 场景列表（包含heading, location, time_of_day, action, elements）
- characters: 角色列表
- locations: 地点列表

场景元素类型：
- Action: 动作描述
- Dialogue: 对白（包含character, parenthetical, dialogue）
- Transition: 转场
- Shot: 镜头

输出格式示例：
{{
  \"title\": \"剧本标题\",
  \"format\": \"Hollywood\",
  \"scenes\": [
    {{
      \"heading\": \"INT. 咖啡馆 - DAY\",
      \"location\": \"咖啡馆\",
      \"time_of_day\": \"DAY\",
      \"action\": \"角色走进咖啡馆\",
      \"elements\": [
        {{
          \"type\": \"Action\",
          \"content\": \"角色走进咖啡馆\"
        }},
        {{
          \"type\": \"Dialogue\",
          \"character\": \"角色名\",
          \"parenthetical\": \"微笑\",
          \"dialogue\": \"对白内容\"
        }}
      ]
    }}
  ],
  \"characters\": [\"角色1\", \"角色2\"],
  \"locations\": [\"咖啡馆\", \"街道\"]
}}",
            format_name, text
        )
    }

    fn parse_script(&self, json_text: &str, format: &ScriptFormat) -> Result<Script, String> {
        let parsed: serde_json::Value =
            serde_json::from_str(json_text).map_err(|e| format!("解析剧本失败: {}", e))?;

        let title = parsed["title"]
            .as_str()
            .unwrap_or("未命名剧本")
            .to_string();

        let scenes = self.parse_script_scenes(&parsed["scenes"])?;
        let characters = self.parse_string_array(&parsed["characters"])?;
        let locations = self.parse_string_array(&parsed["locations"])?;

        let metadata = ScriptMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            source_text: json_text.to_string(),
            format: format.clone(),
        };

        Ok(Script {
            title,
            format: format.clone(),
            scenes,
            characters,
            locations,
            metadata,
        })
    }

    fn parse_script_scenes(&self, scenes_value: &serde_json::Value) -> Result<Vec<ScriptScene>, String> {
        let scenes = scenes_value
            .as_array()
            .ok_or_else(|| "scenes不是数组".to_string())?;

        let mut result = Vec::new();
        for scene_value in scenes {
            let heading = scene_value["heading"]
                .as_str()
                .ok_or_else(|| "缺少heading".to_string())?
                .to_string();

            let location = scene_value["location"]
                .as_str()
                .ok_or_else(|| "缺少location".to_string())?
                .to_string();

            let time_of_day = scene_value["time_of_day"]
                .as_str()
                .ok_or_else(|| "缺少time_of_day".to_string())?
                .to_string();

            let action = scene_value["action"]
                .as_str()
                .ok_or_else(|| "缺少action".to_string())?
                .to_string();

            let elements = self.parse_script_elements(&scene_value["elements"])?;

            result.push(ScriptScene {
                heading,
                location,
                time_of_day,
                action,
                elements,
            });
        }

        Ok(result)
    }

    fn parse_script_elements(
        &self,
        elements_value: &serde_json::Value,
    ) -> Result<Vec<ScriptElement>, String> {
        let elements = elements_value
            .as_array()
            .ok_or_else(|| "elements不是数组".to_string())?;

        let mut result = Vec::new();
        for element_value in elements {
            let element = self.parse_script_element(element_value)?;
            result.push(element);
        }

        Ok(result)
    }

    fn parse_script_element(&self, value: &serde_json::Value) -> Result<ScriptElement, String> {
        let element_type = value["type"]
            .as_str()
            .ok_or_else(|| "缺少type".to_string())?;

        match element_type {
            "Action" => Ok(ScriptElement::Action {
                content: value["content"]
                    .as_str()
                    .ok_or_else(|| "缺少content".to_string())?
                    .to_string(),
            }),
            "Dialogue" => Ok(ScriptElement::Dialogue {
                character: value["character"]
                    .as_str()
                    .ok_or_else(|| "缺少character".to_string())?
                    .to_string(),
                parenthetical: value["parenthetical"].as_str().map(|s| s.to_string()),
                dialogue: value["dialogue"]
                    .as_str()
                    .ok_or_else(|| "缺少dialogue".to_string())?
                    .to_string(),
            }),
            "Transition" => Ok(ScriptElement::Transition {
                transition: value["transition"]
                    .as_str()
                    .ok_or_else(|| "缺少transition".to_string())?
                    .to_string(),
            }),
            "Shot" => Ok(ScriptElement::Shot {
                shot_type: value["shot_type"]
                    .as_str()
                    .ok_or_else(|| "缺少shot_type".to_string())?
                    .to_string(),
            }),
            _ => Err(format!("未知的元素类型: {}", element_type)),
        }
    }

    fn parse_string_array(&self, value: &serde_json::Value) -> Result<Vec<String>, String> {
        let array = value
            .as_array()
            .ok_or_else(|| "不是数组".to_string())?;

        let mut result = Vec::new();
        for item in array {
            if let Some(s) = item.as_str() {
                result.push(s.to_string());
            }
        }

        Ok(result)
    }

    pub async fn optimize_for_screen(&self, script: &Script) -> Result<Script, String> {
        let prompt = format!(
            "请优化以下剧本，使其更适合影视表达：

{}

优化方向：
1. 精简对白，增加视觉表达
2. 强化动作描述
3. 调整节奏
4. 增强戏剧冲突
5. 优化场景转换

请以相同的JSON格式返回优化后的剧本。",
            serde_json::to_string(script).unwrap_or_default()
        );

        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.4),
            max_tokens: None,
            stream: Some(false),
        };

        let response = self
            .ai_model
            .complete(request)
            .await
            .map_err(|e| e.to_string())?;

        self.parse_script(&response.content, &script.format)
    }
}
