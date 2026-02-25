use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTag {
    pub id: String,
    pub character_id: String,
    pub tag_type: TagType,
    pub name: String,
    pub value: Option<String>,
    pub description: Option<String>,
    pub color: String,
    pub weight: TagWeight,
    pub metadata: TagMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TagType {
    #[serde(rename = "personality")]
    Personality,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "skill")]
    Skill,
    #[serde(rename = "relationship")]
    Relationship,
    #[serde(rename = "trait")]
    Trait,
    #[serde(rename = "archetype")]
    Archetype,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagWeight {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagMetadata {
    pub created_at: i64,
    pub updated_at: i64,
    pub auto_assigned: bool,
    pub source: TagSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagSource {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "ai_suggested")]
    AiSuggested,
    #[serde(rename = "template")]
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTagCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tag_type: TagType,
    pub default_tags: Vec<String>,
    pub color_palette: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagLibrary {
    pub categories: Vec<CharacterTagCategory>,
    pub predefined_tags: HashMap<String, Vec<PredefinedTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredefinedTag {
    pub name: String,
    pub description: String,
    pub default_color: String,
    pub default_weight: TagWeight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTagCollection {
    pub character_id: String,
    pub character_name: String,
    pub tags: Vec<CharacterTag>,
    pub tag_groups: TagGroups,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagGroups {
    pub personality_tags: Vec<CharacterTag>,
    pub role_tags: Vec<CharacterTag>,
    pub skill_tags: Vec<CharacterTag>,
    pub relationship_tags: Vec<CharacterTag>,
    pub trait_tags: Vec<CharacterTag>,
    pub custom_tags: Vec<CharacterTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSearchResult {
    pub tags: Vec<CharacterTag>,
    pub characters: Vec<TagMatchCharacter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagMatchCharacter {
    pub character_id: String,
    pub character_name: String,
    pub matched_tags: Vec<String>,
    pub match_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagStatistics {
    pub total_tags: i32,
    pub tag_type_distribution: HashMap<String, i32>,
    pub weight_distribution: HashMap<String, i32>,
    pub most_used_tags: Vec<(String, i32)>,
    pub characters_with_tags: i32,
}

pub struct CharacterTagManager;

impl CharacterTagManager {
    pub fn create_tag(
        character_id: &str,
        tag_type: TagType,
        name: &str,
        value: Option<&str>,
        description: Option<&str>,
        color: &str,
        weight: TagWeight,
        auto_assigned: bool,
        source: TagSource,
    ) -> CharacterTag {
        let id = format!("{}_{}_{}", character_id, name, chrono::Utc::now().timestamp());
        let timestamp = chrono::Utc::now().timestamp();

        let metadata = TagMetadata {
            created_at: timestamp,
            updated_at: timestamp,
            auto_assigned,
            source,
        };

        CharacterTag {
            id,
            character_id: character_id.to_string(),
            tag_type,
            name: name.to_string(),
            value: value.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            color: color.to_string(),
            weight,
            metadata,
        }
    }

    pub fn organize_tags(tags: Vec<CharacterTag>) -> TagGroups {
        let mut personality_tags = Vec::new();
        let mut role_tags = Vec::new();
        let mut skill_tags = Vec::new();
        let mut relationship_tags = Vec::new();
        let mut trait_tags = Vec::new();
        let mut custom_tags = Vec::new();

        for tag in tags {
            match tag.tag_type {
                TagType::Personality => personality_tags.push(tag),
                TagType::Role => role_tags.push(tag),
                TagType::Skill => skill_tags.push(tag),
                TagType::Relationship => relationship_tags.push(tag),
                TagType::Trait => trait_tags.push(tag),
                TagType::Archetype => trait_tags.push(tag),
                TagType::Custom => custom_tags.push(tag),
            }
        }

        TagGroups {
            personality_tags,
            role_tags,
            skill_tags,
            relationship_tags,
            trait_tags,
            custom_tags,
        }
    }

    pub fn search_tags(
        all_tags: Vec<CharacterTag>,
        characters: &HashMap<String, String>,
        query: Option<&str>,
        tag_types: Option<Vec<TagType>>,
        min_weight: Option<TagWeight>,
    ) -> TagSearchResult {
        let mut filtered_tags: Vec<CharacterTag> = all_tags;

        if let Some(q) = query {
            let q_lower = q.to_lowercase();
            let mut result = Vec::new();
            for tag in filtered_tags {
                if tag.name.to_lowercase().contains(&q_lower)
                    || tag.description.as_ref().map(|d| d.to_lowercase().contains(&q_lower)).unwrap_or(false)
                {
                    result.push(tag);
                }
            }
            filtered_tags = result;
        }

        if let Some(types) = tag_types {
            let type_set: HashSet<_> = types.iter().collect();
            let mut result = Vec::new();
            for tag in filtered_tags {
                if type_set.contains(&tag.tag_type) {
                    result.push(tag);
                }
            }
            filtered_tags = result;
        }

        if let Some(min_w) = min_weight {
            let mut result = Vec::new();
            for tag in filtered_tags {
                let include = match (&tag.weight, &min_w) {
                    (TagWeight::Critical, _) => true,
                    (TagWeight::High, TagWeight::Low | TagWeight::Medium) => true,
                    (TagWeight::Medium, TagWeight::Low) => true,
                    (TagWeight::Low, TagWeight::Low) => true,
                    _ => false,
                };
                if include {
                    result.push(tag);
                }
            }
            filtered_tags = result;
        }

        let mut character_matches: HashMap<String, Vec<String>> = HashMap::new();
        for tag in &filtered_tags {
            character_matches
                .entry(tag.character_id.clone())
                .or_insert_with(Vec::new)
                .push(tag.name.clone());
        }

        let mut characters_result = Vec::new();
        for (character_id, matched_tags) in character_matches {
            let match_count = matched_tags.len() as i32;
            if let Some(character_name) = characters.get(&character_id) {
                characters_result.push(TagMatchCharacter {
                    character_id,
                    character_name: character_name.clone(),
                    matched_tags,
                    match_count,
                });
            }
        }

        characters_result.sort_by(|a, b| b.match_count.cmp(&a.match_count));

        TagSearchResult {
            tags: filtered_tags,
            characters: characters_result,
        }
    }

    pub fn calculate_statistics(tags: Vec<CharacterTag>, _character_count: i32) -> TagStatistics {
        let total_tags = tags.len() as i32;
        let mut tag_type_distribution: HashMap<String, i32> = HashMap::new();
        let mut weight_distribution: HashMap<String, i32> = HashMap::new();
        let mut tag_usage: HashMap<String, i32> = HashMap::new();

        for tag in &tags {
            let type_name = serde_json::to_string(&tag.tag_type).unwrap_or_default();
            *tag_type_distribution.entry(type_name).or_insert(0) += 1;

            let weight_name = serde_json::to_string(&tag.weight).unwrap_or_default();
            *weight_distribution.entry(weight_name).or_insert(0) += 1;

            *tag_usage.entry(tag.name.clone()).or_insert(0) += 1;
        }

        let mut most_used_tags: Vec<(String, i32)> = tag_usage.into_iter().collect();
        most_used_tags.sort_by(|a, b| b.1.cmp(&a.1));
        most_used_tags.truncate(10);

        let characters_with_tags: HashSet<String> = tags.iter().map(|t| t.character_id.clone()).collect();

        TagStatistics {
            total_tags,
            tag_type_distribution,
            weight_distribution,
            most_used_tags,
            characters_with_tags: characters_with_tags.len() as i32,
        }
    }

    pub fn get_tag_library() -> TagLibrary {
        let mut categories = Vec::new();
        let mut predefined_tags = HashMap::new();

        let personality_tags = vec![
            PredefinedTag {
                name: "勇敢".to_string(),
                description: "面对困难时不退缩".to_string(),
                default_color: "#FF6B6B".to_string(),
                default_weight: TagWeight::Medium,
            },
            PredefinedTag {
                name: "善良".to_string(),
                description: "心地善良，乐于助人".to_string(),
                default_color: "#4ECDC4".to_string(),
                default_weight: TagWeight::Medium,
            },
            PredefinedTag {
                name: "聪明".to_string(),
                description: "思维敏捷，智慧过人".to_string(),
                default_color: "#45B7D1".to_string(),
                default_weight: TagWeight::High,
            },
            PredefinedTag {
                name: "狡诈".to_string(),
                description: "心思缜密，善于算计".to_string(),
                default_color: "#96CEB4".to_string(),
                default_weight: TagWeight::High,
            },
            PredefinedTag {
                name: "忧郁".to_string(),
                description: "性格忧郁，多愁善感".to_string(),
                default_color: "#FFEAA7".to_string(),
                default_weight: TagWeight::Medium,
            },
        ];

        let role_tags = vec![
            PredefinedTag {
                name: "主角".to_string(),
                description: "故事的主要人物".to_string(),
                default_color: "#FF6B6B".to_string(),
                default_weight: TagWeight::Critical,
            },
            PredefinedTag {
                name: "配角".to_string(),
                description: "次要角色".to_string(),
                default_color: "#4ECDC4".to_string(),
                default_weight: TagWeight::Medium,
            },
            PredefinedTag {
                name: "反派".to_string(),
                description: "对立面角色".to_string(),
                default_color: "#E74C3C".to_string(),
                default_weight: TagWeight::High,
            },
            PredefinedTag {
                name: "导师".to_string(),
                description: "指导者角色".to_string(),
                default_color: "#9B59B6".to_string(),
                default_weight: TagWeight::High,
            },
        ];

        predefined_tags.insert("personality".to_string(), personality_tags);
        predefined_tags.insert("role".to_string(), role_tags);

        categories.push(CharacterTagCategory {
            id: "personality".to_string(),
            name: "性格特质".to_string(),
            description: "描述角色的性格特点".to_string(),
            tag_type: TagType::Personality,
            default_tags: vec!["勇敢".to_string(), "善良".to_string(), "聪明".to_string()],
            color_palette: vec![
                "#FF6B6B".to_string(),
                "#4ECDC4".to_string(),
                "#45B7D1".to_string(),
                "#96CEB4".to_string(),
                "#FFEAA7".to_string(),
            ],
        });

        categories.push(CharacterTagCategory {
            id: "role".to_string(),
            name: "角色定位".to_string(),
            description: "角色在故事中的定位".to_string(),
            tag_type: TagType::Role,
            default_tags: vec!["主角".to_string(), "配角".to_string(), "反派".to_string()],
            color_palette: vec![
                "#FF6B6B".to_string(),
                "#4ECDC4".to_string(),
                "#E74C3C".to_string(),
                "#9B59B6".to_string(),
            ],
        });

        TagLibrary {
            categories,
            predefined_tags,
        }
    }
}
