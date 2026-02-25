import React, { useState, useEffect } from "react";
import {
  X,
  Sparkles,
  Loader2,
  Copy,
  Download,
  Check,
  FileText,
  ChevronDown,
  ChevronRight,
  Users,
  MapPin,
} from "lucide-react";
import { multimediaService } from "../services/multimedia.service";
import type {
  Script,
  ScriptScene,
  ScriptDialogue,
  ScriptConversionOptions,
  ScriptFormat,
} from "../types/multimedia";
import { ScriptFormat as SF, SCRIPT_FORMAT_LABELS } from "../types/multimedia";
import type { Chapter } from "../types";

interface ScriptConverterDialogProps {
  isOpen: boolean;
  onClose: () => void;
  projectId: string;
  chapters: Chapter[];
  currentChapterId?: string;
}

export function ScriptConverterDialog({
  isOpen,
  onClose,
  projectId,
  chapters,
  currentChapterId,
}: ScriptConverterDialogProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedChapterId, setSelectedChapterId] = useState<string>("");
  const [script, setScript] = useState<Script | null>(null);
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null);
  const [expandedScenes, setExpandedScenes] = useState<Set<number>>(new Set([0]));
  const [options, setOptions] = useState<ScriptConversionOptions>(
    multimediaService.getScriptDefaults()
  );
  const [showOptions, setShowOptions] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setSelectedChapterId(currentChapterId || "");
      setScript(null);
      setError(null);
      setExpandedScenes(new Set([0]));
    }
  }, [isOpen, currentChapterId]);

  const handleGenerate = async () => {
    if (!selectedChapterId) {
      setError("请选择一个章节");
      return;
    }

    setLoading(true);
    setError(null);
    setScript(null);

    try {
      const result = await multimediaService.generateScript({
        chapterId: selectedChapterId,
        options,
      });
      setScript(result);
      setExpandedScenes(new Set([0]));
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`生成失败: ${errorMessage}`);
    } finally {
      setLoading(false);
    }
  };

  const toggleScene = (index: number) => {
    const newExpanded = new Set(expandedScenes);
    if (newExpanded.has(index)) {
      newExpanded.delete(index);
    } else {
      newExpanded.add(index);
    }
    setExpandedScenes(newExpanded);
  };

  const handleCopyScene = async (scene: ScriptScene, index: number) => {
    const text = formatSceneText(scene);
    try {
      await navigator.clipboard.writeText(text);
      setCopiedIndex(index);
      setTimeout(() => setCopiedIndex(null), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  const handleCopyAll = async () => {
    if (!script) return;
    const text = formatScriptText(script);
    try {
      await navigator.clipboard.writeText(text);
      setCopiedIndex(-1);
      setTimeout(() => setCopiedIndex(null), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  const handleExport = () => {
    if (!script) return;
    const text = formatScriptText(script);
    const blob = new Blob([text], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `剧本_${script.title}_${new Date().toISOString().slice(0, 10)}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const formatDialogue = (dialogue: ScriptDialogue): string => {
    let text = `${dialogue.character.toUpperCase()}\n`;
    if (dialogue.parenthetical) {
      text += `(${dialogue.parenthetical})\n`;
    }
    text += `${dialogue.text}\n`;
    return text;
  };

  const formatSceneText = (scene: ScriptScene): string => {
    let text = `\n${"=".repeat(60)}\n`;
    text += `场景 ${scene.sceneNumber}\n`;
    text += `${"=".repeat(60)}\n\n`;
    text += `${scene.heading}\n\n`;
    text += `${scene.action}\n\n`;

    if (scene.dialogue?.length > 0) {
      text += `${"─".repeat(40)}\n`;
      scene.dialogue.forEach((d) => {
        text += formatDialogue(d) + "\n";
      });
    }

    if (scene.notes) {
      text += `\n[备注: ${scene.notes}]\n`;
    }

    return text;
  };

  const formatScriptText = (s: Script): string => {
    let text = `${s.title}\n`;
    text += `${"=".repeat(60)}\n\n`;
    text += `格式: ${SCRIPT_FORMAT_LABELS[s.format as ScriptFormat] || s.format}\n`;
    text += `生成时间: ${s.metadata.generatedAt}\n\n`;

    if (s.characters?.length > 0) {
      text += `角色列表:\n`;
      s.characters.forEach((char) => {
        text += `  - ${char.name}`;
        if (char.description) text += `: ${char.description}`;
        text += "\n";
      });
      text += "\n";
    }

    text += `${"═".repeat(60)}\n`;
    text += `剧本正文\n`;
    text += `${"═".repeat(60)}\n`;

    s.scenes.forEach((scene) => {
      text += formatSceneText(scene);
    });

    return text;
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-4xl bg-white dark:bg-slate-800 rounded-lg shadow-xl max-h-[90vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2">
            <FileText className="w-5 h-5 text-blue-500" />
            <h3 className="font-semibold text-slate-900 dark:text-slate-100">AI 剧本格式转换</h3>
          </div>
          <button
            onClick={onClose}
            className="p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
          >
            <X className="w-5 h-5 text-slate-500" />
          </button>
        </div>

        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-4 mb-4">
            <div className="flex-1">
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                选择章节
              </label>
              <select
                value={selectedChapterId}
                onChange={(e) => setSelectedChapterId(e.target.value)}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="">请选择章节</option>
                {chapters.map((chapter) => (
                  <option key={chapter.id} value={chapter.id}>
                    {chapter.title}
                  </option>
                ))}
              </select>
            </div>
            <button
              onClick={handleGenerate}
              disabled={loading || !selectedChapterId}
              className="flex items-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed mt-5"
            >
              {loading ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  转换中...
                </>
              ) : (
                <>
                  <Sparkles className="w-4 h-4" />
                  转换为剧本
                </>
              )}
            </button>
          </div>

          <button
            onClick={() => setShowOptions(!showOptions)}
            className="flex items-center gap-1 text-sm text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
          >
            {showOptions ? (
              <ChevronDown className="w-4 h-4" />
            ) : (
              <ChevronRight className="w-4 h-4" />
            )}
            转换选项
          </button>

          {showOptions && (
            <div className="mt-3 p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  目标格式
                </label>
                <select
                  value={options.targetFormat}
                  onChange={(e) =>
                    setOptions({ ...options, targetFormat: e.target.value as ScriptFormat })
                  }
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                >
                  {Object.entries(SCRIPT_FORMAT_LABELS).map(([key, label]) => (
                    <option key={key} value={key}>
                      {label}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  对白风格
                </label>
                <select
                  value={options.dialogueStyle}
                  onChange={(e) =>
                    setOptions({
                      ...options,
                      dialogueStyle: e.target.value as "standard" | "naturalistic" | "stylized",
                    })
                  }
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                >
                  <option value="standard">标准</option>
                  <option value="naturalistic">自然</option>
                  <option value="stylized">风格化</option>
                </select>
              </div>
              <div className="flex items-center gap-4 col-span-2">
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.includeSceneNumbers}
                    onChange={(e) =>
                      setOptions({ ...options, includeSceneNumbers: e.target.checked })
                    }
                    className="rounded border-slate-300"
                  />
                  场景编号
                </label>
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.includeCharacterDescriptions}
                    onChange={(e) =>
                      setOptions({ ...options, includeCharacterDescriptions: e.target.checked })
                    }
                    className="rounded border-slate-300"
                  />
                  角色描述
                </label>
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.includeCameraDirections}
                    onChange={(e) =>
                      setOptions({ ...options, includeCameraDirections: e.target.checked })
                    }
                    className="rounded border-slate-300"
                  />
                  镜头指示
                </label>
              </div>
            </div>
          )}

          {error && (
            <div className="mt-3 p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
              {error}
            </div>
          )}
        </div>

        {script && script.scenes.length > 0 && (
          <div className="p-4 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
            <div className="flex items-center gap-4 text-sm text-slate-600 dark:text-slate-400">
              <span>共 {script.scenes.length} 个场景</span>
              <span className="flex items-center gap-1">
                <Users className="w-4 h-4" />
                {script.characters?.length || 0} 个角色
              </span>
            </div>
            <div className="flex items-center gap-2">
              <button
                onClick={handleCopyAll}
                className="flex items-center gap-1 px-3 py-1.5 text-sm bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 rounded-lg transition-colors"
              >
                {copiedIndex === -1 ? (
                  <>
                    <Check className="w-4 h-4 text-green-500" />
                    已复制
                  </>
                ) : (
                  <>
                    <Copy className="w-4 h-4" />
                    复制全部
                  </>
                )}
              </button>
              <button
                onClick={handleExport}
                className="flex items-center gap-1 px-3 py-1.5 text-sm bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 rounded-lg transition-colors"
              >
                <Download className="w-4 h-4" />
                导出
              </button>
            </div>
          </div>
        )}

        <div className="flex-1 overflow-y-auto p-4">
          {!script && !loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <FileText className="w-12 h-12 mb-4" />
              <p className="text-sm">选择章节后点击"转换为剧本"</p>
              <p className="text-xs mt-1">AI 将把小说章节转换为专业的剧本格式</p>
            </div>
          )}

          {loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <Loader2 className="w-12 h-12 animate-spin mb-4" />
              <p className="text-sm">正在转换为剧本格式...</p>
              <p className="text-xs mt-1">AI 正在分析章节内容并生成剧本</p>
            </div>
          )}

          {script && script.scenes.length > 0 && (
            <div className="space-y-4">
              {script.characters && script.characters.length > 0 && (
                <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                  <div className="flex items-center gap-2 mb-2">
                    <Users className="w-4 h-4 text-slate-500" />
                    <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                      角色列表
                    </span>
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {script.characters.map((char, index) => (
                      <span
                        key={index}
                        className="px-2 py-1 bg-white dark:bg-slate-600 rounded text-xs text-slate-600 dark:text-slate-300"
                      >
                        {char.name}
                        {char.description && ` - ${char.description}`}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              {script.scenes.map((scene, sceneIndex) => (
                <div
                  key={sceneIndex}
                  className="border border-slate-200 dark:border-slate-600 rounded-lg overflow-hidden"
                >
                  <div
                    className="flex items-center justify-between p-3 bg-slate-50 dark:bg-slate-700/50 cursor-pointer hover:bg-slate-100 dark:hover:bg-slate-700"
                    onClick={() => toggleScene(sceneIndex)}
                  >
                    <div className="flex items-center gap-3">
                      {expandedScenes.has(sceneIndex) ? (
                        <ChevronDown className="w-4 h-4 text-slate-400" />
                      ) : (
                        <ChevronRight className="w-4 h-4 text-slate-400" />
                      )}
                      <span className="px-2 py-0.5 bg-blue-500 text-white text-xs font-medium rounded">
                        场景 {scene.sceneNumber}
                      </span>
                    </div>
                    <div className="flex items-center gap-3 text-xs text-slate-500 dark:text-slate-400">
                      <span className="flex items-center gap-1">
                        <MapPin className="w-3 h-3" />
                        {scene.heading}
                      </span>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          handleCopyScene(scene, sceneIndex);
                        }}
                        className="flex items-center gap-1 px-2 py-0.5 bg-slate-200 dark:bg-slate-600 rounded hover:bg-slate-300 dark:hover:bg-slate-500"
                      >
                        {copiedIndex === sceneIndex ? (
                          <>
                            <Check className="w-3 h-3 text-green-500" />
                            已复制
                          </>
                        ) : (
                          <>
                            <Copy className="w-3 h-3" />
                            复制
                          </>
                        )}
                      </button>
                    </div>
                  </div>

                  {expandedScenes.has(sceneIndex) && (
                    <div className="p-4 font-mono text-sm">
                      <div className="mb-4">
                        <div className="text-center uppercase font-bold text-slate-800 dark:text-slate-200 mb-4">
                          {scene.heading}
                        </div>
                        <p className="text-slate-700 dark:text-slate-300 leading-relaxed">
                          {scene.action}
                        </p>
                      </div>

                      {scene.dialogue && scene.dialogue.length > 0 && (
                        <div className="space-y-4 mt-4 pt-4 border-t border-slate-200 dark:border-slate-600">
                          {scene.dialogue.map((d, dIndex) => (
                            <div key={dIndex} className="text-center">
                              <div className="font-bold text-slate-800 dark:text-slate-200 uppercase mb-1">
                                {d.character}
                              </div>
                              {d.parenthetical && (
                                <div className="text-xs text-slate-500 dark:text-slate-400 italic mb-1">
                                  ({d.parenthetical})
                                </div>
                              )}
                              <div className="text-slate-700 dark:text-slate-300">{d.text}</div>
                            </div>
                          ))}
                        </div>
                      )}

                      {scene.notes && (
                        <div className="mt-4 pt-4 border-t border-slate-200 dark:border-slate-600 text-xs text-slate-500 dark:text-slate-400 italic">
                          [备注: {scene.notes}]
                        </div>
                      )}
                    </div>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>

        <div className="p-4 border-t border-slate-200 dark:border-slate-700 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg font-medium transition-colors"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}
