use crate::version_control::{VersionControlManager, ProjectSnapshot, VersionDiff, VersionControlConfig};
use crate::models::{Chapter, Character, WorldView, PlotPoint};
use crate::logger::Logger;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use rusqlite::params;

#[tauri::command]
pub async fn create_snapshot(
    app: AppHandle,
    project_id: String,
    version: String,
    description: String,
    auto_generated: bool,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("version_control");
    logger.info(&format!("Creating snapshot for project {}", project_id));

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let chapters = load_chapters(&conn, &project_id)?;
    let characters = load_characters(&conn, &project_id)?;
    let world_views = load_world_views(&conn, &project_id)?;
    let plot_points = load_plot_points(&conn, &project_id)?;

    let snapshot = VersionControlManager::create_snapshot(
        &project_id,
        &version,
        &description,
        chapters,
        characters,
        world_views,
        plot_points,
        auto_generated,
    );

    let created_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO project_snapshots (id, project_id, version, timestamp, description, chapters_json, characters_json, world_views_json, plot_points_json, metadata_json, auto_generated, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            snapshot.id,
            snapshot.project_id,
            snapshot.version,
            snapshot.timestamp,
            snapshot.description,
            serde_json::to_string(&snapshot.chapters).unwrap_or_default(),
            serde_json::to_string(&snapshot.characters).unwrap_or_default(),
            serde_json::to_string(&snapshot.world_views).unwrap_or_default(),
            serde_json::to_string(&snapshot.plot_points).unwrap_or_default(),
            serde_json::to_string(&snapshot.metadata).unwrap_or_default(),
            if snapshot.metadata.auto_generated { 1 } else { 0 },
            created_at,
        ],
    ).map_err(|e| format!("Failed to save snapshot: {}", e))?;

    let max_snapshots = get_max_snapshots(&conn);
    cleanup_old_snapshots(&conn, &project_id, max_snapshots)
        .map_err(|e| format!("Failed to cleanup old snapshots: {}", e))?;

    logger.info("Snapshot created successfully");
    serde_json::to_string(&snapshot).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_snapshots(
    app: AppHandle,
    project_id: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("version_control");
    logger.info(&format!("Getting snapshots for project {}", project_id));

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT id, project_id, version, timestamp, description, metadata_json, auto_generated 
         FROM project_snapshots 
         WHERE project_id = ?1 
         ORDER BY timestamp DESC"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let result: Vec<serde_json::Value> = stmt.query_map(params![project_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?,
            "project_id": row.get::<_, String>(1)?,
            "version": row.get::<_, String>(2)?,
            "timestamp": row.get::<_, i64>(3)?,
            "description": row.get::<_, String>(4)?,
            "metadata": row.get::<_, String>(5)?,
            "auto_generated": row.get::<_, i32>(6)? != 0,
        }))
    }).map_err(|e| format!("Failed to query snapshots: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect snapshots: {}", e))?;

    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_snapshot(
    app: AppHandle,
    snapshot_id: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("version_control");
    logger.info(&format!("Getting snapshot {}", snapshot_id));

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT id, project_id, version, timestamp, description, chapters_json, characters_json, world_views_json, plot_points_json, metadata_json, auto_generated 
         FROM project_snapshots 
         WHERE id = ?1"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let snapshot = stmt.query_row(params![snapshot_id], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, String>(0)?,
            "project_id": row.get::<_, String>(1)?,
            "version": row.get::<_, String>(2)?,
            "timestamp": row.get::<_, i64>(3)?,
            "description": row.get::<_, String>(4)?,
            "chapters": row.get::<_, String>(5)?,
            "characters": row.get::<_, String>(6)?,
            "world_views": row.get::<_, String>(7)?,
            "plot_points": row.get::<_, String>(8)?,
            "metadata": row.get::<_, String>(9)?,
            "auto_generated": row.get::<_, i32>(10)? != 0,
        }))
    }).map_err(|e| format!("Failed to query snapshot: {}", e))?;

    serde_json::to_string(&snapshot).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restore_snapshot(
    app: AppHandle,
    snapshot_id: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("version_control");
    logger.info(&format!("Restoring from snapshot {}", snapshot_id));

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let snapshot_json = get_snapshot(app.clone(), snapshot_id).await?;
    let snapshot: serde_json::Value = serde_json::from_str(&snapshot_json)
        .map_err(|e| format!("Failed to parse snapshot: {}", e))?;

    let project_id = snapshot["project_id"].as_str()
        .ok_or("Missing project_id")?;

    conn.execute("DELETE FROM chapters WHERE project_id = ?1", params![project_id])
        .map_err(|e| format!("Failed to delete chapters: {}", e))?;

    conn.execute("DELETE FROM characters WHERE project_id = ?1", params![project_id])
        .map_err(|e| format!("Failed to delete characters: {}", e))?;

    conn.execute("DELETE FROM world_views WHERE project_id = ?1", params![project_id])
        .map_err(|e| format!("Failed to delete world_views: {}", e))?;

    conn.execute("DELETE FROM plot_points WHERE project_id = ?1", params![project_id])
        .map_err(|e| format!("Failed to delete plot_points: {}", e))?;

    if let Some(chapters) = snapshot["chapters"].as_str() {
        let chapters_data: Vec<Chapter> = serde_json::from_str(chapters)
            .map_err(|e| format!("Failed to parse chapters: {}", e))?;

        for chapter in chapters_data {
            conn.execute(
                "INSERT INTO chapters (id, project_id, title, content, word_count, sort_order, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    chapter.id,
                    chapter.project_id,
                    chapter.title,
                    chapter.content,
                    chapter.word_count,
                    chapter.sort_order,
                    chapter.status,
                    chapter.created_at,
                    chapter.updated_at,
                ],
            ).map_err(|e| format!("Failed to insert chapter: {}", e))?;
        }
    }

    if let Some(characters) = snapshot["characters"].as_str() {
        let characters_data: Vec<Character> = serde_json::from_str(characters)
            .map_err(|e| format!("Failed to parse characters: {}", e))?;

        for character in characters_data {
            conn.execute(
                "INSERT INTO characters (id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                params![
                    character.id,
                    character.project_id,
                    character.name,
                    character.role_type,
                    character.race,
                    character.age,
                    character.gender,
                    character.birth_date,
                    character.appearance,
                    character.personality,
                    character.background,
                    character.skills,
                    character.status,
                    character.bazi,
                    character.ziwei,
                    character.mbti,
                    character.enneagram,
                    character.items,
                    character.avatar_url,
                    character.created_at,
                    character.updated_at,
                ],
            ).map_err(|e| format!("Failed to insert character: {}", e))?;
        }
    }

    if let Some(world_views) = snapshot["world_views"].as_str() {
        let world_views_data: Vec<WorldView> = serde_json::from_str(world_views)
            .map_err(|e| format!("Failed to parse world_views: {}", e))?;

        for world_view in world_views_data {
            conn.execute(
                "INSERT INTO world_views (id, project_id, category, title, content, tags, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    world_view.id,
                    world_view.project_id,
                    world_view.category,
                    world_view.title,
                    world_view.content,
                    world_view.tags,
                    world_view.status,
                    world_view.created_at,
                    world_view.updated_at,
                ],
            ).map_err(|e| format!("Failed to insert world_view: {}", e))?;
        }
    }

    if let Some(plot_points) = snapshot["plot_points"].as_str() {
        let plot_points_data: Vec<PlotPoint> = serde_json::from_str(plot_points)
            .map_err(|e| format!("Failed to parse plot_points: {}", e))?;

        for plot_point in plot_points_data {
            conn.execute(
                "INSERT INTO plot_points (id, project_id, parent_id, title, description, note, chapter_id, status, sort_order, level, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    plot_point.id,
                    plot_point.project_id,
                    plot_point.parent_id,
                    plot_point.title,
                    plot_point.description,
                    plot_point.note,
                    plot_point.chapter_id,
                    plot_point.status,
                    plot_point.sort_order,
                    plot_point.level,
                    plot_point.created_at,
                    plot_point.updated_at,
                ],
            ).map_err(|e| format!("Failed to insert plot_point: {}", e))?;
        }
    }

    logger.info("Snapshot restored successfully");
    Ok("{\"status\":\"success\"}".to_string())
}

#[tauri::command]
pub async fn delete_snapshot(
    app: AppHandle,
    snapshot_id: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("version_control");
    logger.info(&format!("Deleting snapshot {}", snapshot_id));

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    conn.execute("DELETE FROM project_snapshots WHERE id = ?1", params![snapshot_id])
        .map_err(|e| format!("Failed to delete snapshot: {}", e))?;

    logger.info("Snapshot deleted successfully");
    Ok("{\"status\":\"success\"}".to_string())
}

#[tauri::command]
pub async fn compare_snapshots(
    app: AppHandle,
    from_snapshot_id: String,
    to_snapshot_id: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("version_control");
    logger.info(&format!("Comparing snapshots {} and {}", from_snapshot_id, to_snapshot_id));

    let from_snapshot_json = get_snapshot(app.clone(), from_snapshot_id.clone()).await?;
    let to_snapshot_json = get_snapshot(app.clone(), to_snapshot_id.clone()).await?;

    let from_snapshot: serde_json::Value = serde_json::from_str(&from_snapshot_json)
        .map_err(|e| format!("Failed to parse from_snapshot: {}", e))?;

    let to_snapshot: serde_json::Value = serde_json::from_str(&to_snapshot_json)
        .map_err(|e| format!("Failed to parse to_snapshot: {}", e))?;

    let diff = serde_json::json!({
        "from_version": from_snapshot["version"],
        "to_version": to_snapshot["version"],
        "from_timestamp": from_snapshot["timestamp"],
        "to_timestamp": to_snapshot["timestamp"],
        "has_changes": from_snapshot != to_snapshot,
    });

    serde_json::to_string(&diff).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_version_config(
    app: AppHandle,
) -> Result<String, String> {
    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let config = conn.query_row(
        "SELECT auto_save_enabled, auto_save_interval_minutes, max_snapshots_per_project, compression_enabled FROM version_control_config WHERE id = 'config'",
        [],
        |row| {
            Ok(VersionControlConfig {
                auto_save_enabled: row.get::<_, i32>(0)? != 0,
                auto_save_interval_minutes: row.get::<_, i32>(1)?,
                max_snapshots_per_project: row.get::<_, i32>(2)?,
                compression_enabled: row.get::<_, i32>(3)? != 0,
            })
        }
    ).unwrap_or_else(|_| VersionControlConfig::default());

    serde_json::to_string(&config).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_version_config(
    app: AppHandle,
    config_json: String,
) -> Result<String, String> {
    let config: VersionControlConfig = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let updated_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT OR REPLACE INTO version_control_config (id, auto_save_enabled, auto_save_interval_minutes, max_snapshots_per_project, compression_enabled, updated_at) VALUES ('config', ?1, ?2, ?3, ?4, ?5)",
        params![
            if config.auto_save_enabled { 1 } else { 0 },
            config.auto_save_interval_minutes,
            config.max_snapshots_per_project,
            if config.compression_enabled { 1 } else { 0 },
            updated_at,
        ],
    ).map_err(|e| format!("Failed to save config: {}", e))?;

    Ok("{\"status\":\"success\"}".to_string())
}

fn get_db_path(app: &AppHandle) -> Result<PathBuf, String> {
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

fn load_chapters(conn: &rusqlite::Connection, project_id: &str) -> Result<Vec<crate::version_control::ChapterSnapshot>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, sort_order, word_count FROM chapters WHERE project_id = ?1 ORDER BY sort_order"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let chapters = stmt.query_map(params![project_id], |row| {
        Ok(crate::version_control::ChapterSnapshot {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            order: row.get(3)?,
            word_count: row.get(4)?,
        })
    }).map_err(|e| format!("Failed to query chapters: {}", e))?;

    chapters.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect chapters: {}", e))
}

fn load_characters(conn: &rusqlite::Connection, project_id: &str) -> Result<Vec<crate::version_control::CharacterSnapshot>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, personality, appearance, background FROM characters WHERE project_id = ?1"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let characters = stmt.query_map(params![project_id], |row| {
        Ok(crate::version_control::CharacterSnapshot {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            personality: row.get(3)?,
            appearance: row.get(4)?,
            background: row.get(5)?,
        })
    }).map_err(|e| format!("Failed to query characters: {}", e))?;

    characters.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect characters: {}", e))
}

fn load_world_views(conn: &rusqlite::Connection, project_id: &str) -> Result<Vec<crate::version_control::WorldViewSnapshot>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, title, category, description FROM world_views WHERE project_id = ?1"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let world_views = stmt.query_map(params![project_id], |row| {
        Ok(crate::version_control::WorldViewSnapshot {
            id: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            description: row.get(3)?,
        })
    }).map_err(|e| format!("Failed to query world_views: {}", e))?;

    world_views.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect world_views: {}", e))
}

fn load_plot_points(conn: &rusqlite::Connection, project_id: &str) -> Result<Vec<crate::version_control::PlotPointSnapshot>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, chapter_id, sort_order FROM plot_points WHERE project_id = ?1 ORDER BY sort_order"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let plot_points = stmt.query_map(params![project_id], |row| {
        Ok(crate::version_control::PlotPointSnapshot {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            chapter_id: row.get(3)?,
            order: row.get(4)?,
        })
    }).map_err(|e| format!("Failed to query plot_points: {}", e))?;

    plot_points.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect plot_points: {}", e))
}

fn get_max_snapshots(conn: &rusqlite::Connection) -> i32 {
    conn.query_row(
        "SELECT max_snapshots_per_project FROM version_control_config WHERE id = 'config'",
        [],
        |row| row.get::<_, i32>(0)
    ).unwrap_or(50)
}

fn cleanup_old_snapshots(conn: &rusqlite::Connection, project_id: &str, max_snapshots: i32) -> Result<(), String> {
    let snapshots: Vec<(String, i64)> = {
        let mut stmt = conn.prepare(
            "SELECT id, timestamp FROM project_snapshots WHERE project_id = ?1 ORDER BY timestamp DESC"
        ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let snapshots = stmt.query_map(params![project_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        }).map_err(|e| format!("Failed to query snapshots: {}", e))?;

        snapshots.collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect snapshots: {}", e))?
    };

    if snapshots.len() > max_snapshots as usize {
        for (snapshot_id, _) in snapshots.iter().skip(max_snapshots as usize) {
            conn.execute("DELETE FROM project_snapshots WHERE id = ?1", params![snapshot_id])
                .map_err(|e| format!("Failed to delete old snapshot: {}", e))?;
        }
    }

    Ok(())
}
