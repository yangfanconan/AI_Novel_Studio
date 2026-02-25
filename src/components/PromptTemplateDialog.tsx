import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Plus, Edit2, Trash2, RotateCcw, Save, X, FileText } from "lucide-react";

interface PromptTemplate {
  id: string;
  name: string;
  category: string;
  description: string | null;
  system_prompt: string;
  user_prompt_template: string;
  variables: string[];
  is_default: boolean;
  is_custom: boolean;
  created_at: string;
  updated_at: string;
}

interface PromptTemplateDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

const categories = [
  { id: "writing", name: "写作", icon: "✍️" },
  { id: "generation", name: "生成", icon: "🤖" },
  { id: "analysis", name: "分析", icon: "📊" },
  { id: "custom", name: "自定义", icon: "⚙️" },
];

export default function PromptTemplateDialog({ isOpen, onClose }: PromptTemplateDialogProps) {
  const [templates, setTemplates] = useState<PromptTemplate[]>([]);
  const [selectedTemplate, setSelectedTemplate] = useState<PromptTemplate | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [editForm, setEditForm] = useState<Partial<PromptTemplate>>({});
  const [isLoading, setIsLoading] = useState(false);
  const [activeCategory, setActiveCategory] = useState<string>("all");

  useEffect(() => {
    if (isOpen) {
      loadTemplates();
    }
  }, [isOpen]);

  const loadTemplates = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<PromptTemplate[]>("get_custom_prompt_templates");
      setTemplates(result);

      await invoke("initialize_default_prompt_templates");
      const updatedResult = await invoke<PromptTemplate[]>("get_custom_prompt_templates");
      setTemplates(updatedResult);
    } catch (error) {
      console.error("Failed to load templates:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelectTemplate = (template: PromptTemplate) => {
    setSelectedTemplate(template);
    setIsEditing(false);
    setEditForm({});
  };

  const handleCreateNew = () => {
    const newTemplate: Partial<PromptTemplate> = {
      name: "新提示词模板",
      category: "custom",
      description: "",
      system_prompt: "你是一位专业的AI助手。",
      user_prompt_template: "请根据以下内容进行操作：\n{input}",
      variables: ["input"],
    };
    setSelectedTemplate(null);
    setEditForm(newTemplate);
    setIsEditing(true);
  };

  const handleEdit = () => {
    if (selectedTemplate) {
      setEditForm({ ...selectedTemplate });
      setIsEditing(true);
    }
  };

  const handleSave = async () => {
    if (!editForm.name || !editForm.system_prompt || !editForm.user_prompt_template) {
      alert("请填写必要字段");
      return;
    }

    setIsLoading(true);
    try {
      if (selectedTemplate) {
        await invoke("update_prompt_template", {
          request: {
            id: selectedTemplate.id,
            name: editForm.name,
            category: editForm.category || "custom",
            description: editForm.description,
            system_prompt: editForm.system_prompt,
            user_prompt_template: editForm.user_prompt_template,
            variables: editForm.variables || [],
          },
        });
      } else {
        const created = await invoke<PromptTemplate>("create_prompt_template", {
          request: {
            name: editForm.name,
            category: editForm.category || "custom",
            description: editForm.description,
            system_prompt: editForm.system_prompt,
            user_prompt_template: editForm.user_prompt_template,
            variables: editForm.variables || [],
          },
        });
        setSelectedTemplate(created);
      }
      await loadTemplates();
      setIsEditing(false);
      setEditForm({});
    } catch (error) {
      console.error("Failed to save template:", error);
      alert("保存失败: " + error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleDelete = async () => {
    if (!selectedTemplate || selectedTemplate.is_default) {
      alert("无法删除默认模板");
      return;
    }

    if (!confirm("确定要删除此模板吗？")) {
      return;
    }

    setIsLoading(true);
    try {
      await invoke("delete_prompt_template", { id: selectedTemplate.id });
      setSelectedTemplate(null);
      await loadTemplates();
    } catch (error) {
      console.error("Failed to delete template:", error);
      alert("删除失败: " + error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleReset = async () => {
    if (!selectedTemplate || !selectedTemplate.is_default) {
      return;
    }

    if (!confirm("确定要重置此模板为默认值吗？")) {
      return;
    }

    setIsLoading(true);
    try {
      await invoke("reset_prompt_template_to_default", { id: selectedTemplate.id });
      await loadTemplates();
      setIsEditing(false);
    } catch (error) {
      console.error("Failed to reset template:", error);
      alert("重置失败: " + error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCancel = () => {
    setIsEditing(false);
    setEditForm({});
  };

  const filteredTemplates =
    activeCategory === "all" ? templates : templates.filter((t) => t.category === activeCategory);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-5xl max-h-[90vh] overflow-hidden flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold text-slate-800 dark:text-slate-100">
            提示词模板管理
          </h2>
          <button
            onClick={onClose}
            className="text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="flex flex-1 overflow-hidden">
          <div className="w-64 border-r border-slate-200 dark:border-slate-700 flex flex-col">
            <div className="p-3 border-b border-slate-200 dark:border-slate-700">
              <button
                onClick={handleCreateNew}
                className="w-full flex items-center justify-center gap-2 px-3 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
              >
                <Plus className="w-4 h-4" />
                新建模板
              </button>
            </div>

            <div className="p-2 border-b border-slate-200 dark:border-slate-700">
              <div className="flex flex-wrap gap-1">
                <button
                  onClick={() => setActiveCategory("all")}
                  className={`px-2 py-1 text-xs rounded ${
                    activeCategory === "all"
                      ? "bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300"
                      : "bg-slate-100 text-slate-600 dark:bg-slate-700 dark:text-slate-400"
                  }`}
                >
                  全部
                </button>
                {categories.map((cat) => (
                  <button
                    key={cat.id}
                    onClick={() => setActiveCategory(cat.id)}
                    className={`px-2 py-1 text-xs rounded ${
                      activeCategory === cat.id
                        ? "bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300"
                        : "bg-slate-100 text-slate-600 dark:bg-slate-700 dark:text-slate-400"
                    }`}
                  >
                    {cat.icon} {cat.name}
                  </button>
                ))}
              </div>
            </div>

            <div className="flex-1 overflow-y-auto">
              {isLoading && templates.length === 0 ? (
                <div className="p-4 text-center text-slate-500">加载中...</div>
              ) : (
                filteredTemplates.map((template) => (
                  <div
                    key={template.id}
                    onClick={() => handleSelectTemplate(template)}
                    className={`p-3 cursor-pointer border-b border-slate-100 dark:border-slate-700 ${
                      selectedTemplate?.id === template.id
                        ? "bg-blue-50 dark:bg-blue-900/30"
                        : "hover:bg-slate-50 dark:hover:bg-slate-700/50"
                    }`}
                  >
                    <div className="flex items-center gap-2">
                      <FileText className="w-4 h-4 text-slate-400" />
                      <span className="text-sm font-medium text-slate-700 dark:text-slate-300 truncate">
                        {template.name}
                      </span>
                    </div>
                    <div className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                      {categories.find((c) => c.id === template.category)?.name ||
                        template.category}
                      {template.is_default && " · 默认"}
                    </div>
                  </div>
                ))
              )}
            </div>
          </div>

          <div className="flex-1 flex flex-col overflow-hidden">
            {selectedTemplate || isEditing ? (
              <>
                <div className="flex items-center justify-between p-3 border-b border-slate-200 dark:border-slate-700">
                  <div className="flex items-center gap-2">
                    {isEditing ? (
                      <input
                        type="text"
                        value={editForm.name || ""}
                        onChange={(e) => setEditForm({ ...editForm, name: e.target.value })}
                        className="px-2 py-1 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                        placeholder="模板名称"
                      />
                    ) : (
                      <span className="font-medium text-slate-800 dark:text-slate-200">
                        {selectedTemplate?.name}
                      </span>
                    )}
                    {selectedTemplate?.is_default && (
                      <span className="px-2 py-0.5 text-xs bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-300 rounded">
                        默认
                      </span>
                    )}
                  </div>

                  <div className="flex items-center gap-2">
                    {isEditing ? (
                      <>
                        <button
                          onClick={handleCancel}
                          className="p-1.5 text-slate-500 hover:text-slate-700 dark:text-slate-400"
                        >
                          <X className="w-4 h-4" />
                        </button>
                        <button
                          onClick={handleSave}
                          disabled={isLoading}
                          className="p-1.5 text-green-600 hover:text-green-700 dark:text-green-400"
                        >
                          <Save className="w-4 h-4" />
                        </button>
                      </>
                    ) : (
                      <>
                        <button
                          onClick={handleEdit}
                          className="p-1.5 text-slate-500 hover:text-slate-700 dark:text-slate-400"
                          title="编辑"
                        >
                          <Edit2 className="w-4 h-4" />
                        </button>
                        {selectedTemplate?.is_default && (
                          <button
                            onClick={handleReset}
                            className="p-1.5 text-slate-500 hover:text-slate-700 dark:text-slate-400"
                            title="重置为默认"
                          >
                            <RotateCcw className="w-4 h-4" />
                          </button>
                        )}
                        {!selectedTemplate?.is_default && (
                          <button
                            onClick={handleDelete}
                            className="p-1.5 text-red-500 hover:text-red-700 dark:text-red-400"
                            title="删除"
                          >
                            <Trash2 className="w-4 h-4" />
                          </button>
                        )}
                      </>
                    )}
                  </div>
                </div>

                <div className="flex-1 overflow-y-auto p-4 space-y-4">
                  {isEditing ? (
                    <>
                      <div>
                        <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                          分类
                        </label>
                        <select
                          value={editForm.category || "custom"}
                          onChange={(e) => setEditForm({ ...editForm, category: e.target.value })}
                          className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                        >
                          {categories.map((cat) => (
                            <option key={cat.id} value={cat.id}>
                              {cat.icon} {cat.name}
                            </option>
                          ))}
                        </select>
                      </div>

                      <div>
                        <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                          描述
                        </label>
                        <input
                          type="text"
                          value={editForm.description || ""}
                          onChange={(e) =>
                            setEditForm({ ...editForm, description: e.target.value })
                          }
                          className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                          placeholder="模板描述"
                        />
                      </div>
                    </>
                  ) : (
                    selectedTemplate?.description && (
                      <div className="text-sm text-slate-600 dark:text-slate-400">
                        {selectedTemplate.description}
                      </div>
                    )
                  )}

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                      系统提示词 (System Prompt)
                    </label>
                    {isEditing ? (
                      <textarea
                        value={editForm.system_prompt || ""}
                        onChange={(e) =>
                          setEditForm({ ...editForm, system_prompt: e.target.value })
                        }
                        className="w-full h-40 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200 font-mono text-sm"
                        placeholder="设置AI的角色和行为规则..."
                      />
                    ) : (
                      <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg font-mono text-sm text-slate-700 dark:text-slate-300 whitespace-pre-wrap">
                        {selectedTemplate?.system_prompt}
                      </div>
                    )}
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                      用户提示词模板 (User Prompt Template)
                    </label>
                    {isEditing ? (
                      <textarea
                        value={editForm.user_prompt_template || ""}
                        onChange={(e) =>
                          setEditForm({ ...editForm, user_prompt_template: e.target.value })
                        }
                        className="w-full h-40 px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200 font-mono text-sm"
                        placeholder="使用 {变量名} 作为占位符..."
                      />
                    ) : (
                      <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg font-mono text-sm text-slate-700 dark:text-slate-300 whitespace-pre-wrap">
                        {selectedTemplate?.user_prompt_template}
                      </div>
                    )}
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                      变量列表
                    </label>
                    <div className="flex flex-wrap gap-2">
                      {(isEditing ? editForm.variables : selectedTemplate?.variables)?.map(
                        (variable, index) => (
                          <span
                            key={index}
                            className="px-2 py-1 bg-purple-100 text-purple-700 dark:bg-purple-900/50 dark:text-purple-300 rounded text-sm font-mono"
                          >
                            {"{"}
                            {variable}
                            {"}"}
                          </span>
                        )
                      )}
                    </div>
                  </div>
                </div>
              </>
            ) : (
              <div className="flex-1 flex items-center justify-center text-slate-500 dark:text-slate-400">
                选择一个模板查看详情，或点击"新建模板"创建新模板
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
