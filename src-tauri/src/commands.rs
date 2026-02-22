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
    };

    conn.execute(
        "INSERT INTO chapters (id, project_id, title, content, word_count, sort_order, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        .prepare("SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at FROM chapters WHERE project_id = ? ORDER BY sort_order ASC")
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
        .prepare("SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at FROM chapters WHERE id = ?")
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
        .prepare("SELECT id, project_id, title, content, word_count, sort_order, status, created_at, updated_at FROM chapters WHERE id = ?")
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
    log_command_start(&logger, "ai_continue_novel", &format!("model={}", request.model_id));

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

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

    // 设置默认值
    if request.character_context.is_none() {
        request.character_context = Some("暂无角色信息".to_string());
    }
    if request.worldview_context.is_none() {
        request.worldview_context = Some("暂无世界观设定".to_string());
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
            &now,
            &now,
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
