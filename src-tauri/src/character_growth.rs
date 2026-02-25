use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGrowth {
    pub id: String,
    pub character_id: String,
    pub chapter_id: String,
    pub position: i32,
    pub changes: Vec<GrowthChange>,
    pub metadata: GrowthMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthChange {
    pub change_type: GrowthChangeType,
    pub category: String,
    pub description: String,
    pub before: Option<String>,
    pub after: Option<String>,
    pub significance: GrowthSignificance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GrowthChangeType {
    #[serde(rename = "personality")]
    Personality,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "skill")]
    Skill,
    #[serde(rename = "relationship")]
    Relationship,
    #[serde(rename = "knowledge")]
    Knowledge,
    #[serde(rename = "belief")]
    Belief,
    #[serde(rename = "goal")]
    Goal,
    #[serde(rename = "emotion")]
    Emotion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GrowthSignificance {
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "major")]
    Major,
    #[serde(rename = "critical")]
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetadata {
    pub timestamp: i64,
    pub auto_detected: bool,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterGrowthTimeline {
    pub character_id: String,
    pub character_name: String,
    pub timeline: Vec<TimelineEvent>,
    pub summary: GrowthSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub chapter_id: String,
    pub chapter_title: String,
    pub chapter_order: i32,
    pub position: i32,
    pub changes: Vec<GrowthChange>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthSummary {
    pub total_changes: i32,
    pub personality_changes: i32,
    pub status_changes: i32,
    pub skill_changes: i32,
    pub relationship_changes: i32,
    pub major_changes: i32,
    pub critical_changes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthComparison {
    pub from_position: i32,
    pub to_position: i32,
    pub character_id: String,
    pub changes: Vec<GrowthChange>,
    pub analysis: ComparisonAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonAnalysis {
    pub overall_growth: String,
    pub growth_areas: Vec<String>,
    pub stagnation_areas: Vec<String>,
    pub regression_areas: Vec<String>,
    pub recommendation: String,
}

pub struct CharacterGrowthManager;

impl CharacterGrowthManager {
    pub fn create_growth_record(
        character_id: &str,
        chapter_id: &str,
        position: i32,
        changes: Vec<GrowthChange>,
        auto_detected: bool,
        notes: &str,
    ) -> CharacterGrowth {
        let id = format!("{}_{}_{}", character_id, chapter_id, position);
        let timestamp = chrono::Utc::now().timestamp();

        let metadata = GrowthMetadata {
            timestamp,
            auto_detected,
            notes: notes.to_string(),
        };

        CharacterGrowth {
            id,
            character_id: character_id.to_string(),
            chapter_id: chapter_id.to_string(),
            position,
            changes,
            metadata,
        }
    }

    pub fn build_timeline(
        growth_records: Vec<CharacterGrowth>,
        chapter_info: &HashMap<String, (String, i32)>,
        character_name: &str,
    ) -> CharacterGrowthTimeline {
        let mut timeline: Vec<TimelineEvent> = growth_records
            .iter()
            .filter_map(|record| {
                chapter_info.get(&record.chapter_id).map(|(title, order)| TimelineEvent {
                    chapter_id: record.chapter_id.clone(),
                    chapter_title: title.clone(),
                    chapter_order: *order,
                    position: record.position,
                    changes: record.changes.clone(),
                    timestamp: record.metadata.timestamp,
                })
            })
            .collect();

        timeline.sort_by(|a, b| {
            a.chapter_order.cmp(&b.chapter_order)
                .then_with(|| a.position.cmp(&b.position))
        });

        let summary = Self::calculate_summary(&timeline);

        CharacterGrowthTimeline {
            character_id: growth_records.get(0).map(|r| r.character_id.clone()).unwrap_or_default(),
            character_name: character_name.to_string(),
            timeline,
            summary,
        }
    }

    pub fn compare_growth_positions(
        from_record: &CharacterGrowth,
        to_record: &CharacterGrowth,
    ) -> GrowthComparison {
        let character_id = from_record.character_id.clone();
        let changes = Self::compute_growth_diff(from_record, to_record);

        let analysis = Self::analyze_growth(&changes);

        GrowthComparison {
            from_position: from_record.position,
            to_position: to_record.position,
            character_id,
            changes,
            analysis,
        }
    }

    fn calculate_summary(timeline: &[TimelineEvent]) -> GrowthSummary {
        let mut total_changes = 0;
        let mut personality_changes = 0;
        let mut status_changes = 0;
        let mut skill_changes = 0;
        let mut relationship_changes = 0;
        let mut major_changes = 0;
        let mut critical_changes = 0;

        for event in timeline {
            for change in &event.changes {
                total_changes += 1;
                match change.change_type {
                    GrowthChangeType::Personality => personality_changes += 1,
                    GrowthChangeType::Status => status_changes += 1,
                    GrowthChangeType::Skill => skill_changes += 1,
                    GrowthChangeType::Relationship => relationship_changes += 1,
                    _ => {}
                }

                match change.significance {
                    GrowthSignificance::Major => major_changes += 1,
                    GrowthSignificance::Critical => critical_changes += 1,
                    _ => {}
                }
            }
        }

        GrowthSummary {
            total_changes,
            personality_changes,
            status_changes,
            skill_changes,
            relationship_changes,
            major_changes,
            critical_changes,
        }
    }

    fn compute_growth_diff(
        from_record: &CharacterGrowth,
        to_record: &CharacterGrowth,
    ) -> Vec<GrowthChange> {
        let mut changes = Vec::new();

        for change in &to_record.changes {
            let before = Self::find_before_value(from_record, &change.category);
            let after = change.after.clone();

            if before != after || change.significance == GrowthSignificance::Critical {
                changes.push(GrowthChange {
                    change_type: change.change_type.clone(),
                    category: change.category.clone(),
                    description: change.description.clone(),
                    before,
                    after,
                    significance: change.significance.clone(),
                });
            }
        }

        changes
    }

    fn find_before_value(record: &CharacterGrowth, category: &str) -> Option<String> {
        for change in &record.changes {
            if change.category == category {
                return change.after.clone();
            }
        }
        None
    }

    fn analyze_growth(changes: &[GrowthChange]) -> ComparisonAnalysis {
        let mut growth_areas = Vec::new();
        let mut stagnation_areas = Vec::new();
        let mut regression_areas = Vec::new();

        for change in changes {
            match change.significance {
                GrowthSignificance::Major | GrowthSignificance::Critical => {
                    growth_areas.push(change.category.clone());
                }
                GrowthSignificance::Minor => {
                    stagnation_areas.push(change.category.clone());
                }
                GrowthSignificance::Moderate => {
                    if change.after.is_none() || change.after.as_ref().map(|s| s.is_empty()).unwrap_or(false) {
                        regression_areas.push(change.category.clone());
                    } else {
                        growth_areas.push(change.category.clone());
                    }
                }
            }
        }

        let overall_growth = if changes.is_empty() {
            "无变化".to_string()
        } else if growth_areas.len() > regression_areas.len() {
            "积极成长".to_string()
        } else if regression_areas.len() > growth_areas.len() {
            "负面变化".to_string()
        } else {
            "稳定发展".to_string()
        };

        let recommendation = Self::generate_recommendation(&overall_growth, &growth_areas, &regression_areas);

        ComparisonAnalysis {
            overall_growth,
            growth_areas: Self::deduplicate(&growth_areas),
            stagnation_areas: Self::deduplicate(&stagnation_areas),
            regression_areas: Self::deduplicate(&regression_areas),
            recommendation,
        }
    }

    fn generate_recommendation(
        overall_growth: &str,
        growth_areas: &[String],
        regression_areas: &[String],
    ) -> String {
        match overall_growth {
            "积极成长" => {
                if !growth_areas.is_empty() {
                    format!("角色在 {} 方面有明显成长，继续保持。", growth_areas.join("、"))
                } else {
                    "角色稳步发展，建议增加更多挑战和变化。".to_string()
                }
            }
            "负面变化" => {
                if !regression_areas.is_empty() {
                    format!("角色在 {} 方面有所退步，建议关注和改善。", regression_areas.join("、"))
                } else {
                    "角色面临困难，建议增加支持和帮助情节。".to_string()
                }
            }
            _ => "角色发展平衡，可以考虑引入新的挑战或转折点。".to_string(),
        }
    }

    fn deduplicate(items: &[String]) -> Vec<String> {
        let mut seen = std::collections::HashSet::new();
        items.iter().filter(|x| seen.insert(x.clone())).cloned().collect()
    }
}
