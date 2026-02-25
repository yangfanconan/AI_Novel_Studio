use crate::database::get_connection;
use crate::logger::{Logger, log_command_start, log_command_success, log_command_error};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use rusqlite::params;
use chrono::Utc;
use uuid::Uuid;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplateRecord {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub variables: Vec<String>,
    pub is_default: bool,
    pub is_custom: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePromptTemplateRequest {
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub variables: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePromptTemplateRequest {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub variables: Vec<String>,
}

#[tauri::command]
pub async fn get_custom_prompt_templates(app: AppHandle) -> Result<Vec<PromptTemplateRecord>, String> {
    let logger = Logger::new().with_feature("prompt-templates");
    log_command_start(&logger, "get_custom_prompt_templates", "");

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT id, name, category, description, system_prompt, user_prompt_template, 
                variables, is_default, is_custom, created_at, updated_at 
         FROM prompt_templates ORDER BY category, name"
    ).map_err(|e| e.to_string())?;

    let templates = stmt.query_map([], |row| {
        let variables_str: String = row.get(6)?;
        let variables: Vec<String> = serde_json::from_str(&variables_str).unwrap_or_default();
        
        Ok(PromptTemplateRecord {
            id: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            description: row.get(3)?,
            system_prompt: row.get(4)?,
            user_prompt_template: row.get(5)?,
            variables,
            is_default: row.get::<_, i32>(7)? == 1,
            is_custom: row.get::<_, i32>(8)? == 1,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }).map_err(|e| e.to_string())?;

    let result: Vec<PromptTemplateRecord> = templates.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    log_command_success(&logger, "get_custom_prompt_templates", &format!("{} templates", result.len()));
    Ok(result)
}

#[tauri::command]
pub async fn get_prompt_template_by_id(app: AppHandle, id: String) -> Result<PromptTemplateRecord, String> {
    let logger = Logger::new().with_feature("prompt-templates");
    log_command_start(&logger, "get_prompt_template_by_id", &id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let result = conn.query_row(
        "SELECT id, name, category, description, system_prompt, user_prompt_template, 
                variables, is_default, is_custom, created_at, updated_at 
         FROM prompt_templates WHERE id = ?1",
        params![&id],
        |row| {
            let variables_str: String = row.get(6)?;
            let variables: Vec<String> = serde_json::from_str(&variables_str).unwrap_or_default();
            
            Ok(PromptTemplateRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                system_prompt: row.get(4)?,
                user_prompt_template: row.get(5)?,
                variables,
                is_default: row.get::<_, i32>(7)? == 1,
                is_custom: row.get::<_, i32>(8)? == 1,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        }
    );

    match result {
        Ok(template) => {
            log_command_success(&logger, "get_prompt_template_by_id", &template.name);
            Ok(template)
        }
        Err(e) => {
            log_command_error(&logger, "get_prompt_template_by_id", &e.to_string());
            Err(format!("Template not found: {}", id))
        }
    }
}

#[tauri::command]
pub async fn create_prompt_template(app: AppHandle, request: CreatePromptTemplateRequest) -> Result<PromptTemplateRecord, String> {
    let logger = Logger::new().with_feature("prompt-templates");
    log_command_start(&logger, "create_prompt_template", &request.name);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let variables_json = serde_json::to_string(&request.variables).unwrap_or("[]".to_string());

    conn.execute(
        "INSERT INTO prompt_templates (id, name, category, description, system_prompt, user_prompt_template, variables, is_default, is_custom, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, 1, ?8, ?9)",
        params![
            &id,
            &request.name,
            &request.category,
            &request.description,
            &request.system_prompt,
            &request.user_prompt_template,
            &variables_json,
            &now,
            &now
        ],
    ).map_err(|e| e.to_string())?;

    log_command_success(&logger, "create_prompt_template", &request.name);
    
    Ok(PromptTemplateRecord {
        id,
        name: request.name,
        category: request.category,
        description: request.description,
        system_prompt: request.system_prompt,
        user_prompt_template: request.user_prompt_template,
        variables: request.variables,
        is_default: false,
        is_custom: true,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub async fn update_prompt_template(app: AppHandle, request: UpdatePromptTemplateRequest) -> Result<PromptTemplateRecord, String> {
    let logger = Logger::new().with_feature("prompt-templates");
    log_command_start(&logger, "update_prompt_template", &request.id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();
    let variables_json = serde_json::to_string(&request.variables).unwrap_or("[]".to_string());

    conn.execute(
        "UPDATE prompt_templates SET name = ?1, category = ?2, description = ?3, system_prompt = ?4, user_prompt_template = ?5, variables = ?6, updated_at = ?7 WHERE id = ?8",
        params![
            &request.name,
            &request.category,
            &request.description,
            &request.system_prompt,
            &request.user_prompt_template,
            &variables_json,
            &now,
            &request.id
        ],
    ).map_err(|e| e.to_string())?;

    log_command_success(&logger, "update_prompt_template", &request.name);

    get_prompt_template_by_id(app, request.id).await
}

#[tauri::command]
pub async fn delete_prompt_template(app: AppHandle, id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("prompt-templates");
    log_command_start(&logger, "delete_prompt_template", &id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let is_default: bool = conn.query_row(
        "SELECT is_default FROM prompt_templates WHERE id = ?1",
        params![&id],
        |row| Ok(row.get::<_, i32>(0)? == 1)
    ).unwrap_or(false);

    if is_default {
        return Err("Cannot delete default template".to_string());
    }

    conn.execute(
        "DELETE FROM prompt_templates WHERE id = ?1",
        params![&id],
    ).map_err(|e| e.to_string())?;

    log_command_success(&logger, "delete_prompt_template", &id);
    Ok(())
}

#[tauri::command]
pub async fn reset_prompt_template_to_default(app: AppHandle, id: String) -> Result<(), String> {
    let logger = Logger::new().with_feature("prompt-templates");
    log_command_start(&logger, "reset_prompt_template_to_default", &id);

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let default_prompts = get_default_prompts();
    if let Some(default) = default_prompts.iter().find(|p| p.id == id) {
        let now = Utc::now().to_rfc3339();
        let variables_json = serde_json::to_string(&default.variables).unwrap_or("[]".to_string());
        
        conn.execute(
            "UPDATE prompt_templates SET system_prompt = ?1, user_prompt_template = ?2, variables = ?3, updated_at = ?4 WHERE id = ?5",
            params![
                &default.system_prompt,
                &default.user_prompt_template,
                &variables_json,
                &now,
                &id
            ],
        ).map_err(|e| e.to_string())?;

        log_command_success(&logger, "reset_prompt_template_to_default", &id);
        Ok(())
    } else {
        Err("Default template not found".to_string())
    }
}

#[tauri::command]
pub async fn initialize_default_prompt_templates(app: AppHandle) -> Result<(), String> {
    let logger = Logger::new().with_feature("prompt-templates");
    log_command_start(&logger, "initialize_default_prompt_templates", "");

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path).map_err(|e| e.to_string())?;

    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM prompt_templates WHERE is_default = 1",
        [],
        |row| row.get(0)
    ).unwrap_or(0);

    if count > 0 {
        log_command_success(&logger, "initialize_default_prompt_templates", "already initialized");
        return Ok(());
    }

    let default_prompts = get_default_prompts();
    let now = Utc::now().to_rfc3339();

    for prompt in default_prompts {
        let variables_json = serde_json::to_string(&prompt.variables).unwrap_or("[]".to_string());
        
        conn.execute(
            "INSERT INTO prompt_templates (id, name, category, description, system_prompt, user_prompt_template, variables, is_default, is_custom, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, 0, ?8, ?9)",
            params![
                &prompt.id,
                &prompt.name,
                &prompt.category,
                &prompt.description,
                &prompt.system_prompt,
                &prompt.user_prompt_template,
                &variables_json,
                &now,
                &now
            ],
        ).map_err(|e| e.to_string())?;
    }

    log_command_success(&logger, "initialize_default_prompt_templates", "initialized");
    Ok(())
}

struct DefaultPrompt {
    id: String,
    name: String,
    category: String,
    description: String,
    system_prompt: String,
    user_prompt_template: String,
    variables: Vec<String>,
}

fn get_default_prompts() -> Vec<DefaultPrompt> {
    vec![
        DefaultPrompt {
            id: "novel-continuation".to_string(),
            name: "小说续写".to_string(),
            category: "writing".to_string(),
            description: "根据上下文续写小说内容".to_string(),
            system_prompt: r#"你是一位专业的小说作家，擅长各种文学流派的创作。

在续写时，你必须严格遵守以下规则：

1. **角色名称一致性**：
   - 必须使用【角色信息】中提供的角色名称，绝对不能自行创造新名字
   - 如果文中提到的角色在【角色信息】中找不到，保持原文中的称呼方式

2. **角色性格一致性**：
   - 角色的言行必须符合其性格设定
   - 对话风格要符合角色的身份和背景

3. **世界观一致性**：
   - 遵守【世界观设定】中的规则和设定

4. **情节连贯性**：
   - 续写内容要与前文自然衔接"#.to_string(),
            user_prompt_template: r#"【世界观设定】
{worldview_context}

【角色信息】
{character_context}

【前文内容】
{context}

【续写要求】
{instruction}

请直接续写内容："#.to_string(),
            variables: vec!["context".to_string(), "instruction".to_string(), "character_context".to_string(), "worldview_context".to_string()],
        },
        DefaultPrompt {
            id: "novel-rewrite".to_string(),
            name: "小说重写".to_string(),
            category: "writing".to_string(),
            description: "根据要求重写指定内容".to_string(),
            system_prompt: "你是一位专业的编辑和作家，擅长修改和优化文学作品。请根据指令对给定的文本进行重写，在保持原意的基础上提升文采、调整语调或优化表达。".to_string(),
            user_prompt_template: "原文：\n{content}\n\n重写要求：{instruction}\n\n请直接输出重写后的内容：".to_string(),
            variables: vec!["content".to_string(), "instruction".to_string()],
        },
        DefaultPrompt {
            id: "character-generate".to_string(),
            name: "角色生成".to_string(),
            category: "generation".to_string(),
            description: "根据项目背景生成角色信息".to_string(),
            system_prompt: "你是一位专业的小说角色设计师，擅长创建有深度、有特色的人物角色。生成的角色应该有独特的性格、外貌、背景和动机。".to_string(),
            user_prompt_template: "项目类型：{genre}\n项目描述：{description}\n\n请生成一个角色，包含姓名、性别、年龄、外貌、性格、背景等信息：".to_string(),
            variables: vec!["genre".to_string(), "description".to_string()],
        },
        DefaultPrompt {
            id: "outline-generate".to_string(),
            name: "大纲生成".to_string(),
            category: "generation".to_string(),
            description: "根据项目信息生成故事大纲".to_string(),
            system_prompt: "你是一位资深的故事架构师，擅长设计引人入胜的情节结构。请根据给定的信息创建一个完整的故事大纲，包括主要情节点、转折和高潮。".to_string(),
            user_prompt_template: "项目类型：{genre}\n项目描述：{description}\n主要角色：{characters}\n\n请生成故事大纲：".to_string(),
            variables: vec!["genre".to_string(), "description".to_string(), "characters".to_string()],
        },
        DefaultPrompt {
            id: "worldview-generate".to_string(),
            name: "世界观生成".to_string(),
            category: "generation".to_string(),
            description: "根据项目背景生成世界观设定".to_string(),
            system_prompt: "你是一位专业的世界观设计师，擅长构建完整、自洽的虚构世界。请根据给定的信息创建世界观设定，包括地理、历史、文化、规则等元素。".to_string(),
            user_prompt_template: "项目类型：{genre}\n项目描述：{description}\n\n请生成世界观设定：".to_string(),
            variables: vec!["genre".to_string(), "description".to_string()],
        },
        DefaultPrompt {
            id: "chapter-analyze".to_string(),
            name: "章节分析".to_string(),
            category: "analysis".to_string(),
            description: "分析章节内容提取关键信息".to_string(),
            system_prompt: "你是一位专业的文学分析师，擅长从文本中提取关键信息、分析结构、识别角色和情节元素。".to_string(),
            user_prompt_template: "请分析以下章节内容，提取：1.主要角色 2.关键事件 3.场景 4.情感基调\n\n章节内容：\n{content}\n\n请输出分析结果：".to_string(),
            variables: vec!["content".to_string()],
        },
    ]
}
