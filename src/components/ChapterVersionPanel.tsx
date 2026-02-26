import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Chapter, ChapterVersion, ChapterEvaluation } from "../types";
import { Star, CheckCircle, RefreshCw, FileText, BarChart3 } from "lucide-react";
import { TypewriterEffect } from "./TypewriterEffect";

interface ChapterVersionPanelProps {
  chapter: Chapter;
  projectId: string;
  onUpdateChapter: (chapter: Chapter) => void;
  onClose: () => void;
}

export const ChapterVersionPanel: React.FC<ChapterVersionPanelProps> = ({
  chapter,
  projectId,
  onUpdateChapter,
  onClose,
}) => {
  const [generating, setGenerating] = useState(false);
  const [evaluating, setEvaluating] = useState(false);
  const [selecting, setSelecting] = useState(false);
  const [versions, setVersions] = useState<ChapterVersion[]>(chapter.versions || []);
  const [evaluation, setEvaluation] = useState<ChapterEvaluation | null>(chapter.evaluation || null);
  const [selectedVersionIndex, setSelectedVersionIndex] = useState<number>(0);

  const handleGenerateVersions = async () => {
    setGenerating(true);
    try {
      const updatedChapter = await invoke<Chapter>("generate_chapter_versions", {
        request: {
          project_id: projectId,
          chapter_id: chapter.id,
          context: chapter.content,
          num_versions: 3,
        },
      });
      setVersions(updatedChapter.versions || []);
      onUpdateChapter(updatedChapter);
    } catch (error) {
      console.error("生成版本失败:", error);
    } finally {
      setGenerating(false);
    }
  };

  const handleSelectVersion = async (index: number) => {
    setSelecting(true);
    setSelectedVersionIndex(index);
    try {
      const updatedChapter = await invoke<Chapter>("select_chapter_version", {
        request: {
          project_id: projectId,
          chapter_id: chapter.id,
          version_index: index,
        },
      });
      onUpdateChapter(updatedChapter);
      onClose();
    } catch (error) {
      console.error("选择版本失败:", error);
    } finally {
      setSelecting(false);
    }
  };

  const handleEvaluateChapter = async () => {
    setEvaluating(true);
    try {
      const updatedChapter = await invoke<Chapter>("evaluate_chapter", {
        request: {
          project_id: projectId,
          chapter_id: chapter.id,
        },
      });
      setEvaluation(updatedChapter.evaluation || null);
      onUpdateChapter(updatedChapter);
    } catch (error) {
      console.error("评估章节失败:", error);
    } finally {
      setEvaluating(false);
    }
  };

  const getScoreColor = (score: number) => {
    if (score >= 80) return "text-green-600";
    if (score >= 60) return "text-yellow-600";
    return "text-red-600";
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-4 py-3 border-b bg-muted/30">
        <div className="flex items-center gap-2">
          <FileText className="w-5 h-5 text-primary" />
          <h2 className="font-semibold">章节版本与评估</h2>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={handleEvaluateChapter}
            disabled={evaluating || !chapter.content}
            className="px-3 py-1.5 text-sm rounded-md bg-primary/10 hover:bg-primary/20 text-primary disabled:opacity-50 flex items-center gap-1.5 transition-colors"
          >
            <BarChart3 className="w-4 h-4" />
            {evaluating ? "评估中..." : "评估"}
          </button>
          <button
            onClick={handleGenerateVersions}
            disabled={generating}
            className="px-3 py-1.5 text-sm rounded-md bg-primary text-white hover:bg-primary/90 disabled:opacity-50 flex items-center gap-1.5 transition-colors"
          >
            <RefreshCw className={`w-4 h-4 ${generating ? "animate-spin" : ""}`} />
            {generating ? "生成中..." : "生成版本"}
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        {evaluation && (
          <div className="mb-6 p-4 bg-gradient-to-br from-blue-50 to-indigo-50 dark:from-blue-950/30 dark:to-indigo-950/30 rounded-lg border border-blue-200 dark:border-blue-800">
            <h3 className="font-semibold mb-3 flex items-center gap-2">
              <Star className="w-5 h-5 text-yellow-500" />
              章节评估
            </h3>
            <div className="grid grid-cols-2 md:grid-cols-5 gap-3 mb-3">
              <div className="text-center p-2 bg-white dark:bg-gray-800 rounded-md">
                <div className={`text-2xl font-bold ${getScoreColor(evaluation.score)}`}>
                  {evaluation.score.toFixed(0)}
                </div>
                <div className="text-xs text-muted-foreground">总分</div>
              </div>
              <div className="text-center p-2 bg-white dark:bg-gray-800 rounded-md">
                <div className={`text-lg font-semibold ${getScoreColor(evaluation.coherence)}`}>
                  {evaluation.coherence.toFixed(0)}
                </div>
                <div className="text-xs text-muted-foreground">连贯性</div>
              </div>
              <div className="text-center p-2 bg-white dark:bg-gray-800 rounded-md">
                <div className={`text-lg font-semibold ${getScoreColor(evaluation.style_consistency)}`}>
                  {evaluation.style_consistency.toFixed(0)}
                </div>
                <div className="text-xs text-muted-foreground">风格一致性</div>
              </div>
              <div className="text-center p-2 bg-white dark:bg-gray-800 rounded-md">
                <div className={`text-lg font-semibold ${getScoreColor(evaluation.character_consistency)}`}>
                  {evaluation.character_consistency.toFixed(0)}
                </div>
                <div className="text-xs text-muted-foreground">角色一致性</div>
              </div>
              <div className="text-center p-2 bg-white dark:bg-gray-800 rounded-md">
                <div className={`text-lg font-semibold ${getScoreColor(evaluation.plot_advancement)}`}>
                  {evaluation.plot_advancement.toFixed(0)}
                </div>
                <div className="text-xs text-muted-foreground">情节推进</div>
              </div>
            </div>
            <p className="text-sm text-gray-700 dark:text-gray-300 mb-2">
              <span className="font-medium">评价：</span>{evaluation.summary}
            </p>
            {evaluation.suggestions.length > 0 && (
              <div>
                <p className="text-sm font-medium mb-1">改进建议：</p>
                <ul className="text-sm text-gray-600 dark:text-gray-400 list-disc list-inside space-y-1">
                  {evaluation.suggestions.map((suggestion, i) => (
                    <li key={i}>{suggestion}</li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        )}

        {versions.length > 0 ? (
          <div className="space-y-4">
            <h3 className="font-semibold mb-3">选择版本</h3>
            {versions.map((version, index) => (
              <button
                key={index}
                onClick={() => handleSelectVersion(index)}
                disabled={selecting}
                className={`w-full text-left p-4 rounded-lg border-2 transition-all ${
                  selectedVersionIndex === index
                    ? "border-primary bg-primary/5"
                    : "border-border hover:border-primary/50 hover:bg-muted/30"
                }`}
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex items-center gap-2">
                    <span className="px-2 py-0.5 text-xs font-medium rounded-full bg-primary/10 text-primary">
                      版本 {index + 1}
                    </span>
                    <span className="px-2 py-0.5 text-xs rounded-full bg-muted text-muted-foreground">
                      {version.style}
                    </span>
                  </div>
                  {selectedVersionIndex === index && (
                    <CheckCircle className="w-5 h-5 text-primary" />
                  )}
                </div>
                <div className="prose prose-sm dark:prose-invert max-h-40 overflow-y-auto">
                  <TypewriterEffect
                    text={version.content}
                    speed={10}
                    className="text-sm text-gray-700 dark:text-gray-300"
                    showCursor={false}
                  />
                </div>
              </button>
            ))}
          </div>
        ) : (
          <div className="text-center py-12 text-muted-foreground">
            <FileText className="w-12 h-12 mx-auto mb-3 opacity-30" />
            <p>点击"生成版本"按钮，AI将为您生成多个版本供选择</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default ChapterVersionPanel;
