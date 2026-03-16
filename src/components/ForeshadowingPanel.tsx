import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Foreshadowing, ForeshadowingStats } from "../types";
import { Sparkles, CheckCircle2, AlertTriangle, Lightbulb, TrendingUp, Plus } from "lucide-react";
import { foreshadowingService } from "../services/api";

interface ForeshadowingPanelProps {
  projectId: string;
  chapterId?: string;
  chapterNumber?: number;
  chapterTitle?: string;
}

const FORESHADOWING_TYPES = [
  { value: "object", label: "物品伏笔", icon: "🔮" },
  { value: "event", label: "事件伏笔", icon: "📅" },
  { value: "dialogue", label: "对话伏笔", icon: "💬" },
  { value: "setting", label: "设定伏笔", icon: "🌍" },
  { value: "character", label: "角色伏笔", icon: "👤" },
];

const IMPORTANCE_LEVELS = [
  { value: "critical", label: "关键", color: "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400" },
  { value: "major", label: "重要", color: "bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-400" },
  { value: "medium", label: "普通", color: "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400" },
  { value: "minor", label: "次要", color: "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400" },
];

export const ForeshadowingPanel: React.FC<ForeshadowingPanelProps> = ({
  projectId,
  chapterId,
  chapterNumber,
  chapterTitle,
}) => {
  const [foreshadowings, setForeshadowings] = useState<Foreshadowing[]>([]);
  const [stats, setStats] = useState<ForeshadowingStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [formData, setFormData] = useState({
    description: "",
    foreshadowing_type: "object",
    keywords: [] as string[],
    importance: "medium",
    expected_payoff_chapter: undefined as number | undefined,
    author_note: "",
  });

  useEffect(() => {
    loadForeshadowings();
    loadStats();
  }, [projectId]);

  const loadForeshadowings = async () => {
    try {
      const data = await foreshadowingService.getForeshadowings(projectId);
      setForeshadowings(data);
    } catch (error) {
      console.error("加载伏笔失败:", error);
    } finally {
      setLoading(false);
    }
  };

  const loadStats = async () => {
    try {
      const data = await foreshadowingService.getStats(projectId);
      setStats(data);
    } catch (error) {
      console.error("加载统计数据失败:", error);
    }
  };

  const handleCreate = async () => {
    if (!formData.description.trim()) return;

    try {
      await foreshadowingService.createForeshadowing({
        project_id: projectId,
        chapter_id: chapterId || "",
        chapter_number: chapterNumber || 0,
        chapter_title: chapterTitle || "",
        description: formData.description,
        foreshadowing_type: formData.foreshadowing_type,
        keywords: formData.keywords,
        importance: formData.importance,
        expected_payoff_chapter: formData.expected_payoff_chapter,
        author_note: formData.author_note || undefined,
      });
      setFormData({
        description: "",
        foreshadowing_type: "object",
        keywords: [],
        importance: "medium",
        expected_payoff_chapter: undefined,
        author_note: "",
      });
      setShowCreateForm(false);
      loadForeshadowings();
      loadStats();
    } catch (error) {
      console.error("创建伏笔失败:", error);
    }
  };

  const handleResolve = async (foreshadowingId: string, actualChapter: number) => {
    try {
      await foreshadowingService.resolveForeshadowing({
        foreshadowing_id: foreshadowingId,
        actual_payoff_chapter: actualChapter,
        resolution_text: "",
        quality_score: undefined,
      });
      loadForeshadowings();
      loadStats();
    } catch (error) {
      console.error("回收伏笔失败:", error);
    }
  };

  const getImportanceConfig = (importance: string) => {
    return IMPORTANCE_LEVELS.find((level) => level.value === importance) || IMPORTANCE_LEVELS[2];
  };

  const getTypeConfig = (type: string) => {
    return FORESHADOWING_TYPES.find((t) => t.value === type) || FORESHADOWING_TYPES[0];
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case "planted":
        return "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400";
      case "paid_off":
        return "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400";
      case "overdue":
        return "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400";
      default:
        return "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-400";
    }
  };

  return (
    <div className="flex flex-col h-full bg-background dark:bg-gray-900">
      <div className="flex items-center justify-between px-4 py-3 border-b bg-muted/30">
        <div className="flex items-center gap-2">
          <Sparkles className="w-5 h-5 text-primary" />
          <h2 className="font-semibold">伏笔追踪</h2>
        </div>
        <button
          onClick={() => setShowCreateForm(true)}
          className="px-3 py-1.5 text-sm rounded-md bg-primary text-white hover:bg-primary/90 flex items-center gap-1.5 transition-colors"
        >
          <Plus className="w-4 h-4" />
          添加伏笔
        </button>
      </div>

      {stats && (
        <div className="grid grid-cols-5 gap-3 px-4 py-4 bg-gradient-to-r from-blue-50/50 to-indigo-50/50 dark:from-blue-950/20 dark:to-indigo-950/20">
          <div className="text-center p-3 bg-background dark:bg-gray-800 rounded-lg shadow-sm">
            <div className="text-2xl font-bold text-primary">{stats.total_foreshadowings}</div>
            <div className="text-xs text-muted-foreground mt-1">总数</div>
          </div>
          <div className="text-center p-3 bg-background dark:bg-gray-800 rounded-lg shadow-sm">
            <div className="text-2xl font-bold text-blue-600">{stats.planted_count}</div>
            <div className="text-xs text-muted-foreground mt-1">已埋设</div>
          </div>
          <div className="text-center p-3 bg-background dark:bg-gray-800 rounded-lg shadow-sm">
            <div className="text-2xl font-bold text-green-600">{stats.paid_off_count}</div>
            <div className="text-xs text-muted-foreground mt-1">已回收</div>
          </div>
          <div className="text-center p-3 bg-background dark:bg-gray-800 rounded-lg shadow-sm">
            <div className="text-2xl font-bold text-orange-600">{stats.unresolved_count}</div>
            <div className="text-xs text-muted-foreground mt-1">待回收</div>
          </div>
          <div className="text-center p-3 bg-background dark:bg-gray-800 rounded-lg shadow-sm">
            <div className="text-lg font-bold text-purple-600">
              {stats.avg_resolution_distance.toFixed(1)}
            </div>
            <div className="text-xs text-muted-foreground mt-1">平均间距</div>
          </div>
        </div>
      )}

      {(stats?.recommendations ?? []).length > 0 && (
        <div className="mx-4 mb-3 p-3 bg-amber-50 dark:bg-amber-950/30 border border-amber-200 dark:border-amber-800 rounded-lg">
          <div className="flex items-start gap-2">
            <Lightbulb className="w-5 h-5 text-amber-600 flex-shrink-0" />
            <div className="flex-1">
              <h4 className="font-medium text-amber-900 dark:text-amber-200 mb-1">建议</h4>
              <ul className="text-sm text-amber-800 dark:text-amber-300 space-y-1">
                {(stats?.recommendations ?? []).map((rec, i) => (
                  <li key={i}>{rec}</li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}

      {showCreateForm && (
        <div className="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4">
          <div className="bg-background dark:bg-gray-800 rounded-lg shadow-xl w-full max-w-md max-h-[90vh] overflow-y-auto">
            <div className="flex items-center justify-between p-4 border-b">
              <h3 className="text-lg font-semibold">添加伏笔</h3>
              <button
                onClick={() => setShowCreateForm(false)}
                className="p-1 hover:bg-muted rounded-md transition-colors"
              >
                ✕
              </button>
            </div>
            <div className="p-4 space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">伏笔类型</label>
                <div className="grid grid-cols-5 gap-2">
                  {FORESHADOWING_TYPES.map((type) => (
                    <button
                      key={type.value}
                      type="button"
                      onClick={() => setFormData({ ...formData, foreshadowing_type: type.value })}
                      className={`p-2 text-center rounded-lg border-2 transition-all ${
                        formData.foreshadowing_type === type.value
                          ? "border-primary bg-primary/5"
                          : "border-border hover:border-primary/50"
                      }`}
                    >
                      <div className="text-2xl">{type.icon}</div>
                      <div className="text-xs mt-1">{type.label}</div>
                    </button>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">重要程度</label>
                <div className="flex gap-2">
                  {IMPORTANCE_LEVELS.map((level) => (
                    <button
                      key={level.value}
                      type="button"
                      onClick={() => setFormData({ ...formData, importance: level.value })}
                      className={`px-4 py-2 text-sm rounded-md transition-all ${
                        formData.importance === level.value
                          ? `${level.color} border-2 border-current`
                          : "bg-muted hover:bg-muted/80"
                      }`}
                    >
                      {level.label}
                    </button>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">描述</label>
                <textarea
                  value={formData.description}
                  onChange={(e) => setFormData({ ...formData, description: e.target.value })}
                  placeholder="描述这个伏笔的内容和意义..."
                  className="w-full px-3 py-2 text-sm border rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-primary/20 dark:bg-gray-700"
                  rows={4}
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">关键词（逗号分隔）</label>
                <input
                  type="text"
                  value={formData.keywords.join(", ")}
                  onChange={(e) =>
                    setFormData({
                      ...formData,
                      keywords: e.target.value.split(",").map((k) => k.trim()).filter((k) => k),
                    })
                  }
                  placeholder="钥匙, 信件, 秘密"
                  className="w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-primary/20 dark:bg-gray-700"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">预计回收章节</label>
                <input
                  type="number"
                  value={formData.expected_payoff_chapter || ""}
                  onChange={(e) =>
                    setFormData({
                      ...formData,
                      expected_payoff_chapter: e.target.value ? parseInt(e.target.value) : undefined,
                    })
                  }
                  placeholder="如：第15章"
                  className="w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-primary/20 dark:bg-gray-700"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">作者备注</label>
                <textarea
                  value={formData.author_note}
                  onChange={(e) => setFormData({ ...formData, author_note: e.target.value })}
                  placeholder="补充说明..."
                  className="w-full px-3 py-2 text-sm border rounded-md resize-none focus:outline-none focus:ring-2 focus:ring-primary/20 dark:bg-gray-700"
                  rows={2}
                />
              </div>

              <div className="flex justify-end gap-2">
                <button
                  onClick={() => setShowCreateForm(false)}
                  className="px-4 py-2 text-sm rounded-md border hover:bg-muted transition-colors"
                >
                  取消
                </button>
                <button
                  onClick={handleCreate}
                  disabled={!formData.description.trim()}
                  className="px-4 py-2 text-sm rounded-md bg-primary text-white hover:bg-primary/90 disabled:opacity-50 transition-colors"
                >
                  创建
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      <div className="flex-1 overflow-y-auto px-4 pb-4">
        {loading ? (
          <div className="flex items-center justify-center h-40">
            <div className="text-muted-foreground">加载中...</div>
          </div>
        ) : foreshadowings.length === 0 ? (
          <div className="text-center py-12 text-muted-foreground">
            <Sparkles className="w-12 h-12 mx-auto mb-3 opacity-30" />
            <p>还没有伏笔记录，点击上方按钮添加</p>
          </div>
        ) : (
          <div className="space-y-3">
            {foreshadowings.map((item) => (
              <div
                key={item.id}
                className="p-4 rounded-lg border-2 hover:border-primary/50 transition-all bg-card dark:bg-gray-800"
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex items-center gap-2">
                    <span className="text-xl">{getTypeConfig(item.foreshadowing_type).icon}</span>
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="text-xs px-2 py-0.5 rounded-full bg-primary/10 text-primary">
                          第{item.chapter_number}章
                        </span>
                        <span className={`text-xs px-2 py-0.5 rounded-full ${getStatusColor(item.status)}`}>
                          {item.status === "planted" ? "已埋设" : item.status === "paid_off" ? "已回收" : item.status}
                        </span>
                      </div>
                      <h4 className="font-medium mt-1">{item.chapter_title}</h4>
                    </div>
                  </div>
                  <span className={`text-xs px-2 py-1 rounded-md ${getImportanceConfig(item.importance).color}`}>
                    {getImportanceConfig(item.importance).label}
                  </span>
                </div>

                <p className="text-sm text-gray-700 dark:text-gray-300 mb-2">{item.description}</p>

                {item.keywords.length > 0 && (
                  <div className="flex flex-wrap gap-1 mb-2">
                    {item.keywords.map((keyword, i) => (
                      <span
                        key={i}
                        className="text-xs px-2 py-0.5 rounded-full bg-muted text-muted-foreground"
                      >
                        {keyword}
                      </span>
                    ))}
                  </div>
                )}

                <div className="flex items-center justify-between text-xs text-muted-foreground">
                  <div className="flex items-center gap-3">
                    {item.expected_payoff_chapter && (
                      <span>
                        预计：第{item.expected_payoff_chapter}章
                      </span>
                    )}
                    {item.actual_payoff_chapter && (
                      <span className="text-green-600 dark:text-green-400">
                        回收于第{item.actual_payoff_chapter}章
                      </span>
                    )}
                  </div>

                  {item.status === "planted" && (
                    <button
                      onClick={() => {
                        const chapter = prompt("请输入回收的章节号：", item.expected_payoff_chapter?.toString() || "");
                        if (chapter) {
                          handleResolve(item.id, parseInt(chapter));
                        }
                      }}
                      className="text-primary hover:underline"
                    >
                      标记回收
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default ForeshadowingPanel;
