use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplateConfig {
    pub scene_image: String,
    pub scene_video: String,
    pub negative: String,
    pub screenplay: String,
}

impl Default for PromptTemplateConfig {
    fn default() -> Self {
        Self {
            scene_image: "{{style_tokens}}, {{character_description}}, {{visual_content}}, {{camera}}, {{quality_tokens}}".to_string(),
            scene_video: "{{character_description}}, {{visual_content}}, {{action}}, {{camera}}".to_string(),
            negative: "blurry, low quality, watermark, text, logo, signature, bad anatomy, deformed, mutated".to_string(),
            screenplay: r#"你是一个专业的视频剧本创作者。请根据以下描述创作一个短视频剧本：

描述：{{prompt}}

要求：
1. 创作 {{scene_count}} 个场景
2. 每个场景包含：场景编号、旁白、视觉内容描述、角色动作、镜头类型、角色外观描述
3. visualContent/action/camera/characterDescription 用英文描述
4. narration 用中文
5. 不要输出 mood/情绪 字段（前端不需要）

输出格式为 JSON：
{
  "title": "视频标题",
  "scenes": [
    {
      "sceneId": 1,
      "narration": "中文旁白",
      "visualContent": "English visual description",
      "action": "English character action",
      "camera": "Camera type in English (Close-up/Medium Shot/Wide Shot/etc.)",
      "characterDescription": "English character appearance description"
    }
  ]
}"#.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIScene {
    pub scene_id: i32,
    pub narration: String,
    pub visual_content: String,
    pub action: String,
    pub camera: String,
    pub character_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AICharacter {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub char_type: String,
    pub visual_traits: String,
    pub style_tokens: Vec<String>,
    pub color_palette: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub style_tokens: Vec<String>,
    pub quality_tokens: Vec<String>,
}

pub struct PromptCompiler {
    templates: PromptTemplateConfig,
}

impl PromptCompiler {
    pub fn new() -> Self {
        Self {
            templates: PromptTemplateConfig::default(),
        }
    }

    pub fn with_templates(templates: PromptTemplateConfig) -> Self {
        Self { templates }
    }

    pub fn compile(
        &self,
        template_id: &str,
        variables: HashMap<String, String>,
    ) -> Result<String, String> {
        let template = match template_id {
            "scene_image" => &self.templates.scene_image,
            "scene_video" => &self.templates.scene_video,
            "negative" => &self.templates.negative,
            "screenplay" => &self.templates.screenplay,
            _ => return Err(format!("Template '{}' not found", template_id)),
        };

        Ok(self.interpolate(template, variables))
    }

    fn interpolate(
        &self,
        template: &str,
        variables: HashMap<String, String>,
    ) -> String {
        let mut result = template.to_string();
        
        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, &value);
        }

        result = regex::Regex::new(r"\{\{\w+\}\}")
            .unwrap()
            .replace_all(&result, "")
            .to_string();

        result
    }

    pub fn compile_scene_image_prompt(
        &self,
        scene: &AIScene,
        characters: &[AICharacter],
        config: &GenerationConfig,
    ) -> Result<String, String> {
        let character_desc = if scene.character_description.is_empty() {
            characters
                .iter()
                .map(|c| c.visual_traits.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            scene.character_description.clone()
        };

        let mut variables = HashMap::new();
        variables.insert("style_tokens".to_string(), config.style_tokens.join(", "));
        variables.insert("character_description".to_string(), character_desc);
        variables.insert("visual_content".to_string(), scene.visual_content.clone());
        variables.insert("camera".to_string(), scene.camera.clone());
        variables.insert("quality_tokens".to_string(), config.quality_tokens.join(", "));

        self.compile("scene_image", variables)
    }

    pub fn compile_scene_video_prompt(
        &self,
        scene: &AIScene,
        characters: &[AICharacter],
    ) -> Result<String, String> {
        let character_desc = if scene.character_description.is_empty() {
            characters
                .iter()
                .map(|c| c.visual_traits.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            scene.character_description.clone()
        };

        let mut variables = HashMap::new();
        variables.insert("character_description".to_string(), character_desc);
        variables.insert("visual_content".to_string(), scene.visual_content.clone());
        variables.insert("action".to_string(), scene.action.clone());
        variables.insert("camera".to_string(), scene.camera.clone());

        self.compile("scene_video", variables)
    }

    pub fn compile_screenplay_prompt(
        &self,
        user_prompt: &str,
        scene_count: i32,
    ) -> Result<String, String> {
        let mut variables = HashMap::new();
        variables.insert("prompt".to_string(), user_prompt.to_string());
        variables.insert("scene_count".to_string(), scene_count.to_string());

        self.compile("screenplay", variables)
    }

    pub fn get_negative_prompt(&self, additional_terms: Option<Vec<String>>) -> String {
        let mut negative = self.templates.negative.clone();
        if let Some(terms) = additional_terms {
            if !terms.is_empty() {
                negative.push_str(", ");
                negative.push_str(&terms.join(", "));
            }
        }
        negative
    }

    pub fn update_templates(&mut self, updates: PromptTemplateConfig) {
        self.templates = updates;
    }

    pub fn get_templates(&self) -> &PromptTemplateConfig {
        &self.templates
    }
}

impl Default for PromptCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub async fn compile_image_prompt(
    scene_json: String,
    characters_json: String,
    style_tokens: Vec<String>,
    quality_tokens: Vec<String>,
) -> Result<String, String> {
    let scene: AIScene = serde_json::from_str(&scene_json)
        .map_err(|e| format!("解析场景失败: {}", e))?;
    
    let characters: Vec<AICharacter> = serde_json::from_str(&characters_json)
        .map_err(|e| format!("解析角色失败: {}", e))?;

    let config = GenerationConfig {
        style_tokens,
        quality_tokens,
    };

    let compiler = PromptCompiler::new();
    compiler.compile_scene_image_prompt(&scene, &characters, &config)
}

#[tauri::command]
pub async fn compile_video_prompt(
    scene_json: String,
    characters_json: String,
) -> Result<String, String> {
    let scene: AIScene = serde_json::from_str(&scene_json)
        .map_err(|e| format!("解析场景失败: {}", e))?;
    
    let characters: Vec<AICharacter> = serde_json::from_str(&characters_json)
        .map_err(|e| format!("解析角色失败: {}", e))?;

    let compiler = PromptCompiler::new();
    compiler.compile_scene_video_prompt(&scene, &characters)
}

#[tauri::command]
pub async fn compile_screenplay_prompt(
    prompt: String,
    scene_count: i32,
) -> Result<String, String> {
    let compiler = PromptCompiler::new();
    compiler.compile_screenplay_prompt(&prompt, scene_count)
}

#[tauri::command]
pub async fn get_negative_prompt(
    additional_terms: Option<Vec<String>>,
) -> Result<String, String> {
    let compiler = PromptCompiler::new();
    Ok(compiler.get_negative_prompt(additional_terms))
}
