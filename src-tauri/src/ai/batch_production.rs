use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::scene_manager::{SceneManager, ScriptScene, CreateSceneRequest, SceneStatistics};
use super::script_parser::{ScriptParser, ParsedScene, ParsedScreenplay};
use super::prompt_compiler::{PromptCompiler, AIScene, AICharacter, GenerationConfig};
use super::character_bible::CharacterBibleManager;
use super::task_queue::{TaskQueue, CreateTaskRequest, QueuedTask, TaskType, TaskPriority};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProductionConfig {
    pub image_provider: Option<String>,
    pub video_provider: Option<String>,
    pub style_tokens: Vec<String>,
    pub quality_tokens: Vec<String>,
    pub max_concurrent_tasks: i32,
    pub retry_failed_tasks: bool,
}

impl Default for BatchProductionConfig {
    fn default() -> Self {
        Self {
            image_provider: Some("openai".to_string()),
            video_provider: None,
            style_tokens: vec!["cinematic".to_string(), "dramatic lighting".to_string()],
            quality_tokens: vec!["high quality".to_string(), "detailed".to_string()],
            max_concurrent_tasks: 3,
            retry_failed_tasks: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProductionJob {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub status: BatchJobStatus,
    pub total_scenes: i32,
    pub completed_scenes: i32,
    pub failed_scenes: i32,
    pub config: BatchProductionConfig,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BatchJobStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionProgress {
    pub job_id: String,
    pub current_scene: i32,
    pub total_scenes: i32,
    pub current_status: String,
    pub percentage: f32,
    pub estimated_remaining_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBatchJobRequest {
    pub project_id: String,
    pub name: String,
    pub source_type: BatchSourceType,
    pub source_content: Option<String>,
    pub chapter_ids: Option<Vec<String>>,
    pub scene_count: Option<i32>,
    pub config: Option<BatchProductionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchSourceType {
    NovelText,
    AiGenerated,
    ChapterContent,
    ExistingScenes,
}

pub struct BatchProductionManager {
    jobs: Arc<RwLock<HashMap<String, BatchProductionJob>>>,
    progress: Arc<RwLock<HashMap<String, ProductionProgress>>>,
}

impl BatchProductionManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            progress: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_job(&self, request: CreateBatchJobRequest) -> BatchProductionJob {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let job = BatchProductionJob {
            id: id.clone(),
            project_id: request.project_id,
            name: request.name,
            status: BatchJobStatus::Pending,
            total_scenes: 0,
            completed_scenes: 0,
            failed_scenes: 0,
            config: request.config.unwrap_or_default(),
            created_at: now.clone(),
            updated_at: now,
        };

        let mut jobs = self.jobs.write().await;
        jobs.insert(id.clone(), job.clone());

        let progress = ProductionProgress {
            job_id: id.clone(),
            current_scene: 0,
            total_scenes: 0,
            current_status: "initialized".to_string(),
            percentage: 0.0,
            estimated_remaining_seconds: None,
        };

        let mut prog = self.progress.write().await;
        prog.insert(id, progress);

        job
    }

    pub async fn get_job(&self, id: &str) -> Option<BatchProductionJob> {
        let jobs = self.jobs.read().await;
        jobs.get(id).cloned()
    }

    pub async fn get_project_jobs(&self, project_id: &str) -> Vec<BatchProductionJob> {
        let jobs = self.jobs.read().await;
        jobs.values()
            .filter(|j| j.project_id == project_id)
            .cloned()
            .collect()
    }

    pub async fn update_job_status(&self, id: &str, status: BatchJobStatus) -> Option<BatchProductionJob> {
        let mut jobs = self.jobs.write().await;
        if let Some(job) = jobs.get_mut(id) {
            job.status = status;
            job.updated_at = Utc::now().to_rfc3339();
            return Some(job.clone());
        }
        None
    }

    pub async fn get_progress(&self, job_id: &str) -> Option<ProductionProgress> {
        let progress = self.progress.read().await;
        progress.get(job_id).cloned()
    }

    pub async fn update_progress(&self, job_id: &str, current: i32, total: i32, status: &str) {
        let mut progress = self.progress.write().await;
        if let Some(prog) = progress.get_mut(job_id) {
            prog.current_scene = current;
            prog.total_scenes = total;
            prog.current_status = status.to_string();
            prog.percentage = if total > 0 {
                (current as f32 / total as f32) * 100.0
            } else {
                0.0
            };
        }
    }

    pub async fn prepare_scenes_from_text(
        &self,
        text: &str,
        scene_count: i32,
    ) -> Result<Vec<CreateSceneRequest>, String> {
        let parser = ScriptParser::new();
        let options = super::script_parser::ScriptParseOptions {
            scene_count: Some(scene_count),
            ..Default::default()
        };

        let screenplay = parser.parse_novel_to_scenes(text, &options)?;
        
        Ok(screenplay.scenes.into_iter().enumerate().map(|(idx, scene)| {
            CreateSceneRequest {
                project_id: "".to_string(),
                chapter_id: None,
                scene_index: idx as i32,
                narration: scene.narration,
                visual_content: scene.visual_content,
                action: scene.action,
                camera: scene.camera,
                character_description: scene.character_description,
            }
        }).collect())
    }

    pub async fn prepare_scenes_from_ai_response(
        &self,
        json_response: &str,
    ) -> Result<Vec<CreateSceneRequest>, String> {
        let parser = ScriptParser::new();
        let screenplay = parser.parse_ai_response(json_response)?;
        
        Ok(screenplay.scenes.into_iter().enumerate().map(|(idx, scene)| {
            CreateSceneRequest {
                project_id: "".to_string(),
                chapter_id: None,
                scene_index: idx as i32,
                narration: scene.narration,
                visual_content: scene.visual_content,
                action: scene.action,
                camera: scene.camera,
                character_description: scene.character_description,
            }
        }).collect())
    }

    pub async fn generate_prompts_for_scenes(
        &self,
        scenes: &[ScriptScene],
        characters: &[super::character_bible::CharacterBible],
        config: &BatchProductionConfig,
    ) -> Vec<(String, String)> {
        let compiler = PromptCompiler::new();
        let ai_characters: Vec<AICharacter> = characters.iter().map(|c| {
            AICharacter {
                id: c.id.clone(),
                name: c.name.clone(),
                char_type: c.char_type.clone(),
                visual_traits: c.visual_traits.clone(),
                style_tokens: c.style_tokens.clone(),
                color_palette: c.color_palette.clone(),
            }
        }).collect();

        let gen_config = GenerationConfig {
            style_tokens: config.style_tokens.clone(),
            quality_tokens: config.quality_tokens.clone(),
        };

        let mut prompts = Vec::new();
        
        for scene in scenes {
            let ai_scene = AIScene {
                scene_id: scene.scene_index,
                narration: scene.narration.clone(),
                visual_content: scene.visual_content.clone(),
                action: scene.action.clone(),
                camera: scene.camera.clone(),
                character_description: scene.character_description.clone(),
            };

            if let Ok(prompt) = compiler.compile_scene_image_prompt(&ai_scene, &ai_characters, &gen_config) {
                prompts.push((scene.id.clone(), prompt));
            }
        }

        prompts
    }

    pub async fn create_tasks_from_scenes(
        &self,
        scenes: &[ScriptScene],
        prompts: &[(String, String)],
        config: &BatchProductionConfig,
    ) -> Vec<QueuedTask> {
        let mut queue = TaskQueue::with_max_concurrent(config.max_concurrent_tasks as usize);
        let mut tasks = Vec::new();

        for (scene_id, prompt) in prompts {
            if let Ok(task) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                queue.add_task(CreateTaskRequest {
                    project_id: scenes.iter()
                        .find(|s| &s.id == scene_id)
                        .map(|s| s.project_id.clone())
                        .unwrap_or_default(),
                    task_type: TaskType::ImageGeneration,
                    priority: Some(TaskPriority::Normal),
                    provider: config.image_provider.clone(),
                    input_data: serde_json::json!({
                        "scene_id": scene_id,
                        "prompt": prompt,
                    }),
                    max_retries: Some(3),
                })
            })) {
                tasks.push(task);
            }
        }

        tasks
    }

    pub async fn cancel_job(&self, id: &str) -> Option<BatchProductionJob> {
        self.update_job_status(id, BatchJobStatus::Cancelled).await
    }

    pub async fn pause_job(&self, id: &str) -> Option<BatchProductionJob> {
        self.update_job_status(id, BatchJobStatus::Paused).await
    }

    pub async fn resume_job(&self, id: &str) -> Option<BatchProductionJob> {
        self.update_job_status(id, BatchJobStatus::Running).await
    }

    pub async fn delete_job(&self, id: &str) -> bool {
        let mut jobs = self.jobs.write().await;
        let mut progress = self.progress.write().await;
        
        progress.remove(id);
        jobs.remove(id).is_some()
    }

    pub async fn get_job_statistics(&self) -> HashMap<String, i32> {
        let jobs = self.jobs.read().await;
        let mut stats = HashMap::new();
        
        let mut pending = 0;
        let mut running = 0;
        let mut completed = 0;
        let mut failed = 0;
        let mut cancelled = 0;

        for job in jobs.values() {
            match job.status {
                BatchJobStatus::Pending => pending += 1,
                BatchJobStatus::Running => running += 1,
                BatchJobStatus::Paused => pending += 1,
                BatchJobStatus::Completed => completed += 1,
                BatchJobStatus::Failed => failed += 1,
                BatchJobStatus::Cancelled => cancelled += 1,
            }
        }

        stats.insert("pending".to_string(), pending);
        stats.insert("running".to_string(), running);
        stats.insert("completed".to_string(), completed);
        stats.insert("failed".to_string(), failed);
        stats.insert("cancelled".to_string(), cancelled);
        stats.insert("total".to_string(), jobs.len() as i32);

        stats
    }
}

impl Default for BatchProductionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub async fn create_batch_production_job(
    request: CreateBatchJobRequest,
) -> Result<BatchProductionJob, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.create_job(request).await)
}

#[tauri::command]
pub async fn get_batch_production_job(id: String) -> Result<Option<BatchProductionJob>, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.get_job(&id).await)
}

#[tauri::command]
pub async fn get_project_batch_jobs(project_id: String) -> Result<Vec<BatchProductionJob>, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.get_project_jobs(&project_id).await)
}

#[tauri::command]
pub async fn cancel_batch_job(id: String) -> Result<Option<BatchProductionJob>, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.cancel_job(&id).await)
}

#[tauri::command]
pub async fn pause_batch_job(id: String) -> Result<Option<BatchProductionJob>, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.pause_job(&id).await)
}

#[tauri::command]
pub async fn resume_batch_job(id: String) -> Result<Option<BatchProductionJob>, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.resume_job(&id).await)
}

#[tauri::command]
pub async fn get_batch_job_progress(id: String) -> Result<Option<ProductionProgress>, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.get_progress(&id).await)
}

#[tauri::command]
pub async fn prepare_scenes_from_novel(
    text: String,
    scene_count: i32,
) -> Result<Vec<CreateSceneRequest>, String> {
    let manager = BatchProductionManager::new();
    manager.prepare_scenes_from_text(&text, scene_count).await
}

#[tauri::command]
pub async fn prepare_scenes_from_ai(
    json_response: String,
) -> Result<Vec<CreateSceneRequest>, String> {
    let manager = BatchProductionManager::new();
    manager.prepare_scenes_from_ai_response(&json_response).await
}

#[tauri::command]
pub async fn get_batch_job_statistics() -> Result<HashMap<String, i32>, String> {
    let manager = BatchProductionManager::new();
    Ok(manager.get_job_statistics().await)
}
