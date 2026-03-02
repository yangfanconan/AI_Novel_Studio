use serde::{Deserialize, Serialize};
use chrono::Utc;
use rusqlite::{Connection, params, OptionalExtension};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::logger::Logger;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "pending" => TaskStatus::Pending,
            "running" => TaskStatus::Running,
            "completed" => TaskStatus::Completed,
            "failed" => TaskStatus::Failed,
            "cancelled" => TaskStatus::Cancelled,
            _ => TaskStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Vectorization,
    SummaryGeneration,
    AIReview,
    GuardrailsCheck,
    Snapshot,
    Export,
    Custom(String),
}

impl TaskType {
    pub fn as_str(&self) -> &str {
        match self {
            TaskType::Vectorization => "vectorization",
            TaskType::SummaryGeneration => "summary_generation",
            TaskType::AIReview => "ai_review",
            TaskType::GuardrailsCheck => "guardrails_check",
            TaskType::Snapshot => "snapshot",
            TaskType::Export => "export",
            TaskType::Custom(name) => name.as_str(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "vectorization" => TaskType::Vectorization,
            "summary_generation" => TaskType::SummaryGeneration,
            "ai_review" => TaskType::AIReview,
            "guardrails_check" => TaskType::GuardrailsCheck,
            "snapshot" => TaskType::Snapshot,
            "export" => TaskType::Export,
            other => TaskType::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundTask {
    pub id: String,
    pub task_type: String,
    pub status: String,
    pub payload: String,
    pub progress: f32,
    pub result: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub task_id: String,
    pub progress: f32,
    pub message: String,
}

pub type TaskCallback = Arc<Mutex<Option<Box<dyn Fn(TaskProgress) + Send + Sync>>>>;

pub struct AsyncTaskService {
    logger: Logger,
    callback: TaskCallback,
}

impl AsyncTaskService {
    pub fn new() -> Self {
        Self {
            logger: Logger::new().with_feature("async_task_service"),
            callback: Arc::new(Mutex::new(None)),
        }
    }

    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(TaskProgress) + Send + Sync + 'static,
    {
        self.callback = Arc::new(Mutex::new(Some(Box::new(callback))));
        self
    }

    async fn report_progress(&self, task_id: &str, progress: f32, message: &str) {
        if let Some(callback) = self.callback.lock().await.as_ref() {
            callback(TaskProgress {
                task_id: task_id.to_string(),
                progress,
                message: message.to_string(),
            });
        }
    }

    pub fn create_task(
        &self,
        conn: &Connection,
        task_type: TaskType,
        payload: &str,
    ) -> Result<String, String> {
        let task_id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO task_queue (id, task_type, status, payload, progress, created_at) VALUES (?1, ?2, ?3, ?4, 0.0, ?5)",
            params![&task_id, task_type.as_str(), TaskStatus::Pending.as_str(), payload, &now],
        ).map_err(|e| {
            self.logger.error(&format!("Failed to create task: {}", e));
            format!("创建任务失败: {}", e)
        })?;

        self.logger.info(&format!("Task created: {} ({})", task_id, task_type.as_str()));
        Ok(task_id)
    }

    pub fn get_task(&self, conn: &Connection, task_id: &str) -> Result<Option<BackgroundTask>, String> {
        let task = conn.query_row(
            "SELECT id, task_type, status, payload, progress, result, error_message, created_at, started_at, completed_at FROM task_queue WHERE id = ?1",
            params![task_id],
            |row| Ok(BackgroundTask {
                id: row.get(0)?,
                task_type: row.get(1)?,
                status: row.get(2)?,
                payload: row.get(3)?,
                progress: row.get::<_, f64>(4)? as f32,
                result: row.get(5)?,
                error_message: row.get(6)?,
                created_at: row.get(7)?,
                started_at: row.get(8)?,
                completed_at: row.get(9)?,
            }),
        ).optional().map_err(|e| {
            self.logger.error(&format!("Failed to get task: {}", e));
            format!("获取任务失败: {}", e)
        })?;

        Ok(task)
    }

    pub fn get_pending_tasks(&self, conn: &Connection) -> Result<Vec<BackgroundTask>, String> {
        let mut stmt = conn
            .prepare(
                "SELECT id, task_type, status, payload, progress, result, error_message, created_at, started_at, completed_at FROM task_queue WHERE status = 'pending' ORDER BY created_at ASC"
            )
            .map_err(|e| format!("查询任务失败: {}", e))?;

        let tasks = stmt
            .query_map([], |row| {
                Ok(BackgroundTask {
                    id: row.get(0)?,
                    task_type: row.get(1)?,
                    status: row.get(2)?,
                    payload: row.get(3)?,
                    progress: row.get::<_, f64>(4)? as f32,
                    result: row.get(5)?,
                    error_message: row.get(6)?,
                    created_at: row.get(7)?,
                    started_at: row.get(8)?,
                    completed_at: row.get(9)?,
                })
            })
            .map_err(|e| format!("获取任务列表失败: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("解析任务列表失败: {}", e))?;

        Ok(tasks)
    }

    pub fn update_task_status(
        &self,
        conn: &Connection,
        task_id: &str,
        status: TaskStatus,
        progress: f32,
        result: Option<&str>,
        error: Option<&str>,
    ) -> Result<(), String> {
        let now = Utc::now().to_rfc3339();
        let status_str = status.as_str();

        let sql = match status {
            TaskStatus::Running => "UPDATE task_queue SET status = ?1, progress = ?2, started_at = ?3 WHERE id = ?4",
            TaskStatus::Completed => "UPDATE task_queue SET status = ?1, progress = ?2, result = ?3, completed_at = ?4 WHERE id = ?5",
            TaskStatus::Failed => "UPDATE task_queue SET status = ?1, progress = ?2, error_message = ?3, completed_at = ?4 WHERE id = ?5",
            _ => "UPDATE task_queue SET status = ?1, progress = ?2 WHERE id = ?3",
        };

        match status {
            TaskStatus::Running => {
                conn.execute(sql, params![status_str, progress, &now, task_id])
            }
            TaskStatus::Completed => {
                conn.execute(sql, params![status_str, progress, result, &now, task_id])
            }
            TaskStatus::Failed => {
                conn.execute(sql, params![status_str, progress, error, &now, task_id])
            }
            _ => {
                conn.execute(sql, params![status_str, progress, task_id])
            }
        }
        .map_err(|e| {
            self.logger.error(&format!("Failed to update task: {}", e));
            format!("更新任务状态失败: {}", e)
        })?;

        self.logger.info(&format!("Task {} status updated to: {}", task_id, status_str));
        Ok(())
    }

    pub fn cancel_task(&self, conn: &Connection, task_id: &str) -> Result<(), String> {
        self.update_task_status(
            conn,
            task_id,
            TaskStatus::Cancelled,
            0.0,
            None,
            Some("用户取消"),
        )
    }

    pub fn cleanup_completed_tasks(&self, conn: &Connection, days_old: i32) -> Result<usize, String> {
        let cutoff = Utc::now() - chrono::Duration::days(days_old as i64);
        let cutoff_str = cutoff.to_rfc3339();

        let count = conn.execute(
            "DELETE FROM task_queue WHERE status IN ('completed', 'failed', 'cancelled') AND completed_at < ?1",
            params![&cutoff_str],
        ).map_err(|e| {
            self.logger.error(&format!("Failed to cleanup tasks: {}", e));
            format!("清理任务失败: {}", e)
        })?;

        self.logger.info(&format!("Cleaned up {} old tasks", count));
        Ok(count)
    }

    pub async fn execute_vectorization_task(
        &self,
        conn: &Connection,
        task_id: &str,
        chapter_id: &str,
    ) -> Result<(), String> {
        self.update_task_status(conn, task_id, TaskStatus::Running, 0.1, None, None)?;
        self.report_progress(task_id, 0.1, "开始向量化").await;

        self.update_task_status(conn, task_id, TaskStatus::Running, 0.5, None, None)?;
        self.report_progress(task_id, 0.5, "处理中").await;

        use crate::services::vector_store_service::VectorStoreService;
        let vector_store = VectorStoreService::new();
        
        match vector_store.vectorize_chapter(chapter_id).await {
            Ok(_) => {
                self.update_task_status(conn, task_id, TaskStatus::Completed, 1.0, Some("向量化完成"), None)?;
                self.report_progress(task_id, 1.0, "完成").await;
                Ok(())
            }
            Err(e) => {
                self.update_task_status(conn, task_id, TaskStatus::Failed, 0.5, None, Some(&e))?;
                Err(e)
            }
        }
    }

    pub async fn execute_summary_task(
        &self,
        conn: &Connection,
        task_id: &str,
        chapter_id: &str,
    ) -> Result<String, String> {
        self.update_task_status(conn, task_id, TaskStatus::Running, 0.1, None, None)?;
        self.report_progress(task_id, 0.1, "开始生成摘要").await;

        let content: String = conn.query_row(
            "SELECT content FROM chapters WHERE id = ?1",
            params![chapter_id],
            |row| row.get(0),
        ).map_err(|e| format!("获取章节内容失败: {}", e))?;

        if content.trim().is_empty() {
            self.update_task_status(conn, task_id, TaskStatus::Completed, 1.0, Some("章节内容为空"), None)?;
            return Ok("章节内容为空".to_string());
        }

        self.update_task_status(conn, task_id, TaskStatus::Running, 0.5, None, None)?;
        self.report_progress(task_id, 0.5, "AI生成中").await;

        use crate::ai::service::AIService;
        let ai_service = AIService::new();
        
        let system_prompt = "你是一个专业的小说编辑。请为以下章节内容生成一个简洁的摘要（200字以内），突出本章的主要事件和情节发展。";
        
        match ai_service.complete("default", system_prompt, &content, None).await {
            Ok(summary) => {
                let summary = summary.trim().to_string();
                
                conn.execute(
                    "UPDATE chapters SET summary = ?1 WHERE id = ?2",
                    params![&summary, chapter_id],
                ).ok();

                self.update_task_status(conn, task_id, TaskStatus::Completed, 1.0, Some(&summary), None)?;
                self.report_progress(task_id, 1.0, "完成").await;
                Ok(summary)
            }
            Err(e) => {
                self.update_task_status(conn, task_id, TaskStatus::Failed, 0.5, None, Some(&e))?;
                Err(e)
            }
        }
    }
}

impl Default for AsyncTaskService {
    fn default() -> Self {
        Self::new()
    }
}
