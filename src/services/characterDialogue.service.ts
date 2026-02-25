import { invoke } from '@tauri-apps/api/core';

export interface CharacterInfo {
  id: string;
  name: string;
  role_type?: string;
  personality?: string;
  background?: string;
}

export interface DialogueMessage {
  id: string;
  session_id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  message_type: string;
  character_state?: Record<string, string>;
  emotional_context?: string;
  scene_context?: string;
  tokens_used: number;
  created_at: string;
}

export interface DialogueSettings {
  ai_model: string;
  temperature: number;
  max_tokens: number;
}

export interface DialogueSession {
  id: string;
  character_id: string;
  chapter_id?: string;
  session_name: string;
  system_prompt?: string;
  context_summary?: string;
  messages: DialogueMessage[];
  settings: DialogueSettings;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface DialogueContext {
  character: CharacterInfo;
  conversation_history: DialogueMessage[];
  current_emotion?: string;
  scene_context?: string;
}

export interface DialogueMetadata {
  timestamp: number;
  model: string;
  tokens_used: number;
  generation_time: number;
  quality_score?: number;
}

export interface CharacterDialogue {
  id: string;
  character_id: string;
  user_message: string;
  ai_response: string;
  context: DialogueContext;
  metadata: DialogueMetadata;
}

export interface CreateSessionRequest {
  character_id: string;
  chapter_id?: string;
  session_name: string;
  system_prompt?: string;
  ai_model?: string;
  temperature?: number;
  max_tokens?: number;
}

export interface SendMessageRequest {
  session_id: string;
  user_message: string;
  character_state?: Record<string, string>;
  emotional_context?: string;
  scene_context?: string;
}

export interface UpdateSessionRequest {
  session_id: string;
  session_name?: string;
  system_prompt?: string;
  context_summary?: string;
  ai_model?: string;
  temperature?: number;
  max_tokens?: number;
  is_active?: boolean;
}

export class CharacterDialogueService {
  private static instance: CharacterDialogueService;

  private constructor() {}

  static getInstance(): CharacterDialogueService {
    if (!CharacterDialogueService.instance) {
      CharacterDialogueService.instance = new CharacterDialogueService();
    }
    return CharacterDialogueService.instance;
  }

  async createSession(request: CreateSessionRequest): Promise<DialogueSession> {
    return invoke<DialogueSession>('create_dialogue_session', { request });
  }

  async getSessions(characterId?: string, chapterId?: string): Promise<DialogueSession[]> {
    return invoke<DialogueSession[]>('get_dialogue_sessions', {
      characterId,
      chapterId,
    });
  }

  async getSession(sessionId: string): Promise<DialogueSession> {
    return invoke<DialogueSession>('get_dialogue_session', { sessionId });
  }

  async sendMessage(request: SendMessageRequest): Promise<CharacterDialogue> {
    return invoke<CharacterDialogue>('send_dialogue_message', { request });
  }

  async updateSession(request: UpdateSessionRequest): Promise<DialogueSession> {
    return invoke<DialogueSession>('update_dialogue_session', { request });
  }

  async deleteSession(sessionId: string): Promise<boolean> {
    return invoke<boolean>('delete_dialogue_session', { sessionId });
  }

  async deleteMessage(messageId: string): Promise<boolean> {
    return invoke<boolean>('delete_dialogue_message', { messageId });
  }

  async regenerateResponse(messageId: string): Promise<string> {
    return invoke<string>('regenerate_ai_response', { messageId });
  }

  async createQuickSession(characterId: string, characterName: string): Promise<DialogueSession> {
    return this.createSession({
      character_id: characterId,
      session_name: `${characterName} 对话`,
      system_prompt: `你正在扮演${characterName}这个角色。请根据角色设定，以第一人称的口吻进行对话。`,
    });
  }
}

export const characterDialogueService = CharacterDialogueService.getInstance();
