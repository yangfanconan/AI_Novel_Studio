use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfyUIConfig {
    pub server_url: String,
    pub client_id: Option<String>,
    pub timeout_seconds: Option<u32>,
}

impl Default for ComfyUIConfig {
    fn default() -> Self {
        Self {
            server_url: "http://127.0.0.1:8188".to_string(),
            client_id: Some(Uuid::new_v4().to_string()),
            timeout_seconds: Some(600),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowNode {
    pub id: i32,
    #[serde(rename = "type")]
    pub node_type: String,
    pub pos: Vec<f32>,
    pub size: Vec<f32>,
    pub flags: HashMap<String, serde_json::Value>,
    pub order: i32,
    pub mode: i32,
    pub inputs: Vec<WorkflowInput>,
    pub outputs: Vec<WorkflowOutput>,
    pub properties: HashMap<String, serde_json::Value>,
    pub widgets_values: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInput {
    pub name: String,
    #[serde(rename = "type")]
    pub input_type: String,
    pub link: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOutput {
    pub name: String,
    #[serde(rename = "type")]
    pub output_type: String,
    pub links: Option<Vec<i32>>,
    pub slot_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowLink {
    pub id: i32,
    #[serde(rename = "type")]
    pub link_type: String,
    pub from_node: i32,
    pub from_slot: i32,
    pub to_node: i32,
    pub to_slot: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfyUIWorkflow {
    pub last_node_id: i32,
    pub last_link_id: i32,
    pub nodes: Vec<WorkflowNode>,
    pub links: Vec<WorkflowLink>,
}

impl ComfyUIWorkflow {
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("Failed to parse workflow: {}", e))
    }

    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Failed to serialize workflow: {}", e))
    }

    pub fn to_prompt(&self) -> HashMap<String, serde_json::Value> {
        let mut prompt: HashMap<String, serde_json::Value> = HashMap::new();
        
        for node in &self.nodes {
            let mut node_data = HashMap::new();
            node_data.insert("class_type".to_string(), serde_json::json!(node.node_type));
            
            let mut inputs = HashMap::new();
            for (key, value) in &node.properties {
                inputs.insert(key.clone(), value.clone());
            }
            
            for (idx, widget) in node.widgets_values.iter().enumerate() {
                inputs.insert(format!("_widget_{}", idx), widget.clone());
            }

            for input in &node.inputs {
                if let Some(link_id) = input.link {
                    if let Some(link) = self.links.iter().find(|l| l.id == link_id) {
                        inputs.insert(
                            input.name.clone(),
                            serde_json::json!([format!("{}", link.from_node), link.from_slot]),
                        );
                    }
                }
            }

            node_data.insert("inputs".to_string(), serde_json::json!(inputs));
            prompt.insert(format!("{}", node.id), serde_json::json!(node_data));
        }

        prompt
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRequest {
    pub prompt: HashMap<String, serde_json::Value>,
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptResponse {
    pub prompt_id: String,
    pub number: i32,
    pub node_errors: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub queue_running: Vec<QueueItem>,
    pub queue_pending: Vec<QueueItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub number: i32,
    pub prompt_id: String,
    pub outputs: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStatus {
    pub status: String,
    pub prompt_id: Option<String>,
    pub node: Option<String>,
    pub progress: Option<f32>,
    pub outputs: Option<HashMap<String, HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    pub filename: String,
    pub subfolder: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub url: Option<String>,
    pub base64_data: Option<String>,
}

pub struct ComfyUIClient {
    config: Arc<RwLock<ComfyUIConfig>>,
    http_client: reqwest::Client,
}

impl ComfyUIClient {
    pub fn new(config: ComfyUIConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(600))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(ComfyUIConfig::default())
    }

    pub async fn update_config(&self, new_config: ComfyUIConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    pub async fn get_config(&self) -> ComfyUIConfig {
        self.config.read().await.clone()
    }

    pub async fn check_connection(&self) -> Result<bool, String> {
        let config = self.config.read().await;
        let url = format!("{}/system_stats", config.server_url);
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Connection failed: {}", e))?;

        Ok(response.status().is_success())
    }

    pub async fn queue_prompt(
        &self,
        workflow: &ComfyUIWorkflow,
    ) -> Result<PromptResponse, String> {
        let config = self.config.read().await;
        let url = format!("{}/prompt", config.server_url);

        let prompt = workflow.to_prompt();
        let request = PromptRequest {
            prompt,
            client_id: config.client_id.clone(),
        };

        let response = self.http_client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to queue prompt: {}", e))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Queue prompt failed: {}", error_text));
        }

        response.json::<PromptResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }

    pub async fn get_queue_status(&self) -> Result<QueueStatus, String> {
        let config = self.config.read().await;
        let url = format!("{}/queue", config.server_url);

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get queue status: {}", e))?;

        response.json::<QueueStatus>()
            .await
            .map_err(|e| format!("Failed to parse queue status: {}", e))
    }

    pub async fn get_history(&self, prompt_id: &str) -> Result<serde_json::Value, String> {
        let config = self.config.read().await;
        let url = format!("{}/history/{}", config.server_url, prompt_id);

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get history: {}", e))?;

        response.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse history: {}", e))
    }

    pub async fn get_image(
        &self,
        filename: &str,
        subfolder: &str,
        image_type: &str,
    ) -> Result<Vec<u8>, String> {
        let config = self.config.read().await;
        let url = format!(
            "{}/view?filename={}&subfolder={}&type={}",
            config.server_url,
            urlencoding::encode(filename),
            urlencoding::encode(subfolder),
            image_type
        );

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get image: {}", e))?;

        response.bytes()
            .await
            .map(|b| b.to_vec())
            .map_err(|e| format!("Failed to read image data: {}", e))
    }

    pub async fn upload_image(
        &self,
        image_data: Vec<u8>,
        filename: &str,
        overwrite: bool,
    ) -> Result<String, String> {
        use reqwest::multipart;
        let config = self.config.read().await;
        let url = format!("{}/upload/image", config.server_url);

        let part = multipart::Part::bytes(image_data)
            .file_name(filename.to_string())
            .mime_str("image/png")
            .map_err(|e| format!("Failed to create multipart: {}", e))?;

        let form = multipart::Form::new()
            .part("image", part)
            .text("overwrite", overwrite.to_string());

        let response = self.http_client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Failed to upload image: {}", e))?;

        let result: serde_json::Value = response.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse upload response: {}", e))?;

        result.get("name")
            .and_then(|v: &serde_json::Value| v.as_str())
            .map(|s| s.to_string())
            .ok_or("Failed to get uploaded filename".to_string())
    }

    pub async fn wait_for_completion(
        &self,
        prompt_id: &str,
        timeout_seconds: u32,
    ) -> Result<Vec<GeneratedImage>, String> {
        let start = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(timeout_seconds as u64);
        let poll_interval = std::time::Duration::from_secs(2);

        loop {
            if start.elapsed() > timeout {
                return Err("Timeout waiting for completion".to_string());
            }

            let history = self.get_history(prompt_id).await?;
            
            if let Some(prompt_history) = history.get(prompt_id) {
                if let Some(outputs) = prompt_history.get("outputs") {
                    let mut images = Vec::new();
                    
                    for (_node_id, node_output) in outputs.as_object().unwrap_or(&serde_json::Map::new()) {
                        if let Some(images_array) = node_output.get("images").and_then(|v| v.as_array()) {
                            for img in images_array {
                                if let (Some(filename), Some(subfolder), Some(img_type)) = (
                                    img.get("filename").and_then(|v| v.as_str()),
                                    img.get("subfolder").and_then(|v| v.as_str()),
                                    img.get("type").and_then(|v| v.as_str()),
                                ) {
                                    images.push(GeneratedImage {
                                        filename: filename.to_string(),
                                        subfolder: subfolder.to_string(),
                                        image_type: img_type.to_string(),
                                        url: None,
                                        base64_data: None,
                                    });
                                }
                            }
                        }
                    }

                    if !images.is_empty() {
                        return Ok(images);
                    }
                }

                if prompt_history.get("status").is_some() {
                    let status = prompt_history.get("status").unwrap();
                    if status.get("completed").and_then(|v| v.as_bool()).unwrap_or(false) {
                        return Ok(Vec::new());
                    }
                }
            }

            tokio::time::sleep(poll_interval).await;
        }
    }

    pub async fn interrupt(&self) -> Result<(), String> {
        let config = self.config.read().await;
        let url = format!("{}/interrupt", config.server_url);

        self.http_client
            .post(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to interrupt: {}", e))?;

        Ok(())
    }

    pub async fn clear_queue(&self) -> Result<(), String> {
        let config = self.config.read().await;
        let url = format!("{}/queue", config.server_url);

        let body = serde_json::json!({ "clear": true });

        self.http_client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to clear queue: {}", e))?;

        Ok(())
    }

    pub async fn get_object_info(&self) -> Result<serde_json::Value, String> {
        let config = self.config.read().await;
        let url = format!("{}/object_info", config.server_url);

        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to get object info: {}", e))?;

        response.json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse object info: {}", e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfyUIGenerationRequest {
    pub workflow_json: String,
    pub wait_for_completion: Option<bool>,
    pub timeout_seconds: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComfyUIGenerationResult {
    pub prompt_id: String,
    pub status: String,
    pub images: Vec<GeneratedImage>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn comfyui_check_connection(config: Option<ComfyUIConfig>) -> Result<bool, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    client.check_connection().await
}

#[tauri::command]
pub async fn comfyui_queue_prompt(
    workflow_json: String,
    config: Option<ComfyUIConfig>,
) -> Result<PromptResponse, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    let workflow = ComfyUIWorkflow::from_json(&workflow_json)?;
    client.queue_prompt(&workflow).await
}

#[tauri::command]
pub async fn comfyui_get_queue_status(config: Option<ComfyUIConfig>) -> Result<QueueStatus, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    client.get_queue_status().await
}

#[tauri::command]
pub async fn comfyui_wait_for_completion(
    prompt_id: String,
    timeout_seconds: Option<u32>,
    config: Option<ComfyUIConfig>,
) -> Result<Vec<GeneratedImage>, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    client.wait_for_completion(&prompt_id, timeout_seconds.unwrap_or(600)).await
}

#[tauri::command]
pub async fn comfyui_generate_image(
    request: ComfyUIGenerationRequest,
    config: Option<ComfyUIConfig>,
) -> Result<ComfyUIGenerationResult, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    let workflow = ComfyUIWorkflow::from_json(&request.workflow_json)?;

    let prompt_response = client.queue_prompt(&workflow).await?;
    let prompt_id = prompt_response.prompt_id;

    if request.wait_for_completion.unwrap_or(true) {
        let timeout = request.timeout_seconds.unwrap_or(600);
        match client.wait_for_completion(&prompt_id, timeout).await {
            Ok(images) => Ok(ComfyUIGenerationResult {
                prompt_id,
                status: "completed".to_string(),
                images,
                error: None,
            }),
            Err(e) => Ok(ComfyUIGenerationResult {
                prompt_id,
                status: "failed".to_string(),
                images: vec![],
                error: Some(e),
            }),
        }
    } else {
        Ok(ComfyUIGenerationResult {
            prompt_id,
            status: "queued".to_string(),
            images: vec![],
            error: None,
        })
    }
}

#[tauri::command]
pub async fn comfyui_get_image_base64(
    filename: String,
    subfolder: String,
    image_type: String,
    config: Option<ComfyUIConfig>,
) -> Result<String, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    let image_data = client.get_image(&filename, &subfolder, &image_type).await?;
    Ok(base64::encode(&image_data))
}

#[tauri::command]
pub async fn comfyui_upload_image(
    image_base64: String,
    filename: String,
    config: Option<ComfyUIConfig>,
) -> Result<String, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    let image_data = base64::decode(&image_base64)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    client.upload_image(image_data, &filename, true).await
}

#[tauri::command]
pub async fn comfyui_interrupt(config: Option<ComfyUIConfig>) -> Result<(), String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    client.interrupt().await
}

#[tauri::command]
pub async fn comfyui_clear_queue(config: Option<ComfyUIConfig>) -> Result<(), String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    client.clear_queue().await
}

#[tauri::command]
pub async fn comfyui_get_object_info(config: Option<ComfyUIConfig>) -> Result<serde_json::Value, String> {
    let client = ComfyUIClient::new(config.unwrap_or_default());
    client.get_object_info().await
}
