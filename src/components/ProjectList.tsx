import React, { useState, useEffect, useRef } from "react";
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
  ChevronDown,
  Wrench,
  Sparkles,
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
  onOpenBlueprint?: () => void;
  onDeleteProject?: (projectId: string) => void;
  onRenameProject?: () => void;
  onExportProject?: (projectId: string) => void;
}

const DropdownMenu: React.FC<{
  trigger: React.ReactNode;
  children: React.ReactNode;
  isOpen: boolean;
  onToggle: () => void;
}> = ({ trigger, children, isOpen, onToggle }) => {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        if (isOpen) onToggle();
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [isOpen, onToggle]);

  return (
    <div className="relative" ref={menuRef}>
      <div onClick={onToggle}>{trigger}</div>
      {isOpen && (
        <div className="absolute right-0 top-full mt-1 z-20 bg-popover border border-border rounded-lg shadow-lg py-1 min-w-[160px]">
          {children}
        </div>
      )}
    </div>
  );
};

const MenuItem: React.FC<{
  icon?: React.ReactNode;
  label: string;
  onClick: () => void;
  variant?: "default" | "danger";
}> = ({ icon, label, onClick, variant = "default" }) => (
  <button
    onClick={onClick}
    className={`w-full px-3 py-2 text-left text-sm hover:bg-accent flex items-center gap-2 ${
      variant === "danger" ? "text-destructive" : ""
    }`}
  >
    {icon && <span className="w-4 h-4">{icon}</span>}
    {label}
  </button>
);

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
  onOpenBlueprint,
  onDeleteProject,
  onRenameProject,
  onExportProject,
}) => {
  const [activeMenuId, setActiveMenuId] = useState<string | null>(null);
  const [toolsMenuOpen, setToolsMenuOpen] = useState(false);
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
      <div className="flex items-center justify-between px-3 py-2 border-b border-border gap-2 bg-gray-50/50">
        <div className="flex items-center gap-2 min-w-0">
          <Folder className="w-5 h-5 text-primary shrink-0" />
          <h2 className="text-base font-semibold truncate">项目列表</h2>
          <span className="text-xs text-muted-foreground bg-muted px-1.5 py-0.5 rounded">
            {projects.length}
          </span>
        </div>
        <div className="flex items-center gap-1">
          <button
            onClick={onRefresh}
            className="p-1.5 hover:bg-accent rounded-md transition-colors shrink-0"
            title="刷新 (R)"
          >
            <RotateCcw className="w-4 h-4" />
          </button>

          <DropdownMenu
            isOpen={toolsMenuOpen}
            onToggle={() => setToolsMenuOpen(!toolsMenuOpen)}
            trigger={
              <button
                className="flex items-center gap-1 px-2 py-1.5 hover:bg-accent rounded-md transition-colors text-sm"
                title="工具菜单"
              >
                <Wrench className="w-4 h-4" />
                <ChevronDown className="w-3 h-3" />
              </button>
            }
          >
            <div className="px-3 py-1.5 text-xs text-muted-foreground border-b border-border">
              创作工具
            </div>
            <MenuItem
              icon={<List className="w-4 h-4" />}
              label="大纲管理"
              onClick={() => {
                onOpenOutline?.();
                setToolsMenuOpen(false);
              }}
            />
            <MenuItem
              icon={<Layers className="w-4 h-4" />}
              label="批量生成"
              onClick={() => {
                onOpenBatchGenerator?.();
                setToolsMenuOpen(false);
              }}
            />
            <MenuItem
              icon={<SearchCode className="w-4 h-4" />}
              label="逆向分析"
              onClick={() => {
                onOpenReverseAnalysis?.();
                setToolsMenuOpen(false);
              }}
            />
            <MenuItem
              icon={<Layers className="w-4 h-4" />}
              label="项目蓝图"
              onClick={() => {
                onOpenBlueprint?.();
                setToolsMenuOpen(false);
              }}
            />
            <div className="px-3 py-1.5 text-xs text-muted-foreground border-t border-b border-border">
              系统设置
            </div>
            <MenuItem
              icon={<FileText className="w-4 h-4" />}
              label="提示词管理"
              onClick={() => {
                onOpenPromptTemplates?.();
                setToolsMenuOpen(false);
              }}
            />
            <MenuItem
              icon={<Image className="w-4 h-4" />}
              label="多媒体设置"
              onClick={() => {
                onOpenMultimediaSettings?.();
                setToolsMenuOpen(false);
              }}
            />
            <MenuItem
              icon={<Puzzle className="w-4 h-4" />}
              label="插件管理"
              onClick={() => {
                onOpenPluginManager?.();
                setToolsMenuOpen(false);
              }}
            />
            <MenuItem
              icon={<Settings className="w-4 h-4" />}
              label="模型设置"
              onClick={() => {
                onOpenSettings?.();
                setToolsMenuOpen(false);
              }}
            />
            <div className="px-3 py-1.5 text-xs text-muted-foreground border-t border-border">
              数据操作
            </div>
            <MenuItem
              icon={<Upload className="w-4 h-4" />}
              label="导入文件"
              onClick={() => {
                onOpenImportDialog?.();
                setToolsMenuOpen(false);
              }}
            />
          </DropdownMenu>

          <button
            onClick={() => {
              onCreateProject();
              uiLogger.click("ProjectList", "create_project");
            }}
            className="flex items-center gap-1 px-3 py-1.5 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors shrink-0 text-sm font-medium"
          >
            <Plus className="w-4 h-4 shrink-0" />
            <span className="hidden sm:inline">新建</span>
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-auto">
        {projects.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center p-6">
            <div className="w-20 h-20 rounded-full bg-muted flex items-center justify-center mb-4">
              <Folder className="w-10 h-10 text-muted-foreground" />
            </div>
            <h3 className="text-lg font-semibold mb-2">暂无项目</h3>
            <p className="text-sm text-muted-foreground mb-4">
              点击"新建"按钮开始你的创作之旅
            </p>
            <button
              onClick={onCreateProject}
              className="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-lg hover:bg-primary/90 transition-colors"
            >
              <Sparkles className="w-4 h-4" />
              创建第一个项目
            </button>
          </div>
        ) : (
          <div className="p-2 space-y-1">
            {projects.map((project) => (
              <div key={project.id} className="relative group">
                <button
                  onClick={() => onSelectProject(project)}
                  className={`w-full text-left px-3 py-2.5 rounded-lg transition-all ${
                    currentProject?.id === project.id
                      ? "bg-primary/10 border-2 border-primary shadow-sm"
                      : "hover:bg-accent border-2 border-transparent"
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1 min-w-0">
                      <p className="font-medium truncate text-sm">{project.name}</p>
                      {project.description && (
                        <p className="text-xs mt-0.5 text-muted-foreground line-clamp-2">
                          {project.description}
                        </p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2 mt-1.5">
                    {project.genre && (
                      <span className="text-xs px-2 py-0.5 rounded-full bg-primary/10 text-primary">
                        {genreMap[project.genre] || project.genre}
                      </span>
                    )}
                    <span className="text-xs text-muted-foreground">
                      {new Date(project.updated_at).toLocaleDateString()}
                    </span>
                  </div>
                </button>
                <button
                  data-testid="more-button"
                  onClick={(e) => handleMenuClick(e, project.id)}
                  className={`absolute top-2 right-2 p-1.5 rounded-md transition-all ${
                    currentProject?.id === project.id || activeMenuId === project.id
                      ? "opacity-100 bg-background shadow-sm"
                      : "opacity-0 group-hover:opacity-100 hover:bg-background"
                  }`}
                >
                  <MoreHorizontal className="w-4 h-4" />
                </button>
                {activeMenuId === project.id && (
                  <div
                    data-menu
                    className="absolute right-2 top-10 z-10 bg-popover border border-border rounded-lg shadow-lg py-1 min-w-[120px]"
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
                    <div className="border-t border-border my-1" />
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
