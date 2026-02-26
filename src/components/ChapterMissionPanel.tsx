import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Clapperboard, Plus, Save, Loader2, Sparkles, RefreshCw, User, X, Clock, Ban, Check, ListMusic, Target } from "lucide-react";
import type { ChapterMission, Blueprint, StoryBeat } from "../types";

interface ChapterMissionPanelProps {
  projectId: string;
  chapterId: string;
  chapterNumber: number;
  chapterTitle: string;
  chapterOutline?: string;
  blueprint?: Blueprint;
  onMissionUpdated?: (mission: ChapterMission) => void;
}

export const ChapterMissionPanel: React.FC<ChapterMissionPanelProps> = ({
  projectId,
  chapterId,
  chapterNumber,
  chapterTitle,
  chapterOutline,
  blueprint,
  onMissionUpdated,
}) => {
  const [mission, setMission] = useState<ChapterMission | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [isGenerating, setIsGenerating] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [beats, setBeats] = useState<StoryBeat[]>([]);
  const [isLoadingBeats, setIsLoadingBeats] = useState(false);
  const [showBeatSelector, setShowBeatSelector] = useState(false);

  const loadMission = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<ChapterMission | null>("get_chapter_mission", { chapterId });
      setMission(result);
    } catch (error) {
      console.error("加载导演脚本失败:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const generateMissionWithAI = async () => {
    setIsGenerating(true);
    try {
      const blueprintContext = blueprint
        ? `角色: ${blueprint.characters.map(c => c.name).join(", ")}, 设定: ${blueprint.settings.map(s => s.name).join(", ")}`
        : undefined;

      const result = await invoke<ChapterMission>("generate_chapter_mission_with_ai", {
        chapterId,
        chapterNumber,
        chapterOutline,
        blueprintContext,
      });
      setMission(result);
    } catch (error) {
      console.error("AI生成导演脚本失败:", error);
      alert("AI生成导演脚本失败: " + (error as Error).message);
    } finally {
      setIsGenerating(false);
    }
  };

  const saveMission = async () => {
    if (!mission) return;

    setIsSaving(true);
    try {
      const result = await invoke<ChapterMission>("update_chapter_mission", {
        request: {
          missionId: mission.id,
          macroBeat: mission.macro_beat,
          microBeats: mission.micro_beats,
          pov: mission.pov,
          tone: mission.tone,
          pacing: mission.pacing,
          allowedNewCharacters: mission.allowed_new_characters,
          forbiddenCharacters: mission.forbidden_characters,
          beatId: mission.beat_id,
        },
      });
      setMission(result);
      onMissionUpdated?.(result);
    } catch (error) {
      console.error("保存导演脚本失败:", error);
      alert("保存导演脚本失败: " + (error as Error).message);
    } finally {
      setIsSaving(false);
    }
  };

  const loadBeats = async () => {
    setIsLoadingBeats(true);
    try {
      const result = await invoke<StoryBeat[]>("get_story_beats", { projectId });
      setBeats(result);
    } catch (error) {
      console.error("加载节拍列表失败:", error);
    } finally {
      setIsLoadingBeats(false);
    }
  };

  const selectBeat = (beat: StoryBeat) => {
    if (!mission) return;
    setMission({
      ...mission,
      beat_id: beat.id,
      selected_beat: beat,
      macro_beat: beat.description || beat.title,
    });
    setShowBeatSelector(false);
  };

  useEffect(() => {
    loadMission();
    loadBeats();
  }, [chapterId, projectId]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full">
        <Loader2 className="w-8 h-8 animate-spin" />
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b border-border bg-muted/30">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Clapperboard className="w-5 h-5 text-primary" />
            <h2 className="text-lg font-semibold">章节导演脚本</h2>
            <span className="text-xs text-muted-foreground">L2导演层</span>
            <span className="px-2 py-0.5 bg-blue-100 dark:bg-blue-900 text-blue-900 dark:text-blue-100 text-xs font-medium rounded-full">
              第{chapterNumber}章
            </span>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={loadMission}
              className="p-2 hover:bg-accent rounded-md transition-colors"
              title="刷新"
            >
              <RefreshCw className="w-4 h-4" />
            </button>
            <button
              onClick={generateMissionWithAI}
              disabled={isGenerating}
              className="px-3 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 transition-colors flex items-center gap-2"
            >
              {isGenerating ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  AI生成中...
                </>
              ) : (
                <>
                  <Sparkles className="w-4 h-4" />
                  AI生成
                </>
              )}
            </button>
            <button
              onClick={saveMission}
              disabled={isSaving || !mission}
              className="px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 transition-colors flex items-center gap-2"
            >
              {isSaving ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  保存中...
                </>
              ) : (
                <>
                  <Save className="w-4 h-4" />
                  保存
                </>
              )}
            </button>
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        {!mission ? (
          <div className="text-center py-12">
            <Clapperboard className="w-16 h-16 text-muted-foreground mx-auto mb-4" />
            <h3 className="text-lg font-medium mb-2">尚未创建导演脚本</h3>
            <p className="text-sm text-muted-foreground mb-6 max-w-md mx-auto">
              导演脚本（Chapter Mission）控制每章的节奏、节拍、视角等信息，
              为AI写作提供精确的指导。
            </p>
            <button
              onClick={generateMissionWithAI}
              disabled={isGenerating}
              className="px-6 py-3 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 transition-colors flex items-center justify-center gap-2 mx-auto"
            >
              {isGenerating ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  AI生成中...
                </>
              ) : (
                <>
                  <Sparkles className="w-4 h-4" />
                  使用AI生成导演脚本
                </>
              )}
            </button>
          </div>
        ) : (
          <>
            <div>
              <div className="flex items-center justify-between mb-2">
                <label className="text-sm font-medium flex items-center gap-2">
                  <Check className="w-4 h-4 text-yellow-500" />
                  宏观节拍
                </label>
                <button
                  onClick={() => setShowBeatSelector(true)}
                  disabled={isLoadingBeats}
                  className="px-2 py-1 text-sm bg-purple-100 dark:bg-purple-900 text-purple-900 dark:text-purple-100 rounded hover:bg-purple-200 dark:hover:bg-purple-800 transition-colors flex items-center gap-1"
                >
                  {isLoadingBeats ? (
                    <>
                      <Loader2 className="w-3 h-3 animate-spin" />
                      加载中...
                    </>
                  ) : (
                    <>
                      <Target className="w-3 h-3" />
                      从大纲选择节拍
                    </>
                  )}
                </button>
              </div>
              <input
                type="text"
                value={mission.macro_beat}
                onChange={(e) =>
                  setMission({ ...mission, macro_beat: e.target.value })
                }
                className="w-full px-4 py-3 border-2 border-yellow-300 dark:border-yellow-700 rounded-lg bg-background focus:outline-none focus:border-yellow-500 text-base"
                placeholder="本章的主要事件或目标（一句话概括）"
              />
              {mission.selected_beat && (
                <div className="mt-2 p-2 bg-blue-50 dark:bg-blue-950/30 rounded-md border border-blue-200 dark:border-blue-800">
                  <div className="text-xs text-blue-600 dark:text-blue-400 flex items-center gap-1">
                    <ListMusic className="w-3 h-3" />
                    已选节拍：{mission.selected_beat.title} (第{mission.selected_beat.chapter_number}章)
                  </div>
                </div>
              )}
              <p className="text-xs text-muted-foreground mt-1">
                每章应该对应一个宏观节拍，确保故事推进的连贯性
              </p>
            </div>

            <div>
              <div className="flex items-center justify-between mb-2">
                <label className="text-sm font-medium flex items-center gap-2">
                  <Check className="w-4 h-4 text-blue-500" />
                  微观节拍
                </label>
                <button
                  onClick={() => {
                    const beat = prompt("请输入微观节拍:");
                    if (beat) {
                      setMission({
                        ...mission,
                        micro_beats: [...mission.micro_beats, beat],
                      });
                    }
                  }}
                  className="px-2 py-1 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors flex items-center gap-1"
                >
                  <Plus className="w-3 h-3" />
                  添加节拍
                </button>
              </div>
              <div className="space-y-2">
                {mission.micro_beats.map((beat, index) => (
                  <div key={index} className="flex items-start gap-2">
                    <span className="flex-shrink-0 w-6 h-6 rounded-full bg-blue-100 dark:bg-blue-900 text-blue-900 dark:text-blue-100 text-xs font-medium flex items-center justify-center mt-1">
                      {index + 1}
                    </span>
                    <div className="flex-1">
                      <input
                        type="text"
                        value={beat}
                        onChange={(e) => {
                          const newBeats = [...mission.micro_beats];
                          newBeats[index] = e.target.value;
                          setMission({ ...mission, micro_beats: newBeats });
                        }}
                        className="w-full px-3 py-2 border border-input rounded-md bg-background focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="具体场景或情节点"
                      />
                    </div>
                    <button
                      onClick={() => {
                        const newBeats = mission.micro_beats.filter((_, i) => i !== index);
                        setMission({ ...mission, micro_beats: newBeats });
                      }}
                      className="p-1 text-red-500 hover:bg-red-50 dark:hover:bg-red-950/30 rounded mt-1"
                    >
                      <X className="w-4 h-4" />
                    </button>
                  </div>
                ))}
              </div>
              {mission.micro_beats.length === 0 && (
                <p className="text-xs text-muted-foreground mt-2">
                  添加3-5个微观节拍来细化宏观节拍，指导AI写作
                </p>
              )}
            </div>

            <div className="grid grid-cols-3 gap-4">
              <div>
                <label className="block text-sm font-medium mb-2 flex items-center gap-2">
                  <User className="w-4 h-4 text-purple-500" />
                  视角（POV）
                </label>
                <input
                  type="text"
                  value={mission.pov || ""}
                  onChange={(e) =>
                    setMission({ ...mission, pov: e.target.value })
                  }
                  className="w-full px-3 py-2 border border-input rounded-md bg-background"
                  placeholder="视角角色名"
                />
                <p className="text-xs text-muted-foreground mt-1">
                  指定本章的叙事视角角色
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2 flex items-center gap-2">
                  <Clock className="w-4 h-4 text-green-500" />
                  节奏（Pacing）
                </label>
                <select
                  value={mission.pacing || ""}
                  onChange={(e) =>
                    setMission({ ...mission, pacing: e.target.value })
                  }
                  className="w-full px-3 py-2 border border-input rounded-md bg-background"
                >
                  <option value="">选择节奏</option>
                  <option value="慢">慢</option>
                  <option value="中">中</option>
                  <option value="快">快</option>
                </select>
                <p className="text-xs text-muted-foreground mt-1">
                  根据情节需要调整节奏
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2 flex items-center gap-2">
                  <X className="w-4 h-4 text-orange-500" />
                  基调（Tone）
                </label>
                <input
                  type="text"
                  value={mission.tone || ""}
                  onChange={(e) =>
                    setMission({ ...mission, tone: e.target.value })
                  }
                  className="w-full px-3 py-2 border border-input rounded-md bg-background"
                  placeholder="紧张/温馨/悬疑/欢快..."
                />
                <p className="text-xs text-muted-foreground mt-1">
                  本章的情感基调
                </p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="p-4 bg-green-50 dark:bg-green-950/30 rounded-lg border border-green-200 dark:border-green-800">
                <label className="block text-sm font-medium mb-2 flex items-center gap-2">
                  <Plus className="w-4 h-4 text-green-600 dark:text-green-400" />
                  允许新登场角色
                </label>
                <div className="space-y-2">
                  {mission.allowed_new_characters.map((char, index) => (
                    <div key={index} className="flex items-center gap-2">
                      <span className="flex-1 px-3 py-1.5 bg-green-100 dark:bg-green-900 text-green-900 dark:text-green-100 text-sm rounded-md">
                        {char}
                      </span>
                      <button
                        onClick={() => {
                          const newChars = mission.allowed_new_characters.filter((_, i) => i !== index);
                          setMission({ ...mission, allowed_new_characters: newChars });
                        }}
                        className="p-1 text-red-500 hover:bg-red-50 dark:hover:bg-red-950/30 rounded"
                      >
                        <X className="w-4 h-4" />
                      </button>
                    </div>
                  ))}
                  <button
                    onClick={() => {
                      const char = prompt("请输入角色名:");
                      if (char) {
                        setMission({
                          ...mission,
                          allowed_new_characters: [...mission.allowed_new_characters, char],
                        });
                      }
                    }}
                    className="w-full px-3 py-2 text-sm bg-green-600 text-white rounded hover:bg-green-700 transition-colors flex items-center justify-center gap-1"
                  >
                    <Plus className="w-4 h-4" />
                    添加角色
                  </button>
                </div>
                <p className="text-xs text-muted-foreground mt-2">
                  本章可以引入的新角色
                </p>
              </div>

              <div className="p-4 bg-red-50 dark:bg-red-950/30 rounded-lg border border-red-200 dark:border-red-800">
                <label className="block text-sm font-medium mb-2 flex items-center gap-2">
                  <Ban className="w-4 h-4 text-red-600 dark:text-red-400" />
                  禁止角色
                </label>
                <div className="space-y-2">
                  {mission.forbidden_characters.map((char, index) => (
                    <div key={index} className="flex items-center gap-2">
                      <span className="flex-1 px-3 py-1.5 bg-red-100 dark:bg-red-900 text-red-900 dark:text-red-100 text-sm rounded-md">
                        {char}
                      </span>
                      <button
                        onClick={() => {
                          const newChars = mission.forbidden_characters.filter((_, i) => i !== index);
                          setMission({ ...mission, forbidden_characters: newChars });
                        }}
                        className="p-1 text-red-500 hover:bg-red-50 dark:hover:bg-red-950/30 rounded"
                      >
                        <X className="w-4 h-4" />
                    </button>
                    </div>
                  ))}
                  <button
                    onClick={() => {
                      const char = prompt("请输入角色名:");
                      if (char) {
                        setMission({
                          ...mission,
                          forbidden_characters: [...mission.forbidden_characters, char],
                        });
                      }
                    }}
                    className="w-full px-3 py-2 text-sm bg-red-600 text-white rounded hover:bg-red-700 transition-colors flex items-center justify-center gap-1"
                  >
                    <Ban className="w-4 h-4" />
                    添加禁止角色
                  </button>
                </div>
                <p className="text-xs text-muted-foreground mt-2">
                  本章不允许出现的角色（防止信息泄露）
                </p>
              </div>
            </div>
          </>
        )}
      </div>

      {showBeatSelector && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-background rounded-lg shadow-xl w-full max-w-2xl max-h-[80vh] flex flex-col">
            <div className="p-4 border-b border-border flex items-center justify-between">
              <h3 className="text-lg font-semibold flex items-center gap-2">
                <Target className="w-5 h-5 text-purple-500" />
                选择大纲节拍
              </h3>
              <button
                onClick={() => setShowBeatSelector(false)}
                className="p-2 hover:bg-accent rounded-md transition-colors"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            <div className="flex-1 overflow-y-auto p-4">
              {beats.length === 0 ? (
                <div className="text-center py-12 text-muted-foreground">
                  大纲中没有可用的节拍
                </div>
              ) : (
                <div className="space-y-3">
                  {beats.map((beat) => (
                    <button
                      key={beat.id}
                      onClick={() => selectBeat(beat)}
                      className="w-full text-left p-4 border border-border rounded-lg hover:border-purple-500 hover:bg-purple-50 dark:hover:bg-purple-950/30 transition-all"
                    >
                      <div className="flex items-start gap-3">
                        <div className="flex-shrink-0">
                          {beat.beat_type === "chapter" && (
                            <Check className="w-5 h-5 text-yellow-500" />
                          )}
                          {beat.beat_type === "scene" && (
                            <ListMusic className="w-5 h-5 text-blue-500" />
                          )}
                          {beat.beat_type === "beat" && (
                            <Target className="w-5 h-5 text-green-500" />
                          )}
                        </div>
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-2 mb-1">
                            <span className="font-medium text-sm">{beat.title}</span>
                            <span className="px-2 py-0.5 bg-purple-100 dark:bg-purple-900 text-purple-900 dark:text-purple-100 text-xs rounded-full">
                              第{beat.chapter_number}章
                            </span>
                          </div>
                          <p className="text-sm text-muted-foreground line-clamp-2">
                            {beat.description}
                          </p>
                        </div>
                      </div>
                    </button>
                  ))}
                </div>
              )}
            </div>
            <div className="p-4 border-t border-border bg-muted/30">
              <p className="text-xs text-muted-foreground text-center">
                选择一个节拍后，该节拍将作为本章的宏观节拍
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
