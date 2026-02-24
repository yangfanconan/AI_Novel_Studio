import React, { useState, useEffect } from 'react';
import {
  X,
  Sparkles,
  Loader2,
  Copy,
  Download,
  Check,
  Film,
  ChevronDown,
  ChevronRight,
  Clock,
  Camera,
  Sun,
  Moon,
  Sunrise,
  Sunset
} from 'lucide-react';
import { multimediaService } from '../services/multimedia.service';
import type {
  Storyboard,
  StoryboardScene,
  Shot,
  StoryboardOptions,
  StoryboardFormat,
  VisualStyle,
  ShotType,
  TimeOfDay
} from '../types/multimedia';
import {
  ShotType as ST,
  VisualStyle as VS,
  StoryboardFormat as SF,
  TimeOfDay as TOD,
  SHOT_TYPE_LABELS as STL,
  VISUAL_STYLE_LABELS as VSL,
  TIME_OF_DAY_LABELS as TOL
} from '../types/multimedia';
import type { Chapter } from '../types';

interface StoryboardGeneratorDialogProps {
  isOpen: boolean;
  onClose: () => void;
  projectId: string;
  chapters: Chapter[];
  currentChapterId?: string;
}

export function StoryboardGeneratorDialog({
  isOpen,
  onClose,
  projectId,
  chapters,
  currentChapterId,
}: StoryboardGeneratorDialogProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedChapterId, setSelectedChapterId] = useState<string>('');
  const [storyboard, setStoryboard] = useState<Storyboard | null>(null);
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null);
  const [expandedScenes, setExpandedScenes] = useState<Set<number>>(new Set([0]));
  const [options, setOptions] = useState<Partial<StoryboardOptions>>(
    multimediaService.getStoryboardDefaults()
  );
  const [showOptions, setShowOptions] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setSelectedChapterId(currentChapterId || '');
      setStoryboard(null);
      setError(null);
      setExpandedScenes(new Set([0]));
    }
  }, [isOpen, currentChapterId]);

  const handleGenerate = async () => {
    if (!selectedChapterId) {
      setError('请选择一个章节');
      return;
    }

    setLoading(true);
    setError(null);
    setStoryboard(null);

    try {
      const result = await multimediaService.generateStoryboard({
        chapterId: selectedChapterId,
        options
      });
      setStoryboard(result);
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

  const handleCopyShot = async (shot: Shot, sceneIndex: number, shotIndex: number) => {
    const text = formatShotText(shot);
    try {
      await navigator.clipboard.writeText(text);
      setCopiedIndex(sceneIndex * 100 + shotIndex);
      setTimeout(() => setCopiedIndex(null), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const handleCopyAll = async () => {
    if (!storyboard) return;
    const text = formatStoryboardText(storyboard);
    try {
      await navigator.clipboard.writeText(text);
      setCopiedIndex(-1);
      setTimeout(() => setCopiedIndex(null), 2000);
    } catch (err) {
      console.error('Failed to copy:', err);
    }
  };

  const handleExport = () => {
    if (!storyboard) return;
    const text = formatStoryboardText(storyboard);
    const blob = new Blob([text], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `分镜脚本_${storyboard.title}_${new Date().toISOString().slice(0, 10)}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const formatShotText = (shot: Shot): string => {
    let text = `镜头 ${shot.shotNumber}\n`;
    text += `景别: ${STL[shot.shotType as ShotType] || shot.shotType}\n`;
    text += `描述: ${shot.description}\n`;
    if (shot.camera?.type) {
      text += `镜头运动: ${shot.camera.type}`;
      if (shot.camera.direction) text += ` ${shot.camera.direction}`;
      text += '\n';
    }
    if (shot.duration) text += `时长: ${shot.duration}秒\n`;
    if (shot.characters?.length) text += `角色: ${shot.characters.join('、')}\n`;
    if (shot.action) text += `动作: ${shot.action}\n`;
    if (shot.dialogue) {
      text += `对白: ${shot.dialogue.character}: "${shot.dialogue.text}"\n`;
    }
    if (shot.soundEffects?.length) text += `音效: ${shot.soundEffects.join(', ')}\n`;
    if (shot.visualPrompt) text += `视觉提示词: ${shot.visualPrompt}\n`;
    if (shot.visualNotes) text += `备注: ${shot.visualNotes}\n`;
    return text;
  };

  const formatSceneText = (scene: StoryboardScene): string => {
    let text = `\n${'='.repeat(50)}\n`;
    text += `场景 ${scene.sceneNumber}: ${scene.title}\n`;
    text += `${'='.repeat(50)}\n`;
    text += `地点: ${scene.location}\n`;
    text += `时间: ${TOL[scene.timeOfDay as TimeOfDay] || scene.timeOfDay}\n`;
    text += `预计时长: ${scene.estimatedDuration}秒\n`;
    if (scene.notes) text += `备注: ${scene.notes}\n`;
    text += `\n`;
    scene.shots.forEach((shot) => {
      text += formatShotText(shot) + '\n';
      text += `-'.repeat(30)}\n`;
    });
    return text;
  };

  const formatStoryboardText = (sb: Storyboard): string => {
    let text = `分镜脚本: ${sb.title}\n`;
    text += `格式: ${sb.format}\n`;
    text += `风格: ${sb.style}\n`;
    text += `总时长: ${sb.totalDuration}秒\n`;
    text += `生成时间: ${sb.metadata.generatedAt}\n`;
    text += '\n';
    sb.scenes.forEach((scene) => {
      text += formatSceneText(scene);
    });
    return text;
  };

  const getTimeIcon = (timeOfDay: TimeOfDay) => {
    switch (timeOfDay) {
      case TOD.DAWN:
        return <Sunrise className="w-4 h-4 text-orange-400" />;
      case TOD.MORNING:
      case TOD.NOON:
      case TOD.AFTERNOON:
        return <Sun className="w-4 h-4 text-yellow-400" />;
      case TOD.DUSK:
        return <Sunset className="w-4 h-4 text-orange-500" />;
      case TOD.EVENING:
      case TOD.NIGHT:
        return <Moon className="w-4 h-4 text-blue-400" />;
      default:
        return <Sun className="w-4 h-4 text-gray-400" />;
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-4xl bg-white dark:bg-slate-800 rounded-lg shadow-xl max-h-[90vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2">
            <Film className="w-5 h-5 text-purple-500" />
            <h3 className="font-semibold text-slate-900 dark:text-slate-100">AI 分镜脚本生成</h3>
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
                  生成分镜脚本
                </>
              )}
            </button>
          </div>

          <button
            onClick={() => setShowOptions(!showOptions)}
            className="flex items-center gap-1 text-sm text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
          >
            {showOptions ? <ChevronDown className="w-4 h-4" /> : <ChevronRight className="w-4 h-4" />}
            高级选项
          </button>

          {showOptions && (
            <div className="mt-3 p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg grid grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  格式
                </label>
                <select
                  value={options.format || SF.FILM}
                  onChange={(e) => setOptions({ ...options, format: e.target.value as StoryboardFormat })}
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                >
                  <option value={SF.FILM}>电影</option>
                  <option value={SF.ANIMATION}>动画</option>
                  <option value={SF.COMMERCIAL}>广告</option>
                  <option value={SF.DOCUMENTARY}>纪录片</option>
                  <option value={SF.MUSIC_VIDEO}>音乐视频</option>
                </select>
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  视觉风格
                </label>
                <select
                  value={options.style || VS.CINEMATIC}
                  onChange={(e) => setOptions({ ...options, style: e.target.value as VisualStyle })}
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                >
                  {Object.entries(VSL).map(([key, label]) => (
                    <option key={key} value={key}>{label}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  详细程度
                </label>
                <select
                  value={options.detailLevel || 'standard'}
                  onChange={(e) => setOptions({ ...options, detailLevel: e.target.value as 'basic' | 'standard' | 'detailed' })}
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                >
                  <option value="basic">基础</option>
                  <option value="standard">标准</option>
                  <option value="detailed">详细</option>
                </select>
              </div>
              <div className="flex items-center gap-4 pt-4">
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.includeDialogue !== false}
                    onChange={(e) => setOptions({ ...options, includeDialogue: e.target.checked })}
                    className="rounded border-slate-300"
                  />
                  包含对白
                </label>
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.includeVisualPrompts !== false}
                    onChange={(e) => setOptions({ ...options, includeVisualPrompts: e.target.checked })}
                    className="rounded border-slate-300"
                  />
                  生成视觉提示词
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

        {storyboard && storyboard.scenes.length > 0 && (
          <div className="p-4 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
            <div className="flex items-center gap-4 text-sm text-slate-600 dark:text-slate-400">
              <span>共 {storyboard.scenes.length} 个场景</span>
              <span className="flex items-center gap-1">
                <Clock className="w-4 h-4" />
                总时长: {storyboard.totalDuration}秒
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
          {!storyboard && !loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <Film className="w-12 h-12 mb-4" />
              <p className="text-sm">选择章节后点击"生成分镜脚本"</p>
              <p className="text-xs mt-1">AI 将根据章节内容自动生成专业的分镜脚本</p>
            </div>
          )}

          {loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <Loader2 className="w-12 h-12 animate-spin mb-4" />
              <p className="text-sm">正在生成分镜脚本...</p>
              <p className="text-xs mt-1">AI 正在分析章节内容并生成分镜</p>
            </div>
          )}

          {storyboard && storyboard.scenes.length > 0 && (
            <div className="space-y-4">
              {storyboard.scenes.map((scene, sceneIndex) => (
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
                      <span className="px-2 py-0.5 bg-purple-500 text-white text-xs font-medium rounded">
                        场景 {scene.sceneNumber}
                      </span>
                      <span className="font-medium text-slate-900 dark:text-slate-100">
                        {scene.title}
                      </span>
                    </div>
                    <div className="flex items-center gap-3 text-xs text-slate-500 dark:text-slate-400">
                      <span className="flex items-center gap-1">
                        {getTimeIcon(scene.timeOfDay)}
                        {TOL[scene.timeOfDay as TimeOfDay]}
                      </span>
                      <span>{scene.location}</span>
                      <span className="flex items-center gap-1">
                        <Clock className="w-3 h-3" />
                        {scene.estimatedDuration}s
                      </span>
                      <span className="flex items-center gap-1">
                        <Camera className="w-3 h-3" />
                        {scene.shots.length} 镜头
                      </span>
                    </div>
                  </div>

                  {expandedScenes.has(sceneIndex) && (
                    <div className="p-3 space-y-3">
                      {scene.shots.map((shot, shotIndex) => (
                        <div
                          key={shotIndex}
                          className="p-3 bg-white dark:bg-slate-800 rounded-lg border border-slate-100 dark:border-slate-600"
                        >
                          <div className="flex items-start justify-between mb-2">
                            <div className="flex items-center gap-2">
                              <span className="px-2 py-0.5 bg-blue-500 text-white text-xs rounded">
                                镜头 {shot.shotNumber}
                              </span>
                              <span className="px-2 py-0.5 bg-slate-200 dark:bg-slate-600 text-slate-600 dark:text-slate-300 text-xs rounded">
                                {STL[shot.shotType as ShotType] || shot.shotType}
                              </span>
                              {shot.duration && (
                                <span className="text-xs text-slate-500 dark:text-slate-400">
                                  {shot.duration}s
                                </span>
                              )}
                            </div>
                            <button
                              onClick={() => handleCopyShot(shot, sceneIndex, shotIndex)}
                              className="flex items-center gap-1 px-2 py-0.5 text-xs bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 rounded transition-colors"
                            >
                              {copiedIndex === sceneIndex * 100 + shotIndex ? (
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

                          <p className="text-sm text-slate-700 dark:text-slate-300 mb-2">
                            {shot.description}
                          </p>

                          <div className="grid grid-cols-2 gap-2 text-xs">
                            {shot.camera?.type && (
                              <div>
                                <span className="text-slate-500 dark:text-slate-400">镜头运动: </span>
                                <span className="text-slate-700 dark:text-slate-300">
                                  {shot.camera.type}
                                  {shot.camera.direction && ` ${shot.camera.direction}`}
                                </span>
                              </div>
                            )}
                            {shot.characters?.length > 0 && (
                              <div>
                                <span className="text-slate-500 dark:text-slate-400">角色: </span>
                                <span className="text-slate-700 dark:text-slate-300">
                                  {shot.characters.join('、')}
                                </span>
                              </div>
                            )}
                          </div>

                          {shot.action && (
                            <div className="mt-2 text-xs text-slate-600 dark:text-slate-400">
                              动作: {shot.action}
                            </div>
                          )}

                          {shot.dialogue && (
                            <div className="mt-2 p-2 bg-slate-50 dark:bg-slate-700 rounded text-xs italic text-slate-700 dark:text-slate-300">
                              <span className="font-medium not-italic">{shot.dialogue.character}:</span> "{shot.dialogue.text}"
                            </div>
                          )}

                          {shot.soundEffects?.length > 0 && (
                            <div className="mt-2 text-xs text-slate-500 dark:text-slate-400">
                              音效: {shot.soundEffects.join(', ')}
                            </div>
                          )}

                          {shot.visualPrompt && (
                            <div className="mt-2 p-2 bg-purple-50 dark:bg-purple-900/20 rounded border border-purple-200 dark:border-purple-800">
                              <div className="text-xs text-purple-600 dark:text-purple-400 font-medium mb-1">
                                视觉提示词
                              </div>
                              <p className="text-xs text-purple-800 dark:text-purple-300">
                                {shot.visualPrompt}
                              </p>
                            </div>
                          )}

                          {shot.visualNotes && (
                            <div className="mt-2 text-xs text-slate-500 dark:text-slate-400">
                              备注: {shot.visualNotes}
                            </div>
                          )}
                        </div>
                      ))}
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
