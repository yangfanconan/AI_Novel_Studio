use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub id: String,
    pub project_id: String,
    pub version: String,
    pub timestamp: i64,
    pub description: String,
    pub chapters: Vec<ChapterSnapshot>,
    pub characters: Vec<CharacterSnapshot>,
    pub world_views: Vec<WorldViewSnapshot>,
    pub plot_points: Vec<PlotPointSnapshot>,
    pub metadata: SnapshotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterSnapshot {
    pub id: String,
    pub title: String,
    pub content: String,
    pub order: i32,
    pub word_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSnapshot {
    pub id: String,
    pub name: String,
    pub description: String,
    pub personality: String,
    pub appearance: String,
    pub background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldViewSnapshot {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotPointSnapshot {
    pub id: String,
    pub title: String,
    pub content: String,
    pub chapter_id: Option<String>,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub total_words: i32,
    pub total_chapters: i32,
    pub total_characters: i32,
    pub auto_generated: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiff {
    pub from_version: String,
    pub to_version: String,
    pub timestamp: i64,
    pub chapter_changes: Vec<ChapterDiff>,
    pub character_changes: Vec<CharacterDiff>,
    pub world_view_changes: Vec<WorldViewDiff>,
    pub plot_point_changes: Vec<PlotPointDiff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterDiff {
    pub id: String,
    pub action: DiffAction,
    pub changes: Vec<TextChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDiff {
    pub id: String,
    pub name: String,
    pub action: DiffAction,
    pub field_changes: Vec<FieldChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldViewDiff {
    pub id: String,
    pub name: String,
    pub action: DiffAction,
    pub field_changes: Vec<FieldChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotPointDiff {
    pub id: String,
    pub title: String,
    pub action: DiffAction,
    pub field_changes: Vec<FieldChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffAction {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "deleted")]
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChange {
    pub position: i32,
    pub removed: String,
    pub added: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldChange {
    pub field: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfig {
    pub auto_save_enabled: bool,
    pub auto_save_interval_minutes: i32,
    pub max_snapshots_per_project: i32,
    pub compression_enabled: bool,
}

impl Default for VersionControlConfig {
    fn default() -> Self {
        VersionControlConfig {
            auto_save_enabled: true,
            auto_save_interval_minutes: 30,
            max_snapshots_per_project: 50,
            compression_enabled: true,
        }
    }
}

pub struct VersionControlManager;

impl VersionControlManager {
    pub fn create_snapshot(
        project_id: &str,
        version: &str,
        description: &str,
        chapters: Vec<ChapterSnapshot>,
        characters: Vec<CharacterSnapshot>,
        world_views: Vec<WorldViewSnapshot>,
        plot_points: Vec<PlotPointSnapshot>,
        auto_generated: bool,
    ) -> ProjectSnapshot {
        let total_words: i32 = chapters.iter().map(|c| c.word_count).sum();
        let total_chapters = chapters.len() as i32;
        let total_characters = characters.len() as i32;

        let timestamp = chrono::Utc::now().timestamp();

        let metadata = SnapshotMetadata {
            total_words,
            total_chapters,
            total_characters,
            auto_generated,
            tags: Self::generate_tags(&chapters, &characters),
        };

        let id = format!("{}_{}", project_id, timestamp);

        ProjectSnapshot {
            id,
            project_id: project_id.to_string(),
            version: version.to_string(),
            timestamp,
            description: description.to_string(),
            chapters,
            characters,
            world_views,
            plot_points,
            metadata,
        }
    }

    pub fn compare_snapshots(from: &ProjectSnapshot, to: &ProjectSnapshot) -> VersionDiff {
        let chapter_changes = Self::compare_chapters(&from.chapters, &to.chapters);
        let character_changes = Self::compare_characters(&from.characters, &to.characters);
        let world_view_changes = Self::compare_world_views(&from.world_views, &to.world_views);
        let plot_point_changes = Self::compare_plot_points(&from.plot_points, &to.plot_points);

        VersionDiff {
            from_version: from.version.clone(),
            to_version: to.version.clone(),
            timestamp: to.timestamp,
            chapter_changes,
            character_changes,
            world_view_changes,
            plot_point_changes,
        }
    }

    fn compare_chapters(from: &[ChapterSnapshot], to: &[ChapterSnapshot]) -> Vec<ChapterDiff> {
        let mut changes = Vec::new();

        let from_map: HashMap<&str, &ChapterSnapshot> = from.iter().map(|c| (c.id.as_str(), c)).collect();
        let to_map: HashMap<&str, &ChapterSnapshot> = to.iter().map(|c| (c.id.as_str(), c)).collect();

        for (id, chapter) in &to_map {
            if let Some(from_chapter) = from_map.get(id) {
                if chapter.content != from_chapter.content || chapter.title != from_chapter.title {
                    changes.push(ChapterDiff {
                        id: id.to_string(),
                        action: DiffAction::Modified,
                        changes: Self::compute_text_diff(&from_chapter.content, &chapter.content),
                    });
                }
            } else {
                changes.push(ChapterDiff {
                    id: id.to_string(),
                    action: DiffAction::Created,
                    changes: vec![],
                });
            }
        }

        for id in from_map.keys() {
            if !to_map.contains_key(id) {
                changes.push(ChapterDiff {
                    id: id.to_string(),
                    action: DiffAction::Deleted,
                    changes: vec![],
                });
            }
        }

        changes
    }

    fn compare_characters(from: &[CharacterSnapshot], to: &[CharacterSnapshot]) -> Vec<CharacterDiff> {
        let mut changes = Vec::new();

        let from_map: HashMap<&str, &CharacterSnapshot> = from.iter().map(|c| (c.id.as_str(), c)).collect();
        let to_map: HashMap<&str, &CharacterSnapshot> = to.iter().map(|c| (c.id.as_str(), c)).collect();

        for (id, character) in &to_map {
            if let Some(from_char) = from_map.get(id) {
                let field_changes = Self::compare_character_fields(from_char, character);
                if !field_changes.is_empty() {
                    changes.push(CharacterDiff {
                        id: id.to_string(),
                        name: character.name.clone(),
                        action: DiffAction::Modified,
                        field_changes,
                    });
                }
            } else {
                changes.push(CharacterDiff {
                    id: id.to_string(),
                    name: character.name.clone(),
                    action: DiffAction::Created,
                    field_changes: vec![],
                });
            }
        }

        for id in from_map.keys() {
            if !to_map.contains_key(id) {
                changes.push(CharacterDiff {
                    id: id.to_string(),
                    name: from_map.get(id).unwrap().name.clone(),
                    action: DiffAction::Deleted,
                    field_changes: vec![],
                });
            }
        }

        changes
    }

    fn compare_world_views(from: &[WorldViewSnapshot], to: &[WorldViewSnapshot]) -> Vec<WorldViewDiff> {
        let mut changes = Vec::new();

        let from_map: HashMap<&str, &WorldViewSnapshot> = from.iter().map(|w| (w.id.as_str(), w)).collect();
        let to_map: HashMap<&str, &WorldViewSnapshot> = to.iter().map(|w| (w.id.as_str(), w)).collect();

        for (id, world_view) in &to_map {
            if let Some(from_wv) = from_map.get(id) {
                let field_changes = Self::compare_world_view_fields(from_wv, world_view);
                if !field_changes.is_empty() {
                    changes.push(WorldViewDiff {
                        id: id.to_string(),
                        name: world_view.name.clone(),
                        action: DiffAction::Modified,
                        field_changes,
                    });
                }
            } else {
                changes.push(WorldViewDiff {
                    id: id.to_string(),
                    name: world_view.name.clone(),
                    action: DiffAction::Created,
                    field_changes: vec![],
                });
            }
        }

        for id in from_map.keys() {
            if !to_map.contains_key(id) {
                changes.push(WorldViewDiff {
                    id: id.to_string(),
                    name: from_map.get(id).unwrap().name.clone(),
                    action: DiffAction::Deleted,
                    field_changes: vec![],
                });
            }
        }

        changes
    }

    fn compare_plot_points(from: &[PlotPointSnapshot], to: &[PlotPointSnapshot]) -> Vec<PlotPointDiff> {
        let mut changes = Vec::new();

        let from_map: HashMap<&str, &PlotPointSnapshot> = from.iter().map(|p| (p.id.as_str(), p)).collect();
        let to_map: HashMap<&str, &PlotPointSnapshot> = to.iter().map(|p| (p.id.as_str(), p)).collect();

        for (id, plot_point) in &to_map {
            if let Some(from_pp) = from_map.get(id) {
                let field_changes = Self::compare_plot_point_fields(from_pp, plot_point);
                if !field_changes.is_empty() {
                    changes.push(PlotPointDiff {
                        id: id.to_string(),
                        title: plot_point.title.clone(),
                        action: DiffAction::Modified,
                        field_changes,
                    });
                }
            } else {
                changes.push(PlotPointDiff {
                    id: id.to_string(),
                    title: plot_point.title.clone(),
                    action: DiffAction::Created,
                    field_changes: vec![],
                });
            }
        }

        for id in from_map.keys() {
            if !to_map.contains_key(id) {
                changes.push(PlotPointDiff {
                    id: id.to_string(),
                    title: from_map.get(id).unwrap().title.clone(),
                    action: DiffAction::Deleted,
                    field_changes: vec![],
                });
            }
        }

        changes
    }

    fn compare_character_fields(from: &CharacterSnapshot, to: &CharacterSnapshot) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        if from.description != to.description {
            changes.push(FieldChange {
                field: "description".to_string(),
                old_value: Some(from.description.clone()),
                new_value: Some(to.description.clone()),
            });
        }

        if from.personality != to.personality {
            changes.push(FieldChange {
                field: "personality".to_string(),
                old_value: Some(from.personality.clone()),
                new_value: Some(to.personality.clone()),
            });
        }

        if from.appearance != to.appearance {
            changes.push(FieldChange {
                field: "appearance".to_string(),
                old_value: Some(from.appearance.clone()),
                new_value: Some(to.appearance.clone()),
            });
        }

        if from.background != to.background {
            changes.push(FieldChange {
                field: "background".to_string(),
                old_value: Some(from.background.clone()),
                new_value: Some(to.background.clone()),
            });
        }

        changes
    }

    fn compare_world_view_fields(from: &WorldViewSnapshot, to: &WorldViewSnapshot) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        if from.category != to.category {
            changes.push(FieldChange {
                field: "category".to_string(),
                old_value: Some(from.category.clone()),
                new_value: Some(to.category.clone()),
            });
        }

        if from.description != to.description {
            changes.push(FieldChange {
                field: "description".to_string(),
                old_value: Some(from.description.clone()),
                new_value: Some(to.description.clone()),
            });
        }

        changes
    }

    fn compare_plot_point_fields(from: &PlotPointSnapshot, to: &PlotPointSnapshot) -> Vec<FieldChange> {
        let mut changes = Vec::new();

        if from.title != to.title {
            changes.push(FieldChange {
                field: "title".to_string(),
                old_value: Some(from.title.clone()),
                new_value: Some(to.title.clone()),
            });
        }

        if from.content != to.content {
            changes.push(FieldChange {
                field: "content".to_string(),
                old_value: Some(from.content.clone()),
                new_value: Some(to.content.clone()),
            });
        }

        if from.chapter_id != to.chapter_id {
            changes.push(FieldChange {
                field: "chapter_id".to_string(),
                old_value: from.chapter_id.clone(),
                new_value: to.chapter_id.clone(),
            });
        }

        if from.order != to.order {
            changes.push(FieldChange {
                field: "order".to_string(),
                old_value: Some(from.order.to_string()),
                new_value: Some(to.order.to_string()),
            });
        }

        changes
    }

    fn compute_text_diff(from: &str, to: &str) -> Vec<TextChange> {
        let mut changes = Vec::new();
        let from_lines: Vec<&str> = from.lines().collect();
        let to_lines: Vec<&str> = to.lines().collect();

        let max_lines = from_lines.len().max(to_lines.len());

        for i in 0..max_lines {
            let from_line = from_lines.get(i).unwrap_or(&"");
            let to_line = to_lines.get(i).unwrap_or(&"");

            if from_line != to_line {
                changes.push(TextChange {
                    position: i as i32,
                    removed: if from_line.is_empty() { String::new() } else { from_line.to_string() },
                    added: if to_line.is_empty() { String::new() } else { to_line.to_string() },
                });
            }
        }

        changes
    }

    fn generate_tags(chapters: &[ChapterSnapshot], characters: &[CharacterSnapshot]) -> Vec<String> {
        let mut tags = Vec::new();

        let total_words: i32 = chapters.iter().map(|c| c.word_count).sum();
        if total_words < 5000 {
            tags.push("初稿".to_string());
        } else if total_words < 20000 {
            tags.push("短篇小说".to_string());
        } else if total_words < 50000 {
            tags.push("中篇小说".to_string());
        } else {
            tags.push("长篇小说".to_string());
        }

        if characters.len() < 5 {
            tags.push("少角色".to_string());
        } else if characters.len() < 15 {
            tags.push("中等角色数".to_string());
        } else {
            tags.push("多角色".to_string());
        }

        tags
    }
}
