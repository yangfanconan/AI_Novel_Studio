use crate::collaboration::{CollaborationManager, User, CursorPosition, Operation, CollaborationSession};
use crate::logger::Logger;
use std::sync::Arc;

#[derive(Clone)]
pub struct CollaborationState {
    manager: Arc<CollaborationManager>,
}

impl CollaborationState {
    pub fn new() -> Self {
        Self {
            manager: Arc::new(CollaborationManager::new()),
        }
    }
}

impl Default for CollaborationState {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub async fn collab_create_session(
    project_id: String,
    state: tauri::State<'_, CollaborationState>,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("collaboration");
    logger.info(&format!("Creating collaboration session for project {}", project_id));

    let session_id = state.manager.create_session(project_id);
    Ok(session_id)
}

#[tauri::command]
pub async fn collab_join_session(
    session_id: String,
    user: User,
    state: tauri::State<'_, CollaborationState>,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("collaboration");
    logger.info(&format!("User {} joining session {}", user.id, session_id));

    state.manager.join_session(&session_id, user)
}

#[tauri::command]
pub async fn collab_leave_session(
    session_id: String,
    user_id: String,
    state: tauri::State<'_, CollaborationState>,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("collaboration");
    logger.info(&format!("User {} leaving session {}", user_id, session_id));

    state.manager.leave_session(&session_id, &user_id)
}

#[tauri::command]
pub async fn collab_broadcast_operation(
    session_id: String,
    operation: Operation,
    state: tauri::State<'_, CollaborationState>,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("collaboration");
    logger.info(&format!("Broadcasting operation {} in session {}", operation.id, session_id));

    state.manager.broadcast_operation(&session_id, operation)
}

#[tauri::command]
pub async fn collab_update_cursor(
    session_id: String,
    cursor: CursorPosition,
    state: tauri::State<'_, CollaborationState>,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("collaboration");
    logger.info(&format!("Updating cursor for user {} in session {}", cursor.user_id, session_id));

    state.manager.update_cursor(&session_id, cursor)
}

#[tauri::command]
pub async fn collab_get_session(
    session_id: String,
    state: tauri::State<'_, CollaborationState>,
) -> Result<Option<CollaborationSession>, String> {
    let logger = Logger::new().with_feature("collaboration");
    logger.info(&format!("Getting session {}", session_id));

    Ok(state.manager.get_session(&session_id))
}

#[tauri::command]
pub async fn collab_get_user_cursors(
    session_id: String,
    state: tauri::State<'_, CollaborationState>,
) -> Result<serde_json::Value, String> {
    let logger = Logger::new().with_feature("collaboration");
    logger.info(&format!("Getting user cursors for session {}", session_id));

    let cursors = state.manager.get_user_cursors(&session_id);
    Ok(serde_json::to_value(&cursors).unwrap_or(serde_json::Value::Null))
}

#[tauri::command]
pub async fn collab_generate_user_id() -> Result<String, String> {
    let user_id = format!("user_{}", uuid::Uuid::new_v4());
    Ok(user_id)
}

#[tauri::command]
pub async fn collab_generate_color() -> Result<String, String> {
    let colors = vec![
        "#FF6B6B", "#4ECDC4", "#45B7D1", "#FFA07A",
        "#98D8C8", "#F7DC6F", "#BB8FCE", "#85C1E9",
        "#F8C471", "#52BE80", "#EC7063", "#5DADE2"
    ];
    let color = colors[rand::random::<usize>() % colors.len()];
    Ok(color.to_string())
}
