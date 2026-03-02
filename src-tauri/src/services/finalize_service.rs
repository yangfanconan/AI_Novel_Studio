use serde::{Deserialize, Serialize};
use chrono::Utc;
use rusqlite::{Connection, params};

use crate::models::{Chapter, ChapterVersion, ChapterEvaluation};
use crate::services::vector_store_service::VectorStoreService;
use crate::ai::service::AIService;
use crate::logger::Logger;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizeResult {
    pub success: bool,
    pub chapter_id: String,
    pub summary: Option<String>,
    pub vectorized: bool,
    pub version_saved: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizeConfig {
    pub generate_summary: bool,
    pub vectorize_content: bool,
    pub save_version_history: bool,
}

impl Default for FinalizeConfig {
    fn default() -> Self {
        Self {
            generate_summary: true,
            vectorize_content: true,
            save_version_history: true,
        }
    }
}

pub struct FinalizeService {
    ai_service: AIService,
    vector_store: VectorStoreService,
    logger: Logger,
}

impl FinalizeService {
    pub fn new() -> Self {
        Self {
            ai_service: AIService::new(),
            vector_store: VectorStoreService::new(),
            logger: Logger::new().with_feature("finalize_service"),
        }
    }

    pub async fn finalize_chapter(
        &self,
        conn: &Connection,
        chapter_id: &str,
        selected_version_index: usize,
        versions: &[ChapterVersion],
        config: FinalizeConfig,
    ) -> Result<FinalizeResult, String> {
        self.logger.info(&format!(
            "Finalizing chapter: {}, selected version: {}", 
            chapter_id, selected_version_index
        ));

        let selected_version = versions.get(selected_version_index)
            .ok_or_else(|| format!("Invalid version index: {}", selected_version_index))?;

        let now = Utc::now().to_rfc3339();
        let word_count = selected_version.content.chars().count() as i32;

        conn.execute(
            "UPDATE chapters SET content = ?1, word_count = ?2, status = 'published', updated_at = ?3, generation_status = 'successful' WHERE id = ?4",
            params![&selected_version.content, word_count, &now, chapter_id],
        ).map_err(|e| {
            self.logger.error(&format!("Failed to update chapter: {}", e));
            format!("更新章节失败: {}", e)
        })?;

        let mut summary = None;
        if config.generate_summary {
            match self.generate_summary(&selected_version.content).await {
                Ok(s) => {
                    summary = Some(s.clone());
                    conn.execute(
                        "UPDATE chapters SET summary = ?1 WHERE id = ?2",
                        params![&s, chapter_id],
                    ).ok();
                }
                Err(e) => {
                    self.logger.error(&format!("Failed to generate summary: {}", e));
                }
            }
        }

        let mut vectorized = false;
        if config.vectorize_content {
            match self.vector_store.vectorize_chapter(chapter_id).await {
                Ok(_) => {
                    vectorized = true;
                    self.logger.info(&format!("Chapter vectorized: {}", chapter_id));
                }
                Err(e) => {
                    self.logger.error(&format!("Failed to vectorize chapter: {}", e));
                }
            }
        }

        if config.save_version_history {
            for (i, version) in versions.iter().enumerate() {
                let selected = i == selected_version_index;
                conn.execute(
                    "INSERT OR REPLACE INTO chapter_versions (id, chapter_id, version_number, content, style_hint, created_at, selected) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![
                        &version.id,
                        chapter_id,
                        version.version_number,
                        &version.content,
                        &version.style_hint,
                        &version.created_at,
                        selected,
                    ],
                ).ok();
            }
        }

        self.logger.info(&format!("Chapter finalized successfully: {}", chapter_id));

        Ok(FinalizeResult {
            success: true,
            chapter_id: chapter_id.to_string(),
            summary,
            vectorized,
            version_saved: config.save_version_history,
            message: "章节定稿成功".to_string(),
        })
    }

    async fn generate_summary(&self, content: &str) -> Result<String, String> {
        if content.trim().is_empty() {
            return Ok("章节内容为空".to_string());
        }

        let system_prompt = "你是一个专业的小说编辑。请为以下章节内容生成一个简洁的摘要（200字以内），突出本章的主要事件和情节发展。";
        
        let summary = self.ai_service
            .complete("default", system_prompt, content, None)
            .await
            .map_err(|e| format!("AI生成摘要失败: {}", e))?;

        Ok(summary.trim().to_string())
    }

    pub async fn save_evaluation(
        &self,
        conn: &Connection,
        evaluation: &ChapterEvaluation,
    ) -> Result<(), String> {
        conn.execute(
            "INSERT OR REPLACE INTO chapter_evaluations (id, chapter_id, consistency_score, creativity_score, completeness_score, rhythm_score, overall_score, feedback, recommended_version, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                &evaluation.id,
                &evaluation.chapter_id,
                evaluation.consistency_score,
                evaluation.creativity_score,
                evaluation.completeness_score,
                evaluation.rhythm_score,
                evaluation.overall_score,
                &evaluation.feedback,
                evaluation.recommended_version,
                &evaluation.created_at,
            ],
        ).map_err(|e| {
            self.logger.error(&format!("Failed to save evaluation: {}", e));
            format!("保存评审结果失败: {}", e)
        })?;

        Ok(())
    }

    pub async fn create_snapshot(
        &self,
        conn: &Connection,
        chapter_id: &str,
        note: &str,
    ) -> Result<String, String> {
        let snapshot_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let chapter: Chapter = conn.query_row(
            "SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary FROM chapters WHERE id = ?1",
            params![chapter_id],
            |row| Ok(Chapter {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                word_count: row.get(4)?,
                sort_order: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                versions: None,
                evaluation: None,
                generation_status: None,
                summary: row.get(9).ok(),
            }),
        ).map_err(|e| format!("获取章节失败: {}", e))?;

        let snapshot_data = serde_json::to_string(&chapter).unwrap_or_default();

        conn.execute(
            "INSERT INTO chapter_snapshots (id, chapter_id, snapshot_data, note, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![&snapshot_id, chapter_id, &snapshot_data, note, &now],
        ).map_err(|e| {
            self.logger.error(&format!("Failed to create snapshot: {}", e));
            format!("创建快照失败: {}", e)
        })?;

        self.logger.info(&format!("Snapshot created for chapter: {}", chapter_id));
        Ok(snapshot_id)
    }
}

impl Default for FinalizeService {
    fn default() -> Self {
        Self::new()
    }
}
