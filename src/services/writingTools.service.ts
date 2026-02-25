import { invoke } from '@tauri-apps/api/core';

export interface SensitiveWordMatch {
  word: string;
  position: number;
  context: string;
  severity: string;
}

export interface SensitiveWordDetection {
  sensitive_words: SensitiveWordMatch[];
  total_count: number;
  severity: string;
}

export interface TypoMatch {
  original: string;
  correction: string;
  position: number;
  context: string;
}

export interface TypoDetection {
  typos: TypoMatch[];
  total_count: number;
}

export interface GrammarIssue {
  position: number;
  issue_type: string;
  description: string;
  suggestion: string;
}

export interface GrammarCheck {
  grammar_issues: GrammarIssue[];
  total_count: number;
}

export interface FormatChange {
  change_type: string;
  position: number;
  original: string;
  corrected: string;
}

export interface FormatNormalization {
  original: string;
  normalized: string;
  changes: FormatChange[];
}

export interface FullWritingToolsAnalysis {
  sensitive_words: SensitiveWordDetection;
  typos: TypoDetection;
  grammar: GrammarCheck;
  format: FormatNormalization;
}

class WritingToolsService {
  async detectSensitiveWords(text: string): Promise<SensitiveWordDetection> {
    return await invoke<SensitiveWordDetection>('detect_sensitive_words', { text });
  }

  async detectTypos(text: string): Promise<TypoDetection> {
    return await invoke<TypoDetection>('detect_typos', { text });
  }

  async checkGrammar(text: string): Promise<GrammarCheck> {
    return await invoke<GrammarCheck>('check_grammar', { text });
  }

  async normalizeFormat(text: string): Promise<FormatNormalization> {
    return await invoke<FormatNormalization>('normalize_format', { text });
  }

  async runFullWritingTools(text: string): Promise<FullWritingToolsAnalysis> {
    return await invoke<FullWritingToolsAnalysis>('run_full_writing_tools', { text });
  }
}

export const writingToolsService = new WritingToolsService();
