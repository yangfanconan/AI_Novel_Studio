import { invoke } from "@tauri-apps/api/core";
import type {
  Project,
  CreateProjectRequest,
  Chapter,
  SaveChapterRequest,
  Character,
  CreateCharacterRequest,
  UpdateCharacterRequest,
  PlotPoint,
  CreatePlotPointRequest,
  UpdatePlotPointRequest,
  CreateCharacterRelationRequest,
  UpdateCharacterRelationRequest,
  CharacterGraph,
  WorldView,
  CreateWorldViewRequest,
  UpdateWorldViewRequest,
  GeneratedCharacter,
  GeneratedRelation,
  GeneratedWorldView,
  GeneratedPlotPoint,
  StoryboardScene,
  FormatOptions,
  FormattedContent,
} from "../types";

export const projectService = {
  async createProject(request: CreateProjectRequest): Promise<Project> {
    return await invoke("create_project", { request });
  },

  async getProjects(): Promise<Project[]> {
    return await invoke("get_projects");
  },

  async updateProject(
    id: string,
    name?: string,
    description?: string,
    genre?: string
  ): Promise<Project> {
    return await invoke("update_project", { projectId: id, name, description, genre });
  },

  async deleteProject(id: string): Promise<void> {
    return await invoke("delete_project", { projectId: id });
  },
};

export const chapterService = {
  async saveChapter(request: SaveChapterRequest): Promise<Chapter> {
    return await invoke("save_chapter", { request });
  },

  async getChapters(projectId: string): Promise<Chapter[]> {
    return await invoke("get_chapters", { projectId });
  },

  async deleteChapter(id: string): Promise<void> {
    return await invoke("delete_chapter", { chapterId: id });
  },

  async updateChapter(id: string, title?: string, content?: string): Promise<Chapter> {
    return await invoke("update_chapter", { chapterId: id, title, content });
  },
};

export const characterService = {
  async createCharacter(request: CreateCharacterRequest): Promise<Character> {
    return await invoke("create_character", { request });
  },

  async getCharacters(projectId: string): Promise<Character[]> {
    return await invoke("get_characters", { projectId });
  },

  async updateCharacter(id: string, data: UpdateCharacterRequest): Promise<Character> {
    return await invoke("update_character", { characterId: id, update: data });
  },

  async deleteCharacter(id: string): Promise<void> {
    return await invoke("delete_character", { characterId: id });
  },
};

export const plotPointService = {
  async createPlotPoint(request: CreatePlotPointRequest): Promise<PlotPoint> {
    return await invoke("create_plot_point", { request });
  },

  async getPlotPoints(projectId: string): Promise<PlotPoint[]> {
    return await invoke("get_plot_points", { projectId });
  },

  async updatePlotPoint(request: UpdatePlotPointRequest): Promise<PlotPoint> {
    return await invoke("update_plot_point", { request });
  },

  async deletePlotPoint(id: string): Promise<void> {
    return await invoke("delete_plot_point", { plotPointId: id });
  },
};

export const worldViewService = {
  async createWorldView(request: CreateWorldViewRequest): Promise<WorldView> {
    return await invoke("create_world_view", { request });
  },

  async getWorldViews(projectId: string, category?: string): Promise<WorldView[]> {
    return await invoke("get_world_views", { projectId, category });
  },

  async updateWorldView(id: string, request: UpdateWorldViewRequest): Promise<WorldView> {
    return await invoke("update_world_view", { worldViewId: id, request });
  },

  async deleteWorldView(id: string): Promise<void> {
    return await invoke("delete_world_view", { worldViewId: id });
  },
};

export const relationService = {
  async createRelation(request: CreateCharacterRelationRequest): Promise<void> {
    return await invoke("create_character_relation", { request });
  },

  async getCharacterGraph(projectId: string): Promise<CharacterGraph> {
    return await invoke("get_character_graph", { projectId });
  },

  async updateRelation(id: string, request: UpdateCharacterRelationRequest): Promise<void> {
    return await invoke("update_character_relation", { relationId: id, request });
  },

  async deleteRelation(id: string): Promise<void> {
    return await invoke("delete_character_relation", { relationId: id });
  },
};

export const aiGeneratorService = {
  // AI 生成角色
  async generateCharacter(
    projectId: string,
    options?: { type?: string; description?: string }
  ): Promise<GeneratedCharacter> {
    return await invoke("ai_generate_character", {
      request: {
        project_id: projectId,
        character_type: options?.type,
        description: options?.description,
      },
    });
  },

  // AI 生成角色关系
  async generateCharacterRelations(projectId: string): Promise<GeneratedRelation[]> {
    return await invoke("ai_generate_character_relations", {
      request: { project_id: projectId },
    });
  },

  // AI 生成世界观
  async generateWorldView(projectId: string, category: string): Promise<GeneratedWorldView> {
    return await invoke("ai_generate_worldview", {
      request: { project_id: projectId, category },
    });
  },

  // AI 生成情节点
  async generatePlotPoints(
    projectId: string,
    context?: string,
    direction?: string
  ): Promise<GeneratedPlotPoint[]> {
    return await invoke("ai_generate_plot_points", {
      request: {
        project_id: projectId,
        context,
        direction,
      },
    });
  },

  // 生成分镜提示词
  async generateStoryboard(chapterId?: string, plotPointId?: string): Promise<StoryboardScene[]> {
    return await invoke("ai_generate_storyboard", {
      request: {
        chapter_id: chapterId,
        plot_point_id: plotPointId,
      },
    });
  },

  // AI 一键排版
  async formatContent(content: string, options?: FormatOptions): Promise<FormattedContent> {
    return await invoke("ai_format_content", {
      request: { content, options },
    });
  },
};
