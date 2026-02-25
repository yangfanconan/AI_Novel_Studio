import React, { useState, useEffect } from "react";
import {
  Sparkles,
  RefreshCw,
  Loader2,
  ChevronDown,
  AlignLeft,
  Film,
  FileText,
  BookOpen,
  Palette,
} from "lucide-react";
import { aiService } from "../services/ai.service";
import { aiGeneratorService } from "../services/api";
import { logger } from "../utils/logger";
import { debugLogger } from "../utils/debugLogger";
import { StoryboardDialog } from "./StoryboardDialog";
import { StoryboardGeneratorDialog } from "./StoryboardGeneratorDialog";
import { ScriptConverterDialog } from "./ScriptConverterDialog";
import { ComicGeneratorDialog } from "./ComicGeneratorDialog";
import { IllustrationGeneratorDialog } from "./IllustrationGeneratorDialog";
import type { Chapter, FormatOptions, FormattedContent, Character } from "../types";

interface AIToolbarProps {
  content: string;
  onInsert: (text: string) => void;
  onRewrite: (text: string) => void;
  disabled?: boolean;
  projectId?: string;
  chapters?: Chapter[];
  currentChapterId?: string;
  characters?: Character[];
  selectedText?: string;
}

export const AIToolbar: React.FC<AIToolbarProps> = ({
  content,
  onInsert,
  onRewrite,
  disabled = false,
  projectId,
  chapters = [],
  currentChapterId,
  characters = [],
  selectedText,
}) => {
  const [models, setModels] = useState<string[]>([]);
  const [selectedModel, setSelectedModel] = useState<string>("");
  const [isContinuing, setIsContinuing] = useState(false);
  const [isRewriting, setIsRewriting] = useState(false);
  const [showMenu, setShowMenu] = useState(false);
  const [isFormatting, setIsFormatting] = useState(false);
  const [showFormatMenu, setShowFormatMenu] = useState(false);
  const [isStoryboardOpen, setIsStoryboardOpen] = useState(false);
  const [isStoryboardGeneratorOpen, setIsStoryboardGeneratorOpen] = useState(false);
  const [isScriptConverterOpen, setIsScriptConverterOpen] = useState(false);
  const [isComicGeneratorOpen, setIsComicGeneratorOpen] = useState(false);
  const [isIllustrationGeneratorOpen, setIsIllustrationGeneratorOpen] = useState(false);
  const [showMultimediaMenu, setShowMultimediaMenu] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadModels();
  }, []);

  const loadModels = async () => {
    debugLogger.info("Loading AI models", {
      component: "AIToolbar",
      feature: "ai-toolbar",
      action: "load_models",
    });
    try {
      const [availableModels, defaultModel] = await Promise.all([
        aiService.getModels(),
        aiService.getDefaultModel(),
      ]);
      setModels(availableModels);
      debugLogger.info("AI models loaded", {
        count: availableModels.length,
        models: availableModels,
        defaultModel,
        component: "AIToolbar",
        feature: "ai-toolbar",
        action: "load_models_success",
      });

      if (availableModels.length > 0 && !selectedModel) {
        if (defaultModel && availableModels.includes(defaultModel)) {
          setSelectedModel(defaultModel);
        } else {
          setSelectedModel(availableModels[0]);
        }
      }
    } catch (error) {
      logger.error("Failed to load AI models", error, { feature: "ai-toolbar" });
      debugLogger.error("Failed to load AI models", error as Error, {
        component: "AIToolbar",
        feature: "ai-toolbar",
      });
    }
  };

  const handleContinue = async () => {
    debugLogger.info("AI Continue button clicked", {
      selectedModel,
      contentLength: content.length,
      isContinuing,
      component: "AIToolbar",
      feature: "ai-toolbar",
      action: "continue_button_clicked",
    });

    if (!selectedModel) {
      debugLogger.warn("No model selected", { component: "AIToolbar", feature: "ai-toolbar" });
      return;
    }

    if (!content.trim()) {
      debugLogger.warn("Content is empty", { component: "AIToolbar", feature: "ai-toolbar" });
      return;
    }

    if (isContinuing) {
      debugLogger.warn("Already continuing", { component: "AIToolbar", feature: "ai-toolbar" });
      return;
    }

    setIsContinuing(true);
    setError(null);
    logger.info("Starting AI continuation", {
      feature: "ai-toolbar",
      data: { model: selectedModel, contentLength: content.length },
    });
    debugLogger.info("Starting AI continuation", {
      model: selectedModel,
      contentLength: content.length,
      component: "AIToolbar",
      feature: "ai-toolbar",
      action: "continue_start",
    });

    try {
      const generated = await aiService.continueNovel({
        model_id: selectedModel,
        context: content,
        instruction: "请续写下一段内容，保持文风和故事连贯性。",
        project_id: projectId,
      });

      debugLogger.info("AI continuation result received", {
        resultLength: generated.length,
        resultPreview: generated.substring(0, 100),
        component: "AIToolbar",
        feature: "ai-toolbar",
        action: "continue_result_received",
      });
      console.log("AI continuation result:", generated);
      onInsert(generated);
      logger.info("AI continuation completed", {
        feature: "ai-toolbar",
        resultLength: generated.length,
      });
      debugLogger.info("AI continuation completed", {
        resultLength: generated.length,
        component: "AIToolbar",
        feature: "ai-toolbar",
        action: "continue_completed",
      });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      const isAuthError =
        errorMessage.includes("401") ||
        errorMessage.includes("Unauthorized") ||
        errorMessage.includes("令牌已过期");

      if (isAuthError) {
        console.warn("API密钥已过期或无效，请在设置中更新");
        setError("API密钥已过期或无效，请检查设置");
        debugLogger.warn("API密钥验证失败", {
          errorMessage,
          component: "AIToolbar",
          feature: "ai-toolbar",
          action: "continue_failed",
        });
      } else {
        console.error("AI continuation error:", errorMessage);
        setError(`生成失败: ${errorMessage}`);
        logger.error("AI continuation failed", error, { feature: "ai-toolbar" });
        debugLogger.error(
          `AI continuation failed: ${errorMessage}`,
          error instanceof Error ? error : new Error(errorMessage),
          {
            errorMessage,
            component: "AIToolbar",
            feature: "ai-toolbar",
            action: "continue_failed",
          }
        );
      }
    } finally {
      setIsContinuing(false);
    }
  };

  const handleRewrite = async () => {
    console.log("AI Rewrite button clicked", {
      selectedModel,
      contentLength: content.length,
      isRewriting,
    });

    if (!selectedModel) {
      console.warn("No model selected");
      return;
    }

    if (!content.trim()) {
      console.warn("Content is empty");
      return;
    }

    if (isRewriting) {
      console.warn("Already rewriting");
      return;
    }

    setIsRewriting(true);
    logger.info("Starting AI rewrite", {
      feature: "ai-toolbar",
      data: { model: selectedModel, contentLength: content.length },
    });

    try {
      const rewritten = await aiService.rewriteContent({
        model_id: selectedModel,
        content,
        instruction: "请优化这段文字，提升文采和表达。",
      });

      console.log("AI rewrite result:", rewritten);
      onRewrite(rewritten);
      logger.info("AI rewrite completed", {
        feature: "ai-toolbar",
        resultLength: rewritten.length,
      });
    } catch (error) {
      console.error("AI rewrite error:", error);
      logger.error("AI rewrite failed", error, { feature: "ai-toolbar" });
    } finally {
      setIsRewriting(false);
    }
  };

  const handleFormat = async (style: FormatOptions["style"]) => {
    if (!content.trim()) return;

    setIsFormatting(true);
    try {
      const options: FormatOptions = {
        style,
        indent_size: 2,
        line_spacing: "1.5",
        paragraph_spacing: 1,
        preserve_dialogue_format: true,
        auto_punctuate: true,
      };

      const result: FormattedContent = await aiGeneratorService.formatContent(content, options);
      onRewrite(result.formatted_content);
      console.log("Format changes applied:", result.changes_applied);
    } catch (error) {
      console.error("Format error:", error);
    } finally {
      setIsFormatting(false);
      setShowFormatMenu(false);
    }
  };

  return (
    <>
      {error && (
        <div className="px-4 py-2 bg-red-50 border-b border-red-200 flex items-center justify-between">
          <span className="text-sm text-red-600">{error}</span>
          <button onClick={() => setError(null)} className="text-red-400 hover:text-red-600">
            ✕
          </button>
        </div>
      )}
      <div className="flex items-center gap-2 px-4 py-2 border-b border-border bg-card flex-wrap">
        <div className="flex items-center gap-2">
          <Sparkles className="w-4 h-4 text-primary" />
          <span className="text-sm font-medium text-foreground">AI 助手</span>
        </div>

        <div className="h-4 w-px bg-border mx-1" />

        <div className="relative">
          <button
            onClick={() => setShowMenu(!showMenu)}
            disabled={disabled || models.length === 0}
            className="flex items-center gap-1 px-2 py-1 text-sm bg-muted rounded-md hover:bg-muted/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {selectedModel || "选择模型"}
            <ChevronDown className="w-3 h-3" />
          </button>

          {showMenu && models.length > 0 && (
            <>
              <div className="fixed inset-0 z-10" onClick={() => setShowMenu(false)} />
              <div className="absolute top-full left-0 mt-1 w-48 bg-popover border border-border rounded-md shadow-lg z-20 max-h-48 overflow-y-auto">
                {models.map((model) => (
                  <button
                    key={model}
                    onClick={() => {
                      setSelectedModel(model);
                      setShowMenu(false);
                    }}
                    className={`w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors ${
                      selectedModel === model ? "bg-accent" : ""
                    }`}
                  >
                    {model}
                  </button>
                ))}
              </div>
            </>
          )}
        </div>

        <div className="h-4 w-px bg-border mx-1" />

        <button
          onClick={handleContinue}
          disabled={disabled || !selectedModel || !content.trim() || isContinuing || isRewriting}
          className="flex items-center gap-1 px-3 py-1.5 text-sm text-primary-foreground bg-primary rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {isContinuing ? (
            <>
              <Loader2 className="w-3 h-3 animate-spin" />
              生成中...
            </>
          ) : (
            <>
              <Sparkles className="w-3 h-3" />
              AI 续写
            </>
          )}
        </button>

        <button
          onClick={handleRewrite}
          disabled={disabled || !selectedModel || !content.trim() || isContinuing || isRewriting}
          className="flex items-center gap-1 px-3 py-1.5 text-sm bg-muted rounded-md hover:bg-muted/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {isRewriting ? (
            <>
              <Loader2 className="w-3 h-3 animate-spin" />
              优化中...
            </>
          ) : (
            <>
              <RefreshCw className="w-3 h-3" />
              AI 优化
            </>
          )}
        </button>

        <div className="h-4 w-px bg-border mx-1" />

        {/* AI 排版 */}
        <div className="relative">
          <button
            onClick={() => setShowFormatMenu(!showFormatMenu)}
            disabled={disabled || !content.trim() || isFormatting}
            className="flex items-center gap-1 px-3 py-1.5 text-sm bg-muted rounded-md hover:bg-muted/80 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isFormatting ? (
              <>
                <Loader2 className="w-3 h-3 animate-spin" />
                排版中...
              </>
            ) : (
              <>
                <AlignLeft className="w-3 h-3" />
                AI 排版
                <ChevronDown className="w-3 h-3" />
              </>
            )}
          </button>

          {showFormatMenu && (
            <>
              <div className="fixed inset-0 z-10" onClick={() => setShowFormatMenu(false)} />
              <div className="absolute top-full left-0 mt-1 w-36 bg-popover border border-border rounded-md shadow-lg z-20">
                <button
                  onClick={() => handleFormat("standard")}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors"
                >
                  标准格式
                </button>
                <button
                  onClick={() => handleFormat("novel")}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors"
                >
                  小说格式
                </button>
                <button
                  onClick={() => handleFormat("script")}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors"
                >
                  剧本格式
                </button>
                <button
                  onClick={() => handleFormat("poetry")}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors"
                >
                  诗歌格式
                </button>
              </div>
            </>
          )}
        </div>

        {/* 生成分镜 */}
        <button
          onClick={() => setIsStoryboardOpen(true)}
          disabled={disabled || !currentChapterId}
          className="flex items-center gap-1 px-3 py-1.5 text-sm bg-purple-500 text-white rounded-md hover:bg-purple-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          <Film className="w-3 h-3" />
          生成分镜
        </button>

        {/* 多媒体生成 */}
        <div className="relative">
          <button
            onClick={() => setShowMultimediaMenu(!showMultimediaMenu)}
            disabled={disabled || !currentChapterId}
            className="flex items-center gap-1 px-3 py-1.5 text-sm bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-md hover:from-purple-600 hover:to-pink-600 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <Sparkles className="w-3 h-3" />
            多媒体
            <ChevronDown className="w-3 h-3" />
          </button>

          {showMultimediaMenu && (
            <>
              <div className="fixed inset-0 z-10" onClick={() => setShowMultimediaMenu(false)} />
              <div className="absolute top-full left-0 mt-1 w-44 bg-popover border border-border rounded-md shadow-lg z-20">
                <button
                  onClick={() => {
                    setShowMultimediaMenu(false);
                    setIsStoryboardGeneratorOpen(true);
                  }}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors flex items-center gap-2"
                >
                  <Film className="w-4 h-4 text-purple-500" />
                  分镜脚本生成
                </button>
                <button
                  onClick={() => {
                    setShowMultimediaMenu(false);
                    setIsScriptConverterOpen(true);
                  }}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors flex items-center gap-2"
                >
                  <FileText className="w-4 h-4 text-blue-500" />
                  剧本格式转换
                </button>
                <button
                  onClick={() => {
                    setShowMultimediaMenu(false);
                    setIsComicGeneratorOpen(true);
                  }}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors flex items-center gap-2"
                >
                  <BookOpen className="w-4 h-4 text-orange-500" />
                  漫画分镜生成
                </button>
                <button
                  onClick={() => {
                    setShowMultimediaMenu(false);
                    setIsIllustrationGeneratorOpen(true);
                  }}
                  className="w-full px-3 py-2 text-left text-sm hover:bg-accent transition-colors flex items-center gap-2"
                >
                  <Palette className="w-4 h-4 text-pink-500" />
                  插画生成
                </button>
              </div>
            </>
          )}
        </div>
      </div>

      {/* 分镜对话框 */}
      {projectId && (
        <StoryboardDialog
          isOpen={isStoryboardOpen}
          onClose={() => setIsStoryboardOpen(false)}
          projectId={projectId}
          chapters={chapters}
          currentChapterId={currentChapterId}
        />
      )}

      {/* 分镜脚本生成对话框 */}
      {projectId && (
        <StoryboardGeneratorDialog
          isOpen={isStoryboardGeneratorOpen}
          onClose={() => setIsStoryboardGeneratorOpen(false)}
          projectId={projectId}
          chapters={chapters}
          currentChapterId={currentChapterId}
        />
      )}

      {/* 剧本格式转换对话框 */}
      {projectId && (
        <ScriptConverterDialog
          isOpen={isScriptConverterOpen}
          onClose={() => setIsScriptConverterOpen(false)}
          projectId={projectId}
          chapters={chapters}
          currentChapterId={currentChapterId}
        />
      )}

      {/* 漫画分镜生成对话框 */}
      {projectId && (
        <ComicGeneratorDialog
          isOpen={isComicGeneratorOpen}
          onClose={() => setIsComicGeneratorOpen(false)}
          projectId={projectId}
          chapters={chapters}
          currentChapterId={currentChapterId}
        />
      )}

      {/* 插画生成对话框 */}
      {projectId && (
        <IllustrationGeneratorDialog
          isOpen={isIllustrationGeneratorOpen}
          onClose={() => setIsIllustrationGeneratorOpen(false)}
          projectId={projectId}
          chapters={chapters}
          characters={characters}
          currentChapterId={currentChapterId}
          selectedText={selectedText}
        />
      )}
    </>
  );
};
