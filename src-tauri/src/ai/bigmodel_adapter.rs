use super::models::{AIRequest, AIResponse, AIStreamChunk, Usage};
use super::traits::{AIModel, ModelStream};
use crate::logger::Logger;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct BigModelRequest {
    model: String,
    messages: Vec<BigModelMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    stream: Option<bool>,
}

#[derive(Debug, Serialize)]
struct BigModelMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct BigModelResponse {
    choices: Vec<BigModelChoice>,
    usage: BigModelUsage,
}

#[derive(Debug, Deserialize)]
struct BigModelChoice {
    message: BigModelMessageContent,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BigModelMessageContent {
    content: String,
}

#[derive(Debug, Deserialize)]
struct BigModelUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct BigModelStreamChunk {
    choices: Vec<BigModelStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct BigModelStreamChoice {
    delta: BigModelStreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BigModelStreamDelta {
    content: Option<String>,
}

pub struct BigModelAdapter {
    api_key: String,
    base_url: String,
    model: String,
    client: Client,
    logger: Logger,
}

impl BigModelAdapter {
    pub fn new(api_key: String, model: String) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| Client::new());
        
        Self {
            api_key,
            base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
            model,
            client,
            logger: Logger::new().with_feature("bigmodel-adapter"),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }
}

#[async_trait::async_trait]
impl AIModel for BigModelAdapter {
    fn get_name(&self) -> String {
        self.model.clone()
    }

    fn get_provider(&self) -> String {
        "BigModel".to_string()
    }

    async fn complete(&self, request: AIRequest) -> Result<AIResponse, String> {
        self.logger.info(&format!("Starting BigModel completion with model: {}", self.model));

        let bigmodel_request = BigModelRequest {
            model: self.model.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|m| BigModelMessage {
                    role: m.role,
                    content: m.content,
                })
                .collect(),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: Some(false),
        };

        self.logger.debug(&format!("Sending request to BigModel: {:?}", bigmodel_request));

        let response = self
            .client
            .post(&format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&bigmodel_request)
            .send()
            .await
            .map_err(|e| {
                let error_str = format!("{}", e);
                self.logger.error(&format!("Failed to send request to BigModel: {}", error_str));
                format!("Request failed: {}", error_str)
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            self.logger.error(&format!("BigModel API error: {} - {}", status, error_text));
            return Err(format!("BigModel API error: {} - {}", status, error_text));
        }

        response
            .json()
            .await
            .map_err(|e| {
                let error_str = format!("{}", e);
                self.logger.error(&format!("Failed to parse BigModel response: {}", error_str));
                format!("Failed to parse response: {}", error_str)
            })
            .and_then(|response: BigModelResponse| {
                let choice = response.choices.first().ok_or_else(|| {
                    self.logger.error("BigModel response has no choices");
                    "No choices in response".to_string()
                })?;

                let ai_response = AIResponse {
                    content: choice.message.content.clone(),
                    finish_reason: choice.finish_reason.clone(),
                    usage: Some(Usage {
                        prompt_tokens: response.usage.prompt_tokens,
                        completion_tokens: response.usage.completion_tokens,
                        total_tokens: response.usage.total_tokens,
                    }),
                };

                self.logger.info(&format!("BigModel completion successful: {} chars", choice.message.content.len()));

                Ok(ai_response)
            })
    }

    async fn complete_stream(&self, request: AIRequest) -> Result<ModelStream, String> {
        self.logger.info(&format!("Starting BigModel stream completion with model: {}", self.model));

        let bigmodel_request = BigModelRequest {
            model: self.model.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|m| BigModelMessage {
                    role: m.role,
                    content: m.content,
                })
                .collect(),
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: Some(true),
        };

        let client = self.client.clone();
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();
        let logger = self.logger.clone();

        let response = client
            .post(&format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&bigmodel_request)
            .send()
            .await
            .map_err(|e| {
                let error_str = format!("{}", e);
                logger.error(&format!("Failed to send streaming request: {}", error_str));
                format!("Stream request failed: {}", error_str)
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            logger.error(&format!("BigModel streaming error: {} - {}", status, error_text));
            return Err(format!("BigModel streaming error: {} - {}", status, error_text));
        }

        let chunks = Self::parse_stream_chunks(response, logger).await;
        let item_stream = stream::iter(chunks);

        Ok(ModelStream::new(Box::new(item_stream)))
    }
}

impl BigModelAdapter {
    async fn parse_stream_chunks(
        response: reqwest::Response,
        logger: Logger,
    ) -> Vec<Result<AIStreamChunk, String>> {
        let mut chunks = Vec::new();
        let mut byte_stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = byte_stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let text = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&text);

                    let lines: Vec<String> = buffer.split('\n').map(|s| s.to_string()).collect();
                    buffer = lines.last().cloned().unwrap_or_default();

                    for line in lines.iter().take(lines.len() - 1) {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

                        if !line.starts_with("data: ") {
                            continue;
                        }

                        let json_str = &line[6..];
                        if json_str == "[DONE]" {
                            chunks.push(Ok(AIStreamChunk {
                                content: String::new(),
                                done: true,
                            }));
                            return chunks;
                        }

                        if let Ok(chunk_data) = serde_json::from_str::<BigModelStreamChunk>(json_str) {
                            if let Some(choice) = chunk_data.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    if !content.is_empty() {
                                        logger.debug(&format!("Stream chunk received: {} chars", content.len()));
                                        chunks.push(Ok(AIStreamChunk {
                                            content: content.clone(),
                                            done: false,
                                        }));
                                    }
                                }

                                if choice.finish_reason.is_some() {
                                    chunks.push(Ok(AIStreamChunk {
                                        content: String::new(),
                                        done: true,
                                    }));
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let error_str = format!("{}", e);
                    logger.error(&format!("Failed to read stream chunk: {}", error_str));
                    chunks.push(Err(format!("Failed to read chunk: {}", error_str)));
                }
            }
        }

        chunks
    }
}
