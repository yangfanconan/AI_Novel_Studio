import React, { useState } from "react";
import {
  ChevronDown,
  ChevronRight,
  Plus,
  Edit,
  Trash2,
  Link,
  Sparkles,
  Loader2,
} from "lucide-react";
import { PlotPointNode, PlotPoint } from "../types";
import { invoke } from "@tauri-apps/api/core";
import { AIGenerateDialog } from "./AIGenerateDialog";
import { ConfirmDialog } from "./ConfirmDialog";

interface PlotPointListProps {
  projectId: string;
  onEditPlotPoint: (plotPoint: PlotPointNode) => void;
  onLinkToChapter: (plotPoint: PlotPointNode) => void;
  onAIGeneratePlotPoints?: (data: any) => Promise<void>;
}

export function PlotPointList({
  projectId,
  onEditPlotPoint,
  onLinkToChapter,
  onAIGeneratePlotPoints,
}: PlotPointListProps) {
  const [plotPoints, setPlotPoints] = useState<PlotPointNode[]>([]);
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isAIDialogOpen, setIsAIDialogOpen] = useState(false);
  const [deleteConfirm, setDeleteConfirm] = useState<{
    isOpen: boolean;
    nodeId: string | null;
    nodeTitle: string;
  }>({
    isOpen: false,
    nodeId: null,
    nodeTitle: "",
  });

  const loadPlotPoints = async () => {
    setLoading(true);
    setError(null);
    try {
      const points = await invoke<PlotPoint[]>("get_plot_points", { projectId });
      const tree = buildPlotTree(points);
      setPlotPoints(tree);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  const buildPlotTree = (points: PlotPoint[]): PlotPointNode[] => {
    const nodeMap = new Map<string, PlotPointNode>();
    const rootNodes: PlotPointNode[] = [];

    points.forEach((point) => {
      nodeMap.set(point.id, { ...point, children: [] });
    });

    points.forEach((point) => {
      const node = nodeMap.get(point.id)!;
      if (point.parent_id && nodeMap.has(point.parent_id)) {
        nodeMap.get(point.parent_id)!.children.push(node);
      } else {
        rootNodes.push(node);
      }
    });

    return rootNodes;
  };

  const toggleExpand = (nodeId: string) => {
    setExpandedNodes((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(nodeId)) {
        newSet.delete(nodeId);
      } else {
        newSet.add(nodeId);
      }
      return newSet;
    });
  };

  const handleDeleteClick = (e: React.MouseEvent, nodeId: string, nodeTitle: string) => {
    e.stopPropagation();
    setDeleteConfirm({
      isOpen: true,
      nodeId,
      nodeTitle,
    });
  };

  const handleDeleteConfirm = async () => {
    const nodeId = deleteConfirm.nodeId;
    if (!nodeId) return;

    try {
      await invoke("delete_plot_point", { plotPointId: nodeId });
      await loadPlotPoints();
    } catch (err) {
      setError(err as string);
    }
    setDeleteConfirm({ isOpen: false, nodeId: null, nodeTitle: "" });
  };

  const handleDeleteCancel = () => {
    setDeleteConfirm({ isOpen: false, nodeId: null, nodeTitle: "" });
  };

  const handleAIConfirm = async (data: any) => {
    if (onAIGeneratePlotPoints) {
      await onAIGeneratePlotPoints(data);
    }
    await loadPlotPoints();
    setIsAIDialogOpen(false);
  };

  const renderNode = (node: PlotPointNode, level: number = 0) => {
    const isExpanded = expandedNodes.has(node.id);
    const hasChildren = node.children.length > 0;

    return (
      <div key={node.id} className="select-none">
        <div
          className="flex items-center gap-2 py-2 px-3 hover:bg-slate-100 dark:hover:bg-slate-800 rounded cursor-pointer group"
          style={{ paddingLeft: `${level * 16 + 12}px` }}
          onClick={() => hasChildren && toggleExpand(node.id)}
        >
          {hasChildren && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                toggleExpand(node.id);
              }}
              className="p-0.5 hover:bg-slate-200 dark:hover:bg-slate-700 rounded"
            >
              {isExpanded ? (
                <ChevronDown className="w-4 h-4" />
              ) : (
                <ChevronRight className="w-4 h-4" />
              )}
            </button>
          )}
          {!hasChildren && <div className="w-5" />}

          <div className="flex-1 min-w-0">
            <div className="font-medium text-sm text-slate-900 dark:text-slate-100 truncate">
              {node.title}
            </div>
            {node.description && (
              <div className="text-xs text-slate-500 dark:text-slate-400 truncate">
                {node.description}
              </div>
            )}
          </div>

          <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
            {node.chapter_id && (
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onLinkToChapter(node);
                }}
                className="p-1.5 hover:bg-slate-200 dark:hover:bg-slate-700 rounded text-blue-500"
                title="关联章节"
              >
                <Link className="w-4 h-4" />
              </button>
            )}
            <button
              onClick={(e) => {
                e.stopPropagation();
                onEditPlotPoint(node);
              }}
              className="p-1.5 hover:bg-slate-200 dark:hover:bg-slate-700 rounded"
              title="编辑"
            >
              <Edit className="w-4 h-4" />
            </button>
            <button
              onClick={(e) => handleDeleteClick(e, node.id, node.title)}
              className="p-1.5 hover:bg-slate-200 dark:hover:bg-slate-700 rounded text-red-500"
              title="删除"
            >
              <Trash2 className="w-4 h-4" />
            </button>
          </div>
        </div>

        {isExpanded && hasChildren && (
          <div>{node.children.map((child) => renderNode(child, level + 1))}</div>
        )}
      </div>
    );
  };

  React.useEffect(() => {
    loadPlotPoints();
  }, [projectId]);

  return (
    <>
      <div className="h-full flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h3 className="font-semibold text-slate-900 dark:text-slate-100">情节点大纲</h3>
          <div className="flex items-center gap-2">
            {onAIGeneratePlotPoints && (
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
                onEditPlotPoint({
                  id: "",
                  project_id: projectId,
                  parent_id: null,
                  title: "",
                  description: null,
                  note: null,
                  chapter_id: null,
                  status: "draft",
                  sort_order: 0,
                  level: 0,
                  created_at: new Date().toISOString(),
                  updated_at: new Date().toISOString(),
                  children: [],
                })
              }
              className="flex items-center gap-2 px-3 py-1.5 bg-blue-500 hover:bg-blue-600 text-white rounded text-sm font-medium"
            >
              <Plus className="w-4 h-4" />
              新建
            </button>
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

          {!loading && !error && plotPoints.length === 0 && (
            <div className="flex items-center justify-center h-full text-slate-400">
              <div className="text-center">
                <p className="text-sm">暂无情节点</p>
                <p className="text-xs mt-1">点击"新建"或"AI 生成"开始创建</p>
              </div>
            </div>
          )}

          {!loading && !error && plotPoints.length > 0 && (
            <div className="group">{plotPoints.map((node) => renderNode(node))}</div>
          )}
        </div>
      </div>

      <AIGenerateDialog
        isOpen={isAIDialogOpen}
        onClose={() => setIsAIDialogOpen(false)}
        type="plotpoint"
        projectId={projectId}
        onConfirm={handleAIConfirm}
      />

      <ConfirmDialog
        isOpen={deleteConfirm.isOpen}
        title="删除情节点"
        message={`确定要删除情节点"${deleteConfirm.nodeTitle}"吗？子节点也会被删除。`}
        confirmText="删除"
        cancelText="取消"
        variant="danger"
        onConfirm={handleDeleteConfirm}
        onCancel={handleDeleteCancel}
      />
    </>
  );
}
