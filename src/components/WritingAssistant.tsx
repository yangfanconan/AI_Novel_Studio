import React, { useState, useEffect, useCallback } from "react";
import {
  WritingChoice,
  WritingSuggestion,
  ConsistencyWarning,
  DetectedCharacter,
} from "../types/writingAssistant";
import { writingAssistantService } from "../services/writingAssistant.service";
import { Character, KnowledgeContext } from "../types";
import { characterService } from "../services/api";
import { logger } from "../utils/logger";

interface WritingAssistantProps {
  projectId: string;
  chapterId: string;
  currentContent: string;
  onChoiceSelected: (preview: string) => void;
  onCreateCharacter: (name: string) => void;
  onCreateWorldView?: (title: string) => void;
}

const isPronoun = (name: string): boolean => {
  const pronouns = [
    "我",
    "你",
    "他",
    "她",
    "它",
    "我们",
    "你们",
    "他们",
    "她们",
    "它们",
    "自己",
    "咱们",
    "这",
    "那",
    "谁",
    "哪",
    "什么",
    "怎么",
    "怎样",
    "如何",
  ];
  return pronouns.includes(name.trim()) || name.length <= 1;
};

const WritingAssistant: React.FC<WritingAssistantProps> = ({
  projectId,
  chapterId,
  currentContent,
  onChoiceSelected,
  onCreateCharacter,
  onCreateWorldView,
}) => {
  const [suggestion, setSuggestion] = useState<WritingSuggestion | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [validating, setValidating] = useState(false);
  const [validationResult, setValidationResult] = useState<{
    detected_characters: DetectedCharacter[];
    consistency_warnings: ConsistencyWarning[];
    new_characters: string[];
    new_settings: string[];
  } | null>(null);
  const [characters, setCharacters] = useState<Character[]>([]);
  const [activeTab, setActiveTab] = useState<"choices" | "validation" | "context">("choices");
  const [expandedWarning, setExpandedWarning] = useState<string | null>(null);
  const [knowledgeContext, setKnowledgeContext] = useState<KnowledgeContext | null>(null);
  const [loadingContext, setLoadingContext] = useState(false);

  useEffect(() => {
    loadCharacters();
  }, [projectId]);

  useEffect(() => {
    if (projectId) {
      loadKnowledgeContext();
    }
  }, [projectId, chapterId]);

  const loadCharacters = async () => {
    try {
      const chars = await characterService.getCharacters(projectId);
      setCharacters(chars);
    } catch (error) {
      logger.error("Failed to load characters", error);
    }
  };

  const loadKnowledgeContext = async () => {
    setLoadingContext(true);
    try {
      const context = await writingAssistantService.buildKnowledgeContext(projectId, chapterId);
      setKnowledgeContext(context);
    } catch (error) {
      logger.error("Failed to load knowledge context", error);
    } finally {
      setLoadingContext(false);
    }
  };

  const generateChoices = useCallback(async () => {
    if (!currentContent || currentContent.length < 100) {
      return;
    }

    setLoading(true);
    setError(null);
    try {
      const result = await writingAssistantService.generateWritingChoices({
        project_id: projectId,
        chapter_id: chapterId,
        current_content: currentContent,
      });
      setSuggestion(result);
    } catch (error) {
      logger.error("Failed to generate choices", error);
      const errorMessage = error instanceof Error ? error.message : String(error);
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [projectId, chapterId, currentContent]);

  const validateContent = useCallback(async () => {
    if (!currentContent || currentContent.length < 50) {
      return;
    }

    setValidating(true);
    try {
      const result = await writingAssistantService.validateWriting({
        project_id: projectId,
        content: currentContent,
      });
      setValidationResult(result);
      setActiveTab("validation");
    } catch (error) {
      logger.error("Failed to validate content", error);
    } finally {
      setValidating(false);
    }
  }, [projectId, currentContent]);

  const handleChoiceClick = (choice: WritingChoice) => {
    onChoiceSelected(choice.preview);
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case "high":
        return "text-red-500 bg-red-50 border-red-200";
      case "medium":
        return "text-yellow-600 bg-yellow-50 border-yellow-200";
      default:
        return "text-blue-500 bg-blue-50 border-blue-200";
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity) {
      case "high":
        return "🔴";
      case "medium":
        return "🟡";
      default:
        return "🔵";
    }
  };

  return (
    <div className="w-80 bg-white border-l border-gray-200 flex flex-col h-full">
      <div className="p-4 border-b border-gray-200">
        <h3 className="text-lg font-semibold text-gray-800 flex items-center gap-2">
          <span>🤖</span> AI 写作助手
        </h3>
      </div>

      <div className="flex border-b border-gray-200">
        <button
          className={`flex-1 py-2 text-sm font-medium ${
            activeTab === "choices"
              ? "text-blue-600 border-b-2 border-blue-600"
              : "text-gray-500 hover:text-gray-700"
          }`}
          onClick={() => setActiveTab("choices")}
        >
          续写选项
        </button>
        <button
          className={`flex-1 py-2 text-sm font-medium ${
            activeTab === "validation"
              ? "text-blue-600 border-b-2 border-blue-600"
              : "text-gray-500 hover:text-gray-700"
          }`}
          onClick={() => setActiveTab("validation")}
        >
          一致性检查
          {validationResult && validationResult.consistency_warnings.length > 0 && (
            <span className="ml-1 px-1.5 py-0.5 text-xs bg-red-100 text-red-600 rounded-full">
              {validationResult.consistency_warnings.length}
            </span>
          )}
        </button>
        <button
          className={`flex-1 py-2 text-sm font-medium ${
            activeTab === "context"
              ? "text-blue-600 border-b-2 border-blue-600"
              : "text-gray-500 hover:text-gray-700"
          }`}
          onClick={() => setActiveTab("context")}
        >
          知识库
        </button>
      </div>

      <div className="flex-1 overflow-y-auto">
        {activeTab === "choices" ? (
          <div className="p-4">
            <button
              onClick={generateChoices}
              disabled={loading || !currentContent || currentContent.length < 100}
              className="w-full py-2 px-4 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors mb-4"
            >
              {loading ? (
                <span className="flex items-center justify-center gap-2">
                  <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                    <circle
                      className="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      strokeWidth="4"
                      fill="none"
                    />
                    <path
                      className="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    />
                  </svg>
                  生成中...
                </span>
              ) : (
                "✨ 生成续写选项"
              )}
            </button>

            {currentContent && currentContent.length < 100 && (
              <p className="text-sm text-gray-500 text-center">请先输入至少100字的内容</p>
            )}

            {error && (
              <div className="p-3 bg-red-50 border border-red-200 rounded-lg">
                <p className="text-sm text-red-700 font-medium mb-1">❌ 生成失败</p>
                <p className="text-xs text-red-600">{error}</p>
                {(error.includes("401") ||
                  error.includes("Unauthorized") ||
                  error.includes("API")) && (
                  <p className="text-xs text-red-500 mt-1">请检查API密钥是否正确配置</p>
                )}
              </div>
            )}

            {suggestion && (
              <div className="space-y-4">
                {suggestion.consistency_warnings.length > 0 && (
                  <div className="p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                    <p className="text-sm text-yellow-700 font-medium mb-2">⚠️ 检测到一致性问题</p>
                    <ul className="text-xs text-yellow-600 space-y-1">
                      {suggestion.consistency_warnings.slice(0, 2).map((w, i) => (
                        <li key={i}>
                          {w.character_name}: {w.expected} → {w.actual}
                        </li>
                      ))}
                    </ul>
                  </div>
                )}

                {suggestion.new_characters.length > 0 && (
                  <div className="p-3 bg-purple-50 border border-purple-200 rounded-lg">
                    <p className="text-sm text-purple-700 font-medium mb-2">🆕 检测到新角色</p>
                    <div className="flex flex-wrap gap-2">
                      {suggestion.new_characters.map((name, i) => (
                        <button
                          key={i}
                          onClick={() => onCreateCharacter(name)}
                          className="text-xs px-2 py-1 bg-purple-100 text-purple-600 rounded hover:bg-purple-200 transition-colors"
                        >
                          + {name}
                        </button>
                      ))}
                    </div>
                  </div>
                )}

                <div className="space-y-3">
                  <p className="text-sm font-medium text-gray-700">选择续写方向：</p>
                  {suggestion.choices.map((choice) => (
                    <div
                      key={choice.id}
                      onClick={() => handleChoiceClick(choice)}
                      className="p-3 bg-gray-50 border border-gray-200 rounded-lg cursor-pointer hover:border-blue-300 hover:bg-blue-50 transition-all"
                    >
                      <div className="flex items-center gap-2 mb-2">
                        <span className="text-lg">{choice.direction_icon}</span>
                        <span className="font-medium text-gray-800">{choice.direction}</span>
                        <span className="text-xs px-2 py-0.5 bg-gray-200 text-gray-600 rounded">
                          {choice.emotional_tone}
                        </span>
                      </div>
                      <p className="text-sm text-gray-600 line-clamp-3 mb-2">{choice.preview}</p>
                      <p className="text-xs text-gray-400">{choice.hint}</p>
                      {choice.characters.length > 0 && (
                        <div className="flex flex-wrap gap-1 mt-2">
                          {choice.characters.map((char, i) => (
                            <span
                              key={i}
                              className="text-xs px-1.5 py-0.5 bg-blue-100 text-blue-600 rounded"
                            >
                              {char}
                            </span>
                          ))}
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        ) : activeTab === "validation" ? (
          <div className="p-4">
            <button
              onClick={validateContent}
              disabled={validating || !currentContent || currentContent.length < 50}
              className="w-full py-2 px-4 bg-green-500 text-white rounded-lg hover:bg-green-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors mb-4"
            >
              {validating ? (
                <span className="flex items-center justify-center gap-2">
                  <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                    <circle
                      className="opacity-25"
                      cx="12"
                      cy="12"
                      r="10"
                      stroke="currentColor"
                      strokeWidth="4"
                      fill="none"
                    />
                    <path
                      className="opacity-75"
                      fill="currentColor"
                      d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    />
                  </svg>
                  检查中...
                </span>
              ) : (
                "🔍 检查一致性"
              )}
            </button>

            {validationResult && (
              <div className="space-y-4">
                {validationResult.detected_characters.filter((char) => !isPronoun(char.name))
                  .length > 0 && (
                  <div className="p-3 bg-blue-50 border border-blue-200 rounded-lg">
                    <p className="text-sm font-medium text-blue-700 mb-2">
                      👥 出场角色 (
                      {
                        validationResult.detected_characters.filter((char) => !isPronoun(char.name))
                          .length
                      }
                      )
                    </p>
                    <div className="space-y-2">
                      {validationResult.detected_characters
                        .filter((char) => !isPronoun(char.name))
                        .map((char, i) => (
                          <div key={i} className="flex items-center justify-between text-sm">
                            <span
                              className={
                                char.is_new ? "text-purple-600 font-medium" : "text-gray-700"
                              }
                            >
                              {char.is_new && "🆕 "}
                              {char.name}
                            </span>
                            {char.is_new && (
                              <button
                                onClick={() => {
                                  logger.debug("Creating character", { name: char.name });
                                  onCreateCharacter(char.name);
                                }}
                                className="text-xs px-2 py-1 bg-purple-100 text-purple-600 rounded hover:bg-purple-200 transition-colors"
                              >
                                创建
                              </button>
                            )}
                          </div>
                        ))}
                    </div>
                  </div>
                )}

                {validationResult.consistency_warnings.length > 0 ? (
                  <div className="space-y-2">
                    <p className="text-sm font-medium text-red-700">
                      ⚠️ 一致性问题 ({validationResult.consistency_warnings.length})
                    </p>
                    {validationResult.consistency_warnings.map((warning, i) => (
                      <div
                        key={i}
                        className={`p-3 border rounded-lg ${getSeverityColor(warning.severity)}`}
                      >
                        <div
                          className="flex items-center justify-between cursor-pointer"
                          onClick={() =>
                            setExpandedWarning(expandedWarning === `${i}` ? null : `${i}`)
                          }
                        >
                          <div className="flex items-center gap-2">
                            <span>{getSeverityIcon(warning.severity)}</span>
                            <span className="font-medium">
                              {warning.character_name || warning.warning_type}
                            </span>
                          </div>
                          <span className="text-xs">{expandedWarning === `${i}` ? "▼" : "▶"}</span>
                        </div>
                        {expandedWarning === `${i}` && (
                          <div className="mt-2 text-sm space-y-1">
                            <p>
                              <span className="font-medium">设定：</span>
                              {warning.expected}
                            </p>
                            <p>
                              <span className="font-medium">实际：</span>
                              {warning.actual}
                            </p>
                          </div>
                        )}
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="p-4 bg-green-50 border border-green-200 rounded-lg text-center">
                    <span className="text-2xl">✅</span>
                    <p className="text-sm text-green-700 mt-2">未发现一致性问题</p>
                  </div>
                )}

                {validationResult.new_settings.length > 0 && (
                  <div className="p-3 bg-orange-50 border border-orange-200 rounded-lg">
                    <p className="text-sm font-medium text-orange-700 mb-2">📝 新设定/名词</p>
                    <div className="flex flex-wrap gap-2">
                      {validationResult.new_settings.map((setting, i) => (
                        <div key={i} className="flex items-center gap-1">
                          <span className="text-xs px-2 py-1 bg-orange-100 text-orange-600 rounded">
                            {setting}
                          </span>
                          {onCreateWorldView && (
                            <button
                              onClick={() => {
                                logger.debug("Creating worldview", { setting });
                                onCreateWorldView(setting);
                              }}
                              className="text-xs px-1.5 py-1 bg-orange-200 text-orange-700 rounded hover:bg-orange-300 transition-colors"
                              title="添加到世界观设定"
                            >
                              +
                            </button>
                          )}
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        ) : activeTab === "context" ? (
          <div className="p-4">
            <div className="flex items-center justify-between mb-4">
              <h4 className="text-sm font-medium text-gray-700">项目知识库</h4>
              <button
                onClick={loadKnowledgeContext}
                disabled={loadingContext}
                className="text-xs px-2 py-1 bg-blue-100 text-blue-600 rounded hover:bg-blue-200"
              >
                {loadingContext ? "刷新中..." : "刷新"}
              </button>
            </div>

            {loadingContext ? (
              <div className="text-center py-4 text-gray-500">加载中...</div>
            ) : knowledgeContext ? (
              <div className="space-y-4">
                {knowledgeContext.active_characters.length > 0 && (
                  <div className="p-3 bg-blue-50 border border-blue-200 rounded-lg">
                    <p className="text-sm font-medium text-blue-700 mb-2">
                      👥 主要角色 ({knowledgeContext.active_characters.length})
                    </p>
                    <div className="flex flex-wrap gap-1">
                      {knowledgeContext.active_characters.map((name, i) => (
                        <span
                          key={i}
                          className="text-xs px-2 py-1 bg-blue-100 text-blue-600 rounded"
                        >
                          {name}
                        </span>
                      ))}
                    </div>
                  </div>
                )}

                {knowledgeContext.characters_summary && (
                  <div className="p-3 bg-purple-50 border border-purple-200 rounded-lg">
                    <p className="text-sm font-medium text-purple-700 mb-2">📋 角色信息摘要</p>
                    <p className="text-xs text-purple-600 whitespace-pre-line max-h-32 overflow-y-auto">
                      {knowledgeContext.characters_summary}
                    </p>
                  </div>
                )}

                {knowledgeContext.worldview_summary && (
                  <div className="p-3 bg-green-50 border border-green-200 rounded-lg">
                    <p className="text-sm font-medium text-green-700 mb-2">🌍 世界观摘要</p>
                    <p className="text-xs text-green-600 whitespace-pre-line max-h-32 overflow-y-auto">
                      {knowledgeContext.worldview_summary}
                    </p>
                  </div>
                )}

                {knowledgeContext.key_events.length > 0 && (
                  <div className="p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                    <p className="text-sm font-medium text-yellow-700 mb-2">
                      📅 关键事件 ({knowledgeContext.key_events.length})
                    </p>
                    <ul className="text-xs text-yellow-600 space-y-1">
                      {knowledgeContext.key_events.slice(0, 5).map((event, i) => (
                        <li key={i} className="truncate">
                          • {event}
                        </li>
                      ))}
                      {knowledgeContext.key_events.length > 5 && (
                        <li className="text-yellow-500">
                          ...还有 {knowledgeContext.key_events.length - 5} 个事件
                        </li>
                      )}
                    </ul>
                  </div>
                )}

                {knowledgeContext.plot_summary && (
                  <div className="p-3 bg-orange-50 border border-orange-200 rounded-lg">
                    <p className="text-sm font-medium text-orange-700 mb-2">📖 剧情概要</p>
                    <p className="text-xs text-orange-600 whitespace-pre-line max-h-32 overflow-y-auto">
                      {knowledgeContext.plot_summary}
                    </p>
                  </div>
                )}
              </div>
            ) : (
              <div className="text-center py-8 text-gray-500">
                <p>无法加载知识库上下文</p>
                <button
                  onClick={loadKnowledgeContext}
                  className="mt-2 text-xs text-blue-500 hover:underline"
                >
                  重试
                </button>
              </div>
            )}
          </div>
        ) : null}
      </div>

      <div className="p-3 border-t border-gray-200 bg-gray-50">
        <div className="text-xs text-gray-500 space-y-1">
          <p>💡 提示：写作助手会分析你的内容</p>
          <p>并提供多种续写方向供选择</p>
        </div>
      </div>
    </div>
  );
};

export default WritingAssistant;
