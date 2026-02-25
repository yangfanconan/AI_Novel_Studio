import { invoke } from "@tauri-apps/api/core";

export type GrowthChangeType =
  | "personality"
  | "status"
  | "skill"
  | "relationship"
  | "knowledge"
  | "belief"
  | "goal"
  | "emotion";
export type GrowthSignificance = "minor" | "moderate" | "major" | "critical";

export interface GrowthChange {
  change_type: GrowthChangeType;
  category: string;
  description: string;
  before?: string;
  after?: string;
  significance: GrowthSignificance;
}

export interface GrowthMetadata {
  timestamp: number;
  auto_detected: boolean;
  notes: string;
}

export interface CharacterGrowth {
  id: string;
  character_id: string;
  chapter_id: string;
  position: number;
  changes: GrowthChange[];
  metadata: GrowthMetadata;
}

export interface TimelineEvent {
  chapter_id: string;
  chapter_title: string;
  chapter_order: number;
  position: number;
  changes: GrowthChange[];
  timestamp: number;
}

export interface GrowthSummary {
  total_changes: number;
  personality_changes: number;
  status_changes: number;
  skill_changes: number;
  relationship_changes: number;
  major_changes: number;
  critical_changes: number;
}

export interface CharacterGrowthTimeline {
  character_id: string;
  character_name: string;
  timeline: TimelineEvent[];
  summary: GrowthSummary;
}

export interface ComparisonAnalysis {
  overall_growth: string;
  growth_areas: string[];
  stagnation_areas: string[];
  regression_areas: string[];
  recommendation: string;
}

export interface GrowthComparison {
  from_position: number;
  to_position: number;
  character_id: string;
  changes: GrowthChange[];
  analysis: ComparisonAnalysis;
}

export type TagType =
  | "personality"
  | "role"
  | "skill"
  | "relationship"
  | "trait"
  | "archetype"
  | "custom";
export type TagWeight = "low" | "medium" | "high" | "critical";
export type TagSource = "manual" | "ai_suggested" | "template";

export interface TagMetadata {
  created_at: number;
  updated_at: number;
  auto_assigned: boolean;
  source: TagSource;
}

export interface CharacterTag {
  id: string;
  character_id: string;
  tag_type: TagType;
  name: string;
  value?: string;
  description?: string;
  color: string;
  weight: TagWeight;
  metadata: TagMetadata;
}

export interface TagGroups {
  personality_tags: CharacterTag[];
  role_tags: CharacterTag[];
  skill_tags: CharacterTag[];
  relationship_tags: CharacterTag[];
  trait_tags: CharacterTag[];
  custom_tags: CharacterTag[];
}

export interface CharacterTagCollection {
  character_id: string;
  character_name: string;
  tags: CharacterTag[];
  tag_groups: TagGroups;
}

export interface TagMatchCharacter {
  character_id: string;
  character_name: string;
  matched_tags: string[];
  match_count: number;
}

export interface TagSearchResult {
  tags: CharacterTag[];
  characters: TagMatchCharacter[];
}

export interface PredefinedTag {
  name: string;
  description: string;
  default_color: string;
  default_weight: TagWeight;
}

export interface CharacterTagCategory {
  id: string;
  name: string;
  description: string;
  tag_type: TagType;
  default_tags: string[];
  color_palette: string[];
}

export interface TagLibrary {
  categories: CharacterTagCategory[];
  predefined_tags: Record<string, PredefinedTag[]>;
}

export interface TagStatistics {
  total_tags: number;
  tag_type_distribution: Record<string, number>;
  weight_distribution: Record<string, number>;
  most_used_tags: [string, number][];
  characters_with_tags: number;
}

class CharacterGrowthService {
  async createGrowthRecord(
    characterId: string,
    chapterId: string,
    position: number,
    changes: GrowthChange[],
    notes: string
  ): Promise<CharacterGrowth> {
    return await invoke<CharacterGrowth>("create_growth_record", {
      characterId,
      chapterId,
      position,
      changesJson: JSON.stringify(changes),
      notes,
    });
  }

  async getGrowthTimeline(characterId: string): Promise<CharacterGrowthTimeline> {
    return await invoke<CharacterGrowthTimeline>("get_growth_timeline", { characterId });
  }

  async compareGrowthPositions(
    characterId: string,
    fromPosition: number,
    toPosition: number
  ): Promise<GrowthComparison> {
    return await invoke<GrowthComparison>("compare_growth_positions", {
      characterId,
      fromPosition,
      toPosition,
    });
  }

  async createCharacterTag(
    characterId: string,
    tagType: TagType,
    name: string,
    color: string,
    weight: TagWeight,
    value?: string,
    description?: string,
    autoAssigned = false,
    source: TagSource = "manual"
  ): Promise<CharacterTag> {
    return await invoke<CharacterTag>("create_character_tag", {
      characterId,
      tagTypeJson: JSON.stringify(tagType),
      name,
      color,
      weightJson: JSON.stringify(weight),
      value,
      description,
      autoAssigned,
      sourceJson: JSON.stringify(source),
    });
  }

  async getCharacterTags(characterId: string): Promise<CharacterTagCollection> {
    return await invoke<CharacterTagCollection>("get_character_tags", { characterId });
  }

  async deleteCharacterTag(tagId: string): Promise<{ status: string }> {
    return await invoke<{ status: string }>("delete_character_tag", { tagId });
  }

  async searchTags(
    projectId: string,
    query?: string,
    tagTypes?: TagType[],
    minWeight?: TagWeight
  ): Promise<TagSearchResult> {
    return await invoke<TagSearchResult>("search_tags", {
      projectId,
      query,
      tagTypesJson: tagTypes ? JSON.stringify(tagTypes) : undefined,
      minWeightJson: minWeight ? JSON.stringify(minWeight) : undefined,
    });
  }

  async getTagLibrary(): Promise<TagLibrary> {
    return await invoke<TagLibrary>("get_tag_library");
  }

  async getTagStatistics(projectId: string): Promise<TagStatistics> {
    return await invoke<TagStatistics>("get_tag_statistics", { projectId });
  }
}

export const characterGrowthService = new CharacterGrowthService();
