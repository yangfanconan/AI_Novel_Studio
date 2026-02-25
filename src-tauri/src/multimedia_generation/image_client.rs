use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageProviderConfig {
    pub id: String,
    pub name: String,
    pub api_key: String,
    pub api_base: String,
    pub model: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: i32,
    pub height: i32,
    pub steps: Option<i32>,
    pub cfg_scale: Option<f32>,
    pub seed: Option<i64>,
    pub num_images: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGenerationResponse {
    pub images: Vec<GeneratedImage>,
    pub created: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    pub url: Option<String>,
    pub b64_json: Option<String>,
    pub revised_prompt: Option<String>,
}

pub struct ImageClient {
    http_client: reqwest::Client,
}

impl ImageClient {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn generate_image(
        &self,
        config: &ImageProviderConfig,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, String> {
        match config.id.as_str() {
            "openai" => self.generate_with_openai(config, request).await,
            "stability" => self.generate_with_stability(config, request).await,
            "comfyui" => self.generate_with_comfyui(config, request).await,
            _ => Err(format!("Unknown provider: {}", config.id)),
        }
    }

    async fn generate_with_openai(
        &self,
        config: &ImageProviderConfig,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, String> {
        let url = format!("{}/images/generations", config.api_base);
        
        let body = serde_json::json!({
            "model": config.model,
            "prompt": request.prompt,
            "n": request.num_images.unwrap_or(1),
            "size": format!("{}x{}", request.width, request.height),
            "response_format": "url"
        });

        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API错误: {}", error_text));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        let images = json["data"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        Some(GeneratedImage {
                            url: item["url"].as_str().map(String::from),
                            b64_json: item["b64_json"].as_str().map(String::from),
                            revised_prompt: item["revised_prompt"].as_str().map(String::from),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(ImageGenerationResponse {
            images,
            created: chrono::Utc::now().timestamp(),
        })
    }

    async fn generate_with_stability(
        &self,
        config: &ImageProviderConfig,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, String> {
        let url = format!(
            "{}/v1/generation/{}/text-to-image",
            config.api_base, config.model
        );

        let body = serde_json::json!({
            "text_prompts": [
                {
                    "text": request.prompt,
                    "weight": 1.0
                },
                {
                    "text": request.negative_prompt.unwrap_or_default(),
                    "weight": -1.0
                }
            ],
            "cfg_scale": request.cfg_scale.unwrap_or(7.0),
            "height": request.height,
            "width": request.width,
            "steps": request.steps.unwrap_or(30),
            "samples": request.num_images.unwrap_or(1),
        });

        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API错误: {}", error_text));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        let images = json["artifacts"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        Some(GeneratedImage {
                            url: None,
                            b64_json: item["base64"].as_str().map(String::from),
                            revised_prompt: None,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(ImageGenerationResponse {
            images,
            created: chrono::Utc::now().timestamp(),
        })
    }

    async fn generate_with_comfyui(
        &self,
        config: &ImageProviderConfig,
        request: ImageGenerationRequest,
    ) -> Result<ImageGenerationResponse, String> {
        let url = format!("{}/prompt", config.api_base);

        let workflow = serde_json::json!({
            "3": {
                "class_type": "KSampler",
                "inputs": {
                    "seed": request.seed.unwrap_or_else(|| chrono::Utc::now().timestamp_millis()),
                    "steps": request.steps.unwrap_or(20),
                    "cfg": request.cfg_scale.unwrap_or(7.0),
                    "sampler_name": "euler",
                    "scheduler": "normal",
                    "denoise": 1.0,
                    "model": ["4", 0],
                    "positive": ["6", 0],
                    "negative": ["7", 0],
                    "latent_image": ["5", 0]
                }
            },
            "4": {
                "class_type": "CheckpointLoaderSimple",
                "inputs": {
                    "ckpt_name": config.model
                }
            },
            "5": {
                "class_type": "EmptyLatentImage",
                "inputs": {
                    "width": request.width,
                    "height": request.height,
                    "batch_size": request.num_images.unwrap_or(1)
                }
            },
            "6": {
                "class_type": "CLIPTextEncode",
                "inputs": {
                    "text": request.prompt,
                    "clip": ["4", 1]
                }
            },
            "7": {
                "class_type": "CLIPTextEncode",
                "inputs": {
                    "text": request.negative_prompt.unwrap_or_default(),
                    "clip": ["4", 1]
                }
            },
            "8": {
                "class_type": "VAEDecode",
                "inputs": {
                    "samples": ["3", 0],
                    "vae": ["4", 2]
                }
            },
            "9": {
                "class_type": "SaveImage",
                "inputs": {
                    "filename_prefix": "InfiniteNote",
                    "images": ["8", 0]
                }
            }
        });

        let response = self.http_client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&workflow)
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API错误: {}", error_text));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("解析响应失败: {}", e))?;

        let prompt_id = json["prompt_id"]
            .as_str()
            .ok_or("Missing prompt_id in response")?;

        let images = vec![GeneratedImage {
            url: Some(format!("{}/view?filename=InfiniteNote_{}.png", config.api_base, prompt_id)),
            b64_json: None,
            revised_prompt: None,
        }];

        Ok(ImageGenerationResponse {
            images,
            created: chrono::Utc::now().timestamp(),
        })
    }

    pub fn parse_aspect_ratio(aspect_ratio: &str) -> (i32, i32) {
        match aspect_ratio {
            "1:1" => (512, 512),
            "16:9" => (768, 432),
            "9:16" => (432, 768),
            "4:3" => (640, 480),
            "3:4" => (480, 640),
            "2:3" => (512, 768),
            "3:2" => (768, 512),
            _ => (512, 512),
        }
    }
}

impl Default for ImageClient {
    fn default() -> Self {
        Self::new()
    }
}
