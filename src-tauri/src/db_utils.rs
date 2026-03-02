use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use rusqlite::{Connection, OptionalExtension};
use crate::logger::Logger;

pub fn get_db_path(app: &AppHandle) -> Result<PathBuf, String> {
    if cfg!(debug_assertions) {
        let mut project_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        project_dir.push("novel_studio_dev.db");
        Ok(std::fs::canonicalize(&project_dir).unwrap_or(project_dir))
    } else {
        let app_data_dir = app.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;
        Ok(app_data_dir.join("novel_studio.db"))
    }
}

pub fn get_connection(db_path: &PathBuf) -> Result<Connection, String> {
    Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))
}

pub fn get_db_connection(app: &AppHandle) -> Result<Connection, String> {
    let db_path = get_db_path(app)?;
    get_connection(&db_path)
}

pub fn resolve_default_model(conn: &Connection, model_id: &str) -> Result<String, String> {
    if model_id == "default" {
        let default_model: Option<String> = conn
            .query_row(
                "SELECT value FROM app_settings WHERE key = 'default_model'",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| format!("获取默认模型失败: {}", e))?;
        
        match default_model {
            Some(id) => Ok(id),
            None => Ok("glm-4-flash".to_string()),
        }
    } else {
        Ok(model_id.to_string())
    }
}

pub fn get_logger(app: &AppHandle) -> Logger {
    let state = app.state::<std::sync::Arc<tokio::sync::RwLock<Logger>>>();
    let logger = state.blocking_read();
    logger.clone()
}
