use tauri::{AppHandle, Manager};
use crate::models::{*, AIParams, APIKeyInfo, ModelInfo};
use crate::database::get_connection;
use crate::logger::{Logger, log_command_start, log_command_success, log_command_error};
use crate::ai::{ModelConfig, PromptTemplate};
use crate::ai::models::{
    AICompletionRequest, AIRewriteRequest,
    AIGenerateCharacterRequest, AIGenerateCharacterRelationsRequest,
    AIGenerateWorldViewRequest, AIGeneratePlotPointsRequest,
    AIGenerateStoryboardRequest, AIFormatContentRequest,
};
use crate::ai::service::AIService;
use crate::ai::{
    GeneratedCharacter, GeneratedCharacterRelation,
    GeneratedWorldView, GeneratedPlotPoint, GeneratedStoryboard,
};
use crate::export::{ExportFormat, ExportMetadata, ExportContent};
use crate::import::{ImportFormat, ImportResult, import_from_txt, import_from_markdown, import_from_docx};
use uuid::Uuid;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use rusqlite::{params, OptionalExtension};
use std::path::PathBuf;

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

#[tauri::command]
pub async fn create_project(app: AppHandle, request: CreateProjectRequest) -> Result<Project, String> {
    let logger = Logger::new().with_feature("project-service");
    log_command_start(&logger, "create_project", &format!("{:?}", request));

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let project = Project {
        id: id.clone(),
        name: request.name.clone(),
        description: request.description,
        genre: request.genre,
        template: request.template,
        status: "active".to_string(),
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    conn.execute(
        "INSERT INTO projects (id, name, description, genre, template, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            project.id,
            project.name,
            project.description,
            project.genre,
            project.template,
            project.status,
            project.created_at,
            project.updated_at,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to insert project: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "create_project", &format!("Created project: {}", project.id));
    Ok(project)
}

#[tauri::command]
pub async fn get_projects(app: AppHandle) -> Result<Vec<Project>, String> {
    let logger = Logger::new().with_feature("project-service");
    log_command_start(&logger, "get_projects", "");

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn
        .prepare("SELECT id, name, description, genre, template, status, created_at, updated_at FROM projects ORDER BY updated_at DESC")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let projects_iter = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                genre: row.get(3)?,
                template: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| {
            logger.error(&format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

    let mut projects = Vec::new();
    for project in projects_iter {
        projects.push(project.map_err(|e| {
            logger.error(&format!("Failed to map project: {}", e));
            e.to_string()
        })?);
    }

    log_command_success(&logger, "get_projects", &format!("Retrieved {} projects", projects.len()));
    Ok(projects)
}

#[tauri::command]
pub async fn delete_project(app: AppHandle, projectId: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("project-service");
    log_command_start(&logger, "delete_project", &format!("projectId: {}", projectId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "DELETE FROM projects WHERE id = ?",
        [&projectId],
    ).map_err(|e| {
        logger.error(&format!("Failed to delete project: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "delete_project", &format!("Deleted project: {}", projectId));
    Ok(())
}

#[tauri::command]
pub async fn update_project(
    app: AppHandle,
    projectId: String,
    name: Option<String>,
    description: Option<String>,
    genre: Option<String>,
    template: Option<String>,
) -> Result<Project, String> {
    let logger = Logger::new().with_feature("project-service");
    log_command_start(&logger, "update_project", &format!("projectId: {}", projectId));

    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "UPDATE projects SET name = COALESCE(?, name), description = COALESCE(?, description), genre = COALESCE(?, genre), template = COALESCE(?, template), updated_at = ? WHERE id = ?",
        params![name, description, genre, template, now, projectId],
    ).map_err(|e| {
        logger.error(&format!("Failed to update project: {}", e));
        e.to_string()
    })?;

    let mut stmt = conn
        .prepare("SELECT id, name, description, genre, template, status, created_at, updated_at FROM projects WHERE id = ?")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let project = stmt
        .query_row(&[&projectId], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                genre: row.get(3)?,
                template: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| {
            log_command_error(&logger, "update_project", &format!("Failed to fetch updated project: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "update_project", &format!("Updated project: {}", projectId));
    Ok(project)
}

#[tauri::command]
pub async fn save_chapter(app: AppHandle, request: SaveChapterRequest) -> Result<Chapter, String> {
    let logger = Logger::new().with_feature("chapter-service");
    log_command_start(&logger, "save_chapter", &format!("{:?}", request));

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let word_count = request.content.chars().count() as i32;
    let sort_order = request.sort_order.unwrap_or(0);

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let chapter = Chapter {
        id: id.clone(),
        project_id: request.project_id.clone(),
        title: request.title.clone(),
        content: request.content.clone(),
        word_count,
        sort_order,
        status: "draft".to_string(),
        created_at: now.clone(),
        updated_at: now.clone(),
        versions: None,
        evaluation: None,
        summary: None,
        generation_status: None,
    };

    conn.execute(
        "INSERT INTO chapters (id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
            None::<String>,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to insert chapter: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "save_chapter", &format!("Created chapter: {}", chapter.id));
    Ok(chapter)
}

#[tauri::command]
pub async fn get_chapters(app: AppHandle, projectId: String) -> Result<Vec<Chapter>, String> {
    let logger = Logger::new().with_feature("chapter-service");
    log_command_start(&logger, "get_chapters", &format!("projectId: {}", projectId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary FROM chapters WHERE project_id = ? ORDER BY sort_order ASC")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let chapters_iter = stmt
        .query_map(&[&projectId], |row| {
            Ok(Chapter {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                word_count: row.get(4)?,
                sort_order: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                versions: None,
                evaluation: None,
                generation_status: None,
                summary: row.get(9).ok(),
            })
        })
        .map_err(|e| {
            logger.error(&format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

    let mut chapters = Vec::new();
    for chapter in chapters_iter {
        chapters.push(chapter.map_err(|e| {
            logger.error(&format!("Failed to map chapter: {}", e));
            e.to_string()
        })?);
    }

    log_command_success(&logger, "get_chapters", &format!("Retrieved {} chapters", chapters.len()));
    Ok(chapters)
}

#[tauri::command]
pub async fn get_chapter(app: AppHandle, chapterId: String) -> Result<Chapter, String> {
    let logger = Logger::new().with_feature("chapter-service");
    log_command_start(&logger, "get_chapter", &format!("chapterId: {}", chapterId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary FROM chapters WHERE id = ?")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let chapter = stmt
        .query_row(&[&chapterId], |row| {
            Ok(Chapter {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                word_count: row.get(4)?,
                sort_order: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                versions: None,
                evaluation: None,
                generation_status: None,
                summary: row.get(9).ok(),
            })
        })
        .map_err(|e| {
            log_command_error(&logger, "get_chapter", &format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "get_chapter", &format!("Retrieved chapter: {}", chapterId));
    Ok(chapter)
}

#[tauri::command]
pub async fn update_chapter(
    app: AppHandle,
    chapterId: String,
    title: Option<String>,
    content: Option<String>,
) -> Result<Chapter, String> {
    let logger = Logger::new().with_feature("chapter-service");
    log_command_start(&logger, "update_chapter", &format!("chapterId: {}", chapterId));

    let now = Utc::now().to_rfc3339();
    let word_count = content.as_ref().map(|c| c.chars().count() as i32);

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "UPDATE chapters SET title = COALESCE(?, title), content = COALESCE(?, content), word_count = COALESCE(?, word_count), updated_at = ? WHERE id = ?",
        params![title, content, word_count, now, chapterId],
    ).map_err(|e| {
        logger.error(&format!("Failed to update chapter: {}", e));
        e.to_string()
    })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary FROM chapters WHERE id = ?")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let chapter = stmt
        .query_row(&[&chapterId], |row| {
            Ok(Chapter {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                content: row.get(3)?,
                word_count: row.get(4)?,
                sort_order: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                versions: None,
                evaluation: None,
                generation_status: None,
                summary: row.get(9).ok(),
            })
        })
        .map_err(|e| {
            log_command_error(&logger, "update_chapter", &format!("Failed to fetch updated chapter: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "update_chapter", &format!("Updated chapter: {}", chapterId));
    Ok(chapter)
}

#[tauri::command]
pub async fn delete_chapter(app: AppHandle, chapterId: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("chapter-service");
    log_command_start(&logger, "delete_chapter", &format!("chapterId: {}", chapterId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "DELETE FROM chapters WHERE id = ?",
        [&chapterId],
    ).map_err(|e| {
        logger.error(&format!("Failed to delete chapter: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "delete_chapter", &format!("Deleted chapter: {}", chapterId));
    Ok(())
}

#[tauri::command]
pub async fn create_character(app: AppHandle, request: CreateCharacterRequest) -> Result<Character, String> {
    let logger = Logger::new().with_feature("character-service");
    log_command_start(&logger, "create_character", &format!("{:?}", request));

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let character = Character {
        id: id.clone(),
        project_id: request.project_id.clone(),
        name: request.name.clone(),
        role_type: request.role_type,
        race: request.race,
        age: request.age,
        gender: request.gender,
        birth_date: request.birth_date,
        appearance: request.appearance,
        personality: request.personality,
        background: request.background,
        skills: request.skills,
        status: request.status,
        bazi: request.bazi,
        ziwei: request.ziwei,
        mbti: request.mbti,
        enneagram: request.enneagram,
        items: request.items,
        avatar_url: None,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    conn.execute(
        "INSERT INTO characters (id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    ).map_err(|e| {
        logger.error(&format!("Failed to insert character: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "create_character", &format!("Created character: {}", character.id));
    Ok(character)
}

#[tauri::command]
pub async fn get_characters(app: AppHandle, projectId: String) -> Result<Vec<Character>, String> {
    let logger = Logger::new().with_feature("character-service");
    log_command_start(&logger, "get_characters", &format!("projectId: {}", projectId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at FROM characters WHERE project_id = ? ORDER BY created_at DESC")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let characters_iter = stmt
        .query_map(&[&projectId], |row| {
            Ok(Character {
                id: row.get(0)?,
                project_id: row.get(1)?,
                name: row.get(2)?,
                role_type: row.get(3)?,
                race: row.get(4)?,
                age: row.get(5)?,
                gender: row.get(6)?,
                birth_date: row.get(7)?,
                appearance: row.get(8)?,
                personality: row.get(9)?,
                background: row.get(10)?,
                skills: row.get(11)?,
                status: row.get(12)?,
                bazi: row.get(13)?,
                ziwei: row.get(14)?,
                mbti: row.get(15)?,
                enneagram: row.get(16)?,
                items: row.get(17)?,
                avatar_url: row.get(18)?,
                created_at: row.get(19)?,
                updated_at: row.get(20)?,
            })
        })
        .map_err(|e| {
            logger.error(&format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

    let mut characters = Vec::new();
    for character in characters_iter {
        characters.push(character.map_err(|e| {
            logger.error(&format!("Failed to map character: {}", e));
            e.to_string()
        })?);
    }

    log_command_success(&logger, "get_characters", &format!("Retrieved {} characters", characters.len()));
    Ok(characters)
}

#[tauri::command]
pub async fn update_character(app: AppHandle, characterId: String, update: serde_json::Value) -> Result<Character, String> {
    let logger = Logger::new().with_feature("character-service");
    log_command_start(&logger, "update_character", &format!("characterId: {}, update: {:?}", characterId, update));

    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let name = update.get("name").and_then(|v| v.as_str());
    let role_type = update.get("role_type").and_then(|v| v.as_str());
    let race = update.get("race").and_then(|v| v.as_str());
    let age = update.get("age").and_then(|v| v.as_i64());
    let gender = update.get("gender").and_then(|v| v.as_str());
    let birth_date = update.get("birth_date").and_then(|v| v.as_str());
    let appearance = update.get("appearance").and_then(|v| v.as_str());
    let personality = update.get("personality").and_then(|v| v.as_str());
    let background = update.get("background").and_then(|v| v.as_str());
    let skills = update.get("skills").and_then(|v| v.as_str());
    let status = update.get("status").and_then(|v| v.as_str());
    let bazi = update.get("bazi").and_then(|v| v.as_str());
    let ziwei = update.get("ziwei").and_then(|v| v.as_str());
    let mbti = update.get("mbti").and_then(|v| v.as_str());
    let enneagram = update.get("enneagram").and_then(|v| v.as_str());
    let items = update.get("items").and_then(|v| v.as_str());

    conn.execute(
        "UPDATE characters SET name = COALESCE(?, name), role_type = COALESCE(?, role_type), race = COALESCE(?, race), age = COALESCE(?, age), gender = COALESCE(?, gender), birth_date = COALESCE(?, birth_date), appearance = COALESCE(?, appearance), personality = COALESCE(?, personality), background = COALESCE(?, background), skills = COALESCE(?, skills), status = COALESCE(?, status), bazi = COALESCE(?, bazi), ziwei = COALESCE(?, ziwei), mbti = COALESCE(?, mbti), enneagram = COALESCE(?, enneagram), items = COALESCE(?, items), updated_at = ? WHERE id = ?",
        params![name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, now, characterId],
    )
        .map_err(|e| {
            logger.error(&format!("Failed to update character: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at FROM characters WHERE id = ?")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let character = stmt
        .query_row(&[&characterId], |row| {
            Ok(Character {
                id: row.get(0)?,
                project_id: row.get(1)?,
                name: row.get(2)?,
                role_type: row.get(3)?,
                race: row.get(4)?,
                age: row.get(5)?,
                gender: row.get(6)?,
                birth_date: row.get(7)?,
                appearance: row.get(8)?,
                personality: row.get(9)?,
                background: row.get(10)?,
                skills: row.get(11)?,
                status: row.get(12)?,
                bazi: row.get(13)?,
                ziwei: row.get(14)?,
                mbti: row.get(15)?,
                enneagram: row.get(16)?,
                items: row.get(17)?,
                avatar_url: row.get(18)?,
                created_at: row.get(19)?,
                updated_at: row.get(20)?,
            })
        })
        .map_err(|e| {
            log_command_error(&logger, "update_character", &format!("Failed to fetch updated character: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "update_character", &format!("Updated character: {}", characterId));
    Ok(character)
}

#[tauri::command]
pub async fn delete_character(app: AppHandle, characterId: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("character-service");
    log_command_start(&logger, "delete_character", &format!("characterId: {}", characterId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "DELETE FROM characters WHERE id = ?",
        [&characterId],
    ).map_err(|e| {
        logger.error(&format!("Failed to delete character: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "delete_character", &format!("Deleted character: {}", characterId));
    Ok(())
}

#[tauri::command]
pub async fn create_plot_point(app: AppHandle, request: CreatePlotPointRequest) -> Result<PlotPoint, String> {
    let logger = Logger::new().with_feature("plot-point-service");
    log_command_start(&logger, "create_plot_point", &format!("{:?}", request));

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let parent_id = request.parent_id.clone();

    let plot_point = PlotPoint {
        id: id.clone(),
        project_id: request.project_id.clone(),
        parent_id,
        title: request.title.clone(),
        description: request.description,
        note: request.note,
        chapter_id: request.chapter_id,
        status: "draft".to_string(),
        sort_order: request.sort_order.unwrap_or(0),
        level: 0,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    conn.execute(
        "INSERT INTO plot_points (id, project_id, parent_id, title, description, note, chapter_id, status, sort_order, level, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    ).map_err(|e| {
        logger.error(&format!("Failed to insert plot point: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "create_plot_point", &format!("Created plot point: {}", plot_point.id));
    Ok(plot_point)
}

#[tauri::command]
pub async fn get_plot_points(app: AppHandle, projectId: String) -> Result<Vec<PlotPoint>, String> {
    let logger = Logger::new().with_feature("plot-point-service");
    log_command_start(&logger, "get_plot_points", &format!("projectId: {}", projectId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, parent_id, title, description, note, chapter_id, status, sort_order, level, created_at, updated_at FROM plot_points WHERE project_id = ? ORDER BY sort_order ASC")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let plot_points_iter = stmt
        .query_map(&[&projectId], |row| {
            Ok(PlotPoint {
                id: row.get(0)?,
                project_id: row.get(1)?,
                parent_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                note: row.get(5)?,
                chapter_id: row.get(6)?,
                status: row.get(7)?,
                sort_order: row.get(8)?,
                level: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| {
            logger.error(&format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

    let mut plot_points = Vec::new();
    for plot_point in plot_points_iter {
        plot_points.push(plot_point.map_err(|e| {
            logger.error(&format!("Failed to map plot point: {}", e));
            e.to_string()
        })?);
    }

    log_command_success(&logger, "get_plot_points", &format!("Retrieved {} plot points", plot_points.len()));
    Ok(plot_points)
}

#[tauri::command]
pub async fn update_plot_point(app: AppHandle, request: UpdatePlotPointRequest) -> Result<PlotPoint, String> {
    let logger = Logger::new().with_feature("plot-point-service");
    log_command_start(&logger, "update_plot_point", &format!("{:?}", request));

    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "UPDATE plot_points SET title = COALESCE(?, title), description = COALESCE(?, description), note = COALESCE(?, note), chapter_id = COALESCE(?, chapter_id), status = COALESCE(?, status), sort_order = COALESCE(?, sort_order), parent_id = COALESCE(?, parent_id), updated_at = ? WHERE id = ?",
        params![request.title, request.description, request.note, request.chapter_id, request.status, request.sort_order, request.parent_id, now, request.id],
    ).map_err(|e| {
        logger.error(&format!("Failed to update plot point: {}", e));
        e.to_string()
    })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, parent_id, title, description, note, chapter_id, status, sort_order, level, created_at, updated_at FROM plot_points WHERE id = ?")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let plot_point = stmt
        .query_row(&[&request.id], |row| {
            Ok(PlotPoint {
                id: row.get(0)?,
                project_id: row.get(1)?,
                parent_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                note: row.get(5)?,
                chapter_id: row.get(6)?,
                status: row.get(7)?,
                sort_order: row.get(8)?,
                level: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| {
            log_command_error(&logger, "update_plot_point", &format!("Failed to fetch updated plot point: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "update_plot_point", &format!("Updated plot point: {}", request.id));
    Ok(plot_point)
}

#[tauri::command]
pub async fn delete_plot_point(app: AppHandle, plotPointId: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("plot-point-service");
    log_command_start(&logger, "delete_plot_point", &format!("plotPointId: {}", plotPointId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "DELETE FROM plot_points WHERE id = ?",
        [&plotPointId],
    ).map_err(|e| {
        logger.error(&format!("Failed to delete plot point: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "delete_plot_point", &format!("Deleted plot point: {}", plotPointId));
    Ok(())
}

#[tauri::command]
pub async fn create_character_relation(app: AppHandle, request: CreateCharacterRelationRequest) -> Result<CharacterRelation, String> {
    let logger = Logger::new().with_feature("character-relation-service");
    log_command_start(&logger, "create_character_relation", &format!("{:?}", request));

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let relation = CharacterRelation {
        id: id.clone(),
        project_id: request.project_id.clone(),
        from_character_id: request.from_character_id.clone(),
        to_character_id: request.to_character_id.clone(),
        relation_type: request.relation_type.clone(),
        description: request.description,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    conn.execute(
        "INSERT INTO character_relations (id, project_id, from_character_id, to_character_id, relation_type, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            relation.id,
            relation.project_id,
            relation.from_character_id,
            relation.to_character_id,
            relation.relation_type,
            relation.description,
            relation.created_at,
            relation.updated_at,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to insert character relation: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "create_character_relation", &format!("Created character relation: {}", relation.id));
    Ok(relation)
}

#[tauri::command]
pub async fn get_character_relations(app: AppHandle, projectId: String) -> Result<Vec<CharacterRelation>, String> {
    let logger = Logger::new().with_feature("character-relation-service");
    log_command_start(&logger, "get_character_relations", &format!("projectId: {}", projectId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, from_character_id, to_character_id, relation_type, description, created_at, updated_at FROM character_relations WHERE project_id = ? ORDER BY created_at DESC")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let relations_iter = stmt
        .query_map(&[&projectId], |row| {
            Ok(CharacterRelation {
                id: row.get(0)?,
                project_id: row.get(1)?,
                from_character_id: row.get(2)?,
                to_character_id: row.get(3)?,
                relation_type: row.get(4)?,
                description: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| {
            logger.error(&format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

    let mut relations = Vec::new();
    for relation in relations_iter {
        relations.push(relation.map_err(|e| {
            logger.error(&format!("Failed to map character relation: {}", e));
            e.to_string()
        })?);
    }

    log_command_success(&logger, "get_character_relations", &format!("Retrieved {} character relations", relations.len()));
    Ok(relations)
}

#[tauri::command]
pub async fn update_character_relation(app: AppHandle, request: UpdateCharacterRelationRequest) -> Result<CharacterRelation, String> {
    let logger = Logger::new().with_feature("character-relation-service");
    log_command_start(&logger, "update_character_relation", &format!("{:?}", request));

    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "UPDATE character_relations SET relation_type = COALESCE(?, relation_type), description = COALESCE(?, description), updated_at = ? WHERE id = ?",
        params![request.relation_type, request.description, now, request.id],
    ).map_err(|e| {
        logger.error(&format!("Failed to update character relation: {}", e));
        e.to_string()
    })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, from_character_id, to_character_id, relation_type, description, created_at, updated_at FROM character_relations WHERE id = ?")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let relation = stmt
        .query_row(&[&request.id], |row| {
            Ok(CharacterRelation {
                id: row.get(0)?,
                project_id: row.get(1)?,
                from_character_id: row.get(2)?,
                to_character_id: row.get(3)?,
                relation_type: row.get(4)?,
                description: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .map_err(|e| {
            log_command_error(&logger, "update_character_relation", &format!("Failed to fetch updated character relation: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "update_character_relation", &format!("Updated: {}", request.id));
    Ok(relation)
}

#[tauri::command]
pub async fn delete_character_relation(app: AppHandle, id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("character-relation-service");
    log_command_start(&logger, "delete_character_relation", &format!("id: {}", id));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "DELETE FROM character_relations WHERE id = ?",
        [&id],
    ).map_err(|e| {
        logger.error(&format!("Failed to delete character relation from database: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "delete_character_relation", &format!("Deleted: {}", id));
    Ok(())
}

#[tauri::command]
pub async fn create_world_view(app: AppHandle, request: CreateWorldViewRequest) -> Result<WorldView, String> {
    let logger = Logger::new().with_feature("worldview-service");
    log_command_start(&logger, "create_world_view", &format!("{:?}", request));

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let world_view = WorldView {
        id: id.clone(),
        project_id: request.project_id.clone(),
        category: request.category.clone(),
        title: request.title.clone(),
        content: request.content.clone(),
        tags: request.tags,
        status: "draft".to_string(),
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    conn.execute(
        "INSERT INTO world_views (id, project_id, category, title, content, tags, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
    ).map_err(|e| {
        logger.error(&format!("Failed to insert world view: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "create_world_view", &format!("Created world view: {}", world_view.id));
    Ok(world_view)
}

#[tauri::command]
pub async fn get_world_views(app: AppHandle, projectId: String, category: Option<String>) -> Result<Vec<WorldView>, String> {
    let logger = Logger::new().with_feature("worldview-service");
    log_command_start(&logger, "get_world_views", &format!("projectId: {}, category: {:?}", projectId, category));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let views = if let Some(cat) = &category {
        let mut stmt = conn.prepare(
            "SELECT id, project_id, category, title, content, tags, status, created_at, updated_at FROM world_views WHERE project_id = ? AND category = ? ORDER BY updated_at DESC"
        )
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

        let rows = stmt.query_map([&projectId as &dyn rusqlite::ToSql, cat as &dyn rusqlite::ToSql], |row| {
            Ok(WorldView {
                id: row.get(0)?,
                project_id: row.get(1)?,
                category: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                tags: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| {
            logger.error(&format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| {
                logger.error(&format!("Failed to map world view: {}", e));
                e.to_string()
            })?);
        }
        result
    } else {
        let mut stmt = conn.prepare(
            "SELECT id, project_id, category, title, content, tags, status, created_at, updated_at FROM world_views WHERE project_id = ? ORDER BY updated_at DESC"
        )
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

        let rows = stmt.query_map([&projectId as &dyn rusqlite::ToSql], |row| {
            Ok(WorldView {
                id: row.get(0)?,
                project_id: row.get(1)?,
                category: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                tags: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| {
            logger.error(&format!("Failed to execute query: {}", e));
            e.to_string()
        })?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| {
                logger.error(&format!("Failed to map world view: {}", e));
                e.to_string()
            })?);
        }
        result
    };

    log_command_success(&logger, "get_world_views", &format!("Retrieved {} world views", views.len()));
    Ok(views)
}

#[tauri::command]
pub async fn update_world_view(app: AppHandle, request: UpdateWorldViewRequest) -> Result<WorldView, String> {
    let logger = Logger::new().with_feature("worldview-service");
    log_command_start(&logger, "update_world_view", &format!("{:?}", request));

    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "UPDATE world_views SET category = COALESCE(?, category), title = COALESCE(?, title), content = COALESCE(?, content), tags = COALESCE(?, tags), status = COALESCE(?, status), updated_at = ? WHERE id = ?",
        params![request.category, request.title, request.content, request.tags, request.status, now, request.id],
    ).map_err(|e| {
        logger.error(&format!("Failed to update world view: {}", e));
        e.to_string()
    })?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, category, title, content, tags, status, created_at, updated_at FROM world_views WHERE id = ?")
        .map_err(|e| {
            logger.error(&format!("Failed to prepare statement: {}", e));
            e.to_string()
        })?;

    let world_view = stmt
        .query_row(&[&request.id], |row| {
            Ok(WorldView {
                id: row.get(0)?,
                project_id: row.get(1)?,
                category: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                tags: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| {
            log_command_error(&logger, "update_world_view", &format!("Failed to fetch updated world view: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "update_world_view", &format!("Updated world view: {}", request.id));
    Ok(world_view)
}

#[tauri::command]
pub async fn delete_world_view(app: AppHandle, id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("worldview-service");
    log_command_start(&logger, "delete_world_view", &format!("id: {}", id));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    conn.execute(
        "DELETE FROM world_views WHERE id = ?",
        [&id],
    ).map_err(|e| {
        logger.error(&format!("Failed to delete world view: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "delete_world_view", &format!("Deleted world view: {}", id));
    Ok(())
}

#[tauri::command]
pub async fn get_character_graph(
    app: AppHandle,
    projectId: String,
) -> Result<CharacterGraph, String> {
    let logger = Logger::new().with_feature("character-graph-service");
    log_command_start(&logger, "get_character_graph", &format!("projectId: {}", projectId));

    let db_path = get_db_path(&app)?;

    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

    let mut stmt = conn.prepare(
        "SELECT id, project_id, name, avatar_url FROM characters WHERE project_id = ?"
    )
    .map_err(|e| {
        logger.error(&format!("Failed to prepare statement: {}", e));
        e.to_string()
    })?;

    let character_iter = stmt.query_map([&projectId], |row| {
        Ok(CharacterNode {
            id: row.get(0)?,
            name: row.get(2)?,
            avatar_url: row.get(3)?,
        })
    })
    .map_err(|e| {
        logger.error(&format!("Failed to query characters: {}", e));
        e.to_string()
    })?;

    let mut nodes = Vec::new();
    for character in character_iter {
        nodes.push(character.map_err(|e| {
            logger.error(&format!("Failed to map character: {}", e));
            e.to_string()
        })?);
    }

    let mut stmt = conn.prepare(
        "SELECT cr.id, cr.from_character_id, cr.to_character_id, cr.relation_type, cr.description, c1.name, c2.name FROM character_relations cr JOIN characters c1 ON cr.from_character_id = c1.id JOIN characters c2 ON cr.to_character_id = c2.id WHERE cr.project_id = ?"
    )
    .map_err(|e| {
        logger.error(&format!("Failed to prepare statement: {}", e));
        e.to_string()
    })?;

    let edge_iter = stmt.query_map([&projectId], |row| {
        Ok(CharacterEdge {
            id: row.get(0)?,
            from: row.get(1)?,
            to: row.get(2)?,
            label: row.get(3)?,
            description: row.get(4)?,
        })
    })
    .map_err(|e| {
        logger.error(&format!("Failed to query relations: {}", e));
        e.to_string()
    })?;

    let mut edges = Vec::new();
    for edge in edge_iter {
        edges.push(edge.map_err(|e| {
            logger.error(&format!("Failed to map relation: {}", e));
            e.to_string()
        })?);
    }

    let node_count = nodes.len();
    let edge_count = edges.len();
    let graph = CharacterGraph { nodes, edges };
    log_command_success(&logger, "get_character_graph", &format!("Retrieved graph with {} nodes and {} edges", node_count, edge_count));
    Ok(graph)
}

#[tauri::command]
pub async fn register_openai_model(
    app: AppHandle,
    request: ModelConfig,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("ai-model-service");
    log_command_start(&logger, "register_openai_model", &format!("{:?}", request));

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let openai_adapter = crate::ai::OpenAIAdapter::new(
        request.api_key.unwrap_or_default(),
        request.name.clone()
    ).with_base_url(request.api_endpoint);
    
    let model_arc = std::sync::Arc::new(openai_adapter) as std::sync::Arc<dyn crate::ai::AIModel>;
    service.get_registry().register_model(request.id.clone(), model_arc).await;

    log_command_success(&logger, "register_openai_model", &format!("OpenAI model registered: {}", request.id));
    Ok(())
}

#[tauri::command]
pub async fn register_ollama_model(
    app: AppHandle,
    request: ModelConfig,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("ai-model-service");
    log_command_start(&logger, "register_ollama_model", &format!("{:?}", request));

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let ollama_adapter = crate::ai::OllamaAdapter::new(request.name.clone())
        .with_base_url(request.api_endpoint);
    
    let model_arc = std::sync::Arc::new(ollama_adapter) as std::sync::Arc<dyn crate::ai::AIModel>;
    service.get_registry().register_model(request.id.clone(), model_arc).await;

    log_command_success(&logger, "register_ollama_model", &format!("Ollama model registered: {}", request.id));
    Ok(())
}

#[tauri::command]
pub async fn get_models(
    app: AppHandle,
) -> Result<Vec<String>, String> {
    let logger = Logger::new().with_feature("ai-model-service");
    log_command_start(&logger, "get_models", "");

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let models = service.get_registry().list_models().await;

    log_command_success(&logger, "get_models", &format!("Retrieved {} models", models.len()));
    Ok(models)
}

#[tauri::command]
pub async fn ai_continue_novel(
    app: AppHandle,
    mut request: AICompletionRequest,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("ai-novel-service");
    log_command_start(&logger, "ai_continue_novel", &format!("model={}, chapter_mission_id={:?}", request.model_id, request.chapter_mission_id));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    // L3写作层：如果有chapter_mission_id，获取导演脚本
    let mut mission_context: Option<String> = None;
    let mut allowed_new_characters: Vec<String> = vec![];
    let mut forbidden_characters: Vec<String> = vec![];
    let mut director_pov: Option<String> = None;
    let mut director_tone: Option<String> = None;
    let mut director_pacing: Option<String> = None;

    if let Some(ref mission_id) = request.chapter_mission_id {
        let mut stmt = conn
            .prepare(
                "SELECT macro_beat, micro_beats, pov, tone, pacing, allowed_new_characters, forbidden_characters, beat_id
                 FROM chapter_missions WHERE id = ?"
            )
            .map_err(|e| e.to_string())?;

        if let Ok((macro_beat, micro_beats, pov, tone, pacing, allowed_new_chars, forbidden_chars, _beat_id)) =
            stmt.query_row([mission_id], |row| {
                let macro_beat: String = row.get(0)?;
                let micro_beats_json: String = row.get(1)?;
                let pov: Option<String> = row.get(2)?;
                let tone: Option<String> = row.get(3)?;
                let pacing: Option<String> = row.get(4)?;
                let allowed_new_chars_json: String = row.get(5)?;
                let forbidden_chars_json: String = row.get(6)?;
                let _beat_id: Option<String> = row.get(7)?;

                let micro_beats: Vec<String> = serde_json::from_str(&micro_beats_json).unwrap_or_default();
                let allowed_new_chars: Vec<String> = serde_json::from_str(&allowed_new_chars_json).unwrap_or_default();
                let forbidden_chars: Vec<String> = serde_json::from_str(&forbidden_chars_json).unwrap_or_default();

                Ok((macro_beat, micro_beats, pov, tone, pacing, allowed_new_chars, forbidden_chars, _beat_id))
            }) {
            director_pov = pov.clone();
            director_tone = tone.clone();
            director_pacing = pacing.clone();
            allowed_new_characters = allowed_new_chars.clone();
            forbidden_characters = forbidden_chars.clone();

            // 构建导演脚本上下文
            let mut mission_parts = vec![];
            mission_parts.push("【章节导演脚本】".to_string());
            mission_parts.push(format!("宏观节拍: {}", macro_beat));
            if !micro_beats.is_empty() {
                mission_parts.push("微观节拍:".to_string());
                for (i, beat) in micro_beats.iter().enumerate() {
                    mission_parts.push(format!("  {}. {}", i + 1, beat));
                }
            }
            if let Some(p) = &pov { mission_parts.push(format!("视角: {}", p)); }
            if let Some(t) = &tone { mission_parts.push(format!("基调: {}", t)); }
            if let Some(p) = &pacing { mission_parts.push(format!("节奏: {}", p)); }
            if !allowed_new_characters.is_empty() {
                mission_parts.push(format!("可登场新角色: {}", allowed_new_characters.join(", ")));
            }
            if !forbidden_characters.is_empty() {
                mission_parts.push(format!("禁止登场角色: {}", forbidden_characters.join(", ")));
            }

            mission_context = Some(mission_parts.join("\n"));
            logger.info(&format!("Loaded chapter mission: POV={:?}, Tone={:?}, Pacing={:?}", pov, tone, pacing));
        } else {
            logger.warn(&format!("Chapter mission not found: {}", mission_id));
        }
    }

    // 如果有project_id且没有提供上下文，自动获取
    if let Some(ref project_id) = request.project_id {
        if request.character_context.is_none() {
            let mut stmt = conn
                .prepare(
                    "SELECT name, role_type, race, gender, age, personality, skills, status
                     FROM characters WHERE project_id = ?"
                )
                .map_err(|e| e.to_string())?;

            let characters: Vec<String> = stmt
                .query_map([project_id], |row| {
                    let name: String = row.get(0)?;
                    let role_type: Option<String> = row.get(1)?;
                    let race: Option<String> = row.get(2)?;
                    let gender: Option<String> = row.get(3)?;
                    let age: Option<i32> = row.get(4)?;
                    let personality: Option<String> = row.get(5)?;
                    let skills: Option<String> = row.get(6)?;
                    let status: Option<String> = row.get(7)?;

                    let mut parts = vec![format!("【{}】", name)];
                    if let Some(r) = role_type {
                        let role_label = match r.as_str() {
                            "protagonist" => "主角",
                            "deuteragonist" => "第二主角",
                            "antagonist" => "反派",
                            "supporting" => "配角",
                            "minor" => "小角色",
                            _ => &r,
                        };
                        parts.push(format!("身份: {}", role_label));
                    }
                    if let Some(r) = race { parts.push(format!("种族: {}", r)); }
                    if let Some(g) = gender { parts.push(format!("性别: {}", g)); }
                    if let Some(a) = age { parts.push(format!("年龄: {}", a)); }
                    if let Some(p) = personality { parts.push(format!("性格: {}", p)); }
                    if let Some(s) = skills { parts.push(format!("技能: {}", s)); }
                    if let Some(s) = status { parts.push(format!("状态: {}", s)); }

                    Ok(parts.join(" | "))
                })
                .map_err(|e| e.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;

            request.character_context = Some(characters.join("\n"));
        }

        if request.worldview_context.is_none() {
            let mut stmt = conn
                .prepare(
                    "SELECT category, title, content FROM world_views WHERE project_id = ? LIMIT 10"
                )
                .map_err(|e| e.to_string())?;

            let worldviews: Vec<String> = stmt
                .query_map([project_id], |row| {
                    let category: String = row.get(0)?;
                    let title: String = row.get(1)?;
                    let content: String = row.get(2)?;
                    Ok(format!("【{} - {}】\n{}", category, title, content))
                })
                .map_err(|e| e.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;

            request.worldview_context = Some(worldviews.join("\n\n"));
        }
    }

    // L3写作层：信息可见性过滤
    if !forbidden_characters.is_empty() {
        if let Some(ref char_context) = request.character_context {
            let filtered: Vec<&str> = char_context
                .lines()
                .filter(|line| {
                    !forbidden_characters.iter().any(|forbidden| {
                        line.contains(&format!("【{}】", forbidden))
                    })
                })
                .collect();
            request.character_context = Some(filtered.join("\n"));
            logger.info(&format!("Filtered {} forbidden characters from context", forbidden_characters.len()));
        }
    }

    // 设置默认值
    if request.character_context.is_none() {
        request.character_context = Some("暂无角色信息".to_string());
    }
    if request.worldview_context.is_none() {
        request.worldview_context = Some("暂无世界观设定".to_string());
    }

    // L3写作层：将导演脚本上下文注入到instruction中
    if let Some(mission) = mission_context {
        let enhanced_instruction = format!(
            "{}\n\n{}",
            request.instruction,
            mission
        );
        request.instruction = enhanced_instruction;
        logger.info("Injected chapter mission context into instruction");
    }

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;

    let result = service.continue_novel(request, None).await.map_err(|e| {
        logger.error(&format!("Failed to continue novel: {}", e));
        e
    })?;

    log_command_success(&logger, "ai_continue_novel", "Novel continuation completed");
    Ok(result)
}

#[tauri::command]
pub async fn ai_rewrite_content(
    app: AppHandle,
    request: AIRewriteRequest,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("ai-rewrite-service");
    log_command_start(&logger, "ai_rewrite_content", &format!("{:?}", request));

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.rewrite_content(request).await.map_err(|e| {
        logger.error(&format!("Failed to rewrite content: {}", e));
        e
    })?;

    log_command_success(&logger, "ai_rewrite_content", "Content rewrite completed");
    Ok(result)
}

#[tauri::command]
pub async fn get_prompt_templates(
    app: AppHandle,
) -> Result<Vec<PromptTemplate>, String> {
    let logger = Logger::new().with_feature("ai-prompt-service");
    log_command_start(&logger, "get_prompt_templates", "");

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let templates = service.get_prompt_manager().list_templates(None).await;

    log_command_success(&logger, "get_prompt_templates", &format!("Retrieved {} templates", templates.len()));
    Ok(templates)
}

#[tauri::command]
pub async fn save_debug_log(
    entry: DebugLogEntry,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("debug-logger");
    log_command_start(&logger, "save_debug_log", &format!("{:?}", entry));

    let log_line = format!(
        "[{}] [{}] [{}] [{}] {} | {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
        entry.level,
        entry.source,
        entry.feature.unwrap_or_else(|| "N/A".to_string()),
        entry.message,
        serde_json::to_string(&entry.data).unwrap_or_else(|_| "N/A".to_string())
    );

    println!("{}", log_line);
    Ok(())
}

#[tauri::command]
pub async fn save_debug_log_file(
    content: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("debug-logger");
    log_command_start(&logger, "save_debug_log_file", "Saving debug logs to file");

    let log_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?;

    let log_path = log_dir.join("debug_logs.log");
    std::fs::write(&log_path, content)
        .map_err(|e| format!("Failed to write debug log file: {}", e))?;

    log_command_success(&logger, "save_debug_log_file", &format!("Debug logs saved to {:?}", log_path));
    Ok(log_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn set_bigmodel_api_key(
    app: AppHandle,
    api_key: String,
) -> Result<(), String> {
    let logger = Logger::new().with_feature("ai-settings");
    log_command_start(&logger, "set_bigmodel_api_key", "Updating BigModel API key");

    std::env::set_var("BIGMODEL_API_KEY", &api_key);

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    service.get_registry().initialize_default_bigmodel_models().await;

    log_command_success(&logger, "set_bigmodel_api_key", "BigModel API key updated successfully");
    Ok(())
}

#[tauri::command]
pub async fn get_bigmodel_api_key() -> Result<String, String> {
    let logger = Logger::new().with_feature("ai-settings");
    log_command_start(&logger, "get_bigmodel_api_key", "Getting BigModel API key");

    let api_key = std::env::var("BIGMODEL_API_KEY")
        .unwrap_or_else(|_| String::new());

    let masked_key = if api_key.len() > 8 {
        format!("{}****{}", &api_key[..4], &api_key[api_key.len()-4..])
    } else {
        "****".to_string()
    };

    log_command_success(&logger, "get_bigmodel_api_key", &format!("API key retrieved: {}", masked_key));
    Ok(api_key)
}

#[tauri::command]
pub async fn get_all_debug_logs() -> Result<String, String> {
    let logger = Logger::new().with_feature("debug-logs");
    log_command_start(&logger, "get_all_debug_logs", "Retrieving all debug logs");

    let log_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?;

    let log_path = log_dir.join("debug_logs.log");

    if !log_path.exists() {
        log_command_success(&logger, "get_all_debug_logs", "No debug logs found");
        return Ok("No debug logs found".to_string());
    }

    let content = std::fs::read_to_string(&log_path)
        .map_err(|e| format!("Failed to read debug log file: {}", e))?;

    let line_count = content.lines().count();
    log_command_success(&logger, "get_all_debug_logs", &format!("Retrieved {} log entries", line_count));
    Ok(content)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UILogEntry {
    component: String,
    action: String,
    timestamp: i64,
    data: Option<serde_json::Value>,
}

#[tauri::command]
pub async fn save_ui_logs(logs: Vec<UILogEntry>) -> Result<(), String> {
    let logger = crate::logger::Logger::new().with_feature("ui-logs");
    
    logger.info(&format!("Received {} UI log entries from frontend", logs.len()));
    
    for entry in &logs {
        let data_str = entry.data.as_ref()
            .map(|d| serde_json::to_string(&d).unwrap_or_default())
            .unwrap_or_else(|| "null".to_string());
        
        let log_msg = format!(
            "[UI] {} {} @ {} - {}",
            entry.component,
            entry.action,
            entry.timestamp,
            data_str
        );
        
        println!("{}", log_msg);
        logger.debug(&log_msg);
    }
    
    logger.info(&format!("Successfully processed {} UI log entries", logs.len()));
    
    Ok(())
}

// ==================== AI 生成命令 ====================

/// AI生成角色
#[tauri::command]
pub async fn ai_generate_character(
    app: AppHandle,
    request: AIGenerateCharacterRequest,
) -> Result<GeneratedCharacter, String> {
    let logger = Logger::new().with_feature("ai-generator");
    log_command_start(&logger, "ai_generate_character", &format!("projectId: {}", request.project_id));

    // 获取项目信息、世界观设定和已有角色
    let (genre, worldviews, existing_characters) = {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

        // 获取项目题材
        let genre: String = conn
            .query_row(
                "SELECT COALESCE(genre, '小说') FROM projects WHERE id = ?",
                [&request.project_id],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "小说".to_string());

        // 获取世界观设定（取最重要的几条）
        let mut stmt = conn
            .prepare("SELECT category, title, content FROM world_views WHERE project_id = ? ORDER BY created_at DESC LIMIT 5")
            .map_err(|e| e.to_string())?;
        
        let worldviews: Vec<(String, String, String)> = stmt
            .query_map(&[&request.project_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 获取已有角色
        let mut stmt = conn
            .prepare("SELECT name, gender, age, personality FROM characters WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        
        let existing_characters: Vec<(String, Option<String>, Option<i32>, Option<String>)> = stmt
            .query_map(&[&request.project_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        (genre, worldviews, existing_characters)
    };

    let mut request = request;
    if request.genre.is_none() {
        request.genre = Some(genre);
    }

    // 构建上下文
    let worldviews_context = if worldviews.is_empty() {
        "暂无世界观设定".to_string()
    } else {
        worldviews
            .iter()
            .map(|(cat, title, content)| format!("[{}] {}: {}", cat, title, content.chars().take(100).collect::<String>()))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let existing_chars_context = if existing_characters.is_empty() {
        "暂无已有角色".to_string()
    } else {
        existing_characters
            .iter()
            .map(|(name, gender, age, personality)| {
                format!("- {} ({}, {}岁): {}", 
                    name, 
                    gender.as_deref().unwrap_or("未知"), 
                    age.unwrap_or(0),
                    personality.as_deref().unwrap_or("无描述"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.generate_character_with_context(
        request, 
        &worldviews_context,
        &existing_chars_context
    ).await.map_err(|e| {
        log_command_error(&logger, "ai_generate_character", &e);
        e
    })?;

    log_command_success(&logger, "ai_generate_character", &format!("Generated character: {}", result.name));
    Ok(result)
}

/// AI生成角色关系
#[tauri::command]
pub async fn ai_generate_character_relations(
    app: AppHandle,
    request: AIGenerateCharacterRelationsRequest,
) -> Result<Vec<GeneratedCharacterRelation>, String> {
    let logger = Logger::new().with_feature("ai-generator");
    log_command_start(&logger, "ai_generate_character_relations", &format!("projectId: {}", request.project_id));

    // 使用块来限制数据库连接的生命周期
    let (characters, project_context) = {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

        // 获取项目中的所有角色
        let mut stmt = conn
            .prepare("SELECT id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at FROM characters WHERE project_id = ?")
            .map_err(|e| {
                logger.error(&format!("Failed to prepare statement: {}", e));
                e.to_string()
            })?;

        let characters_iter = stmt
            .query_map(&[&request.project_id], |row| {
                Ok(Character {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    role_type: row.get(3)?,
                    race: row.get(4)?,
                    age: row.get(5)?,
                    gender: row.get(6)?,
                    birth_date: row.get(7)?,
                    appearance: row.get(8)?,
                    personality: row.get(9)?,
                    background: row.get(10)?,
                    skills: row.get(11)?,
                    status: row.get(12)?,
                    bazi: row.get(13)?,
                    ziwei: row.get(14)?,
                    mbti: row.get(15)?,
                    enneagram: row.get(16)?,
                    items: row.get(17)?,
                    avatar_url: row.get(18)?,
                    created_at: row.get(19)?,
                    updated_at: row.get(20)?,
                })
            })
            .map_err(|e| {
                logger.error(&format!("Failed to query characters: {}", e));
                e.to_string()
            })?;

        let mut characters = Vec::new();
        for character in characters_iter {
            characters.push(character.map_err(|e| {
                logger.error(&format!("Failed to map character: {}", e));
                e.to_string()
            })?);
        }

        // 获取项目描述作为上下文
        let project_context: String = conn
            .query_row(
                "SELECT COALESCE(description, name) FROM projects WHERE id = ?",
                [&request.project_id],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "未知故事背景".to_string());

        (characters, project_context)
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.generate_character_relations(request, &characters, &project_context).await.map_err(|e| {
        log_command_error(&logger, "ai_generate_character_relations", &e);
        e
    })?;

    log_command_success(&logger, "ai_generate_character_relations", &format!("Generated {} relations", result.len()));
    Ok(result)
}

/// AI生成世界观
#[tauri::command]
pub async fn ai_generate_worldview(
    app: AppHandle,
    request: AIGenerateWorldViewRequest,
) -> Result<GeneratedWorldView, String> {
    let logger = Logger::new().with_feature("ai-generator");
    log_command_start(&logger, "ai_generate_worldview", &format!("projectId: {}, category: {}", request.project_id, request.category));

    // 使用块来限制数据库连接的生命周期
    let (genre, existing_worldviews, characters, plot_points) = {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

        // 获取项目题材
        let genre: String = conn
            .query_row(
                "SELECT COALESCE(genre, '小说') FROM projects WHERE id = ?",
                [&request.project_id],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "小说".to_string());

        // 获取已有世界观设定
        let mut stmt = conn
            .prepare("SELECT id, project_id, category, title, content, tags, status, created_at, updated_at FROM world_views WHERE project_id = ?")
            .map_err(|e| {
                logger.error(&format!("Failed to prepare statement: {}", e));
                e.to_string()
            })?;

        let worldviews_iter = stmt
            .query_map(&[&request.project_id], |row| {
                Ok(WorldView {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    category: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    tags: row.get(5)?,
                    status: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| {
                logger.error(&format!("Failed to query world views: {}", e));
                e.to_string()
            })?;

        let mut existing_worldviews = Vec::new();
        for worldview in worldviews_iter {
            existing_worldviews.push(worldview.map_err(|e| {
                logger.error(&format!("Failed to map world view: {}", e));
                e.to_string()
            })?);
        }

        // 获取角色信息
        let mut stmt = conn
            .prepare("SELECT name, gender, age, personality, background FROM characters WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        
        let characters: Vec<(String, Option<String>, Option<i32>, Option<String>, Option<String>)> = stmt
            .query_map(&[&request.project_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 获取情节点
        let mut stmt = conn
            .prepare("SELECT title, description FROM plot_points WHERE project_id = ? ORDER BY sort_order ASC LIMIT 10")
            .map_err(|e| e.to_string())?;
        
        let plot_points: Vec<(String, Option<String>)> = stmt
            .query_map(&[&request.project_id], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        (genre, existing_worldviews, characters, plot_points)
    };

    // 构建角色上下文
    let characters_context = if characters.is_empty() {
        "暂无角色".to_string()
    } else {
        characters
            .iter()
            .map(|(name, gender, age, personality, background)| {
                format!("- {} ({}, {}岁): {} | {}", 
                    name, 
                    gender.as_deref().unwrap_or("未知"), 
                    age.unwrap_or(0),
                    personality.as_deref().unwrap_or("无性格"),
                    background.as_deref().map(|b| b.chars().take(50).collect::<String>()).unwrap_or_default())
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    // 构建情节上下文
    let plot_context = if plot_points.is_empty() {
        "暂无情节".to_string()
    } else {
        plot_points
            .iter()
            .map(|(title, desc)| format!("- {}: {}", title, desc.as_deref().unwrap_or("无描述")))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.generate_worldview_with_context(
        request, 
        &genre, 
        &existing_worldviews,
        &characters_context,
        &plot_context
    ).await.map_err(|e| {
        log_command_error(&logger, "ai_generate_worldview", &e);
        e
    })?;

    log_command_success(&logger, "ai_generate_worldview", &format!("Generated worldview: {}", result.title));
    Ok(result)
}

/// AI生成情节点
#[tauri::command]
pub async fn ai_generate_plot_points(
    app: AppHandle,
    request: AIGeneratePlotPointsRequest,
) -> Result<Vec<GeneratedPlotPoint>, String> {
    let logger = Logger::new().with_feature("ai-generator");
    log_command_start(&logger, "ai_generate_plot_points", &format!("projectId: {}", request.project_id));

    // 使用块来限制数据库连接的生命周期
    let (project_info, existing_plots, characters, worldviews) = {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

        // 获取项目信息
        let project_info: String = conn
            .query_row(
                "SELECT COALESCE(description, name) || ' (' || COALESCE(genre, '小说') || ')' FROM projects WHERE id = ?",
                [&request.project_id],
                |row| row.get(0),
            )
            .unwrap_or_else(|_| "未知项目".to_string());

        // 获取已有情节点
        let mut stmt = conn
            .prepare("SELECT id, project_id, parent_id, title, description, note, chapter_id, status, sort_order, level, created_at, updated_at FROM plot_points WHERE project_id = ? ORDER BY sort_order ASC")
            .map_err(|e| {
                logger.error(&format!("Failed to prepare statement: {}", e));
                e.to_string()
            })?;

        let plot_points_iter = stmt
            .query_map(&[&request.project_id], |row| {
                Ok(PlotPoint {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    title: row.get(3)?,
                    description: row.get(4)?,
                    note: row.get(5)?,
                    chapter_id: row.get(6)?,
                    status: row.get(7)?,
                    sort_order: row.get(8)?,
                    level: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            })
            .map_err(|e| {
                logger.error(&format!("Failed to query plot points: {}", e));
                e.to_string()
            })?;

        let mut existing_plots = Vec::new();
        for plot_point in plot_points_iter {
            existing_plots.push(plot_point.map_err(|e| {
                logger.error(&format!("Failed to map plot point: {}", e));
                e.to_string()
            })?);
        }

        // 获取角色信息
        let mut stmt = conn
            .prepare("SELECT name, gender, age, personality FROM characters WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        
        let characters: Vec<(String, Option<String>, Option<i32>, Option<String>)> = stmt
            .query_map(&[&request.project_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 获取世界观设定
        let mut stmt = conn
            .prepare("SELECT category, title, content FROM world_views WHERE project_id = ? ORDER BY created_at DESC LIMIT 5")
            .map_err(|e| e.to_string())?;
        
        let worldviews: Vec<(String, String, String)> = stmt
            .query_map(&[&request.project_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        (project_info, existing_plots, characters, worldviews)
    };

    // 构建角色上下文
    let characters_context = if characters.is_empty() {
        "暂无角色".to_string()
    } else {
        characters
            .iter()
            .map(|(name, gender, age, personality)| {
                format!("- {} ({}, {}岁): {}", 
                    name, 
                    gender.as_deref().unwrap_or("未知"), 
                    age.unwrap_or(0),
                    personality.as_deref().unwrap_or("无描述"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    // 构建世界观上下文
    let worldviews_context = if worldviews.is_empty() {
        "暂无世界观设定".to_string()
    } else {
        worldviews
            .iter()
            .map(|(cat, title, content)| format!("[{}] {}: {}", cat, title, content.chars().take(80).collect::<String>()))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.generate_plot_points_with_context(
        request, 
        &project_info, 
        &existing_plots,
        &characters_context,
        &worldviews_context
    ).await.map_err(|e| {
        log_command_error(&logger, "ai_generate_plot_points", &e);
        e
    })?;

    log_command_success(&logger, "ai_generate_plot_points", &format!("Generated {} plot points", result.len()));
    Ok(result)
}

/// AI生成分镜提示词
#[tauri::command]
pub async fn ai_generate_storyboard(
    app: AppHandle,
    request: AIGenerateStoryboardRequest,
) -> Result<Vec<GeneratedStoryboard>, String> {
    let logger = Logger::new().with_feature("ai-generator");
    log_command_start(&logger, "ai_generate_storyboard", &format!("chapterId: {:?}, plotPointId: {:?}", request.chapter_id, request.plot_point_id));

    // 获取内容 - 使用块来限制数据库连接的生命周期
    let content = if let Some(ref content) = request.content {
        content.clone()
    } else {
        // 需要从数据库获取内容
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            e.to_string()
        })?;

        if let Some(ref chapter_id) = request.chapter_id {
            conn.query_row(
                "SELECT content FROM chapters WHERE id = ?",
                [chapter_id],
                |row| row.get(0),
            ).map_err(|e| {
                logger.error(&format!("Failed to get chapter content: {}", e));
                e.to_string()
            })?
        } else if let Some(ref plot_point_id) = request.plot_point_id {
            conn.query_row(
                "SELECT COALESCE(description, title) FROM plot_points WHERE id = ?",
                [plot_point_id],
                |row| row.get(0),
            ).map_err(|e| {
                logger.error(&format!("Failed to get plot point content: {}", e));
                e.to_string()
            })?
        } else {
            return Err("Either content, chapterId, or plotPointId must be provided".to_string());
        }
    };

    if content.trim().is_empty() {
        return Err("Content is empty".to_string());
    }

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.generate_storyboard(request, &content).await.map_err(|e| {
        log_command_error(&logger, "ai_generate_storyboard", &e);
        e
    })?;

    log_command_success(&logger, "ai_generate_storyboard", &format!("Generated {} storyboard shots", result.len()));
    Ok(result)
}

/// AI一键排版
#[tauri::command]
pub async fn ai_format_content(
    app: AppHandle,
    request: AIFormatContentRequest,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("ai-generator");
    log_command_start(&logger, "ai_format_content", &format!("content length: {} chars", request.content.len()));

    if request.content.trim().is_empty() {
        return Err("Content is empty".to_string());
    }

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.format_content(request).await.map_err(|e| {
        log_command_error(&logger, "ai_format_content", &e);
        e
    })?;

    log_command_success(&logger, "ai_format_content", "Content formatted successfully");
    Ok(result)
}

// ==================== 系统设置命令 ====================

/// 获取默认模型
#[tauri::command]
pub async fn get_default_model(app: AppHandle) -> Result<Option<String>, String> {
    let logger = Logger::new().with_feature("settings");
    log_command_start(&logger, "get_default_model", "");

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        e.to_string()
    })?;

    let result: Option<String> = conn
        .query_row(
            "SELECT value FROM app_settings WHERE key = 'default_model'",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| {
            logger.error(&format!("Failed to get default model: {}", e));
            e.to_string()
        })?;

    log_command_success(&logger, "get_default_model", &format!("Default model: {:?}", result));
    Ok(result)
}

/// 设置默认模型
#[tauri::command]
pub async fn set_default_model(app: AppHandle, modelId: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("settings");
    log_command_start(&logger, "set_default_model", &format!("modelId: {}", modelId));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        e.to_string()
    })?;

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR REPLACE INTO app_settings (key, value, updated_at) VALUES ('default_model', ?, ?)",
        params![modelId, now],
    ).map_err(|e| {
        logger.error(&format!("Failed to set default model: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "set_default_model", &format!("Default model set to: {}", modelId));
    Ok(())
}

/// 获取 AI 参数
#[tauri::command]
pub async fn get_ai_params(app: AppHandle) -> Result<AIParams, String> {
    let logger = Logger::new().with_feature("settings");
    log_command_start(&logger, "get_ai_params", "");

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        e.to_string()
    })?;

    let params_json: Option<String> = conn
        .query_row(
            "SELECT value FROM app_settings WHERE key = 'ai_params'",
            [],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| {
            logger.error(&format!("Failed to get AI params: {}", e));
            e.to_string()
        })?;

    let params = if let Some(json) = params_json {
        serde_json::from_str(&json).unwrap_or_default()
    } else {
        AIParams::default()
    };

    log_command_success(&logger, "get_ai_params", &format!("AI params: {:?}", params));
    Ok(params)
}

/// 设置 AI 参数
#[tauri::command]
pub async fn set_ai_params(app: AppHandle, params: AIParams) -> Result<(), String> {
    let logger = Logger::new().with_feature("settings");
    log_command_start(&logger, "set_ai_params", &format!("{:?}", params));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        e.to_string()
    })?;

    let now = Utc::now().to_rfc3339();
    let params_json = serde_json::to_string(&params).map_err(|e| {
        logger.error(&format!("Failed to serialize AI params: {}", e));
        e.to_string()
    })?;

    conn.execute(
        "INSERT OR REPLACE INTO app_settings (key, value, updated_at) VALUES ('ai_params', ?, ?)",
        params![params_json, now],
    ).map_err(|e| {
        logger.error(&format!("Failed to set AI params: {}", e));
        e.to_string()
    })?;

    log_command_success(&logger, "set_ai_params", "AI params saved successfully");
    Ok(())
}

/// 获取 API 密钥列表（不返回实际密钥）
#[tauri::command]
pub async fn get_api_keys(app: AppHandle) -> Result<Vec<APIKeyInfo>, String> {
    let logger = Logger::new().with_feature("settings");
    log_command_start(&logger, "get_api_keys", "");

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        e.to_string()
    })?;

    // 定义支持的提供商
    let providers = vec![
        ("bigmodel", "智谱 GLM"),
        ("openai", "OpenAI"),
        ("anthropic", "Anthropic"),
        ("ollama", "Ollama"),
    ];

    let mut result = Vec::new();

    for (provider_id, provider_name) in providers {
        let key_info: Option<(String, i32)> = conn
            .query_row(
                "SELECT api_key, is_configured FROM api_keys WHERE provider = ?",
                [&provider_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|e| {
                logger.error(&format!("Failed to get API key for {}: {}", provider_id, e));
                e.to_string()
            })?;

        if let Some((api_key, _)) = key_info {
            let masked_key = if api_key.len() > 8 {
                format!("{}****{}", &api_key[..4], &api_key[api_key.len()-4..])
            } else {
                "****".to_string()
            };
            result.push(APIKeyInfo {
                provider: provider_id.to_string(),
                provider_name: provider_name.to_string(),
                is_configured: true,
                masked_key: Some(masked_key),
            });
        } else {
            result.push(APIKeyInfo {
                provider: provider_id.to_string(),
                provider_name: provider_name.to_string(),
                is_configured: false,
                masked_key: None,
            });
        }
    }

    log_command_success(&logger, "get_api_keys", &format!("Retrieved {} API key infos", result.len()));
    Ok(result)
}

/// 设置 API 密钥
#[tauri::command]
pub async fn set_api_key(app: AppHandle, provider: String, apiKey: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("settings");
    log_command_start(&logger, "set_api_key", &format!("provider: {}", provider));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        e.to_string()
    })?;

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR REPLACE INTO api_keys (provider, api_key, is_configured, updated_at) VALUES (?, ?, 1, ?)",
        params![provider, apiKey, now],
    ).map_err(|e| {
        logger.error(&format!("Failed to set API key: {}", e));
        e.to_string()
    })?;

    // 如果是 bigmodel，同时更新环境变量和重新初始化模型
    if provider == "bigmodel" {
        std::env::set_var("BIGMODEL_API_KEY", &apiKey);
        
        let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
        let service = ai_service.read().await;
        service.get_registry().initialize_default_bigmodel_models().await;
    }

    log_command_success(&logger, "set_api_key", &format!("API key set for: {}", provider));
    Ok(())
}

/// 获取带默认标记的模型列表
#[tauri::command]
pub async fn get_models_with_default(app: AppHandle) -> Result<Vec<ModelInfo>, String> {
    let logger = Logger::new().with_feature("settings");
    log_command_start(&logger, "get_models_with_default", "");

    // 获取默认模型
    let db_path = get_db_path(&app)?;
    let default_model: Option<String> = get_connection(&db_path)
        .ok()
        .and_then(|conn| {
            conn.query_row(
                "SELECT value FROM app_settings WHERE key = 'default_model'",
                [],
                |row| row.get(0),
            )
            .optional()
            .ok()
            .flatten()
        });

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let model_ids = service.get_registry().list_models().await;
    
    let models: Vec<ModelInfo> = model_ids
        .into_iter()
        .map(|id| {
            let is_default = default_model.as_ref() == Some(&id);
            let provider = if id.starts_with("glm") {
                "智谱 GLM"
            } else if id.starts_with("gpt") {
                "OpenAI"
            } else {
                "Other"
            };
            ModelInfo {
                id: id.clone(),
                name: id,
                provider: provider.to_string(),
                is_default,
            }
        })
        .collect();

    log_command_success(&logger, "get_models_with_default", &format!("Retrieved {} models", models.len()));
    Ok(models)
}

/// 生成续写选项
#[tauri::command]
pub async fn generate_writing_choices(
    app: AppHandle,
    request: GenerateWritingChoicesRequest,
) -> Result<WritingSuggestion, String> {
    let logger = Logger::new().with_feature("ai-writing");
    log_command_start(&logger, "generate_writing_choices", &format!("chapter: {}", request.chapter_id));

    // 获取项目上下文
    let (characters, worldviews, plot_points) = {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

        // 获取角色
        let mut stmt = conn
            .prepare("SELECT id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at FROM characters WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        let characters: Vec<Character> = stmt
            .query_map([&request.project_id], |row| {
                Ok(Character {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    role_type: row.get(3)?,
                    race: row.get(4)?,
                    age: row.get(5)?,
                    gender: row.get(6)?,
                    birth_date: row.get(7)?,
                    appearance: row.get(8)?,
                    personality: row.get(9)?,
                    background: row.get(10)?,
                    skills: row.get(11)?,
                    status: row.get(12)?,
                    bazi: row.get(13)?,
                    ziwei: row.get(14)?,
                    mbti: row.get(15)?,
                    enneagram: row.get(16)?,
                    items: row.get(17)?,
                    avatar_url: row.get(18)?,
                    created_at: row.get(19)?,
                    updated_at: row.get(20)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 获取世界观
        let mut stmt = conn
            .prepare("SELECT id, project_id, category, title, content, tags, status, created_at, updated_at FROM world_views WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        let worldviews: Vec<WorldView> = stmt
            .query_map([&request.project_id], |row| {
                Ok(WorldView {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    category: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    tags: row.get(5)?,
                    status: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 获取情节点
        let mut stmt = conn
            .prepare("SELECT id, project_id, parent_id, title, description, note, chapter_id, status, sort_order, level, created_at, updated_at FROM plot_points WHERE project_id = ? ORDER BY sort_order")
            .map_err(|e| e.to_string())?;
        let plot_points: Vec<PlotPoint> = stmt
            .query_map([&request.project_id], |row| {
                Ok(PlotPoint {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    title: row.get(3)?,
                    description: row.get(4)?,
                    note: row.get(5)?,
                    chapter_id: row.get(6)?,
                    status: row.get(7)?,
                    sort_order: row.get(8)?,
                    level: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        (characters, worldviews, plot_points)
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.generate_writing_choices(request, &characters, &worldviews, &plot_points).await.map_err(|e| {
        log_command_error(&logger, "generate_writing_choices", &e);
        e
    })?;

    log_command_success(&logger, "generate_writing_choices", &format!("Generated {} choices", result.choices.len()));
    Ok(result)
}

/// 验证写作内容
#[tauri::command]
pub async fn validate_writing(
    app: AppHandle,
    request: ValidateWritingRequest,
) -> Result<ValidationResult, String> {
    let logger = Logger::new().with_feature("ai-writing");
    log_command_start(&logger, "validate_writing", &format!("project: {}", request.project_id));

    // 获取项目上下文
    let (characters, worldviews, relations) = {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

        // 获取角色
        let mut stmt = conn
            .prepare("SELECT id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at FROM characters WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        let characters: Vec<Character> = stmt
            .query_map([&request.project_id], |row| {
                Ok(Character {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    name: row.get(2)?,
                    role_type: row.get(3)?,
                    race: row.get(4)?,
                    age: row.get(5)?,
                    gender: row.get(6)?,
                    birth_date: row.get(7)?,
                    appearance: row.get(8)?,
                    personality: row.get(9)?,
                    background: row.get(10)?,
                    skills: row.get(11)?,
                    status: row.get(12)?,
                    bazi: row.get(13)?,
                    ziwei: row.get(14)?,
                    mbti: row.get(15)?,
                    enneagram: row.get(16)?,
                    items: row.get(17)?,
                    avatar_url: row.get(18)?,
                    created_at: row.get(19)?,
                    updated_at: row.get(20)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 获取世界观
        let mut stmt = conn
            .prepare("SELECT id, project_id, category, title, content, tags, status, created_at, updated_at FROM world_views WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        let worldviews: Vec<WorldView> = stmt
            .query_map([&request.project_id], |row| {
                Ok(WorldView {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    category: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    tags: row.get(5)?,
                    status: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        // 获取角色关系
        let mut stmt = conn
            .prepare("SELECT id, project_id, from_character_id, to_character_id, relation_type, description, created_at, updated_at FROM character_relations WHERE project_id = ?")
            .map_err(|e| e.to_string())?;
        let relations: Vec<CharacterRelation> = stmt
            .query_map([&request.project_id], |row| {
                Ok(CharacterRelation {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    from_character_id: row.get(2)?,
                    to_character_id: row.get(3)?,
                    relation_type: row.get(4)?,
                    description: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();

        (characters, worldviews, relations)
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;
    
    let result = service.validate_writing(request, &characters, &worldviews, &relations).await.map_err(|e| {
        log_command_error(&logger, "validate_writing", &e);
        e
    })?;

    log_command_success(&logger, "validate_writing", &format!("Found {} warnings", result.consistency_warnings.len()));
    Ok(result)
}

/// 创建剧情节点
#[tauri::command]
pub async fn create_plot_node(app: AppHandle, request: CreatePlotNodeRequest) -> Result<PlotNode, String> {
    let logger = Logger::new().with_feature("plot-nodes");
    log_command_start(&logger, "create_plot_node", &request.title);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let characters_json = serde_json::to_string(&request.characters_involved).unwrap_or_else(|_| "[]".to_string());
    let word_count = request.content.chars().count() as i32;

    // 获取排序号
    let sort_order: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM plot_nodes WHERE project_id = ? AND (parent_node_id = ? OR (parent_node_id IS NULL AND ? IS NULL))",
            params![&request.project_id, &request.parent_node_id, &request.parent_node_id],
            |row| row.get(0),
        )
        .unwrap_or(0);

    conn.execute(
        "INSERT INTO plot_nodes (id, project_id, chapter_id, parent_node_id, title, summary, content, choice_made, characters_involved, location, emotional_tone, word_count, is_main_path, branch_name, sort_order, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            &id,
            &request.project_id,
            &request.chapter_id,
            &request.parent_node_id,
            &request.title,
            &request.summary,
            &request.content,
            &request.choice_made,
            &characters_json,
            &request.location,
            &request.emotional_tone,
            word_count,
            request.is_main_path as i32,
            &request.branch_name,
            sort_order,
            now.clone(),
            now,
        ],
    ).map_err(|e| e.to_string())?;

    let node = PlotNode {
        id,
        project_id: request.project_id,
        chapter_id: request.chapter_id,
        parent_node_id: request.parent_node_id,
        title: request.title,
        summary: request.summary,
        content: request.content,
        choice_made: request.choice_made,
        characters_involved: request.characters_involved,
        location: request.location,
        emotional_tone: request.emotional_tone,
        word_count,
        is_main_path: request.is_main_path,
        branch_name: request.branch_name,
        sort_order,
        created_at: now.clone(),
        updated_at: now,
    };

    log_command_success(&logger, "create_plot_node", &format!("Created node: {}", node.title));
    Ok(node)
}

/// 获取剧情树
#[tauri::command]
pub async fn get_plot_tree(app: AppHandle, project_id: String) -> Result<PlotTree, String> {
    let logger = Logger::new().with_feature("plot-nodes");
    log_command_start(&logger, "get_plot_tree", &project_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, project_id, chapter_id, parent_node_id, title, summary, content, choice_made, characters_involved, location, emotional_tone, word_count, is_main_path, branch_name, sort_order, created_at, updated_at FROM plot_nodes WHERE project_id = ? ORDER BY sort_order")
        .map_err(|e| e.to_string())?;

    let nodes: Vec<PlotNode> = stmt
        .query_map([&project_id], |row| {
            let characters_json: String = row.get(8)?;
            let characters: Vec<String> = serde_json::from_str(&characters_json).unwrap_or_default();
            Ok(PlotNode {
                id: row.get(0)?,
                project_id: row.get(1)?,
                chapter_id: row.get(2)?,
                parent_node_id: row.get(3)?,
                title: row.get(4)?,
                summary: row.get(5)?,
                content: row.get(6)?,
                choice_made: row.get(7)?,
                characters_involved: characters,
                location: row.get(9)?,
                emotional_tone: row.get(10)?,
                word_count: row.get(11)?,
                is_main_path: row.get::<_, i32>(12)? == 1,
                branch_name: row.get(13)?,
                sort_order: row.get(14)?,
                created_at: row.get(15)?,
                updated_at: row.get(16)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // 找出根节点（没有父节点的节点）
    let root_nodes: Vec<String> = nodes
        .iter()
        .filter(|n| n.parent_node_id.is_none())
        .map(|n| n.id.clone())
        .collect();

    log_command_success(&logger, "get_plot_tree", &format!("Retrieved {} nodes, {} roots", nodes.len(), root_nodes.len()));
    Ok(PlotTree { nodes, root_nodes })
}

/// 删除剧情节点
#[tauri::command]
pub async fn delete_plot_node(app: AppHandle, node_id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("plot-nodes");
    log_command_start(&logger, "delete_plot_node", &node_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM plot_nodes WHERE id = ?", [&node_id])
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "delete_plot_node", "Node deleted");
    Ok(())
}

// ============== 角色时间线事件命令 ==============

/// 创建角色时间线事件
#[tauri::command]
pub async fn create_character_timeline_event(
    app: AppHandle,
    request: CreateCharacterTimelineEventRequest,
) -> Result<CharacterTimelineEvent, String> {
    let logger = Logger::new().with_feature("character-timeline");
    log_command_start(&logger, "create_character_timeline_event", &request.character_id);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let sort_order = request.sort_order.unwrap_or(0);

    conn.execute(
        "INSERT INTO character_timeline_events 
        (id, character_id, event_type, event_title, event_description, story_time, 
         real_chapter_id, emotional_state, state_changes, sort_order, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            id,
            request.character_id,
            request.event_type,
            request.event_title,
            request.event_description,
            request.story_time,
            request.real_chapter_id,
            request.emotional_state,
            request.state_changes,
            sort_order,
            now,
            now,
        ],
    ).map_err(|e| e.to_string())?;

    let event = CharacterTimelineEvent {
        id,
        character_id: request.character_id,
        event_type: request.event_type,
        event_title: request.event_title,
        event_description: request.event_description,
        story_time: request.story_time,
        real_chapter_id: request.real_chapter_id,
        emotional_state: request.emotional_state,
        state_changes: request.state_changes,
        sort_order,
        created_at: now.clone(),
        updated_at: now,
    };

    log_command_success(&logger, "create_character_timeline_event", &event.id);
    Ok(event)
}

/// 获取角色的所有时间线事件
#[tauri::command]
pub async fn get_character_timeline(app: AppHandle, character_id: String) -> Result<Vec<CharacterTimelineEvent>, String> {
    let logger = Logger::new().with_feature("character-timeline");
    log_command_start(&logger, "get_character_timeline", &character_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, character_id, event_type, event_title, event_description, 
                    story_time, real_chapter_id, emotional_state, state_changes, 
                    sort_order, created_at, updated_at
             FROM character_timeline_events 
             WHERE character_id = ? 
             ORDER BY sort_order ASC, created_at ASC"
        )
        .map_err(|e| e.to_string())?;

    let events = stmt
        .query_map([&character_id], |row| {
            Ok(CharacterTimelineEvent {
                id: row.get(0)?,
                character_id: row.get(1)?,
                event_type: row.get(2)?,
                event_title: row.get(3)?,
                event_description: row.get(4)?,
                story_time: row.get(5)?,
                real_chapter_id: row.get(6)?,
                emotional_state: row.get(7)?,
                state_changes: row.get(8)?,
                sort_order: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "get_character_timeline", &format!("Retrieved {} events", events.len()));
    Ok(events)
}

/// 更新角色时间线事件
#[tauri::command]
pub async fn update_character_timeline_event(
    app: AppHandle,
    event_id: String,
    request: UpdateCharacterTimelineEventRequest,
) -> Result<CharacterTimelineEvent, String> {
    let logger = Logger::new().with_feature("character-timeline");
    log_command_start(&logger, "update_character_timeline_event", &event_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE character_timeline_events SET 
         event_type = COALESCE(?, event_type),
         event_title = COALESCE(?, event_title),
         event_description = COALESCE(?, event_description),
         story_time = COALESCE(?, story_time),
         real_chapter_id = COALESCE(?, real_chapter_id),
         emotional_state = COALESCE(?, emotional_state),
         state_changes = COALESCE(?, state_changes),
         sort_order = COALESCE(?, sort_order),
         updated_at = ?
         WHERE id = ?",
        params![
            request.event_type,
            request.event_title,
            request.event_description,
            request.story_time,
            request.real_chapter_id,
            request.emotional_state,
            request.state_changes,
            request.sort_order,
            now,
            event_id,
        ],
    ).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, character_id, event_type, event_title, event_description, 
                    story_time, real_chapter_id, emotional_state, state_changes, 
                    sort_order, created_at, updated_at
             FROM character_timeline_events WHERE id = ?"
        )
        .map_err(|e| e.to_string())?;

    let event = stmt
        .query_row([&event_id], |row| {
            Ok(CharacterTimelineEvent {
                id: row.get(0)?,
                character_id: row.get(1)?,
                event_type: row.get(2)?,
                event_title: row.get(3)?,
                event_description: row.get(4)?,
                story_time: row.get(5)?,
                real_chapter_id: row.get(6)?,
                emotional_state: row.get(7)?,
                state_changes: row.get(8)?,
                sort_order: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "update_character_timeline_event", &event_id);
    Ok(event)
}

/// 删除角色时间线事件
#[tauri::command]
pub async fn delete_character_timeline_event(app: AppHandle, event_id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("character-timeline");
    log_command_start(&logger, "delete_character_timeline_event", &event_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM character_timeline_events WHERE id = ?", [&event_id])
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "delete_character_timeline_event", &event_id);
    Ok(())
}

// ============== 世界观时间线事件命令 ==============

/// 创建世界观时间线事件
#[tauri::command]
pub async fn create_worldview_timeline_event(
    app: AppHandle,
    request: CreateWorldViewTimelineEventRequest,
) -> Result<WorldViewTimelineEvent, String> {
    let logger = Logger::new().with_feature("worldview-timeline");
    log_command_start(&logger, "create_worldview_timeline_event", &request.worldview_id);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let sort_order = request.sort_order.unwrap_or(0);

    conn.execute(
        "INSERT INTO worldview_timeline_events 
        (id, worldview_id, event_type, event_title, event_description, story_time, 
         impact_scope, related_characters, sort_order, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            id,
            request.worldview_id,
            request.event_type,
            request.event_title,
            request.event_description,
            request.story_time,
            request.impact_scope,
            request.related_characters,
            sort_order,
            now,
            now,
        ],
    ).map_err(|e| e.to_string())?;

    let event = WorldViewTimelineEvent {
        id,
        worldview_id: request.worldview_id,
        event_type: request.event_type,
        event_title: request.event_title,
        event_description: request.event_description,
        story_time: request.story_time,
        impact_scope: request.impact_scope,
        related_characters: request.related_characters,
        sort_order,
        created_at: now.clone(),
        updated_at: now,
    };

    log_command_success(&logger, "create_worldview_timeline_event", &event.id);
    Ok(event)
}

/// 获取世界观的所有时间线事件
#[tauri::command]
pub async fn get_worldview_timeline(app: AppHandle, worldview_id: String) -> Result<Vec<WorldViewTimelineEvent>, String> {
    let logger = Logger::new().with_feature("worldview-timeline");
    log_command_start(&logger, "get_worldview_timeline", &worldview_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, worldview_id, event_type, event_title, event_description, 
                    story_time, impact_scope, related_characters, sort_order, created_at, updated_at
             FROM worldview_timeline_events 
             WHERE worldview_id = ? 
             ORDER BY sort_order ASC, created_at ASC"
        )
        .map_err(|e| e.to_string())?;

    let events = stmt
        .query_map([&worldview_id], |row| {
            Ok(WorldViewTimelineEvent {
                id: row.get(0)?,
                worldview_id: row.get(1)?,
                event_type: row.get(2)?,
                event_title: row.get(3)?,
                event_description: row.get(4)?,
                story_time: row.get(5)?,
                impact_scope: row.get(6)?,
                related_characters: row.get(7)?,
                sort_order: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "get_worldview_timeline", &format!("Retrieved {} events", events.len()));
    Ok(events)
}

/// 更新世界观时间线事件
#[tauri::command]
pub async fn update_worldview_timeline_event(
    app: AppHandle,
    event_id: String,
    request: UpdateWorldViewTimelineEventRequest,
) -> Result<WorldViewTimelineEvent, String> {
    let logger = Logger::new().with_feature("worldview-timeline");
    log_command_start(&logger, "update_worldview_timeline_event", &event_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE worldview_timeline_events SET 
         event_type = COALESCE(?, event_type),
         event_title = COALESCE(?, event_title),
         event_description = COALESCE(?, event_description),
         story_time = COALESCE(?, story_time),
         impact_scope = COALESCE(?, impact_scope),
         related_characters = COALESCE(?, related_characters),
         sort_order = COALESCE(?, sort_order),
         updated_at = ?
         WHERE id = ?",
        params![
            request.event_type,
            request.event_title,
            request.event_description,
            request.story_time,
            request.impact_scope,
            request.related_characters,
            request.sort_order,
            now,
            event_id,
        ],
    ).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, worldview_id, event_type, event_title, event_description, 
                    story_time, impact_scope, related_characters, sort_order, created_at, updated_at
             FROM worldview_timeline_events WHERE id = ?"
        )
        .map_err(|e| e.to_string())?;

    let event = stmt
        .query_row([&event_id], |row| {
            Ok(WorldViewTimelineEvent {
                id: row.get(0)?,
                worldview_id: row.get(1)?,
                event_type: row.get(2)?,
                event_title: row.get(3)?,
                event_description: row.get(4)?,
                story_time: row.get(5)?,
                impact_scope: row.get(6)?,
                related_characters: row.get(7)?,
                sort_order: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "update_worldview_timeline_event", &event_id);
    Ok(event)
}

/// 删除世界观时间线事件
#[tauri::command]
pub async fn delete_worldview_timeline_event(app: AppHandle, event_id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("worldview-timeline");
    log_command_start(&logger, "delete_worldview_timeline_event", &event_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM worldview_timeline_events WHERE id = ?", [&event_id])
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "delete_worldview_timeline_event", &event_id);
    Ok(())
}

// ============== 知识库命令 ==============

/// 创建知识条目
#[tauri::command]
pub async fn create_knowledge_entry(
    app: AppHandle,
    request: CreateKnowledgeEntryRequest,
) -> Result<KnowledgeEntry, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "create_knowledge_entry", &request.project_id);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let source_type = request.source_type.unwrap_or_else(|| "manual".to_string());
    let importance = request.importance.unwrap_or(0);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO knowledge_entries 
        (id, project_id, entry_type, title, content, source_type, source_id, keywords, importance, is_verified, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
        params![
            id,
            request.project_id,
            request.entry_type,
            request.title,
            request.content,
            source_type,
            request.source_id,
            request.keywords,
            importance,
            now,
            now,
        ],
    ).map_err(|e| e.to_string())?;

    let entry = KnowledgeEntry {
        id,
        project_id: request.project_id,
        entry_type: request.entry_type,
        title: request.title,
        content: request.content,
        source_type,
        source_id: request.source_id,
        keywords: request.keywords,
        importance,
        is_verified: false,
        created_at: now.clone(),
        updated_at: now,
    };

    log_command_success(&logger, "create_knowledge_entry", &entry.id);
    Ok(entry)
}

/// 获取项目的所有知识条目
#[tauri::command]
pub async fn get_knowledge_entries(app: AppHandle, project_id: String) -> Result<Vec<KnowledgeEntry>, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "get_knowledge_entries", &project_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, project_id, entry_type, title, content, source_type, source_id, 
                    keywords, importance, is_verified, created_at, updated_at
             FROM knowledge_entries 
             WHERE project_id = ? 
             ORDER BY importance DESC, updated_at DESC"
        )
        .map_err(|e| e.to_string())?;

    let entries = stmt
        .query_map([&project_id], |row| {
            Ok(KnowledgeEntry {
                id: row.get(0)?,
                project_id: row.get(1)?,
                entry_type: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                source_type: row.get(5)?,
                source_id: row.get(6)?,
                keywords: row.get(7)?,
                importance: row.get(8)?,
                is_verified: row.get::<_, i32>(9)? != 0,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "get_knowledge_entries", &format!("Retrieved {} entries", entries.len()));
    Ok(entries)
}

/// 按类型获取知识条目
#[tauri::command]
pub async fn get_knowledge_entries_by_type(
    app: AppHandle, 
    project_id: String, 
    entry_type: String
) -> Result<Vec<KnowledgeEntry>, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "get_knowledge_entries_by_type", &format!("{}/{}", project_id, entry_type));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, project_id, entry_type, title, content, source_type, source_id, 
                    keywords, importance, is_verified, created_at, updated_at
             FROM knowledge_entries 
             WHERE project_id = ? AND entry_type = ?
             ORDER BY importance DESC, updated_at DESC"
        )
        .map_err(|e| e.to_string())?;

    let entries = stmt
        .query_map(params![&project_id, &entry_type], |row| {
            Ok(KnowledgeEntry {
                id: row.get(0)?,
                project_id: row.get(1)?,
                entry_type: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                source_type: row.get(5)?,
                source_id: row.get(6)?,
                keywords: row.get(7)?,
                importance: row.get(8)?,
                is_verified: row.get::<_, i32>(9)? != 0,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "get_knowledge_entries_by_type", &format!("Retrieved {} entries", entries.len()));
    Ok(entries)
}

/// 更新知识条目
#[tauri::command]
pub async fn update_knowledge_entry(
    app: AppHandle,
    request: UpdateKnowledgeEntryRequest,
) -> Result<KnowledgeEntry, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "update_knowledge_entry", &request.id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();
    let is_verified = request.is_verified.map(|v| if v { 1 } else { 0 });

    conn.execute(
        "UPDATE knowledge_entries SET 
         entry_type = COALESCE(?, entry_type),
         title = COALESCE(?, title),
         content = COALESCE(?, content),
         keywords = COALESCE(?, keywords),
         importance = COALESCE(?, importance),
         is_verified = COALESCE(?, is_verified),
         updated_at = ?
         WHERE id = ?",
        params![
            request.entry_type,
            request.title,
            request.content,
            request.keywords,
            request.importance,
            is_verified,
            now,
            request.id,
        ],
    ).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, project_id, entry_type, title, content, source_type, source_id, 
                    keywords, importance, is_verified, created_at, updated_at
             FROM knowledge_entries WHERE id = ?"
        )
        .map_err(|e| e.to_string())?;

    let entry = stmt
        .query_row([&request.id], |row| {
            Ok(KnowledgeEntry {
                id: row.get(0)?,
                project_id: row.get(1)?,
                entry_type: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                source_type: row.get(5)?,
                source_id: row.get(6)?,
                keywords: row.get(7)?,
                importance: row.get(8)?,
                is_verified: row.get::<_, i32>(9)? != 0,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "update_knowledge_entry", &request.id);
    Ok(entry)
}

/// 删除知识条目
#[tauri::command]
pub async fn delete_knowledge_entry(app: AppHandle, entry_id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "delete_knowledge_entry", &entry_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM knowledge_entries WHERE id = ?", [&entry_id])
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "delete_knowledge_entry", &entry_id);
    Ok(())
}

/// 搜索知识条目
#[tauri::command]
pub async fn search_knowledge(
    app: AppHandle,
    request: SearchKnowledgeRequest,
) -> Result<Vec<KnowledgeSearchResult>, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "search_knowledge", &request.query);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let limit = request.limit.unwrap_or(20);
    let search_pattern = format!("%{}%", request.query);

    let sql = if let Some(ref types) = request.entry_types {
        let placeholders: Vec<String> = types.iter().map(|_| "?".to_string()).collect();
        format!(
            "SELECT id, project_id, entry_type, title, content, source_type, source_id, 
                    keywords, importance, is_verified, created_at, updated_at
             FROM knowledge_entries 
             WHERE project_id = ? AND entry_type IN ({}) AND (title LIKE ? OR content LIKE ? OR keywords LIKE ?)
             ORDER BY importance DESC
             LIMIT ?",
            placeholders.join(",")
        )
    } else {
        "SELECT id, project_id, entry_type, title, content, source_type, source_id, 
                keywords, importance, is_verified, created_at, updated_at
         FROM knowledge_entries 
         WHERE project_id = ? AND (title LIKE ? OR content LIKE ? OR keywords LIKE ?)
         ORDER BY importance DESC
         LIMIT ?".to_string()
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let results = if let Some(ref types) = request.entry_types {
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![
            Box::new(request.project_id.clone()),
            Box::new(search_pattern.clone()),
            Box::new(search_pattern.clone()),
            Box::new(search_pattern.clone()),
        ];
        for t in types {
            params_vec.push(Box::new(t.clone()));
        }
        params_vec.push(Box::new(limit));

        let params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
        
        stmt.query_map(params.as_slice(), |row| {
            Ok(KnowledgeSearchResult {
                entry: KnowledgeEntry {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    entry_type: row.get(2)?,
                    title: row.get(3)?,
                    content: row.get(4)?,
                    source_type: row.get(5)?,
                    source_id: row.get(6)?,
                    keywords: row.get(7)?,
                    importance: row.get(8)?,
                    is_verified: row.get::<_, i32>(9)? != 0,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                },
                relevance_score: 1.0,
                match_type: "keyword".to_string(),
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?
    } else {
        stmt.query_map(
            params![&request.project_id, &search_pattern, &search_pattern, &search_pattern, limit],
            |row| {
                Ok(KnowledgeSearchResult {
                    entry: KnowledgeEntry {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        entry_type: row.get(2)?,
                        title: row.get(3)?,
                        content: row.get(4)?,
                        source_type: row.get(5)?,
                        source_id: row.get(6)?,
                        keywords: row.get(7)?,
                        importance: row.get(8)?,
                        is_verified: row.get::<_, i32>(9)? != 0,
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                    },
                    relevance_score: 1.0,
                    match_type: "keyword".to_string(),
                })
            },
        )
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?
    };

    log_command_success(&logger, "search_knowledge", &format!("Found {} results", results.len()));
    Ok(results)
}

/// 创建知识关系
#[tauri::command]
pub async fn create_knowledge_relation(
    app: AppHandle,
    request: CreateKnowledgeRelationRequest,
) -> Result<KnowledgeRelation, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "create_knowledge_relation", &request.project_id);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let strength = request.strength.unwrap_or(1);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO knowledge_relations 
        (id, project_id, from_entry_id, to_entry_id, relation_type, description, strength, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            id,
            request.project_id,
            request.from_entry_id,
            request.to_entry_id,
            request.relation_type,
            request.description,
            strength,
            now,
        ],
    ).map_err(|e| e.to_string())?;

    let relation = KnowledgeRelation {
        id,
        project_id: request.project_id,
        from_entry_id: request.from_entry_id,
        to_entry_id: request.to_entry_id,
        relation_type: request.relation_type,
        description: request.description,
        strength,
        created_at: now,
    };

    log_command_success(&logger, "create_knowledge_relation", &relation.id);
    Ok(relation)
}

/// 获取知识条目的所有关系
#[tauri::command]
pub async fn get_knowledge_relations(app: AppHandle, entry_id: String) -> Result<Vec<KnowledgeRelation>, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "get_knowledge_relations", &entry_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, project_id, from_entry_id, to_entry_id, relation_type, description, strength, created_at
             FROM knowledge_relations 
             WHERE from_entry_id = ? OR to_entry_id = ?
             ORDER BY strength DESC"
        )
        .map_err(|e| e.to_string())?;

    let relations = stmt
        .query_map(params![&entry_id, &entry_id], |row| {
            Ok(KnowledgeRelation {
                id: row.get(0)?,
                project_id: row.get(1)?,
                from_entry_id: row.get(2)?,
                to_entry_id: row.get(3)?,
                relation_type: row.get(4)?,
                description: row.get(5)?,
                strength: row.get(6)?,
                created_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "get_knowledge_relations", &format!("Retrieved {} relations", relations.len()));
    Ok(relations)
}

/// 删除知识关系
#[tauri::command]
pub async fn delete_knowledge_relation(app: AppHandle, relation_id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "delete_knowledge_relation", &relation_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM knowledge_relations WHERE id = ?", [&relation_id])
        .map_err(|e| e.to_string())?;

    log_command_success(&logger, "delete_knowledge_relation", &relation_id);
    Ok(())
}

/// 构建知识上下文（用于AI写作）
#[tauri::command]
pub async fn build_knowledge_context(
    app: AppHandle,
    request: BuildKnowledgeContextRequest,
) -> Result<KnowledgeContext, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "build_knowledge_context", &request.project_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let include_characters = request.include_characters.unwrap_or(true);
    let include_worldview = request.include_worldview.unwrap_or(true);
    let include_plot = request.include_plot.unwrap_or(true);
    let include_timeline = request.include_timeline.unwrap_or(true);

    // 构建角色摘要
    let characters_summary = if include_characters {
        let mut stmt = conn
            .prepare(
                "SELECT name, role_type, race, gender, age, personality, skills, status
                 FROM characters WHERE project_id = ?"
            )
            .map_err(|e| e.to_string())?;

        let characters: Vec<String> = stmt
            .query_map([&request.project_id], |row| {
                let name: String = row.get(0)?;
                let role_type: Option<String> = row.get(1)?;
                let race: Option<String> = row.get(2)?;
                let gender: Option<String> = row.get(3)?;
                let age: Option<i32> = row.get(4)?;
                let personality: Option<String> = row.get(5)?;
                let skills: Option<String> = row.get(6)?;
                let status: Option<String> = row.get(7)?;

                let mut parts = vec![name];
                if let Some(r) = role_type { parts.push(format!("[{}]", r)); }
                if let Some(r) = race { parts.push(format!("种族:{}", r)); }
                if let Some(g) = gender { parts.push(format!("性别:{}", g)); }
                if let Some(a) = age { parts.push(format!("年龄:{}", a)); }
                if let Some(p) = personality { parts.push(format!("性格:{}", p)); }
                if let Some(s) = skills { parts.push(format!("技能:{}", s)); }
                if let Some(s) = status { parts.push(format!("状态:{}", s)); }

                Ok(parts.join(" | "))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        characters.join("\n")
    } else {
        String::new()
    };

    // 构建世界观摘要
    let worldview_summary = if include_worldview {
        let mut stmt = conn
            .prepare(
                "SELECT category, title, content
                 FROM world_views WHERE project_id = ?"
            )
            .map_err(|e| e.to_string())?;

        let worldviews: Vec<String> = stmt
            .query_map([&request.project_id], |row| {
                let category: String = row.get(0)?;
                let title: String = row.get(1)?;
                let content: String = row.get(2)?;
                Ok(format!("[{}] {} - {}", category, title, content))
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        worldviews.join("\n")
    } else {
        String::new()
    };

    // 构建剧情摘要
    let plot_summary = if include_plot {
        if let Some(chapter_id) = &request.chapter_id {
            let mut stmt = conn
                .prepare(
                    "SELECT title, summary FROM plot_nodes 
                     WHERE chapter_id = ? OR project_id = (SELECT project_id FROM chapters WHERE id = ?)
                     ORDER BY sort_order"
                )
                .map_err(|e| e.to_string())?;

            let plots: Vec<String> = stmt
                .query_map(params![chapter_id, chapter_id], |row| {
                    let title: String = row.get(0)?;
                    let summary: Option<String> = row.get(1)?;
                    Ok(format!("{} - {}", title, summary.unwrap_or_default()))
                })
                .map_err(|e| e.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.to_string())?;

            plots.join("\n")
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // 获取关键事件
    let key_events = if include_timeline {
        let mut stmt = conn
            .prepare(
                "SELECT event_title FROM character_timeline_events 
                 WHERE character_id IN (SELECT id FROM characters WHERE project_id = ?)
                 ORDER BY sort_order LIMIT 10"
            )
            .map_err(|e| e.to_string())?;

        let events: Vec<String> = stmt
            .query_map([&request.project_id], |row| row.get(0))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;
        events
    } else {
        vec![]
    };

    // 获取活跃角色
    let active_characters: Vec<String> = conn
        .query_row(
            "SELECT GROUP_CONCAT(name, ',') FROM characters WHERE project_id = ? AND role_type IN ('protagonist', 'deuteragonist')",
            [&request.project_id],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "".to_string())
        .split(',')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let context = KnowledgeContext {
        project_id: request.project_id,
        characters_summary,
        worldview_summary,
        plot_summary,
        key_events,
        active_characters,
        current_location: None,
        timeline_context: String::new(),
    };

    log_command_success(&logger, "build_knowledge_context", "Context built");
    Ok(context)
}

/// 从角色自动生成知识条目
#[tauri::command]
pub async fn sync_character_to_knowledge(
    app: AppHandle,
    character_id: String,
) -> Result<KnowledgeEntry, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "sync_character_to_knowledge", &character_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    // 获取角色信息
    let character = conn
        .query_row(
            "SELECT id, project_id, name, role_type, race, gender, age, personality, background, skills, status
             FROM characters WHERE id = ?",
            [&character_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                    row.get::<_, Option<String>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                    row.get::<_, Option<i32>>(6)?,
                    row.get::<_, Option<String>>(7)?,
                    row.get::<_, Option<String>>(8)?,
                    row.get::<_, Option<String>>(9)?,
                    row.get::<_, Option<String>>(10)?,
                ))
            },
        )
        .map_err(|e| e.to_string())?;

    let (_id, project_id, name, role_type, race, gender, age, personality, background, skills, status) = character;

    // 构建知识内容
    let mut content_parts = vec![];
    if let Some(ref r) = role_type { content_parts.push(format!("身份: {}", r)); }
    if let Some(ref r) = race { content_parts.push(format!("种族: {}", r)); }
    if let Some(ref g) = gender { content_parts.push(format!("性别: {}", g)); }
    if let Some(a) = age { content_parts.push(format!("年龄: {}", a)); }
    if let Some(ref p) = personality { content_parts.push(format!("性格: {}", p)); }
    if let Some(ref b) = background { content_parts.push(format!("背景: {}", b)); }
    if let Some(ref s) = skills { content_parts.push(format!("技能: {}", s)); }
    if let Some(ref s) = status { content_parts.push(format!("状态: {}", s)); }

    let content = content_parts.join("\n");
    let keywords = format!("{},{},{}", name, role_type.unwrap_or_default(), race.unwrap_or_default());

    // 检查是否已存在
    let existing_id: Option<String> = conn
        .query_row(
            "SELECT id FROM knowledge_entries WHERE source_type = 'character' AND source_id = ?",
            [&character_id],
            |row| row.get(0),
        )
        .ok();

    let now = Utc::now().to_rfc3339();

    if let Some(existing) = existing_id {
        // 更新现有条目
        conn.execute(
            "UPDATE knowledge_entries SET title = ?, content = ?, keywords = ?, updated_at = ? WHERE id = ?",
            params![&name, &content, &keywords, &now, &existing],
        )
        .map_err(|e| e.to_string())?;

        let entry = conn
            .query_row(
                "SELECT id, project_id, entry_type, title, content, source_type, source_id, keywords, importance, is_verified, created_at, updated_at FROM knowledge_entries WHERE id = ?",
                [&existing],
                |row| {
                    Ok(KnowledgeEntry {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        entry_type: row.get(2)?,
                        title: row.get(3)?,
                        content: row.get(4)?,
                        source_type: row.get(5)?,
                        source_id: row.get(6)?,
                        keywords: row.get(7)?,
                        importance: row.get(8)?,
                        is_verified: row.get::<_, i32>(9)? != 0,
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                    })
                },
            )
            .map_err(|e| e.to_string())?;

        log_command_success(&logger, "sync_character_to_knowledge", &entry.id);
        Ok(entry)
    } else {
        // 创建新条目
        let new_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO knowledge_entries (id, project_id, entry_type, title, content, source_type, source_id, keywords, importance, is_verified, created_at, updated_at) VALUES (?, ?, 'character', ?, ?, 'character', ?, ?, 5, 1, ?, ?)",
            params![&new_id, &project_id, &name, &content, &character_id, &keywords, &now, &now],
        )
        .map_err(|e| e.to_string())?;

        let entry = KnowledgeEntry {
            id: new_id,
            project_id,
            entry_type: "character".to_string(),
            title: name,
            content,
            source_type: "character".to_string(),
            source_id: Some(character_id),
            keywords: Some(keywords),
            importance: 5,
            is_verified: true,
            created_at: now.clone(),
            updated_at: now,
        };

        log_command_success(&logger, "sync_character_to_knowledge", &entry.id);
        Ok(entry)
    }
}

/// 从世界观自动生成知识条目
#[tauri::command]
pub async fn sync_worldview_to_knowledge(
    app: AppHandle,
    worldview_id: String,
) -> Result<KnowledgeEntry, String> {
    let logger = Logger::new().with_feature("knowledge");
    log_command_start(&logger, "sync_worldview_to_knowledge", &worldview_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    // 获取世界观信息
    let worldview = conn
        .query_row(
            "SELECT id, project_id, category, title, content, tags
             FROM world_views WHERE id = ?",
            [&worldview_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            },
        )
        .map_err(|e| e.to_string())?;

    let (_id, project_id, category, title, content, tags) = worldview;
    let keywords = tags.unwrap_or_else(|| category.clone());

    // 检查是否已存在
    let existing_id: Option<String> = conn
        .query_row(
            "SELECT id FROM knowledge_entries WHERE source_type = 'worldview' AND source_id = ?",
            [&worldview_id],
            |row| row.get(0),
        )
        .ok();

    let now = Utc::now().to_rfc3339();

    if let Some(existing) = existing_id {
        conn.execute(
            "UPDATE knowledge_entries SET title = ?, content = ?, keywords = ?, updated_at = ? WHERE id = ?",
            params![&title, &content, &keywords, &now, &existing],
        )
        .map_err(|e| e.to_string())?;

        let entry = conn
            .query_row(
                "SELECT id, project_id, entry_type, title, content, source_type, source_id, keywords, importance, is_verified, created_at, updated_at FROM knowledge_entries WHERE id = ?",
                [&existing],
                |row| {
                    Ok(KnowledgeEntry {
                        id: row.get(0)?,
                        project_id: row.get(1)?,
                        entry_type: row.get(2)?,
                        title: row.get(3)?,
                        content: row.get(4)?,
                        source_type: row.get(5)?,
                        source_id: row.get(6)?,
                        keywords: row.get(7)?,
                        importance: row.get(8)?,
                        is_verified: row.get::<_, i32>(9)? != 0,
                        created_at: row.get(10)?,
                        updated_at: row.get(11)?,
                    })
                },
            )
            .map_err(|e| e.to_string())?;

        log_command_success(&logger, "sync_worldview_to_knowledge", &entry.id);
        Ok(entry)
    } else {
        let new_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO knowledge_entries (id, project_id, entry_type, title, content, source_type, source_id, keywords, importance, is_verified, created_at, updated_at) VALUES (?, ?, 'worldview', ?, ?, 'worldview', ?, ?, 3, 1, ?, ?)",
            params![&new_id, &project_id, &title, &content, &worldview_id, &keywords, &now, &now],
        )
        .map_err(|e| e.to_string())?;

        let entry = KnowledgeEntry {
            id: new_id,
            project_id,
            entry_type: "worldview".to_string(),
            title,
            content,
            source_type: "worldview".to_string(),
            source_id: Some(worldview_id),
            keywords: Some(keywords),
            importance: 3,
            is_verified: true,
            created_at: now.clone(),
            updated_at: now,
        };

        log_command_success(&logger, "sync_worldview_to_knowledge", &entry.id);
        Ok(entry)
    }
}

// ============== 多媒体生成命令 ==============

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryboardRequest {
    pub chapter_id: Option<String>,
    pub content: Option<String>,
    pub options: Option<StoryboardOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryboardOptions {
    pub format: Option<String>,
    pub style: Option<String>,
    pub detail_level: Option<String>,
    pub include_dialogue: Option<bool>,
    pub include_camera_movement: Option<bool>,
    pub include_sound_effects: Option<bool>,
    pub include_visual_prompts: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryboardResult {
    pub id: String,
    pub title: String,
    pub format: String,
    pub style: String,
    pub scenes: Vec<StoryboardScene>,
    pub total_duration: i32,
    pub metadata: StoryboardMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryboardScene {
    pub scene_number: i32,
    pub title: String,
    pub location: String,
    pub time_of_day: String,
    pub shots: Vec<Shot>,
    pub estimated_duration: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shot {
    pub shot_number: i32,
    pub shot_type: String,
    pub description: String,
    pub camera: Option<CameraMovement>,
    pub characters: Vec<String>,
    pub action: Option<String>,
    pub dialogue: Option<Dialogue>,
    pub sound_effects: Option<Vec<String>>,
    pub duration: i32,
    pub visual_prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CameraMovement {
    pub movement_type: String,
    pub direction: Option<String>,
    pub speed: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dialogue {
    pub character: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoryboardMetadata {
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptRequest {
    pub chapter_id: Option<String>,
    pub content: Option<String>,
    pub options: ScriptOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptOptions {
    pub target_format: Option<String>,
    pub include_scene_numbers: Option<bool>,
    pub include_character_descriptions: Option<bool>,
    pub dialogue_style: Option<String>,
    pub include_camera_directions: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptResult {
    pub id: String,
    pub title: String,
    pub format: String,
    pub scenes: Vec<ScriptScene>,
    pub characters: Vec<ScriptCharacter>,
    pub metadata: ScriptMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptScene {
    pub scene_number: i32,
    pub heading: String,
    pub action: String,
    pub characters: Vec<ScriptCharacter>,
    pub dialogue: Vec<ScriptDialogue>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptCharacter {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptDialogue {
    pub character: String,
    pub parenthetical: Option<String>,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptMetadata {
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicRequest {
    pub chapter_id: Option<String>,
    pub content: Option<String>,
    pub options: ComicOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicOptions {
    pub style: Option<String>,
    pub page_layout: Option<String>,
    pub panels_per_page: Option<i32>,
    pub include_captions: Option<bool>,
    pub include_sound_effects: Option<bool>,
    pub generate_images: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicResult {
    pub id: String,
    pub title: String,
    pub style: String,
    pub pages: Vec<ComicPage>,
    pub characters: Vec<ComicCharacter>,
    pub metadata: ComicMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicPage {
    pub page_number: i32,
    pub layout: String,
    pub panels: Vec<ComicPanel>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicPanel {
    pub panel_number: i32,
    pub shape: String,
    pub description: String,
    pub caption: Option<String>,
    pub dialogue: Vec<ComicDialogue>,
    pub sound_effects: Option<Vec<String>>,
    pub visual_prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicDialogue {
    pub character: String,
    pub text: String,
    pub balloon_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicCharacter {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComicMetadata {
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IllustrationRequest {
    pub scene_id: Option<String>,
    pub content: Option<String>,
    pub character_ids: Option<Vec<String>>,
    pub options: IllustrationOptions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IllustrationOptions {
    pub style: Option<String>,
    pub aspect_ratio: Option<String>,
    pub quality: Option<String>,
    pub custom_prompt: Option<String>,
    pub negative_prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IllustrationResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub style: String,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub aspect_ratio: String,
    pub image_data: Option<String>,
    pub metadata: IllustrationMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IllustrationMetadata {
    pub generated_at: String,
}

/// 生成分镜脚本
#[tauri::command]
pub async fn multimedia_generate_storyboard(
    app: AppHandle,
    request: StoryboardRequest,
) -> Result<StoryboardResult, String> {
    let logger = Logger::new().with_feature("multimedia");
    log_command_start(&logger, "multimedia_generate_storyboard", &format!("chapter: {:?}", request.chapter_id));

    let content = if let Some(chapter_id) = &request.chapter_id {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
        let content: String = conn
            .query_row("SELECT content FROM chapters WHERE id = ?", [chapter_id], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        content
    } else if let Some(content) = &request.content {
        content.clone()
    } else {
        return Err("请提供章节ID或内容".to_string());
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;

    let style = request.options.as_ref()
        .and_then(|o| o.style.clone())
        .unwrap_or_else(|| "cinematic".to_string());

    let prompt = format!(
        "请将以下小说内容转换为专业的分镜脚本格式。\
        \n\n小说内容：\n{}\
        \n\n请按以下JSON格式输出分镜脚本（不要包含任何其他说明文字）：\
        {{\
          \"title\": \"分镜标题\",\
          \"scenes\": [\
            {{\
              \"scene_number\": 1,\
              \"title\": \"场景标题\",\
              \"location\": \"地点\",\
              \"time_of_day\": \"morning/afternoon/evening/night\",\
              \"shots\": [\
                {{\
                  \"shot_number\": 1,\
                  \"shot_type\": \"close_up/medium_shot/long_shot\",\
                  \"description\": \"镜头描述\",\
                  \"camera\": {{\"movement_type\": \"static/pan/tilt/dolly\", \"direction\": \"left/right\"}},\
                  \"characters\": [\"角色名\"],\
                  \"action\": \"动作描述\",\
                  \"dialogue\": {{\"character\": \"角色\", \"text\": \"台词\"}},\
                  \"duration\": 5,\
                  \"visual_prompt\": \"用于AI生成图像的英文提示词\"\
                }}\
              ],\
              \"estimated_duration\": 30,\
              \"notes\": \"备注\"\
            }}\
          ],\
          \"total_duration\": 120\
        }}",
        content.chars().take(3000).collect::<String>()
    );

    let model_id = "glm-4-flash".to_string();
    let response = service.complete(&model_id, "你是一位专业的分镜师，请根据用户的要求生成JSON格式的分镜脚本。只返回JSON，不要包含任何其他文字。", &prompt).await.map_err(|e| e.to_string())?;

    let json_start = response.find('{').unwrap_or(0);
    let json_end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
    let json_str = &response[json_start..json_end];

    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!({}));

    let scenes = parsed.get("scenes")
        .and_then(|s| serde_json::from_value(s.clone()).ok())
        .unwrap_or_default();

    let total_duration = parsed.get("total_duration")
        .and_then(|d| d.as_i64())
        .unwrap_or(0) as i32;

    let title = parsed.get("title")
        .and_then(|t| t.as_str())
        .unwrap_or("分镜脚本")
        .to_string();

    let result = StoryboardResult {
        id: Uuid::new_v4().to_string(),
        title,
        format: "film".to_string(),
        style,
        scenes,
        total_duration,
        metadata: StoryboardMetadata {
            generated_at: Utc::now().to_rfc3339(),
        },
    };

    log_command_success(&logger, "multimedia_generate_storyboard", &result.id);
    Ok(result)
}

/// 生成剧本
#[tauri::command]
pub async fn multimedia_generate_script(
    app: AppHandle,
    request: ScriptRequest,
) -> Result<ScriptResult, String> {
    let logger = Logger::new().with_feature("multimedia");
    log_command_start(&logger, "multimedia_generate_script", &format!("chapter: {:?}", request.chapter_id));

    let content = if let Some(chapter_id) = &request.chapter_id {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
        let content: String = conn
            .query_row("SELECT content FROM chapters WHERE id = ?", [chapter_id], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        content
    } else if let Some(content) = &request.content {
        content.clone()
    } else {
        return Err("请提供章节ID或内容".to_string());
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;

    let target_format = request.options.target_format.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("standard");

    let prompt = format!(
        "请将以下小说内容转换为{}格式的剧本。\
        \n\n小说内容：\n{}\
        \n\n请按以下JSON格式输出剧本（不要包含任何其他说明文字）：\
        {{\
          \"title\": \"剧本标题\",\
          \"scenes\": [\
            {{\
              \"scene_number\": 1,\
              \"heading\": \"场景标题（如：内景 客厅 日\"），\
              \"action\": \"场景描述和动作\",\
              \"characters\": [{{\"name\": \"角色名\", \"description\": \"简短描述\"}}],\
              \"dialogue\": [\
                {{\"character\": \"角色名\", \"parenthetical\": \"情绪/动作\", \"text\": \"台词\"}}\
              ],\
              \"notes\": \"备注\"\
            }}\
          ],\
          \"characters\": [{{\"name\": \"角色名\", \"description\": \"角色描述\"}}]\
        }}",
        target_format,
        content.chars().take(3000).collect::<String>()
    );

    let model_id = "glm-4-flash".to_string();
    let response = service.complete(&model_id, "你是一位专业的编剧，请根据用户的要求将小说转换为JSON格式的剧本。只返回JSON，不要包含任何其他文字。", &prompt).await.map_err(|e| e.to_string())?;

    let json_start = response.find('{').unwrap_or(0);
    let json_end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
    let json_str = &response[json_start..json_end];

    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!({}));

    let scenes: Vec<ScriptScene> = parsed.get("scenes")
        .and_then(|s| serde_json::from_value(s.clone()).ok())
        .unwrap_or_default();

    let characters: Vec<ScriptCharacter> = parsed.get("characters")
        .and_then(|c| serde_json::from_value(c.clone()).ok())
        .unwrap_or_default();

    let title = parsed.get("title")
        .and_then(|t| t.as_str())
        .unwrap_or("剧本")
        .to_string();

    let result = ScriptResult {
        id: Uuid::new_v4().to_string(),
        title,
        format: target_format.to_string(),
        scenes,
        characters,
        metadata: ScriptMetadata {
            generated_at: Utc::now().to_rfc3339(),
        },
    };

    log_command_success(&logger, "multimedia_generate_script", &result.id);
    Ok(result)
}

/// 生成漫画分镜
#[tauri::command]
pub async fn multimedia_generate_comic(
    app: AppHandle,
    request: ComicRequest,
) -> Result<ComicResult, String> {
    let logger = Logger::new().with_feature("multimedia");
    log_command_start(&logger, "multimedia_generate_comic", &format!("chapter: {:?}", request.chapter_id));

    let content = if let Some(chapter_id) = &request.chapter_id {
        let db_path = get_db_path(&app)?;
        let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
        let content: String = conn
            .query_row("SELECT content FROM chapters WHERE id = ?", [chapter_id], |row| row.get(0))
            .map_err(|e| e.to_string())?;
        content
    } else if let Some(content) = &request.content {
        content.clone()
    } else {
        return Err("请提供章节ID或内容".to_string());
    };

    let ai_service = app.state::<std::sync::Arc<tokio::sync::RwLock<AIService>>>();
    let service = ai_service.read().await;

    let style = request.options.style.as_ref()
        .map(|s| s.clone())
        .unwrap_or_else(|| "anime".to_string());

    let panels_per_page = request.options.panels_per_page.unwrap_or(4);

    let prompt = format!(
        "请将以下小说内容转换为漫画分镜脚本格式。\
        \n\n小说内容：\n{}\
        \n\n请按以下JSON格式输出漫画分镜（不要包含任何其他说明文字）：\
        {{\
          \"title\": \"漫画标题\",\
          \"pages\": [\
            {{\
              \"page_number\": 1,\
              \"layout\": \"four_grid\",\
              \"panels\": [\
                {{\
                  \"panel_number\": 1,\
                  \"shape\": \"rectangle\",\
                  \"description\": \"画面描述\",\
                  \"caption\": \"旁白文字\",\
                  \"dialogue\": [{{\"character\": \"角色\", \"text\": \"台词\", \"balloon_type\": \"speech\"}}],\
                  \"sound_effects\": [\"音效文字\"],\
                  \"visual_prompt\": \"用于AI生成图像的英文提示词，包含画面构图、角色动作、表情等\"\
                }}\
              ],\
              \"notes\": \"页面备注\"\
            }}\
          ],\
          \"characters\": [{{\"name\": \"角色名\"}}]\
        }}\
        \n\n注意：每个页面大约{}个分格",
        content.chars().take(3000).collect::<String>(),
        panels_per_page
    );

    let model_id = "glm-4-flash".to_string();
    let response = service.complete(&model_id, "你是一位专业的漫画分镜师，请根据用户的要求将小说转换为JSON格式的漫画分镜。只返回JSON，不要包含任何其他文字。", &prompt).await.map_err(|e| e.to_string())?;

    let json_start = response.find('{').unwrap_or(0);
    let json_end = response.rfind('}').map(|i| i + 1).unwrap_or(response.len());
    let json_str = &response[json_start..json_end];

    let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap_or(serde_json::json!({}));

    let pages: Vec<ComicPage> = parsed.get("pages")
        .and_then(|p| serde_json::from_value(p.clone()).ok())
        .unwrap_or_default();

    let characters: Vec<ComicCharacter> = parsed.get("characters")
        .and_then(|c| serde_json::from_value(c.clone()).ok())
        .unwrap_or_default();

    let title = parsed.get("title")
        .and_then(|t| t.as_str())
        .unwrap_or("漫画分镜")
        .to_string();

    let result = ComicResult {
        id: Uuid::new_v4().to_string(),
        title,
        style,
        pages,
        characters,
        metadata: ComicMetadata {
            generated_at: Utc::now().to_rfc3339(),
        },
    };

    log_command_success(&logger, "multimedia_generate_comic", &result.id);
    Ok(result)
}

/// 生成插画
#[tauri::command]
pub async fn multimedia_generate_illustration(
    request: IllustrationRequest,
) -> Result<IllustrationResult, String> {
    let logger = Logger::new().with_feature("multimedia");
    log_command_start(&logger, "multimedia_generate_illustration", &format!("scene: {:?}", request.scene_id));

    let content = request.content.clone().unwrap_or_default();

    let style = request.options.style.clone().unwrap_or_else(|| "cinematic".to_string());
    let aspect_ratio = request.options.aspect_ratio.clone().unwrap_or_else(|| "16:9".to_string());
    let custom_prompt = request.options.custom_prompt.clone().unwrap_or_default();
    let negative_prompt = request.options.negative_prompt.clone();

    let prompt = if !custom_prompt.is_empty() {
        format!(
            "{}, {}, high quality, detailed",
            content,
            custom_prompt
        )
    } else {
        format!(
            "Create a {} style illustration: {}. High quality, detailed, professional artwork.",
            style,
            content
        )
    };

    let result = IllustrationResult {
        id: Uuid::new_v4().to_string(),
        title: "AI 插画".to_string(),
        description: content,
        style,
        prompt,
        negative_prompt,
        aspect_ratio,
        image_data: None,
        metadata: IllustrationMetadata {
            generated_at: Utc::now().to_rfc3339(),
        },
    };

    log_command_success(&logger, "multimedia_generate_illustration", &result.id);
    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportProjectRequest {
    pub project_id: String,
    pub format: String,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportChapterRequest {
    pub chapter_id: String,
    pub format: String,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub success: bool,
    pub output_path: String,
    pub file_size: u64,
    pub format: String,
}

pub fn format_from_str(format_str: &str) -> Result<ExportFormat, String> {
    match format_str.to_lowercase().as_str() {
        "docx" | "word" | "md" | "markdown" => Ok(ExportFormat::Docx),
        "pdf" => Ok(ExportFormat::Pdf),
        "epub" => Ok(ExportFormat::Epub),
        "txt" | "text" => Ok(ExportFormat::Txt),
        _ => Err(format!("不支持的导出格式: {}", format_str)),
    }
}

#[tauri::command]
pub async fn export_project(
    app: AppHandle,
    request: ExportProjectRequest,
) -> Result<ExportResult, String> {
    let logger = Logger::new().with_feature("export");
    log_command_start(&logger, "export_project", &format!("project: {}, format: {}", request.project_id, request.format));

    let export_format = format_from_str(&request.format)?;

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let project: (String, String, String, String) = conn
        .query_row(
            "SELECT id, title, description, author FROM projects WHERE id = ?",
            [&request.project_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|e| e.to_string())?;

    let chapters: Vec<(String, String, i32, String)> = conn
        .prepare("SELECT id, title, chapter_number, content FROM chapters WHERE project_id = ? ORDER BY chapter_number")
        .map_err(|e| e.to_string())?
        .query_map([&request.project_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let export_dir = app_data_dir.join("exports");

    if !export_dir.exists() {
        std::fs::create_dir_all(&export_dir).map_err(|e| e.to_string())?;
    }

    let filename = format!("{}_{}.{}", sanitize_filename(&project.1), Utc::now().format("%Y%m%d_%H%M%S"), export_format.extension());
    let output_path = if let Some(path) = request.output_path {
        PathBuf::from(path)
    } else {
        export_dir.join(&filename)
    };

    let metadata = ExportMetadata {
        title: project.1.clone(),
        author: project.3.clone(),
        description: Some(project.2.clone()),
        created_at: Utc::now().to_rfc3339(),
        word_count: chapters.iter().map(|c| c.3.chars().count()).sum(),
        chapter_count: chapters.len(),
    };

    let content = ExportContent {
        metadata,
        chapters: chapters.iter().map(|c| crate::export::ChapterContent {
            id: c.0.clone(),
            title: c.1.clone(),
            number: c.2 as usize,
            content: c.3.clone(),
        }).collect(),
    };

    match export_format {
        ExportFormat::Docx => {
            crate::export::export_as_docx(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Pdf => {
            crate::export::export_as_pdf(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Epub => {
            crate::export::export_as_epub(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Txt => {
            crate::export::export_as_txt(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Md => {
            crate::export::export_as_md(&content, &output_path).map_err(|e| e.to_string())?;
        }
    }

    let file_size = std::fs::metadata(&output_path).map_err(|e| e.to_string())?.len();

    let result = ExportResult {
        success: true,
        output_path: output_path.to_string_lossy().to_string(),
        file_size,
        format: export_format.extension().to_string(),
    };

    log_command_success(&logger, "export_project", &result.output_path);
    Ok(result)
}

#[tauri::command]
pub async fn export_chapter(
    app: AppHandle,
    request: ExportChapterRequest,
) -> Result<ExportResult, String> {
    let logger = Logger::new().with_feature("export");
    log_command_start(&logger, "export_chapter", &format!("chapter: {}, format: {}", request.chapter_id, request.format));

    let export_format = format_from_str(&request.format)?;

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let chapter: (String, String, String, i32, String, String) = conn
        .query_row(
            "SELECT c.id, c.title, c.content, c.chapter_number, p.title, p.author FROM chapters c JOIN projects p ON c.project_id = p.id WHERE c.id = ?",
            [&request.chapter_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
        )
        .map_err(|e| e.to_string())?;

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let export_dir = app_data_dir.join("exports");

    if !export_dir.exists() {
        std::fs::create_dir_all(&export_dir).map_err(|e| e.to_string())?;
    }

    let filename = format!("{}_{}.{}", sanitize_filename(&chapter.1), chapter.3, export_format.extension());
    let output_path = if let Some(path) = request.output_path {
        PathBuf::from(path)
    } else {
        export_dir.join(&filename)
    };

    let metadata = ExportMetadata {
        title: chapter.1.clone(),
        author: chapter.5.clone(),
        description: None,
        created_at: Utc::now().to_rfc3339(),
        word_count: chapter.2.chars().count(),
        chapter_count: 1,
    };

    let content = ExportContent {
        metadata,
        chapters: vec![crate::export::ChapterContent {
            id: chapter.0.clone(),
            title: chapter.1.clone(),
            number: chapter.3 as usize,
            content: chapter.2.clone(),
        }],
    };

    match export_format {
        ExportFormat::Docx => {
            crate::export::export_as_docx(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Pdf => {
            crate::export::export_as_pdf(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Epub => {
            crate::export::export_as_epub(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Txt => {
            crate::export::export_as_txt(&content, &output_path).map_err(|e| e.to_string())?;
        }
        ExportFormat::Md => {
            crate::export::export_as_md(&content, &output_path).map_err(|e| e.to_string())?;
        }
    }

    let file_size = std::fs::metadata(&output_path).map_err(|e| e.to_string())?.len();

    let result = ExportResult {
        success: true,
        output_path: output_path.to_string_lossy().to_string(),
        file_size,
        format: export_format.extension().to_string(),
    };

    log_command_success(&logger, "export_chapter", &result.output_path);
    Ok(result)
}

#[tauri::command]
pub async fn get_export_formats() -> Result<Vec<String>, String> {
    Ok(vec![
        "docx".to_string(),
        "pdf".to_string(),
        "epub".to_string(),
        "txt".to_string(),
    ])
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportFileRequest {
    pub file_path: String,
    pub format: String,
}

#[tauri::command]
pub async fn import_file(
    request: ImportFileRequest,
) -> Result<ImportResult, String> {
    let logger = Logger::new().with_feature("import");
    log_command_start(&logger, "import_file", &format!("path: {}, format: {}", request.file_path, request.format));

    let format = match request.format.to_lowercase().as_str() {
        "txt" => ImportFormat::Txt,
        "md" | "markdown" => ImportFormat::Md,
        "docx" => ImportFormat::Docx,
        _ => return Err(format!("不支持的导入格式: {}", request.format)),
    };

    let path = std::path::Path::new(&request.file_path);
    if !path.exists() {
        return Err(format!("文件不存在: {}", request.file_path));
    }

    let result: ImportResult = match format {
        ImportFormat::Txt => import_from_txt(path).map_err(|e: anyhow::Error| e.to_string())?,
        ImportFormat::Md => import_from_markdown(path).map_err(|e: anyhow::Error| e.to_string())?,
        ImportFormat::Docx => import_from_docx(path).map_err(|e: anyhow::Error| e.to_string())?,
    };

    log_command_success(&logger, "import_file", &format!("{} chapters, {} words", result.chapter_count, result.word_count));
    Ok(result)
}

#[tauri::command]
pub async fn import_to_project(
    app: AppHandle,
    request: ImportFileRequest,
    project_id: String,
) -> Result<ImportResult, String> {
    let logger = Logger::new().with_feature("import");
    log_command_start(&logger, "import_to_project", &format!("project: {}, path: {}", project_id, request.file_path));

    let import_result = import_file(request).await?;
    
    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    for (index, chapter) in import_result.chapters.iter().enumerate() {
        let chapter_id = Uuid::new_v4().to_string();
        let sort_order = (index + 1) as i32;
        
        conn.execute(
            "INSERT INTO chapters (id, project_id, title, content, sort_order, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
            params![
                &chapter_id,
                &project_id,
                &chapter.title,
                &chapter.content,
                sort_order,
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339()
            ],
        ).map_err(|e| format!("创建章节失败: {}", e))?;
    }

    conn.execute(
        "UPDATE projects SET updated_at = ? WHERE id = ?",
        params![Utc::now().to_rfc3339(), &project_id],
    ).map_err(|e| format!("更新项目时间失败: {}", e))?;

    log_command_success(&logger, "import_to_project", &format!("imported {} chapters", import_result.chapter_count));
    Ok(import_result)
}

#[tauri::command]
pub async fn generate_chapter_versions(
    app: AppHandle,
    request: GenerateChapterVersionsRequest,
) -> Result<Chapter, String> {
    let logger = Logger::new().with_feature("chapter-versions");
    log_command_start(&logger, "generate_chapter_versions", &request.chapter_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let chapter: Chapter = conn.query_row(
        "SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary FROM chapters WHERE id = ?1",
        params![&request.chapter_id],
        |row| Ok(Chapter {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            word_count: row.get(4)?,
            sort_order: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            versions: None,
            evaluation: None,
            generation_status: Some("generating".to_string()),
            summary: row.get(9).ok(),
        }),
    ).map_err(|e| format!("章节未找到: {}", e))?;

    let num_versions = request.num_versions.unwrap_or(3);
    let styles = vec!["标准".to_string(), "文艺".to_string(), "紧凑".to_string()];

    let mut versions = Vec::new();
    let ai_service = AIService::new();

    for i in 0..num_versions as usize {
        let style = styles.get(i).cloned().unwrap_or_else(|| "标准".to_string());
        
        let prompt = format!(
            "请以{}风格续写以下内容：\n\n{}\n\n要求：保持文风一致，情节连贯",
            style,
            request.context
        );

        let ai_request = AICompletionRequest {
            model_id: "default".to_string(),
            context: prompt.clone(),
            instruction: format!("生成{}风格的章节内容", style),
            temperature: Some(0.8),
            max_tokens: Some(2000),
            stream: Some(false),
            character_context: None,
            worldview_context: None,
            project_id: Some(request.project_id.clone()),
            chapter_mission_id: None,
        };

        match ai_service.continue_novel(ai_request, None).await {
            Ok(content) => {
                versions.push(ChapterVersion {
                    content,
                    style: style.clone(),
                    created_at: Some(Utc::now().to_rfc3339()),
                });
            }
            Err(e) => {
                logger.warn(&format!("生成{}版本失败: {}", style, e));
            }
        }
    }

    if versions.is_empty() {
        return Err("所有版本生成失败".to_string());
    }

    let versions_json = serde_json::to_string(&versions).map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE chapters SET versions = ?1, generation_status = ?2, updated_at = ?3 WHERE id = ?4",
        params![
            versions_json,
            "waiting_for_confirm",
            Utc::now().to_rfc3339(),
            &request.chapter_id
        ],
    ).map_err(|e| format!("更新章节失败: {}", e))?;

    let updated_chapter = Chapter {
        id: chapter.id,
        project_id: chapter.project_id,
        title: chapter.title,
        content: chapter.content,
        word_count: chapter.word_count,
        sort_order: chapter.sort_order,
        status: chapter.status,
        created_at: chapter.created_at,
        updated_at: Utc::now().to_rfc3339(),
        versions: Some(versions),
        evaluation: None,
        generation_status: Some("waiting_for_confirm".to_string()),
        summary: chapter.summary,
    };

    log_command_success(&logger, "generate_chapter_versions", &format!("生成{}个版本", num_versions));
    Ok(updated_chapter)
}

#[tauri::command]
pub async fn select_chapter_version(
    app: AppHandle,
    request: SelectChapterVersionRequest,
) -> Result<Chapter, String> {
    let logger = Logger::new().with_feature("chapter-versions");
    log_command_start(&logger, "select_chapter_version", &format!("chapter: {}, version: {}", request.chapter_id, request.version_index));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let versions_json: Option<String> = conn.query_row(
        "SELECT versions FROM chapters WHERE id = ?1",
        params![&request.chapter_id],
        |row| row.get(0),
    ).map_err(|e| format!("章节未找到: {}", e))?;

    let versions: Vec<ChapterVersion> = match versions_json {
        Some(json) => serde_json::from_str(&json).map_err(|e| e.to_string())?,
        None => return Err("没有可用版本".to_string()),
    };

    let selected_version = versions.get(request.version_index as usize)
        .ok_or_else(|| "版本索引无效".to_string())?;

    let word_count = selected_version.content.chars().count() as i32;
    
    conn.execute(
        "UPDATE chapters SET content = ?1, word_count = ?2, generation_status = ?3, updated_at = ?4 WHERE id = ?5",
        params![
            &selected_version.content,
            word_count,
            "successful",
            Utc::now().to_rfc3339(),
            &request.chapter_id
        ],
    ).map_err(|e| format!("更新章节失败: {}", e))?;

    let updated_chapter: Chapter = conn.query_row(
        "SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary FROM chapters WHERE id = ?1",
        params![&request.chapter_id],
        |row| Ok(Chapter {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            word_count: row.get(4)?,
            sort_order: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            summary: row.get(9).ok(),
            versions: Some(versions),
            evaluation: None,
            generation_status: Some("successful".to_string()),
        }),
    ).map_err(|e| e.to_string())?;

    log_command_success(&logger, "select_chapter_version", &format!("已选择版本{}", request.version_index));
    Ok(updated_chapter)
}

#[tauri::command]
pub async fn evaluate_chapter(
    app: AppHandle,
    request: EvaluateChapterRequest,
) -> Result<Chapter, String> {
    let logger = Logger::new().with_feature("chapter-evaluation");
    log_command_start(&logger, "evaluate_chapter", &request.chapter_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let chapter: Chapter = conn.query_row(
        "SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at, summary FROM chapters WHERE id = ?1",
        params![&request.chapter_id],
        |row| Ok(Chapter {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            word_count: row.get(4)?,
            sort_order: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            versions: None,
            evaluation: None,
            summary: row.get(9).ok(),
            generation_status: Some("evaluating".to_string()),
        }),
    ).map_err(|e| format!("章节未找到: {}", e))?;

    let ai_service = AIService::new();

    let prompt = format!(
        "请评估以下章节内容的质量，从多个维度打分并给出建议：\n\n标题：{}\n内容：\n{}\n\n请以JSON格式返回评估结果，包含：score(总分0-100), coherence(连贯性0-100), style_consistency(风格一致性0-100), character_consistency(角色一致性0-100), plot_advancement(情节推进0-100), summary(简短评价), suggestions(改进建议数组)",
        chapter.title,
        chapter.content
    );

    let ai_request = AICompletionRequest {
        model_id: "default".to_string(),
        context: prompt.clone(),
        instruction: "评估章节质量".to_string(),
        temperature: Some(0.3),
        max_tokens: Some(1000),
        stream: Some(false),
        character_context: None,
        worldview_context: None,
        project_id: Some(request.project_id.clone()),
        chapter_mission_id: None,
    };

    let evaluation_result = ai_service.continue_novel(ai_request, None).await
        .map_err(|e| format!("AI评估失败: {}", e))?;

    let evaluation: ChapterEvaluation = {
        let json_str = evaluation_result.trim_start_matches("```json").trim_end_matches("```").trim();
        serde_json::from_str(json_str).unwrap_or_else(|_| ChapterEvaluation {
            score: 75.0,
            coherence: 75.0,
            style_consistency: 75.0,
            character_consistency: 75.0,
            plot_advancement: 75.0,
            summary: "自动评估完成".to_string(),
            suggestions: vec!["建议人工复核".to_string()],
            evaluated_at: Utc::now().to_rfc3339(),
        })
    };

    let evaluation_json = serde_json::to_string(&evaluation).map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE chapters SET evaluation = ?1, generation_status = ?2, updated_at = ?3 WHERE id = ?4",
        params![
            evaluation_json,
            "evaluated",
            Utc::now().to_rfc3339(),
            &request.chapter_id
        ],
    ).map_err(|e| format!("更新章节失败: {}", e))?;

    let updated_chapter = Chapter {
        id: chapter.id,
        project_id: chapter.project_id,
        title: chapter.title,
        content: chapter.content,
        word_count: chapter.word_count,
        sort_order: chapter.sort_order,
        status: chapter.status,
        created_at: chapter.created_at,
        updated_at: Utc::now().to_rfc3339(),
        versions: None,
        evaluation: Some(evaluation),
        generation_status: Some("evaluated".to_string()),
        summary: chapter.summary,
    };

    log_command_success(&logger, "evaluate_chapter", &format!("评分: {}", updated_chapter.evaluation.as_ref().unwrap().score));
    Ok(updated_chapter)
}

#[tauri::command]
pub async fn create_foreshadowing(
    app: AppHandle,
    request: CreateForeshadowingRequest,
) -> Result<Foreshadowing, String> {
    let logger = Logger::new().with_feature("foreshadowing");
    log_command_start(&logger, "create_foreshadowing", &request.chapter_title);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let id = format!("foreshadowing_{}", Uuid::new_v4().to_string());
    let importance = request.importance.unwrap_or_else(|| "medium".to_string());
    let now = Utc::now().to_rfc3339();
    let status = "planted".to_string();

    conn.execute(
        "INSERT INTO foreshadowings (id, project_id, chapter_id, chapter_number, chapter_title, description, foreshadowing_type, keywords, status, importance, expected_payoff_chapter, author_note, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            &id,
            &request.project_id,
            &request.chapter_id,
            &request.chapter_number,
            &request.chapter_title,
            &request.description,
            &request.foreshadowing_type,
            serde_json::to_string(&request.keywords.clone().unwrap_or_default()).map_err(|e| e.to_string())?,
            "planted",
            &importance,
            &request.expected_payoff_chapter,
            &request.author_note,
            &now,
            &now,
        ],
    ).map_err(|e| format!("创建伏笔失败: {}", e))?;

    let foreshadowing = Foreshadowing {
        id: id.clone(),
        project_id: request.project_id,
        chapter_id: request.chapter_id,
        chapter_number: request.chapter_number,
        chapter_title: request.chapter_title,
        description: request.description,
        foreshadowing_type: request.foreshadowing_type,
        keywords: request.keywords.clone().unwrap_or_default(),
        status: Some(status),
        importance: Some(importance),
        expected_payoff_chapter: request.expected_payoff_chapter,
        actual_payoff_chapter: None,
        author_note: request.author_note,
        ai_confidence: None,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    log_command_success(&logger, "create_foreshadowing", &id);
    Ok(foreshadowing)
}

#[tauri::command]
pub async fn get_foreshadowings(
    app: AppHandle,
    project_id: String,
) -> Result<Vec<Foreshadowing>, String> {
    let logger = Logger::new().with_feature("foreshadowing");
    log_command_start(&logger, "get_foreshadowings", &project_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare("SELECT id, project_id, chapter_id, chapter_number, chapter_title, description, foreshadowing_type, keywords, status, importance, expected_payoff_chapter, actual_payoff_chapter, author_note, ai_confidence, created_at, updated_at FROM foreshadowings WHERE project_id = ?1 ORDER BY chapter_number ASC").map_err(|e| e.to_string())?;

    let mut foreshadowings = Vec::new();
    let mut rows = stmt.query(params![&project_id]).map_err(|e| e.to_string())?;

    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let keywords_json: String = row.get(6).map_err(|e| e.to_string())?;
        let keywords: Vec<String> = serde_json::from_str(&keywords_json).unwrap_or_default();

        foreshadowings.push(Foreshadowing {
            id: row.get(0).map_err(|e| e.to_string())?,
            project_id: row.get(1).map_err(|e| e.to_string())?,
            chapter_id: row.get(2).map_err(|e| e.to_string())?,
            chapter_number: row.get(3).map_err(|e| e.to_string())?,
            chapter_title: row.get(4).map_err(|e| e.to_string())?,
            description: row.get(5).map_err(|e| e.to_string())?,
            foreshadowing_type: row.get(7).map_err(|e| e.to_string())?,
            keywords,
            status: row.get(8).ok(),
            importance: row.get(9).ok(),
            expected_payoff_chapter: row.get(10).ok(),
            actual_payoff_chapter: row.get(11).ok(),
            author_note: row.get(12).ok(),
            ai_confidence: row.get(13).ok(),
            created_at: row.get(14).map_err(|e| e.to_string())?,
            updated_at: row.get(15).map_err(|e| e.to_string())?,
        });
    }

    log_command_success(&logger, "get_foreshadowings", &format!("获取{}个伏笔", foreshadowings.len()));
    Ok(foreshadowings)
}

#[tauri::command]
pub async fn resolve_foreshadowing(
    app: AppHandle,
    request: ResolveForeshadowingRequest,
) -> Result<Foreshadowing, String> {
    let logger = Logger::new().with_feature("foreshadowing");
    log_command_start(&logger, "resolve_foreshadowing", &request.foreshadowing_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE foreshadowings SET status = ?1, actual_payoff_chapter = ?2, updated_at = ?3 WHERE id = ?4",
        params![
            "paid_off",
            &request.actual_payoff_chapter,
            now,
            &request.foreshadowing_id,
        ],
    ).map_err(|e| format!("更新伏笔失败: {}", e))?;

    let foreshadowing: Foreshadowing = conn.query_row(
        "SELECT id, project_id, chapter_id, chapter_number, chapter_title, description, foreshadowing_type, keywords, status, importance, expected_payoff_chapter, actual_payoff_chapter, author_note, ai_confidence, created_at, updated_at FROM foreshadowings WHERE id = ?1",
        params![&request.foreshadowing_id],
        |row| {
            let keywords_json: String = row.get(6)?;
            let keywords: Vec<String> = serde_json::from_str(&keywords_json).unwrap_or_default();
            Ok(Foreshadowing {
                id: row.get(0)?,
                project_id: row.get(1)?,
                chapter_id: row.get(2)?,
                chapter_number: row.get(3)?,
                chapter_title: row.get(4)?,
                description: row.get(5)?,
                foreshadowing_type: row.get(7)?,
                keywords,
                status: row.get(8)?,
                importance: row.get(9)?,
                expected_payoff_chapter: row.get(10)?,
                actual_payoff_chapter: row.get(11)?,
                author_note: row.get(12)?,
                ai_confidence: row.get(13)?,
                created_at: row.get(14)?,
                updated_at: row.get(15)?,
            })
        },
    ).map_err(|e| format!("伏笔不存在: {}", e))?;

    log_command_success(&logger, "resolve_foreshadowing", &format!("伏笔回收于第{}章", request.actual_payoff_chapter));
    Ok(foreshadowing)
}

#[tauri::command]
pub async fn get_foreshadowing_stats(
    app: AppHandle,
    project_id: String,
) -> Result<ForeshadowingStats, String> {
    let logger = Logger::new().with_feature("foreshadowing");
    log_command_start(&logger, "get_foreshadowing_stats", &project_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let foreshadowings = get_foreshadowings(app.clone(), project_id).await?;

    let total = foreshadowings.len() as i32;
    let planted = foreshadowings.iter().filter(|f| f.status.as_deref() == Some("planted")).count() as i32;
    let paid_off = foreshadowings.iter().filter(|f| f.status.as_deref() == Some("paid_off")).count() as i32;

    let mut unresolved_count = 0;
    let mut overdue_count = 0;
    let mut total_distance = 0i32;
    let mut resolved_count = 0;

    for f in &foreshadowings {
        if f.status.as_deref() == Some("planted") {
            unresolved_count += 1;
        }
        if f.actual_payoff_chapter.is_some() {
            let distance = f.actual_payoff_chapter.unwrap() - f.chapter_number;
            total_distance += distance;
            resolved_count += 1;
        }
    }

    let avg_distance = if resolved_count > 0 {
        total_distance as f32 / resolved_count as f32
    } else {
        0.0
    };

    let mut recommendations = Vec::new();
    if unresolved_count > 3 {
        recommendations.push(format!("有{}个伏笔未回收，建议在后续章节中处理", unresolved_count));
    }
    if avg_distance > 10.0 {
        recommendations.push("伏笔回收距离较长，可能影响读者记忆".to_string());
    }

    let stats = ForeshadowingStats {
        total_foreshadowings: total,
        planted_count: planted,
        paid_off_count: paid_off,
        overdue_count,
        unresolved_count,
        abandoned_count: 0,
        avg_resolution_distance: avg_distance,
        recommendations,
    };

    log_command_success(&logger, "get_foreshadowing_stats", &format!("统计: 总数{}, 已回收{}", total, paid_off));
    Ok(stats)
}

#[tauri::command]
pub async fn calculate_emotion_curve(
    app: AppHandle,
    request: EmotionCurveRequest,
) -> Result<EmotionCurveResponse, String> {
    let logger = Logger::new().with_feature("emotion-curve");
    log_command_start(&logger, "calculate_emotion_curve", &request.arc_type);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let chapters: Vec<(String, String, i32)> = conn.prepare(
        "SELECT id, title, sort_order FROM chapters WHERE project_id = ?1 ORDER BY sort_order ASC"
    )
    .map_err(|e| e.to_string())?
    .query_map(params![&request.project_id], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    })
    .map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;

    let total_chapters = if request.total_chapters > 0 { request.total_chapters } else { chapters.len() as i32 };

    let arc_type = request.arc_type.as_str();
    let mut curve_data = Vec::new();

    for (i, (id, title, _)) in chapters.iter().enumerate() {
        let chapter_num = (i + 1) as i32;
        let position = if total_chapters > 0 { (chapter_num as f32) / (total_chapters as f32) } else { 0.5 };

        let (emotion_min, emotion_max, phase_name) = match arc_type {
            "standard" | "slow_burn" => {
                if position < 0.10 { (30, 50, "铺垫期") }
                else if position < 0.25 { (50, 70, "上升期") }
                else if position < 0.35 { (70, 90, "第一高潮") }
                else if position < 0.50 { (50, 70, "发展期") }
                else if position < 0.60 { (40, 60, "低谷期") }
                else if position < 0.75 { (60, 80, "反转期") }
                else if position < 0.90 { (75, 95, "最终上升") }
                else { (85, 100, "大高潮") }
            }
            "fast_paced" => {
                if position < 0.05 { (50, 65, "快速开场") }
                else if position < 0.20 { (65, 85, "第一波") }
                else if position < 0.35 { (55, 70, "短暂喘息") }
                else if position < 0.50 { (70, 90, "第二波") }
                else if position < 0.65 { (60, 75, "转折") }
                else if position < 0.80 { (75, 95, "第三波") }
                else { (85, 100, "终极高潮") }
            }
            "wave" => {
                if position < 0.10 { (30, 50, "开篇") }
                else if position < 0.20 { (60, 80, "小高潮1") }
                else if position < 0.30 { (40, 55, "回落1") }
                else if position < 0.40 { (65, 85, "小高潮2") }
                else if position < 0.50 { (45, 60, "回落2") }
                else if position < 0.60 { (70, 90, "中期高潮") }
                else if position < 0.70 { (50, 65, "回落3") }
                else if position < 0.80 { (75, 92, "小高潮3") }
                else if position < 0.90 { (55, 70, "最后回落") }
                else { (85, 100, "终极高潮") }
            }
            _ => {
                if position < 0.10 { (30, 50, "铺垫期") }
                else if position < 0.25 { (50, 70, "上升期") }
                else if position < 0.35 { (70, 90, "第一高潮") }
                else if position < 0.50 { (50, 70, "发展期") }
                else if position < 0.60 { (40, 60, "低谷期") }
                else if position < 0.75 { (60, 80, "反转期") }
                else if position < 0.90 { (75, 95, "最终上升") }
                else { (85, 100, "大高潮") }
            }
        };

        let segment_length = emotion_max - emotion_min;
        let segment_progress = if segment_length > 0 {
            let start = if position < 0.10 { 0.0 }
            else if position < 0.25 { 0.10 }
            else if position < 0.35 { 0.25 }
            else if position < 0.50 { 0.35 }
            else if position < 0.60 { 0.50 }
            else if position < 0.75 { 0.60 }
            else if position < 0.90 { 0.75 }
            else { 0.90 };
            (position - start) / 0.10
        } else { 0.5 };

        let emotion_target = emotion_min as f32 + (segment_progress * segment_length as f32);

        let (pacing, thrill_density, dialogue_ratio) = match phase_name.as_ref() {
            "铺垫期" | "开篇" => ("慢速", 0.3, 0.4),
            "上升期" | "快速开场" | "第一波" => ("中速", 0.5, 0.5),
            "第一高潮" | "小高潮1" | "小高潮2" | "小高潮3" => ("快速", 0.8, 0.6),
            "发展期" | "短暂喘息" | "回落1" | "回落2" | "回落3" | "最后回落" => ("中速", 0.4, 0.7),
            "低谷期" => ("慢速", 0.2, 0.8),
            "反转期" | "转折" => ("变速", 0.9, 0.5),
            "最终上升" | "第三波" => ("中速", 0.6, 0.6),
            "大高潮" | "终极高潮" => ("快速", 0.95, 0.4),
            _ => ("中速", 0.5, 0.5),
        };

        let recommendations = if emotion_target > 80.0 {
            vec!["本章情绪强度较高，注意控制节奏".to_string()]
        } else if emotion_target < 40.0 {
            vec!["本章情绪较低，可以增加冲突".to_string()]
        } else {
            vec![]
        };

        curve_data.push(EmotionCurveData {
            chapter_number: chapter_num,
            chapter_title: title.clone(),
            position,
            phase_name: phase_name.to_string(),
            emotion_target,
            emotion_range: (emotion_min, emotion_max),
            pacing: pacing.to_string(),
            thrill_density,
            dialogue_ratio,
            recommendations,
        });
    }

    let emotions: Vec<f32> = curve_data.iter().map(|d| d.emotion_target).collect();
    let avg_emotion = if emotions.is_empty() { 0.0 } else { emotions.iter().sum::<f32>() / emotions.len() as f32 };

    let emotion_variance = if emotions.len() > 1 {
        let mean = avg_emotion;
        let variance: f32 = emotions.iter().map(|&e| (e - mean).powf(2.0)).sum::<f32>() / emotions.len() as f32;
        variance.sqrt()
    } else { 0.0 };

    let climax_chapters: Vec<i32> = curve_data.iter()
        .filter(|d| d.emotion_target > 75.0)
        .map(|d| d.chapter_number)
        .collect();

    let pacing_balance = 0.5;

    let overall_stats = EmotionCurveStats {
        avg_emotion,
        emotion_variance,
        climax_chapters,
        pacing_balance,
    };

    let data_count = curve_data.len();
    let response = EmotionCurveResponse {
        arc_type: request.arc_type.clone(),
        total_chapters,
        curve_data,
        overall_stats,
    };

    log_command_success(&logger, "calculate_emotion_curve", &format!("生成{}条数据", data_count));
    Ok(response)
}

#[tauri::command]
pub async fn optimize_chapter(
    app: AppHandle,
    request: OptimizeChapterRequest,
) -> Result<OptimizeChapterResponse, String> {
    let logger = Logger::new().with_feature("optimizer");
    log_command_start(&logger, "optimize_chapter", &format!("章节ID: {}, 维度: {}", request.chapter_id, request.dimension));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path)
        .map_err(|e| {
            logger.error(&format!("Failed to get database connection: {}", e));
            format!("数据库连接失败: {}", e)
        })?;

    // 验证优化维度
    let dimension = request.dimension.as_str();
    if !matches!(dimension, "dialogue" | "environment" | "psychology" | "rhythm") {
        log_command_error(&logger, "optimize_chapter", &format!("不支持的优化维度: {}", dimension));
        return Err(format!("不支持的优化维度: {}, 支持的维度: dialogue, environment, psychology, rhythm", dimension));
    }

    // 获取章节内容
    let chapter_result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT content FROM chapters WHERE id = ?1",
        params![request.chapter_id],
        |row| row.get(0),
    );

    let original_content = chapter_result.map_err(|e| {
        logger.error(&format!("Failed to get chapter: {}", e));
        format!("获取章节失败: {}", e)
    })?;

    if original_content.is_empty() {
        log_command_error(&logger, "optimize_chapter", "章节内容为空");
        return Err("章节内容为空，无法优化".to_string());
    }

    // 获取项目角色信息（用于对话和心理优化）
    let project_id_result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT project_id FROM chapters WHERE id = ?1",
        params![&request.chapter_id],
        |row| row.get(0),
    );

    let project_id = project_id_result.map_err(|e| {
        logger.error(&format!("Failed to get project_id: {}", e));
        format!("获取项目ID失败: {}", e)
    })?;

    let character_context = if matches!(dimension, "dialogue" | "psychology") {
        let mut characters_stmt = conn.prepare(
            "SELECT name, personality, background, extra FROM characters WHERE project_id = ?1 LIMIT 5"
        ).map_err(|e| {
            logger.error(&format!("Failed to prepare characters query: {}", e));
            format!("准备角色查询失败: {}", e)
        })?;

        let mut character_info = String::new();
        let mut character_rows = characters_stmt.query(params![&project_id])
            .map_err(|e| {
                logger.error(&format!("Failed to query characters: {}", e));
                format!("查询角色失败: {}", e)
            })?;

        while let Some(character) = character_rows.next().map_err(|e| {
            logger.error(&format!("Failed to iterate characters: {}", e));
            format!("迭代角色失败: {}", e)
        })? {
            let name: String = character.get(0).unwrap_or_default();
            let personality: String = character.get(1).unwrap_or_default();
            let background: String = character.get(2).unwrap_or_default();
            if !name.is_empty() {
                character_info.push_str(&format!("\n- {}: {}, {}", name, personality, background));
            }
        }
        if !character_info.is_empty() {
            Some(character_info)
        } else {
            None
        }
    } else {
        None
    };

    // 构建优化提示词
    let system_prompt = match dimension {
        "dialogue" => r#"你是一位专注于小说对话优化的编辑大师。你的任务是对已有的章节内容进行对话层面的深度优化，让每一句对话都更加生动、真实、有层次。

## 优化原则

### 1. 角色声音独特化
每个角色都应该有独特的说话方式：
- 用词习惯（文雅/粗犷/专业术语/网络用语）
- 句式特点（长句/短句/疑问句多/陈述句多）
- 语气词使用（嗯、啊、呢、吧、哦）
- 口头禅和特殊表达

### 2. 潜台词丰富化
好的对话从来不直接表达真实想法：
- "你还好吗？" → 可能是"你还爱我吗？"
- "随便你。" → 可能是"你敢试试看。"
- "我没事。" → 可能是"我很受伤但不想说。"

### 3. 对话节奏感
- 紧张时：短句、打断、重复
- 放松时：长句、完整表达、闲聊
- 冲突时：针锋相对、话中带刺
- 和解时：欲言又止、小心试探

### 4. 非语言元素
对话不只是说话，还包括：
- 说话时的动作（转身、低头、握拳）
- 表情变化（皱眉、微笑、眼神闪烁）
- 语气变化（声音变小、语速加快）
- 停顿和沉默

### 5. 信息传递效率
- 删除无意义的寒暄
- 通过对话推动情节
- 在对话中自然地透露信息
- 避免"说明文式"对话

## 输入格式
{
  "original_content": "需要优化的章节内容",
  "characters": "角色信息",
  "additional_notes": "额外优化指令"
}

## 输出格式
{
  "optimized_content": "优化后的完整章节内容",
  "optimization_notes": "优化说明，列出主要改动点"
}

## 注意事项
1. **保持原意**：优化对话但不改变情节走向
2. **角色一致**：确保优化后的对话符合角色设定
3. **适度原则**：不是每句话都需要潜台词，自然为上
4. **上下文连贯**：优化后的对话要与前后文衔接流畅

请只返回JSON格式的结果，不要包含其他文字。"#,
        "environment" => r#"你是一位专注于小说环境描写的编辑大师。你的任务是对已有的章节内容进行环境描写层面的深度优化，让场景更加生动、氛围更加浓郁、环境与情节更加融合。

## 优化原则

### 1. 环境服务于情绪
环境描写不是装饰，而是情绪的放大器：
- 悲伤场景：阴雨、枯叶、冷色调
- 紧张场景：狭窄空间、昏暗光线、不详的声音
- 温馨场景：暖光、熟悉的气味、柔和的触感
- 希望场景：破晓、新绿、清新的空气

### 2. 五感全开
好的环境描写调动所有感官：
- **视觉**：颜色、光影、形状、动态
- **听觉**：声音、噪音、沉默、回响
- **嗅觉**：气味、香气、臭味、熟悉的味道
- **触觉**：温度、质感、湿度、风
- **味觉**：空气的味道、嘴里的感觉

### 3. 细节的选择性
不是描写所有东西，而是选择有意义的细节：
- 能反映角色心理的细节
- 能暗示情节发展的细节
- 能增强氛围的细节
- 能唤起读者共鸣的细节

### 4. 动态环境
环境不是静止的背景：
- 光线在变化
- 声音在起伏
- 气味在流动
- 温度在改变

### 5. 环境与角色互动
角色如何感知和回应环境：
- 角色注意到什么？（反映心理状态）
- 角色如何与环境互动？（反映性格）
- 环境如何影响角色的行为？

## 输入格式
{
  "original_content": "需要优化的章节内容",
  "target_emotion": "目标情绪氛围",
  "additional_notes": "额外优化指令"
}

## 输出格式
{
  "optimized_content": "优化后的完整章节内容",
  "optimization_notes": "优化说明，列出主要改动点"
}

## 注意事项
1. **适度原则**：环境描写要恰到好处，不要喧宾夺主
2. **节奏配合**：紧张情节少描写，舒缓情节多渲染
3. **角色视角**：通过角色的眼睛看环境，而不是上帝视角
4. **前后呼应**：环境的变化可以暗示情节的发展
5. **避免俗套**：寻找新鲜的比喻和描写角度

请只返回JSON格式的结果，不要包含其他文字。"#,
        "psychology" => r#"你是一位专注于小说心理描写的编辑大师。你的任务是对已有的章节内容进行心理活动层面的深度优化，让角色的内心世界更加丰富、真实、有层次。

## 优化原则

### 1. 心理活动要符合角色DNA
如果角色有DNA档案，心理活动必须与之一致：
- **童年创伤**会在特定情境下被触发
- **核心恐惧**会影响角色的判断和反应
- **内心渴望**会在关键时刻浮现
- **思维模式**决定了角色如何处理信息

### 2. 情绪的复杂性
真实的人不会只有一种情绪：
- 愤怒里会有委屈
- 悲伤里会有解脱
- 快乐里会有不安
- 恐惧里会有好奇

### 3. 思维的跳跃性
人的思维不是线性的：
- 会突然想起不相关的事
- 会被某个细节带走
- 会在重要时刻走神
- 会有莫名其妙的联想

### 4. 内心与外在的矛盾
人说的和想的往往不一样：
- 嘴上说"没关系"，心里在滴血
- 表面冷静，内心慌乱
- 装作不在意，其实很在意
- 故作坚强，实则脆弱

### 5. 心理活动的节奏
- 紧张时：思维快速、碎片化、重复
- 放松时：思维舒缓、发散、联想丰富
- 震惊时：思维停滞、空白、慢动作
- 决策时：思维来回、权衡、挣扎

## 心理描写技巧

### 1. 内心独白
直接展示角色的想法：
> 完了。这下真的完了。他盯着那封邮件，大脑像是被按了暂停键。为什么？为什么偏偏是今天？

### 2. 意识流
展示思维的自然流动：
> 咖啡凉了。她应该再点一杯。但是钱包里只剩下三十块。三十块。妈妈说过，女孩子出门要带够钱。妈妈。妈妈现在在做什么？

### 3. 身体反应
通过身体感受展示心理：
> 胃像是被人攥住了，一阵阵地抽紧。他知道这种感觉——上一次是三年前，在医院的走廊里。

### 4. 记忆闪回
用回忆展示心理根源：
> "你永远都是这样。"她的声音和十年前一模一样。十年前，在那个下着雨的傍晚，母亲也是用这种语气说的。

### 5. 自我对话
角色与自己的对话：
> 冷静。你要冷静。深呼吸。一、二、三......不行，心跳还是太快了。再来一次。

## 输入格式
{
  "original_content": "需要优化的章节内容",
  "character_dna": "角色DNA信息",
  "additional_notes": "额外优化指令"
}

## 输出格式
{
  "optimized_content": "优化后的完整章节内容",
  "optimization_notes": "优化说明，列出主要改动点"
}

## 注意事项
1. **真实性**：心理活动要符合角色设定和情境
2. **适度性**：不是每个时刻都需要大段心理描写
3. **节奏感**：心理描写的长度要与情节节奏匹配
4. **独特性**：每个角色的心理活动应该有不同的风格
5. **连贯性**：心理变化要有逻辑，不能跳跃太大

请只返回JSON格式的结果，不要包含其他文字。"#,
        "rhythm" => r#"你是一位专注于小说节奏和韵律的编辑大师。你的任务是优化文章的节奏感，让阅读体验更加流畅和沉浸。

## 优化原则

### 1. 句子长度变化
- 长短句交替，像呼吸一样自然
- 紧张时用短句，舒缓时用长句
- 避免连续多个相同长度的句子

### 2. 段落节奏
- 重要情节放慢，细致描写
- 过渡情节加快，简洁带过
- 高潮部分可以用单句成段

### 3. 标点符号
- 善用省略号表示思绪飘散
- 用破折号表示突然转念
- 感叹号要克制使用

### 4. 韵律感
- 注意句尾的音节变化
- 避免重复的句式结构
- 适当使用排比增强气势

## 输入格式
{
  "original_content": "需要优化的章节内容",
  "additional_notes": "额外优化指令"
}

## 输出格式
{
  "optimized_content": "优化后的完整章节内容",
  "optimization_notes": "优化说明，列出主要改动点"
}

## 注意事项
1. **自然流畅**：节奏变化要自然，不要刻意
2. **符合情绪**：节奏要配合场景情绪
3. **保持原意**：不要因为节奏改变而改变原意
4. **适度原则**：不要过度追求技巧而失去自然

请只返回JSON格式的结果，不要包含其他文字。"#,
        _ => return Err(format!("不支持的优化维度: {}", dimension)),
    };

    // 构建用户消息
    let dimension_name = match dimension {
        "dialogue" => "对话",
        "environment" => "环境描写",
        "psychology" => "心理活动",
        "rhythm" => "节奏韵律",
        _ => "未知",
    };

    let mut user_input = serde_json::json!({
        "original_content": original_content,
        "additional_notes": request.additional_notes.unwrap_or_else(|| "无额外指令".to_string())
    });

    if let Some(characters) = character_context {
        user_input["characters"] = serde_json::Value::String(characters);
    }

    // 调用AI服务
    let ai_service = AIService::new();

    let ai_request = AICompletionRequest {
        model_id: "default".to_string(),
        context: system_prompt.to_string(),
        instruction: user_input.to_string(),
        temperature: Some(0.7),
        max_tokens: Some(8000),
        stream: Some(false),
        character_context: None,
        worldview_context: None,
        project_id: None,
        chapter_mission_id: None,
    };

    let ai_response = ai_service.continue_novel(ai_request, None).await.map_err(|e| {
        logger.error(&format!("AI optimization failed: {}", e));
        format!("AI优化失败: {}", e)
    })?;

    // 解析AI响应
    let response_text = ai_response.trim();

    // 尝试从响应中提取JSON
    let (optimized_content, optimization_notes) = if response_text.contains("{") && response_text.contains("}") {
        // 尝试提取JSON部分
        let start_idx = response_text.find('{').unwrap_or(0);
        let end_idx = response_text.rfind('}').unwrap_or(response_text.len());
        let json_str = &response_text[start_idx..=end_idx];

        match serde_json::from_str::<serde_json::Value>(json_str) {
            Ok(parsed) => {
                let content = parsed.get("optimized_content")
                    .and_then(|v| v.as_str())
                    .unwrap_or(response_text);
                let notes = parsed.get("optimization_notes")
                    .and_then(|v| v.as_str())
                    .unwrap_or("优化完成");
                (content.to_string(), notes.to_string())
            }
            Err(_) => {
                logger.warn("Failed to parse JSON from AI response, using raw text");
                (response_text.to_string(), "优化完成（响应格式非标准JSON）".to_string())
            }
        }
    } else {
        (response_text.to_string(), "优化完成".to_string())
    };

    let response = OptimizeChapterResponse {
        optimized_content,
        optimization_notes,
        dimension: dimension.to_string(),
    };

    log_command_success(&logger, "optimize_chapter", &format!("维度: {}, 完成", dimension_name));
    Ok(response)
}

#[tauri::command]
pub async fn create_blueprint(
    app: AppHandle,
    request: CreateBlueprintRequest,
) -> Result<Blueprint, String> {
    let logger = Logger::new().with_feature("blueprint");
    log_command_start(&logger, "create_blueprint", &format!("项目ID: {}, 标题: {}", request.project_id, request.title));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    // 生成唯一ID
    let blueprint_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    // 收集项目信息用于AI生成
    let (characters_json, relationships_json, settings_json) = {
        let mut characters_stmt = conn.prepare("SELECT name, personality, role, extra FROM characters WHERE project_id = ?1").map_err(|e| {
            logger.error(&format!("Failed to prepare characters query: {}", e));
            format!("准备角色查询失败: {}", e)
        })?;

        let characters: Vec<serde_json::Value> = characters_stmt
            .query_map(params![&request.project_id], |row| {
                Ok(serde_json::json!({
                    "name": row.get::<_, String>(0).unwrap_or_default(),
                    "personality": row.get::<_, String>(1).unwrap_or_default(),
                    "role": row.get::<_, String>(2).unwrap_or_default(),
                    "extra": row.get::<_, String>(3).unwrap_or_default(),
                }))
            })
            .map_err(|e| {
                logger.error(&format!("Failed to query characters: {}", e));
                format!("查询角色失败: {}", e)
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                logger.error(&format!("Failed to collect characters: {}", e));
                format!("收集角色数据失败: {}", e)
            })?;

        let mut relationships_stmt = conn.prepare("SELECT from_char, to_char, relationship_type, description FROM character_relations WHERE project_id = ?1").map_err(|e| {
            logger.error(&format!("Failed to prepare relationships query: {}", e));
            format!("准备关系查询失败: {}", e)
        })?;

        let relationships: Vec<serde_json::Value> = relationships_stmt
            .query_map(params![&request.project_id], |row| {
                Ok(serde_json::json!({
                    "from": row.get::<_, String>(0).unwrap_or_default(),
                    "to": row.get::<_, String>(1).unwrap_or_default(),
                    "relationship_type": row.get::<_, String>(2).unwrap_or_default(),
                    "description": row.get::<_, String>(3).unwrap_or_default(),
                }))
            })
            .map_err(|e| {
                logger.error(&format!("Failed to query relationships: {}", e));
                format!("查询关系失败: {}", e)
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                logger.error(&format!("Failed to collect relationships: {}", e));
                format!("收集关系数据失败: {}", e)
            })?;

        let mut settings_stmt = conn.prepare("SELECT category, name, description, details FROM world_views WHERE project_id = ?1").map_err(|e| {
            logger.error(&format!("Failed to prepare settings query: {}", e));
            format!("准备设定查询失败: {}", e)
        })?;

        let settings: Vec<serde_json::Value> = settings_stmt
            .query_map(params![&request.project_id], |row| {
                Ok(serde_json::json!({
                    "category": row.get::<_, String>(0).unwrap_or_default(),
                    "name": row.get::<_, String>(1).unwrap_or_default(),
                    "description": row.get::<_, String>(2).unwrap_or_default(),
                    "details": row.get::<_, String>(3).unwrap_or_default(),
                }))
            })
            .map_err(|e| {
                logger.error(&format!("Failed to query settings: {}", e));
                format!("查询设定失败: {}", e)
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                logger.error(&format!("Failed to collect settings: {}", e));
                format!("收集设定数据失败: {}", e)
            })?;

        (
            serde_json::to_string(&characters).unwrap_or_default(),
            serde_json::to_string(&relationships).unwrap_or_default(),
            serde_json::to_string(&settings).unwrap_or_default(),
        )
    };

    // 调用AI生成蓝图
    let ai_service = AIService::new();

    let system_prompt = r#"你是一位世界蓝图构建专家。你的任务是根据提供的故事信息，生成一份完整的世界蓝图。

## 蓝图结构
蓝图包含三个核心部分：
1. 角色蓝图 - 定义主要角色的核心特质和故事定位
2. 关系蓝图 - 定义角色之间的动态关系
3. 设定蓝图 - 定义世界观和关键设定

## 输入信息
{
  "title": "故事标题",
  "genre": "类型",
  "target_length": "目标字数",
  "characters": [角色列表],
  "relationships": [关系列表],
  "settings": [设定列表]
}

## 输出格式
返回JSON格式的蓝图，包含：
{
  "characters": [
    {
      "name": "角色名",
      "role": "角色定位（主角/反派/配角等）",
      "personality": "性格描述",
      "background": "背景故事",
      "arc_type": "角色弧类型",
      "is_main_character": true/false
    }
  ],
  "relationships": [
    {
      "from": "角色A",
      "to": "角色B",
      "relationship_type": "关系类型",
      "description": "关系描述"
    }
  ],
  "settings": [
    {
      "category": "类别",
      "name": "设定名称",
      "description": "描述",
      "details": "详细说明"
    }
  ]
}

## 注意事项
1. 角色要区分主次，主要角色要有完整的弧线
2. 关系要动态且有意义，能推动情节发展
3. 设定要服务于故事，避免冗余
4. 保持与已有信息的一致性

请只返回JSON格式的结果，不要包含其他文字。"#;

    let user_input = serde_json::json!({
        "title": request.title,
        "genre": request.genre,
        "target_length": request.target_length,
        "characters": characters_json,
        "relationships": relationships_json,
        "settings": settings_json,
    });

    let ai_request = AICompletionRequest {
        model_id: "default".to_string(),
        context: system_prompt.to_string(),
        instruction: user_input.to_string(),
        temperature: Some(0.7),
        max_tokens: Some(4000),
        stream: Some(false),
        character_context: None,
        worldview_context: None,
        project_id: None,
        chapter_mission_id: None,
    };

    let ai_response = ai_service.continue_novel(ai_request, None).await.map_err(|e| {
        logger.error(&format!("AI blueprint generation failed: {}", e));
        format!("AI蓝图生成失败: {}", e)
    })?;

    // 解析AI响应
    let response_text = ai_response.trim();
    let (bp_characters, bp_relationships, bp_settings) = if response_text.contains("{") && response_text.contains("}") {
        let start_idx = response_text.find('{').unwrap_or(0);
        let end_idx = response_text.rfind('}').unwrap_or(response_text.len());
        let json_str = &response_text[start_idx..=end_idx];

        match serde_json::from_str::<serde_json::Value>(json_str) {
            Ok(parsed) => {
                let chars = parsed.get("characters")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter().filter_map(|v| {
                            Some(BlueprintCharacter {
                                name: v.get("name")?.as_str()?.to_string(),
                                role: v.get("role")?.as_str().map(|s| s.to_string()),
                                personality: v.get("personality")?.as_str().map(|s| s.to_string()),
                                background: v.get("background")?.as_str().map(|s| s.to_string()),
                                arc_type: v.get("arc_type")?.as_str().map(|s| s.to_string()),
                                is_main_character: v.get("is_main_character")?.as_bool().unwrap_or(false),
                            })
                        }).collect()
                    })
                    .unwrap_or_default();
                let rels = parsed.get("relationships")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter().filter_map(|v| {
                            Some(BlueprintRelationship {
                                from: v.get("from")?.as_str()?.to_string(),
                                to: v.get("to")?.as_str()?.to_string(),
                                relationship_type: v.get("relationship_type")?.as_str()?.to_string(),
                                description: v.get("description")?.as_str().map(|s| s.to_string()),
                            })
                        }).collect()
                    })
                    .unwrap_or_default();
                let sets = parsed.get("settings")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter().filter_map(|v| {
                            Some(BlueprintSetting {
                                category: v.get("category")?.as_str()?.to_string(),
                                name: v.get("name")?.as_str()?.to_string(),
                                description: v.get("description")?.as_str().map(|s| s.to_string()),
                                details: v.get("details")?.as_str().map(|s| s.to_string()),
                            })
                        }).collect()
                    })
                    .unwrap_or_default();
                (chars, rels, sets)
            }
            Err(_) => {
                logger.warn("Failed to parse JSON from AI response, using empty arrays");
                (vec![], vec![], vec![])
            }
        }
    } else {
        (vec![], vec![], vec![])
    };

    let characters_json = serde_json::to_string(&bp_characters).unwrap_or_default();
    let relationships_json = serde_json::to_string(&bp_relationships).unwrap_or_default();
    let settings_json = serde_json::to_string(&bp_settings).unwrap_or_default();

    // 插入数据库
    conn.execute(
        "INSERT INTO blueprints (id, project_id, title, genre, target_length, characters, relationships, settings, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            &blueprint_id,
            &request.project_id,
            &request.title,
            &request.genre,
            &request.target_length,
            &characters_json,
            &relationships_json,
            &settings_json,
            &now,
            &now,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to insert blueprint: {}", e));
        format!("插入蓝图失败: {}", e)
    })?;

    let blueprint = Blueprint {
        id: blueprint_id.clone(),
        project_id: request.project_id,
        title: request.title,
        genre: request.genre,
        target_length: request.target_length,
        characters: bp_characters,
        relationships: bp_relationships,
        settings: bp_settings,
        created_at: now.clone(),
        updated_at: now,
    };

    log_command_success(&logger, "create_blueprint", &format!("蓝图ID: {}", blueprint_id));
    Ok(blueprint)
}

#[tauri::command]
pub async fn get_blueprint(
    app: AppHandle,
    project_id: String,
) -> Result<Option<Blueprint>, String> {
    let logger = Logger::new().with_feature("blueprint");
    log_command_start(&logger, "get_blueprint", &format!("项目ID: {}", project_id));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let result = conn.query_row(
        "SELECT id, project_id, title, genre, target_length, characters, relationships, settings, created_at, updated_at
        FROM blueprints WHERE project_id = ?1",
        params![&project_id],
        |row| {
            let characters_json: String = row.get(4).unwrap_or_default();
            let relationships_json: String = row.get(5).unwrap_or_default();
            let settings_json: String = row.get(6).unwrap_or_default();

            let characters: Vec<BlueprintCharacter> = serde_json::from_str(&characters_json).unwrap_or_default();
            let relationships: Vec<BlueprintRelationship> = serde_json::from_str(&relationships_json).unwrap_or_default();
            let settings: Vec<BlueprintSetting> = serde_json::from_str(&settings_json).unwrap_or_default();

            Ok(Blueprint {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                genre: row.get(3).ok(),
                target_length: row.get(4).ok(),
                characters,
                relationships,
                settings,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        },
    );

    match result {
        Ok(blueprint) => {
            log_command_success(&logger, "get_blueprint", "找到蓝图");
            Ok(Some(blueprint))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            log_command_success(&logger, "get_blueprint", "未找到蓝图");
            Ok(None)
        }
        Err(e) => {
            logger.error(&format!("Failed to query blueprint: {}", e));
            Err(format!("查询蓝图失败: {}", e))
        }
    }
}

#[tauri::command]
pub async fn update_blueprint(
    app: AppHandle,
    request: UpdateBlueprintRequest,
) -> Result<Blueprint, String> {
    let logger = Logger::new().with_feature("blueprint");
    log_command_start(&logger, "update_blueprint", &format!("蓝图ID: {}", request.blueprint_id));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    // 获取现有蓝图
    let existing = conn.query_row(
        "SELECT id, project_id, title, genre, target_length, characters, relationships, settings, created_at, updated_at
        FROM blueprints WHERE id = ?1",
        params![&request.blueprint_id],
        |row| {
            let characters_json: String = row.get(4).unwrap_or_default();
            let relationships_json: String = row.get(5).unwrap_or_default();
            let settings_json: String = row.get(6).unwrap_or_default();

            let characters: Vec<BlueprintCharacter> = serde_json::from_str(&characters_json).unwrap_or_default();
            let relationships: Vec<BlueprintRelationship> = serde_json::from_str(&relationships_json).unwrap_or_default();
            let settings: Vec<BlueprintSetting> = serde_json::from_str(&settings_json).unwrap_or_default();

            Ok(Blueprint {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                genre: row.get(3).ok(),
                target_length: row.get(4).ok(),
                characters,
                relationships,
                settings,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        },
    );

    let mut blueprint = match existing {
        Ok(bp) => bp,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            log_command_error(&logger, "update_blueprint", "未找到蓝图");
            return Err("未找到指定的蓝图".to_string());
        }
        Err(e) => {
            logger.error(&format!("Failed to query blueprint: {}", e));
            return Err(format!("查询蓝图失败: {}", e));
        }
    };

    // 更新字段
    if let Some(title) = request.title {
        blueprint.title = title;
    }
    if let Some(genre) = request.genre {
        blueprint.genre = Some(genre);
    }
    if let Some(target_length) = request.target_length {
        blueprint.target_length = Some(target_length);
    }
    if let Some(characters) = request.characters {
        blueprint.characters = characters;
    }
    if let Some(relationships) = request.relationships {
        blueprint.relationships = relationships;
    }
    if let Some(settings) = request.settings {
        blueprint.settings = settings;
    }

    let now = Utc::now().to_rfc3339();
    let characters_json = serde_json::to_string(&blueprint.characters).unwrap_or_default();
    let relationships_json = serde_json::to_string(&blueprint.relationships).unwrap_or_default();
    let settings_json = serde_json::to_string(&blueprint.settings).unwrap_or_default();

    // 更新数据库
    conn.execute(
        "UPDATE blueprints SET title = ?1, genre = ?2, target_length = ?3, characters = ?4, relationships = ?5, settings = ?6, updated_at = ?7
        WHERE id = ?8",
        params![
            &blueprint.title,
            &blueprint.genre,
            &blueprint.target_length,
            &characters_json,
            &relationships_json,
            &settings_json,
            &now,
            &request.blueprint_id,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to update blueprint: {}", e));
        format!("更新蓝图失败: {}", e)
    })?;

    blueprint.updated_at = now;

    log_command_success(&logger, "update_blueprint", "更新完成");
    Ok(blueprint)
}

#[tauri::command]
pub async fn create_chapter_mission(
    app: AppHandle,
    request: CreateChapterMissionRequest,
) -> Result<ChapterMission, String> {
    let logger = Logger::new().with_feature("chapter_mission");
    log_command_start(&logger, "create_chapter_mission", &format!("章节ID: {}, 章节号: {}", request.chapter_id, request.chapter_number));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let mission_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO chapter_missions (id, chapter_id, chapter_number, macro_beat, micro_beats, pov, tone, pacing, allowed_new_characters, forbidden_characters, beat_id, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            &mission_id,
            &request.chapter_id,
            &request.chapter_number,
            None::<String>,
            None::<String>,
            None::<String>,
            None::<String>,
            None::<String>,
            None::<String>,
            None::<String>,
            &now,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to insert chapter mission: {}", e));
        format!("插入章节导演脚本失败: {}", e)
    })?;

    let mission = ChapterMission {
        id: mission_id.clone(),
        chapter_id: request.chapter_id,
        chapter_number: request.chapter_number,
        macro_beat: String::new(),
        micro_beats: vec![],
        pov: None,
        tone: None,
        pacing: None,
        allowed_new_characters: vec![],
        forbidden_characters: vec![],
        beat_id: None,
        created_at: now,
    };

    log_command_success(&logger, "create_chapter_mission", &format!("导演脚本ID: {}", mission_id));
    Ok(mission)
}

#[tauri::command]
pub async fn get_chapter_mission(
    app: AppHandle,
    chapter_id: String,
) -> Result<Option<ChapterMission>, String> {
    let logger = Logger::new().with_feature("chapter_mission");
    log_command_start(&logger, "get_chapter_mission", &format!("章节ID: {}", chapter_id));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let result = conn.query_row(
        "SELECT id, chapter_id, chapter_number, macro_beat, micro_beats, pov, tone, pacing, allowed_new_characters, forbidden_characters, beat_id, created_at
            FROM chapter_missions WHERE chapter_id = ?1",
        params![&chapter_id],
        |row| {
            let micro_beats_json: String = row.get(4).unwrap_or_default();
            let allowed_new_json: String = row.get(7).unwrap_or_default();
            let forbidden_json: String = row.get(8).unwrap_or_default();

            let micro_beats: Vec<String> = serde_json::from_str(&micro_beats_json).unwrap_or_default();
            let allowed_new: Vec<String> = serde_json::from_str(&allowed_new_json).unwrap_or_default();
            let forbidden: Vec<String> = serde_json::from_str(&forbidden_json).unwrap_or_default();

            Ok(ChapterMission {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                chapter_number: row.get(2)?,
                macro_beat: row.get(3).unwrap_or_default(),
                micro_beats,
                pov: row.get(5).ok(),
                tone: row.get(6).ok(),
                pacing: row.get(7).ok(),
                allowed_new_characters: allowed_new,
                forbidden_characters: forbidden,
                beat_id: row.get(9).ok(),
                created_at: row.get(10)?,
            })
        },
    );

    match result {
        Ok(mission) => {
            log_command_success(&logger, "get_chapter_mission", "找到导演脚本");
            Ok(Some(mission))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            log_command_success(&logger, "get_chapter_mission", "未找到导演脚本");
            Ok(None)
        }
        Err(e) => {
            logger.error(&format!("Failed to query chapter mission: {}", e));
            Err(format!("查询章节导演脚本失败: {}", e))
        }
    }
}

#[tauri::command]
pub async fn update_chapter_mission(
    app: AppHandle,
    request: UpdateChapterMissionRequest,
) -> Result<ChapterMission, String> {
    let logger = Logger::new().with_feature("chapter_mission");
    log_command_start(&logger, "update_chapter_mission", &format!("导演脚本ID: {}", request.mission_id));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let existing = conn.query_row(
        "SELECT id, chapter_id, chapter_number, macro_beat, micro_beats, pov, tone, pacing, allowed_new_characters, forbidden_characters, beat_id, created_at
            FROM chapter_missions WHERE id = ?1",
        params![&request.mission_id],
        |row| {
            let micro_beats_json: String = row.get(4).unwrap_or_default();
            let allowed_new_json: String = row.get(7).unwrap_or_default();
            let forbidden_json: String = row.get(8).unwrap_or_default();

            let micro_beats: Vec<String> = serde_json::from_str(&micro_beats_json).unwrap_or_default();
            let allowed_new: Vec<String> = serde_json::from_str(&allowed_new_json).unwrap_or_default();
            let forbidden: Vec<String> = serde_json::from_str(&forbidden_json).unwrap_or_default();

            Ok(ChapterMission {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                chapter_number: row.get(2)?,
                macro_beat: row.get(3).unwrap_or_default(),
                micro_beats,
                pov: row.get(5).ok(),
                tone: row.get(6).ok(),
                pacing: row.get(7).ok(),
                allowed_new_characters: allowed_new,
                forbidden_characters: forbidden,
                beat_id: row.get(9).ok(),
                created_at: row.get(10)?,
            })
        },
    );

    let mut mission = match existing {
        Ok(m) => m,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            log_command_error(&logger, "update_chapter_mission", "未找到导演脚本");
            return Err("未找到指定的章节导演脚本".to_string());
        }
        Err(e) => {
            logger.error(&format!("Failed to query chapter mission: {}", e));
            return Err(format!("查询章节导演脚本失败: {}", e));
        }
    };

    if let Some(macro_beat) = request.macro_beat {
        mission.macro_beat = macro_beat;
    }
    if let Some(micro_beats) = request.micro_beats {
        mission.micro_beats = micro_beats;
    }
    if let Some(pov) = request.pov {
        mission.pov = Some(pov);
    }
    if let Some(tone) = request.tone {
        mission.tone = Some(tone);
    }
    if let Some(pacing) = request.pacing {
        mission.pacing = Some(pacing);
    }
    if let Some(allowed_new) = request.allowed_new_characters {
        mission.allowed_new_characters = allowed_new;
    }
    if let Some(forbidden) = request.forbidden_characters {
        mission.forbidden_characters = forbidden;
    }
    if let Some(beat_id) = request.beat_id {
        mission.beat_id = Some(beat_id);
    }

    let micro_beats_json = serde_json::to_string(&mission.micro_beats).unwrap_or_default();
    let allowed_new_json = serde_json::to_string(&mission.allowed_new_characters).unwrap_or_default();
    let forbidden_json = serde_json::to_string(&mission.forbidden_characters).unwrap_or_default();

    conn.execute(
        "UPDATE chapter_missions SET macro_beat = ?1, micro_beats = ?2, pov = ?3, tone = ?4, pacing = ?5, allowed_new_characters = ?6, forbidden_characters = ?7, beat_id = ?8
            WHERE id = ?9",
        params![
            &mission.macro_beat,
            &micro_beats_json,
            &mission.pov,
            &mission.tone,
            &mission.pacing,
            &allowed_new_json,
            &forbidden_json,
            &mission.beat_id,
            &request.mission_id,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to update chapter mission: {}", e));
        format!("更新章节导演脚本失败: {}", e)
    })?;

    log_command_success(&logger, "update_chapter_mission", "更新完成");
    Ok(mission)
}

#[tauri::command]
pub async fn generate_chapter_mission_with_ai(
    app: AppHandle,
    chapter_id: String,
    chapter_number: i32,
    chapter_outline: Option<String>,
    blueprint_context: Option<String>,
) -> Result<ChapterMission, String> {
    let logger = Logger::new().with_feature("chapter_mission");
    log_command_start(&logger, "generate_chapter_mission_with_ai", &format!("章节ID: {}, 章节号: {}", chapter_id, chapter_number));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let ai_service = AIService::new();

    let system_prompt = r#"你是一位专业的章节导演（Chapter Director）。你的任务是根据章节大纲和项目蓝图，为每一章生成导演脚本（Chapter Mission）。

## 导演脚本结构
导演脚本包含以下元素：
1. **宏观节拍** - 本章的主要事件/目标
2. **微观节拍** - 将宏观节拍分解为3-5个具体场景或情节点
3. **视角（POV）** - 指定本章的叙事视角角色
4. **基调（Tone）** - 本章的情感基调（紧张/温馨/悬疑/欢快等）
5. **节奏（Pacing）** - 本章的节奏类型（慢/中/快）
6. **允许新登场角色** - 本章可以引入的新角色列表
7. **禁止角色** - 本章不允许出现的角色列表

## 设计原则
1. **每章一个节拍** - 确保每个宏观节拍对应一章
2. **视角一致性** - 尽量保持视角的一致性
3. **节奏变化** - 根据情节需要调整节奏
4. **角色出场控制** - 合理安排角色出场时机
5. **信息可见性** - 根据章节序号控制角色和设定的可见性

## 输入信息
{
  "chapter_number": 章节号,
  "chapter_outline": 章节大纲,
  "blueprint_context": 项目蓝图上下文
}

## 输出格式
返回JSON格式的导演脚本：
{
  "macro_beat": "宏观节拍描述",
  "micro_beats": ["微观节拍1", "微观节拍2", "微观节拍3"],
  "pov": "视角角色名",
  "tone": "基调",
  "pacing": "节奏",
  "allowed_new_characters": ["新角色名1", "新角色名2"],
  "forbidden_characters": ["禁止角色名1"]
}

## 注意事项
1. 宏观节拍要简洁明了，一句话概括本章核心
2. 微观节拍要具体，能指导AI写作
3. 视角角色应该是本章的主要行动者
4. 基调和节奏要服务于情节需要
5. 允许新登场角色列表只列出确实需要在本章登场的新角色
6. 禁止角色列表是为了防止信息泄露，只列出确实不能出现的角色

请只返回JSON格式的结果，不要包含其他文字。"#;

    let user_input = serde_json::json!({
        "chapter_number": chapter_number,
        "chapter_outline": chapter_outline.unwrap_or_else(|| "无大纲".to_string()),
        "blueprint_context": blueprint_context.unwrap_or_else(|| "无蓝图上下文".to_string()),
    });

    let ai_request = AICompletionRequest {
        model_id: "default".to_string(),
        context: system_prompt.to_string(),
        instruction: user_input.to_string(),
        temperature: Some(0.7),
        max_tokens: Some(2000),
        stream: Some(false),
        character_context: None,
        worldview_context: None,
        project_id: None,
        chapter_mission_id: None,
    };

    let ai_response = ai_service.continue_novel(ai_request, None).await.map_err(|e| {
        logger.error(&format!("AI mission generation failed: {}", e));
        format!("AI导演脚本生成失败: {}", e)
    })?;

    let response_text = ai_response.trim();
    let (macro_beat, micro_beats, pov, tone, pacing, allowed_new, forbidden) = if response_text.contains("{") && response_text.contains("}") {
        let start_idx = response_text.find('{').unwrap_or(0);
        let end_idx = response_text.rfind('}').unwrap_or(response_text.len());
        let json_str = &response_text[start_idx..=end_idx];

        match serde_json::from_str::<serde_json::Value>(json_str) {
            Ok(parsed) => {
                let mb = parsed.get("macro_beat").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let mbs: Vec<String> = parsed.get("micro_beats")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default();
                let p = parsed.get("pov").and_then(|v| v.as_str()).map(|s| s.to_string());
                let t = parsed.get("tone").and_then(|v| v.as_str()).map(|s| s.to_string());
                let pac = parsed.get("pacing").and_then(|v| v.as_str()).map(|s| s.to_string());
                let anc: Vec<String> = parsed.get("allowed_new_characters")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default();
                let forb: Vec<String> = parsed.get("forbidden_characters")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                    .unwrap_or_default();
                (mb, mbs, p, t, pac, anc, forb)
            }
            Err(_) => {
                logger.warn("Failed to parse JSON from AI response, using defaults");
                (String::new(), vec![], None, None, None, vec![], vec![])
            }
        }
    } else {
        (String::new(), vec![], None, None, None, vec![], vec![])
    };

    let mission_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let micro_beats_json = serde_json::to_string(&micro_beats).unwrap_or_default();
    let allowed_new_json = serde_json::to_string(&allowed_new).unwrap_or_default();
    let forbidden_json = serde_json::to_string(&forbidden).unwrap_or_default();

    conn.execute(
        "INSERT OR REPLACE INTO chapter_missions (id, chapter_id, chapter_number, macro_beat, micro_beats, pov, tone, pacing, allowed_new_characters, forbidden_characters, beat_id, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            &mission_id,
            &chapter_id,
            &chapter_number,
            &macro_beat,
            &micro_beats_json,
            &pov,
            &tone,
            &pacing,
            &allowed_new_json,
            &forbidden_json,
            None::<String>,
            &now,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to insert chapter mission: {}", e));
        format!("插入章节导演脚本失败: {}", e)
    })?;

    let mission = ChapterMission {
        id: mission_id,
        chapter_id,
        chapter_number,
        macro_beat,
        micro_beats,
        pov,
        tone,
        pacing,
        allowed_new_characters: allowed_new,
        forbidden_characters: forbidden,
        beat_id: None,
        created_at: now,
    };

    log_command_success(&logger, "generate_chapter_mission_with_ai", "生成完成");
    Ok(mission)
}

#[tauri::command]
pub async fn get_story_beats(
    app: tauri::AppHandle,
    project_id: String,
) -> Result<Vec<StoryBeat>, String> {
    let logger = Logger::new().with_feature("get_story_beats");
    log_command_start(&logger, "get_story_beats", &format!("project_id={}", project_id));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let mut stmt = conn.prepare(
        "SELECT id, parent_id, title, content, node_type, sort_order
         FROM outline_nodes
         WHERE project_id = ?1
         AND node_type IN ('chapter', 'scene', 'beat')
         ORDER BY sort_order ASC"
    ).map_err(|e| {
        logger.error(&format!("Failed to prepare query: {}", e));
        format!("准备查询失败: {}", e)
    })?;

    let mut rows = stmt.query(params![&project_id]).map_err(|e| {
        logger.error(&format!("Failed to query outline nodes: {}", e));
        format!("查询大纲节点失败: {}", e)
    })?;

    let mut beats = Vec::new();
    let mut chapter_number = 0;

    while let Some(row) = rows.next().map_err(|e| {
        logger.error(&format!("Failed to iterate rows: {}", e));
        format!("迭代查询结果失败: {}", e)
    })? {
        let node_id: String = row.get(0).map_err(|e| e.to_string())?;
        let parent_id: Option<String> = row.get(1).ok();
        let title: String = row.get(2).map_err(|e| e.to_string())?;
        let content: String = row.get(3).map_err(|e| e.to_string())?;
        let node_type: String = row.get(4).map_err(|e| e.to_string())?;
        let sort_order: i32 = row.get(5).map_err(|e| e.to_string())?;

        let beat_type = if node_type == "chapter" {
            chapter_number += 1;
            "chapter".to_string()
        } else if node_type == "scene" {
            "scene".to_string()
        } else {
            "beat".to_string()
        };

        beats.push(StoryBeat {
            id: format!("beat_{}", Uuid::new_v4().to_string()),
            outline_node_id: node_id,
            title,
            description: content,
            chapter_number,
            beat_type,
            status: "pending".to_string(),
        });
    }

    log_command_success(&logger, "get_story_beats", &format!("获取{}个节拍", beats.len()));
    Ok(beats)
}

#[tauri::command]
pub async fn create_chapter_guardrails(
    app: tauri::AppHandle,
    request: CreateChapterGuardrailsRequest,
) -> Result<ChapterGuardrails, String> {
    let logger = Logger::new().with_feature("create_chapter_guardrails");
    log_command_start(&logger, "create_chapter_guardrails", &format!("{:?}", request));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let id = format!("guardrails_{}", Uuid::new_v4().to_string());
    let now = Utc::now().to_rfc3339();

    let forbidden_characters = request.forbidden_characters.clone().unwrap_or_default();
    let forbidden_topics = request.forbidden_topics.clone().unwrap_or_default();
    let forbidden_emojis = request.forbidden_emojis.clone().unwrap_or_default();

    let forbidden_chars_json = serde_json::to_string(&forbidden_characters).unwrap_or_default();
    let forbidden_topics_json = serde_json::to_string(&forbidden_topics).unwrap_or_default();
    let forbidden_emojis_json = serde_json::to_string(&forbidden_emojis).unwrap_or_default();

    conn.execute(
        "INSERT INTO chapter_guardrails (id, chapter_id, chapter_number, forbidden_characters, forbidden_topics, forbidden_emojis, min_length, max_length, required_beat_completion, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            &id,
            &request.chapter_id,
            &request.chapter_number,
            &forbidden_chars_json,
            &forbidden_topics_json,
            &forbidden_emojis_json,
            request.min_length.unwrap_or(0),
            request.max_length.unwrap_or(100000),
            if request.required_beat_completion.unwrap_or(true) { 1 } else { 0 },
            &now,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to insert chapter guardrails: {}", e));
        format!("插入章节护栏失败: {}", e)
    })?;

    let guardrails = ChapterGuardrails {
        id: id.clone(),
        chapter_id: request.chapter_id,
        chapter_number: request.chapter_number,
        forbidden_characters,
        forbidden_topics,
        forbidden_emojis,
        min_length: request.min_length.unwrap_or(0),
        max_length: request.max_length.unwrap_or(100000),
        required_beat_completion: request.required_beat_completion.unwrap_or(true),
        created_at: now,
    };

    log_command_success(&logger, "create_chapter_guardrails", &id);
    Ok(guardrails)
}

#[tauri::command]
pub async fn get_chapter_guardrails(
    app: tauri::AppHandle,
    chapter_id: String,
) -> Result<Option<ChapterGuardrails>, String> {
    let logger = Logger::new().with_feature("get_chapter_guardrails");
    log_command_start(&logger, "get_chapter_guardrails", &format!("chapter_id={}", chapter_id));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let result = conn.query_row(
        "SELECT id, chapter_id, chapter_number, forbidden_characters, forbidden_topics, forbidden_emojis, min_length, max_length, required_beat_completion, created_at
            FROM chapter_guardrails WHERE chapter_id = ?1",
        params![&chapter_id],
        |row| {
            let forbidden_chars_json: String = row.get(3).unwrap_or_default();
            let forbidden_topics_json: String = row.get(4).unwrap_or_default();
            let forbidden_emojis_json: String = row.get(5).unwrap_or_default();

            let forbidden_chars: Vec<String> = serde_json::from_str(&forbidden_chars_json).unwrap_or_default();
            let forbidden_topics: Vec<String> = serde_json::from_str(&forbidden_topics_json).unwrap_or_default();
            let forbidden_emojis: Vec<String> = serde_json::from_str(&forbidden_emojis_json).unwrap_or_default();

            Ok(ChapterGuardrails {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                chapter_number: row.get(2)?,
                forbidden_characters: forbidden_chars,
                forbidden_topics: forbidden_topics,
                forbidden_emojis: forbidden_emojis,
                min_length: row.get(6)?,
                max_length: row.get(7)?,
                required_beat_completion: {
                    let val: i32 = row.get(8)?;
                    val != 0
                },
                created_at: row.get(9)?,
            })
        },
    );

    match result {
        Ok(guardrails) => {
            log_command_success(&logger, "get_chapter_guardrails", &guardrails.id);
            Ok(Some(guardrails))
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            log_command_success(&logger, "get_chapter_guardrails", "未找到护栏");
            Ok(None)
        }
        Err(e) => {
            logger.error(&format!("Failed to query guardrails: {}", e));
            Err(format!("查询章节护栏失败: {}", e))
        }
    }
}

#[tauri::command]
pub async fn update_chapter_guardrails(
    app: tauri::AppHandle,
    request: UpdateChapterGuardrailsRequest,
) -> Result<ChapterGuardrails, String> {
    let logger = Logger::new().with_feature("update_chapter_guardrails");
    log_command_start(&logger, "update_chapter_guardrails", &format!("{:?}", request));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let existing = conn.query_row(
        "SELECT id, chapter_id, chapter_number, forbidden_characters, forbidden_topics, forbidden_emojis, min_length, max_length, required_beat_completion, created_at
            FROM chapter_guardrails WHERE id = ?1",
        params![&request.guardrails_id],
        |row| {
            let forbidden_chars_json: String = row.get(3).unwrap_or_default();
            let forbidden_topics_json: String = row.get(4).unwrap_or_default();
            let forbidden_emojis_json: String = row.get(5).unwrap_or_default();

            let forbidden_chars: Vec<String> = serde_json::from_str(&forbidden_chars_json).unwrap_or_default();
            let forbidden_topics: Vec<String> = serde_json::from_str(&forbidden_topics_json).unwrap_or_default();
            let forbidden_emojis: Vec<String> = serde_json::from_str(&forbidden_emojis_json).unwrap_or_default();

            Ok(ChapterGuardrails {
                id: row.get(0)?,
                chapter_id: row.get(1)?,
                chapter_number: row.get(2)?,
                forbidden_characters: forbidden_chars,
                forbidden_topics: forbidden_topics,
                forbidden_emojis: forbidden_emojis,
                min_length: row.get(6)?,
                max_length: row.get(7)?,
                required_beat_completion: {
                    let val: i32 = row.get(8)?;
                    val != 0
                },
                created_at: row.get(9)?,
            })
        },
    );

    let mut guardrails = match existing {
        Ok(g) => g,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err("未找到指定的护栏记录".to_string());
        }
        Err(e) => {
            logger.error(&format!("Failed to query guardrails: {}", e));
            return Err(format!("查询章节护栏失败: {}", e));
        }
    };

    if let Some(forbidden_chars) = request.forbidden_characters {
        guardrails.forbidden_characters = forbidden_chars;
    }
    if let Some(forbidden_topics) = request.forbidden_topics {
        guardrails.forbidden_topics = forbidden_topics;
    }
    if let Some(forbidden_emojis) = request.forbidden_emojis {
        guardrails.forbidden_emojis = forbidden_emojis;
    }
    if let Some(min_length) = request.min_length {
        guardrails.min_length = min_length;
    }
    if let Some(max_length) = request.max_length {
        guardrails.max_length = max_length;
    }
    if let Some(required) = request.required_beat_completion {
        guardrails.required_beat_completion = required;
    }

    let forbidden_chars_json = serde_json::to_string(&guardrails.forbidden_characters).unwrap_or_default();
    let forbidden_topics_json = serde_json::to_string(&guardrails.forbidden_topics).unwrap_or_default();
    let forbidden_emojis_json = serde_json::to_string(&guardrails.forbidden_emojis).unwrap_or_default();

    conn.execute(
        "UPDATE chapter_guardrails SET forbidden_characters = ?1, forbidden_topics = ?2, forbidden_emojis = ?3, min_length = ?4, max_length = ?5, required_beat_completion = ?6
            WHERE id = ?7",
        params![
            &forbidden_chars_json,
            &forbidden_topics_json,
            &forbidden_emojis_json,
            guardrails.min_length,
            guardrails.max_length,
            if guardrails.required_beat_completion { 1 } else { 0 },
            &request.guardrails_id,
        ],
    ).map_err(|e| {
        logger.error(&format!("Failed to update chapter guardrails: {}", e));
        format!("更新章节护栏失败: {}", e)
    })?;

    log_command_success(&logger, "update_chapter_guardrails", &request.guardrails_id);
    Ok(guardrails)
}

#[tauri::command]
pub async fn check_content_against_guardrails(
    app: tauri::AppHandle,
    request: CheckContentAgainstGuardrailsRequest,
) -> Result<CheckContentAgainstGuardrailsResponse, String> {
    let logger = Logger::new().with_feature("check_content_against_guardrails");
    log_command_start(&logger, "check_content_against_guardrails", &format!("chapter_id={}", request.chapter_id));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let guardrails = conn.query_row(
        "SELECT forbidden_characters, forbidden_topics, forbidden_emojis, min_length, max_length
            FROM chapter_guardrails WHERE chapter_id = ?1",
        params![&request.chapter_id],
        |row| {
            let forbidden_chars_json: String = row.get(0).unwrap_or_default();
            let forbidden_topics_json: String = row.get(1).unwrap_or_default();
            let forbidden_emojis_json: String = row.get(2).unwrap_or_default();

            let forbidden_chars: Vec<String> = serde_json::from_str(&forbidden_chars_json).unwrap_or_default();
            let forbidden_topics: Vec<String> = serde_json::from_str(&forbidden_topics_json).unwrap_or_default();
            let forbidden_emojis: Vec<String> = serde_json::from_str(&forbidden_emojis_json).unwrap_or_default();

            Ok((forbidden_chars, forbidden_topics, forbidden_emojis, row.get(3)?, row.get(4)?))
        },
    );

    let (forbidden_chars, forbidden_topics, forbidden_emojis, min_length, max_length) = match guardrails {
        Ok(g) => g,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            (vec![], vec![], vec![], 0, 100000)
        }
        Err(e) => {
            logger.error(&format!("Failed to query guardrails: {}", e));
            return Err(format!("查询章节护栏失败: {}", e));
        }
    };

    let mut violations = Vec::new();
    let mut suggestions = Vec::new();
    let content_length = request.content.chars().count() as i32;

    if content_length < min_length {
        violations.push(GuardrailViolation {
            violation_type: "length".to_string(),
            message: format!("内容长度{}字低于最小要求{}字", content_length, min_length),
            severity: "error".to_string(),
        });
        suggestions.push(format!("请扩展内容至至少{}字", min_length));
    }

    if content_length > max_length {
        violations.push(GuardrailViolation {
            violation_type: "length".to_string(),
            message: format!("内容长度{}字超过最大限制{}字", content_length, max_length),
            severity: "warning".to_string(),
        });
        suggestions.push(format!("建议缩减内容至{}字以内", max_length));
    }

    for forbidden_char in &forbidden_chars {
        if request.content.contains(forbidden_char) {
            violations.push(GuardrailViolation {
                violation_type: "forbidden_character".to_string(),
                message: format!("内容中包含禁止角色：{}", forbidden_char),
                severity: "error".to_string(),
            });
            suggestions.push(format!("请移除角色\"{}\"的相关内容", forbidden_char));
        }
    }

    for forbidden_topic in &forbidden_topics {
        if request.content.contains(forbidden_topic) {
            violations.push(GuardrailViolation {
                violation_type: "forbidden_topic".to_string(),
                message: format!("内容中涉及禁止话题：{}", forbidden_topic),
                severity: "warning".to_string(),
            });
            suggestions.push(format!("请移除话题\"{}\"的相关内容", forbidden_topic));
        }
    }

    for forbidden_emoji in &forbidden_emojis {
        if request.content.contains(forbidden_emoji) {
            violations.push(GuardrailViolation {
                violation_type: "forbidden_emoji".to_string(),
                message: format!("内容中包含禁止表情符号：{}", forbidden_emoji),
                severity: "warning".to_string(),
            });
            suggestions.push(format!("请移除表情符号\"{}\"", forbidden_emoji));
        }
    }

    let passed = violations.is_empty() || violations.iter().all(|v| v.severity != "error");

    log_command_success(&logger, "check_content_against_guardrails", &format!("passed={}, violations={}", passed, violations.len()));
    Ok(CheckContentAgainstGuardrailsResponse {
        passed,
        violations,
        suggestions,
    })
}

#[tauri::command]
pub async fn vectorize_chapter(
    app: tauri::AppHandle,
    request: VectorizeChapterRequest,
) -> Result<VectorizeChapterResponse, String> {
    let logger = Logger::new().with_feature("vectorize_chapter");
    log_command_start(&logger, "vectorize_chapter", &format!("chapter_id={}", request.chapter_id));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let chapter: String = conn.query_row(
        "SELECT content FROM chapters WHERE id = ?1",
        params![&request.chapter_id],
        |row| row.get(0),
    ).map_err(|e| {
        logger.error(&format!("Failed to query chapter: {}", e));
        format!("查询章节失败: {}", e)
    })?;

    let chunk_size = request.chunk_size.unwrap_or(500) as usize;
    let overlap = request.overlap.unwrap_or(50) as usize;
    let chars: Vec<char> = chapter.chars().collect();
    let now = Utc::now().to_rfc3339();

    let mut chunks_created = 0;
    let mut start = 0;

    while start < chars.len() {
        let end = std::cmp::min(start + chunk_size, chars.len());
        let chunk_text: String = chars[start..end].iter().collect();

        if !chunk_text.trim().is_empty() {
            let chunk_id = format!("chunk_{}", Uuid::new_v4().to_string());
            let metadata = serde_json::json!({
                "start_pos": start,
                "end_pos": end,
                "chunk_size": chunk_size,
                "overlap": overlap,
            }).to_string();

            conn.execute(
                "INSERT INTO vector_chunks (id, chapter_id, chunk_index, content, metadata, created_at)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    &chunk_id,
                    &request.chapter_id,
                    chunks_created,
                    &chunk_text,
                    &metadata,
                    &now,
                ],
            ).map_err(|e| {
                logger.error(&format!("Failed to insert chunk: {}", e));
                format!("插入向量块失败: {}", e)
            })?;

            chunks_created += 1;
        }

        start += chunk_size - overlap;
    }

    log_command_success(&logger, "vectorize_chapter", &format!("创建{}个块", chunks_created));
    Ok(VectorizeChapterResponse {
        chunks_created,
        chapter_id: request.chapter_id,
    })
}

#[tauri::command]
pub async fn search_chunks(
    app: tauri::AppHandle,
    request: SearchChunksRequest,
) -> Result<SearchChunksResponse, String> {
    let logger = Logger::new().with_feature("search_chunks");
    log_command_start(&logger, "search_chunks", &format!("query={}", request.query));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let mut stmt = conn.prepare(
        "SELECT id, chapter_id, chunk_index, content, metadata, created_at
         FROM vector_chunks
         WHERE content LIKE ?1
         ORDER BY chunk_index ASC
         LIMIT ?2"
    ).map_err(|e| {
        logger.error(&format!("Failed to prepare query: {}", e));
        format!("准备查询失败: {}", e)
    })?;

    let top_k = request.top_k.unwrap_or(5) as usize;
    let search_pattern = format!("%{}%", request.query);
    let mut rows = stmt.query(params![&search_pattern, top_k]).map_err(|e| {
        logger.error(&format!("Failed to search chunks: {}", e));
        format!("搜索向量块失败: {}", e)
    })?;

    let mut results = Vec::new();
    let query_lower = request.query.to_lowercase();

    while let Some(row) = rows.next().map_err(|e| {
        logger.error(&format!("Failed to iterate rows: {}", e));
        format!("迭代查询结果失败: {}", e)
    })? {
        let chunk = VectorChunk {
            id: row.get(0).map_err(|e| e.to_string())?,
            chapter_id: row.get(1).map_err(|e| e.to_string())?,
            chunk_index: row.get(2).map_err(|e| e.to_string())?,
            content: row.get(3).map_err(|e| e.to_string())?,
            metadata: row.get(4).map_err(|e| e.to_string())?,
            created_at: row.get(5).map_err(|e| e.to_string())?,
        };

        let chunk_lower = chunk.content.to_lowercase();
        let similarity = calculate_similarity(&query_lower, &chunk_lower);

        results.push(ChunkSearchResult {
            chunk,
            similarity,
        });
    }

    results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

    let results: Vec<ChunkSearchResult> = results.into_iter().take(top_k).collect();

    log_command_success(&logger, "search_chunks", &format!("找到{}个结果", results.len()));
    Ok(SearchChunksResponse {
        results,
        query: request.query,
    })
}

fn calculate_similarity(query: &str, chunk: &str) -> f64 {
    let query_words: Vec<&str> = query.split_whitespace().collect();
    let chunk_words: Vec<&str> = chunk.split_whitespace().collect();

    if query_words.is_empty() {
        return 0.0;
    }

    let mut matches = 0;
    for qword in &query_words {
        for cword in &chunk_words {
            if cword.contains(qword) || qword.contains(cword) {
                matches += 1;
                break;
            }
        }
    }

    matches as f64 / query_words.len() as f64
}

#[tauri::command]
pub async fn generate_chapter_summary(
    app: tauri::AppHandle,
    chapter_id: String,
) -> Result<String, String> {
    let logger = Logger::new().with_feature("generate_chapter_summary");
    log_command_start(&logger, "generate_chapter_summary", &format!("chapter_id={}", chapter_id));

    let db_path = get_db_path(&app).map_err(|e| {
        logger.error(&format!("Failed to get database path: {}", e));
        format!("获取数据库路径失败: {}", e)
    })?;

    let conn = get_connection(&db_path).map_err(|e| {
        logger.error(&format!("Failed to get database connection: {}", e));
        format!("数据库连接失败: {}", e)
    })?;

    let chapter: String = conn.query_row(
        "SELECT content FROM chapters WHERE id = ?1",
        params![&chapter_id],
        |row| row.get(0),
    ).map_err(|e| {
        logger.error(&format!("Failed to query chapter: {}", e));
        format!("查询章节失败: {}", e)
    })?;

    if chapter.trim().is_empty() {
        log_command_success(&logger, "generate_chapter_summary", "章节内容为空");
        return Ok("章节内容为空".to_string());
    }

    let ai_service = AIService::new();

    let system_prompt = "你是一个专业的小说编辑。请为以下章节内容生成一个简洁的摘要（200字以内），突出本章的主要事件和情节发展。".to_string();

    let response = ai_service.complete("default", &system_prompt, &chapter).await.map_err(|e| {
        logger.error(&format!("AI生成摘要失败: {}", e));
        format!("AI生成摘要失败: {}", e)
    })?;

    let summary = response.trim().to_string();

    conn.execute(
        "UPDATE chapters SET summary = ?1 WHERE id = ?2",
        params![&summary, &chapter_id],
    ).map_err(|e| {
        logger.error(&format!("Failed to update chapter summary: {}", e));
        format!("更新章节摘要失败: {}", e)
    })?;

    log_command_success(&logger, "generate_chapter_summary", &format!("摘要生成完成，长度：{}", summary.len()));
    Ok(summary)
}
