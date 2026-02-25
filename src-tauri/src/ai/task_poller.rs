use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncTaskResult {
    pub task_id: String,
    pub status: TaskStatus,
    pub progress: Option<u32>,
    pub result_url: Option<String>,
    pub error: Option<String>,
    pub estimated_time: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

pub struct PollOptions<F>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AsyncTaskResult, String>> + Send>> + Send + Sync,
{
    pub fetch_status: F,
    pub on_progress: Option<Box<dyn Fn(u32, &str) + Send + Sync>>,
    pub is_cancelled: Option<Box<dyn Fn() -> bool + Send + Sync>>,
    pub interval: Option<Duration>,
    pub timeout: Option<Duration>,
}

pub struct TaskPoller {
    default_interval: Duration,
    default_timeout: Duration,
    max_timeout: Duration,
}

impl TaskPoller {
    pub fn new() -> Self {
        Self {
            default_interval: Duration::from_secs(3),
            default_timeout: Duration::from_secs(600),
            max_timeout: Duration::from_secs(1800),
        }
    }

    pub async fn poll<F>(
        &self,
        task_id: &str,
        task_type: &str,
        fetch_status: F,
        on_progress: Option<Box<dyn Fn(u32, &str) + Send + Sync>>,
        is_cancelled: Option<Box<dyn Fn() -> bool + Send + Sync>>,
        interval: Option<Duration>,
        timeout: Option<Duration>,
    ) -> Result<AsyncTaskResult, String>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AsyncTaskResult, String>> + Send>> + Send + Sync,
    {
        let interval = interval.unwrap_or(self.default_interval);
        let timeout = timeout.unwrap_or(self.default_timeout);
        let start_time = std::time::Instant::now();
        let mut effective_timeout = timeout;
        let mut poll_count = 0;

        log::info!(
            "[TaskPoller] Starting poll for {} task: {}",
            task_type,
            task_id
        );

        loop {
            poll_count += 1;

            if let Some(ref check_cancel) = is_cancelled {
                if check_cancel() {
                    log::info!("[TaskPoller] Task {} cancelled by user", task_id);
                    return Err("Task cancelled".to_string());
                }
            }

            let elapsed = start_time.elapsed();
            if elapsed > effective_timeout {
                let minutes = effective_timeout.as_secs() / 60;
                log::error!(
                    "[TaskPoller] Task {} timed out after {} minutes",
                    task_id,
                    minutes
                );
                return Err(format!("{} generation timeout after {} minutes", task_type, minutes));
            }

            match fetch_status().await {
                Ok(result) => {
                    let progress = result.progress.unwrap_or(0);
                    let status_str = match result.status {
                        TaskStatus::Pending => "pending",
                        TaskStatus::Processing => "processing",
                        TaskStatus::Completed => "completed",
                        TaskStatus::Failed => "failed",
                    };

                    if let Some(ref callback) = on_progress {
                        callback(progress, status_str);
                    }

                    if let Some(estimated) = result.estimated_time {
                        if estimated > 0 {
                            let buffered = Duration::from_secs((estimated as u64 * 2 + 120).min(1800));
                            if buffered > effective_timeout {
                                effective_timeout = buffered.min(self.max_timeout);
                                log::info!(
                                    "[TaskPoller] Extended timeout to {} minutes based on server estimate",
                                    effective_timeout.as_secs() / 60
                                );
                            }
                        }
                    }

                    match result.status {
                        TaskStatus::Completed => {
                            log::info!(
                                "[TaskPoller] Task {} completed after {} polls",
                                task_id,
                                poll_count
                            );
                            return Ok(result);
                        }
                        TaskStatus::Failed => {
                            log::error!("[TaskPoller] Task {} failed: {:?}", task_id, result.error);
                            return Err(result.error.unwrap_or_else(|| "Task failed".to_string()));
                        }
                        _ => {}
                    }

                    if poll_count % 10 == 0 {
                        log::info!(
                            "[TaskPoller] Task {} still {}, progress: {}%, poll #{}",
                            task_id,
                            status_str,
                            progress,
                            poll_count
                        );
                    }
                }
                Err(e) => {
                    if e.contains("cancelled") || e.contains("timeout") || e.contains("failed") {
                        return Err(e);
                    }
                    log::warn!(
                        "[TaskPoller] Network error on poll #{}, will retry: {}",
                        poll_count,
                        e
                    );
                }
            }

            sleep(interval).await;
        }
    }

    pub async fn poll_simple<F>(
        &self,
        fetch_status: F,
        timeout: Option<Duration>,
        interval: Option<Duration>,
    ) -> Result<AsyncTaskResult, String>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AsyncTaskResult, String>> + Send>> + Send + Sync,
    {
        self.poll(
            "simple",
            "image",
            fetch_status,
            None,
            None,
            interval,
            timeout,
        )
        .await
    }
}

impl Default for TaskPoller {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub async fn poll_task_status(
    task_id: String,
    timeout_seconds: Option<u64>,
    interval_seconds: Option<u64>,
) -> Result<AsyncTaskResult, String> {
    let poller = TaskPoller::new();
    let timeout = timeout_seconds.map(Duration::from_secs);
    let interval = interval_seconds.map(Duration::from_secs);

    poller
        .poll_simple(
            move || {
                let tid = task_id.clone();
                Box::pin(async move {
                    Ok(AsyncTaskResult {
                        task_id: tid.clone(),
                        status: TaskStatus::Completed,
                        progress: Some(100),
                        result_url: Some("placeholder_result_url".to_string()),
                        error: None,
                        estimated_time: None,
                    })
                })
            },
            timeout,
            interval,
        )
        .await
}
