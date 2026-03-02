use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;

use crate::models::{Chapter, ChapterMission, ChapterVersion, ChapterEvaluation};
use crate::services::writer_context_builder::WriterContextBuilder;
use crate::services::guardrails_service::GuardrailsService;
use crate::services::vector_store_service::VectorStoreService;
use crate::ai::service::AIService;
use crate::logger::Logger;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowConfig {
    pub skip_mission_generation: bool,
    pub skip_guardrails_check: bool,
    pub skip_ai_review: bool,
    pub skip_vectorization: bool,
    pub skip_summary_generation: bool,
    pub version_count: u32,
    pub style_hints: Vec<String>,
}

impl Default for FlowConfig {
    fn default() -> Self {
        Self {
            skip_mission_generation: false,
            skip_guardrails_check: false,
            skip_ai_review: false,
            skip_vectorization: false,
            skip_summary_generation: false,
            version_count: 3,
            style_hints: vec![
                "情绪更细腻，节奏更慢，多写内心戏和感官描写".to_string(),
                "冲突更强，节奏更快，多写动作和对话".to_string(),
                "悬念更重，多埋伏笔，结尾钩子更强".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub chapter_id: String,
    pub versions: Vec<ChapterVersion>,
    pub evaluation: Option<ChapterEvaluation>,
    pub guardrails_result: Option<GuardrailsResult>,
    pub recommended_version: Option<u32>,
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardrailsResult {
    pub passed: bool,
    pub violations: Vec<String>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineProgress {
    pub step: String,
    pub progress: f32,
    pub message: String,
}

pub type ProgressCallback = Arc<RwLock<Option<Box<dyn Fn(PipelineProgress) + Send + Sync>>>>;

pub struct PipelineOrchestrator {
    ai_service: AIService,
    writer_context_builder: WriterContextBuilder,
    guardrails: GuardrailsService,
    vector_store: VectorStoreService,
    logger: Logger,
    progress_callback: ProgressCallback,
}

impl PipelineOrchestrator {
    pub fn new() -> Self {
        Self {
            ai_service: AIService::new(),
            writer_context_builder: WriterContextBuilder::new(),
            guardrails: GuardrailsService::new(),
            vector_store: VectorStoreService::new(),
            logger: Logger::new().with_feature("pipeline_orchestrator"),
            progress_callback: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_progress_callback<F>(mut self, callback: F) -> Self 
    where 
        F: Fn(PipelineProgress) + Send + Sync + 'static 
    {
        self.progress_callback = Arc::new(RwLock::new(Some(Box::new(callback))));
        self
    }

    async fn report_progress(&self, step: &str, progress: f32, message: &str) {
        if let Some(callback) = self.progress_callback.read().await.as_ref() {
            callback(PipelineProgress {
                step: step.to_string(),
                progress,
                message: message.to_string(),
            });
        }
    }

    pub async fn generate_chapter(
        &self,
        project_id: &str,
        chapter_id: &str,
        mission: Option<ChapterMission>,
        config: FlowConfig,
    ) -> Result<GenerationResult, String> {
        self.logger.info(&format!("Starting pipeline for chapter: {}", chapter_id));
        
        self.report_progress("init", 0.0, "初始化生成流程").await;

        let mut versions = Vec::new();
        let mut evaluation = None;
        let mut guardrails_result = None;
        let mut recommended_version = None;

        self.report_progress("context", 0.1, "收集上下文信息").await;

        let writer_context = self.writer_context_builder
            .build_visibility_context(project_id, chapter_id, mission.clone())
            .await
            .map_err(|e| {
                self.logger.error(&format!("Failed to build context: {}", e));
                format!("构建上下文失败: {}", e)
            })?;

        self.report_progress("generation", 0.2, "开始生成多版本内容").await;

        let version_count = config.version_count.max(1).min(5) as usize;
        let style_hints = if config.style_hints.is_empty() {
            FlowConfig::default().style_hints
        } else {
            config.style_hints
        };

        for i in 0..version_count {
            let progress = 0.2 + (i as f32 / version_count as f32) * 0.4;
            self.report_progress("generation", progress, &format!("生成版本 {}/{}", i + 1, version_count)).await;

            let style_hint = style_hints.get(i % style_hints.len()).cloned().unwrap_or_default();
            
            let content = self.ai_service
                .complete("default", &writer_context.system_prompt, &writer_context.user_prompt, None)
                .await
                .map_err(|e| {
                    self.logger.error(&format!("AI generation failed for version {}: {}", i, e));
                    format!("AI生成失败 (版本 {}): {}", i + 1, e)
                })?;

            let version = ChapterVersion {
                id: uuid::Uuid::new_v4().to_string(),
                chapter_id: chapter_id.to_string(),
                version_number: (i + 1) as i32,
                content: content.trim().to_string(),
                style_hint: Some(style_hint),
                created_at: Utc::now().to_rfc3339(),
                selected: false,
            };
            versions.push(version);
        }

        if !config.skip_guardrails_check {
            self.report_progress("guardrails", 0.65, "执行护栏检查").await;
            
            if let Some(first_version) = versions.first() {
                let check_result = self.guardrails
                    .check_chapter(&first_version.content, mission.as_ref())
                    .await
                    .map_err(|e| {
                        self.logger.error(&format!("Guardrails check failed: {}", e));
                        format!("护栏检查失败: {}", e)
                    })?;

                guardrails_result = Some(GuardrailsResult {
                    passed: check_result.passed,
                    violations: check_result.violations.iter()
                        .map(|v| format!("{:?}", v))
                        .collect(),
                    suggestions: check_result.suggestions,
                });
            }
        }

        if !config.skip_ai_review && versions.len() > 1 {
            self.report_progress("review", 0.75, "AI评审多版本内容").await;
            
            let contents: Vec<&str> = versions.iter().map(|v| v.content.as_str()).collect();
            
            match self.ai_service.review_versions(&contents).await {
                Ok(review) => {
                    evaluation = Some(ChapterEvaluation {
                        id: uuid::Uuid::new_v4().to_string(),
                        chapter_id: chapter_id.to_string(),
                        consistency_score: review.consistency_score,
                        creativity_score: review.creativity_score,
                        completeness_score: review.completeness_score,
                        rhythm_score: review.rhythm_score,
                        overall_score: review.overall_score,
                        feedback: review.feedback,
                        recommended_version: review.recommended_version,
                        created_at: Utc::now().to_rfc3339(),
                    });
                    recommended_version = Some(review.recommended_version);
                }
                Err(e) => {
                    self.logger.error(&format!("AI review failed: {}", e));
                }
            }
        }

        self.report_progress("complete", 0.95, "生成流程完成").await;

        let result = GenerationResult {
            chapter_id: chapter_id.to_string(),
            versions,
            evaluation,
            guardrails_result,
            recommended_version,
            status: "waiting_for_confirm".to_string(),
            error: None,
        };

        self.logger.info(&format!("Pipeline completed for chapter: {}", chapter_id));
        self.report_progress("complete", 1.0, "流程完成").await;

        Ok(result)
    }

    pub async fn finalize_chapter(
        &self,
        chapter_id: &str,
        selected_version_index: usize,
        config: FlowConfig,
    ) -> Result<Chapter, String> {
        self.logger.info(&format!("Finalizing chapter: {}, version: {}", chapter_id, selected_version_index));
        
        self.report_progress("finalize", 0.0, "开始定稿流程").await;

        self.report_progress("finalize", 0.3, "更新章节状态").await;

        if !config.skip_summary_generation {
            self.report_progress("finalize", 0.5, "生成章节摘要").await;
        }

        if !config.skip_vectorization {
            self.report_progress("finalize", 0.7, "章节向量化入库").await;
            
            if let Err(e) = self.vector_store.vectorize_chapter(chapter_id).await {
                self.logger.error(&format!("Vectorization failed: {}", e));
            }
        }

        self.report_progress("finalize", 1.0, "定稿完成").await;

        self.logger.info(&format!("Chapter finalized: {}", chapter_id));
        
        Err("Finalize not fully implemented - need database connection".to_string())
    }
}

impl Default for PipelineOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
