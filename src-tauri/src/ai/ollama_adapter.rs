use super::models::{AIRequest, AIResponse, AIStreamChunk, Usage};
use super::traits::{AIModel, ModelStream};
use crate::logger::Logger;
use futures::stream::{self, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: Option<bool>,
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: Option<f32>,
    num_predict: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OllamaResponse {
    model: String,
    message: OllamaMessageContent,
    done: bool,
    prompt_eval_count: Option<u32>,
    eval_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OllamaMessageContent {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OllamaStreamChunk {
    model: String,
    message: OllamaMessageContent,
    done: bool,
}

pub struct OllamaAdapter {
    base_url: String,
    model: String,
    client: Client,
    logger: Logger,
}

impl OllamaAdapter {
    pub fn new(model: String) -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model,
            client: Client::new(),
            logger: Logger::new().with_feature("ollama-adapter"),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    async fn check_connection(&self) -> Result<(), String> {
        let response = self
            .client
            .get(&format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map_err(|e| {
                let error_str = format!("{}", e);
                self.logger.error(&format!("Failed to connect to Ollama: {}", error_str));
                format!("Connection failed: {}", error_str)
            })?;

        if !response.status().is_success() {
            self.logger.error(&format!("Ollama returned status: {}", response.status()));
            return Err(format!("Ollama returned status: {}", response.status()));
        }

        self.logger.debug("Ollama connection check successful");
        Ok(())
    }

    async fn send_request(&self, request: AIRequest) -> Result<OllamaResponse, String> {
        let ollama_request = OllamaRequest {
            model: self.model.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|m| OllamaMessage {
                    role: m.role,
                    content: m.content,
                })
                .collect(),
            stream: Some(false),
            options: Some(OllamaOptions {
                temperature: request.temperature,
                num_predict: request.max_tokens,
            }),
        };

        self.logger.debug(&format!("Sending request to Ollama: {:?}", ollama_request));

        let response = self
            .client
            .post(&format!("{}/api/chat", self.base_url))
            .json(&ollama_request)
            .send()
            .await
            .map_err(|e| {
                let error_str = format!("{}", e);
                self.logger.error(&format!("Failed to send request to Ollama: {}", error_str));
                format!("Request failed: {}", error_str)
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            self.logger.error(&format!("Ollama API error: {} - {}", status, error_text));
            return Err(format!("Ollama API error: {} - {}", status, error_text));
        }

        response
            .json()
            .await
            .map_err(|e| {
                let error_str = format!("{}", e);
                self.logger.error(&format!("Failed to parse Ollama response: {}", error_str));
                format!("Failed to parse response: {}", error_str)
            })
    }

    async fn parse_stream_chunks(
        response: reqwest::Response,
        logger: Logger,
    ) -> Vec<Result<AIStreamChunk, String>> {
        let mut chunks = Vec::new();
        let mut byte_stream = response.bytes_stream();

        while let Some(chunk_result) = byte_stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    if let Ok(chunk_data) = serde_json::from_slice::<OllamaStreamChunk>(&chunk) {
                        let content = chunk_data.message.content;

                        if !content.is_empty() {
                            logger.debug(&format!("Stream chunk received: {} chars", content.len()));
                            chunks.push(Ok(AIStreamChunk {
                                content,
                                done: false,
                            }));
                        }

                        if chunk_data.done {
                            logger.debug("Ollama stream completed");
                            chunks.push(Ok(AIStreamChunk {
                                content: String::new(),
                                done: true,
                            }));
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

#[async_trait::async_trait]
impl AIModel for OllamaAdapter {
    fn get_name(&self) -> String {
        self.model.clone()
    }

    fn get_provider(&self) -> String {
        "Ollama".to_string()
    }

    async fn complete(&self, request: AIRequest) -> Result<AIResponse, String> {
        self.check_connection().await?;

        self.logger.info(&format!("Starting Ollama completion with model: {}", self.model));

        let response = self.send_request(request).await?;

        let ai_response = AIResponse {
            content: response.message.content.clone(),
            finish_reason: if response.done {
                Some("stop".to_string())
            } else {
                None
            },
            usage: Some(Usage {
                prompt_tokens: response.prompt_eval_count.unwrap_or(0),
                completion_tokens: response.eval_count.unwrap_or(0),
                total_tokens: response
                    .prompt_eval_count
                    .unwrap_or(0)
                    .saturating_add(response.eval_count.unwrap_or(0)),
            }),
        };

        self.logger.info(&format!("Ollama completion successful: {} chars", response.message.content.len()));

        Ok(ai_response)
    }

    async fn complete_stream(&self, request: AIRequest) -> Result<ModelStream, String> {
        self.check_connection().await?;

        self.logger.info(&format!("Starting Ollama stream completion with model: {}", self.model));

        let ollama_request = OllamaRequest {
            model: self.model.clone(),
            messages: request
                .messages
                .into_iter()
                .map(|m| OllamaMessage {
                    role: m.role,
                    content: m.content,
                })
                .collect(),
            stream: Some(true),
            options: Some(OllamaOptions {
                temperature: request.temperature,
                num_predict: request.max_tokens,
            }),
        };

        let client = self.client.clone();
        let base_url = self.base_url.clone();
        let logger = self.logger.clone();

        let response = client
            .post(&format!("{}/api/chat", base_url))
            .json(&ollama_request)
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
            logger.error(&format!("Ollama streaming error: {} - {}", status, error_text));
            return Err(format!("Ollama streaming error: {} - {}", status, error_text));
        }

        let chunks = Self::parse_stream_chunks(response, logger).await;
        let item_stream = stream::iter(chunks);

        Ok(ModelStream::new(Box::new(item_stream)))
    }
}
