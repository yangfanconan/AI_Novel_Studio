import React, { useState, useEffect } from "react";
import { X, Sparkles, Loader2, Copy, Download, Check, Film } from "lucide-react";
import { aiGeneratorService } from "../services/api";
import type { StoryboardScene, Chapter } from "../types";

interface StoryboardDialogProps {
  isOpen: boolean;
  onClose: () => void;
  projectId: string;
  chapters: Chapter[];
  currentChapterId?: string;
}

export function StoryboardDialog({
  isOpen,
  onClose,
  projectId,
  chapters,
  currentChapterId,
}: StoryboardDialogProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedChapterId, setSelectedChapterId] = useState<string>("");
  const [scenes, setScenes] = useState<StoryboardScene[]>([]);
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null);

  useEffect(() => {
    if (isOpen) {
      setSelectedChapterId(currentChapterId || "");
      setScenes([]);
      setError(null);
    }
  }, [isOpen, currentChapterId]);

  const handleGenerate = async () => {
    if (!selectedChapterId) {
      setError("请选择一个章节");
      return;
    }

    setLoading(true);
    setError(null);
    setScenes([]);

    try {
      const result = await aiGeneratorService.generateStoryboard(selectedChapterId, undefined);
      setScenes(result);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`生成失败: ${errorMessage}`);
    } finally {
      setLoading(false);
    }
  };

  const handleCopyScene = async (scene: StoryboardScene, index: number) => {
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
    const text = scenes
      .map((scene, index) => formatSceneText(scene, index + 1))
      .join("\n\n---\n\n");
    try {
      await navigator.clipboard.writeText(text);
      setCopiedIndex(-1);
      setTimeout(() => setCopiedIndex(null), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  const handleExport = () => {
    const text = scenes
      .map((scene, index) => formatSceneText(scene, index + 1))
      .join("\n\n---\n\n");
    const blob = new Blob([text], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `分镜_${selectedChapterId}_${new Date().toISOString().slice(0, 10)}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const formatSceneText = (scene: StoryboardScene, sceneNumber?: number): string => {
    const num = sceneNumber ?? scene.scene_number;
    let text = `【场景 ${num}】\n`;
    text += `描述: ${scene.description}\n`;
    if (scene.camera_angle) text += `机位: ${scene.camera_angle}\n`;
    if (scene.lighting) text += `灯光: ${scene.lighting}\n`;
    if (scene.mood) text += `氛围: ${scene.mood}\n`;
    if (scene.character_actions && scene.character_actions.length > 0) {
      text += `角色动作:\n${scene.character_actions.map((a) => `  - ${a}`).join("\n")}\n`;
    }
    if (scene.dialogue) text += `对白: ${scene.dialogue}\n`;
    if (scene.notes) text += `备注: ${scene.notes}\n`;
    if (scene.visual_prompt) text += `视觉提示词: ${scene.visual_prompt}`;
    return text;
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-3xl bg-white dark:bg-slate-800 rounded-lg shadow-xl max-h-[90vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2">
            <Film className="w-5 h-5 text-purple-500" />
            <h3 className="font-semibold text-slate-900 dark:text-slate-100">AI 分镜生成</h3>
          </div>
          <button
            onClick={onClose}
            className="p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
          >
            <X className="w-5 h-5 text-slate-500" />
          </button>
        </div>

        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-4">
            <div className="flex-1">
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                选择章节
              </label>
              <select
                value={selectedChapterId}
                onChange={(e) => setSelectedChapterId(e.target.value)}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-purple-500"
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
              className="flex items-center gap-2 px-4 py-2 bg-purple-500 hover:bg-purple-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed mt-5"
            >
              {loading ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  生成中...
                </>
              ) : (
                <>
                  <Sparkles className="w-4 h-4" />
                  生成分镜
                </>
              )}
            </button>
          </div>

          {error && (
            <div className="mt-3 p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
              {error}
            </div>
          )}
        </div>

        {scenes.length > 0 && (
          <div className="p-4 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
            <span className="text-sm text-slate-600 dark:text-slate-400">
              共生成 {scenes.length} 个分镜场景
            </span>
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
          {scenes.length === 0 && !loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <Film className="w-12 h-12 mb-4" />
              <p className="text-sm">选择章节后点击"生成分镜"</p>
              <p className="text-xs mt-1">AI 将根据章节内容自动生成分镜提示词</p>
            </div>
          )}

          {scenes.length > 0 && (
            <div className="space-y-4">
              {scenes.map((scene, index) => (
                <div
                  key={index}
                  className="p-4 bg-slate-50 dark:bg-slate-700/50 rounded-lg border border-slate-200 dark:border-slate-600"
                >
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-2">
                      <span className="px-2 py-1 bg-purple-500 text-white text-xs font-medium rounded">
                        场景 {scene.scene_number}
                      </span>
                      {scene.mood && (
                        <span className="px-2 py-1 bg-slate-200 dark:bg-slate-600 text-slate-600 dark:text-slate-300 text-xs rounded">
                          {scene.mood}
                        </span>
                      )}
                    </div>
                    <button
                      onClick={() => handleCopyScene(scene, index)}
                      className="flex items-center gap-1 px-2 py-1 text-xs bg-slate-200 dark:bg-slate-600 hover:bg-slate-300 dark:hover:bg-slate-500 rounded transition-colors"
                    >
                      {copiedIndex === index ? (
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

                  <div className="space-y-2 text-sm">
                    <p className="text-slate-900 dark:text-slate-100 font-medium">
                      {scene.description}
                    </p>

                    <div className="grid grid-cols-2 gap-2 text-xs">
                      {scene.camera_angle && (
                        <div>
                          <span className="text-slate-500 dark:text-slate-400">机位: </span>
                          <span className="text-slate-700 dark:text-slate-300">
                            {scene.camera_angle}
                          </span>
                        </div>
                      )}
                      {scene.lighting && (
                        <div>
                          <span className="text-slate-500 dark:text-slate-400">灯光: </span>
                          <span className="text-slate-700 dark:text-slate-300">
                            {scene.lighting}
                          </span>
                        </div>
                      )}
                    </div>

                    {scene.character_actions && scene.character_actions.length > 0 && (
                      <div>
                        <span className="text-slate-500 dark:text-slate-400 text-xs">
                          角色动作:
                        </span>
                        <ul className="mt-1 text-xs text-slate-700 dark:text-slate-300 list-disc list-inside">
                          {scene.character_actions.map((action, i) => (
                            <li key={i}>{action}</li>
                          ))}
                        </ul>
                      </div>
                    )}

                    {scene.dialogue && (
                      <div className="p-2 bg-slate-100 dark:bg-slate-600 rounded text-xs italic text-slate-700 dark:text-slate-300">
                        "{scene.dialogue}"
                      </div>
                    )}

                    {scene.notes && (
                      <div className="text-xs text-slate-500 dark:text-slate-400">
                        备注: {scene.notes}
                      </div>
                    )}

                    {scene.visual_prompt && (
                      <div className="mt-2 p-2 bg-purple-50 dark:bg-purple-900/20 rounded border border-purple-200 dark:border-purple-800">
                        <div className="text-xs text-purple-600 dark:text-purple-400 font-medium mb-1">
                          视觉提示词
                        </div>
                        <p className="text-xs text-purple-800 dark:text-purple-300">
                          {scene.visual_prompt}
                        </p>
                      </div>
                    )}
                  </div>
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
