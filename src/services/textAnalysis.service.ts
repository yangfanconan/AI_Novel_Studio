import { invoke } from "@tauri-apps/api/core";

export interface WritingStyleAnalysis {
  avg_sentence_length: number;
  avg_word_length: number;
  vocabulary_richness: number;
  sentence_variety: string[];
  tone: string;
  writing_style_tags: string[];
}

export interface RhythmAnalysis {
  pacing_score: number;
  pacing_segments: PacingSegment[];
  action_vs_description_ratio: number;
  dialogue_ratio: number;
}

export interface PacingSegment {
  start_position: number;
  end_position: number;
  intensity: number;
  segment_type: string;
}

export interface EmotionAnalysis {
  overall_emotion: string;
  emotion_curve: EmotionPoint[];
  emotion_changes: number;
  dominant_emotions: EmotionScore[];
}

export interface EmotionPoint {
  position: number;
  emotion: string;
  intensity: number;
}

export interface EmotionScore {
  emotion: string;
  score: number;
}

export interface ReadabilityAnalysis {
  flesch_score: number;
  reading_level: string;
  avg_sentence_complexity: number;
  syllable_count: number;
  word_count: number;
}

export interface RepetitionDetection {
  repeated_words: RepeatedItem[];
  repeated_phrases: RepeatedItem[];
  repetition_score: number;
}

export interface RepeatedItem {
  text: string;
  count: number;
  positions: number[];
}

export interface LogicCheck {
  logical_issues: LogicIssue[];
  character_consistency_issues: ConsistencyIssue[];
  timeline_issues: TimelineIssue[];
  overall_score: number;
}

export interface LogicIssue {
  position: number;
  issue_type: string;
  description: string;
  severity: string;
}

export interface ConsistencyIssue {
  character_name: string;
  issue_type: string;
  description: string;
  positions: number[];
}

export interface TimelineIssue {
  position: number;
  issue_type: string;
  description: string;
}

export interface FullAnalysis {
  writing_style: WritingStyleAnalysis;
  rhythm: RhythmAnalysis;
  emotion: EmotionAnalysis;
  readability: ReadabilityAnalysis;
  repetitions: RepetitionDetection;
  logic: LogicCheck;
}

class TextAnalysisService {
  async analyzeWritingStyle(text: string): Promise<WritingStyleAnalysis> {
    return await invoke<WritingStyleAnalysis>("analyze_writing_style", { text });
  }

  async analyzeRhythm(text: string): Promise<RhythmAnalysis> {
    return await invoke<RhythmAnalysis>("analyze_rhythm", { text });
  }

  async analyzeEmotion(text: string): Promise<EmotionAnalysis> {
    return await invoke<EmotionAnalysis>("analyze_emotion", { text });
  }

  async analyzeReadability(text: string): Promise<ReadabilityAnalysis> {
    return await invoke<ReadabilityAnalysis>("analyze_readability", { text });
  }

  async detectRepetitions(text: string, minRepetitions = 3): Promise<RepetitionDetection> {
    return await invoke<RepetitionDetection>("detect_repetitions", {
      text,
      minRepetitions,
    });
  }

  async checkLogic(text: string, characters: any[]): Promise<LogicCheck> {
    const charactersJson = JSON.stringify(characters);
    return await invoke<LogicCheck>("check_logic", {
      text,
      charactersJson,
    });
  }

  async runFullAnalysis(text: string, characters?: any[]): Promise<FullAnalysis> {
    const charactersJson = characters ? JSON.stringify(characters) : undefined;
    return await invoke<FullAnalysis>("run_full_analysis", {
      text,
      charactersJson,
    });
  }
}

export const textAnalysisService = new TextAnalysisService();
