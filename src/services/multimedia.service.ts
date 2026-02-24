import { invoke } from '@tauri-apps/api/core';
import { logger } from '../utils/logger';
import {
  StoryboardFormat,
  VisualStyle,
  ScriptFormat,
  ComicPanelLayout
} from '../types/multimedia';
import type {
  Storyboard,
  StoryboardOptions,
  GenerateStoryboardRequest,
  Script,
  ScriptConversionOptions,
  GenerateScriptRequest,
  Comic,
  ComicGenerationOptions,
  GenerateComicRequest,
  Illustration,
  IllustrationOptions,
  GenerateIllustrationRequest,
  ImageGenerationRequest,
  ImageGenerationResult,
  Scene
} from '../types/multimedia';

export const multimediaService = {
  async generateStoryboard(request: GenerateStoryboardRequest): Promise<Storyboard> {
    const track = logger.trackAction('generateStoryboard');
    logger.info('Generating storyboard', {
      feature: 'multimedia-service',
      data: { chapterId: request.chapterId, plotPointId: request.plotPointId }
    });

    try {
      const result = await invoke<Storyboard>('multimedia_generate_storyboard', { request });

      logger.info('Storyboard generated successfully', {
        feature: 'multimedia-service',
        sceneCount: result.scenes?.length || 0
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to generate storyboard', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async generateScript(request: GenerateScriptRequest): Promise<Script> {
    const track = logger.trackAction('generateScript');
    logger.info('Generating script', {
      feature: 'multimedia-service',
      data: { targetFormat: request.options?.targetFormat }
    });

    try {
      const result = await invoke<Script>('multimedia_generate_script', { request });

      logger.info('Script generated successfully', {
        feature: 'multimedia-service',
        sceneCount: result.scenes?.length || 0
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to generate script', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async generateComic(request: GenerateComicRequest): Promise<Comic> {
    const track = logger.trackAction('generateComic');
    logger.info('Generating comic', {
      feature: 'multimedia-service',
      data: { style: request.options?.style }
    });

    try {
      const result = await invoke<Comic>('multimedia_generate_comic', { request });

      logger.info('Comic generated successfully', {
        feature: 'multimedia-service',
        pageCount: result.pages?.length || 0
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to generate comic', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async generateIllustration(request: GenerateIllustrationRequest): Promise<Illustration> {
    const track = logger.trackAction('generateIllustration');
    logger.info('Generating illustration', {
      feature: 'multimedia-service',
      data: { style: request.options?.style }
    });

    try {
      const result = await invoke<Illustration>('multimedia_generate_illustration', { request });

      logger.info('Illustration generated successfully', {
        feature: 'multimedia-service'
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to generate illustration', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async extractScenes(chapterId: string): Promise<Scene[]> {
    const track = logger.trackAction('extractScenes');
    logger.info('Extracting scenes from chapter', {
      feature: 'multimedia-service',
      chapterId
    });

    try {
      const result = await invoke<Scene[]>('multimedia_extract_scenes', { chapterId });

      logger.info('Scenes extracted successfully', {
        feature: 'multimedia-service',
        sceneCount: result.length
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to extract scenes', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async generateImage(request: ImageGenerationRequest): Promise<ImageGenerationResult> {
    const track = logger.trackAction('generateImage');
    logger.info('Generating image', {
      feature: 'multimedia-service',
      data: { style: request.style, aspectRatio: request.aspectRatio }
    });

    try {
      const result = await invoke<ImageGenerationResult>('multimedia_generate_image', { request });

      logger.info('Image generated successfully', {
        feature: 'multimedia-service',
        imageCount: result.images?.length || 0
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to generate image', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async exportStoryboard(storyboardId: string, format: 'json' | 'txt' | 'pdf'): Promise<string> {
    const track = logger.trackAction('exportStoryboard');
    logger.info('Exporting storyboard', {
      feature: 'multimedia-service',
      data: { storyboardId, format }
    });

    try {
      const result = await invoke<string>('multimedia_export_storyboard', {
        storyboardId,
        format
      });

      logger.info('Storyboard exported successfully', {
        feature: 'multimedia-service'
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to export storyboard', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async exportScript(scriptId: string, format: 'json' | 'txt' | 'fountain' | 'pdf'): Promise<string> {
    const track = logger.trackAction('exportScript');
    logger.info('Exporting script', {
      feature: 'multimedia-service',
      data: { scriptId, format }
    });

    try {
      const result = await invoke<string>('multimedia_export_script', {
        scriptId,
        format
      });

      logger.info('Script exported successfully', {
        feature: 'multimedia-service'
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to export script', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  async exportComic(comicId: string, format: 'json' | 'pdf' | 'cbz'): Promise<string> {
    const track = logger.trackAction('exportComic');
    logger.info('Exporting comic', {
      feature: 'multimedia-service',
      data: { comicId, format }
    });

    try {
      const result = await invoke<string>('multimedia_export_comic', {
        comicId,
        format
      });

      logger.info('Comic exported successfully', {
        feature: 'multimedia-service'
      });
      track();
      return result;
    } catch (error) {
      logger.error('Failed to export comic', error, {
        feature: 'multimedia-service'
      });
      throw error;
    }
  },

  getStoryboardDefaults(): Partial<StoryboardOptions> {
    return {
      format: StoryboardFormat.FILM,
      style: VisualStyle.CINEMATIC,
      detailLevel: 'standard',
      includeDialogue: true,
      includeCameraMovement: true,
      includeSoundEffects: true,
      includeVisualPrompts: true
    };
  },

  getScriptDefaults(): ScriptConversionOptions {
    return {
      targetFormat: ScriptFormat.STANDARD,
      includeSceneNumbers: true,
      includeCharacterDescriptions: true,
      dialogueStyle: 'standard',
      includeCameraDirections: false
    };
  },

  getComicDefaults(): ComicGenerationOptions {
    return {
      style: VisualStyle.ANIME,
      pageLayout: ComicPanelLayout.FOUR_GRID,
      panelsPerPage: 4,
      includeCaptions: true,
      includeSoundEffects: true,
      generateImages: false
    };
  },

  getIllustrationDefaults(): IllustrationOptions {
    return {
      style: VisualStyle.CINEMATIC,
      aspectRatio: '16:9',
      quality: 'high',
      includeCharacters: true,
      characterConsistency: false
    };
  }
};
