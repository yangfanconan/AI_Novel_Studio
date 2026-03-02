import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, Check, Star, Eye, ChevronLeft, ChevronRight } from "lucide-react";

interface ChapterVersion {
  id: string;
  chapter_id: string;
  version_number: number;
  content: string;
  style_hint?: string;
  created_at: string;
  selected: boolean;
}

interface ChapterEvaluation {
  id: string;
  chapter_id: string;
  consistency_score: number;
  creativity_score: number;
  completeness_score: number;
  rhythm_score: number;
  overall_score: number;
  feedback: string;
  recommended_version: number;
  created_at: string;
}

interface VersionComparisonPanelProps {
  chapterId: string;
  versions: ChapterVersion[];
  evaluation?: ChapterEvaluation;
  onClose: () => void;
  onSelectVersion: (versionIndex: number) => void;
}

export default function VersionComparisonPanel({
  chapterId,
  versions,
  evaluation,
  onClose,
  onSelectVersion,
}: VersionComparisonPanelProps) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [viewMode, setViewMode] = useState<"single" | "split">("single");
  const [compareIndex, setCompareIndex] = useState(1);

  const currentVersion = versions[currentIndex];
  const compareVersion = versions[compareIndex];

  const getScoreColor = (score: number) => {
    if (score >= 8) return "text-green-500";
    if (score >= 6) return "text-yellow-500";
    return "text-red-500";
  };

  const getScoreLabel = (score: number) => {
    if (score >= 8) return "优秀";
    if (score >= 6) return "良好";
    if (score >= 4) return "一般";
    return "需改进";
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background rounded-lg shadow-xl w-full max-w-6xl h-[90vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-border">
          <div className="flex items-center gap-4">
            <h2 className="text-xl font-semibold">版本对比</h2>
            <div className="flex items-center gap-2 bg-muted rounded-lg p-1">
              <button
                onClick={() => setViewMode("single")}
                className={`px-3 py-1 rounded-md text-sm ${
                  viewMode === "single"
                    ? "bg-primary text-primary-foreground"
                    : "hover:bg-accent"
                }`}
              >
                单版本
              </button>
              <button
                onClick={() => setViewMode("split")}
                className={`px-3 py-1 rounded-md text-sm ${
                  viewMode === "split"
                    ? "bg-primary text-primary-foreground"
                    : "hover:bg-accent"
                }`}
              >
                对比
              </button>
            </div>
          </div>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {evaluation && (
          <div className="p-4 border-b border-border bg-muted/50">
            <div className="flex items-center gap-6">
              <div className="flex items-center gap-2">
                <Star className="w-5 h-5 text-yellow-500" />
                <span className="font-medium">AI推荐：版本 {evaluation.recommended_version}</span>
              </div>
              <div className="flex items-center gap-4 text-sm">
                <span>一致性: <span className={getScoreColor(evaluation.consistency_score)}>{evaluation.consistency_score.toFixed(1)}</span></span>
                <span>创意: <span className={getScoreColor(evaluation.creativity_score)}>{evaluation.creativity_score.toFixed(1)}</span></span>
                <span>完整性: <span className={getScoreColor(evaluation.completeness_score)}>{evaluation.completeness_score.toFixed(1)}</span></span>
                <span>节奏: <span className={getScoreColor(evaluation.rhythm_score)}>{evaluation.rhythm_score.toFixed(1)}</span></span>
                <span>总分: <span className={`font-bold ${getScoreColor(evaluation.overall_score)}`}>{evaluation.overall_score.toFixed(1)}</span></span>
              </div>
            </div>
            {evaluation.feedback && (
              <p className="mt-2 text-sm text-muted-foreground">{evaluation.feedback}</p>
            )}
          </div>
        )}

        <div className="flex-1 overflow-hidden flex">
          {viewMode === "single" ? (
            <div className="flex-1 flex flex-col">
              <div className="flex items-center justify-between p-3 border-b border-border bg-muted/30">
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setCurrentIndex(Math.max(0, currentIndex - 1))}
                    disabled={currentIndex === 0}
                    className="p-1 hover:bg-accent rounded disabled:opacity-50"
                  >
                    <ChevronLeft className="w-5 h-5" />
                  </button>
                  <span className="font-medium">版本 {currentIndex + 1} / {versions.length}</span>
                  <button
                    onClick={() => setCurrentIndex(Math.min(versions.length - 1, currentIndex + 1))}
                    disabled={currentIndex === versions.length - 1}
                    className="p-1 hover:bg-accent rounded disabled:opacity-50"
                  >
                    <ChevronRight className="w-5 h-5" />
                  </button>
                </div>
                {currentVersion?.style_hint && (
                  <span className="text-sm text-muted-foreground px-2 py-1 bg-muted rounded">
                    {currentVersion.style_hint}
                  </span>
                )}
                {evaluation && evaluation.recommended_version === currentIndex + 1 && (
                  <span className="text-sm text-yellow-600 flex items-center gap-1">
                    <Star className="w-4 h-4" /> AI推荐
                  </span>
                )}
              </div>
              <div className="flex-1 overflow-auto p-4">
                <div className="prose prose-sm max-w-none whitespace-pre-wrap">
                  {currentVersion?.content || "暂无内容"}
                </div>
              </div>
            </div>
          ) : (
            <>
              <div className="flex-1 flex flex-col border-r border-border">
                <div className="flex items-center justify-between p-3 border-b border-border bg-muted/30">
                  <div className="flex items-center gap-2">
                    <select
                      value={currentIndex}
                      onChange={(e) => setCurrentIndex(Number(e.target.value))}
                      className="px-2 py-1 border border-border rounded bg-background"
                    >
                      {versions.map((_, i) => (
                        <option key={i} value={i}>版本 {i + 1}</option>
                      ))}
                    </select>
                    {evaluation && evaluation.recommended_version === currentIndex + 1 && (
                      <Star className="w-4 h-4 text-yellow-500" />
                    )}
                  </div>
                  {currentVersion?.style_hint && (
                    <span className="text-xs text-muted-foreground">{currentVersion.style_hint}</span>
                  )}
                </div>
                <div className="flex-1 overflow-auto p-4">
                  <div className="prose prose-sm max-w-none whitespace-pre-wrap text-sm">
                    {currentVersion?.content || "暂无内容"}
                  </div>
                </div>
              </div>

              <div className="flex-1 flex flex-col">
                <div className="flex items-center justify-between p-3 border-b border-border bg-muted/30">
                  <div className="flex items-center gap-2">
                    <select
                      value={compareIndex}
                      onChange={(e) => setCompareIndex(Number(e.target.value))}
                      className="px-2 py-1 border border-border rounded bg-background"
                    >
                      {versions.map((_, i) => (
                        <option key={i} value={i}>版本 {i + 1}</option>
                      ))}
                    </select>
                    {evaluation && evaluation.recommended_version === compareIndex + 1 && (
                      <Star className="w-4 h-4 text-yellow-500" />
                    )}
                  </div>
                  {compareVersion?.style_hint && (
                    <span className="text-xs text-muted-foreground">{compareVersion.style_hint}</span>
                  )}
                </div>
                <div className="flex-1 overflow-auto p-4">
                  <div className="prose prose-sm max-w-none whitespace-pre-wrap text-sm">
                    {compareVersion?.content || "暂无内容"}
                  </div>
                </div>
              </div>
            </>
          )}
        </div>

        <div className="flex items-center justify-between p-4 border-t border-border">
          <div className="flex items-center gap-2">
            {versions.map((_, i) => (
              <button
                key={i}
                onClick={() => setCurrentIndex(i)}
                className={`w-8 h-8 rounded-full flex items-center justify-center text-sm ${
                  i === currentIndex
                    ? "bg-primary text-primary-foreground"
                    : "bg-muted hover:bg-accent"
                }`}
              >
                {i + 1}
              </button>
            ))}
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={onClose}
              className="px-4 py-2 text-sm rounded-md hover:bg-accent transition-colors"
            >
              取消
            </button>
            <button
              onClick={() => onSelectVersion(currentIndex)}
              className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors flex items-center gap-2"
            >
              <Check className="w-4 h-4" />
              选择此版本
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
