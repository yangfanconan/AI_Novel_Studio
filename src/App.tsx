import React, { useEffect, useState, useRef, useCallback } from "react";
import {
  Settings,
  Layers,
  Globe,
  Network,
  Database,
  Download,
  Puzzle,
  Film,
  User,
  BookOpen,
  Sparkles,
  TrendingUp,
} from "lucide-react";
import { TextEditor } from "./components/TextEditor";
import { ProjectList } from "./components/ProjectList";
import { ChapterList } from "./components/ChapterList";
import { CharacterList } from "./components/CharacterList";
import { CreateProjectDialog } from "./components/CreateProjectDialog";
import { InputDialog } from "./components/InputDialog";
import { CharacterDialog } from "./components/CharacterDialog";
import { ModelSettingsDialog } from "./components/ModelSettingsDialog";
import { ExportDialog } from "./components/ExportDialog";
import ImportDialog from "./components/ImportDialog";
import PluginManager from "./components/PluginManager";
import PromptTemplateDialog from "./components/PromptTemplateDialog";
import MultimediaSettingsDialog from "./components/MultimediaSettingsDialog";
import OutlinePanel from "./components/OutlinePanel";
import BatchGenerator from "./components/BatchGenerator";
import ReverseAnalysisDialog from "./components/ReverseAnalysisDialog";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { uiLogger } from "./utils/uiLogger";
import { debugLogger } from "./utils/debugLogger";
import { PlotPointList } from "./components/PlotPointList";
import { PlotPointEditor } from "./components/PlotPointEditor";
import { WorldViewList } from "./components/WorldViewList";
import { WorldViewEditor } from "./components/WorldViewEditor";
import { CharacterRelationGraph } from "./components/CharacterRelationGraph";
import { KnowledgeBase } from "./components/KnowledgeBase";
import { SmartDebugPanel } from "./components/SmartDebugPanel";
import { ResizableLayout } from "./components/ResizableLayout";
import { useToast, ToastContainer } from "./components/Toast";
import CharacterBiblePanel from "./components/CharacterBiblePanel";
import BatchProductionPanel from "./components/BatchProductionPanel";
import SceneEditor from "./components/SceneEditor";
import ComfyUIPanel from "./components/ComfyUIPanel";
import WorkflowTemplateEditor from "./components/WorkflowTemplateEditor";
import SeedancePanel from "./components/SeedancePanel";
import StoryboardEditor from "./components/StoryboardEditor";
import ChapterVersionPanel from "./components/ChapterVersionPanel";
import ForeshadowingPanel from "./components/ForeshadowingPanel";
import { EmotionCurvePanel } from "./components/EmotionCurvePanel";
import { ChapterOptimizerPanel } from "./components/ChapterOptimizerPanel";
import { BlueprintEditor } from "./components/BlueprintEditor";
import { ChapterMissionPanel } from "./components/ChapterMissionPanel";
import { useProjectStore } from "./stores/projectStore";
import {
  projectService,
  chapterService,
  characterService,
  worldViewService,
  plotPointService,
} from "./services/api";
import type {
  CreateProjectRequest,
  SaveChapterRequest,
  CreateCharacterRequest,
  UpdateCharacterRequest,
  Character,
  PlotPointNode,
  WorldView,
  Chapter,
} from "./types";
import { invoke } from "@tauri-apps/api/core";
import { useKeyboardShortcuts } from "./hooks/useKeyboardShortcuts";

function App() {
  const {
    projects,
    currentProject,
    chapters,
    currentChapter,
    characters,
    isLoading,
    setProjects,
    setCurrentProject,
    setChapters,
    setCurrentChapter,
    setCharacters,
    setIsLoading,
    addProject,
    addChapter,
    updateChapter,
    removeChapter,
  } = useProjectStore();

  const { toasts, showToast, removeToast } = useToast();
  const autoSaveTimerRef = useRef<NodeJS.Timeout>();
  const [isSaving, setIsSaving] = useState(false);

  const [isCreateProjectDialogOpen, setIsCreateProjectDialogOpen] = useState(false);
  const [isChapterNameDialogOpen, setIsChapterNameDialogOpen] = useState(false);
  const [isProjectRenameDialogOpen, setIsProjectRenameDialogOpen] = useState(false);
  const [isChapterRenameDialogOpen, setIsChapterRenameDialogOpen] = useState(false);
  const [isCharacterDialogOpen, setIsCharacterDialogOpen] = useState(false);
  const [isModelSettingsDialogOpen, setIsModelSettingsDialogOpen] = useState(false);
  const [isExportDialogOpen, setIsExportDialogOpen] = useState(false);
  const [isImportDialogOpen, setIsImportDialogOpen] = useState(false);
  const [isPluginManagerOpen, setIsPluginManagerOpen] = useState(false);
  const [isPromptTemplateOpen, setIsPromptTemplateOpen] = useState(false);
  const [isMultimediaSettingsOpen, setIsMultimediaSettingsOpen] = useState(false);
  const [isOutlineOpen, setIsOutlineOpen] = useState(false);
  const [isBatchGeneratorOpen, setIsBatchGeneratorOpen] = useState(false);
  const [isReverseAnalysisOpen, setIsReverseAnalysisOpen] = useState(false);
  const [isCharacterBibleOpen, setIsCharacterBibleOpen] = useState(false);
  const [isBatchProductionOpen, setIsBatchProductionOpen] = useState(false);
  const [isSceneEditorOpen, setIsSceneEditorOpen] = useState(false);
  const [isComfyUIPanelOpen, setIsComfyUIPanelOpen] = useState(false);
  const [isWorkflowEditorOpen, setIsWorkflowEditorOpen] = useState(false);
  const [isSeedancePanelOpen, setIsSeedancePanelOpen] = useState(false);
  const [isStoryboardEditorOpen, setIsStoryboardEditorOpen] = useState(false);
  const [isChapterVersionPanelOpen, setIsChapterVersionPanelOpen] = useState(false);
  const [isChapterOptimizerOpen, setIsChapterOptimizerOpen] = useState(false);
  const [isBlueprintEditorOpen, setIsBlueprintEditorOpen] = useState(false);
  const [isChapterMissionPanelOpen, setIsChapterMissionPanelOpen] = useState(false);
  const [editingTemplateId, setEditingTemplateId] = useState<string | undefined>();
  const [editingScene, setEditingScene] = useState<any>(null);
  const [exportProjectId, setExportProjectId] = useState<string | null>(null);
  const [exportChapterId, setExportChapterId] = useState<string | null>(null);
  const [blueprint, setBlueprint] = useState<any>(null);
  const [currentChapterMissionId, setCurrentChapterMissionId] = useState<string | null>(null);

  useEffect(() => {
    uiLogger.mount("App");
    return () => uiLogger.unmount("App");
  }, [isModelSettingsDialogOpen]);
  const [editingCharacter, setEditingCharacter] = useState<Character | undefined>();
  const [initialCharacterName, setInitialCharacterName] = useState<string>("");
  const [initialWorldViewTitle, setInitialWorldViewTitle] = useState<string>("");

  const [editorContent, setEditorContent] = useState("");
  const [rightPanelTab, setRightPanelTab] = useState<
    "chapters" | "plot" | "worldview" | "relations" | "knowledge" | "foreshadowing" | "emotion" | "moyin"
  >("chapters");
  const [isPlotPointEditorOpen, setIsPlotPointEditorOpen] = useState(false);
  const [editingPlotPoint, setEditingPlotPoint] = useState<PlotPointNode | null>(null);
  const [isWorldViewEditorOpen, setIsWorldViewEditorOpen] = useState(false);
  const [editingWorldView, setEditingWorldView] = useState<WorldView | null>(null);

  useEffect(() => {
    loadProjects();
  }, []);

  useEffect(() => {
    if (currentProject) {
      loadChapters(currentProject.id);
      loadCharacters(currentProject.id);
    } else {
      setChapters([]);
      setCurrentChapter(null);
      setCharacters([]);
    }
  }, [currentProject?.id]);

  const loadProjects = async () => {
    setIsLoading(true);
    try {
      const result = await projectService.getProjects();
      setProjects(result);
    } catch (error) {
      console.error("Failed to load projects:", error);
      showToast("加载项目失败", "error");
    } finally {
      setIsLoading(false);
    }
  };

  const loadChapters = async (projectId: string) => {
    try {
      debugLogger.info("Loading chapters", {
        projectId,
        component: "App",
        feature: "chapter-list",
      });
      const result = await chapterService.getChapters(projectId);
      setChapters(result);
      debugLogger.info("Chapters loaded successfully", {
        count: result.length,
        component: "App",
        feature: "chapter-list",
      });
    } catch (error) {
      console.error("Failed to load chapters:", error);
      debugLogger.error("Failed to load chapters", error as Error, {
        projectId,
        component: "App",
        feature: "chapter-list",
      });
      showToast("加载章节失败", "error");
    }
  };

  const loadCharacters = async (projectId: string) => {
    try {
      debugLogger.info("Loading characters", {
        projectId,
        component: "App",
        feature: "character-list",
      });
      const result = await characterService.getCharacters(projectId);
      setCharacters(result);
      debugLogger.info("Characters loaded successfully", {
        count: result.length,
        component: "App",
        feature: "character-list",
      });
    } catch (error) {
      console.error("Failed to load characters:", error);
      debugLogger.error("Failed to load characters", error as Error, {
        projectId,
        component: "App",
        feature: "character-list",
      });
      showToast("加载角色失败", "error");
    }
  };

  const handleCreateProject = async (data: {
    name: string;
    description?: string;
    genre?: string;
  }) => {
    try {
      const request: CreateProjectRequest = {
        name: data.name,
        description: data.description,
        genre: data.genre,
      };
      const newProject = await projectService.createProject(request);
      addProject(newProject);
      setCurrentProject(newProject);
      localStorage.setItem("current-project-id", newProject.id);
      setIsCreateProjectDialogOpen(false);
      showToast("项目创建成功", "success");
    } catch (error) {
      console.error("Failed to create project:", error);
      showToast("创建项目失败，请重试", "error");
    }
  };

  const handleDeleteProject = async (projectId: string) => {
    if (!currentProject) return;

    const confirmDelete = window.confirm("确定要删除这个项目吗？此操作不可撤销。");
    if (!confirmDelete) return;

    try {
      await projectService.deleteProject(projectId);
      if (currentProject?.id === projectId) {
        setCurrentProject(null);
      }
      await loadProjects();
      showToast("项目删除成功", "success");
    } catch (error) {
      console.error("[App] Failed to delete project:", error);
      showToast("删除项目失败: " + (error as Error).message, "error");
    }
  };

  const handleRenameProject = async (newName: string) => {
    if (!currentProject) return;

    try {
      const updatedProject = await projectService.updateProject(currentProject.id, newName);
      await loadProjects();
      if (currentProject?.id === updatedProject.id) {
        setCurrentProject(updatedProject);
      }
      setIsProjectRenameDialogOpen(false);
      showToast("项目重命名成功", "success");
    } catch (error) {
      console.error("Failed to rename project:", error);
      showToast("重命名失败", "error");
    }
  };

  const handleSelectProject = async (project: any) => {
    if (currentProject?.id === project.id) return;

    try {
      setCurrentProject(project);
      localStorage.setItem("current-project-id", project.id);
      await loadChapters(project.id);
      await loadCharacters(project.id);
    } catch (error) {
      console.error("Failed to load project data:", error);
      showToast("加载项目失败", "error");
    }
  };

  const handleSelectChapter = async (chapter: typeof currentChapter) => {
    if (chapter) {
      setCurrentChapter(chapter);
      setEditorContent(chapter.content);
      
      try {
        const mission = await invoke<any>("get_chapter_mission", {
          chapterId: chapter.id,
        });
        setCurrentChapterMissionId(mission?.id || null);
      } catch (error) {
        setCurrentChapterMissionId(null);
      }
    }
  };

  const handleCreateChapter = () => {
    setIsChapterNameDialogOpen(true);
  };

  const handleChapterNameSubmit = async (title: string) => {
    if (!currentProject) return;

    try {
      const request: SaveChapterRequest = {
        project_id: currentProject.id,
        title,
        content: "",
        sort_order: chapters.length,
      };
      const newChapter = await chapterService.saveChapter(request);
      addChapter(newChapter);
      setCurrentChapter(newChapter);
      setEditorContent("");
      setIsChapterNameDialogOpen(false);
      showToast("章节创建成功", "success");
    } catch (error) {
      console.error("Failed to create chapter:", error);
      showToast("创建章节失败", "error");
    }
  };

  const handleDeleteChapter = async (chapterId: string) => {
    try {
      await chapterService.deleteChapter(chapterId);
      if (currentChapter?.id === chapterId) {
        setCurrentChapter(null);
        setEditorContent("");
      }
      await loadChapters(currentProject!.id);
      removeChapter(chapterId);
      showToast("章节删除成功", "success");
    } catch (error) {
      console.error("Failed to delete chapter:", error);
      showToast("删除章节失败", "error");
    }
  };

  const handleRenameChapter = async (newTitle: string) => {
    if (!currentChapter) {
      console.warn("No current chapter selected");
      return;
    }

    try {
      const updatedChapter = await chapterService.updateChapter(currentChapter.id, newTitle);
      await loadChapters(currentProject!.id);
      setCurrentChapter(updatedChapter);
      setIsChapterRenameDialogOpen(false);
      showToast("章节重命名成功", "success");
    } catch (error) {
      console.error("Failed to rename chapter:", error);
      showToast("重命名失败", "error");
    }
  };

  const handleEditorChange = (content: string) => {
    setEditorContent(content);
    if (currentChapter) {
      updateChapter(currentChapter.id, content);
    }
  };

  const handleSaveChapter = async () => {
    if (!currentChapter || !currentProject) return;

    try {
      const updatedChapter = await chapterService.updateChapter(
        currentChapter.id,
        currentChapter.title,
        editorContent
      );
      setCurrentChapter(updatedChapter);
      showToast("保存成功", "success");
    } catch (error) {
      console.error("Failed to save chapter:", error);
      showToast("保存失败，请重试", "error");
    }
  };

  const cycleRightPanelTab = useCallback(() => {
    const tabs: Array<typeof rightPanelTab> = ["chapters", "plot", "worldview", "relations", "knowledge", "foreshadowing", "emotion", "moyin"];
    const currentIndex = tabs.indexOf(rightPanelTab);
    const nextIndex = (currentIndex + 1) % tabs.length;
    setRightPanelTab(tabs[nextIndex]);
  }, [rightPanelTab]);

  const closeAllDialogs = useCallback(() => {
    if (isCreateProjectDialogOpen) setIsCreateProjectDialogOpen(false);
    if (isChapterNameDialogOpen) setIsChapterNameDialogOpen(false);
    if (isCharacterDialogOpen) setIsCharacterDialogOpen(false);
    if (isModelSettingsDialogOpen) setIsModelSettingsDialogOpen(false);
    if (isExportDialogOpen) setIsExportDialogOpen(false);
    if (isImportDialogOpen) setIsImportDialogOpen(false);
    if (isPlotPointEditorOpen) setIsPlotPointEditorOpen(false);
    if (isWorldViewEditorOpen) setIsWorldViewEditorOpen(false);
  }, [isCreateProjectDialogOpen, isChapterNameDialogOpen, isCharacterDialogOpen, 
      isModelSettingsDialogOpen, isExportDialogOpen, isImportDialogOpen, 
      isPlotPointEditorOpen, isWorldViewEditorOpen]);

  const shortcuts = [
    { key: "s", ctrl: true, action: handleSaveChapter, description: "保存当前章节" },
    { key: "n", ctrl: true, action: () => setIsCreateProjectDialogOpen(true), description: "新建项目" },
    { key: "e", ctrl: true, action: () => currentProject && setIsExportDialogOpen(true), description: "导出" },
    { key: ",", ctrl: true, action: () => setIsModelSettingsDialogOpen(true), description: "设置" },
    { key: "]", ctrl: true, action: cycleRightPanelTab, description: "切换右侧面板" },
    { key: "Escape", action: closeAllDialogs, description: "关闭对话框" },
  ];

  useKeyboardShortcuts(shortcuts, true);

  const handleCreateCharacter = () => {
    setEditingCharacter(undefined);
    setIsCharacterDialogOpen(true);
  };

  const handleEditCharacter = (character: Character) => {
    setEditingCharacter(character);
    setIsCharacterDialogOpen(true);
  };

  const handleDeleteCharacter = async (characterId: string) => {
    try {
      await characterService.deleteCharacter(characterId);
      await loadCharacters(currentProject!.id);
      showToast("角色删除成功", "success");
    } catch (error) {
      console.error("Failed to delete character:", error);
      showToast("删除角色失败", "error");
    }
  };

  const handleCharacterSubmit = async (data: {
    name: string;
    age?: number;
    gender?: string;
    appearance?: string;
    personality?: string;
    background?: string;
  }) => {
    try {
      const request: CreateCharacterRequest = {
        project_id: currentProject!.id,
        ...data,
      };

      if (editingCharacter) {
        await characterService.updateCharacter(editingCharacter.id, data);
        showToast("角色更新成功", "success");
      } else {
        await characterService.createCharacter(request);
        showToast("角色创建成功", "success");
      }

      await loadCharacters(currentProject!.id);
      setIsCharacterDialogOpen(false);
    } catch (error) {
      console.error("Failed to save character:", error);
      showToast(editingCharacter ? "更新角色失败" : "创建角色失败", "error");
    }
  };

  // 从写作助手快速创建角色
  const handleQuickCreateCharacter = (name: string) => {
    setEditingCharacter(undefined);
    setInitialCharacterName(name);
    setIsCharacterDialogOpen(true);
  };

  // 从写作助手快速创建世界观
  const handleQuickCreateWorldView = (title: string) => {
    setEditingWorldView(null);
    setInitialWorldViewTitle(title);
    setIsWorldViewEditorOpen(true);
  };

  const handleEditPlotPoint = (plotPoint: PlotPointNode) => {
    setEditingPlotPoint(plotPoint);
    setIsPlotPointEditorOpen(true);
  };

  const handlePlotPointEditorClose = () => {
    setEditingPlotPoint(null);
    setIsPlotPointEditorOpen(false);
  };

  const handlePlotPointSaved = async () => {
    handlePlotPointEditorClose();
    showToast("情节点保存成功", "success");
  };

  const handleLinkToChapter = async (plotPoint: PlotPointNode) => {
    try {
      const updatedChapter = chapters.find((c) => c.id === plotPoint.chapter_id);
      if (updatedChapter) {
        setCurrentChapter(updatedChapter);
        setEditorContent(updatedChapter.content);
      }
    } catch (error) {
      console.error("Failed to navigate to chapter:", error);
      showToast("跳转到章节失败", "error");
    }
  };

  const handleEditWorldView = (worldView: WorldView) => {
    setEditingWorldView(worldView);
    setIsWorldViewEditorOpen(true);
  };

  const handleWorldViewEditorClose = () => {
    setEditingWorldView(null);
    setIsWorldViewEditorOpen(false);
  };

  const handleWorldViewSaved = () => {
    handleWorldViewEditorClose();
    showToast("世界观保存成功", "success");
  };

  // AI 生成角色
  const handleAIGenerateCharacter = async (data: any) => {
    try {
      const request: CreateCharacterRequest = {
        project_id: currentProject!.id,
        ...data,
      };
      await characterService.createCharacter(request);
      await loadCharacters(currentProject!.id);
      showToast("AI 生成角色成功", "success");
    } catch (error) {
      console.error("Failed to create AI character:", error);
      showToast("AI 生成角色失败", "error");
      throw error;
    }
  };

  // AI 生成世界观
  const handleAIGenerateWorldView = async (data: any) => {
    try {
      await worldViewService.createWorldView({
        project_id: currentProject!.id,
        ...data,
      });
      showToast("AI 生成世界观成功", "success");
    } catch (error) {
      console.error("Failed to create AI worldview:", error);
      showToast("AI 生成世界观失败", "error");
      throw error;
    }
  };

  // AI 生成情节点
  const handleAIGeneratePlotPoints = async (data: any) => {
    try {
      await plotPointService.createPlotPoint({
        project_id: currentProject!.id,
        ...data,
      });
      showToast("AI 生成情节点成功", "success");
    } catch (error) {
      console.error("Failed to create AI plot point:", error);
      showToast("AI 生成情节点失败", "error");
      throw error;
    }
  };

  const handleExportProject = (projectId: string) => {
    setExportProjectId(projectId);
    setExportChapterId(null);
    setIsExportDialogOpen(true);
  };

  const handleExportChapter = (chapterId: string) => {
    setExportChapterId(chapterId);
    setExportProjectId(null);
    setIsExportDialogOpen(true);
  };

  const handleOpenMission = async (chapter: Chapter) => {
    setIsChapterMissionPanelOpen(true);
    try {
      const bp = await invoke<any>("get_blueprint", {
        request: { project_id: currentProject!.id },
      });
      setBlueprint(bp);
    } catch (error) {
      console.error("Failed to load blueprint:", error);
    }
  };

  const handleCloseExportDialog = () => {
    setIsExportDialogOpen(false);
    setExportProjectId(null);
    setExportChapterId(null);
  };

  const handleImportSuccess = async (result: any) => {
    if (currentProject && result.chapters && result.chapters.length > 0) {
      for (const chapter of result.chapters) {
        await chapterService.saveChapter({
          project_id: currentProject.id,
          title: chapter.title,
          content: chapter.content,
        });
      }
      await loadChapters(currentProject.id);
      showToast(`成功导入 ${result.chapter_count} 个章节`, "success");
    }
  };

  return (
    <ResizableLayout
      leftPanel={
        <>
          <ProjectList
            projects={projects}
            currentProject={currentProject}
            onSelectProject={handleSelectProject}
            onCreateProject={() => {
              setIsCreateProjectDialogOpen(true);
            }}
            onDeleteProject={handleDeleteProject}
            onRenameProject={() => setIsProjectRenameDialogOpen(true)}
            onOpenPluginManager={() => {
              setIsPluginManagerOpen(true);
            }}
            onOpenImportDialog={() => {
              setIsImportDialogOpen(true);
            }}
            onOpenPromptTemplates={() => {
              setIsPromptTemplateOpen(true);
            }}
            onOpenMultimediaSettings={() => {
              setIsMultimediaSettingsOpen(true);
            }}
            onOpenOutline={() => {
              setIsOutlineOpen(true);
            }}
            onOpenBatchGenerator={() => {
              setIsBatchGeneratorOpen(true);
            }}
            onOpenReverseAnalysis={() => {
              setIsReverseAnalysisOpen(true);
            }}
            onOpenBlueprint={() => {
              setIsBlueprintEditorOpen(true);
            }}
            onOpenSettings={() => {
              setIsModelSettingsDialogOpen(true);
            }}
            onRefresh={() => {
              window.location.reload();
            }}
            onExportProject={handleExportProject}
          />
        </>
      }
      centerPanel={
        <>
          <SmartDebugPanel />
          <ToastContainer toasts={toasts} onRemove={removeToast} />
          {currentChapter ? (
            <TextEditor
              content={editorContent}
              onChange={handleEditorChange}
              onSave={handleSaveChapter}
              wordCount={currentChapter.word_count}
              isSaving={isSaving}
              projectId={currentProject?.id}
              chapters={chapters}
              currentChapterId={currentChapter?.id}
              characters={characters}
              onCreateCharacter={handleQuickCreateCharacter}
              onCreateWorldView={handleQuickCreateWorldView}
              chapterMissionId={currentChapterMissionId}
            />
          ) : currentProject ? (
            <div className="flex items-center justify-center h-full text-muted-foreground">
              <div className="text-center">
                <p className="text-lg font-medium">请选择或创建一个章节</p>
                <p className="text-sm mt-2">在右侧列表中点击"新建章节"开始创作</p>
              </div>
            </div>
          ) : (
            <div className="flex items-center justify-center h-full text-muted-foreground">
              <div className="text-center">
                <p className="text-lg font-medium">欢迎使用 AI Novel Studio</p>
                <p className="text-sm mt-2">在左侧列表中创建或选择一个项目开始创作</p>
              </div>
            </div>
          )}
        </>
      }
      rightPanel={
        <>
          <div className="flex border-b border-border bg-muted/30">
            <button
              onClick={() => setRightPanelTab("chapters")}
              title="章节管理"
              className={`flex-1 py-2.5 px-2 text-xs font-medium transition-all duration-200 flex flex-col items-center justify-center gap-0.5 rounded-t-lg mx-0.5 mt-1 ${
                rightPanelTab === "chapters"
                  ? "bg-background text-primary shadow-sm border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground hover:bg-background/50"
              }`}
            >
              <BookOpen className="w-4 h-4" />
              <span className="text-[10px]">章节</span>
            </button>
            <button
              onClick={() => setRightPanelTab("plot")}
              title="情节点管理"
              className={`flex-1 py-2.5 px-2 text-xs font-medium transition-all duration-200 flex flex-col items-center justify-center gap-0.5 rounded-t-lg mx-0.5 mt-1 ${
                rightPanelTab === "plot"
                  ? "bg-background text-primary shadow-sm border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground hover:bg-background/50"
              }`}
            >
              <Layers className="w-4 h-4" />
              <span className="text-[10px]">情节</span>
            </button>
            <button
              onClick={() => setRightPanelTab("worldview")}
              title="世界观设定"
              className={`flex-1 py-2.5 px-2 text-xs font-medium transition-all duration-200 flex flex-col items-center justify-center gap-0.5 rounded-t-lg mx-0.5 mt-1 ${
                rightPanelTab === "worldview"
                  ? "bg-background text-primary shadow-sm border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground hover:bg-background/50"
              }`}
            >
              <Globe className="w-4 h-4" />
              <span className="text-[10px]">世界观</span>
            </button>
            <button
              onClick={() => setRightPanelTab("relations")}
              title="角色关系图"
              className={`flex-1 py-2.5 px-2 text-xs font-medium transition-all duration-200 flex flex-col items-center justify-center gap-0.5 rounded-t-lg mx-0.5 mt-1 ${
                rightPanelTab === "relations"
                  ? "bg-background text-primary shadow-sm border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground hover:bg-background/50"
              }`}
            >
              <Network className="w-4 h-4" />
              <span className="text-[10px]">关系</span>
            </button>
            <button
              onClick={() => setRightPanelTab("knowledge")}
              title="知识库"
              className={`flex-1 py-2.5 px-2 text-xs font-medium transition-all duration-200 flex flex-col items-center justify-center gap-0.5 rounded-t-lg mx-0.5 mt-1 ${
                rightPanelTab === "knowledge"
                  ? "bg-background text-primary shadow-sm border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground hover:bg-background/50"
              }`}
            >
              <Database className="w-4 h-4" />
              <span className="text-[10px]">知识</span>
            </button>
            <button
              onClick={() => setRightPanelTab("foreshadowing")}
              title="伏笔追踪"
              className={`flex-1 py-2.5 px-2 text-xs font-medium transition-all duration-200 flex flex-col items-center justify-center gap-0.5 rounded-t-lg mx-0.5 mt-1 ${
                rightPanelTab === "foreshadowing"
                  ? "bg-background text-primary shadow-sm border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground hover:bg-background/50"
              }`}
            >
              <Sparkles className="w-4 h-4" />
              <span className="text-[10px]">伏笔</span>
            </button>
            <button
              onClick={() => setRightPanelTab("moyin")}
              title="影视制作工具"
              className={`flex-1 py-2.5 px-2 text-xs font-medium transition-all duration-200 flex flex-col items-center justify-center gap-0.5 rounded-t-lg mx-0.5 mt-1 ${
                rightPanelTab === "moyin"
                  ? "bg-background text-primary shadow-sm border-b-2 border-primary"
                  : "text-muted-foreground hover:text-foreground hover:bg-background/50"
              }`}
            >
              <Film className="w-4 h-4" />
              <span className="text-[10px]">影视</span>
            </button>
          </div>

          {rightPanelTab === "chapters" ? (
            <div className="flex-1 flex flex-col overflow-hidden">
              <div className="flex-1 overflow-hidden">
                <ChapterList
                  chapters={chapters}
                  currentChapter={currentChapter}
                  onSelectChapter={handleSelectChapter}
                  onCreateChapter={handleCreateChapter}
                  onDeleteChapter={handleDeleteChapter}
                  onRenameChapter={() => setIsChapterRenameDialogOpen(true)}
                  onExportChapter={handleExportChapter}
                  onOpenVersions={currentChapter ? () => setIsChapterVersionPanelOpen(true) : undefined}
                  onOpenOptimizer={currentChapter ? () => setIsChapterOptimizerOpen(true) : undefined}
                  onOpenMission={currentChapter ? handleOpenMission : undefined}
                />
              </div>
              <div className="h-64 border-t border-border">
                <CharacterList
                  characters={characters}
                  projectId={currentProject?.id}
                  onCreateCharacter={handleCreateCharacter}
                  onEditCharacter={handleEditCharacter}
                  onDeleteCharacter={handleDeleteCharacter}
                  onAIGenerateCharacter={handleAIGenerateCharacter}
                />
              </div>
            </div>
          ) : rightPanelTab === "plot" ? (
            <div className="flex-1 flex flex-col overflow-hidden">
              {currentProject ? (
                <PlotPointList
                  projectId={currentProject.id}
                  onEditPlotPoint={handleEditPlotPoint}
                  onLinkToChapter={handleLinkToChapter}
                  onAIGeneratePlotPoints={handleAIGeneratePlotPoints}
                />
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  <div className="text-center">
                    <p className="text-sm">请先选择一个项目</p>
                  </div>
                </div>
              )}
            </div>
          ) : rightPanelTab === "worldview" ? (
            <div className="flex-1 flex flex-col overflow-hidden">
              {currentProject ? (
                <WorldViewList
                  projectId={currentProject.id}
                  onEditWorldView={handleEditWorldView}
                  onAIGenerateWorldView={handleAIGenerateWorldView}
                />
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  <div className="text-center">
                    <p className="text-lg">请先选择一个项目</p>
                  </div>
                </div>
              )}
            </div>
          ) : rightPanelTab === "knowledge" ? (
            <div className="flex-1 flex flex-col overflow-hidden">
              {currentProject ? (
                <KnowledgeBase projectId={currentProject.id} />
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  <div className="text-center">
                    <p className="text-lg">请先选择一个项目</p>
                  </div>
                </div>
              )}
            </div>
          ) : rightPanelTab === "foreshadowing" ? (
            <div className="flex-1 flex flex-col overflow-hidden">
              {currentProject ? (
                <ForeshadowingPanel
                  projectId={currentProject.id}
                  chapterId={currentChapter?.id}
                  chapterNumber={currentChapter ? chapters.indexOf(currentChapter) + 1 : undefined}
                  chapterTitle={currentChapter?.title}
                />
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  <div className="text-center">
                    <p className="text-lg">请先选择一个项目</p>
                  </div>
                </div>
              )}
            </div>
          ) : rightPanelTab === "moyin" ? (
            <div className="flex-1 flex flex-col overflow-hidden">
              {currentProject ? (
                <div className="flex flex-col h-full">
                  <div className="flex flex-wrap gap-1 p-2 border-b bg-muted/20">
                    <button
                      onClick={() => setIsCharacterBibleOpen(true)}
                      title="角色圣经 - 管理角色深度设定"
                      className="px-2.5 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-1.5 bg-background hover:bg-primary/10 hover:text-primary border border-border hover:border-primary/30"
                    >
                      <User className="w-3.5 h-3.5" />
                      角色圣经
                    </button>
                    <button
                      onClick={() => setIsBatchProductionOpen(true)}
                      title="批量生产 - 批量生成场景内容"
                      className="px-2.5 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-1.5 bg-background hover:bg-primary/10 hover:text-primary border border-border hover:border-primary/30"
                    >
                      <Film className="w-3.5 h-3.5" />
                      批量生产
                    </button>
                    <button
                      onClick={() => setIsComfyUIPanelOpen(true)}
                      title="ComfyUI - AI图像生成工作流"
                      className="px-2.5 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-1.5 bg-background hover:bg-primary/10 hover:text-primary border border-border hover:border-primary/30"
                    >
                      <Network className="w-3.5 h-3.5" />
                      ComfyUI
                    </button>
                    <button
                      onClick={() => {
                        setEditingTemplateId(undefined);
                        setIsWorkflowEditorOpen(true);
                      }}
                      title="模板编辑器 - 创建工作流模板"
                      className="px-2.5 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-1.5 bg-background hover:bg-primary/10 hover:text-primary border border-border hover:border-primary/30"
                    >
                      <Puzzle className="w-3.5 h-3.5" />
                      模板
                    </button>
                    <button
                      onClick={() => setIsSeedancePanelOpen(true)}
                      title="Seedance 2.0 - AI视频生成"
                      className="px-2.5 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-1.5 bg-background hover:bg-primary/10 hover:text-primary border border-border hover:border-primary/30"
                    >
                      <Film className="w-3.5 h-3.5" />
                      Seedance
                    </button>
                    <button
                      onClick={() => setIsStoryboardEditorOpen(true)}
                      title="分镜编辑器 - 可视化分镜管理"
                      className="px-2.5 py-1.5 text-xs font-medium rounded-md transition-all flex items-center gap-1.5 bg-background hover:bg-primary/10 hover:text-primary border border-border hover:border-primary/30"
                    >
                      <Layers className="w-3.5 h-3.5" />
                      分镜
                    </button>
                  </div>
                  <div className="flex-1 overflow-hidden">
                    <CharacterBiblePanel
                      projectId={currentProject.id}
                    />
                  </div>
                </div>
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  <div className="text-center">
                    <p className="text-lg">请先选择一个项目</p>
                  </div>
                </div>
              )}
            </div>
          ) : (
            <div className="flex-1 flex flex-col overflow-hidden">
              {currentProject ? (
                <CharacterRelationGraph projectId={currentProject.id} characters={characters} />
              ) : (
                <div className="flex items-center justify-center h-full text-muted-foreground">
                  <div className="text-center">
                    <p className="text-sm">请先选择一个项目</p>
                  </div>
                </div>
              )}
            </div>
          )}
        </>
      }
    >
      <CreateProjectDialog
        isOpen={isCreateProjectDialogOpen}
        onClose={() => setIsCreateProjectDialogOpen(false)}
        onSubmit={handleCreateProject}
      />

      <InputDialog
        isOpen={isChapterNameDialogOpen}
        title="新建章节"
        message="请输入章节标题："
        defaultValue={`第${chapters.length + 1}章`}
        onSubmit={handleChapterNameSubmit}
        onCancel={() => setIsChapterNameDialogOpen(false)}
      />

      <InputDialog
        isOpen={isProjectRenameDialogOpen}
        title="重命名项目"
        message="请输入新的项目名称："
        defaultValue={currentProject?.name}
        onSubmit={handleRenameProject}
        onCancel={() => setIsProjectRenameDialogOpen(false)}
      />

      <InputDialog
        isOpen={isChapterRenameDialogOpen}
        title="重命名章节"
        message="请输入新的章节标题："
        defaultValue={currentChapter?.title}
        onSubmit={handleRenameChapter}
        onCancel={() => setIsChapterRenameDialogOpen(false)}
      />

      <CharacterDialog
        isOpen={isCharacterDialogOpen}
        character={editingCharacter}
        initialName={initialCharacterName}
        onSubmit={handleCharacterSubmit}
        onCancel={() => {
          setIsCharacterDialogOpen(false);
          setInitialCharacterName("");
        }}
      />

      <ModelSettingsDialog
        open={isModelSettingsDialogOpen}
        onClose={() => setIsModelSettingsDialogOpen(false)}
      />

      <ExportDialog
        isOpen={isExportDialogOpen}
        onClose={handleCloseExportDialog}
        projectId={exportProjectId}
        chapterId={exportChapterId}
        projectName={currentProject?.name}
      />

      <ImportDialog
        isOpen={isImportDialogOpen}
        onClose={() => setIsImportDialogOpen(false)}
        projectId={currentProject?.id}
        onImportSuccess={handleImportSuccess}
      />

      <PromptTemplateDialog
        isOpen={isPromptTemplateOpen}
        onClose={() => setIsPromptTemplateOpen(false)}
      />

      <MultimediaSettingsDialog
        isOpen={isMultimediaSettingsOpen}
        onClose={() => setIsMultimediaSettingsOpen(false)}
      />

      {currentProject && (
        <OutlinePanel
          projectId={currentProject.id}
          isOpen={isOutlineOpen}
          onClose={() => setIsOutlineOpen(false)}
        />
      )}

      {currentProject && (
        <BatchGenerator
          isOpen={isBatchGeneratorOpen}
          onClose={() => setIsBatchGeneratorOpen(false)}
          projectId={currentProject.id}
        />
      )}

      <ReverseAnalysisDialog
        isOpen={isReverseAnalysisOpen}
        onClose={() => setIsReverseAnalysisOpen(false)}
        onImportResults={(result) => {
          showToast("逆向分析结果已导入", "success");
        }}
      />

      {isBatchProductionOpen && currentProject && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-4xl h-[80vh] overflow-hidden">
            <BatchProductionPanel
              projectId={currentProject.id}
              dbPath={`~/Library/Application Support/com.infinitenote/app.db`}
              onSceneSelect={(scene) => {
                setEditingScene(scene);
                setIsSceneEditorOpen(true);
              }}
            />
            <div className="absolute top-2 right-2">
              <button
                onClick={() => setIsBatchProductionOpen(false)}
                className="p-2 bg-white rounded-full shadow hover:bg-gray-100"
              >
                ×
              </button>
            </div>
          </div>
        </div>
      )}

      {isSceneEditorOpen && editingScene && currentProject && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-3xl h-[85vh] overflow-hidden">
            <SceneEditor
              scene={editingScene}
              dbPath={`~/Library/Application Support/com.infinitenote/app.db`}
              projectId={currentProject.id}
              characters={[]}
              onClose={() => {
                setIsSceneEditorOpen(false);
                setEditingScene(null);
              }}
              onSaved={() => {
                showToast("场景保存成功", "success");
              }}
            />
          </div>
        </div>
      )}

      {isComfyUIPanelOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-5xl h-[90vh] overflow-hidden">
            <ComfyUIPanel />
            <button
              onClick={() => setIsComfyUIPanelOpen(false)}
              className="absolute top-4 right-4 px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
            >
              Close
            </button>
          </div>
        </div>
      )}

      {isWorkflowEditorOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-4xl h-[90vh] overflow-hidden">
            <WorkflowTemplateEditor
              templateId={editingTemplateId}
              onSave={(template) => {
                showToast("模板保存成功", "success");
                setIsWorkflowEditorOpen(false);
              }}
              onCancel={() => setIsWorkflowEditorOpen(false)}
            />
          </div>
        </div>
      )}

      {isSeedancePanelOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-5xl h-[90vh] overflow-hidden">
            <button
              onClick={() => setIsSeedancePanelOpen(false)}
              className="absolute top-4 right-4 px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
            >
              Close
            </button>
            <SeedancePanel />
          </div>
        </div>
      )}

      {isStoryboardEditorOpen && currentProject && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-6xl h-[90vh] overflow-hidden relative">
            <button
              onClick={() => setIsStoryboardEditorOpen(false)}
              className="absolute top-4 right-4 px-4 py-2 bg-gray-200 rounded hover:bg-gray-300 z-10"
            >
              关闭
            </button>
            <StoryboardEditor />
          </div>
        </div>
      )}

      {isChapterVersionPanelOpen && currentProject && currentChapter && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-background dark:bg-gray-900 rounded-lg shadow-xl w-full max-w-4xl h-[80vh] overflow-hidden relative">
            <button
              onClick={() => setIsChapterVersionPanelOpen(false)}
              className="absolute top-4 right-4 px-3 py-1.5 bg-muted hover:bg-muted/80 rounded-md z-10 text-sm transition-colors"
            >
              关闭
            </button>
            <ChapterVersionPanel
              chapter={currentChapter}
              projectId={currentProject.id}
              onUpdateChapter={(updatedChapter) => {
                const index = chapters.findIndex(ch => ch.id === updatedChapter.id);
                if (index >= 0) {
                  const newChapters = [...chapters];
                  newChapters[index] = updatedChapter;
                  setChapters(newChapters);
                  if (currentChapter?.id === updatedChapter.id) {
                    setCurrentChapter(updatedChapter);
                  }
                }
              }}
              onClose={() => setIsChapterVersionPanelOpen(false)}
            />
          </div>
        </div>
      )}

      {isPluginManagerOpen && (
        <div className="fixed inset-0 bg-background z-50">
          <ErrorBoundary>
            <PluginManager onClose={() => setIsPluginManagerOpen(false)} />
          </ErrorBoundary>
        </div>
      )}

      {isPlotPointEditorOpen && (
        <PlotPointEditor
          plotPoint={editingPlotPoint}
          availableChapters={chapters.map((c) => ({ id: c.id, title: c.title }))}
          availableParentPoints={[]}
          onClose={handlePlotPointEditorClose}
          onSave={handlePlotPointSaved}
        />
      )}

      {isWorldViewEditorOpen && (
        <WorldViewEditor
          worldView={editingWorldView}
          projectId={currentProject?.id || ""}
          initialTitle={initialWorldViewTitle}
          onClose={() => {
            handleWorldViewEditorClose();
            setInitialWorldViewTitle("");
          }}
          onSave={() => {
            handleWorldViewSaved();
            setInitialWorldViewTitle("");
          }}
        />
      )}

      {isChapterOptimizerOpen && currentProject && currentChapter && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-background dark:bg-gray-900 rounded-lg shadow-xl w-full max-w-3xl h-[90vh] overflow-hidden relative">
            <button
              onClick={() => setIsChapterOptimizerOpen(false)}
              className="absolute top-4 right-4 px-3 py-1.5 bg-muted hover:bg-muted/80 rounded-md z-10 text-sm transition-colors"
            >
              关闭
            </button>
            <ChapterOptimizerPanel
              chapterId={currentChapter.id}
              projectId={currentProject.id}
              chapterTitle={currentChapter.title}
              onOptimizationApplied={(content) => {
                const index = chapters.findIndex(ch => ch.id === currentChapter.id);
                if (index >= 0) {
                  const newChapters = [...chapters];
                  newChapters[index] = { ...currentChapter, content };
                  setChapters(newChapters);
                  setCurrentChapter({ ...currentChapter, content });
                }
              }}
            />
          </div>
        </div>
      )}

      {isBlueprintEditorOpen && currentProject && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-background dark:bg-gray-900 rounded-lg shadow-xl w-full max-w-5xl h-[90vh] overflow-hidden relative">
            <button
              onClick={() => setIsBlueprintEditorOpen(false)}
              className="absolute top-4 right-4 px-3 py-1.5 bg-muted hover:bg-muted/80 rounded-md z-10 text-sm transition-colors"
            >
              关闭
            </button>
            <BlueprintEditor
              projectId={currentProject.id}
              onClose={() => setIsBlueprintEditorOpen(false)}
            />
          </div>
        </div>
      )}

      {isChapterMissionPanelOpen && currentProject && currentChapter && (
        <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
          <div className="bg-background dark:bg-gray-900 rounded-lg shadow-xl w-full max-w-4xl h-[85vh] overflow-hidden relative">
            <button
              onClick={() => setIsChapterMissionPanelOpen(false)}
              className="absolute top-4 right-4 px-3 py-1.5 bg-muted hover:bg-muted/80 rounded-md z-10 text-sm transition-colors"
            >
              关闭
            </button>
            <ChapterMissionPanel
              projectId={currentProject.id}
              chapterId={currentChapter.id}
              chapterNumber={chapters.findIndex(ch => ch.id === currentChapter.id) + 1}
              chapterTitle={currentChapter.title}
              chapterOutline={undefined}
              blueprint={blueprint}
              onMissionUpdated={(mission) => {
                console.log("Mission updated:", mission);
                setCurrentChapterMissionId(mission.id);
              }}
            />
          </div>
        </div>
      )}
    </ResizableLayout>
  );
}

export default App;
