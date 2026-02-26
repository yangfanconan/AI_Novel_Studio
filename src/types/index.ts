export interface Project {
  id: string;
  name: string;
  description?: string;
  genre?: string;
  template?: string;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface CreateProjectRequest {
  name: string;
  description?: string;
  genre?: string;
  template?: string;
}

export interface Chapter {
  id: string;
  project_id: string;
  title: string;
  content: string;
  word_count: number;
  sort_order: number;
  status: string;
  created_at: string;
  updated_at: string;
  versions?: ChapterVersion[];
  evaluation?: ChapterEvaluation;
  generation_status?: string;
}

export interface ChapterVersion {
  content: string;
  style: string;
  created_at?: string;
}

export interface ChapterEvaluation {
  score: number;
  coherence: number;
  style_consistency: number;
  character_consistency: number;
  plot_advancement: number;
  summary: string;
  suggestions: string[];
  evaluated_at: string;
}

export interface GenerateChapterVersionsRequest {
  project_id: string;
  chapter_id: string;
  context: string;
  num_versions?: number;
  style?: string;
}

export interface EvaluateChapterRequest {
  project_id: string;
  chapter_id: string;
}

export interface SelectChapterVersionRequest {
  project_id: string;
  chapter_id: string;
  version_index: number;
}

export interface SaveChapterRequest {
  project_id: string;
  title: string;
  content: string;
  sort_order?: number;
}

export interface Character {
  id: string;
  project_id: string;
  name: string;
  role_type?: string;
  race?: string;
  age?: number;
  gender?: string;
  birth_date?: string;
  appearance?: string;
  personality?: string;
  description?: string;
  tags?: string[];
  background?: string;
  skills?: string;
  status?: string;
  bazi?: string;
  ziwei?: string;
  mbti?: string;
  enneagram?: string;
  items?: string;
  avatar_url?: string;
  relationships?: CharacterRelation[];
  created_at: string;
  updated_at: string;
}

export interface CreateCharacterRequest {
  project_id: string;
  name: string;
  role_type?: string;
  race?: string;
  age?: number;
  gender?: string;
  birth_date?: string;
  appearance?: string;
  personality?: string;
  background?: string;
  skills?: string;
  status?: string;
  bazi?: string;
  ziwei?: string;
  mbti?: string;
  enneagram?: string;
  items?: string;
}

export interface CharacterTimelineEvent {
  id: string;
  character_id: string;
  event_type: string;
  event_title: string;
  event_description: string;
  story_time?: string;
  real_chapter_id?: string;
  emotional_state?: string;
  state_changes?: string;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface CreateCharacterTimelineEventRequest {
  character_id: string;
  event_type: string;
  event_title: string;
  event_description: string;
  story_time?: string;
  real_chapter_id?: string;
  emotional_state?: string;
  state_changes?: string;
  sort_order?: number;
}

export interface UpdateCharacterTimelineEventRequest {
  event_type?: string;
  event_title?: string;
  event_description?: string;
  story_time?: string;
  real_chapter_id?: string;
  emotional_state?: string;
  state_changes?: string;
  sort_order?: number;
}

export interface UpdateCharacterRequest {
  name?: string;
  role_type?: string;
  race?: string;
  age?: number;
  gender?: string;
  birth_date?: string;
  appearance?: string;
  personality?: string;
  background?: string;
  skills?: string;
  status?: string;
  bazi?: string;
  ziwei?: string;
  mbti?: string;
  enneagram?: string;
  items?: string;
}

export interface PlotPoint {
  id: string;
  project_id: string;
  parent_id: string | null;
  title: string;
  description: string | null;
  note: string | null;
  chapter_id: string | null;
  status: string;
  sort_order: number;
  level: number;
  created_at: string;
  updated_at: string;
}

export interface CreatePlotPointRequest {
  project_id: string;
  parent_id?: string;
  title: string;
  description?: string;
  note?: string;
  chapter_id?: string;
  sort_order?: number;
}

export interface UpdatePlotPointRequest {
  id: string;
  title?: string;
  description?: string;
  note?: string;
  chapter_id?: string;
  status?: string;
  sort_order?: number;
  parent_id?: string;
}

export interface PlotPointNode extends PlotPoint {
  children: PlotPointNode[];
}

export interface WorldView {
  id: string;
  project_id: string;
  category: string;
  title: string;
  content: string;
  tags: string | null;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface CreateWorldViewRequest {
  project_id: string;
  category: string;
  title: string;
  content: string;
  tags?: string;
}

export interface UpdateWorldViewRequest {
  id: string;
  category?: string;
  title?: string;
  content?: string;
  tags?: string;
  status?: string;
}

export interface WorldViewTimelineEvent {
  id: string;
  worldview_id: string;
  event_type: string;
  event_title: string;
  event_description: string;
  story_time?: string;
  impact_scope?: string;
  related_characters?: string;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface CreateWorldViewTimelineEventRequest {
  worldview_id: string;
  event_type: string;
  event_title: string;
  event_description: string;
  story_time?: string;
  impact_scope?: string;
  related_characters?: string;
  sort_order?: number;
}

export interface UpdateWorldViewTimelineEventRequest {
  event_type?: string;
  event_title?: string;
  event_description?: string;
  story_time?: string;
  impact_scope?: string;
  related_characters?: string;
  sort_order?: number;
}

export interface CharacterRelation {
  id: string;
  project_id: string;
  from_character_id: string;
  to_character_id: string;
  relation_type: string;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateCharacterRelationRequest {
  project_id: string;
  from_character_id: string;
  to_character_id: string;
  relation_type: string;
  description?: string;
}

export interface UpdateCharacterRelationRequest {
  id: string;
  relation_type?: string;
  description?: string;
}

export interface CharacterNode {
  id: string;
  name: string;
  avatar_url: string | null;
}

export interface CharacterEdge {
  id: string;
  from: string;
  to: string;
  label: string;
  description: string | null;
}

export interface CharacterGraph {
  nodes: CharacterNode[];
  edges: CharacterEdge[];
}

// ============ AI 生成相关类型 ============

export interface GeneratedCharacter {
  name: string;
  role_type?: string;
  race?: string;
  age?: number;
  gender?: string;
  birth_date?: string;
  appearance?: string;
  personality?: string;
  background?: string;
  mbti?: string;
  enneagram?: string;
  bazi?: string;
  ziwei?: string;
  skills?: string;
  status?: string;
  items?: string;
  tags?: string[];
}

export interface GeneratedRelation {
  from_character_name: string;
  to_character_name: string;
  relation_type: string;
  description?: string;
}

export interface GeneratedWorldView {
  category: string;
  title: string;
  content: string;
  tags?: string[];
}

export interface GeneratedPlotPoint {
  title: string;
  description?: string;
  note?: string;
  suggested_chapter?: string;
  priority?: "high" | "medium" | "low";
}

export interface StoryboardScene {
  scene_number: number;
  description: string;
  camera_angle?: string;
  lighting?: string;
  mood?: string;
  character_actions?: string[];
  dialogue?: string;
  notes?: string;
  visual_prompt?: string;
}

export interface FormatOptions {
  style?: "standard" | "novel" | "script" | "poetry";
  indent_size?: number;
  line_spacing?: "single" | "double" | "1.5";
  paragraph_spacing?: number;
  preserve_dialogue_format?: boolean;
  auto_punctuate?: boolean;
}

export interface FormattedContent {
  original_content: string;
  formatted_content: string;
  word_count: number;
  paragraph_count: number;
  changes_applied: string[];
}

// ==================== 知识库相关 ====================

export interface KnowledgeEntry {
  id: string;
  project_id: string;
  entry_type: string;
  title: string;
  content: string;
  source_type: string;
  source_id?: string;
  keywords?: string;
  importance: number;
  is_verified: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateKnowledgeEntryRequest {
  project_id: string;
  entry_type: string;
  title: string;
  content: string;
  source_type?: string;
  source_id?: string;
  keywords?: string;
  importance?: number;
}

export interface UpdateKnowledgeEntryRequest {
  id: string;
  entry_type?: string;
  title?: string;
  content?: string;
  keywords?: string;
  importance?: number;
  is_verified?: boolean;
}

export interface KnowledgeRelation {
  id: string;
  project_id: string;
  from_entry_id: string;
  to_entry_id: string;
  relation_type: string;
  description?: string;
  strength: number;
  created_at: string;
}

export interface CreateKnowledgeRelationRequest {
  project_id: string;
  from_entry_id: string;
  to_entry_id: string;
  relation_type: string;
  description?: string;
  strength?: number;
}

export interface KnowledgeContext {
  project_id: string;
  characters_summary: string;
  worldview_summary: string;
  plot_summary: string;
  key_events: string[];
  active_characters: string[];
  current_location?: string;
  timeline_context: string;
}

export interface BuildKnowledgeContextRequest {
  project_id: string;
  chapter_id?: string;
  include_characters?: boolean;
  include_worldview?: boolean;
  include_plot?: boolean;
  include_timeline?: boolean;
  max_tokens?: number;
}

export interface KnowledgeSearchResult {
  entry: KnowledgeEntry;
  relevance_score: number;
  match_type: string;
}

export interface SearchKnowledgeRequest {
  project_id: string;
  query: string;
  entry_types?: string[];
  limit?: number;
}

export interface Foreshadowing {
  id: string;
  project_id: string;
  chapter_id: string;
  chapter_number: number;
  chapter_title: string;
  description: string;
  foreshadowing_type: string;
  keywords: string[];
  status: string;
  importance: string;
  expected_payoff_chapter?: number;
  actual_payoff_chapter?: number;
  author_note?: string;
  ai_confidence?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateForeshadowingRequest {
  project_id: string;
  chapter_id: string;
  chapter_number: number;
  chapter_title: string;
  description: string;
  foreshadowing_type: string;
  keywords?: string[];
  importance?: string;
  expected_payoff_chapter?: number;
  author_note?: string;
}

export interface ResolveForeshadowingRequest {
  foreshadowing_id: string;
  actual_payoff_chapter: number;
  resolution_text: string;
  quality_score?: number;
}

export interface ForeshadowingStats {
  total_foreshadowings: number;
  planted_count: number;
  paid_off_count: number;
  overdue_count: number;
  unresolved_count: number;
  abandoned_count: number;
  avg_resolution_distance: number;
  recommendations: string[];
}

export interface EmotionCurveRequest {
  project_id: string;
  arc_type: string;
  total_chapters: number;
}

export interface EmotionCurveData {
  chapter_number: number;
  chapter_title: string;
  position: number;
  phase_name: string;
  emotion_target: number;
  emotion_range: [number, number];
  pacing: string;
  thrill_density: number;
  dialogue_ratio: number;
  recommendations: string[];
}

export interface EmotionCurveResponse {
  arc_type: string;
  total_chapters: number;
  curve_data: EmotionCurveData[];
  overall_stats: EmotionCurveStats;
}

export interface EmotionCurveStats {
  avg_emotion: number;
  emotion_variance: number;
  climax_chapters: number[];
  pacing_balance: number;
}

export interface OptimizeChapterRequest {
  project_id: string;
  chapter_id: string;
  dimension: string;
  additional_notes?: string;
}

export interface OptimizeChapterResponse {
  optimized_content: string;
  optimization_notes: string;
  dimension: string;
}

export interface BlueprintCharacter {
  name: string;
  role?: string;
  personality?: string;
  background?: string;
  arc_type?: string;
  is_main_character: boolean;
}

export interface BlueprintRelationship {
  from: string;
  to: string;
  relationship_type: string;
  description?: string;
}

export interface BlueprintSetting {
  category: string;
  name: string;
  description?: string;
  details?: string;
}

export interface Blueprint {
  id: string;
  project_id: string;
  title: string;
  genre?: string;
  target_length?: number;
  characters: BlueprintCharacter[];
  relationships: BlueprintRelationship[];
  settings: BlueprintSetting[];
  created_at: string;
  updated_at: string;
}

export interface CreateBlueprintRequest {
  project_id: string;
  title: string;
  genre?: string;
  target_length?: number;
}

export interface UpdateBlueprintRequest {
  blueprint_id: string;
  title?: string;
  genre?: string;
  target_length?: number;
  characters?: BlueprintCharacter[];
  relationships?: BlueprintRelationship[];
  settings?: BlueprintSetting[];
}

export interface ChapterMission {
  id: string;
  chapter_id: string;
  chapter_number: number;
  macro_beat: string;
  micro_beats: string[];
  pov?: string;
  tone?: string;
  pacing?: string;
  allowed_new_characters: string[];
  forbidden_characters: string[];
  beat_id?: string;
  selected_beat?: StoryBeat;
  created_at: string;
}

export interface CreateChapterMissionRequest {
  chapter_id: string;
  chapter_number: number;
}

export interface UpdateChapterMissionRequest {
  mission_id: string;
  macro_beat?: string;
  micro_beats?: string[];
  pov?: string;
  tone?: string;
  pacing?: string;
  allowed_new_characters?: string[];
  forbidden_characters?: string[];
  beat_id?: string;
}

export interface StoryBeat {
  id: string;
  outline_node_id: string;
  title: string;
  description: string;
  chapter_number: number;
  beat_type: string;
  status: string;
}

export interface ChapterGuardrails {
  id: string;
  chapter_id: string;
  chapter_number: number;
  forbidden_characters: string[];
  forbidden_topics: string[];
  forbidden_emojis: string[];
  min_length: number;
  max_length: number;
  required_beat_completion: boolean;
  created_at: string;
}

export interface CreateChapterGuardrailsRequest {
  chapter_id: string;
  chapter_number: number;
  forbidden_characters?: string[];
  forbidden_topics?: string[];
  forbidden_emojis?: string[];
  min_length?: number;
  max_length?: number;
  required_beat_completion?: boolean;
}

export interface UpdateChapterGuardrailsRequest {
  guardrails_id: string;
  forbidden_characters?: string[];
  forbidden_topics?: string[];
  forbidden_emojis?: string[];
  min_length?: number;
  max_length?: number;
  required_beat_completion?: boolean;
}

export interface CheckContentAgainstGuardrailsRequest {
  chapter_id: string;
  content: string;
}

export interface GuardrailViolation {
  type: string;
  message: string;
  severity: "error" | "warning" | "info";
}

export interface CheckContentAgainstGuardrailsResponse {
  passed: boolean;
  violations: GuardrailViolation[];
  suggestions: string[];
}

export interface VectorChunk {
  id: string;
  chapter_id: string;
  chunk_index: number;
  content: string;
  metadata: Record<string, any>;
  created_at: string;
}

export interface VectorizeChapterRequest {
  chapter_id: string;
  chunk_size?: number;
  overlap?: number;
}

export interface VectorizeChapterResponse {
  chunks_created: number;
  chapter_id: string;
}

export interface SearchChunksRequest {
  query: string;
  chapter_id?: string;
  project_id?: string;
  top_k?: number;
}

export interface ChunkSearchResult {
  chunk: VectorChunk;
  similarity: number;
}

export interface SearchChunksResponse {
  results: ChunkSearchResult[];
  query: string;
}
