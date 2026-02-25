import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  X,
  Plus,
  Edit2,
  Trash2,
  ChevronRight,
  ChevronDown,
  FileText,
  Folder,
  Sparkles,
  LayoutTemplate,
  Save,
  RefreshCw,
} from "lucide-react";

interface OutlineNode {
  id: string;
  project_id: string;
  parent_id: string | null;
  title: string;
  content: string;
  node_type: "arc" | "chapter" | "scene" | "beat";
  sort_order: number;
  status: "planned" | "inprogress" | "completed" | "skipped";
  word_count_target: number | null;
  word_count_actual: number;
  created_at: string;
  updated_at: string;
}

interface OutlineTemplate {
  id: string;
  name: string;
  description: string;
  structure: TemplateNode[];
}

interface TemplateNode {
  title: string;
  node_type: string;
  description: string;
  children: TemplateNode[];
}

interface OutlinePanelProps {
  projectId: string;
  isOpen: boolean;
  onClose: () => void;
}

const nodeTypeIcons: Record<string, React.ReactNode> = {
  arc: <Folder className="w-4 h-4 text-purple-500" />,
  chapter: <FileText className="w-4 h-4 text-blue-500" />,
  scene: <FileText className="w-4 h-4 text-green-500" />,
  beat: <FileText className="w-4 h-4 text-gray-500" />,
};

const statusColors: Record<string, string> = {
  planned: "bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300",
  inprogress: "bg-blue-100 text-blue-600 dark:bg-blue-900 dark:text-blue-300",
  completed: "bg-green-100 text-green-600 dark:bg-green-900 dark:text-green-300",
  skipped: "bg-yellow-100 text-yellow-600 dark:bg-yellow-900 dark:text-yellow-300",
};

const statusNames: Record<string, string> = {
  planned: "计划中",
  inprogress: "进行中",
  completed: "已完成",
  skipped: "已跳过",
};

export default function OutlinePanel({ projectId, isOpen, onClose }: OutlinePanelProps) {
  const [nodes, setNodes] = useState<OutlineNode[]>([]);
  const [templates, setTemplates] = useState<OutlineTemplate[]>([]);
  const [selectedNode, setSelectedNode] = useState<OutlineNode | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [editForm, setEditForm] = useState<Partial<OutlineNode>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [showTemplates, setShowTemplates] = useState(false);
  const [showGenerateDialog, setShowGenerateDialog] = useState(false);
  const [expandedNodes, setExpandedNodes] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (isOpen && projectId) {
      loadData();
    }
  }, [isOpen, projectId]);

  const loadData = async () => {
    setIsLoading(true);
    try {
      const [nodesResult, templatesResult] = await Promise.all([
        invoke<OutlineNode[]>("get_outline_nodes", { projectId }),
        invoke<OutlineTemplate[]>("get_outline_templates"),
      ]);
      setNodes(nodesResult);
      setTemplates(templatesResult);
    } catch (error) {
      console.error("Failed to load outline data:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const buildTree = (nodes: OutlineNode[], parentId: string | null = null): OutlineNode[] => {
    return nodes
      .filter((n) => n.parent_id === parentId)
      .sort((a, b) => a.sort_order - b.sort_order)
      .map((node) => ({
        ...node,
        children: buildTree(nodes, node.id),
      })) as OutlineNode[];
  };

  const toggleExpand = (nodeId: string) => {
    setExpandedNodes((prev) => {
      const next = new Set(prev);
      if (next.has(nodeId)) {
        next.delete(nodeId);
      } else {
        next.add(nodeId);
      }
      return next;
    });
  };

  const handleCreateNode = async (
    parentId: string | null = null,
    type: "arc" | "chapter" | "scene" = "chapter"
  ) => {
    try {
      const newNode = await invoke<OutlineNode>("create_outline_node", {
        request: {
          project_id: projectId,
          parent_id: parentId,
          title: `新${type === "arc" ? "故事弧" : type === "chapter" ? "章节" : "场景"}`,
          content: "",
          node_type: type,
          sort_order: nodes.filter((n) => n.parent_id === parentId).length,
        },
      });
      setNodes([...nodes, newNode]);
      setSelectedNode(newNode);
      setIsEditing(true);
      setEditForm({ ...newNode });
    } catch (error) {
      console.error("Failed to create node:", error);
    }
  };

  const handleUpdateNode = async () => {
    if (!selectedNode || !editForm.title) return;

    setIsLoading(true);
    try {
      const updated = await invoke<OutlineNode>("update_outline_node", {
        request: {
          id: selectedNode.id,
          title: editForm.title,
          content: editForm.content,
          status: editForm.status,
          word_count_target: editForm.word_count_target,
        },
      });
      setNodes(nodes.map((n) => (n.id === updated.id ? updated : n)));
      setSelectedNode(updated);
      setIsEditing(false);
    } catch (error) {
      console.error("Failed to update node:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDeleteNode = async (nodeId: string) => {
    if (!confirm("确定要删除此节点及其所有子节点吗？")) return;

    try {
      await invoke("delete_outline_node", { id: nodeId });
      setNodes(nodes.filter((n) => n.id !== nodeId && n.parent_id !== nodeId));
      if (selectedNode?.id === nodeId) {
        setSelectedNode(null);
      }
    } catch (error) {
      console.error("Failed to delete node:", error);
    }
  };

  const handleApplyTemplate = async (templateId: string) => {
    setIsLoading(true);
    try {
      const newNodes = await invoke<OutlineNode[]>("apply_outline_template", {
        projectId,
        templateId,
      });
      setNodes([...nodes, ...newNodes]);
      setShowTemplates(false);
    } catch (error) {
      console.error("Failed to apply template:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleGenerateOutline = async (params: {
    genre: string;
    theme: string;
    characters: string[];
    chapters: number;
    wordsPerChapter: number;
  }) => {
    setIsLoading(true);
    try {
      const result = await invoke("generate_outline_with_ai", {
        request: {
          project_id: projectId,
          genre: params.genre,
          theme: params.theme || null,
          main_characters: params.characters,
          target_chapters: params.chapters,
          target_words_per_chapter: params.wordsPerChapter,
          style: null,
        },
      });

      const savedNodes = await invoke<OutlineNode[]>("save_generated_outline", {
        projectId,
        outline: result,
      });

      setNodes([...nodes, ...savedNodes]);
      setShowGenerateDialog(false);
    } catch (error) {
      console.error("Failed to generate outline:", error);
      alert("生成失败: " + error);
    } finally {
      setIsLoading(false);
    }
  };

  const renderNode = (node: OutlineNode, depth: number = 0) => {
    const children = nodes.filter((n) => n.parent_id === node.id);
    const hasChildren = children.length > 0;
    const isExpanded = expandedNodes.has(node.id);

    return (
      <div key={node.id}>
        <div
          className={`flex items-center gap-2 px-2 py-1.5 cursor-pointer hover:bg-slate-100 dark:hover:bg-slate-700 rounded ${
            selectedNode?.id === node.id ? "bg-blue-50 dark:bg-blue-900/30" : ""
          }`}
          style={{ paddingLeft: `${depth * 16 + 8}px` }}
          onClick={() => setSelectedNode(node)}
        >
          {hasChildren && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                toggleExpand(node.id);
              }}
            >
              {isExpanded ? (
                <ChevronDown className="w-4 h-4" />
              ) : (
                <ChevronRight className="w-4 h-4" />
              )}
            </button>
          )}
          {!hasChildren && <span className="w-4" />}

          {nodeTypeIcons[node.node_type]}
          <span className="flex-1 text-sm truncate">{node.title}</span>
          <span className={`px-1.5 py-0.5 text-xs rounded ${statusColors[node.status]}`}>
            {statusNames[node.status]}
          </span>
        </div>

        {isExpanded && hasChildren && (
          <div>
            {children
              .sort((a, b) => a.sort_order - b.sort_order)
              .map((child) => renderNode(child, depth + 1))}
          </div>
        )}
      </div>
    );
  };

  if (!isOpen) return null;

  const rootNodes = nodes
    .filter((n) => n.parent_id === null)
    .sort((a, b) => a.sort_order - b.sort_order);

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-5xl max-h-[90vh] overflow-hidden flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold text-slate-800 dark:text-slate-100">大纲管理</h2>
          <div className="flex items-center gap-2">
            <button
              onClick={() => handleCreateNode(null, "arc")}
              className="flex items-center gap-1 px-3 py-1.5 text-sm bg-blue-500 text-white rounded-lg hover:bg-blue-600"
            >
              <Plus className="w-4 h-4" />
              新建
            </button>
            <button
              onClick={() => setShowTemplates(true)}
              className="flex items-center gap-1 px-3 py-1.5 text-sm bg-purple-500 text-white rounded-lg hover:bg-purple-600"
            >
              <LayoutTemplate className="w-4 h-4" />
              模板
            </button>
            <button
              onClick={() => setShowGenerateDialog(true)}
              className="flex items-center gap-1 px-3 py-1.5 text-sm bg-green-500 text-white rounded-lg hover:bg-green-600"
            >
              <Sparkles className="w-4 h-4" />
              AI生成
            </button>
            <button
              onClick={onClose}
              className="text-slate-500 hover:text-slate-700 dark:text-slate-400"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        <div className="flex flex-1 overflow-hidden">
          <div className="w-72 border-r border-slate-200 dark:border-slate-700 overflow-y-auto">
            {isLoading && nodes.length === 0 ? (
              <div className="p-4 text-center text-slate-500">加载中...</div>
            ) : rootNodes.length === 0 ? (
              <div className="p-4 text-center text-slate-500">
                <p className="mb-2">暂无大纲</p>
                <p className="text-sm">使用模板或AI生成来创建大纲</p>
              </div>
            ) : (
              <div className="py-2">{rootNodes.map((node) => renderNode(node))}</div>
            )}
          </div>

          <div className="flex-1 overflow-y-auto p-4">
            {selectedNode ? (
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  {isEditing ? (
                    <input
                      type="text"
                      value={editForm.title || ""}
                      onChange={(e) => setEditForm({ ...editForm, title: e.target.value })}
                      className="text-lg font-medium px-2 py-1 border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700"
                    />
                  ) : (
                    <h3 className="text-lg font-medium">{selectedNode.title}</h3>
                  )}
                  <div className="flex items-center gap-2">
                    {isEditing ? (
                      <>
                        <button
                          onClick={() => setIsEditing(false)}
                          className="p-1.5 text-slate-500"
                        >
                          <X className="w-4 h-4" />
                        </button>
                        <button onClick={handleUpdateNode} className="p-1.5 text-green-600">
                          <Save className="w-4 h-4" />
                        </button>
                      </>
                    ) : (
                      <>
                        <button
                          onClick={() => {
                            setIsEditing(true);
                            setEditForm({ ...selectedNode });
                          }}
                          className="p-1.5 text-slate-500"
                        >
                          <Edit2 className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => handleCreateNode(selectedNode.id, "scene")}
                          className="p-1.5 text-blue-500"
                        >
                          <Plus className="w-4 h-4" />
                        </button>
                        <button
                          onClick={() => handleDeleteNode(selectedNode.id)}
                          className="p-1.5 text-red-500"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </>
                    )}
                  </div>
                </div>

                <div className="flex items-center gap-4 text-sm">
                  <span className={`px-2 py-1 rounded ${statusColors[selectedNode.status]}`}>
                    {statusNames[selectedNode.status]}
                  </span>
                  {selectedNode.word_count_target && (
                    <span className="text-slate-500">
                      目标: {selectedNode.word_count_target} 字
                    </span>
                  )}
                  {selectedNode.word_count_actual > 0 && (
                    <span className="text-slate-500">
                      实际: {selectedNode.word_count_actual} 字
                    </span>
                  )}
                </div>

                {isEditing ? (
                  <>
                    <div>
                      <label className="block text-sm font-medium mb-1">状态</label>
                      <select
                        value={editForm.status || "planned"}
                        onChange={(e) =>
                          setEditForm({ ...editForm, status: e.target.value as any })
                        }
                        className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
                      >
                        <option value="planned">计划中</option>
                        <option value="inprogress">进行中</option>
                        <option value="completed">已完成</option>
                        <option value="skipped">已跳过</option>
                      </select>
                    </div>
                    <div>
                      <label className="block text-sm font-medium mb-1">目标字数</label>
                      <input
                        type="number"
                        value={editForm.word_count_target || ""}
                        onChange={(e) =>
                          setEditForm({
                            ...editForm,
                            word_count_target: e.target.value ? parseInt(e.target.value) : null,
                          })
                        }
                        className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
                      />
                    </div>
                    <div>
                      <label className="block text-sm font-medium mb-1">内容概要</label>
                      <textarea
                        value={editForm.content || ""}
                        onChange={(e) => setEditForm({ ...editForm, content: e.target.value })}
                        className="w-full h-40 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
                        placeholder="输入章节概要..."
                      />
                    </div>
                  </>
                ) : (
                  <div className="p-4 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                    {selectedNode.content || "暂无内容概要"}
                  </div>
                )}
              </div>
            ) : (
              <div className="h-full flex items-center justify-center text-slate-500">
                选择一个节点查看详情
              </div>
            )}
          </div>
        </div>
      </div>

      {showTemplates && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-lg p-6">
            <h3 className="text-lg font-semibold mb-4">选择大纲模板</h3>
            <div className="space-y-3">
              {templates.map((template) => (
                <div
                  key={template.id}
                  onClick={() => handleApplyTemplate(template.id)}
                  className="p-4 border border-slate-200 dark:border-slate-600 rounded-lg cursor-pointer hover:border-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20"
                >
                  <div className="font-medium">{template.name}</div>
                  <div className="text-sm text-slate-500 mt-1">{template.description}</div>
                </div>
              ))}
            </div>
            <button
              onClick={() => setShowTemplates(false)}
              className="mt-4 px-4 py-2 text-slate-600 hover:text-slate-800"
            >
              取消
            </button>
          </div>
        </div>
      )}

      {showGenerateDialog && (
        <GenerateOutlineDialog
          onGenerate={handleGenerateOutline}
          onClose={() => setShowGenerateDialog(false)}
          isLoading={isLoading}
        />
      )}
    </div>
  );
}

function GenerateOutlineDialog({
  onGenerate,
  onClose,
  isLoading,
}: {
  onGenerate: (params: any) => void;
  onClose: () => void;
  isLoading: boolean;
}) {
  const [genre, setGenre] = useState("玄幻");
  const [theme, setTheme] = useState("");
  const [characters, setCharacters] = useState("");
  const [chapters, setChapters] = useState(20);
  const [wordsPerChapter, setWordsPerChapter] = useState(3000);

  const handleGenerate = () => {
    onGenerate({
      genre,
      theme,
      characters: characters
        .split(/[,，、]/)
        .map((c) => c.trim())
        .filter(Boolean),
      chapters,
      wordsPerChapter,
    });
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-[60]">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-md p-6">
        <h3 className="text-lg font-semibold mb-4">AI 生成大纲</h3>
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">类型</label>
            <select
              value={genre}
              onChange={(e) => setGenre(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
            >
              <option value="玄幻">玄幻</option>
              <option value="仙侠">仙侠</option>
              <option value="都市">都市</option>
              <option value="历史">历史</option>
              <option value="科幻">科幻</option>
              <option value="悬疑">悬疑</option>
              <option value="言情">言情</option>
              <option value="其他">其他</option>
            </select>
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">主题</label>
            <input
              type="text"
              value={theme}
              onChange={(e) => setTheme(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
              placeholder="如：复仇、成长、爱情..."
            />
          </div>
          <div>
            <label className="block text-sm font-medium mb-1">主要角色</label>
            <input
              type="text"
              value={characters}
              onChange={(e) => setCharacters(e.target.value)}
              className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
              placeholder="用逗号分隔多个角色名"
            />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium mb-1">章节数</label>
              <input
                type="number"
                value={chapters}
                onChange={(e) => setChapters(parseInt(e.target.value) || 20)}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-1">每章字数</label>
              <input
                type="number"
                value={wordsPerChapter}
                onChange={(e) => setWordsPerChapter(parseInt(e.target.value) || 3000)}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700"
              />
            </div>
          </div>
        </div>
        <div className="flex justify-end gap-2 mt-6">
          <button onClick={onClose} className="px-4 py-2 text-slate-600">
            取消
          </button>
          <button
            onClick={handleGenerate}
            disabled={isLoading}
            className="flex items-center gap-2 px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 disabled:opacity-50"
          >
            {isLoading && <RefreshCw className="w-4 h-4 animate-spin" />}
            生成大纲
          </button>
        </div>
      </div>
    </div>
  );
}
