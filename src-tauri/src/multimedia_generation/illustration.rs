use crate::multimedia_generation::types::*;
use crate::multimedia_generation::image_client::{ImageClient, ImageProviderConfig, ImageGenerationRequest};
use crate::ai::traits::AIModel;
use crate::ai::models::{AIRequest, AIMessage};
use std::sync::Arc;

pub struct IllustrationGenerator {
    ai_model: Arc<dyn AIModel>,
    image_client: ImageClient,
    provider_config: Option<ImageProviderConfig>,
}

impl IllustrationGenerator {
    pub fn new(ai_model: Arc<dyn AIModel>) -> Self {
        Self {
            ai_model,
            image_client: ImageClient::new(),
            provider_config: None,
        }
    }

    pub fn with_provider(ai_model: Arc<dyn AIModel>, provider: ImageProviderConfig) -> Self {
        Self {
            ai_model,
            image_client: ImageClient::new(),
            provider_config: Some(provider),
        }
    }

    pub fn set_provider(&mut self, provider: ImageProviderConfig) {
        self.provider_config = Some(provider);
    }

    pub async fn generate_scene_illustration(
        &self,
        scene: &Scene,
        options: IllustrationOptions,
    ) -> Result<Illustration, String> {
        let enhanced_prompt = self.enhance_prompt(scene, &options).await?;

        let images = if let Some(ref config) = self.provider_config {
            if config.is_enabled && !config.api_key.is_empty() {
                self.generate_real_images(config, &enhanced_prompt, &options).await?
            } else {
                self.generate_placeholder_images(&enhanced_prompt, &options).await?
            }
        } else {
            self.generate_placeholder_images(&enhanced_prompt, &options).await?
        };

        let metadata = IllustrationMetadata {
            generated_at: chrono::Utc::now().to_rfc3339(),
            model: self.provider_config.as_ref()
                .map(|c| format!("{}:{}", c.id, c.model))
                .unwrap_or_else(|| "placeholder".to_string()),
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

        let views = if let Some(ref config) = self.provider_config {
            if config.is_enabled && !config.api_key.is_empty() {
                self.generate_real_character_views(config, &prompt, &style).await?
            } else {
                self.generate_placeholder_character_views(&prompt, &style).await?
            }
        } else {
            self.generate_placeholder_character_views(&prompt, &style).await?
        };

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

        if let Some(ref config) = self.provider_config {
            if config.is_enabled && !config.api_key.is_empty() {
                let (width, height) = ImageClient::parse_aspect_ratio("2:3");
                let gen_request = ImageGenerationRequest {
                    prompt: format!("Book cover design: {}", description.content),
                    negative_prompt: Some("low quality, blurry, distorted".to_string()),
                    width,
                    height,
                    steps: Some(30),
                    cfg_scale: Some(7.0),
                    seed: None,
                    num_images: Some(1),
                };

                match self.image_client.generate_image(config, gen_request).await {
                    Ok(response) => {
                        if let Some(img) = response.images.first() {
                            if let Some(ref url) = img.url {
                                return Ok(url.clone());
                            }
                            if let Some(ref b64) = img.b64_json {
                                return Ok(format!("data:image/png;base64,{}", b64));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("封面生成失败: {}", e);
                    }
                }
            }
        }

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
        let json_text = json_text.trim();
        let json_text = if json_text.starts_with("```json") {
            json_text.trim_start_matches("```json").trim_end_matches("```").trim()
        } else if json_text.starts_with("```") {
            json_text.trim_start_matches("```").trim_end_matches("```").trim()
        } else {
            json_text
        };
        serde_json::from_str(json_text).map_err(|e| format!("解析提示词失败: {}", e))
    }

    async fn generate_real_images(
        &self,
        config: &ImageProviderConfig,
        enhanced_prompt: &EnhancedPrompt,
        options: &IllustrationOptions,
    ) -> Result<Vec<String>, String> {
        let (width, height) = ImageClient::parse_aspect_ratio(&options.aspect_ratio);
        
        let gen_request = ImageGenerationRequest {
            prompt: enhanced_prompt.positive.clone(),
            negative_prompt: Some(enhanced_prompt.negative.clone()),
            width,
            height,
            steps: Some(enhanced_prompt.parameters.steps),
            cfg_scale: Some(enhanced_prompt.parameters.cfg_scale as f32),
            seed: None,
            num_images: Some(options.variations as i32),
        };

        let response = self.image_client.generate_image(config, gen_request).await?;

        let images: Vec<String> = response.images.iter().filter_map(|img| {
            if let Some(ref url) = img.url {
                Some(url.clone())
            } else if let Some(ref b64) = img.b64_json {
                Some(format!("data:image/png;base64,{}", b64))
            } else {
                None
            }
        }).collect();

        Ok(images)
    }

    async fn generate_placeholder_images(
        &self,
        enhanced_prompt: &EnhancedPrompt,
        options: &IllustrationOptions,
    ) -> Result<Vec<String>, String> {
        let mut images = Vec::new();
        let (width, height) = ImageClient::parse_aspect_ratio(&options.aspect_ratio);

        for i in 0..options.variations {
            let seed = chrono::Utc::now().timestamp_millis() + i as i64;
            let image_url = format!(
                "generated_{}x{}.png?seed={}&prompt={}",
                width,
                height,
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

    async fn generate_real_character_views(
        &self,
        config: &ImageProviderConfig,
        prompt: &str,
        style: &ArtStyle,
    ) -> Result<Vec<CharacterView>, String> {
        let angles = vec!["front view", "three-quarter view", "side view"];
        let mut views = Vec::new();

        for (i, angle) in angles.iter().enumerate() {
            let gen_request = ImageGenerationRequest {
                prompt: format!("{}, {}", prompt, angle),
                negative_prompt: Some("low quality, blurry, distorted, multiple characters".to_string()),
                width: 512,
                height: 512,
                steps: Some(25),
                cfg_scale: Some(7.0),
                seed: None,
                num_images: Some(1),
            };

            match self.image_client.generate_image(config, gen_request).await {
                Ok(response) => {
                    if let Some(img) = response.images.first() {
                        let image = if let Some(ref url) = img.url {
                            url.clone()
                        } else if let Some(ref b64) = img.b64_json {
                            format!("data:image/png;base64,{}", b64)
                        } else {
                            format!("character_view_{}.png", i)
                        };

                        views.push(CharacterView {
                            angle: angle.replace(" view", "").replace("-", "_"),
                            image,
                            embedding: None,
                        });
                    }
                }
                Err(e) => {
                    eprintln!("生成角色视图失败: {}", e);
                    views.push(CharacterView {
                        angle: angle.replace(" view", "").replace("-", "_"),
                        image: format!("character_view_{}_error.png", i),
                        embedding: None,
                    });
                }
            }
        }

        Ok(views)
    }

    async fn generate_placeholder_character_views(
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
}
