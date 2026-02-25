import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Sparkles, RefreshCw, Check, AlertCircle, FileText, Users, Globe } from "lucide-react";

interface BatchGeneratorProps {
  isOpen: boolean;
  onClose: () => void;
  projectId: string;
  type?: "chapters" | "characters" | "worldviews";
  onSuccess?: () => void;
}

interface GenerationProgress {
  current: number;
  total: number;
  current_item: string;
  status: "pending" | "generating" | "completed" | "error";
  results: GenerationResult[];
}

interface GenerationResult {
  name: string;
  success: boolean;
  message?: string;
}

type GenerationType = "chapters" | "characters" | "worldviews";

export default function BatchGenerator({
  isOpen,
  onClose,
  projectId,
  type: initialType,
  onSuccess,
}: BatchGeneratorProps) {
  const [selectedType, setSelectedType] = useState<GenerationType>(initialType || "chapters");
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState<GenerationProgress | null>(null);
  const [config, setConfig] = useState({
    count: 5,
    style: "",
    context: "",
  });

  const typeConfig = {
    chapters: {
      title: "批量生成章节",
      icon: <FileText className="w-5 h-5" />,
      description: "根据大纲自动生成多个章节内容",
      fields: ["count", "style", "context"],
      countLabel: "章节数量",
      stylePlaceholder: "如：轻松幽默、严肃正经...",
      contextPlaceholder: "输入故事背景、主要角色等信息...",
    },
    characters: {
      title: "批量生成角色",
      icon: <Users className="w-5 h-5" />,
      description: "根据项目背景生成多个角色",
      fields: ["count", "style", "context"],
      countLabel: "角色数量",
      stylePlaceholder: "如：武侠风、现代都市...",
      contextPlaceholder: "输入项目类型、世界观背景等信息...",
    },
    worldviews: {
      title: "批量生成世界观",
      icon: <Globe className="w-5 h-5" />,
      description: "生成多个世界观设定模块",
      fields: ["count", "context"],
      countLabel: "设定数量",
      stylePlaceholder: "",
      contextPlaceholder: "输入项目类型、风格等信息...",
    },
  };

  const currentConfig = typeConfig[selectedType];

  const handleGenerate = async () => {
    setIsGenerating(true);
    setProgress({
      current: 0,
      total: config.count,
      current_item: "",
      status: "generating",
      results: [],
    });

    try {
      for (let i = 0; i < config.count; i++) {
        setProgress((prev) =>
          prev
            ? {
                ...prev,
                current: i + 1,
                current_item: `正在生成第 ${i + 1} 个...`,
              }
            : null
        );

        let result: GenerationResult;

        if (selectedType === "chapters") {
          result = await generateChapter(i);
        } else if (selectedType === "characters") {
          result = await generateCharacter(i);
        } else {
          result = await generateWorldview(i);
        }

        setProgress((prev) =>
          prev
            ? {
                ...prev,
                results: [...prev.results, result],
              }
            : null
        );
      }

      setProgress((prev) => (prev ? { ...prev, status: "completed" } : null));
      if (onSuccess) onSuccess();
    } catch (error) {
      console.error("Generation error:", error);
      setProgress((prev) => (prev ? { ...prev, status: "error" } : null));
    } finally {
      setIsGenerating(false);
    }
  };

  const generateChapter = async (index: number): Promise<GenerationResult> => {
    try {
      const prompt = `请生成一个小说章节，风格：${config.style || "自由发挥"}。
背景信息：${config.context || "无"}
这是第 ${index + 1} 章。`;

      const response = await invoke<string>("ai_continue_novel", {
        projectId,
        context: prompt,
        instruction: "生成一个完整的章节，包含标题和正文",
      });

      const titleMatch = response.match(/第[一二三四五六七八九十\d]+章[^\n]*/);
      const title = titleMatch ? titleMatch[0] : `第${index + 1}章`;

      await invoke("save_chapter", {
        request: {
          project_id: projectId,
          title,
          content: response,
        },
      });

      return { name: title, success: true };
    } catch (error) {
      return { name: `第${index + 1}章`, success: false, message: String(error) };
    }
  };

  const generateCharacter = async (_index: number): Promise<GenerationResult> => {
    try {
      const response = await invoke<any>("ai_generate_character", {
        projectId,
        description: config.context || "请生成一个角色",
        style: config.style,
      });

      return { name: response.name || "新角色", success: true };
    } catch (error) {
      return { name: "角色生成失败", success: false, message: String(error) };
    }
  };

  const generateWorldview = async (_index: number): Promise<GenerationResult> => {
    try {
      const response = await invoke<any>("ai_generate_worldview", {
        projectId,
        description: config.context || "请生成一个世界观设定",
      });

      return { name: response.name || "新设定", success: true };
    } catch (error) {
      return { name: "世界观生成失败", success: false, message: String(error) };
    }
  };

  const handleClose = () => {
    if (!isGenerating) {
      setProgress(null);
      onClose();
    }
  };

  const handleTypeChange = (newType: GenerationType) => {
    if (!isGenerating) {
      setSelectedType(newType);
      setProgress(null);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-md overflow-hidden">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2">
            {currentConfig.icon}
            <h2 className="text-lg font-semibold text-slate-800 dark:text-slate-100">批量生成</h2>
          </div>
          <button
            onClick={handleClose}
            disabled={isGenerating}
            className="text-slate-500 hover:text-slate-700 dark:text-slate-400 disabled:opacity-50"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-4 space-y-4">
          <div className="flex gap-2">
            {Object.entries(typeConfig).map(([key, value]) => (
              <button
                key={key}
                onClick={() => handleTypeChange(key as GenerationType)}
                disabled={isGenerating}
                className={`flex-1 flex flex-col items-center gap-1 p-3 rounded-lg border-2 transition-colors ${
                  selectedType === key
                    ? "border-blue-500 bg-blue-50 dark:bg-blue-900/20"
                    : "border-slate-200 dark:border-slate-600 hover:border-slate-300"
                } ${isGenerating ? "opacity-50 cursor-not-allowed" : ""}`}
              >
                {value.icon}
                <span className="text-xs text-slate-600 dark:text-slate-400">
                  {key === "chapters" ? "章节" : key === "characters" ? "角色" : "世界观"}
                </span>
              </button>
            ))}
          </div>

          <p className="text-sm text-slate-600 dark:text-slate-400">{currentConfig.description}</p>

          {!progress || progress.status === "pending" ? (
            <>
              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  {currentConfig.countLabel}
                </label>
                <input
                  type="number"
                  min={1}
                  max={20}
                  value={config.count}
                  onChange={(e) => setConfig({ ...config, count: parseInt(e.target.value) || 1 })}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                />
              </div>

              {currentConfig.fields.includes("style") && (
                <div>
                  <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                    风格
                  </label>
                  <input
                    type="text"
                    value={config.style}
                    onChange={(e) => setConfig({ ...config, style: e.target.value })}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                    placeholder={currentConfig.stylePlaceholder}
                  />
                </div>
              )}

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                  背景信息
                </label>
                <textarea
                  value={config.context}
                  onChange={(e) => setConfig({ ...config, context: e.target.value })}
                  className="w-full h-24 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200 resize-none"
                  placeholder={currentConfig.contextPlaceholder}
                />
              </div>
            </>
          ) : (
            <div className="space-y-3">
              <div className="flex items-center justify-between text-sm">
                <span>{progress.current_item}</span>
                <span className="text-slate-500">
                  {progress.current}/{progress.total}
                </span>
              </div>

              <div className="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2">
                <div
                  className="bg-blue-500 h-2 rounded-full transition-all"
                  style={{ width: `${(progress.current / progress.total) * 100}%` }}
                />
              </div>

              <div className="max-h-40 overflow-y-auto space-y-1">
                {progress.results.map((result, index) => (
                  <div
                    key={index}
                    className={`flex items-center gap-2 text-sm p-2 rounded ${
                      result.success
                        ? "bg-green-50 dark:bg-green-900/20 text-green-600 dark:text-green-400"
                        : "bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400"
                    }`}
                  >
                    {result.success ? (
                      <Check className="w-4 h-4" />
                    ) : (
                      <AlertCircle className="w-4 h-4" />
                    )}
                    <span>{result.name}</span>
                    {result.message && (
                      <span className="text-xs opacity-70">- {result.message}</span>
                    )}
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        <div className="flex items-center justify-end gap-2 p-4 border-t border-slate-200 dark:border-slate-700">
          {!progress || progress.status === "pending" ? (
            <>
              <button
                onClick={handleClose}
                className="px-4 py-2 text-slate-600 dark:text-slate-400"
              >
                取消
              </button>
              <button
                onClick={handleGenerate}
                disabled={isGenerating || config.count < 1}
                className="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50"
              >
                <Sparkles className="w-4 h-4" />
                开始生成
              </button>
            </>
          ) : progress.status === "completed" ? (
            <button
              onClick={handleClose}
              className="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600"
            >
              完成
            </button>
          ) : (
            <div className="flex items-center gap-2 text-slate-500">
              <RefreshCw className="w-4 h-4 animate-spin" />
              生成中...
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
