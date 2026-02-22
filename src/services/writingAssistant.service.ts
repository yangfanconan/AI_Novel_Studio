import { invoke } from '@tauri-apps/api/core';
import { logger } from '../utils/logger';
import {
  PlotNode,
  PlotTree,
  WritingSuggestion,
  ValidationResult,
  GenerateWritingChoicesRequest,
  ValidateWritingRequest,
  CreatePlotNodeRequest,
} from '../types/writingAssistant';
import { KnowledgeContext, KnowledgeSearchResult } from '../types';

export const writingAssistantService = {
  async generateWritingChoices(
    request: GenerateWritingChoicesRequest
  ): Promise<WritingSuggestion> {
    const track = logger.trackAction('generateWritingChoices');
    logger.info('Generating writing choices', { feature: 'writing-assistant' });

    try {
      const result = await invoke<WritingSuggestion>('generate_writing_choices', {
        request,
      });

      logger.info('Writing choices generated', {
        feature: 'writing-assistant',
        choiceCount: result.choices.length,
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to generate writing choices', error, {
        feature: 'writing-assistant',
      });
      throw error;
    }
  },

  async validateWriting(request: ValidateWritingRequest): Promise<ValidationResult> {
    const track = logger.trackAction('validateWriting');
    logger.info('Validating writing content', { feature: 'writing-assistant' });

    try {
      const result = await invoke<ValidationResult>('validate_writing', {
        request,
      });

      logger.info('Writing content validated', {
        feature: 'writing-assistant',
        warningCount: result.consistency_warnings.length,
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to validate writing', error, {
        feature: 'writing-assistant',
      });
      throw error;
    }
  },

  async buildKnowledgeContext(
    projectId: string,
    chapterId?: string
  ): Promise<KnowledgeContext> {
    const track = logger.trackAction('buildKnowledgeContext');
    logger.info('Building knowledge context', { feature: 'knowledge' });

    try {
      const result = await invoke<KnowledgeContext>('build_knowledge_context', {
        request: {
          project_id: projectId,
          chapter_id: chapterId,
          include_characters: true,
          include_worldview: true,
          include_plot: true,
          include_timeline: true,
        },
      });

      logger.info('Knowledge context built', {
        feature: 'knowledge',
        activeCharacters: result.active_characters.length,
        keyEvents: result.key_events.length,
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to build knowledge context', error, {
        feature: 'knowledge',
      });
      throw error;
    }
  },

  async searchKnowledge(
    projectId: string,
    query: string,
    entryTypes?: string[]
  ): Promise<KnowledgeSearchResult[]> {
    const track = logger.trackAction('searchKnowledge');
    logger.info('Searching knowledge', { feature: 'knowledge' });

    try {
      const result = await invoke<KnowledgeSearchResult[]>('search_knowledge', {
        request: {
          project_id: projectId,
          query,
          entry_types: entryTypes,
          limit: 10,
        },
      });

      logger.info('Knowledge search completed', {
        feature: 'knowledge',
        resultCount: result.length,
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to search knowledge', error, {
        feature: 'knowledge',
      });
      throw error;
    }
  },

  async createPlotNode(request: CreatePlotNodeRequest): Promise<PlotNode> {
    const track = logger.trackAction('createPlotNode');
    logger.info('Creating plot node', { feature: 'writing-assistant' });

    try {
      const result = await invoke<PlotNode>('create_plot_node', { request });

      logger.info('Plot node created', {
        feature: 'writing-assistant',
        nodeId: result.id,
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to create plot node', error, {
        feature: 'writing-assistant',
      });
      throw error;
    }
  },

  async getPlotTree(projectId: string): Promise<PlotTree> {
    const track = logger.trackAction('getPlotTree');
    logger.info('Getting plot tree', { feature: 'writing-assistant' });

    try {
      const result = await invoke<PlotTree>('get_plot_tree', { projectId });

      logger.info('Plot tree retrieved', {
        feature: 'writing-assistant',
        nodeCount: result.nodes.length,
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to get plot tree', error, {
        feature: 'writing-assistant',
      });
      throw error;
    }
  },

  async deletePlotNode(nodeId: string): Promise<void> {
    const track = logger.trackAction('deletePlotNode');
    logger.info('Deleting plot node', { feature: 'writing-assistant' });

    try {
      await invoke('delete_plot_node', { nodeId });

      logger.info('Plot node deleted', { feature: 'writing-assistant' });
      track();
    } catch (error) {
      logger.error('Failed to delete plot node', error, {
        feature: 'writing-assistant',
      });
      throw error;
    }
  },
};
