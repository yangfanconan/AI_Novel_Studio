use crate::reverse_analysis::types::*;
use crate::logger::{Logger, log_command_start, log_command_success, log_command_error};
use crate::database::get_connection;
use std::sync::Arc;
use tokio::sync::RwLock;
use regex::Regex;
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use chrono::Utc;
use rusqlite::params;

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

pub async fn analyze_novel(
    _ai_service: Arc<RwLock<crate::ai::AIService>>,
    content: &str,
    title: &str,
    _depth: AnalysisDepth,
) -> Result<ReverseAnalysisResult, String> {
    let logger = Logger::new().with_feature("reverse-analysis");
    log_command_start(&logger, "analyze_novel", title);

    let total_words = content.chars().count();
    let chapters = split_into_chapters(content);
    let chapter_count = chapters.len();

    let characters = extract_characters(content, &chapters);
    let relationships = analyze_relationships(content, &characters);
    let worldviews = extract_worldviews(content);
    let plot_points = extract_plot_points(&chapters);
    let outline = build_outline(&chapters);
    let style_analysis = analyze_style(content);
    let summary = generate_summary(&chapters);

    let result = ReverseAnalysisResult {
        title: title.to_string(),
        summary,
        total_words,
        chapter_count,
        characters,
        relationships,
        worldviews,
        plot_points,
        outline,
        style_analysis,
    };

    log_command_success(&logger, "analyze_novel", &format!("{} chapters, {} characters", chapter_count, result.characters.len()));
    Ok(result)
}

fn split_into_chapters(content: &str) -> Vec<(String, String)> {
    let mut chapters = Vec::new();
    
    let chapter_patterns = vec![
        Regex::new(r"(?m)^(第[零一二三四五六七八九十百千万\d]+章[^\n]*)$").unwrap(),
        Regex::new(r"(?m)^Chapter\s*(\d+)[^\n]*$").unwrap(),
        Regex::new(r"(?m)^(\d+)[\.\s]+[^\n]+$").unwrap(),
    ];

    let mut current_title = "序章".to_string();
    let mut current_content = String::new();
    let mut found_chapters = false;

    for line in content.lines() {
        let trimmed = line.trim();
        let mut is_chapter_start = false;
        let mut chapter_title = String::new();

        for pattern in &chapter_patterns {
            if let Some(caps) = pattern.captures(trimmed) {
                is_chapter_start = true;
                chapter_title = caps[0].to_string();
                break;
            }
        }

        if is_chapter_start {
            if !current_content.trim().is_empty() {
                chapters.push((current_title.clone(), current_content.trim().to_string()));
            }
            current_title = chapter_title;
            current_content = String::new();
            found_chapters = true;
        } else if found_chapters || !trimmed.is_empty() {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }

    if !current_content.trim().is_empty() {
        chapters.push((current_title, current_content.trim().to_string()));
    }

    if chapters.is_empty() {
        chapters.push(("正文".to_string(), content.to_string()));
    }

    chapters
}

fn extract_characters(content: &str, chapters: &[(String, String)]) -> Vec<ExtractedCharacter> {
    let mut characters: Vec<ExtractedCharacter> = Vec::new();
    let mut mention_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    let name_patterns = vec![
        Regex::new(r"[\u4e00-\u9fa5]{2,4}说").unwrap(),
        Regex::new(r"[\u4e00-\u9fa5]{2,4}道").unwrap(),
        Regex::new(r"[\u4e00-\u9fa5]{2,4}想").unwrap(),
        Regex::new(r"[\u4e00-\u9fa5]{2,4}看").unwrap(),
        Regex::new(r#""([^"]+)""#).unwrap(),
    ];

    for pattern in &name_patterns {
        for caps in pattern.captures_iter(content) {
            if let Some(name_match) = caps.get(1).or_else(|| caps.get(0)) {
                let name = name_match.as_str();
                let name = name.trim_end_matches(|c| "说道想看着".contains(c));
                
                if name.len() >= 2 && name.len() <= 4 && name.chars().all(|c| c >= '\u{4e00}' && c <= '\u{9fa5}') {
                    *mention_counts.entry(name.to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    let common_words = std::collections::HashSet::from([
        "就是", "不是", "什么", "这个", "那个", "怎么", "这样", "那样",
        "他的", "她的", "我的", "你的", "咱们", "这里", "那里",
    ]);

    for (name, count) in mention_counts {
        if count >= 3 && !common_words.contains(name.as_str()) {
            characters.push(ExtractedCharacter {
                name: name.clone(),
                aliases: vec![],
                description: String::new(),
                personality: String::new(),
                appearance: String::new(),
                role: if count > 50 { "主角" } else if count > 10 { "配角" } else { "次要角色" }.to_string(),
                first_appearance: chapters.first().map(|(t, _)| t.clone()),
                mention_count: count,
            });
        }
    }

    characters.sort_by(|a, b| b.mention_count.cmp(&a.mention_count));
    characters.truncate(20);
    characters
}

fn analyze_relationships(content: &str, characters: &[ExtractedCharacter]) -> Vec<ExtractedRelationship> {
    let mut relationships = Vec::new();

    if characters.len() < 2 {
        return relationships;
    }

    let relation_patterns: Vec<(Regex, &str)> = vec![
        (Regex::new(r"(\S+)是(\S+)的(父亲|母亲|哥哥|弟弟|姐姐|妹妹|师父|徒弟|朋友|敌人|爱人)").unwrap(), "family"),
        (Regex::new(r"(\S+)与(\S+)(并肩|联手|对峙|相爱|结仇)").unwrap(), "interaction"),
    ];

    for (pattern, rel_type) in relation_patterns {
        for caps in pattern.captures_iter(content) {
            if caps.len() >= 3 {
                let char1 = caps[1].to_string();
                let char2 = caps[2].to_string();
                
                if characters.iter().any(|c| c.name == char1) && characters.iter().any(|c| c.name == char2) {
                    relationships.push(ExtractedRelationship {
                        character1: char1,
                        character2: char2,
                        relationship_type: rel_type.to_string(),
                        description: caps[0].to_string(),
                        strength: 0.5,
                    });
                }
            }
        }
    }

    relationships.truncate(10);
    relationships
}

fn extract_worldviews(content: &str) -> Vec<ExtractedWorldview> {
    let mut worldviews = Vec::new();

    let worldview_patterns: Vec<(Regex, &str)> = vec![
        (Regex::new(r"(\S+界|\S+国|\S+城|\S+山|\S+海|\S+宫)").unwrap(), "地点"),
        (Regex::new(r"(\S+功法|\S+武学|\S+法术|\S+技能)").unwrap(), "能力体系"),
        (Regex::new(r"(\S+境|\S+级|\S+阶|\S+品)").unwrap(), "等级体系"),
    ];

    for (pattern, category) in worldview_patterns {
        let mut found = std::collections::HashSet::new();
        for caps in pattern.captures_iter(content) {
            let name = caps[1].to_string();
            if name.len() >= 2 && name.len() <= 6 && !found.contains(&name) {
                found.insert(name.clone());
                worldviews.push(ExtractedWorldview {
                    name,
                    category: category.to_string(),
                    description: String::new(),
                    details: vec![],
                });
            }
        }
    }

    worldviews.truncate(15);
    worldviews
}

fn extract_plot_points(chapters: &[(String, String)]) -> Vec<ExtractedPlotPoint> {
    let mut plot_points = Vec::new();

    let event_keywords = ["战斗", "冲突", "发现", "相遇", "离别", "死亡", "突破", "觉醒", "背叛", "复仇"];
    
    for (index, (title, content)) in chapters.iter().enumerate() {
        for keyword in event_keywords {
            if content.contains(keyword) {
                plot_points.push(ExtractedPlotPoint {
                    chapter_index: index,
                    title: format!("{} - {}", title, keyword),
                    description: format!("章节中出现了关键的[{}]情节", keyword),
                    plot_type: keyword.to_string(),
                    characters_involved: vec![],
                    importance: 0.5,
                });
                break;
            }
        }
    }

    plot_points.truncate(20);
    plot_points
}

fn build_outline(chapters: &[(String, String)]) -> ExtractedOutline {
    let mut arcs = Vec::new();
    
    if chapters.is_empty() {
        return ExtractedOutline { arcs };
    }

    let total_chapters = chapters.len();
    let arc_size = (total_chapters / 3).max(1);

    let arc_titles = ["铺垫篇", "发展篇", "高潮篇", "结局篇"];
    
    for (i, &title) in arc_titles.iter().enumerate() {
        let start = i * arc_size;
        let end = if i == arc_titles.len() - 1 { total_chapters } else { (i + 1) * arc_size };
        
        if start < total_chapters {
            let arc_chapters: Vec<_> = chapters[start..end.min(total_chapters)].to_vec();
            let summary = arc_chapters.iter()
                .map(|(t, _)| t.as_str())
                .take(5)
                .collect::<Vec<_>>()
                .join(",");

            arcs.push(OutlineArc {
                title: title.to_string(),
                start_chapter: start + 1,
                end_chapter: end,
                summary,
                key_events: vec![],
            });
        }
    }

    ExtractedOutline { arcs }
}

fn analyze_style(content: &str) -> StyleAnalysis {
    let sentences: Vec<&str> = content.split(|c| c == '。' || c == '！' || c == '？').collect();
    let total_sentences = sentences.len().max(1);
    
    let avg_length = sentences.iter().map(|s| s.chars().count()).sum::<usize>() / total_sentences;
    
    let dialogue_pattern = Regex::new(r#""[^"]*""#).unwrap();
    let dialogue_chars: usize = dialogue_pattern.find_iter(content).map(|m| m.as_str().chars().count()).sum();
    let dialogue_ratio = dialogue_chars as f32 / content.chars().count().max(1) as f32;

    let unique_chars: std::collections::HashSet<char> = content.chars().collect();
    let vocabulary_richness = unique_chars.len() as f32 / content.chars().count().max(1) as f32;

    StyleAnalysis {
        writing_style: "待分析".to_string(),
        narrative_voice: "第三人称".to_string(),
        dialogue_ratio,
        description_ratio: 1.0 - dialogue_ratio,
        average_sentence_length: avg_length as f32,
        vocabulary_richness,
        pacing: if avg_length > 50 { "舒缓" } else if avg_length > 30 { "适中" } else { "紧凑" }.to_string(),
        tone: "中性".to_string(),
    }
}

fn generate_summary(chapters: &[(String, String)]) -> String {
    if chapters.is_empty() {
        return "无法生成摘要".to_string();
    }

    let total_words: usize = chapters.iter().map(|(_, c)| c.chars().count()).sum();
    let chapter_titles: Vec<&str> = chapters.iter().take(5).map(|(t, _)| t.as_str()).collect();

    format!(
        "本小说共 {} 章,约 {} 字。前五章: {}...",
        chapters.len(),
        total_words,
        chapter_titles.join(",")
    )
}

#[tauri::command]
pub async fn reverse_analyze_novel(
    ai_service: tauri::State<'_, Arc<RwLock<crate::ai::AIService>>>,
    content: String,
    title: String,
    depth: String,
) -> Result<ReverseAnalysisResult, String> {
    let analysis_depth = match depth.as_str() {
        "basic" => AnalysisDepth::Basic,
        "deep" => AnalysisDepth::Deep,
        _ => AnalysisDepth::Standard,
    };

    let service = ai_service.inner().clone();
    analyze_novel(service, &content, &title, analysis_depth).await
}

#[tauri::command]
pub async fn reverse_analyze_and_import(
    ai_service: tauri::State<'_, Arc<RwLock<crate::ai::AIService>>>,
    app: AppHandle,
    content: String,
    title: String,
    import_characters: bool,
    import_worldviews: bool,
    import_outline: bool,
) -> Result<ReverseAnalysisResult, String> {
    let logger = Logger::new().with_feature("reverse-analysis");
    log_command_start(&logger, "reverse_analyze_and_import", &title);

    let service = ai_service.inner().clone();
    let result = analyze_novel(service, &content, &title, AnalysisDepth::Standard).await?;

    let db_path = get_db_path(&app)?;
    let conn = get_connection(&db_path)
        .map_err(|e| format!("数据库连接失败: {}", e))?;

    let project_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO projects (id, name, description, genre, template, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            project_id,
            title,
            result.summary.clone(),
            "逆向导入",
            "default",
            "active",
            now,
            now,
        ],
    ).map_err(|e| format!("创建项目失败: {}", e))?;

    let chapters = split_into_chapters(&content);
    for (idx, (chapter_title, chapter_content)) in chapters.iter().enumerate() {
        let chapter_id = Uuid::new_v4().to_string();
        let chapter_now = Utc::now().to_rfc3339();
        let word_count = chapter_content.chars().count() as i32;
        
        conn.execute(
            "INSERT INTO chapters (id, project_id, title, content, word_count, sort_order, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                chapter_id,
                project_id,
                chapter_title,
                chapter_content,
                word_count,
                idx as i32,
                "published",
                chapter_now,
                chapter_now,
            ],
        ).map_err(|e| format!("创建章节失败: {}", e))?;
    }

    if import_characters {
        for character in &result.characters {
            let char_id = Uuid::new_v4().to_string();
            let char_now = Utc::now().to_rfc3339();
            
            conn.execute(
                "INSERT INTO characters (id, project_id, name, role_type, race, age, gender, birth_date, appearance, personality, background, skills, status, bazi, ziwei, mbti, enneagram, items, avatar_url, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    char_id,
                    project_id,
                    character.name,
                    character.role,
                    "",
                    None::<String>,
                    None::<String>,
                    None::<String>,
                    character.appearance,
                    character.personality,
                    character.description,
                    None::<String>,
                    "active",
                    None::<String>,
                    None::<String>,
                    None::<String>,
                    None::<String>,
                    None::<String>,
                    None::<String>,
                    char_now,
                    char_now,
                ],
            ).map_err(|e| format!("创建角色失败: {}", e))?;
        }

        for relationship in &result.relationships {
            let rel_id = Uuid::new_v4().to_string();
            let rel_now = Utc::now().to_rfc3339();
            
            let char1_id: Option<String> = conn.query_row(
                "SELECT id FROM characters WHERE project_id = ? AND name = ?",
                params![project_id, relationship.character1],
                |row| row.get(0),
            ).ok();
            
            let char2_id: Option<String> = conn.query_row(
                "SELECT id FROM characters WHERE project_id = ? AND name = ?",
                params![project_id, relationship.character2],
                |row| row.get(0),
            ).ok();

            if let (Some(c1), Some(c2)) = (char1_id, char2_id) {
                conn.execute(
                    "INSERT INTO character_relations (id, project_id, character1_id, character2_id, relation_type, description, strength, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                    params![
                        rel_id,
                        project_id,
                        c1,
                        c2,
                        relationship.relationship_type,
                        relationship.description,
                        relationship.strength,
                        rel_now,
                        rel_now,
                    ],
                ).map_err(|e| format!("创建角色关系失败: {}", e))?;
            }
        }
    }

    if import_worldviews {
        for worldview in &result.worldviews {
            let wv_id = Uuid::new_v4().to_string();
            let wv_now = Utc::now().to_rfc3339();
            
            conn.execute(
                "INSERT INTO world_views (id, project_id, name, category, description, details, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    wv_id,
                    project_id,
                    worldview.name,
                    worldview.category,
                    worldview.description,
                    serde_json::to_string(&worldview.details).unwrap_or_else(|_| "[]".to_string()),
                    wv_now,
                    wv_now,
                ],
            ).map_err(|e| format!("创建世界观失败: {}", e))?;
        }
    }

    if import_outline {
        for (idx, arc) in result.outline.arcs.iter().enumerate() {
            let node_id = Uuid::new_v4().to_string();
            let node_now = Utc::now().to_rfc3339();
            
            conn.execute(
                "INSERT INTO outline_nodes (id, project_id, parent_id, title, content, node_type, sort_order, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    node_id,
                    project_id,
                    None::<String>,
                    arc.title,
                    arc.summary,
                    "arc",
                    idx as i32,
                    "active",
                    node_now,
                    node_now,
                ],
            ).map_err(|e| format!("创建大纲节点失败: {}", e))?;
        }
    }

    log_command_success(&logger, "reverse_analyze_and_import", &format!("project: {}, chapters: {}, characters: {}, worldviews: {}", project_id, chapters.len(), result.characters.len(), result.worldviews.len()));
    
    Ok(result)
}
