use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;
use rusqlite::{Connection, params, Result as SqlResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTemplate {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub workflow_json: String,
    pub preview_image: Option<String>,
    pub tags: Vec<String>,
    pub is_builtin: bool,
    pub is_favorite: bool,
    pub usage_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub workflow_json: String,
    pub preview_image: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTemplateRequest {
    pub id: String,
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub workflow_json: Option<String>,
    pub preview_image: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_favorite: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVariable {
    pub node_id: i32,
    pub field: String,
    pub variable_name: String,
    pub default_value: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedTemplate {
    pub template: WorkflowTemplate,
    pub variables: Vec<WorkflowVariable>,
    pub required_models: Vec<String>,
    pub estimated_vram: Option<i32>,
}

pub struct WorkflowTemplateManager;

impl WorkflowTemplateManager {
    pub fn init_table(conn: &Connection) -> SqlResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS workflow_templates (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                description TEXT,
                workflow_json TEXT NOT NULL,
                preview_image TEXT,
                tags TEXT,
                is_builtin INTEGER DEFAULT 0,
                is_favorite INTEGER DEFAULT 0,
                usage_count INTEGER DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_workflow_templates_category ON workflow_templates(category)",
            [],
        )?;

        Ok(())
    }

    pub fn create(conn: &Connection, request: CreateTemplateRequest) -> SqlResult<WorkflowTemplate> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let tags_json = serde_json::to_string(&request.tags.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());

        conn.execute(
            "INSERT INTO workflow_templates (
                id, name, category, description, workflow_json, preview_image,
                tags, is_builtin, is_favorite, usage_count, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, 0, 0, ?8, ?9)",
            params![
                id,
                request.name,
                request.category,
                request.description,
                request.workflow_json,
                request.preview_image,
                tags_json,
                now,
                now,
            ],
        )?;

        Ok(WorkflowTemplate {
            id,
            name: request.name,
            category: request.category,
            description: request.description,
            workflow_json: request.workflow_json,
            preview_image: request.preview_image,
            tags: request.tags.unwrap_or_default(),
            is_builtin: false,
            is_favorite: false,
            usage_count: 0,
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn get(conn: &Connection, id: &str) -> SqlResult<Option<WorkflowTemplate>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, category, description, workflow_json, preview_image,
                    tags, is_builtin, is_favorite, usage_count, created_at, updated_at
             FROM workflow_templates WHERE id = ?1"
        )?;

        let result = stmt.query_row(params![id], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            
            Ok(WorkflowTemplate {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                workflow_json: row.get(4)?,
                preview_image: row.get(5)?,
                tags,
                is_builtin: row.get::<_, i32>(7)? == 1,
                is_favorite: row.get::<_, i32>(8)? == 1,
                usage_count: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        });

        match result {
            Ok(template) => Ok(Some(template)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_all(conn: &Connection) -> SqlResult<Vec<WorkflowTemplate>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, category, description, workflow_json, preview_image,
                    tags, is_builtin, is_favorite, usage_count, created_at, updated_at
             FROM workflow_templates ORDER BY usage_count DESC, name ASC"
        )?;

        let templates = stmt.query_map([], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            
            Ok(WorkflowTemplate {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                workflow_json: row.get(4)?,
                preview_image: row.get(5)?,
                tags,
                is_builtin: row.get::<_, i32>(7)? == 1,
                is_favorite: row.get::<_, i32>(8)? == 1,
                usage_count: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?;

        templates.collect()
    }

    pub fn get_by_category(conn: &Connection, category: &str) -> SqlResult<Vec<WorkflowTemplate>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, category, description, workflow_json, preview_image,
                    tags, is_builtin, is_favorite, usage_count, created_at, updated_at
             FROM workflow_templates WHERE category = ?1 ORDER BY usage_count DESC, name ASC"
        )?;

        let templates = stmt.query_map(params![category], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            
            Ok(WorkflowTemplate {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                workflow_json: row.get(4)?,
                preview_image: row.get(5)?,
                tags,
                is_builtin: row.get::<_, i32>(7)? == 1,
                is_favorite: row.get::<_, i32>(8)? == 1,
                usage_count: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?;

        templates.collect()
    }

    pub fn get_favorites(conn: &Connection) -> SqlResult<Vec<WorkflowTemplate>> {
        let mut stmt = conn.prepare(
            "SELECT id, name, category, description, workflow_json, preview_image,
                    tags, is_builtin, is_favorite, usage_count, created_at, updated_at
             FROM workflow_templates WHERE is_favorite = 1 ORDER BY usage_count DESC, name ASC"
        )?;

        let templates = stmt.query_map([], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            
            Ok(WorkflowTemplate {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                workflow_json: row.get(4)?,
                preview_image: row.get(5)?,
                tags,
                is_builtin: row.get::<_, i32>(7)? == 1,
                is_favorite: row.get::<_, i32>(8)? == 1,
                usage_count: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?;

        templates.collect()
    }

    pub fn search(conn: &Connection, query: &str) -> SqlResult<Vec<WorkflowTemplate>> {
        let pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, name, category, description, workflow_json, preview_image,
                    tags, is_builtin, is_favorite, usage_count, created_at, updated_at
             FROM workflow_templates 
             WHERE name LIKE ?1 OR description LIKE ?1 OR tags LIKE ?1
             ORDER BY usage_count DESC, name ASC"
        )?;

        let templates = stmt.query_map(params![pattern], |row| {
            let tags_json: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_json).unwrap_or_default();
            
            Ok(WorkflowTemplate {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                description: row.get(3)?,
                workflow_json: row.get(4)?,
                preview_image: row.get(5)?,
                tags,
                is_builtin: row.get::<_, i32>(7)? == 1,
                is_favorite: row.get::<_, i32>(8)? == 1,
                usage_count: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?;

        templates.collect()
    }

    pub fn update(conn: &Connection, request: UpdateTemplateRequest) -> SqlResult<Option<WorkflowTemplate>> {
        let now = Utc::now().to_rfc3339();
        
        let mut updates = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref v) = request.name {
            updates.push("name = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.category {
            updates.push("category = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.description {
            updates.push("description = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.workflow_json {
            updates.push("workflow_json = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.preview_image {
            updates.push("preview_image = ?");
            values.push(Box::new(v.clone()));
        }
        if let Some(ref v) = request.tags {
            updates.push("tags = ?");
            values.push(Box::new(serde_json::to_string(v).unwrap_or_else(|_| "[]".to_string())));
        }
        if let Some(v) = request.is_favorite {
            updates.push("is_favorite = ?");
            values.push(Box::new(if v { 1 } else { 0 }));
        }

        if updates.is_empty() {
            return Self::get(conn, &request.id);
        }

        updates.push("updated_at = ?");
        values.push(Box::new(now));
        values.push(Box::new(request.id.clone()));

        let sql = format!(
            "UPDATE workflow_templates SET {} WHERE id = ?",
            updates.join(", ")
        );

        let params: Vec<&dyn rusqlite::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        conn.execute(&sql, params.as_slice())?;

        Self::get(conn, &request.id)
    }

    pub fn delete(conn: &Connection, id: &str) -> SqlResult<bool> {
        let affected = conn.execute(
            "DELETE FROM workflow_templates WHERE id = ?1 AND is_builtin = 0",
            params![id],
        )?;
        Ok(affected > 0)
    }

    pub fn increment_usage(conn: &Connection, id: &str) -> SqlResult<()> {
        conn.execute(
            "UPDATE workflow_templates SET usage_count = usage_count + 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn toggle_favorite(conn: &Connection, id: &str) -> SqlResult<bool> {
        conn.execute(
            "UPDATE workflow_templates SET is_favorite = NOT is_favorite WHERE id = ?1",
            params![id],
        )?;
        
        let template = Self::get(conn, id)?;
        Ok(template.map(|t| t.is_favorite).unwrap_or(false))
    }

    pub fn get_categories(conn: &Connection) -> SqlResult<Vec<String>> {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT category FROM workflow_templates ORDER BY category"
        )?;

        let categories = stmt.query_map([], |row| row.get(0))?;
        categories.collect()
    }

    pub fn parse_template(template: &WorkflowTemplate) -> ParsedTemplate {
        let mut variables = Vec::new();
        let mut required_models = Vec::new();

        if let Ok(workflow) = super::comfyui_client::ComfyUIWorkflow::from_json(&template.workflow_json) {
            for node in &workflow.nodes {
                if node.node_type == "CheckpointLoaderSimple" {
                    if let Some(model_name) = node.properties.get("ckpt_name")
                        .and_then(|v| v.as_str())
                    {
                        required_models.push(model_name.to_string());
                    }
                }

                for (key, value) in &node.properties {
                    if let Some(str_val) = value.as_str() {
                        if str_val.starts_with("{{") && str_val.ends_with("}}") {
                            let var_name = str_val[2..str_val.len()-2].trim().to_string();
                            variables.push(WorkflowVariable {
                                node_id: node.id,
                                field: key.clone(),
                                variable_name: var_name,
                                default_value: None,
                                description: None,
                            });
                        }
                    }
                }
            }
        }

        ParsedTemplate {
            template: template.clone(),
            variables,
            required_models,
            estimated_vram: None,
        }
    }

    pub fn apply_variables(
        template: &WorkflowTemplate,
        values: &HashMap<String, serde_json::Value>,
    ) -> Result<String, String> {
        let mut workflow = super::comfyui_client::ComfyUIWorkflow::from_json(&template.workflow_json)
            .map_err(|e| format!("Failed to parse workflow: {}", e))?;

        for node in &mut workflow.nodes {
            for (_key, value) in &mut node.properties {
                if let Some(str_val) = value.as_str() {
                    if str_val.starts_with("{{") && str_val.ends_with("}}") {
                        let var_name = str_val[2..str_val.len()-2].trim();
                        if let Some(new_value) = values.get(var_name) {
                            *value = new_value.clone();
                        }
                    }
                }
            }
        }

        workflow.to_json()
    }
}

pub fn get_builtin_templates() -> Vec<CreateTemplateRequest> {
    vec![
        CreateTemplateRequest {
            name: "基础文生图".to_string(),
            category: "text_to_image".to_string(),
            description: Some("基础的文本到图像生成工作流".to_string()),
            workflow_json: r#"{
                "last_node_id": 3,
                "last_link_id": 2,
                "nodes": [
                    {"id": 1, "type": "CheckpointLoaderSimple", "pos": [0, 0], "size": [315, 98], "flags": {}, "order": 0, "mode": 0, "inputs": [], "outputs": [{"name": "MODEL", "type": "MODEL", "links": [1]}, {"name": "CLIP", "type": "CLIP", "links": [2]}, {"name": "VAE", "type": "VAE", "links": null}], "properties": {}, "widgets_values": ["{{checkpoint}}"]},
                    {"id": 2, "type": "CLIPTextEncode", "pos": [0, 150], "size": [400, 200], "flags": {}, "order": 1, "mode": 0, "inputs": [{"name": "clip", "type": "CLIP", "link": 2}], "outputs": [{"name": "CONDITIONING", "type": "CONDITIONING", "links": [3]}], "properties": {}, "widgets_values": ["{{positive_prompt}}"]},
                    {"id": 3, "type": "KSampler", "pos": [500, 0], "size": [315, 262], "flags": {}, "order": 2, "mode": 0, "inputs": [{"name": "model", "type": "MODEL", "link": 1}, {"name": "positive", "type": "CONDITIONING", "link": 3}], "outputs": [], "properties": {}, "widgets_values": [1561848946, "randomize", 20, 8, "euler", "normal", 1]}
                ],
                "links": [[1, 1, 0, 3, 0, "MODEL"], [2, 1, 1, 2, 0, "CLIP"], [3, 2, 0, 3, 1, "CONDITIONING"]]
            }"#.to_string(),
            preview_image: None,
            tags: Some(vec!["basic".to_string(), "txt2img".to_string()]),
        },
        CreateTemplateRequest {
            name: "图生图".to_string(),
            category: "image_to_image".to_string(),
            description: Some("基于输入图像进行风格转换".to_string()),
            workflow_json: r#"{
                "last_node_id": 4,
                "last_link_id": 3,
                "nodes": [
                    {"id": 1, "type": "CheckpointLoaderSimple", "pos": [0, 0], "size": [315, 98], "flags": {}, "order": 0, "mode": 0, "inputs": [], "outputs": [{"name": "MODEL", "type": "MODEL", "links": [1]}, {"name": "CLIP", "type": "CLIP", "links": [2]}, {"name": "VAE", "type": "VAE", "links": [3]}], "properties": {}, "widgets_values": ["{{checkpoint}}"]},
                    {"id": 2, "type": "LoadImage", "pos": [0, 150], "size": [315, 314], "flags": {}, "order": 1, "mode": 0, "inputs": [], "outputs": [{"name": "IMAGE", "type": "IMAGE", "links": [4]}], "properties": {}, "widgets_values": ["{{input_image}}", "image"]},
                    {"id": 3, "type": "CLIPTextEncode", "pos": [0, 500], "size": [400, 200], "flags": {}, "order": 2, "mode": 0, "inputs": [{"name": "clip", "type": "CLIP", "link": 2}], "outputs": [{"name": "CONDITIONING", "type": "CONDITIONING", "links": [5]}], "properties": {}, "widgets_values": ["{{prompt}}"]},
                    {"id": 4, "type": "KSampler", "pos": [500, 0], "size": [315, 262], "flags": {}, "order": 3, "mode": 0, "inputs": [{"name": "model", "type": "MODEL", "link": 1}, {"name": "positive", "type": "CONDITIONING", "link": 5}, {"name": "latent_image", "type": "LATENT", "link": 6}], "outputs": [], "properties": {}, "widgets_values": [1561848946, "randomize", 20, 8, "euler", "normal", 0.7]}
                ],
                "links": [[1, 1, 0, 4, 0, "MODEL"], [2, 1, 1, 3, 0, "CLIP"], [3, 1, 2, 4, 2, "VAE"]]
            }"#.to_string(),
            preview_image: None,
            tags: Some(vec!["img2img".to_string(), "style_transfer".to_string()]),
        },
    ]
}

#[tauri::command]
pub async fn create_workflow_template(
    request: CreateTemplateRequest,
    db_path: String,
) -> Result<WorkflowTemplate, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::init_table(&conn).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::create(&conn, request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_workflow_template(id: String, db_path: String) -> Result<Option<WorkflowTemplate>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::get(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_all_workflow_templates(db_path: String) -> Result<Vec<WorkflowTemplate>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::init_table(&conn).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::get_all(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_templates_by_category(category: String, db_path: String) -> Result<Vec<WorkflowTemplate>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::get_by_category(&conn, &category).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_workflow_templates(query: String, db_path: String) -> Result<Vec<WorkflowTemplate>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::search(&conn, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_workflow_template(
    request: UpdateTemplateRequest,
    db_path: String,
) -> Result<Option<WorkflowTemplate>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::update(&conn, request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_workflow_template(id: String, db_path: String) -> Result<bool, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::delete(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_template_favorite(id: String, db_path: String) -> Result<bool, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::toggle_favorite(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_template_categories(db_path: String) -> Result<Vec<String>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::get_categories(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn parse_workflow_template(id: String, db_path: String) -> Result<ParsedTemplate, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    let template = WorkflowTemplateManager::get(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or("Template not found")?;
    Ok(WorkflowTemplateManager::parse_template(&template))
}

#[tauri::command]
pub async fn apply_template_variables(
    id: String,
    values: HashMap<String, serde_json::Value>,
    db_path: String,
) -> Result<String, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    let template = WorkflowTemplateManager::get(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or("Template not found")?;
    
    WorkflowTemplateManager::increment_usage(&conn, &id).map_err(|e| e.to_string())?;
    
    WorkflowTemplateManager::apply_variables(&template, &values)
}

#[tauri::command]
pub async fn init_builtin_templates(db_path: String) -> Result<Vec<WorkflowTemplate>, String> {
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    WorkflowTemplateManager::init_table(&conn).map_err(|e| e.to_string())?;
    
    let builtin = get_builtin_templates();
    let mut created = Vec::new();
    
    for request in builtin {
        match WorkflowTemplateManager::create(&conn, request) {
            Ok(template) => created.push(template),
            Err(_) => continue,
        }
    }
    
    Ok(created)
}
