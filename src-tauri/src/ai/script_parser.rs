use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedScene {
    pub scene_id: i32,
    pub narration: String,
    pub visual_content: String,
    pub action: String,
    pub camera: String,
    pub character_description: String,
    pub duration_seconds: Option<f32>,
    pub transition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedScreenplay {
    pub title: String,
    pub scenes: Vec<ParsedScene>,
    pub total_duration: f32,
    pub character_references: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptParseOptions {
    pub scene_count: Option<i32>,
    pub language: Option<String>,
    pub include_camera_directions: Option<bool>,
    pub include_transitions: Option<bool>,
}

impl Default for ScriptParseOptions {
    fn default() -> Self {
        Self {
            scene_count: Some(5),
            language: Some("mixed".to_string()),
            include_camera_directions: Some(true),
            include_transitions: Some(true),
        }
    }
}

pub struct ScriptParser {
    camera_patterns: Vec<(&'static str, &'static str)>,
    transition_patterns: Vec<&'static str>,
}

impl ScriptParser {
    pub fn new() -> Self {
        Self {
            camera_patterns: vec![
                ("close-up", "Close-up"),
                ("close up", "Close-up"),
                ("特写", "Close-up"),
                ("medium shot", "Medium Shot"),
                ("中景", "Medium Shot"),
                ("wide shot", "Wide Shot"),
                ("全景", "Wide Shot"),
                ("long shot", "Long Shot"),
                ("远景", "Long Shot"),
                ("extreme close-up", "Extreme Close-up"),
                ("极特写", "Extreme Close-up"),
                ("over the shoulder", "Over-the-Shoulder"),
                ("过肩", "Over-the-Shoulder"),
                ("point of view", "POV"),
                ("主观视角", "POV"),
                ("pov", "POV"),
                ("two shot", "Two Shot"),
                ("双人镜头", "Two Shot"),
                ("establishing shot", "Establishing Shot"),
                ("建立镜头", "Establishing Shot"),
            ],
            transition_patterns: vec![
                "cut to", "fade to", "dissolve to", "wipe to",
                "切至", "淡出", "叠化", "划像",
            ],
        }
    }

    pub fn parse_novel_to_scenes(
        &self,
        text: &str,
        options: &ScriptParseOptions,
    ) -> Result<ParsedScreenplay, String> {
        let paragraphs: Vec<&str> = text
            .split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && s.len() > 10)
            .collect();

        if paragraphs.is_empty() {
            return Err("No valid content to parse".to_string());
        }

        let scene_count = options.scene_count.unwrap_or(5) as usize;
        let chunk_size = (paragraphs.len() as f64 / scene_count as f64).ceil() as usize;
        let chunk_size = chunk_size.max(1);

        let mut scenes = Vec::new();
        let mut character_refs = Vec::new();

        for (idx, chunk) in paragraphs.chunks(chunk_size).enumerate() {
            let combined_text = chunk.join(" ");
            
            let narration = self.extract_narration(&combined_text);
            let visual_content = self.extract_visual_content(&combined_text);
            let action = self.extract_action(&combined_text);
            let camera = self.detect_camera_type(&combined_text);
            let character_desc = self.extract_character_description(&combined_text);

            let chars = self.extract_character_names(&combined_text);
            for c in chars {
                if !character_refs.contains(&c) {
                    character_refs.push(c);
                }
            }

            scenes.push(ParsedScene {
                scene_id: (idx + 1) as i32,
                narration,
                visual_content,
                action,
                camera,
                character_description: character_desc,
                duration_seconds: Some(self.estimate_duration(&combined_text)),
                transition: if idx > 0 && options.include_transitions.unwrap_or(true) {
                    Some("cut to".to_string())
                } else {
                    None
                },
            });

            if scenes.len() >= scene_count {
                break;
            }
        }

        let total_duration: f32 = scenes.iter()
            .filter_map(|s| s.duration_seconds)
            .sum();

        Ok(ParsedScreenplay {
            title: "Generated Screenplay".to_string(),
            scenes,
            total_duration,
            character_references: character_refs,
        })
    }

    pub fn parse_ai_response(&self, json_response: &str) -> Result<ParsedScreenplay, String> {
        let response: serde_json::Value = serde_json::from_str(json_response)
            .map_err(|e| format!("Failed to parse AI response: {}", e))?;

        let title = response.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Generated Screenplay")
            .to_string();

        let scenes_array = response.get("scenes")
            .and_then(|v| v.as_array())
            .ok_or("No scenes found in AI response")?;

        let mut scenes = Vec::new();
        let mut character_refs = Vec::new();

        for (idx, scene_val) in scenes_array.iter().enumerate() {
            let scene = ParsedScene {
                scene_id: scene_val.get("sceneId")
                    .and_then(|v| v.as_i64())
                    .unwrap_or((idx + 1) as i64) as i32,
                narration: scene_val.get("narration")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                visual_content: scene_val.get("visualContent")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                action: scene_val.get("action")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                camera: scene_val.get("camera")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Medium Shot")
                    .to_string(),
                character_description: scene_val.get("characterDescription")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                duration_seconds: Some(5.0),
                transition: None,
            };

            if !scene.character_description.is_empty() {
                character_refs.push(scene.character_description.clone());
            }

            scenes.push(scene);
        }

        let total_duration: f32 = scenes.len() as f32 * 5.0;

        Ok(ParsedScreenplay {
            title,
            scenes,
            total_duration,
            character_references: character_refs,
        })
    }

    fn extract_narration(&self, text: &str) -> String {
        let sentences: Vec<&str> = text
            .split(|c| c == '。' || c == '.' || c == '！' || c == '!' || c == '？' || c == '?')
            .filter(|s| s.trim().len() > 5)
            .collect();

        sentences.iter()
            .take(3)
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join("。")
            + "。"
    }

    fn extract_visual_content(&self, text: &str) -> String {
        let visual_keywords = [
            "阳光", "月光", "灯光", "黑暗", "明亮", "色彩", "金色", "蓝色", "红色",
            "森林", "城市", "街道", "房间", "海边", "山", "天空", "云", "雨", "雪",
            "阳光", "月光", "shadow", "light", "bright", "dark", "color", "forest",
            "city", "street", "room", "ocean", "mountain", "sky", "cloud", "rain", "snow",
        ];

        let words: Vec<&str> = text.split_whitespace().collect();
        let visual_words: Vec<String> = words
            .iter()
            .filter(|w| {
                visual_keywords.iter().any(|kw| w.to_lowercase().contains(kw))
            })
            .map(|w| w.to_string())
            .take(10)
            .collect();

        if visual_words.is_empty() {
            "A detailed scene with atmospheric lighting".to_string()
        } else {
            visual_words.join(", ")
        }
    }

    fn extract_action(&self, text: &str) -> String {
        let action_keywords = [
            "走", "跑", "跳", "坐", "站", "看", "说", "笑", "哭", "转身", "挥",
            "walk", "run", "jump", "sit", "stand", "look", "speak", "smile", "cry", "turn", "wave",
        ];

        let sentences: Vec<&str> = text
            .split(|c| c == '。' || c == '.' || c == '，' || c == ',')
            .collect();

        let action_sentences: Vec<&str> = sentences
            .iter()
            .filter(|s| {
                action_keywords.iter().any(|kw| s.to_lowercase().contains(kw))
            })
            .map(|s| *s)
            .take(2)
            .collect();

        if action_sentences.is_empty() {
            "standing still, looking forward".to_string()
        } else {
            action_sentences.join(", ")
        }
    }

    fn detect_camera_type(&self, text: &str) -> String {
        let text_lower = text.to_lowercase();
        
        for (pattern, camera_type) in &self.camera_patterns {
            if text_lower.contains(pattern) {
                return camera_type.to_string();
            }
        }

        let text_len = text.len();
        if text_len < 50 {
            "Close-up".to_string()
        } else if text_len < 150 {
            "Medium Shot".to_string()
        } else {
            "Wide Shot".to_string()
        }
    }

    fn extract_character_description(&self, text: &str) -> String {
        let char_patterns = [
            ("男主角", "male protagonist, young man"),
            ("女主角", "female protagonist, young woman"),
            ("老人", "elderly person"),
            ("孩子", "child"),
            ("少年", "teenager, young person"),
            ("少女", "teenage girl, young woman"),
            ("男子", "man"),
            ("女子", "woman"),
        ];

        for (pattern, desc) in &char_patterns {
            if text.contains(pattern) {
                return desc.to_string();
            }
        }

        String::new()
    }

    fn extract_character_names(&self, text: &str) -> Vec<String> {
        let name_regex = Regex::new(r#"[""「」『』]([^""「」『』]{2,10})[""「」『』]"#)
            .unwrap();
        
        let mut names = Vec::new();
        for cap in name_regex.captures_iter(text) {
            if let Some(name) = cap.get(1) {
                let name_str = name.as_str().to_string();
                if !names.contains(&name_str) && name_str.len() >= 2 {
                    names.push(name_str);
                }
            }
        }
        names
    }

    fn estimate_duration(&self, text: &str) -> f32 {
        let char_count = text.chars().count() as f32;
        let word_count = text.split_whitespace().count() as f32;
        
        let estimated = (char_count / 20.0).max(3.0).min(15.0);
        let word_factor = (word_count / 30.0).max(3.0).min(15.0);
        
        (estimated + word_factor) / 2.0
    }

    pub fn merge_scenes(
        &self,
        scenes: &[ParsedScene],
        target_count: i32,
    ) -> Vec<ParsedScene> {
        if scenes.len() <= target_count as usize {
            return scenes.to_vec();
        }

        let ratio = scenes.len() as f32 / target_count as f32;
        let mut merged = Vec::new();

        for i in 0..target_count {
            let start = (i as f32 * ratio) as usize;
            let end = ((i + 1) as f32 * ratio as f32) as usize;
            let end = end.min(scenes.len());

            if start >= scenes.len() {
                break;
            }

            let group = &scenes[start..end];
            if group.len() == 1 {
                merged.push(group[0].clone());
            } else {
                let merged_scene = ParsedScene {
                    scene_id: i + 1,
                    narration: group.iter().map(|s| s.narration.as_str()).collect::<Vec<_>>().join(" "),
                    visual_content: group.iter().map(|s| s.visual_content.as_str()).collect::<Vec<_>>().join(", "),
                    action: group.iter().map(|s| s.action.as_str()).collect::<Vec<_>>().join(", "),
                    camera: group[group.len() / 2].camera.clone(),
                    character_description: group.iter()
                        .map(|s| s.character_description.as_str())
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>()
                        .first()
                        .unwrap_or(&"")
                        .to_string(),
                    duration_seconds: Some(group.iter().filter_map(|s| s.duration_seconds).sum()),
                    transition: None,
                };
                merged.push(merged_scene);
            }
        }

        merged
    }

    pub fn export_to_json(&self, screenplay: &ParsedScreenplay) -> Result<String, String> {
        serde_json::to_string_pretty(screenplay)
            .map_err(|e| format!("Failed to serialize screenplay: {}", e))
    }
}

impl Default for ScriptParser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseNovelRequest {
    pub text: String,
    pub scene_count: Option<i32>,
    pub language: Option<String>,
}

#[tauri::command]
pub async fn parse_novel_to_screenplay(
    request: ParseNovelRequest,
) -> Result<String, String> {
    let parser = ScriptParser::new();
    let options = ScriptParseOptions {
        scene_count: request.scene_count,
        language: request.language,
        ..Default::default()
    };

    let screenplay = parser.parse_novel_to_scenes(&request.text, &options)?;
    parser.export_to_json(&screenplay)
}

#[tauri::command]
pub async fn parse_ai_screenplay_response(
    json_response: String,
) -> Result<String, String> {
    let parser = ScriptParser::new();
    let screenplay = parser.parse_ai_response(&json_response)?;
    parser.export_to_json(&screenplay)
}

#[tauri::command]
pub async fn merge_screenplay_scenes(
    scenes_json: String,
    target_count: i32,
) -> Result<String, String> {
    let parser = ScriptParser::new();
    let scenes: Vec<ParsedScene> = serde_json::from_str(&scenes_json)
        .map_err(|e| format!("Failed to parse scenes: {}", e))?;
    
    let merged = parser.merge_scenes(&scenes, target_count);
    
    let screenplay = ParsedScreenplay {
        title: "Merged Screenplay".to_string(),
        scenes: merged,
        total_duration: 0.0,
        character_references: vec![],
    };
    
    parser.export_to_json(&screenplay)
}
