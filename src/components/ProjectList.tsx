import React, { useState, useEffect } from "react";
import {
  Plus,
  Folder,
  Trash2,
  Edit2,
  MoreHorizontal,
  RotateCcw,
  Settings,
  Download,
  Puzzle,
  Upload,
  FileText,
  Image,
  List,
  Layers,
  SearchCode,
} from "lucide-react";
import { projectService } from "../services/api";
import { uiLogger } from "../utils/uiLogger";
import { ConfirmDialog } from "./ConfirmDialog";
import { Project } from "../types";

interface ProjectListProps {
  projects: Project[];
  currentProject: Project | null;
  onSelectProject: (project: Project) => void;
  onCreateProject: () => void;
  onRefresh?: () => void;
  onOpenSettings?: () => void;
  onOpenPluginManager?: () => void;
  onOpenImportDialog?: () => void;
  onOpenPromptTemplates?: () => void;
  onOpenMultimediaSettings?: () => void;
  onOpenOutline?: () => void;
  onOpenBatchGenerator?: () => void;
  onOpenReverseAnalysis?: () => void;
  onDeleteProject?: (projectId: string) => void;
  onRenameProject?: () => void;
  onExportProject?: (projectId: string) => void;
}

export const ProjectList: React.FC<ProjectListProps> = ({
  projects,
  currentProject,
  onSelectProject,
  onCreateProject,
  onRefresh,
  onOpenSettings,
  onOpenPluginManager,
  onOpenImportDialog,
  onOpenPromptTemplates,
  onOpenMultimediaSettings,
  onOpenOutline,
  onOpenBatchGenerator,
  onOpenReverseAnalysis,
  onDeleteProject,
  onRenameProject,
  onExportProject,
}) => {
  const [activeMenuId, setActiveMenuId] = useState<string | null>(null);
  const [deleteConfirm, setDeleteConfirm] = useState<{
    isOpen: boolean;
    projectId: string | null;
    projectName: string;
  }>({
    isOpen: false,
    projectId: null,
    projectName: "",
  });

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (activeMenuId && !(e.target as HTMLElement).closest("[data-menu]")) {
        setActiveMenuId(null);
      }
    };
    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  }, [activeMenuId]);

  const handleMenuClick = (e: React.MouseEvent, projectId: string) => {
    e.stopPropagation();
    setActiveMenuId(activeMenuId === projectId ? null : projectId);
  };

  const handleRename = async (e: React.MouseEvent) => {
    e.stopPropagation();
    const newName = prompt("请输入新的项目名称:");
    if (newName && activeMenuId) {
      try {
        await projectService.updateProject(activeMenuId, newName);
        if (onRefresh) onRefresh();
        setActiveMenuId(null);
      } catch (error) {
        console.error("Failed to rename project:", error);
        alert("重命名失败");
      }
    }
  };

  const handleDeleteClick = (e: React.MouseEvent, projectId: string, projectName: string) => {
    e.stopPropagation();
    e.preventDefault();
    setDeleteConfirm({
      isOpen: true,
      projectId,
      projectName,
    });
    setActiveMenuId(null);
  };

  const handleDeleteConfirm = async () => {
    const projectId = deleteConfirm.projectId;
    if (!projectId) return;

    try {
      if (onDeleteProject) {
        await onDeleteProject(projectId);
      } else {
        await projectService.deleteProject(projectId);
        if (onRefresh) onRefresh();
      }
    } catch (error) {
      console.error("[ProjectList] Failed to delete project:", error);
      alert("删除失败: " + (error as Error).message);
    }
    setDeleteConfirm({ isOpen: false, projectId: null, projectName: "" });
  };

  const handleDeleteCancel = () => {
    setDeleteConfirm({ isOpen: false, projectId: null, projectName: "" });
  };

  const genreMap: Record<string, string> = {
    fantasy: "奇幻",
    scifi: "科幻",
    romance: "言情",
    mystery: "悬疑",
    horror: "恐怖",
    adventure: "冒险",
  };

  return (
    <div className="w-full h-full flex flex-col">
      <div className="flex items-center justify-between px-2 py-3 border-b border-border gap-2">
        <div className="flex items-center gap-2 min-w-0 shrink-0">
          <Folder className="w-5 h-5 text-primary shrink-0" />
          <h2 className="text-lg font-semibold truncate">项目列表</h2>
        </div>
        <div className="flex items-center gap-1 overflow-x-auto shrink-0">
          <button
            onClick={onRefresh}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="刷新项目列表"
          >
            <RotateCcw className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenPluginManager) {
                onOpenPluginManager();
                uiLogger.click("ProjectList", "open_plugin_manager");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="插件管理"
          >
            <Puzzle className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenImportDialog) {
                onOpenImportDialog();
                uiLogger.click("ProjectList", "open_import_dialog");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="导入文件"
          >
            <Upload className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenPromptTemplates) {
                onOpenPromptTemplates();
                uiLogger.click("ProjectList", "open_prompt_templates");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="提示词管理"
          >
            <FileText className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenMultimediaSettings) {
                onOpenMultimediaSettings();
                uiLogger.click("ProjectList", "open_multimedia_settings");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="多媒体设置"
          >
            <Image className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenOutline) {
                onOpenOutline();
                uiLogger.click("ProjectList", "open_outline");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="大纲管理"
          >
            <List className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenBatchGenerator) {
                onOpenBatchGenerator();
                uiLogger.click("ProjectList", "open_batch_generator");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="批量生成"
          >
            <Layers className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenReverseAnalysis) {
                onOpenReverseAnalysis();
                uiLogger.click("ProjectList", "open_reverse_analysis");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="逆向分析"
          >
            <SearchCode className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              if (onOpenSettings) {
                onOpenSettings();
                uiLogger.click("ProjectList", "open_settings");
              }
            }}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="设置"
          >
            <Settings className="w-4 h-4" />
          </button>
          <button
            onClick={() => {
              onCreateProject();
              uiLogger.click("ProjectList", "create_project");
            }}
            className="flex items-center gap-1 px-2 py-1.5 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors shrink-0 whitespace-nowrap"
          >
            <Plus className="w-4 h-4 shrink-0" />
            <span className="text-sm">新建项目</span>
          </button>
        </div>
      </div>
      <div className="flex-1 overflow-auto p-2">
        {projects.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center">
            <Folder className="w-16 h-16 mb-4 text-muted-foreground opacity-50" />
            <h3 className="text-lg font-semibold mb-2">暂无项目</h3>
            <p className="text-sm text-muted-foreground mb-4">点击"新建项目"开始创作</p>
          </div>
        ) : (
          <div className="p-2 space-y-1">
            {projects.map((project) => (
              <div key={project.id} className="relative">
                <button
                  onClick={() => onSelectProject(project)}
                  className={`w-full text-left px-3 py-2 rounded-md transition-colors ${
                    currentProject?.id === project.id
                      ? "bg-primary text-primary-foreground"
                      : "hover:bg-accent hover:text-accent-foreground"
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1 min-w-0">
                      <p className="font-medium truncate">{project.name}</p>
                      {project.description && (
                        <p className="text-xs mt-1 opacity-80 truncate">{project.description}</p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2 mt-2">
                    {project.genre && (
                      <span className="text-xs px-2 py-0.5 rounded-full bg-background/20">
                        {genreMap[project.genre] || project.genre}
                      </span>
                    )}
                    <span className="text-xs opacity-60">
                      {new Date(project.updated_at).toLocaleDateString()}
                    </span>
                  </div>
                </button>
                <button
                  data-testid="more-button"
                  onClick={(e) => handleMenuClick(e, project.id)}
                  className="absolute top-2 right-2 ml-2 opacity-60 hover:opacity-100 p-1 hover:bg-accent rounded transition-colors"
                >
                  <MoreHorizontal className="w-4 h-4" />
                </button>
                {activeMenuId === project.id && (
                  <div
                    data-menu
                    className="absolute right-2 top-10 z-10 bg-popover border border-border rounded-md shadow-lg py-1 min-w-[100px]"
                  >
                    <button
                      onClick={(e) => handleRename(e)}
                      className="w-full px-3 py-2 text-left text-sm hover:bg-accent flex items-center gap-2"
                    >
                      <Edit2 className="w-4 h-4" />
                      重命名
                    </button>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        if (onExportProject) {
                          onExportProject(project.id);
                          uiLogger.click("ProjectList", "export_project");
                        }
                        setActiveMenuId(null);
                      }}
                      className="w-full px-3 py-2 text-left text-sm hover:bg-accent flex items-center gap-2"
                    >
                      <Download className="w-4 h-4" />
                      导出
                    </button>
                    <button
                      onClick={(e) => handleDeleteClick(e, project.id, project.name)}
                      className="w-full px-3 py-2 text-left text-sm hover:bg-accent text-destructive flex items-center gap-2"
                    >
                      <Trash2 className="w-4 h-4" />
                      删除
                    </button>
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
      </div>

      <ConfirmDialog
        isOpen={deleteConfirm.isOpen}
        title="删除项目"
        message={`确定要删除项目"${deleteConfirm.projectName}"吗？此操作不可恢复。`}
        confirmText="删除"
        cancelText="取消"
        variant="danger"
        onConfirm={handleDeleteConfirm}
        onCancel={handleDeleteCancel}
      />
    </div>
  );
};
