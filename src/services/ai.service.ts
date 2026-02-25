import { invoke } from "@tauri-apps/api/core";
import { logger } from "../utils/logger";
import type {
  AIModelConfig,
  PromptTemplate,
  AICompletionRequest,
  AIRewriteRequest,
} from "../types/ai";

export const aiService = {
  async registerOpenAIModel(config: {
    id: string;
    apiKey: string;
    model: string;
    baseUrl?: string;
  }): Promise<void> {
    const track = logger.trackAction("registerOpenAIModel");
    logger.info("Registering OpenAI model", {
      feature: "ai-service",
      data: { id: config.id, model: config.model },
    });

    try {
      await invoke("register_openai_model", {
        id: config.id,
        apiKey: config.apiKey,
        model: config.model,
        baseUrl: config.baseUrl,
      });

      logger.info("OpenAI model registered successfully", {
        feature: "ai-service",
        modelId: config.id,
      });
      track();
    } catch (error) {
      logger.error("Failed to register OpenAI model", error, {
        feature: "ai-service",
        modelId: config.id,
      });
      throw error;
    }
  },

  async registerOllamaModel(config: {
    id: string;
    model: string;
    baseUrl?: string;
  }): Promise<void> {
    const track = logger.trackAction("registerOllamaModel");
    logger.info("Registering Ollama model", {
      feature: "ai-service",
      data: { id: config.id, model: config.model },
    });

    try {
      await invoke("register_ollama_model", {
        id: config.id,
        model: config.model,
        baseUrl: config.baseUrl,
      });

      logger.info("Ollama model registered successfully", {
        feature: "ai-service",
        modelId: config.id,
      });
      track();
    } catch (error) {
      logger.error("Failed to register Ollama model", error, {
        feature: "ai-service",
        modelId: config.id,
      });
      throw error;
    }
  },

  async getModels(): Promise<string[]> {
    const track = logger.trackAction("getModels");
    logger.info("Getting available models", { feature: "ai-service" });

    try {
      const models = await invoke<string[]>("get_models");

      logger.info("Models retrieved successfully", {
        feature: "ai-service",
        count: models.length,
      });
      track();
      return models;
    } catch (error) {
      logger.error("Failed to get models", error, { feature: "ai-service" });
      throw error;
    }
  },

  async getDefaultModel(): Promise<string | null> {
    const track = logger.trackAction("getDefaultModel");
    logger.info("Getting default model", { feature: "ai-service" });

    try {
      const defaultModel = await invoke<string | null>("get_default_model");

      logger.info("Default model retrieved", {
        feature: "ai-service",
        defaultModel,
      });
      track();
      return defaultModel;
    } catch (error) {
      logger.error("Failed to get default model", error, { feature: "ai-service" });
      return null;
    }
  },

  async continueNovel(request: AICompletionRequest): Promise<string> {
    const track = logger.trackAction("continueNovel");
    logger.info("Starting novel continuation", {
      feature: "ai-service",
      data: { model_id: request.model_id },
    });

    try {
      const result = await invoke<string>("ai_continue_novel", { request });

      logger.info("Novel continuation completed", {
        feature: "ai-service",
        resultLength: result.length,
      });
      track();
      return result;
    } catch (error) {
      logger.error("Failed to continue novel", error, {
        feature: "ai-service",
        model_id: request.model_id,
      });
      throw error;
    }
  },

  async rewriteContent(request: AIRewriteRequest): Promise<string> {
    const track = logger.trackAction("rewriteContent");
    logger.info("Starting content rewrite", {
      feature: "ai-service",
      data: { model_id: request.model_id },
    });

    try {
      const result = await invoke<string>("ai_rewrite_content", { request });

      logger.info("Content rewrite completed", {
        feature: "ai-service",
        resultLength: result.length,
      });
      track();
      return result;
    } catch (error) {
      logger.error("Failed to rewrite content", error, {
        feature: "ai-service",
        model_id: request.model_id,
      });
      throw error;
    }
  },

  async getPromptTemplates(category?: string): Promise<PromptTemplate[]> {
    const track = logger.trackAction("getPromptTemplates");
    logger.info("Getting prompt templates", {
      feature: "ai-service",
      data: { category },
    });

    try {
      const templates = await invoke<PromptTemplate[]>("get_prompt_templates", { category });

      logger.info("Prompt templates retrieved", {
        feature: "ai-service",
        count: templates.length,
      });
      track();
      return templates;
    } catch (error) {
      logger.error("Failed to get prompt templates", error, { feature: "ai-service" });
      throw error;
    }
  },
};
