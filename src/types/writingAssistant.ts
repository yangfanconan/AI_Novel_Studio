export interface PlotNode {
  id: string;
  project_id: string;
  chapter_id: string | null;
  parent_node_id: string | null;
  title: string;
  summary: string;
  content: string;
  choice_made: string | null;
  characters_involved: string[];
  location: string | null;
  emotional_tone: string | null;
  word_count: number;
  is_main_path: boolean;
  branch_name: string | null;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface PlotTree {
  nodes: PlotNode[];
  root_nodes: string[];
}

export interface WritingChoice {
  id: string;
  direction: string;
  direction_icon: string;
  preview: string;
  hint: string;
  characters: string[];
  emotional_tone: string;
}

export interface ConsistencyWarning {
  warning_type: string;
  character_name: string | null;
  expected: string;
  actual: string;
  severity: 'low' | 'medium' | 'high';
}

export interface WritingSuggestion {
  choices: WritingChoice[];
  detected_characters: string[];
  new_characters: string[];
  consistency_warnings: ConsistencyWarning[];
  new_settings: string[];
}

export interface DetectedCharacter {
  name: string;
  character_id: string | null;
  is_new: boolean;
  actions: string;
}

export interface ValidationResult {
  detected_characters: DetectedCharacter[];
  new_characters: string[];
  consistency_warnings: ConsistencyWarning[];
  detected_settings: string[];
  new_settings: string[];
}

export interface GenerateWritingChoicesRequest {
  project_id: string;
  chapter_id: string;
  current_content: string;
  model_id?: string;
}

export interface ValidateWritingRequest {
  project_id: string;
  content: string;
}

export interface CreatePlotNodeRequest {
  project_id: string;
  chapter_id: string | null;
  parent_node_id: string | null;
  title: string;
  summary: string;
  content: string;
  choice_made: string | null;
  characters_involved: string[];
  location: string | null;
  emotional_tone: string | null;
  is_main_path: boolean;
  branch_name: string | null;
}
