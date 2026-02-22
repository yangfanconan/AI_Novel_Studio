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
  background?: string;
  skills?: string;
  status?: string;
  bazi?: string;
  ziwei?: string;
  mbti?: string;
  enneagram?: string;
  items?: string;
  avatar_url?: string;
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
  priority?: 'high' | 'medium' | 'low';
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
  style?: 'standard' | 'novel' | 'script' | 'poetry';
  indent_size?: number;
  line_spacing?: 'single' | 'double' | '1.5';
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
