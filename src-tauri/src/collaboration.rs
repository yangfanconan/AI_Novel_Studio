use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    pub user_id: String,
    pub chapter_id: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: String,
    pub user_id: String,
    pub chapter_id: String,
    pub op_type: OperationType,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationType {
    Insert { position: usize, text: String },
    Delete { position: usize, length: usize },
    Replace { position: usize, length: usize, text: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub id: String,
    pub project_id: String,
    pub users: Vec<User>,
    pub active_cursors: HashMap<String, CursorPosition>,
}

pub struct CollaborationManager {
    sessions: Arc<Mutex<HashMap<String, CollaborationSession>>>,
    operation_channels: Arc<Mutex<HashMap<String, broadcast::Sender<Operation>>>>,
}

impl CollaborationManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            operation_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_session(&self, project_id: String) -> String {
        let session_id = format!("session_{}", uuid::Uuid::new_v4());
        let session = CollaborationSession {
            id: session_id.clone(),
            project_id,
            users: vec![],
            active_cursors: HashMap::new(),
        };

        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id.clone(), session);

        let (tx, _) = broadcast::channel(100);
        let mut channels = self.operation_channels.lock().unwrap();
        channels.insert(session_id.clone(), tx);

        session_id
    }

    pub fn join_session(&self, session_id: &str, user: User) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            if !session.users.iter().any(|u| u.id == user.id) {
                session.users.push(user);
            }
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }

    pub fn leave_session(&self, session_id: &str, user_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            session.users.retain(|u| u.id != user_id);
            session.active_cursors.remove(user_id);
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }

    pub fn broadcast_operation(&self, session_id: &str, operation: Operation) -> Result<(), String> {
        let channels = self.operation_channels.lock().unwrap();
        if let Some(tx) = channels.get(session_id) {
            let _ = tx.send(operation);
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }

    pub fn subscribe_operations(&self, session_id: &str) -> Option<broadcast::Receiver<Operation>> {
        let channels = self.operation_channels.lock().unwrap();
        channels.get(session_id).map(|tx| tx.subscribe())
    }

    pub fn update_cursor(&self, session_id: &str, cursor: CursorPosition) -> Result<(), String> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            session.active_cursors.insert(cursor.user_id.clone(), cursor);
            Ok(())
        } else {
            Err("Session not found".to_string())
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<CollaborationSession> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(session_id).cloned()
    }

    pub fn get_user_cursors(&self, session_id: &str) -> HashMap<String, CursorPosition> {
        let sessions = self.sessions.lock().unwrap();
        sessions
            .get(session_id)
            .map(|s| s.active_cursors.clone())
            .unwrap_or_default()
    }
}

impl Default for CollaborationManager {
    fn default() -> Self {
        Self::new()
    }
}
