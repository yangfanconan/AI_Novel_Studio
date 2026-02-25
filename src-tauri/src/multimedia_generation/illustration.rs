use crate::multimedia_generation::types::*;
use crate::ai::traits::AIModel;
use crate::ai::models::{AIRequest, AIResponse, AIMessage};
use std::sync::Arc;

pub struct IllustrationGenerator {
    ai_model: Arc<dyn AIModel>,
}

impl IllustrationGenerator {
    pub fn new(ai_model: Arc<dyn AIModel>) -> Self {
        Self { ai_model }
    }

    pub async fn generate_scene_illustration(
        &self,
        scene: &Scene,
        options: IllustrationOptions,
    ) -> Result<Illustration, String> {
        let enhanced_prompt = self.enhance_prompt(scene, &options).await?;

        let images = self
            .generate_images(&enhanced_prompt, &options)
            .await?;

        let metadata = IllustrationMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            model: "placeholder-model".to_string(),
        };

        Ok(Illustration {
            scene_id: scene.id.clone(),
            images,
            prompt: enhanced_prompt,
            style: options.style,
            metadata,
        })
    }

    pub async fn generate_character_portrait(
        &self,
        character_id: String,
        character_name: String,
        appearance: String,
        style: ArtStyle,
    ) -> Result<CharacterPortrait, String> {
        let prompt = self.build_character_prompt(&character_name, &appearance, &style);

        let views = self
            .generate_character_views(&prompt, &style)
            .await?;

        let expressions = self
            .generate_character_expressions(&prompt, &style)
            .await?;

        let turnaround = self.generate_turnaround(&prompt, &style);

        Ok(CharacterPortrait {
            character_id,
            views,
            expressions,
            turnaround,
        })
    }

    pub async fn generate_cover(
        &self,
        project_name: String,
        project_description: String,
        genre: String,
        style: ArtStyle,
    ) -> Result<String, String> {
        let prompt = format!(
            "请为小说封面生成画面描述：

小说标题：{}
类型：{}
描述：{}
风格：{:?}

请提供一个详细的画面描述，包括：
1. 主要视觉元素
2. 色彩方案
3. 构图建议
4. 氛围描述",
            project_name, genre, project_description, style
        );

        let request = AIRequest {
            model: self.ai_model.get_name(),
            messages: vec![AIMessage {
                role: "user".to_string(),
                content: prompt,
            }],
            temperature: Some(0.5),
            max_tokens: None,
            stream: Some(false),
        };

        let description = self
            .ai_model
            .complete(request)
            .await
            .map_err(|e| e.to_string())?;

        Ok(format!(
            "cover_placeholder_800x1200.png?text={}",
            urlencoding::encode(&description.content)
        ))
    }

    async fn enhance_prompt(
        &self,
        scene: &Scene,
        options: &IllustrationOptions,
    ) -> Result<EnhancedPrompt, String> {
        let prompt = format!(
            "请将以下场景描述转换为适合AI图像生成的详细英文提示词：

场景：{}
地点：{}
时间：{:?}
角色：{}
情感：{:?}
风格：{:?}
质量：{}
纵横比：{}

请提供：
1. 主要提示词
2. 负面提示词
3. 建议的参数设置（steps, cfg_scale, sampler）

以JSON格式输出：
{{
  \"positive\": \"positive prompt\",
  \"negative\": \"negative prompt\",
  \"parameters\": {{
    \"steps\": 30,
    \"cfg_scale\": 7.0,
    \"sampler\": \"euler\"
  }}
}}",
            scene.description,
            scene.location,
            scene.time_of_day,
            scene
                .characters
                .iter()
                .map(|c| format!("{}: {}", c.name, c.appearance.as_deref().unwrap_or("")))
                .collect::<Vec<_>>()
                .join("; "),
            scene.emotional_tone,
            options.style,
            options.quality,
            options.aspect_ratio
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

        self.parse_enhanced_prompt(&response.content)
    }

    fn parse_enhanced_prompt(&self, json_text: &str) -> Result<EnhancedPrompt, String> {
        serde_json::from_str(json_text).map_err(|e| format!("解析提示词失败: {}", e))
    }

    async fn generate_images(
        &self,
        enhanced_prompt: &EnhancedPrompt,
        options: &IllustrationOptions,
    ) -> Result<Vec<String>, String> {
        let mut images = Vec::new();

        for i in 0..options.variations {
            let seed = chrono::Utc::now().timestamp_millis() + i as i64;
            let image_url = format!(
                "generated_{}x{}.png?seed={}&prompt={}",
                self.parse_width(&options.aspect_ratio),
                self.parse_height(&options.aspect_ratio),
                seed,
                urlencoding::encode(&enhanced_prompt.positive)
            );
            images.push(image_url);
        }

        Ok(images)
    }

    fn build_character_prompt(
        &self,
        character_name: &str,
        appearance: &str,
        style: &ArtStyle,
    ) -> String {
        format!(
            "Character portrait of {}: {}. Style: {:?}",
            character_name, appearance, style
        )
    }

    async fn generate_character_views(
        &self,
        prompt: &str,
        style: &ArtStyle,
    ) -> Result<Vec<CharacterView>, String> {
        let angles = vec!["front", "three_quarter", "side"];
        let mut views = Vec::new();

        for (i, angle) in angles.iter().enumerate() {
            let image_url = format!(
                "character_view_{}_{}.png?prompt={}&style={:?}&angle={}",
                i,
                angle,
                urlencoding::encode(prompt),
                style,
                angle
            );

            views.push(CharacterView {
                angle: angle.to_string(),
                image: image_url,
                embedding: None,
            });
        }

        Ok(views)
    }

    async fn generate_character_expressions(
        &self,
        prompt: &str,
        style: &ArtStyle,
    ) -> Result<Vec<CharacterExpression>, String> {
        let expressions = vec!["happy", "sad", "angry", "surprised", "neutral"];
        let mut result = Vec::new();

        for expr in expressions {
            let image_url = format!(
                "character_expression_{}.png?prompt={}&style={:?}&expression={}",
                expr,
                urlencoding::encode(prompt),
                style,
                expr
            );

            result.push(CharacterExpression {
                expression: expr.to_string(),
                image: image_url,
            });
        }

        Ok(result)
    }

    fn generate_turnaround(&self, prompt: &str, style: &ArtStyle) -> String {
        format!(
            "character_turnaround.png?prompt={}&style={:?}",
            urlencoding::encode(prompt),
            style
        )
    }

    fn parse_width(&self, aspect_ratio: &str) -> i32 {
        match aspect_ratio {
            "1:1" => 512,
            "16:9" => 640,
            "9:16" => 360,
            "4:3" => 640,
            "3:4" => 480,
            "2:3" => 360,
            "3:2" => 480,
            _ => 512,
        }
    }

    fn parse_height(&self, aspect_ratio: &str) -> i32 {
        match aspect_ratio {
            "1:1" => 512,
            "16:9" => 360,
            "9:16" => 640,
            "4:3" => 480,
            "3:4" => 640,
            "2:3" => 540,
            "3:2" => 320,
            _ => 512,
        }
    }
}
