import { invoke } from '@tauri-apps/api/core';
import { logger } from '../utils/logger';
import type {
  Project,
  CreateProjectRequest,
  Chapter,
  SaveChapterRequest,
  Character,
  CreateCharacterRequest,
  UpdateCharacterRequest,
} from '../types';

export const projectService = {
  async createProject(request: CreateProjectRequest): Promise<Project> {
    const track = logger.trackAction('createProject');
    logger.info('Creating project', { feature: 'project-service', data: { name: request.name } });

    try {
      const result = await invoke<Project>('create_project', { request });
      logger.info('Project created successfully', { feature: 'project-service', projectId: result.id });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to create project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'project-service',
        action: 'createProject',
        data: { request },
      });
      throw error;
    }
  },

  async getProjects(): Promise<Project[]> {
    logger.info('Fetching projects', { feature: 'project-service' });
    try {
      const result = await invoke<Project[]>('get_projects');
      logger.info(`Retrieved ${result.length} projects`, { feature: 'project-service' });
      return result;
    } catch (error) {
      logger.error('Failed to fetch projects', error instanceof Error ? error : new Error(String(error)), {
        feature: 'project-service',
      });
      throw error;
    }
  },

  async getProject(projectId: string): Promise<Project> {
    try {
      const result = await invoke<Project>('get_project', { projectId });
      logger.info('Project retrieved successfully', { feature: 'project-service', projectId });
      return result;
    } catch (error) {
      logger.error('Failed to fetch project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'project-service',
        data: { projectId },
      });
      throw error;
    }
  },

  async deleteProject(projectId: string): Promise<void> {
    try {
      await invoke('delete_project', { projectId });
      logger.info('Project deleted successfully', { feature: 'project-service', projectId });
    } catch (error) {
      logger.error('Failed to delete project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'project-service',
        data: { projectId },
      });
      throw error;
    }
  },

  async renameProject(projectId: string, newName: string): Promise<Project> {
    try {
      const result = await invoke<Project>('update_project', { projectId, name: newName });
      logger.info('Project renamed successfully', { feature: 'project-service', projectId });
      return result;
    } catch (error) {
      logger.error('Failed to rename project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'project-service',
        data: { projectId, newName },
      });
      throw error;
    }
  },

  async updateProject(projectId: string, name: string, description?: string, genre?: string): Promise<Project> {
    try {
      const result = await invoke<Project>('update_project', { projectId, name, description, genre });
      logger.info('Project updated successfully', { feature: 'project-service', projectId });
      return result;
    } catch (error) {
      logger.error('Failed to update project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'project-service',
        data: { projectId, name, description, genre },
      });
      throw error;
    }
  },
};

export const chapterService = {
  async saveChapter(request: SaveChapterRequest): Promise<Chapter> {
    const track = logger.trackAction('saveChapter');
    logger.info('Saving chapter', {
      feature: 'chapter-service',
      data: { projectId: request.project_id, title: request.title },
    });

    try {
      const result = await invoke<Chapter>('save_chapter', { request });
      logger.info('Chapter saved successfully', { feature: 'chapter-service', chapterId: result.id });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to save chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'chapter-service',
        data: { request },
      });
      throw error;
    }
  },

  async getChapters(projectId: string): Promise<Chapter[]> {
    logger.info('Fetching chapters', { feature: 'chapter-service', projectId });
    try {
      const result = await invoke<Chapter[]>('get_chapters', { projectId });
      logger.info(`Retrieved ${result.length} chapters`, { feature: 'chapter-service', projectId });
      return result;
    } catch (error) {
      logger.error('Failed to fetch chapters', error instanceof Error ? error : new Error(String(error)), {
        feature: 'chapter-service',
        data: { projectId },
      });
      throw error;
    }
  },

  async getChapter(chapterId: string): Promise<Chapter> {
    try {
      const result = await invoke<Chapter>('get_chapter', { chapterId });
      logger.info('Chapter retrieved successfully', { feature: 'chapter-service', chapterId });
      return result;
    } catch (error) {
      logger.error('Failed to fetch chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'chapter-service',
        data: { chapterId },
      });
      throw error;
    }
  },

  async deleteChapter(chapterId: string): Promise<void> {
    try {
      await invoke('delete_chapter', { chapterId });
      logger.info('Chapter deleted successfully', { feature: 'chapter-service', chapterId });
    } catch (error) {
      logger.error('Failed to delete chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'chapter-service',
        data: { chapterId },
      });
      throw error;
    }
  },

  async updateChapter(chapterId: string, title?: string, content?: string): Promise<Chapter> {
    try {
      const result = await invoke<Chapter>('update_chapter', { chapterId, title, content });
      logger.info('Chapter updated successfully', { feature: 'chapter-service', chapterId });
      return result;
    } catch (error) {
      logger.error('Failed to update chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'chapter-service',
        data: { chapterId, title, content },
      });
      throw error;
    }
  },
};

export const characterService = {
  async createCharacter(request: CreateCharacterRequest): Promise<Character> {
    const track = logger.trackAction('createCharacter');
    logger.info('Creating character', {
      feature: 'character-service',
      data: { projectId: request.project_id, name: request.name },
    });

    try {
      const result = await invoke<Character>('create_character', { request });
      logger.info('Character created successfully', { feature: 'character-service', characterId: result.id });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to create character', error instanceof Error ? error : new Error(String(error)), {
        feature: 'character-service',
        data: { request },
      });
      throw error;
    }
  },

  async getCharacters(projectId: string): Promise<Character[]> {
    logger.info('Fetching characters', { feature: 'character-service', projectId });
    try {
      const result = await invoke<Character[]>('get_characters', { projectId });
      logger.info(`Retrieved ${result.length} characters`, { feature: 'character-service', projectId });
      return result;
    } catch (error) {
      logger.error('Failed to fetch characters', error instanceof Error ? error : new Error(String(error)), {
        feature: 'character-service',
        data: { projectId },
      });
      throw error;
    }
  },

  async updateCharacter(characterId: string, update: UpdateCharacterRequest): Promise<Character> {
    try {
      const result = await invoke<Character>('update_character', { characterId, update });
      logger.info('Character updated successfully', { feature: 'character-service', characterId });
      return result;
    } catch (error) {
      logger.error('Failed to update character', error instanceof Error ? error : new Error(String(error)), {
        feature: 'character-service',
        data: { characterId, update },
      });
      throw error;
    }
  },

  async deleteCharacter(characterId: string): Promise<void> {
    try {
      await invoke('delete_character', { characterId });
      logger.info('Character deleted successfully', { feature: 'character-service', characterId });
    } catch (error) {
      logger.error('Failed to delete character', error instanceof Error ? error : new Error(String(error)), {
        feature: 'character-service',
        data: { characterId },
      });
      throw error;
    }
  },
};
