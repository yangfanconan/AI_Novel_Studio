import { invoke } from '@tauri-apps/api/core';

export interface User {
  id: string;
  name: string;
  color: string;
}

export interface CursorPosition {
  user_id: string;
  chapter_id: string;
  line: number;
  column: number;
}

export interface Operation {
  id: string;
  user_id: string;
  chapter_id: string;
  op_type: {
    Insert?: { position: number; text: string };
    Delete?: { position: number; length: number };
    Replace?: { position: number; length: number; text: string };
  };
  timestamp: number;
}

export interface CollaborationSession {
  id: string;
  project_id: string;
  users: User[];
  active_cursors: Record<string, CursorPosition>;
}

class CollaborationService {
  private currentSessionId: string | null = null;
  private currentUserId: string | null = null;
  private currentUserColor: string | null = null;

  async createSession(projectId: string): Promise<string> {
    const sessionId = await invoke<string>('collab_create_session', { projectId });
    this.currentSessionId = sessionId;
    return sessionId;
  }

  async joinSession(sessionId: string, user: User): Promise<void> {
    await invoke('collab_join_session', { sessionId, user });
    this.currentSessionId = sessionId;
  }

  async leaveSession(sessionId: string): Promise<void> {
    await invoke('collab_leave_session', { sessionId, userId: this.currentUserId });
    this.currentSessionId = null;
  }

  async broadcastOperation(sessionId: string, operation: Operation): Promise<void> {
    await invoke('collab_broadcast_operation', { sessionId, operation });
  }

  async updateCursor(sessionId: string, cursor: CursorPosition): Promise<void> {
    await invoke('collab_update_cursor', { sessionId, cursor });
  }

  async getSession(sessionId: string): Promise<CollaborationSession | null> {
    return await invoke<CollaborationSession | null>('collab_get_session', { sessionId });
  }

  async getUserCursors(sessionId: string): Promise<Record<string, CursorPosition>> {
    return await invoke<Record<string, CursorPosition>>('collab_get_user_cursors', { sessionId });
  }

  async generateUserId(): Promise<string> {
    const userId = await invoke<string>('collab_generate_user_id');
    this.currentUserId = userId;
    return userId;
  }

  async generateColor(): Promise<string> {
    const color = await invoke<string>('collab_generate_color');
    this.currentUserColor = color;
    return color;
  }

  getCurrentSessionId(): string | null {
    return this.currentSessionId;
  }

  getCurrentUserId(): string | null {
    return this.currentUserId;
  }

  getCurrentUserColor(): string | null {
    return this.currentUserColor;
  }

  createInsertOperation(
    chapterId: string,
    position: number,
    text: string
  ): Operation {
    return {
      id: `op_${Date.now()}_${Math.random()}`,
      user_id: this.currentUserId || '',
      chapter_id: chapterId,
      op_type: { Insert: { position, text } },
      timestamp: Date.now(),
    };
  }

  createDeleteOperation(
    chapterId: string,
    position: number,
    length: number
  ): Operation {
    return {
      id: `op_${Date.now()}_${Math.random()}`,
      user_id: this.currentUserId || '',
      chapter_id: chapterId,
      op_type: { Delete: { position, length } },
      timestamp: Date.now(),
    };
  }

  createReplaceOperation(
    chapterId: string,
    position: number,
    length: number,
    text: string
  ): Operation {
    return {
      id: `op_${Date.now()}_${Math.random()}`,
      user_id: this.currentUserId || '',
      chapter_id: chapterId,
      op_type: { Replace: { position, length, text } },
      timestamp: Date.now(),
    };
  }
}

export const collaborationService = new CollaborationService();
