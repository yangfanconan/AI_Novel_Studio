use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseAnalysisResult {
    pub title: String,
    pub summary: String,
    pub total_words: usize,
    pub chapter_count: usize,
    pub characters: Vec<ExtractedCharacter>,
    pub relationships: Vec<ExtractedRelationship>,
    pub worldviews: Vec<ExtractedWorldview>,
    pub plot_points: Vec<ExtractedPlotPoint>,
    pub outline: ExtractedOutline,
    pub style_analysis: StyleAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedCharacter {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub personality: String,
    pub appearance: String,
    pub role: String,
    pub first_appearance: Option<String>,
    pub mention_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedRelationship {
    pub character1: String,
    pub character2: String,
    pub relationship_type: String,
    pub description: String,
    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedWorldview {
    pub name: String,
    pub category: String,
    pub description: String,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedPlotPoint {
    pub chapter_index: usize,
    pub title: String,
    pub description: String,
    pub plot_type: String,
    pub characters_involved: Vec<String>,
    pub importance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedOutline {
    pub arcs: Vec<OutlineArc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutlineArc {
    pub title: String,
    pub start_chapter: usize,
    pub end_chapter: usize,
    pub summary: String,
    pub key_events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleAnalysis {
    pub writing_style: String,
    pub narrative_voice: String,
    pub dialogue_ratio: f32,
    pub description_ratio: f32,
    pub average_sentence_length: f32,
    pub vocabulary_richness: f32,
    pub pacing: String,
    pub tone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReverseAnalysisRequest {
    pub content: String,
    pub title: String,
    pub analysis_depth: AnalysisDepth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisDepth {
    Basic,
    Standard,
    Deep,
}
