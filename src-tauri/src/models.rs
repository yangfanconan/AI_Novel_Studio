use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DebugLogEntry {
    pub timestamp: i64,
    pub level: String,
    pub source: String,
    pub feature: Option<String>,
    pub action: Option<String>,
    pub component: Option<String>,
    pub message: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub stack: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub template: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub template: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub word_count: i32,
    pub sort_order: i32,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub versions: Option<Vec<ChapterVersion>>,
    #[serde(default)]
    pub evaluation: Option<ChapterEvaluation>,
    #[serde(default)]
    pub generation_status: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterVersion {
    pub content: String,
    pub style: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterEvaluation {
    pub score: f32,
    pub coherence: f32,
    pub style_consistency: f32,
    pub character_consistency: f32,
    pub plot_advancement: f32,
    pub summary: String,
    pub suggestions: Vec<String>,
    pub evaluated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateChapterVersionsRequest {
    pub project_id: String,
    pub chapter_id: String,
    pub context: String,
    pub num_versions: Option<i32>,
    pub style: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateChapterRequest {
    pub project_id: String,
    pub chapter_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectChapterVersionRequest {
    pub project_id: String,
    pub chapter_id: String,
    pub version_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveChapterRequest {
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub role_type: Option<String>,
    pub race: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub skills: Option<String>,
    pub status: Option<String>,
    pub bazi: Option<String>,
    pub ziwei: Option<String>,
    pub mbti: Option<String>,
    pub enneagram: Option<String>,
    pub items: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterRequest {
    pub project_id: String,
    pub name: String,
    pub role_type: Option<String>,
    pub race: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub skills: Option<String>,
    pub status: Option<String>,
    pub bazi: Option<String>,
    pub ziwei: Option<String>,
    pub mbti: Option<String>,
    pub enneagram: Option<String>,
    pub items: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterTimelineEvent {
    pub id: String,
    pub character_id: String,
    pub event_type: String,
    pub event_title: String,
    pub event_description: String,
    pub story_time: Option<String>,
    pub real_chapter_id: Option<String>,
    pub emotional_state: Option<String>,
    pub state_changes: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterTimelineEventRequest {
    pub character_id: String,
    pub event_type: String,
    pub event_title: String,
    pub event_description: String,
    pub story_time: Option<String>,
    pub real_chapter_id: Option<String>,
    pub emotional_state: Option<String>,
    pub state_changes: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCharacterTimelineEventRequest {
    pub event_type: Option<String>,
    pub event_title: Option<String>,
    pub event_description: Option<String>,
    pub story_time: Option<String>,
    pub real_chapter_id: Option<String>,
    pub emotional_state: Option<String>,
    pub state_changes: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlotPoint {
    pub id: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub note: Option<String>,
    pub chapter_id: Option<String>,
    pub status: String,
    pub sort_order: i32,
    pub level: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlotPointRequest {
    pub project_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub note: Option<String>,
    pub chapter_id: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePlotPointRequest {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub note: Option<String>,
    pub chapter_id: Option<String>,
    pub status: Option<String>,
    pub sort_order: Option<i32>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldView {
    pub id: String,
    pub project_id: String,
    pub category: String,
    pub title: String,
    pub content: String,
    pub tags: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorldViewRequest {
    pub project_id: String,
    pub category: String,
    pub title: String,
    pub content: String,
    pub tags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorldViewRequest {
    pub id: String,
    pub category: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldViewTimelineEvent {
    pub id: String,
    pub worldview_id: String,
    pub event_type: String,
    pub event_title: String,
    pub event_description: String,
    pub story_time: Option<String>,
    pub impact_scope: Option<String>,
    pub related_characters: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorldViewTimelineEventRequest {
    pub worldview_id: String,
    pub event_type: String,
    pub event_title: String,
    pub event_description: String,
    pub story_time: Option<String>,
    pub impact_scope: Option<String>,
    pub related_characters: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorldViewTimelineEventRequest {
    pub event_type: Option<String>,
    pub event_title: Option<String>,
    pub event_description: Option<String>,
    pub story_time: Option<String>,
    pub impact_scope: Option<String>,
    pub related_characters: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterRelation {
    pub id: String,
    pub project_id: String,
    pub from_character_id: String,
    pub to_character_id: String,
    pub relation_type: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCharacterRelationRequest {
    pub project_id: String,
    pub from_character_id: String,
    pub to_character_id: String,
    pub relation_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCharacterRelationRequest {
    pub id: String,
    pub relation_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterNode {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub label: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterGraph {
    pub nodes: Vec<CharacterNode>,
    pub edges: Vec<CharacterEdge>,
}

// ==================== AI 设置相关 ====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIParams {
    pub temperature: f32,
    pub max_tokens: i32,
    pub top_p: f32,
}

impl Default for AIParams {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 2000,
            top_p: 0.9,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIKeyInfo {
    pub provider: String,
    pub provider_name: String,
    pub is_configured: bool,
    pub masked_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub is_default: bool,
}

// ==================== 剧情节点相关 ====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlotNode {
    pub id: String,
    pub project_id: String,
    pub chapter_id: Option<String>,
    pub parent_node_id: Option<String>,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub choice_made: Option<String>,
    pub characters_involved: Vec<String>,
    pub location: Option<String>,
    pub emotional_tone: Option<String>,
    pub word_count: i32,
    pub is_main_path: bool,
    pub branch_name: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlotNodeRequest {
    pub project_id: String,
    pub chapter_id: Option<String>,
    pub parent_node_id: Option<String>,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub choice_made: Option<String>,
    pub characters_involved: Vec<String>,
    pub location: Option<String>,
    pub emotional_tone: Option<String>,
    pub is_main_path: bool,
    pub branch_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePlotNodeRequest {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub characters_involved: Option<Vec<String>>,
    pub location: Option<String>,
    pub emotional_tone: Option<String>,
    pub is_main_path: Option<bool>,
    pub branch_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlotTree {
    pub nodes: Vec<PlotNode>,
    pub root_nodes: Vec<String>,
}

// ==================== AI 续写选项相关 ====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WritingChoice {
    pub id: String,
    pub direction: String,
    pub direction_icon: String,
    pub preview: String,
    pub hint: String,
    pub characters: Vec<String>,
    pub emotional_tone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WritingSuggestion {
    pub choices: Vec<WritingChoice>,
    pub detected_characters: Vec<String>,
    pub new_characters: Vec<String>,
    pub consistency_warnings: Vec<ConsistencyWarning>,
    pub new_settings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConsistencyWarning {
    pub warning_type: String,
    pub character_name: Option<String>,
    pub expected: String,
    pub actual: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWritingChoicesRequest {
    pub project_id: String,
    pub chapter_id: String,
    pub current_content: String,
    pub model_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateWritingRequest {
    pub project_id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub detected_characters: Vec<DetectedCharacter>,
    pub new_characters: Vec<String>,
    pub consistency_warnings: Vec<ConsistencyWarning>,
    pub detected_settings: Vec<String>,
    pub new_settings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectedCharacter {
    pub name: String,
    pub character_id: Option<String>,
    pub is_new: bool,
    #[serde(default)]
    pub actions: String,
}

// ==================== 知识库相关 ====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeEntry {
    pub id: String,
    pub project_id: String,
    pub entry_type: String,
    pub title: String,
    pub content: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub keywords: Option<String>,
    pub importance: i32,
    pub is_verified: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKnowledgeEntryRequest {
    pub project_id: String,
    pub entry_type: String,
    pub title: String,
    pub content: String,
    pub source_type: Option<String>,
    pub source_id: Option<String>,
    pub keywords: Option<String>,
    pub importance: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateKnowledgeEntryRequest {
    pub id: String,
    pub entry_type: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub keywords: Option<String>,
    pub importance: Option<i32>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeRelation {
    pub id: String,
    pub project_id: String,
    pub from_entry_id: String,
    pub to_entry_id: String,
    pub relation_type: String,
    pub description: Option<String>,
    pub strength: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKnowledgeRelationRequest {
    pub project_id: String,
    pub from_entry_id: String,
    pub to_entry_id: String,
    pub relation_type: String,
    pub description: Option<String>,
    pub strength: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeContext {
    pub project_id: String,
    pub characters_summary: String,
    pub worldview_summary: String,
    pub plot_summary: String,
    pub key_events: Vec<String>,
    pub active_characters: Vec<String>,
    pub current_location: Option<String>,
    pub timeline_context: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildKnowledgeContextRequest {
    pub project_id: String,
    pub chapter_id: Option<String>,
    pub include_characters: Option<bool>,
    pub include_worldview: Option<bool>,
    pub include_plot: Option<bool>,
    pub include_timeline: Option<bool>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KnowledgeSearchResult {
    pub entry: KnowledgeEntry,
    pub relevance_score: f32,
    pub match_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchKnowledgeRequest {
    pub project_id: String,
    pub query: String,
    pub entry_types: Option<Vec<String>>,
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Foreshadowing {
    pub id: String,
    pub project_id: String,
    pub chapter_id: String,
    pub chapter_number: i32,
    pub chapter_title: String,
    pub description: String,
    pub foreshadowing_type: String,
    pub keywords: Vec<String>,
    pub status: Option<String>,
    pub importance: Option<String>,
    pub expected_payoff_chapter: Option<i32>,
    pub actual_payoff_chapter: Option<i32>,
    pub author_note: Option<String>,
    pub ai_confidence: Option<f32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateForeshadowingRequest {
    pub project_id: String,
    pub chapter_id: String,
    pub chapter_number: i32,
    pub chapter_title: String,
    pub description: String,
    pub foreshadowing_type: String,
    pub keywords: Option<Vec<String>>,
    pub importance: Option<String>,
    pub expected_payoff_chapter: Option<i32>,
    pub author_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateForeshadowingRequest {
    pub description: Option<String>,
    pub status: Option<String>,
    pub expected_payoff_chapter: Option<i32>,
    pub actual_payoff_chapter: Option<i32>,
    pub author_note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveForeshadowingRequest {
    pub foreshadowing_id: String,
    pub actual_payoff_chapter: i32,
    pub resolution_text: String,
    pub quality_score: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForeshadowingStats {
    pub total_foreshadowings: i32,
    pub planted_count: i32,
    pub paid_off_count: i32,
    pub overdue_count: i32,
    pub unresolved_count: i32,
    pub abandoned_count: i32,
    pub avg_resolution_distance: f32,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionCurveRequest {
    pub project_id: String,
    pub arc_type: String,
    pub total_chapters: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmotionCurveData {
    pub chapter_number: i32,
    pub chapter_title: String,
    pub position: f32,
    pub phase_name: String,
    pub emotion_target: f32,
    pub emotion_range: (i32, i32),
    pub pacing: String,
    pub thrill_density: f32,
    pub dialogue_ratio: f32,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmotionCurveResponse {
    pub arc_type: String,
    pub total_chapters: i32,
    pub curve_data: Vec<EmotionCurveData>,
    pub overall_stats: EmotionCurveStats,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmotionCurveStats {
    pub avg_emotion: f32,
    pub emotion_variance: f32,
    pub climax_chapters: Vec<i32>,
    pub pacing_balance: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizeChapterRequest {
    pub project_id: String,
    pub chapter_id: String,
    pub dimension: String,
    pub additional_notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizeChapterResponse {
    pub optimized_content: String,
    pub optimization_notes: String,
    pub dimension: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blueprint {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub genre: Option<String>,
    pub target_length: Option<i32>,
    pub characters: Vec<BlueprintCharacter>,
    pub relationships: Vec<BlueprintRelationship>,
    pub settings: Vec<BlueprintSetting>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlueprintCharacter {
    pub name: String,
    pub role: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub arc_type: Option<String>,
    pub is_main_character: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlueprintRelationship {
    pub from: String,
    pub to: String,
    pub relationship_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlueprintSetting {
    pub category: String,
    pub name: String,
    pub description: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlueprintRequest {
    pub project_id: String,
    pub title: String,
    pub genre: Option<String>,
    pub target_length: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBlueprintRequest {
    pub blueprint_id: String,
    pub title: Option<String>,
    pub genre: Option<String>,
    pub target_length: Option<i32>,
    pub characters: Option<Vec<BlueprintCharacter>>,
    pub relationships: Option<Vec<BlueprintRelationship>>,
    pub settings: Option<Vec<BlueprintSetting>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterMission {
    pub id: String,
    pub chapter_id: String,
    pub chapter_number: i32,
    pub macro_beat: String,
    pub micro_beats: Vec<String>,
    pub pov: Option<String>,
    pub tone: Option<String>,
    pub pacing: Option<String>,
    pub allowed_new_characters: Vec<String>,
    pub forbidden_characters: Vec<String>,
    pub beat_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChapterMissionRequest {
    pub chapter_id: String,
    pub chapter_number: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChapterMissionRequest {
    pub mission_id: String,
    pub macro_beat: Option<String>,
    pub micro_beats: Option<Vec<String>>,
    pub pov: Option<String>,
    pub tone: Option<String>,
    pub pacing: Option<String>,
    pub allowed_new_characters: Option<Vec<String>>,
    pub forbidden_characters: Option<Vec<String>>,
    pub beat_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoryBeat {
    pub id: String,
    pub outline_node_id: String,
    pub title: String,
    pub description: String,
    pub chapter_number: i32,
    pub beat_type: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChapterGuardrails {
    pub id: String,
    pub chapter_id: String,
    pub chapter_number: i32,
    pub forbidden_characters: Vec<String>,
    pub forbidden_topics: Vec<String>,
    pub forbidden_emojis: Vec<String>,
    pub min_length: i32,
    pub max_length: i32,
    pub required_beat_completion: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChapterGuardrailsRequest {
    pub chapter_id: String,
    pub chapter_number: i32,
    pub forbidden_characters: Option<Vec<String>>,
    pub forbidden_topics: Option<Vec<String>>,
    pub forbidden_emojis: Option<Vec<String>>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub required_beat_completion: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChapterGuardrailsRequest {
    pub guardrails_id: String,
    pub forbidden_characters: Option<Vec<String>>,
    pub forbidden_topics: Option<Vec<String>>,
    pub forbidden_emojis: Option<Vec<String>>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub required_beat_completion: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckContentAgainstGuardrailsRequest {
    pub chapter_id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuardrailViolation {
    #[serde(rename = "type")]
    pub violation_type: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckContentAgainstGuardrailsResponse {
    pub passed: bool,
    pub violations: Vec<GuardrailViolation>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorChunk {
    pub id: String,
    pub chapter_id: String,
    pub chunk_index: i32,
    pub content: String,
    pub metadata: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorizeChapterRequest {
    pub chapter_id: String,
    pub chunk_size: Option<i32>,
    pub overlap: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorizeChapterResponse {
    pub chunks_created: i32,
    pub chapter_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChunksRequest {
    pub query: String,
    pub chapter_id: Option<String>,
    pub project_id: Option<String>,
    pub top_k: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkSearchResult {
    pub chunk: VectorChunk,
    pub similarity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchChunksResponse {
    pub results: Vec<ChunkSearchResult>,
    pub query: String,
}
