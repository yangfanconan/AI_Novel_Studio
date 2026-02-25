use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskType {
    ImageGeneration,
    VideoGeneration,
    AudioGeneration,
    ScriptGeneration,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskPriority {
    Low = 1,
    Normal = 5,
    High = 10,
    Urgent = 20,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedTask {
    pub id: String,
    pub project_id: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub state: TaskState,
    pub provider: Option<String>,
    pub input_data: serde_json::Value,
    pub output_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub progress: u32,
    pub created_at: String,
    pub updated_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

impl Eq for QueuedTask {}

impl PartialEq for QueuedTask {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for QueuedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_priority = match self.priority {
            TaskPriority::Urgent => 20,
            TaskPriority::High => 10,
            TaskPriority::Normal => 5,
            TaskPriority::Low => 1,
        };
        let other_priority = match other.priority {
            TaskPriority::Urgent => 20,
            TaskPriority::High => 10,
            TaskPriority::Normal => 5,
            TaskPriority::Low => 1,
        };
        other_priority.cmp(&self_priority)
    }
}

impl PartialOrd for QueuedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub project_id: String,
    pub task_type: TaskType,
    pub priority: Option<TaskPriority>,
    pub provider: Option<String>,
    pub input_data: serde_json::Value,
    pub max_retries: Option<u32>,
}

pub struct TaskQueue {
    tasks: HashMap<String, QueuedTask>,
    pending_queue: BinaryHeap<QueuedTask>,
    max_concurrent: usize,
    running_count: usize,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            pending_queue: BinaryHeap::new(),
            max_concurrent: 3,
            running_count: 0,
        }
    }

    pub fn with_max_concurrent(max_concurrent: usize) -> Self {
        Self {
            tasks: HashMap::new(),
            pending_queue: BinaryHeap::new(),
            max_concurrent,
            running_count: 0,
        }
    }

    pub fn add_task(&mut self, request: CreateTaskRequest) -> QueuedTask {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let task = QueuedTask {
            id: id.clone(),
            project_id: request.project_id,
            task_type: request.task_type,
            priority: request.priority.unwrap_or(TaskPriority::Normal),
            state: TaskState::Pending,
            provider: request.provider,
            input_data: request.input_data,
            output_data: None,
            error_message: None,
            retry_count: 0,
            max_retries: request.max_retries.unwrap_or(3),
            progress: 0,
            created_at: now.clone(),
            updated_at: now,
            started_at: None,
            completed_at: None,
        };

        self.tasks.insert(id.clone(), task.clone());
        self.pending_queue.push(task.clone());
        task
    }

    pub fn get_task(&self, id: &str) -> Option<&QueuedTask> {
        self.tasks.get(id)
    }

    pub fn get_next_task(&mut self) -> Option<QueuedTask> {
        if self.running_count >= self.max_concurrent {
            return None;
        }

        while let Some(task) = self.pending_queue.pop() {
            if let Some(stored_task) = self.tasks.get_mut(&task.id) {
                if stored_task.state == TaskState::Pending {
                    stored_task.state = TaskState::Running;
                    stored_task.started_at = Some(Utc::now().to_rfc3339());
                    stored_task.updated_at = Utc::now().to_rfc3339();
                    self.running_count += 1;
                    return Some(stored_task.clone());
                }
            }
        }
        None
    }

    pub fn complete_task(&mut self, id: &str, output_data: serde_json::Value) -> Option<QueuedTask> {
        if let Some(task) = self.tasks.get_mut(id) {
            if task.state == TaskState::Running {
                task.state = TaskState::Completed;
                task.output_data = Some(output_data);
                task.progress = 100;
                task.completed_at = Some(Utc::now().to_rfc3339());
                task.updated_at = Utc::now().to_rfc3339();
                self.running_count = self.running_count.saturating_sub(1);
                return Some(task.clone());
            }
        }
        None
    }

    pub fn fail_task(&mut self, id: &str, error: &str) -> Option<QueuedTask> {
        if let Some(task) = self.tasks.get_mut(id) {
            if task.state == TaskState::Running {
                task.error_message = Some(error.to_string());
                task.updated_at = Utc::now().to_rfc3339();
                
                if task.retry_count < task.max_retries {
                    task.retry_count += 1;
                    task.state = TaskState::Pending;
                    self.running_count = self.running_count.saturating_sub(1);
                    self.pending_queue.push(task.clone());
                } else {
                    task.state = TaskState::Failed;
                    self.running_count = self.running_count.saturating_sub(1);
                }
                
                return Some(task.clone());
            }
        }
        None
    }

    pub fn cancel_task(&mut self, id: &str) -> Option<QueuedTask> {
        if let Some(task) = self.tasks.get_mut(id) {
            if task.state == TaskState::Pending || task.state == TaskState::Running {
                task.state = TaskState::Cancelled;
                task.updated_at = Utc::now().to_rfc3339();
                if task.state == TaskState::Running {
                    self.running_count = self.running_count.saturating_sub(1);
                }
                return Some(task.clone());
            }
        }
        None
    }

    pub fn update_progress(&mut self, id: &str, progress: u32) -> Option<QueuedTask> {
        if let Some(task) = self.tasks.get_mut(id) {
            task.progress = progress.min(100);
            task.updated_at = Utc::now().to_rfc3339();
            return Some(task.clone());
        }
        None
    }

    pub fn get_tasks_for_project(&self, project_id: &str) -> Vec<QueuedTask> {
        self.tasks
            .values()
            .filter(|t| t.project_id == project_id)
            .cloned()
            .collect()
    }

    pub fn get_pending_tasks(&self) -> Vec<QueuedTask> {
        self.tasks
            .values()
            .filter(|t| t.state == TaskState::Pending)
            .cloned()
            .collect()
    }

    pub fn get_running_tasks(&self) -> Vec<QueuedTask> {
        self.tasks
            .values()
            .filter(|t| t.state == TaskState::Running)
            .cloned()
            .collect()
    }

    pub fn get_stats(&self) -> TaskQueueStats {
        let mut stats = TaskQueueStats::default();
        for task in self.tasks.values() {
            match task.state {
                TaskState::Pending => stats.pending += 1,
                TaskState::Running => stats.running += 1,
                TaskState::Completed => stats.completed += 1,
                TaskState::Failed => stats.failed += 1,
                TaskState::Cancelled => stats.cancelled += 1,
            }
        }
        stats.total = self.tasks.len();
        stats
    }

    pub fn clear_completed(&mut self) {
        let completed_ids: Vec<String> = self
            .tasks
            .iter()
            .filter(|(_, t)| t.state == TaskState::Completed || t.state == TaskState::Failed || t.state == TaskState::Cancelled)
            .map(|(id, _)| id.clone())
            .collect();

        for id in completed_ids {
            self.tasks.remove(&id);
        }
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskQueueStats {
    pub total: usize,
    pub pending: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
    pub cancelled: usize,
}

#[tauri::command]
pub async fn create_task(request: CreateTaskRequest) -> Result<QueuedTask, String> {
    let mut queue = TaskQueue::new();
    Ok(queue.add_task(request))
}

#[tauri::command]
pub async fn get_task(id: String) -> Result<Option<QueuedTask>, String> {
    let queue = TaskQueue::new();
    Ok(queue.get_task(&id).cloned())
}

#[tauri::command]
pub async fn get_project_tasks(project_id: String) -> Result<Vec<QueuedTask>, String> {
    let queue = TaskQueue::new();
    Ok(queue.get_tasks_for_project(&project_id))
}

#[tauri::command]
pub async fn cancel_task(id: String) -> Result<Option<QueuedTask>, String> {
    let mut queue = TaskQueue::new();
    Ok(queue.cancel_task(&id))
}

#[tauri::command]
pub async fn get_queue_stats() -> Result<TaskQueueStats, String> {
    let queue = TaskQueue::new();
    Ok(queue.get_stats())
}

#[tauri::command]
pub async fn clear_completed_tasks() -> Result<(), String> {
    let mut queue = TaskQueue::new();
    queue.clear_completed();
    Ok(())
}
