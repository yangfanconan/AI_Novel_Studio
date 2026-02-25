use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceImage {
    pub id: String,
    pub url: String,
    #[serde(rename = "analysisResult")]
    pub analysis_result: Option<serde_json::Value>,
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeViewImages {
    pub front: Option<String>,
    pub side: Option<String>,
    pub back: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBible {
    pub id: String,
    pub project_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub char_type: String,
    #[serde(rename = "visualTraits")]
    pub visual_traits: String,
    #[serde(rename = "styleTokens")]
    pub style_tokens: Vec<String>,
    #[serde(rename = "colorPalette")]
    pub color_palette: Vec<String>,
    pub personality: String,
    #[serde(rename = "referenceImages")]
    pub reference_images: Vec<ReferenceImage>,
    #[serde(rename = "threeViewImages")]
    pub three_view_images: Option<ThreeViewImages>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCharacterBibleRequest {
    pub project_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub char_type: String,
    #[serde(rename = "visualTraits")]
    pub visual_traits: String,
    #[serde(rename = "styleTokens")]
    pub style_tokens: Vec<String>,
    #[serde(rename = "colorPalette")]
    pub color_palette: Vec<String>,
    pub personality: String,
}

pub struct CharacterBibleManager {
    characters: HashMap<String, CharacterBible>,
}

impl CharacterBibleManager {
    pub fn new() -> Self {
        Self {
            characters: HashMap::new(),
        }
    }

    pub fn add_character(&mut self, request: CreateCharacterBibleRequest) -> CharacterBible {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let character = CharacterBible {
            id: id.clone(),
            project_id: request.project_id,
            name: request.name,
            char_type: request.char_type,
            visual_traits: request.visual_traits,
            style_tokens: request.style_tokens,
            color_palette: request.color_palette,
            personality: request.personality,
            reference_images: vec![],
            three_view_images: None,
            created_at: now.clone(),
            updated_at: now,
        };

        self.characters.insert(id, character.clone());
        character
    }

    pub fn update_character(
        &mut self,
        id: &str,
        updates: CharacterBibleUpdate,
    ) -> Option<CharacterBible> {
        if let Some(existing) = self.characters.get_mut(id) {
            if let Some(name) = updates.name {
                existing.name = name;
            }
            if let Some(char_type) = updates.char_type {
                existing.char_type = char_type;
            }
            if let Some(visual_traits) = updates.visual_traits {
                existing.visual_traits = visual_traits;
            }
            if let Some(style_tokens) = updates.style_tokens {
                existing.style_tokens = style_tokens;
            }
            if let Some(color_palette) = updates.color_palette {
                existing.color_palette = color_palette;
            }
            if let Some(personality) = updates.personality {
                existing.personality = personality;
            }
            if let Some(reference_images) = updates.reference_images {
                existing.reference_images = reference_images;
            }
            if let Some(three_view_images) = updates.three_view_images {
                existing.three_view_images = Some(three_view_images);
            }
            existing.updated_at = Utc::now().to_rfc3339();
            return Some(existing.clone());
        }
        None
    }

    pub fn get_character(&self, id: &str) -> Option<&CharacterBible> {
        self.characters.get(id)
    }

    pub fn get_characters_for_project(&self, project_id: &str) -> Vec<CharacterBible> {
        self.characters
            .values()
            .filter(|c| c.project_id == project_id)
            .cloned()
            .collect()
    }

    pub fn delete_character(&mut self, id: &str) -> bool {
        self.characters.remove(id).is_some()
    }

    pub fn build_character_prompt(&self, character_ids: &[String]) -> String {
        let characters: Vec<&CharacterBible> = character_ids
            .iter()
            .filter_map(|id| self.characters.get(id))
            .collect();

        if characters.is_empty() {
            return String::new();
        }

        characters
            .iter()
            .map(|c| format!("[{}]: {}", c.name, c.visual_traits))
            .collect::<Vec<_>>()
            .join("; ")
    }

    pub fn build_style_tokens(&self, character_ids: &[String]) -> Vec<String> {
        let characters: Vec<&CharacterBible> = character_ids
            .iter()
            .filter_map(|id| self.characters.get(id))
            .collect();

        let mut token_set = std::collections::HashSet::new();
        for c in characters {
            for token in &c.style_tokens {
                token_set.insert(token.clone());
            }
        }

        token_set.into_iter().collect()
    }

    pub fn export_all(&self) -> Vec<CharacterBible> {
        self.characters.values().cloned().collect()
    }

    pub fn import_all(&mut self, characters: Vec<CharacterBible>) {
        self.characters.clear();
        for c in characters {
            self.characters.insert(c.id.clone(), c);
        }
    }

    pub fn clear(&mut self) {
        self.characters.clear();
    }
}

impl Default for CharacterBibleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterBibleUpdate {
    pub name: Option<String>,
    pub char_type: Option<String>,
    pub visual_traits: Option<String>,
    pub style_tokens: Option<Vec<String>>,
    pub color_palette: Option<Vec<String>>,
    pub personality: Option<String>,
    pub reference_images: Option<Vec<ReferenceImage>>,
    pub three_view_images: Option<ThreeViewImages>,
}

pub fn generate_consistency_prompt(character: &CharacterBible) -> String {
    let mut parts = Vec::new();

    if !character.visual_traits.is_empty() {
        parts.push(character.visual_traits.clone());
    }

    if !character.style_tokens.is_empty() {
        parts.push(character.style_tokens.join(", "));
    }

    parts.push(format!("character: {}", character.name));

    parts.join(", ")
}

pub fn merge_character_analyses(analyses: Vec<serde_json::Value>) -> PartialCharacterBible {
    if analyses.is_empty() {
        return PartialCharacterBible::default();
    }

    if analyses.len() == 1 {
        return PartialCharacterBible {
            visual_traits: analyses[0]
                .get("visualTraits")
                .and_then(|v| v.as_str())
                .map(String::from),
            style_tokens: analyses[0]
                .get("styleTokens")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                }),
            color_palette: analyses[0]
                .get("colorPalette")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                }),
            personality: analyses[0]
                .get("personality")
                .and_then(|v| v.as_str())
                .map(String::from),
        };
    }

    let visual_traits = analyses
        .iter()
        .filter_map(|a| a.get("visualTraits").and_then(|v| v.as_str()))
        .max_by_key(|s| s.len())
        .map(String::from);

    let mut style_token_set = std::collections::HashSet::new();
    for a in &analyses {
        if let Some(tokens) = a.get("styleTokens").and_then(|v| v.as_array()) {
            for t in tokens {
                if let Some(s) = t.as_str() {
                    style_token_set.insert(s.to_string());
                }
            }
        }
    }

    let mut color_set = std::collections::HashSet::new();
    for a in &analyses {
        if let Some(colors) = a.get("colorPalette").and_then(|v| v.as_array()) {
            for c in colors {
                if let Some(s) = c.as_str() {
                    color_set.insert(s.to_string());
                }
            }
        }
    }

    let personality = analyses
        .iter()
        .find_map(|a| a.get("personality").and_then(|v| v.as_str()))
        .map(String::from);

    PartialCharacterBible {
        visual_traits,
        style_tokens: Some(style_token_set.into_iter().collect()),
        color_palette: Some(color_set.into_iter().collect()),
        personality,
    }
}

#[derive(Debug, Clone, Default)]
pub struct PartialCharacterBible {
    pub visual_traits: Option<String>,
    pub style_tokens: Option<Vec<String>>,
    pub color_palette: Option<Vec<String>>,
    pub personality: Option<String>,
}

#[tauri::command]
pub async fn create_character_bible(
    request: CreateCharacterBibleRequest,
) -> Result<CharacterBible, String> {
    let mut manager = CharacterBibleManager::new();
    Ok(manager.add_character(request))
}

#[tauri::command]
pub async fn get_character_bibles(
    project_id: String,
) -> Result<Vec<CharacterBible>, String> {
    let manager = CharacterBibleManager::new();
    Ok(manager.get_characters_for_project(&project_id))
}

#[tauri::command]
pub async fn update_character_bible(
    id: String,
    updates: CharacterBibleUpdate,
) -> Result<CharacterBible, String> {
    let mut manager = CharacterBibleManager::new();
    manager
        .update_character(&id, updates)
        .ok_or_else(|| "Character not found".to_string())
}

#[tauri::command]
pub async fn delete_character_bible(id: String) -> Result<bool, String> {
    let mut manager = CharacterBibleManager::new();
    Ok(manager.delete_character(&id))
}

#[tauri::command]
pub async fn build_consistency_prompt(
    character_ids: Vec<String>,
) -> Result<String, String> {
    let manager = CharacterBibleManager::new();
    Ok(manager.build_character_prompt(&character_ids))
}

#[tauri::command]
pub async fn get_character_style_tokens(
    character_ids: Vec<String>,
) -> Result<Vec<String>, String> {
    let manager = CharacterBibleManager::new();
    Ok(manager.build_style_tokens(&character_ids))
}
