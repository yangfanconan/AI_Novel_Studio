use crate::character_growth::{
    CharacterGrowthManager, CharacterGrowth, GrowthChange, GrowthChangeType, GrowthSignificance, 
    CharacterGrowthTimeline, GrowthComparison
};
use crate::character_tags::{
    CharacterTagManager, CharacterTag, TagType, TagWeight, TagSource,
    CharacterTagCollection
};
use crate::logger::Logger;
use tauri::{AppHandle, Manager};
use rusqlite::params;
use std::collections::HashMap;

#[tauri::command]
pub async fn create_growth_record(
    app: AppHandle,
    character_id: String,
    chapter_id: String,
    position: i32,
    changes_json: String,
    notes: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("character_growth");
    logger.info(&format!("Creating growth record for character {}", character_id));

    let changes: Vec<GrowthChange> = serde_json::from_str(&changes_json)
        .map_err(|e| format!("Failed to parse changes: {}", e))?;

    let growth = CharacterGrowthManager::create_growth_record(
        &character_id,
        &chapter_id,
        position,
        changes,
        true,
        &notes,
    );

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let created_at = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO character_growth_records (id, character_id, chapter_id, position, changes_json, auto_detected, notes, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            growth.id,
            growth.character_id,
            growth.chapter_id,
            growth.position,
            serde_json::to_string(&growth.changes).unwrap_or_default(),
            if growth.metadata.auto_detected { 1 } else { 0 },
            growth.metadata.notes,
            created_at,
        ],
    ).map_err(|e| format!("Failed to save growth record: {}", e))?;

    logger.info("Growth record created successfully");
    serde_json::to_string(&growth).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_growth_timeline(
    app: AppHandle,
    character_id: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("character_growth");
    logger.info(&format!("Getting growth timeline for character {}", character_id));

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT g.id, g.character_id, g.chapter_id, g.position, g.changes_json, g.auto_detected, g.notes, g.created_at,
                c.title, c.sort_order, ch.name
         FROM character_growth_records g
         JOIN chapters c ON g.chapter_id = c.id
         JOIN characters ch ON g.character_id = ch.id
         WHERE g.character_id = ?1
         ORDER BY c.sort_order, g.position"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let growth_records = stmt.query_map(params![character_id], |row| {
        Ok(CharacterGrowth {
            id: row.get(0)?,
            character_id: row.get(1)?,
            chapter_id: row.get(2)?,
            position: row.get(3)?,
            changes: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
            metadata: crate::character_growth::GrowthMetadata {
                timestamp: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc).timestamp())
                    .unwrap_or(chrono::Utc::now().timestamp()),
                auto_detected: row.get::<_, i32>(5)? != 0,
                notes: row.get(6)?,
            },
        })
    }).map_err(|e| format!("Failed to query growth records: {}", e))?;

    let records: Vec<CharacterGrowth> = growth_records
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect growth records: {}", e))?;

    let character_name = records.get(0)
        .and_then(|r| conn.query_row(
            "SELECT name FROM characters WHERE id = ?1",
            params![r.character_id],
            |row| row.get::<_, String>(0)
        ).ok())
        .unwrap_or_default();

    let timeline = CharacterGrowthManager::build_timeline(
        records,
        &HashMap::new(),
        &character_name,
    );

    serde_json::to_string(&timeline).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn compare_growth_positions(
    app: AppHandle,
    character_id: String,
    from_position: i32,
    to_position: i32,
) -> Result<String, String> {
    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let from_record = get_growth_at_position(&conn, &character_id, from_position)?;
    let to_record = get_growth_at_position(&conn, &character_id, to_position)?;

    let comparison = CharacterGrowthManager::compare_growth_positions(&from_record, &to_record);

    serde_json::to_string(&comparison).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_character_tag(
    app: AppHandle,
    character_id: String,
    tag_type_json: String,
    name: String,
    value: Option<String>,
    description: Option<String>,
    color: String,
    weight_json: String,
    auto_assigned: bool,
    source_json: String,
) -> Result<String, String> {
    let tag_type: TagType = serde_json::from_str(&tag_type_json)
        .map_err(|e| format!("Failed to parse tag_type: {}", e))?;
    let weight: TagWeight = serde_json::from_str(&weight_json)
        .map_err(|e| format!("Failed to parse weight: {}", e))?;
    let source: TagSource = serde_json::from_str(&source_json)
        .map_err(|e| format!("Failed to parse source: {}", e))?;

    let tag = CharacterTagManager::create_tag(
        &character_id,
        tag_type,
        &name,
        value.as_deref(),
        description.as_deref(),
        &color,
        weight,
        auto_assigned,
        source,
    );

    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let created_at = chrono::Utc::now().to_rfc3339();
    let updated_at = created_at.clone();

    conn.execute(
        "INSERT INTO character_tags (id, character_id, tag_type, name, value, description, color, weight, auto_assigned, source, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            tag.id,
            tag.character_id,
            serde_json::to_string(&tag.tag_type).unwrap_or_default(),
            tag.name,
            tag.value,
            tag.description,
            tag.color,
            serde_json::to_string(&tag.weight).unwrap_or_default(),
            if auto_assigned { 1 } else { 0 },
            serde_json::to_string(&tag.metadata.source).unwrap_or_default(),
            created_at,
            updated_at,
        ],
    ).map_err(|e| format!("Failed to save tag: {}", e))?;

    serde_json::to_string(&tag).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_character_tags(
    app: AppHandle,
    character_id: String,
) -> Result<String, String> {
    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT id, character_id, tag_type, name, value, description, color, weight, auto_assigned, source, created_at, updated_at
         FROM character_tags
         WHERE character_id = ?1
         ORDER BY created_at DESC"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let tags = stmt.query_map(params![character_id], |row| {
        Ok(CharacterTag {
            id: row.get(0)?,
            character_id: row.get(1)?,
            tag_type: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or(TagType::Custom),
            name: row.get(3)?,
            value: row.get(4)?,
            description: row.get(5)?,
            color: row.get(6)?,
            weight: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or(TagWeight::Medium),
            metadata: crate::character_tags::TagMetadata {
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc).timestamp())
                    .unwrap_or(chrono::Utc::now().timestamp()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(11)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc).timestamp())
                    .unwrap_or(chrono::Utc::now().timestamp()),
                auto_assigned: row.get::<_, i32>(8)? != 0,
                source: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or(TagSource::Manual),
            },
        })
    }).map_err(|e| format!("Failed to query tags: {}", e))?;

    let tags_vec: Vec<CharacterTag> = tags
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect tags: {}", e))?;

    let character_name = conn.query_row(
        "SELECT name FROM characters WHERE id = ?1",
        params![character_id],
        |row| row.get::<_, String>(0)
    ).unwrap_or_default();

    let collection = CharacterTagCollection {
        character_id,
        character_name,
        tags: tags_vec.clone(),
        tag_groups: CharacterTagManager::organize_tags(tags_vec),
    };

    serde_json::to_string(&collection).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_character_tag(
    app: AppHandle,
    tag_id: String,
) -> Result<String, String> {
    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    conn.execute("DELETE FROM character_tags WHERE id = ?1", params![tag_id])
        .map_err(|e| format!("Failed to delete tag: {}", e))?;

    Ok("{\"status\":\"success\"}".to_string())
}

#[tauri::command]
pub async fn search_tags(
    app: AppHandle,
    project_id: String,
    query: Option<String>,
    tag_types_json: Option<String>,
    min_weight_json: Option<String>,
) -> Result<String, String> {
    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT t.id, t.character_id, t.tag_type, t.name, t.value, t.description, t.color, t.weight, t.auto_assigned, t.source, t.created_at, t.updated_at, ch.name
         FROM character_tags t
         JOIN characters ch ON t.character_id = ch.id
         WHERE ch.project_id = ?1"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let tags = stmt.query_map(params![project_id], |row| {
        Ok(CharacterTag {
            id: row.get(0)?,
            character_id: row.get(1)?,
            tag_type: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or(TagType::Custom),
            name: row.get(3)?,
            value: row.get(4)?,
            description: row.get(5)?,
            color: row.get(6)?,
            weight: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or(TagWeight::Medium),
            metadata: crate::character_tags::TagMetadata {
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc).timestamp())
                    .unwrap_or(chrono::Utc::now().timestamp()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(11)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc).timestamp())
                    .unwrap_or(chrono::Utc::now().timestamp()),
                auto_assigned: row.get::<_, i32>(8)? != 0,
                source: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or(TagSource::Manual),
            },
        })
    }).map_err(|e| format!("Failed to query tags: {}", e))?;

    let all_tags: Vec<CharacterTag> = tags
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect tags: {}", e))?;

    let character_names: HashMap<String, String> = conn.prepare(
        "SELECT id, name FROM characters WHERE project_id = ?1"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?
    .query_map(params![project_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| format!("Failed to query characters: {}", e))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("Failed to collect characters: {}", e))?
    .into_iter()
    .collect();

    let tag_types = tag_types_json.as_ref().and_then(|t| serde_json::from_str(t).ok());
    let min_weight = min_weight_json.as_ref().and_then(|w| serde_json::from_str(w).ok());

    let result = CharacterTagManager::search_tags(
        all_tags,
        &character_names,
        query.as_deref(),
        tag_types,
        min_weight,
    );

    serde_json::to_string(&result).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tag_library(
) -> Result<String, String> {
    let library = CharacterTagManager::get_tag_library();
    serde_json::to_string(&library).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tag_statistics(
    app: AppHandle,
    project_id: String,
) -> Result<String, String> {
    let db_path = get_db_path(&app)?;
    let conn = crate::database::get_connection(&db_path)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let mut stmt = conn.prepare(
        "SELECT t.id, t.character_id, t.tag_type, t.name, t.value, t.description, t.color, t.weight, t.auto_assigned, t.source, t.created_at, t.updated_at
         FROM character_tags t
         JOIN characters ch ON t.character_id = ch.id
         WHERE ch.project_id = ?1"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let tags = stmt.query_map(params![project_id], |row| {
        Ok(CharacterTag {
            id: row.get(0)?,
            character_id: row.get(1)?,
            tag_type: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or(TagType::Custom),
            name: row.get(3)?,
            value: row.get(4)?,
            description: row.get(5)?,
            color: row.get(6)?,
            weight: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or(TagWeight::Medium),
            metadata: crate::character_tags::TagMetadata {
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc).timestamp())
                    .unwrap_or(chrono::Utc::now().timestamp()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(11)?)
                    .map(|dt| dt.with_timezone(&chrono::Utc).timestamp())
                    .unwrap_or(chrono::Utc::now().timestamp()),
                auto_assigned: row.get::<_, i32>(8)? != 0,
                source: serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or(TagSource::Manual),
            },
        })
    }).map_err(|e| format!("Failed to query tags: {}", e))?;

    let tags_vec: Vec<CharacterTag> = tags
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect tags: {}", e))?;

    let character_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM characters WHERE project_id = ?1",
        params![project_id],
        |row| row.get(0)
    ).unwrap_or(0);

    let statistics = CharacterTagManager::calculate_statistics(tags_vec, character_count);

    serde_json::to_string(&statistics).map_err(|e| e.to_string())
}

fn get_db_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
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

fn get_growth_at_position(
    conn: &rusqlite::Connection,
    character_id: &str,
    position: i32,
) -> Result<CharacterGrowth, String> {
    conn.query_row(
        "SELECT id, character_id, chapter_id, position, changes_json, auto_detected, notes, created_at
         FROM character_growth_records
         WHERE character_id = ?1 AND position = ?2
         ORDER BY created_at DESC
         LIMIT 1",
        params![character_id, position],
        |row| {
            Ok(CharacterGrowth {
                id: row.get(0)?,
                character_id: row.get(1)?,
                chapter_id: row.get(2)?,
                position: row.get(3)?,
                changes: serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default(),
                metadata: crate::character_growth::GrowthMetadata {
                    timestamp: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                        .unwrap_or_else(|_| chrono::Utc::now().into())
                        .with_timezone(&chrono::Utc)
                        .timestamp(),
                    auto_detected: row.get::<_, i32>(5)? != 0,
                    notes: row.get(6)?,
                },
            })
        }
    ).map_err(|e| format!("Failed to query growth record: {}", e))
}
