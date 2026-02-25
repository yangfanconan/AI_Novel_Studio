use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rusqlite::{Connection, params, Result as SqlResult};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptScene {
    pub id: String,
    pub project_id: String,
    pub chapter_id: Option<String>,
    pub scene_index: i32,
    pub narration: String,
    pub visual_content: String,
    pub action: String,
    pub camera: String,
    pub character_description: String,
    pub generated_image_url: Option<String>,
    pub generated_video_url: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSceneRequest {
    pub project_id: String,
    pub chapter_id: Option<String>,
    pub scene_index: i32,
    pub narration: String,
    pub visual_content: String,
    pub action: String,
    pub camera: String,
    pub character_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateSceneRequest {
    pub id: String,
    pub narration: Option<String>,
    pub visual_content: Option<String>,
    pub action: Option<String>,
    pub camera: Option<String>,
    pub character_description: Option<String>,
    pub generated_image_url: Option<String>,
    pub generated_video_url: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneBatchResult {
    pub created: i32,
    pub updated: i32,
    pub failed: i32,
    pub scenes: Vec<ScriptScene>,
}

pub struct SceneManager;

impl SceneManager {
    pub fn create_scene(conn: &Connection, request: CreateSceneRequest) -> SqlResult<ScriptScene> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO script_scenes (
                id, project_id, chapter_id, scene_index, narration, visual_content,
                action, camera, character_description, status, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'pending', ?10, ?11)",
            params![
                id,
                request.project_id,
                request.chapter_id,
                request.scene_index,
                request.narration,
                request.visual_content,
                request.action,
                request.camera,
                request.character_description,
                now,
                now,
            ],
        )?;

        Ok(ScriptScene {
            id,
            project_id: request.project_id,
            chapter_id: request.chapter_id,
            scene_index: request.scene_index,
            narration: request.narration,
            visual_content: request.visual_content,
            action: request.action,
            camera: request.camera,
            character_description: request.character_description,
            generated_image_url: None,
            generated_video_url: None,
            status: "pending".to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get_scene(conn: &Connection, id: &str) -> SqlResult<Option<ScriptScene>> {
        let mut stmt = conn.prepare(
            "SELECT id, project_id, chapter_id, scene_index, narration, visual_content,
                    action, camera, character_description, generated_image_url,
                    generated_video_url, status, created_at, updated_at
             FROM script_scenes WHERE id = ?1"
        )?;

        let result = stmt.query_row(params![id], |row| {
            Ok(ScriptScene {
                id: row.get(0)?,
                project_id: row.get(1)?,
                chapter_id: row.get(2)?,
                scene_index: row.get(3)?,
                narration: row.get(4)?,
                visual_content: row.get(5)?,
                action: row.get(6)?,
                camera: row.get(7)?,
                character_description: row.get(8)?,
                generated_image_url: row.get(9)?,
                generated_video_url: row.get(10)?,
                status: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        });

        match result {
            Ok(scene) => Ok(Some(scene)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_project_scenes(conn: &Connection, project_id: &str) -> SqlResult<Vec<ScriptScene>> {
        let mut stmt = conn.prepare(
            "SELECT id, project_id, chapter_id, scene_index, narration, visual_content,
                    action, camera, character_description, generated_image_url,
                    generated_video_url, status, created_at, updated_at
             FROM script_scenes WHERE project_id = ?1 ORDER BY scene_index"
        )?;

        let scenes = stmt.query_map(params![project_id], |row| {
            Ok(ScriptScene {
                id: row.get(0)?,
                project_id: row.get(1)?,
                chapter_id: row.get(2)?,
                scene_index: row.get(3)?,
                narration: row.get(4)?,
                visual_content: row.get(5)?,
                action: row.get(6)?,
                camera: row.get(7)?,
                character_description: row.get(8)?,
                generated_image_url: row.get(9)?,
                generated_video_url: row.get(10)?,
                status: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?;

        scenes.collect()
    }

    pub fn get_chapter_scenes(conn: &Connection, chapter_id: &str) -> SqlResult<Vec<ScriptScene>> {
        let mut stmt = conn.prepare(
            "SELECT id, project_id, chapter_id, scene_index, narration, visual_content,
                    action, camera, character_description, generated_image_url,
                    generated_video_url, status, created_at, updated_at
             FROM script_scenes WHERE chapter_id = ?1 ORDER BY scene_index"
        )?;

        let scenes = stmt.query_map(params![chapter_id], |row| {
            Ok(ScriptScene {
                id: row.get(0)?,
                project_id: row.get(1)?,
                chapter_id: row.get(2)?,
                scene_index: row.get(3)?,
                narration: row.get(4)?,
                visual_content: row.get(5)?,
                action: row.get(6)?,
                camera: row.get(7)?,
                character_description: row.get(8)?,
                generated_image_url: row.get(9)?,
                generated_video_url: row.get(10)?,
                status: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?;

        scenes.collect()
    }

    pub fn update_scene(conn: &Connection, request: UpdateSceneRequest) -> SqlResult<Option<ScriptScene>> {
        let now = Utc::now().to_rfc3339();
        
        let mut updates = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref v) = request.narration {
            updates.push("narration = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.visual_content {
            updates.push("visual_content = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.action {
            updates.push("action = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.camera {
            updates.push("camera = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.character_description {
            updates.push("character_description = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.generated_image_url {
            updates.push("generated_image_url = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.generated_video_url {
            updates.push("generated_video_url = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.status {
            updates.push("status = ?");
            values.push(Box::new(v.clone()));
        }

        if updates.is_empty() {
            return Self::get_scene(conn, &request.id);
        }

        updates.push("updated_at = ?");
        values.push(Box::new(now.clone()));

        values.push(Box::new(request.id.clone()));

        let sql = format!(
            "UPDATE script_scenes SET {} WHERE id = ?",
            updates.join(", ")
        );

        let params: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        conn.execute(&sql, params.as_slice())?;

        Self::get_scene(conn, &request.id)
    }

    pub fn delete_scene(conn: &Connection, id: &str) -> SqlResult<bool> {
        let affected = conn.execute("DELETE FROM script_scenes WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    pub fn delete_project_scenes(conn: &Connection, project_id: &str) -> SqlResult<i32> {
        let affected = conn.execute("DELETE FROM script_scenes WHERE project_id = ?1", params![project_id])?;
        Ok(affected as i32)
    }

    pub fn batch_create_scenes(
        conn: &Connection,
        requests: Vec<CreateSceneRequest>,
    ) -> SceneBatchResult {
        let mut result = SceneBatchResult {
            created: 0,
            updated: 0,
            failed: 0,
            scenes: Vec::new(),
        };

        for (idx, request) in requests.into_iter().enumerate() {
            let mut req = request;
            req.scene_index = idx as i32;

            match Self::create_scene(conn, req) {
                Ok(scene) => {
                    result.created += 1;
                    result.scenes.push(scene);
                }
                Err(_) => {
                    result.failed += 1;
                }
            }
        }

        result
    }

    pub fn update_scene_status(
        conn: &Connection,
        id: &str,
        status: &str,
    ) -> SqlResult<Option<ScriptScene>> {
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "UPDATE script_scenes SET status = ?1, updated_at = ?2 WHERE id = ?3",
            params![status, now, id],
        )?;

        Self::get_scene(conn, id)
    }

    pub fn set_generated_image(
        conn: &Connection,
        id: &str,
        image_url: &str,
    ) -> SqlResult<Option<ScriptScene>> {
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "UPDATE script_scenes SET generated_image_url = ?1, status = 'image_ready', updated_at = ?2 WHERE id = ?3",
            params![image_url, now, id],
        )?;

        Self::get_scene(conn, id)
    }

    pub fn set_generated_video(
        conn: &Connection,
        id: &str,
        video_url: &str,
    ) -> SqlResult<Option<ScriptScene>> {
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "UPDATE script_scenes SET generated_video_url = ?1, status = 'completed', updated_at = ?2 WHERE id = ?3",
            params![video_url, now, id],
        )?;

        Self::get_scene(conn, id)
    }

    pub fn get_scenes_by_status(
        conn: &Connection,
        project_id: &str,
        status: &str,
    ) -> SqlResult<Vec<ScriptScene>> {
        let mut stmt = conn.prepare(
            "SELECT id, project_id, chapter_id, scene_index, narration, visual_content,
                    action, camera, character_description, generated_image_url,
                    generated_video_url, status, created_at, updated_at
             FROM script_scenes WHERE project_id = ?1 AND status = ?2 ORDER BY scene_index"
        )?;

        let scenes = stmt.query_map(params![project_id, status], |row| {
            Ok(ScriptScene {
                id: row.get(0)?,
                project_id: row.get(1)?,
                chapter_id: row.get(2)?,
                scene_index: row.get(3)?,
                narration: row.get(4)?,
                visual_content: row.get(5)?,
                action: row.get(6)?,
                camera: row.get(7)?,
                character_description: row.get(8)?,
                generated_image_url: row.get(9)?,
                generated_video_url: row.get(10)?,
                status: row.get(11)?,
                created_at: row.get(12)?,
                updated_at: row.get(13)?,
            })
        })?;

        scenes.collect()
    }

    pub fn get_scene_statistics(conn: &Connection, project_id: &str) -> SqlResult<SceneStatistics> {
        let mut stats = SceneStatistics::default();

        let mut stmt = conn.prepare(
            "SELECT status, COUNT(*) FROM script_scenes WHERE project_id = ?1 GROUP BY status"
        )?;

        let rows = stmt.query_map(params![project_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        })?;

        for row in rows {
            let (status, count) = row?;
            match status.as_str() {
                "pending" => stats.pending = count,
                "processing" => stats.processing = count,
                "image_ready" => stats.image_ready = count,
                "completed" => stats.completed = count,
                "failed" => stats.failed = count,
                _ => {}
            }
        }

        stats.total = stats.pending + stats.processing + stats.image_ready + stats.completed + stats.failed;
        Ok(stats)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SceneStatistics {
    pub total: i32,
    pub pending: i32,
    pub processing: i32,
    pub image_ready: i32,
    pub completed: i32,
    pub failed: i32,
}

#[tauri::command]
pub async fn create_script_scene(
    request: CreateSceneRequest,
    db_path: String,
) -> Result<ScriptScene, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::create_scene(&conn, request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_script_scene(id: String, db_path: String) -> Result<Option<ScriptScene>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::get_scene(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_project_script_scenes(project_id: String, db_path: String) -> Result<Vec<ScriptScene>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::get_project_scenes(&conn, &project_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_chapter_script_scenes(chapter_id: String, db_path: String) -> Result<Vec<ScriptScene>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::get_chapter_scenes(&conn, &chapter_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_script_scene(request: UpdateSceneRequest, db_path: String) -> Result<Option<ScriptScene>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::update_scene(&conn, request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_script_scene(id: String, db_path: String) -> Result<bool, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::delete_scene(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn batch_create_script_scenes(
    requests: Vec<CreateSceneRequest>,
    db_path: String,
) -> Result<SceneBatchResult, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    Ok(SceneManager::batch_create_scenes(&conn, requests))
}

#[tauri::command]
pub async fn update_scene_generation_status(
    id: String,
    status: String,
    db_path: String,
) -> Result<Option<ScriptScene>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::update_scene_status(&conn, &id, &status).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_scene_generated_image(
    id: String,
    image_url: String,
    db_path: String,
) -> Result<Option<ScriptScene>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::set_generated_image(&conn, &id, &image_url).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_scene_generated_video(
    id: String,
    video_url: String,
    db_path: String,
) -> Result<Option<ScriptScene>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::set_generated_video(&conn, &id, &video_url).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_scene_statistics_cmd(project_id: String, db_path: String) -> Result<SceneStatistics, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    SceneManager::get_scene_statistics(&conn, &project_id).map_err(|e| e.to_string())
}
