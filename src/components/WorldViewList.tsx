import React, { useState } from "react";
import { Plus, Edit, Trash2, Tag, Calendar, Sparkles, Loader2 } from "lucide-react";
import { WorldView, GeneratedWorldView } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { AIGenerateDialog } from "./AIGenerateDialog";

interface WorldViewListProps {
  projectId: string;
  onEditWorldView: (worldView: WorldView) => void;
  onAIGenerateWorldView?: (data: unknown) => Promise<void>;
}

const CATEGORIES = [
  { id: "geography", name: "地理环境", icon: "🌍" },
  { id: "history", name: "历史背景", icon: "📜" },
  { id: "culture", name: "文化风俗", icon: "🎭" },
  { id: "politics", name: "政治制度", icon: "🏛️" },
  { id: "economy", name: "经济体系", icon: "💰" },
  { id: "magic", name: "魔法/科技", icon: "✨" },
  { id: "religion", name: "宗教信仰", icon: "🕍️" },
  { id: "races", name: "种族生物", icon: "👥" },
  { id: "other", name: "其他", icon: "📝" },
];

export function WorldViewList({
  projectId,
  onEditWorldView,
  onAIGenerateWorldView,
}: WorldViewListProps) {
  const [worldViews, setWorldViews] = useState<WorldView[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isAIDialogOpen, setIsAIDialogOpen] = useState(false);

  const loadWorldViews = async (category?: string) => {
    setLoading(true);
    setError(null);
    try {
      const views = await invoke<WorldView[]>("get_world_views", {
        projectId,
        category,
      });
      setWorldViews(views);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const handleCategoryChange = (categoryId: string) => {
    setSelectedCategory(categoryId);
    loadWorldViews(categoryId === "" ? undefined : categoryId);
  };

  const handleDelete = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    if (!confirm("确定要删除这个世界观设定吗？")) {
      return;
    }
    try {
      await invoke("delete_world_view", { id });
      await loadWorldViews(selectedCategory === "" ? undefined : selectedCategory);
    } catch (err) {
      setError(err as string);
    }
  };

  const handleAIConfirm = async (data: unknown) => {
    if (onAIGenerateWorldView) {
      await onAIGenerateWorldView(data);
    }
    await loadWorldViews(selectedCategory === "" ? undefined : selectedCategory);
    setIsAIDialogOpen(false);
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
    });
  };

  React.useEffect(() => {
    loadWorldViews();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [projectId]);

  return (
    <>
      <div className="h-full flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h3 className="font-semibold text-slate-900 dark:text-slate-100">世界观设定</h3>
          <div className="flex items-center gap-2">
            {onAIGenerateWorldView && (
              <button
                onClick={() => setIsAIDialogOpen(true)}
                className="flex items-center gap-2 px-3 py-1.5 bg-purple-500 hover:bg-purple-600 text-white rounded text-sm font-medium transition-colors"
              >
                <Sparkles className="w-4 h-4" />
                AI 生成
              </button>
            )}
            <button
              onClick={() =>
                onEditWorldView({
                  id: "",
                  project_id: projectId,
                  category: selectedCategory || "geography",
                  title: "",
                  content: "",
                  tags: null,
                  status: "draft",
                  created_at: new Date().toISOString(),
                  updated_at: new Date().toISOString(),
                })
              }
              className="flex items-center gap-2 px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm font-medium"
            >
              <Plus className="w-4 h-4" />
              新建
            </button>
          </div>
        </div>

        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex gap-2 overflow-x-auto pb-2">
            <button
              onClick={() => handleCategoryChange("")}
              className={`px-3 py-1.5 rounded-full text-sm font-medium whitespace-nowrap transition-colors ${
                selectedCategory === ""
                  ? "bg-blue-500 text-white"
                  : "bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700"
              }`}
            >
              全部
            </button>
            {CATEGORIES.map((cat) => (
              <button
                key={cat.id}
                onClick={() => handleCategoryChange(cat.id)}
                className={`px-3 py-1.5 rounded-full text-sm font-medium whitespace-nowrap transition-colors ${
                  selectedCategory === cat.id
                    ? "bg-blue-500 text-white"
                    : "bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700"
                }`}
              >
                {cat.icon} {cat.name}
              </button>
            ))}
          </div>
        </div>

        <div className="flex-1 overflow-auto">
          {loading && (
            <div className="flex items-center justify-center h-full text-slate-500">
              <Loader2 className="w-5 h-5 animate-spin mr-2" />
              加载中...
            </div>
          )}

          {error && <div className="p-4 text-red-500 text-sm">{error}</div>}

          {!loading && !error && worldViews.length === 0 && (
            <div className="flex items-center justify-center h-full text-slate-400">
              <div className="text-center">
                <p className="text-sm">暂无世界观设定</p>
                <p className="text-xs mt-1">点击"新建"或"AI 生成"开始创建</p>
              </div>
            </div>
          )}

          {!loading && !error && worldViews.length > 0 && (
            <div className="p-4 space-y-3">
              {worldViews.map((view) => (
                <div
                  key={view.id}
                  className="p-4 bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-700 hover:shadow-md transition-shadow group"
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-2">
                        <span className="text-lg">
                          {CATEGORIES.find((c) => c.id === view.category)?.icon || "📝"}
                        </span>
                        <h4 className="font-semibold text-slate-900 dark:text-slate-100 truncate">
                          {view.title}
                        </h4>
                        {view.tags && (
                          <span className="flex items-center gap-1 px-2 py-0.5 bg-slate-100 dark:bg-slate-700 text-xs text-slate-600 dark:text-slate-400 rounded">
                            <Tag className="w-3 h-3" />
                            {view.tags}
                          </span>
                        )}
                      </div>
                      <p className="text-sm text-slate-600 dark:text-slate-400 line-clamp-2">
                        {view.content}
                      </p>
                      <div className="flex items-center gap-4 mt-2 text-xs text-slate-500 dark:text-slate-500">
                        <span className="flex items-center gap-1">
                          <Calendar className="w-3 h-3" />
                          {formatDate(view.created_at)}
                        </span>
                        <span
                          className={`px-2 py-0.5 rounded ${
                            view.status === "draft"
                              ? "bg-slate-100 dark:bg-slate-700 text-slate-600 dark:text-slate-400"
                              : view.status === "in_progress"
                                ? "bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400"
                                : "bg-green-100 dark:bg-green-900/30 text-green-600 dark:text-green-400"
                          }`}
                        >
                          {view.status === "draft"
                            ? "草稿"
                            : view.status === "in_progress"
                              ? "进行中"
                              : "已完成"}
                        </span>
                      </div>
                    </div>
                    <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
                      <button
                        onClick={() => onEditWorldView(view)}
                        className="p-1.5 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
                        title="编辑"
                      >
                        <Edit className="w-4 h-4" />
                      </button>
                      <button
                        onClick={(e) => handleDelete(e, view.id)}
                        className="p-1.5 hover:bg-red-50 dark:hover:bg-red-900/30 rounded text-red-500"
                        title="删除"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      <AIGenerateDialog
        isOpen={isAIDialogOpen}
        onClose={() => setIsAIDialogOpen(false)}
        type="worldview"
        projectId={projectId}
        onConfirm={handleAIConfirm}
      />
    </>
  );
}
