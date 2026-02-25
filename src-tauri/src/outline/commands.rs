use crate::database::get_connection;
use crate::logger::{Logger, log_command_start, log_command_success, log_command_error};
use crate::outline::types::*;
use crate::ai::AIService;
use serde_json;
use tauri::{AppHandle, Manager};
use rusqlite::params;
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;

fn get_db_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    if cfg!(debug_assertions) {
        let mut project_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;
        project_dir.push("novel_studio_dev.db");
        Ok(std::fs::canonicalize(&project_dir).unwrap_or(project_dir))
    } else {
        let app_data_dir = app.path().app_data_dir()
            .map_err(|e| format!("Failed to get app data directory: {}", e))?;
        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
        Ok(app_data_dir.join("novel_studio.db"))
    }
}

fn init_outline_tables(conn: &rusqlite::Connection) -> Result<(), String> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS outline_nodes (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            parent_id TEXT,
            title TEXT NOT NULL,
            content TEXT,
            node_type TEXT NOT NULL,
            sort_order INTEGER DEFAULT 0,
            status TEXT DEFAULT 'planned',
            word_count_target INTEGER,
            word_count_actual INTEGER DEFAULT 0,
            metadata TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_outline_nodes(app: AppHandle, project_id: String) -> Result<Vec<OutlineNode>, String> {
    let logger = Logger::new().with_feature("outline");
    log_command_start(&logger, "get_outline_nodes", &project_id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
    
    init_outline_tables(&conn)?;

    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_id, title, content, node_type, sort_order, 
                status, word_count_target, word_count_actual, metadata, created_at, updated_at
         FROM outline_nodes WHERE project_id = ?1 ORDER BY sort_order"
    ).map_err(|e| e.to_string())?;

    let nodes = stmt.query_map(params![&project_id], |row| {
        Ok(OutlineNode {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_id: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            node_type: match row.get::<_, String>(5)?.as_str() {
                "arc" => OutlineNodeType::Arc,
                "chapter" => OutlineNodeType::Chapter,
                "scene" => OutlineNodeType::Scene,
                "beat" => OutlineNodeType::Beat,
                _ => OutlineNodeType::Scene,
            },
            sort_order: row.get(6)?,
            status: match row.get::<_, String>(7)?.as_str() {
                "planned" => OutlineNodeStatus::Planned,
                "inprogress" => OutlineNodeStatus::InProgress,
                "completed" => OutlineNodeStatus::Completed,
                "skipped" => OutlineNodeStatus::Skipped,
                _ => OutlineNodeStatus::Planned,
            },
            word_count_target: row.get(8)?,
            word_count_actual: row.get(9)?,
            metadata: row.get(10)?,
            created_at: row.get::<_, String>(11)?.parse().unwrap_or_else(|_| Utc::now()),
            updated_at: row.get::<_, String>(12)?.parse().unwrap_or_else(|_| Utc::now()),
        })
    }).map_err(|e| e.to_string())?;

    let result: Vec<OutlineNode> = nodes.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    log_command_success(&logger, "get_outline_nodes", &format!("{} nodes", result.len()));
    Ok(result)
}

#[tauri::command]
pub async fn create_outline_node(app: AppHandle, request: CreateOutlineNodeRequest) -> Result<OutlineNode, String> {
    let logger = Logger::new().with_feature("outline");
    log_command_start(&logger, "create_outline_node", &request.title);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
    
    init_outline_tables(&conn)?;

    let id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let node_type_str = match request.node_type {
        OutlineNodeType::Arc => "arc",
        OutlineNodeType::Chapter => "chapter",
        OutlineNodeType::Scene => "scene",
        OutlineNodeType::Beat => "beat",
    };

    conn.execute(
        "INSERT INTO outline_nodes (id, project_id, parent_id, title, content, node_type, sort_order, status, word_count_target, word_count_actual, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'planned', ?8, 0, ?9, ?10)",
        params![
            &id,
            &request.project_id,
            &request.parent_id,
            &request.title,
            &request.content,
            node_type_str,
            request.sort_order.unwrap_or(0),
            request.word_count_target,
            now.to_rfc3339(),
            now.to_rfc3339()
        ],
    ).map_err(|e| e.to_string())?;

    log_command_success(&logger, "create_outline_node", &request.title);
    
    Ok(OutlineNode {
        id,
        project_id: request.project_id,
        parent_id: request.parent_id,
        title: request.title,
        content: request.content.unwrap_or_default(),
        node_type: request.node_type,
        sort_order: request.sort_order.unwrap_or(0),
        status: OutlineNodeStatus::Planned,
        word_count_target: request.word_count_target,
        word_count_actual: 0,
        metadata: None,
        created_at: now,
        updated_at: now,
    })
}

#[tauri::command]
pub async fn update_outline_node(app: AppHandle, request: UpdateOutlineNodeRequest) -> Result<OutlineNode, String> {
    let logger = Logger::new().with_feature("outline");
    log_command_start(&logger, "update_outline_node", &request.id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let now = Utc::now();
    let status_str = request.status.as_ref().map(|s| match s {
        OutlineNodeStatus::Planned => "planned",
        OutlineNodeStatus::InProgress => "inprogress",
        OutlineNodeStatus::Completed => "completed",
        OutlineNodeStatus::Skipped => "skipped",
    });

    conn.execute(
        "UPDATE outline_nodes SET title = COALESCE(?1, title), content = COALESCE(?2, content), 
         status = COALESCE(?3, status), sort_order = COALESCE(?4, sort_order), 
         word_count_target = COALESCE(?5, word_count_target), updated_at = ?6 WHERE id = ?7",
        params![
            request.title,
            request.content,
            status_str,
            request.sort_order,
            request.word_count_target,
            now.to_rfc3339(),
            &request.id
        ],
    ).map_err(|e| e.to_string())?;

    log_command_success(&logger, "update_outline_node", &request.id);
    
    let node = get_outline_node_by_id(&app, &request.id).await?;
    Ok(node)
}

async fn get_outline_node_by_id(app: &AppHandle, id: &str) -> Result<OutlineNode, String> {
    let db_path = get_db_path(app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT id, project_id, parent_id, title, content, node_type, sort_order, status, word_count_target, word_count_actual, metadata, created_at, updated_at FROM outline_nodes WHERE id = ?1",
        params![id],
        |row| {
            Ok(OutlineNode {
                id: row.get(0)?,
                project_id: row.get(1)?,
                parent_id: row.get(2)?,
                title: row.get(3)?,
                content: row.get(4)?,
                node_type: match row.get::<_, String>(5)?.as_str() {
                    "arc" => OutlineNodeType::Arc,
                    "chapter" => OutlineNodeType::Chapter,
                    "scene" => OutlineNodeType::Scene,
                    "beat" => OutlineNodeType::Beat,
                    _ => OutlineNodeType::Scene,
                },
                sort_order: row.get(6)?,
                status: match row.get::<_, String>(7)?.as_str() {
                    "planned" => OutlineNodeStatus::Planned,
                    "inprogress" => OutlineNodeStatus::InProgress,
                    "completed" => OutlineNodeStatus::Completed,
                    "skipped" => OutlineNodeStatus::Skipped,
                    _ => OutlineNodeStatus::Planned,
                },
                word_count_target: row.get(8)?,
                word_count_actual: row.get(9)?,
                metadata: row.get(10)?,
                created_at: row.get::<_, String>(11)?.parse().unwrap_or_else(|_| Utc::now()),
                updated_at: row.get::<_, String>(12)?.parse().unwrap_or_else(|_| Utc::now()),
            })
        }
    ).map_err(|e| format!("Node not found: {}", e))
}

#[tauri::command]
pub async fn delete_outline_node(app: AppHandle, id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("outline");
    log_command_start(&logger, "delete_outline_node", &id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM outline_nodes WHERE id = ?1 OR parent_id = ?1",
        params![&id],
    ).map_err(|e| e.to_string())?;

    log_command_success(&logger, "delete_outline_node", &id);
    Ok(())
}

#[tauri::command]
pub async fn get_outline_templates() -> Result<Vec<OutlineTemplate>, String> {
    Ok(get_default_templates())
}

#[tauri::command]
pub async fn apply_outline_template(app: AppHandle, project_id: String, template_id: String) -> Result<Vec<OutlineNode>, String> {
    let logger = Logger::new().with_feature("outline");
    log_command_start(&logger, "apply_outline_template", &template_id);

    let templates = get_default_templates();
    let template = templates.iter().find(|t| t.id == template_id)
        .ok_or_else(|| "Template not found".to_string())?;

    let mut created_nodes = Vec::new();
    let mut sort_order = 0;

    fn create_nodes_from_template(
        app: &AppHandle,
        project_id: &str,
        parent_id: Option<&str>,
        nodes: &[TemplateNode],
        sort_order: &mut i32,
        created_nodes: &mut Vec<OutlineNode>,
    ) -> Result<(), String> {
        for node in nodes {
            let db_path = get_db_path(app)?;
            let conn = get_connection(&db_path).map_err(|e| e.to_string())?;
            
            let id = Uuid::new_v4().to_string();
            let now = Utc::now();
            let node_type_str = match node.node_type {
                OutlineNodeType::Arc => "arc",
                OutlineNodeType::Chapter => "chapter",
                OutlineNodeType::Scene => "scene",
                OutlineNodeType::Beat => "beat",
            };

            conn.execute(
                "INSERT INTO outline_nodes (id, project_id, parent_id, title, content, node_type, sort_order, status, word_count_target, word_count_actual, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'planned', NULL, 0, ?8, ?9)",
                params![
                    &id,
                    project_id,
                    parent_id,
                    &node.title,
                    &node.description,
                    node_type_str,
                    *sort_order,
                    now.to_rfc3339(),
                    now.to_rfc3339()
                ],
            ).map_err(|e| e.to_string())?;

            created_nodes.push(OutlineNode {
                id: id.clone(),
                project_id: project_id.to_string(),
                parent_id: parent_id.map(String::from),
                title: node.title.clone(),
                content: node.description.clone(),
                node_type: node.node_type.clone(),
                sort_order: *sort_order,
                status: OutlineNodeStatus::Planned,
                word_count_target: None,
                word_count_actual: 0,
                metadata: None,
                created_at: now,
                updated_at: now,
            });

            *sort_order += 1;

            if !node.children.is_empty() {
                create_nodes_from_template(app, project_id, Some(&id), &node.children, sort_order, created_nodes)?;
            }
        }
        Ok(())
    }

    create_nodes_from_template(&app, &project_id, None, &template.structure, &mut sort_order, &mut created_nodes)?;

    log_command_success(&logger, "apply_outline_template", &format!("{} nodes created", created_nodes.len()));
    Ok(created_nodes)
}

#[tauri::command]
pub async fn generate_outline_with_ai(
    app: AppHandle,
    ai_service: tauri::State<'_, Arc<RwLock<AIService>>>,
    request: GenerateOutlineRequest,
) -> Result<OutlineGenerationResult, String> {
    let logger = Logger::new().with_feature("outline");
    log_command_start(&logger, "generate_outline_with_ai", &request.project_id);

    let service = ai_service.read().await;
    
    let model_id = "glm-4-flash";
    let system_prompt = "你是一位专业的小说大纲设计师，擅长创建引人入胜的故事结构。请按照指定的JSON格式输出大纲，不要包含任何其他内容。";
    
    let prompt = format!(
        r#"请为以下小说项目生成一个详细的故事大纲。

类型：{}
主题：{}
主要角色：{}
目标章节数：{}
每章目标字数：{}
风格要求：{}

请按照以下JSON格式输出，不要包含其他内容：
{{
  "arcs": [
    {{
      "title": "故事弧名称",
      "description": "故事弧描述",
      "chapters": [
        {{
          "title": "章节标题",
          "summary": "章节概要",
          "key_events": ["关键事件1", "关键事件2"],
          "estimated_words": 3000
        }}
      ]
    }}
  ],
  "total_chapters": 20,
  "estimated_words": 60000
}}"#,
        request.genre,
        request.theme.unwrap_or_else(|| "未指定".to_string()),
        request.main_characters.join("、"),
        request.target_chapters,
        request.target_words_per_chapter,
        request.style.unwrap_or_else(|| "无特殊要求".to_string())
    );

    let result = service.complete(model_id, system_prompt, &prompt).await
        .map_err(|e| format!("AI generation failed: {}", e))?;
    
    let json_str = result.trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    
    let outline: OutlineGenerationResult = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse AI response: {} - Response: {}", e, json_str))?;

    log_command_success(&logger, "generate_outline_with_ai", &format!("{} arcs generated", outline.arcs.len()));
    Ok(outline)
}

#[tauri::command]
pub async fn save_generated_outline(app: AppHandle, project_id: String, outline: OutlineGenerationResult) -> Result<Vec<OutlineNode>, String> {
    let logger = Logger::new().with_feature("outline");
    log_command_start(&logger, "save_generated_outline", &project_id);

    let mut created_nodes = Vec::new();
    let mut arc_sort_order = 0;

    for arc in &outline.arcs {
        let arc_node = create_outline_node(app.clone(), CreateOutlineNodeRequest {
            project_id: project_id.clone(),
            parent_id: None,
            title: arc.title.clone(),
            content: Some(arc.description.clone()),
            node_type: OutlineNodeType::Arc,
            sort_order: Some(arc_sort_order),
            word_count_target: None,
        }).await?;
        
        arc_sort_order += 1;
        let arc_id = arc_node.id.clone();
        created_nodes.push(arc_node);

        let mut chapter_sort_order = 0;
        for chapter in &arc.chapters {
            let chapter_node = create_outline_node(app.clone(), CreateOutlineNodeRequest {
                project_id: project_id.clone(),
                parent_id: Some(arc_id.clone()),
                title: chapter.title.clone(),
                content: Some(chapter.summary.clone()),
                node_type: OutlineNodeType::Chapter,
                sort_order: Some(chapter_sort_order),
                word_count_target: Some(chapter.estimated_words),
            }).await?;
            
            chapter_sort_order += 1;
            created_nodes.push(chapter_node);
        }
    }

    log_command_success(&logger, "save_generated_outline", &format!("{} nodes saved", created_nodes.len()));
    Ok(created_nodes)
}
