use crate::character_dialogue::{
    CharacterDialogue, CharacterDialogueManager, DialogueSession, DialogueMessage,
    DialogueSettings, DialogueContext, DialogueMetadata, CharacterInfo
};
use crate::database::get_connection;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::State;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub character_id: String,
    pub chapter_id: Option<String>,
    pub session_name: String,
    pub system_prompt: Option<String>,
    pub ai_model: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub session_id: String,
    pub user_message: String,
    pub character_state: Option<HashMap<String, String>>,
    pub emotional_context: Option<String>,
    pub scene_context: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSessionRequest {
    pub session_id: String,
    pub session_name: Option<String>,
    pub system_prompt: Option<String>,
    pub context_summary: Option<String>,
    pub ai_model: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<i32>,
    pub is_active: Option<bool>,
}

#[tauri::command]
pub async fn create_dialogue_session(
    db_path: State<'_, String>,
    request: CreateSessionRequest,
) -> Result<DialogueSession> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let settings = DialogueSettings {
        ai_model: request.ai_model.unwrap_or_else(|| "default".to_string()),
        temperature: request.temperature.unwrap_or(0.7),
        max_tokens: request.max_tokens.unwrap_or(1000),
    };

    let chapter_id = request.chapter_id.clone().unwrap_or_default();
    let system_prompt = request.system_prompt.clone().unwrap_or_default();

    conn.execute(
        "INSERT INTO character_dialogue_sessions 
         (id, character_id, chapter_id, session_name, system_prompt, context_summary, 
          ai_model, temperature, max_tokens, is_active, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        rusqlite::params![
            &session_id,
            &request.character_id,
            &chapter_id,
            &request.session_name,
            &system_prompt,
            "",
            &settings.ai_model,
            &settings.temperature.to_string(),
            &settings.max_tokens.to_string(),
            1,
            &now,
            &now,
        ],
    ).map_err(|e| e.to_string())?;

    let session = DialogueSession {
        id: session_id,
        character_id: request.character_id,
        chapter_id: request.chapter_id,
        session_name: request.session_name,
        system_prompt: request.system_prompt,
        context_summary: None,
        messages: Vec::new(),
        settings,
        is_active: true,
        created_at: now.clone(),
        updated_at: now,
    };

    Ok(session)
}

#[tauri::command]
pub async fn get_dialogue_sessions(
    db_path: State<'_, String>,
    character_id: Option<String>,
    chapter_id: Option<String>,
) -> Result<Vec<DialogueSession>> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    let sessions_sql = if character_id.is_some() && chapter_id.is_some() {
        "SELECT * FROM character_dialogue_sessions WHERE character_id = ?1 AND chapter_id = ?2 ORDER BY updated_at DESC"
    } else if character_id.is_some() {
        "SELECT * FROM character_dialogue_sessions WHERE character_id = ?1 ORDER BY updated_at DESC"
    } else {
        "SELECT * FROM character_dialogue_sessions ORDER BY updated_at DESC"
    };

    let mut stmt = if character_id.is_some() && chapter_id.is_some() {
        conn.prepare(sessions_sql)
            .map_err(|e| e.to_string())?
    } else if character_id.is_some() {
        conn.prepare(sessions_sql)
            .map_err(|e| e.to_string())?
    } else {
        conn.prepare(sessions_sql)
            .map_err(|e| e.to_string())?
    };

    let sessions = if character_id.is_some() && chapter_id.is_some() {
        let mut sessions = Vec::new();
        let mut rows = stmt.query(rusqlite::params![character_id.unwrap(), chapter_id.unwrap()])
            .map_err(|e| e.to_string())?;
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let session_id: String = row.get::<_, String>(0).map_err(|e| e.to_string())?;
            let settings = DialogueSettings {
                ai_model: row.get::<_, String>(7).map_err(|e| e.to_string())?,
                temperature: row.get::<_, f64>(8).map_err(|e| e.to_string())?,
                max_tokens: row.get::<_, i32>(9).map_err(|e| e.to_string())?,
            };

            let messages = get_session_messages(&conn, &session_id)?;

            sessions.push(DialogueSession {
                id: row.get::<_, String>(0).map_err(|e| e.to_string())?,
                character_id: row.get::<_, String>(1).map_err(|e| e.to_string())?,
                chapter_id: {
                    let val: String = row.get::<_, String>(2).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                session_name: row.get::<_, String>(3).map_err(|e| e.to_string())?,
                system_prompt: {
                    let val: String = row.get::<_, String>(4).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                context_summary: {
                    let val: String = row.get::<_, String>(5).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                messages,
                settings,
                is_active: row.get::<_, bool>(10).map_err(|e| e.to_string())?,
                created_at: row.get::<_, String>(11).map_err(|e| e.to_string())?,
                updated_at: row.get::<_, String>(12).map_err(|e| e.to_string())?,
            });
        }
        sessions
    } else if character_id.is_some() {
        let mut sessions = Vec::new();
        let mut rows = stmt.query(rusqlite::params![character_id.unwrap()])
            .map_err(|e| e.to_string())?;
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let session_id: String = row.get::<_, String>(0).map_err(|e| e.to_string())?;
            let settings = DialogueSettings {
                ai_model: row.get::<_, String>(7).map_err(|e| e.to_string())?,
                temperature: row.get::<_, f64>(8).map_err(|e| e.to_string())?,
                max_tokens: row.get::<_, i32>(9).map_err(|e| e.to_string())?,
            };

            let messages = get_session_messages(&conn, &session_id)?;

            sessions.push(DialogueSession {
                id: row.get::<_, String>(0).map_err(|e| e.to_string())?,
                character_id: row.get::<_, String>(1).map_err(|e| e.to_string())?,
                chapter_id: {
                    let val: String = row.get::<_, String>(2).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                session_name: row.get::<_, String>(3).map_err(|e| e.to_string())?,
                system_prompt: {
                    let val: String = row.get::<_, String>(4).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                context_summary: {
                    let val: String = row.get::<_, String>(5).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                messages,
                settings,
                is_active: row.get::<_, bool>(10).map_err(|e| e.to_string())?,
                created_at: row.get::<_, String>(11).map_err(|e| e.to_string())?,
                updated_at: row.get::<_, String>(12).map_err(|e| e.to_string())?,
            });
        }
        sessions
    } else {
        let mut sessions = Vec::new();
        let mut rows = stmt.query(rusqlite::params![])
            .map_err(|e| e.to_string())?;
        while let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let session_id: String = row.get::<_, String>(0).map_err(|e| e.to_string())?;
            let settings = DialogueSettings {
                ai_model: row.get::<_, String>(7).map_err(|e| e.to_string())?,
                temperature: row.get::<_, f64>(8).map_err(|e| e.to_string())?,
                max_tokens: row.get::<_, i32>(9).map_err(|e| e.to_string())?,
            };

            let messages = get_session_messages(&conn, &session_id)?;

            sessions.push(DialogueSession {
                id: row.get::<_, String>(0).map_err(|e| e.to_string())?,
                character_id: row.get::<_, String>(1).map_err(|e| e.to_string())?,
                chapter_id: {
                    let val: String = row.get::<_, String>(2).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                session_name: row.get::<_, String>(3).map_err(|e| e.to_string())?,
                system_prompt: {
                    let val: String = row.get::<_, String>(4).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                context_summary: {
                    let val: String = row.get::<_, String>(5).map_err(|e| e.to_string())?;
                    if val.is_empty() { None } else { Some(val) }
                },
                messages,
                settings,
                is_active: row.get::<_, bool>(10).map_err(|e| e.to_string())?,
                created_at: row.get::<_, String>(11).map_err(|e| e.to_string())?,
                updated_at: row.get::<_, String>(12).map_err(|e| e.to_string())?,
            });
        }
        sessions
    };

    Ok(sessions)
}

#[tauri::command]
pub async fn get_dialogue_session(
    db_path: State<'_, String>,
    session_id: String,
) -> Result<DialogueSession> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    let session = conn.query_row(
        "SELECT * FROM character_dialogue_sessions WHERE id = ?1",
        rusqlite::params![session_id],
        |row| {
            let settings = DialogueSettings {
                ai_model: row.get::<_, String>(7)?,
                temperature: row.get::<_, f64>(8)?,
                max_tokens: row.get::<_, i32>(9)?,
            };

            Ok(DialogueSession {
                id: row.get::<_, String>(0)?,
                character_id: row.get::<_, String>(1)?,
                chapter_id: {
                    let val: String = row.get::<_, String>(2)?;
                    if val.is_empty() { None } else { Some(val) }
                },
                session_name: row.get::<_, String>(3)?,
                system_prompt: {
                    let val: String = row.get::<_, String>(4)?;
                    if val.is_empty() { None } else { Some(val) }
                },
                context_summary: {
                    let val: String = row.get::<_, String>(5)?;
                    if val.is_empty() { None } else { Some(val) }
                },
                messages: Vec::new(),
                settings,
                is_active: row.get::<_, bool>(10)?,
                created_at: row.get::<_, String>(11)?,
                updated_at: row.get::<_, String>(12)?,
            })
        },
    ).map_err(|e| e.to_string())?;

    let messages = get_session_messages(&conn, &session_id)?;

    Ok(DialogueSession {
        messages,
        ..session
    })
}

#[tauri::command]
pub async fn send_dialogue_message(
    db_path: State<'_, String>,
    request: SendMessageRequest,
) -> Result<CharacterDialogue> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();

    let character = get_character_info(&conn, &request.session_id)?;
    let system_prompt = get_session_system_prompt(&conn, &request.session_id)?;
    let conversation_history = get_session_messages(&conn, &request.session_id)?;

    let user_message_id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO character_dialogue_messages 
         (id, session_id, role, content, message_type, character_state_json, 
          emotional_context, scene_context, tokens_used, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        rusqlite::params![
            &user_message_id,
            &request.session_id,
            "user",
            &request.user_message,
            "text",
            &serde_json::to_string(&request.character_state).unwrap_or_default(),
            &request.emotional_context.clone().unwrap_or_default(),
            &request.scene_context.clone().unwrap_or_default(),
            0,
            &now,
        ],
    ).map_err(|e| e.to_string())?;

    let context = DialogueContext {
        character: character.clone(),
        conversation_history: conversation_history.clone(),
        current_emotion: request.emotional_context.clone(),
        scene_context: request.scene_context.clone(),
    };

    let metadata = DialogueMetadata {
        timestamp: Utc::now().timestamp(),
        model: get_session_model(&conn, &request.session_id)?,
        tokens_used: 0,
        generation_time: 0.0,
        quality_score: None,
    };

    let ai_response = CharacterDialogueManager::generate_ai_response(
        &character,
        &request.user_message,
        &context,
        &metadata,
    );

    let ai_message_id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO character_dialogue_messages 
         (id, session_id, role, content, message_type, character_state_json, 
          emotional_context, scene_context, tokens_used, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        rusqlite::params![
            &ai_message_id,
            &request.session_id,
            "assistant",
            &ai_response,
            "text",
            "",
            "",
            "",
            0,
            &now,
        ],
    ).map_err(|e| e.to_string())?;

    Ok(CharacterDialogue {
        id: ai_message_id,
        character_id: character.id,
        user_message: request.user_message,
        ai_response,
        context,
        metadata,
    })
}

#[tauri::command]
pub async fn update_dialogue_session(
    db_path: State<'_, String>,
    request: UpdateSessionRequest,
) -> Result<DialogueSession> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();

    if let Some(name) = &request.session_name {
        conn.execute(
            "UPDATE character_dialogue_sessions SET session_name = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![name, now, &request.session_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(prompt) = &request.system_prompt {
        conn.execute(
            "UPDATE character_dialogue_sessions SET system_prompt = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![prompt, now, &request.session_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(summary) = &request.context_summary {
        conn.execute(
            "UPDATE character_dialogue_sessions SET context_summary = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![summary, now, &request.session_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(model) = &request.ai_model {
        conn.execute(
            "UPDATE character_dialogue_sessions SET ai_model = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![model, now, &request.session_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(temp) = &request.temperature {
        conn.execute(
            "UPDATE character_dialogue_sessions SET temperature = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![temp, now, &request.session_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(tokens) = &request.max_tokens {
        conn.execute(
            "UPDATE character_dialogue_sessions SET max_tokens = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![tokens, now, &request.session_id],
        ).map_err(|e| e.to_string())?;
    }
    if let Some(active) = &request.is_active {
        let active_value = if *active { 1 } else { 0 };
        conn.execute(
            "UPDATE character_dialogue_sessions SET is_active = ?1, updated_at = ?2 WHERE id = ?3",
            rusqlite::params![active_value, now, &request.session_id],
        ).map_err(|e| e.to_string())?;
    }

    let session_id = request.session_id.clone();
    let session = get_dialogue_session(db_path, session_id).await?;

    Ok(session)
}

#[tauri::command]
pub async fn delete_dialogue_session(
    db_path: State<'_, String>,
    session_id: String,
) -> Result<bool> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM character_dialogue_sessions WHERE id = ?1",
        rusqlite::params![session_id],
    ).map_err(|e| e.to_string())?;

    Ok(true)
}

#[tauri::command]
pub async fn delete_dialogue_message(
    db_path: State<'_, String>,
    message_id: String,
) -> Result<bool> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM character_dialogue_messages WHERE id = ?1",
        [&message_id],
    ).map_err(|e| e.to_string())?;

    Ok(true)
}

#[tauri::command]
pub async fn regenerate_ai_response(
    db_path: State<'_, String>,
    message_id: String,
) -> Result<String> {
    let db_path_inner = db_path.inner().clone();
    let conn = get_connection(std::path::Path::new(&db_path_inner))
        .map_err(|e| e.to_string())?;

    let (session_id, user_message, character_state_json, emotional_context, scene_context) =
        conn.query_row(
            "SELECT session_id, content, character_state_json, emotional_context, scene_context
             FROM character_dialogue_messages
             WHERE id = ?1 AND role = 'user'",
            rusqlite::params![message_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        ).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM character_dialogue_messages WHERE id = ?1 OR 
         (session_id = ?2 AND created_at > (SELECT created_at FROM character_dialogue_messages WHERE id = ?1))",
        rusqlite::params![message_id, &session_id, message_id],
    ).map_err(|e| e.to_string())?;

    let character_state = if character_state_json.is_empty() {
        None
    } else {
        serde_json::from_str(&character_state_json).ok()
    };

    let request = SendMessageRequest {
        session_id: session_id.clone(),
        user_message,
        character_state,
        emotional_context: if emotional_context.is_empty() { None } else { Some(emotional_context) },
        scene_context: if scene_context.is_empty() { None } else { Some(scene_context) },
    };

    let dialogue = send_dialogue_message(db_path, request).await?;

    Ok(dialogue.ai_response)
}

fn get_session_messages(conn: &rusqlite::Connection, session_id: &str) -> Result<Vec<DialogueMessage>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM character_dialogue_messages
         WHERE session_id = ?1
         ORDER BY created_at ASC"
    ).map_err(|e| e.to_string())?;

    let mut messages = Vec::new();
    let mut rows = stmt.query(rusqlite::params![session_id]).map_err(|e| e.to_string())?;

    while let Some(row) = rows.next().map_err(|e| format!("Failed to get next row: {}", e))? {
        messages.push(DialogueMessage {
            id: row.get::<_, String>(0).map_err(|e| e.to_string())?,
            session_id: row.get::<_, String>(1).map_err(|e| e.to_string())?,
            role: row.get::<_, String>(2).map_err(|e| e.to_string())?,
            content: row.get::<_, String>(3).map_err(|e| e.to_string())?,
            message_type: row.get::<_, String>(4).map_err(|e| e.to_string())?,
            character_state: {
                let val: String = row.get::<_, String>(5).map_err(|e| e.to_string())?;
                if val.is_empty() { None } else { serde_json::from_str(&val).ok() }
            },
            emotional_context: {
                let val: String = row.get::<_, String>(6).map_err(|e| e.to_string())?;
                if val.is_empty() { None } else { Some(val) }
            },
            scene_context: {
                let val: String = row.get::<_, String>(7).map_err(|e| e.to_string())?;
                if val.is_empty() { None } else { Some(val) }
            },
            tokens_used: row.get::<_, i32>(8).map_err(|e| e.to_string())?,
            created_at: row.get::<_, String>(9).map_err(|e| e.to_string())?,
        });
    }

    Ok(messages)
}

fn get_character_info(conn: &rusqlite::Connection, session_id: &str) -> Result<CharacterInfo> {
    let character_id: String = conn.query_row(
        "SELECT character_id FROM character_dialogue_sessions WHERE id = ?1",
        rusqlite::params![session_id],
        |row| row.get::<_, String>(0)
    ).map_err(|e| e.to_string())?;

    let (name, role_type, personality, background) = conn.query_row(
        "SELECT name, role_type, personality, background FROM characters WHERE id = ?1",
        rusqlite::params![character_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        },
    ).map_err(|e| e.to_string())?;

    Ok(CharacterInfo {
        id: character_id,
        name,
        role_type,
        personality,
        background,
    })
}

fn get_session_system_prompt(conn: &rusqlite::Connection, session_id: &str) -> Result<Option<String>> {
    let prompt: String = conn.query_row(
        "SELECT system_prompt FROM character_dialogue_sessions WHERE id = ?1",
        rusqlite::params![session_id],
        |row| row.get::<_, String>(0)
    ).map_err(|e| e.to_string())?;

    Ok(if prompt.is_empty() { None } else { Some(prompt) })
}

fn get_session_model(conn: &rusqlite::Connection, session_id: &str) -> Result<String> {
    let model: String = conn.query_row(
        "SELECT ai_model FROM character_dialogue_sessions WHERE id = ?1",
        rusqlite::params![session_id],
        |row| row.get::<_, String>(0)
    ).map_err(|e| e.to_string())?;

    Ok(model)
}
