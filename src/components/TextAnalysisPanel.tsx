import React, { useState } from "react";
import {
  FileText,
  BarChart3,
  Heart,
  BookOpen,
  AlertTriangle,
  CheckCircle,
  TrendingUp,
  Zap,
} from "lucide-react";
import { textAnalysisService, FullAnalysis } from "../services/textAnalysis.service";

interface TextAnalysisPanelProps {
  text: string;
  characters?: any[];
}

export const TextAnalysisPanel: React.FC<TextAnalysisPanelProps> = ({ text, characters = [] }) => {
  const [analysis, setAnalysis] = useState<FullAnalysis | null>(null);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<
    "style" | "rhythm" | "emotion" | "readability" | "repetitions" | "logic"
  >("style");

  const runAnalysis = async () => {
    setLoading(true);
    try {
      const result = await textAnalysisService.runFullAnalysis(text, characters);
      setAnalysis(result);
    } catch (error) {
      console.error("Analysis failed:", error);
    } finally {
      setLoading(false);
    }
  };

  const getScoreColor = (score: number) => {
    if (score >= 80) return "text-green-600";
    if (score >= 60) return "text-yellow-600";
    return "text-red-600";
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case "low":
        return "text-yellow-600";
      case "medium":
        return "text-orange-600";
      case "high":
        return "text-red-600";
      default:
        return "text-gray-600";
    }
  };

  return (
    <div className="border-b border-border bg-card">
      <div className="px-4 py-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <FileText className="w-4 h-4 text-primary" />
            <span className="text-sm font-medium">文本分析</span>
          </div>

          <button
            onClick={runAnalysis}
            disabled={loading || !text.trim()}
            className="flex items-center gap-2 px-4 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? (
              <>
                <div className="w-4 h-4 border-2 border-primary-foreground border-t-transparent rounded-full animate-spin" />
                分析中...
              </>
            ) : (
              <>
                <Zap className="w-4 h-4" />
                开始分析
              </>
            )}
          </button>
        </div>

        {analysis && (
          <>
            <div className="mt-3 flex gap-1 border-b border-border pb-2">
              <button
                onClick={() => setActiveTab("style")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "style"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <BookOpen className="w-3 h-3 inline mr-1" />
                文风
              </button>
              <button
                onClick={() => setActiveTab("rhythm")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "rhythm"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <BarChart3 className="w-3 h-3 inline mr-1" />
                节奏
              </button>
              <button
                onClick={() => setActiveTab("emotion")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "emotion"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <Heart className="w-3 h-3 inline mr-1" />
                情感
              </button>
              <button
                onClick={() => setActiveTab("readability")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "readability"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <BookOpen className="w-3 h-3 inline mr-1" />
                可读性
              </button>
              <button
                onClick={() => setActiveTab("repetitions")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "repetitions"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <TrendingUp className="w-3 h-3 inline mr-1" />
                重复
              </button>
              <button
                onClick={() => setActiveTab("logic")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "logic"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <AlertTriangle className="w-3 h-3 inline mr-1" />
                逻辑
              </button>
            </div>

            <div className="mt-3 space-y-3">
              {activeTab === "style" && (
                <div className="space-y-2">
                  <div className="grid grid-cols-2 gap-3">
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">平均句长</div>
                      <div className="text-lg font-semibold">
                        {analysis.writing_style.avg_sentence_length.toFixed(1)} 字符
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">平均词长</div>
                      <div className="text-lg font-semibold">
                        {analysis.writing_style.avg_word_length.toFixed(1)} 字符
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">词汇丰富度</div>
                      <div
                        className={`text-lg font-semibold ${getScoreColor(analysis.writing_style.vocabulary_richness)}`}
                      >
                        {analysis.writing_style.vocabulary_richness.toFixed(1)}%
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">基调</div>
                      <div className="text-lg font-semibold capitalize">
                        {analysis.writing_style.tone}
                      </div>
                    </div>
                  </div>
                  {analysis.writing_style.writing_style_tags.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">风格标签</div>
                      <div className="flex flex-wrap gap-2">
                        {analysis.writing_style.writing_style_tags.map((tag, index) => (
                          <span
                            key={index}
                            className="px-2 py-1 text-xs bg-primary/10 text-primary rounded-md"
                          >
                            {tag}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}

              {activeTab === "rhythm" && (
                <div className="space-y-3">
                  <div className="grid grid-cols-3 gap-3">
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">节奏评分</div>
                      <div
                        className={`text-lg font-semibold ${getScoreColor(analysis.rhythm.pacing_score)}`}
                      >
                        {analysis.rhythm.pacing_score.toFixed(0)}
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">动作/描述比</div>
                      <div className="text-lg font-semibold">
                        {analysis.rhythm.action_vs_description_ratio.toFixed(1)}%
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">对话比例</div>
                      <div className="text-lg font-semibold">
                        {analysis.rhythm.dialogue_ratio.toFixed(1)}%
                      </div>
                    </div>
                  </div>
                  {analysis.rhythm.pacing_segments.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">节奏分段</div>
                      <div className="space-y-1">
                        {analysis.rhythm.pacing_segments.map((segment, index) => (
                          <div
                            key={index}
                            className="flex items-center justify-between p-2 bg-muted/50 rounded-md"
                          >
                            <span className="text-sm">{segment.segment_type}</span>
                            <span className="text-sm text-muted-foreground">
                              第 {segment.start_position}-{segment.end_position} 段
                            </span>
                            <div className="flex items-center gap-2">
                              <div className="w-16 h-2 bg-border rounded-full overflow-hidden">
                                <div
                                  className="h-full bg-primary transition-all"
                                  style={{ width: `${segment.intensity}%` }}
                                />
                              </div>
                              <span className="text-xs text-muted-foreground">
                                {segment.intensity.toFixed(0)}%
                              </span>
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}

              {activeTab === "emotion" && (
                <div className="space-y-3">
                  <div className="p-3 bg-muted rounded-md">
                    <div className="text-xs text-muted-foreground mb-1">整体情感</div>
                    <div className="text-lg font-semibold capitalize">
                      {analysis.emotion.overall_emotion}
                    </div>
                  </div>
                  <div className="grid grid-cols-2 gap-3">
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">情感变化次数</div>
                      <div className="text-lg font-semibold">
                        {analysis.emotion.emotion_changes}
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">主导情感</div>
                      <div className="text-sm">
                        {analysis.emotion.dominant_emotions.map((e, i) => (
                          <span key={i} className="inline-block mr-2">
                            {e.emotion} ({e.score.toFixed(0)}%)
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                  {analysis.emotion.emotion_curve.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">情感曲线</div>
                      <div className="h-24 flex items-end gap-1">
                        {analysis.emotion.emotion_curve.map((point, index) => (
                          <div
                            key={index}
                            className="flex-1 bg-primary/50 rounded-t-sm relative"
                            style={{ height: `${point.intensity}%` }}
                            title={`${point.emotion}: ${point.intensity.toFixed(0)}%`}
                          />
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}

              {activeTab === "readability" && (
                <div className="space-y-3">
                  <div className="grid grid-cols-2 gap-3">
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">Flesch 评分</div>
                      <div
                        className={`text-lg font-semibold ${getScoreColor(analysis.readability.flesch_score)}`}
                      >
                        {analysis.readability.flesch_score.toFixed(1)}
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">阅读级别</div>
                      <div className="text-lg font-semibold">
                        {analysis.readability.reading_level}
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">句子复杂度</div>
                      <div
                        className={`text-lg font-semibold ${getScoreColor(100 - analysis.readability.avg_sentence_complexity)}`}
                      >
                        {analysis.readability.avg_sentence_complexity.toFixed(1)}
                      </div>
                    </div>
                    <div className="p-3 bg-muted rounded-md">
                      <div className="text-xs text-muted-foreground mb-1">统计</div>
                      <div className="text-sm space-y-1">
                        <div>{analysis.readability.word_count} 词</div>
                        <div>{analysis.readability.syllable_count} 音节</div>
                      </div>
                    </div>
                  </div>
                </div>
              )}

              {activeTab === "repetitions" && (
                <div className="space-y-3">
                  <div className="p-3 bg-muted rounded-md">
                    <div className="text-xs text-muted-foreground mb-1">重复评分</div>
                    <div
                      className={`text-lg font-semibold ${getScoreColor(100 - analysis.repetitions.repetition_score)}`}
                    >
                      {analysis.repetitions.repetition_score.toFixed(1)}%
                    </div>
                  </div>
                  {analysis.repetitions.repeated_words.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">重复词语</div>
                      <div className="space-y-1">
                        {analysis.repetitions.repeated_words.slice(0, 10).map((item, index) => (
                          <div
                            key={index}
                            className="flex items-center justify-between p-2 bg-muted/50 rounded-md"
                          >
                            <span className="text-sm font-medium">"{item.text}"</span>
                            <span className="text-sm text-muted-foreground">
                              {item.count} 次 (位置: {item.positions.slice(0, 3).join(", ")}
                              {item.positions.length > 3 ? "..." : ""})
                            </span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                  {analysis.repetitions.repeated_phrases.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">重复短语</div>
                      <div className="space-y-1">
                        {analysis.repetitions.repeated_phrases.slice(0, 5).map((item, index) => (
                          <div
                            key={index}
                            className="flex items-center justify-between p-2 bg-muted/50 rounded-md"
                          >
                            <span className="text-sm font-medium">"{item.text}"</span>
                            <span className="text-sm text-muted-foreground">{item.count} 次</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}

              {activeTab === "logic" && (
                <div className="space-y-3">
                  <div className="p-3 bg-muted rounded-md">
                    <div className="flex items-center justify-between">
                      <div>
                        <div className="text-xs text-muted-foreground mb-1">逻辑评分</div>
                        <div
                          className={`text-lg font-semibold ${getScoreColor(analysis.logic.overall_score)}`}
                        >
                          {analysis.logic.overall_score.toFixed(0)}
                        </div>
                      </div>
                      {analysis.logic.overall_score >= 80 ? (
                        <CheckCircle className="w-8 h-8 text-green-600" />
                      ) : (
                        <AlertTriangle className="w-8 h-8 text-yellow-600" />
                      )}
                    </div>
                  </div>
                  {analysis.logic.logical_issues.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">逻辑问题</div>
                      <div className="space-y-1">
                        {analysis.logic.logical_issues.map((issue, index) => (
                          <div
                            key={index}
                            className="p-2 bg-yellow-50 border border-yellow-200 rounded-md"
                          >
                            <div className="flex items-center gap-2 mb-1">
                              <AlertTriangle
                                className={`w-4 h-4 ${getSeverityColor(issue.severity)}`}
                              />
                              <span className="text-sm font-medium">{issue.issue_type}</span>
                            </div>
                            <div className="text-sm text-muted-foreground">{issue.description}</div>
                            <div className="text-xs text-muted-foreground mt-1">
                              位置: 第 {issue.position} 段
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                  {analysis.logic.character_consistency_issues.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">角色一致性问题</div>
                      <div className="space-y-1">
                        {analysis.logic.character_consistency_issues.map((issue, index) => (
                          <div
                            key={index}
                            className="p-2 bg-orange-50 border border-orange-200 rounded-md"
                          >
                            <div className="flex items-center gap-2 mb-1">
                              <AlertTriangle className="w-4 h-4 text-orange-600" />
                              <span className="text-sm font-medium">{issue.character_name}</span>
                            </div>
                            <div className="text-sm text-muted-foreground">{issue.description}</div>
                            <div className="text-xs text-muted-foreground mt-1">
                              位置: {issue.positions.join(", ")}
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                  {analysis.logic.timeline_issues.length > 0 && (
                    <div>
                      <div className="text-sm text-muted-foreground mb-2">时间线问题</div>
                      <div className="space-y-1">
                        {analysis.logic.timeline_issues.map((issue, index) => (
                          <div
                            key={index}
                            className="p-2 bg-red-50 border border-red-200 rounded-md"
                          >
                            <div className="flex items-center gap-2 mb-1">
                              <AlertTriangle className="w-4 h-4 text-red-600" />
                              <span className="text-sm font-medium">{issue.issue_type}</span>
                            </div>
                            <div className="text-sm text-muted-foreground">{issue.description}</div>
                            <div className="text-xs text-muted-foreground mt-1">
                              位置: 第 {issue.position} 段
                            </div>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
};
