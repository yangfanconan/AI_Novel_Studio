import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { MessageSquare, TreeDeciduous, Brain, Waves, Sparkles, Loader2, Check, RefreshCw } from "lucide-react";

const OPTIMIZATION_DIMENSIONS = [
  {
    value: "dialogue",
    label: "对话优化",
    description: "让对话更加生动、真实、有层次",
    icon: MessageSquare,
    color: "text-blue-500",
    bgColor: "bg-blue-50 dark:bg-blue-950/30",
    tips: ["角色声音独特化", "潜台词丰富化", "对话节奏感", "非语言元素"],
  },
  {
    value: "environment",
    label: "环境描写",
    description: "让场景更加生动、氛围更加浓郁",
    icon: TreeDeciduous,
    color: "text-green-500",
    bgColor: "bg-green-50 dark:bg-green-950/30",
    tips: ["五感全开", "细节的选择性", "动态环境", "环境与角色互动"],
  },
  {
    value: "psychology",
    label: "心理活动",
    description: "让角色的内心世界更加丰富、真实",
    icon: Brain,
    color: "text-purple-500",
    bgColor: "bg-purple-50 dark:bg-purple-950/30",
    tips: ["符合角色DNA", "情绪的复杂性", "思维的跳跃性", "内心与外在的矛盾"],
  },
  {
    value: "rhythm",
    label: "节奏韵律",
    description: "让阅读体验更加流畅和沉浸",
    icon: Waves,
    color: "text-orange-500",
    bgColor: "bg-orange-50 dark:bg-orange-950/30",
    tips: ["句子长度变化", "段落节奏", "标点符号运用", "韵律感"],
  },
];

interface ChapterOptimizerPanelProps {
  chapterId: string;
  projectId: string;
  chapterTitle: string;
  onOptimizationApplied?: (content: string) => void;
}

export const ChapterOptimizerPanel: React.FC<ChapterOptimizerPanelProps> = ({
  chapterId,
  projectId,
  chapterTitle,
  onOptimizationApplied,
}) => {
  const [selectedDimension, setSelectedDimension] = useState<string>("");
  const [additionalNotes, setAdditionalNotes] = useState("");
  const [isOptimizing, setIsOptimizing] = useState(false);
  const [optimizedContent, setOptimizedContent] = useState<string>("");
  const [optimizationNotes, setOptimizationNotes] = useState<string>("");

  const handleOptimize = async () => {
    if (!selectedDimension) return;

    setIsOptimizing(true);
    setOptimizedContent("");
    setOptimizationNotes("");

    try {
      const response = await invoke("optimize_chapter", {
        request: {
          project_id: projectId,
          chapter_id: chapterId,
          dimension: selectedDimension,
          additional_notes: additionalNotes || undefined,
        },
      });

      if (response) {
        setOptimizedContent((response as any).optimized_content || "");
        setOptimizationNotes((response as any).optimization_notes || "");
      }
    } catch (error) {
      console.error("优化失败:", error);
      alert("优化失败: " + (error as Error).message);
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleApplyOptimization = () => {
    if (!optimizedContent) return;

    if (onOptimizationApplied) {
      onOptimizationApplied(optimizedContent);
    }

    const editorElement = document.querySelector("[contenteditable='true']");
    if (editorElement) {
      editorElement.textContent = optimizedContent;
      editorElement.dispatchEvent(new Event("input", { bubbles: true }));
    }

    alert("优化内容已应用到章节");
  };

  const handleReset = () => {
    setOptimizedContent("");
    setOptimizationNotes("");
    setAdditionalNotes("");
  };

  const selectedConfig = OPTIMIZATION_DIMENSIONS.find(d => d.value === selectedDimension);

  return (
    <div className="flex flex-col h-full">
      <div className="p-4 border-b border-border bg-muted/30">
        <div className="flex items-center gap-2 mb-3">
          <Sparkles className="w-5 h-5 text-primary" />
          <h2 className="text-lg font-semibold">章节优化器</h2>
        </div>
        <p className="text-sm text-muted-foreground">
          选择优化维度，AI将深度分析并优化章节内容
        </p>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {!optimizedContent ? (
          <>
            <div>
              <label className="block text-sm font-medium mb-2">选择优化维度</label>
              <div className="grid grid-cols-1 gap-3">
                {OPTIMIZATION_DIMENSIONS.map((dimension) => {
                  const Icon = dimension.icon;
                  return (
                    <button
                      key={dimension.value}
                      onClick={() => setSelectedDimension(dimension.value)}
                      className={`p-4 rounded-lg border-2 transition-all text-left ${
                        selectedDimension === dimension.value
                          ? `${dimension.bgColor} ${dimension.color} border-primary`
                          : "bg-card border-border hover:border-primary/50"
                      }`}
                    >
                      <div className="flex items-start gap-3">
                        <Icon className={`w-6 h-6 ${selectedDimension === dimension.value ? dimension.color : "text-muted-foreground"}`} />
                        <div className="flex-1">
                          <div className="font-medium mb-1">{dimension.label}</div>
                          <div className="text-xs text-muted-foreground mb-2">{dimension.description}</div>
                          <div className="flex flex-wrap gap-1">
                            {dimension.tips.map((tip, idx) => (
                              <span
                                key={idx}
                                className="text-xs px-2 py-0.5 rounded-full bg-muted"
                              >
                                {tip}
                              </span>
                            ))}
                          </div>
                        </div>
                      </div>
                    </button>
                  );
                })}
              </div>
            </div>

            {selectedDimension && selectedConfig && (
              <div className="space-y-3">
                <div>
                  <label className="block text-sm font-medium mb-2">
                    额外优化指令（可选）
                  </label>
                  <textarea
                    value={additionalNotes}
                    onChange={(e) => setAdditionalNotes(e.target.value)}
                    placeholder="例如：让对话更加紧凑，减少环境描写..."
                    className="w-full h-24 px-3 py-2 text-sm rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-primary focus:border-transparent resize-none"
                  />
                </div>

                <div className={`p-4 rounded-lg ${selectedConfig.bgColor}`}>
                  <h3 className="font-medium mb-2 flex items-center gap-2">
                    <selectedConfig.icon className={`w-4 h-4 ${selectedConfig.color}`} />
                    {selectedConfig.label}说明
                  </h3>
                  <p className="text-sm text-muted-foreground mb-3">
                    {selectedConfig.description}
                  </p>
                  <div className="space-y-2">
                    {selectedConfig.tips.map((tip, idx) => (
                      <div key={idx} className="flex items-center gap-2 text-sm">
                        <Check className={`w-4 h-4 ${selectedConfig.color}`} />
                        <span>{tip}</span>
                      </div>
                    ))}
                  </div>
                </div>

                <button
                  onClick={handleOptimize}
                  disabled={isOptimizing}
                  className="w-full py-3 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors font-medium flex items-center justify-center gap-2"
                >
                  {isOptimizing ? (
                    <>
                      <Loader2 className="w-4 h-4 animate-spin" />
                      正在优化...
                    </>
                  ) : (
                    <>
                      <Sparkles className="w-4 h-4" />
                      开始优化
                    </>
                  )}
                </button>
              </div>
            )}
          </>
        ) : (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <h3 className="font-medium flex items-center gap-2">
                <Check className="w-5 h-5 text-green-500" />
                优化完成
              </h3>
              <button
                onClick={handleReset}
                className="text-sm text-muted-foreground hover:text-foreground flex items-center gap-1"
              >
                <RefreshCw className="w-4 h-4" />
                重新优化
              </button>
            </div>

            {optimizationNotes && (
              <div className="p-4 rounded-lg bg-blue-50 dark:bg-blue-950/30 border border-blue-200 dark:border-blue-800">
                <h4 className="font-medium mb-2 text-blue-900 dark:text-blue-100">
                  优化说明
                </h4>
                <p className="text-sm text-blue-800 dark:text-blue-200 whitespace-pre-wrap">
                  {optimizationNotes}
                </p>
              </div>
            )}

            <div>
              <label className="block text-sm font-medium mb-2">优化后的内容</label>
              <div className="border border-border rounded-lg overflow-hidden">
                <div className="max-h-96 overflow-y-auto p-4 bg-background">
                  <pre className="whitespace-pre-wrap text-sm font-sans leading-relaxed">
                    {optimizedContent}
                  </pre>
                </div>
              </div>
            </div>

            <div className="flex gap-2">
              <button
                onClick={handleApplyOptimization}
                className="flex-1 py-3 px-4 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors font-medium flex items-center justify-center gap-2"
              >
                <Check className="w-4 h-4" />
                应用到章节
              </button>
              <button
                onClick={handleReset}
                className="px-4 py-3 bg-muted text-muted-foreground rounded-md hover:bg-muted/80 transition-colors"
              >
                放弃
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
