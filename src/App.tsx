import React, { useEffect, useState, useRef } from 'react';
import { Settings, Layers, Globe, Network, Database } from 'lucide-react';
import { TextEditor } from './components/TextEditor';
import { ProjectList } from './components/ProjectList';
import { ChapterList } from './components/ChapterList';
import { CharacterList } from './components/CharacterList';
import { CreateProjectDialog } from './components/CreateProjectDialog';
import { InputDialog } from './components/InputDialog';
import { CharacterDialog } from './components/CharacterDialog';
import { ModelSettingsDialog } from './components/ModelSettingsDialog';
import { uiLogger } from './utils/uiLogger';
import { PlotPointList } from './components/PlotPointList';
import { PlotPointEditor } from './components/PlotPointEditor';
import { WorldViewList } from './components/WorldViewList';
import { WorldViewEditor } from './components/WorldViewEditor';
import { CharacterRelationGraph } from './components/CharacterRelationGraph';
import { KnowledgeBase } from './components/KnowledgeBase';
import { SmartDebugPanel } from './components/SmartDebugPanel';
import { ResizableLayout } from './components/ResizableLayout';
import { useToast, ToastContainer } from './components/Toast';
import { useProjectStore } from './stores/projectStore';
import { projectService, chapterService, characterService, worldViewService, plotPointService } from './services/api';
import { debugLogger } from './utils/debugLogger';
import type { CreateProjectRequest, SaveChapterRequest, CreateCharacterRequest, UpdateCharacterRequest, Character, PlotPointNode, WorldView } from './types';
import { invoke } from '@tauri-apps/api/core';

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

  useEffect(() => {
    uiLogger.mount('App');
    return () => uiLogger.unmount('App');

    console.log('isModelSettingsDialogOpen changed to:', isModelSettingsDialogOpen);
  }, [isModelSettingsDialogOpen]);
  const [editingCharacter, setEditingCharacter] = useState<Character | undefined>();
  const [initialCharacterName, setInitialCharacterName] = useState<string>('');
  const [initialWorldViewTitle, setInitialWorldViewTitle] = useState<string>('');

  const [editorContent, setEditorContent] = useState('');
  const [rightPanelTab, setRightPanelTab] = useState<'chapters' | 'plot' | 'worldview' | 'relations' | 'knowledge'>('chapters');
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
      console.error('Failed to load projects:', error);
      showToast('加载项目失败', 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const loadChapters = async (projectId: string) => {
    try {
      debugLogger.info('Loading chapters', { projectId, component: 'App', feature: 'chapter-list' });
      const result = await chapterService.getChapters(projectId);
      setChapters(result);
      debugLogger.info('Chapters loaded successfully', { count: result.length, component: 'App', feature: 'chapter-list' });
    } catch (error) {
      console.error('Failed to load chapters:', error);
      debugLogger.error('Failed to load chapters', error as Error, { projectId, component: 'App', feature: 'chapter-list' });
      showToast('加载章节失败', 'error');
    }
  };

  const loadCharacters = async (projectId: string) => {
    try {
      debugLogger.info('Loading characters', { projectId, component: 'App', feature: 'character-list' });
      const result = await characterService.getCharacters(projectId);
      setCharacters(result);
      debugLogger.info('Characters loaded successfully', { count: result.length, component: 'App', feature: 'character-list' });
    } catch (error) {
      console.error('Failed to load characters:', error);
      debugLogger.error('Failed to load characters', error as Error, { projectId, component: 'App', feature: 'character-list' });
      showToast('加载角色失败', 'error');
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
      localStorage.setItem('current-project-id', newProject.id);
      setIsCreateProjectDialogOpen(false);
      showToast('项目创建成功', 'success');
    } catch (error) {
      console.error('Failed to create project:', error);
      showToast('创建项目失败，请重试', 'error');
    }
  };

  const handleDeleteProject = async (projectId: string) => {
    console.log('[App] handleDeleteProject called with:', projectId);
    try {
      console.log('[App] Calling projectService.deleteProject...');
      await projectService.deleteProject(projectId);
      console.log('[App] deleteProject API call succeeded');
      if (currentProject?.id === projectId) {
        setCurrentProject(null);
      }
      await loadProjects();
      showToast('项目删除成功', 'success');
    } catch (error) {
      console.error('[App] Failed to delete project:', error);
      showToast('删除项目失败: ' + (error as Error).message, 'error');
    }
  };

  const handleRenameProject = async (newName: string) => {
    if (!currentProject) return;

    try {
      const updatedProject = await projectService.updateProject(
        currentProject.id,
        newName
      );
      await loadProjects();
      if (currentProject?.id === updatedProject.id) {
        setCurrentProject(updatedProject);
      }
      setIsProjectRenameDialogOpen(false);
      showToast('项目重命名成功', 'success');
    } catch (error) {
      console.error('Failed to rename project:', error);
      showToast('重命名失败', 'error');
    }
  };

  const handleSelectProject = async (project: any) => {
    if (currentProject?.id === project.id) return;

    try {
      setCurrentProject(project);
      localStorage.setItem('current-project-id', project.id);
      await loadChapters(project.id);
      await loadCharacters(project.id);
    } catch (error) {
      console.error('Failed to load project data:', error);
      showToast('加载项目失败', 'error');
    }
  };

  const handleSelectChapter = (chapter: typeof currentChapter) => {
    if (chapter) {
      setCurrentChapter(chapter);
      setEditorContent(chapter.content);
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
        content: '',
        sort_order: chapters.length,
      };
      const newChapter = await chapterService.saveChapter(request);
      addChapter(newChapter);
      setCurrentChapter(newChapter);
      setEditorContent('');
      setIsChapterNameDialogOpen(false);
      showToast('章节创建成功', 'success');
    } catch (error) {
      console.error('Failed to create chapter:', error);
      showToast('创建章节失败', 'error');
    }
  };

  const handleDeleteChapter = async (chapterId: string) => {
    console.log('handleDeleteChapter called with:', chapterId);
    try {
      await chapterService.deleteChapter(chapterId);
      console.log('Chapter deleted from backend:', chapterId);
      if (currentChapter?.id === chapterId) {
        setCurrentChapter(null);
        setEditorContent('');
      }
      await loadChapters(currentProject!.id);
      removeChapter(chapterId);
      showToast('章节删除成功', 'success');
    } catch (error) {
      console.error('Failed to delete chapter:', error);
      showToast('删除章节失败', 'error');
    }
  };

  const handleRenameChapter = async (newTitle: string) => {
    console.log('handleRenameChapter called with:', newTitle, 'currentChapter:', currentChapter);
    if (!currentChapter) {
      console.warn('No current chapter selected');
      return;
    }

    try {
      console.log('Updating chapter:', currentChapter.id, 'to:', newTitle);
      const updatedChapter = await chapterService.updateChapter(
        currentChapter.id,
        newTitle
      );
      console.log('Chapter updated:', updatedChapter);
      await loadChapters(currentProject!.id);
      setCurrentChapter(updatedChapter);
      setIsChapterRenameDialogOpen(false);
      showToast('章节重命名成功', 'success');
    } catch (error) {
      console.error('Failed to rename chapter:', error);
      showToast('重命名失败', 'error');
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
      const request: SaveChapterRequest = {
        project_id: currentProject.id,
        title: currentChapter.title,
        content: editorContent,
        sort_order: currentChapter.sort_order,
      };
      const updatedChapter = await chapterService.saveChapter(request);
      setCurrentChapter(updatedChapter);
      showToast('保存成功', 'success');
    } catch (error) {
      console.error('Failed to save chapter:', error);
      showToast('保存失败，请重试', 'error');
    }
  };

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
      showToast('角色删除成功', 'success');
    } catch (error) {
      console.error('Failed to delete character:', error);
      showToast('删除角色失败', 'error');
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
        showToast('角色更新成功', 'success');
      } else {
        await characterService.createCharacter(request);
        showToast('角色创建成功', 'success');
      }

      await loadCharacters(currentProject!.id);
      setIsCharacterDialogOpen(false);
    } catch (error) {
      console.error('Failed to save character:', error);
      showToast(editingCharacter ? '更新角色失败' : '创建角色失败', 'error');
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
    showToast('情节点保存成功', 'success');
  };

  const handleLinkToChapter = async (plotPoint: PlotPointNode) => {
    try {
      const updatedChapter = chapters.find(c => c.id === plotPoint.chapter_id);
      if (updatedChapter) {
        setCurrentChapter(updatedChapter);
        setEditorContent(updatedChapter.content);
      }
    } catch (error) {
      console.error('Failed to navigate to chapter:', error);
      showToast('跳转到章节失败', 'error');
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
    showToast('世界观保存成功', 'success');
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
      showToast('AI 生成角色成功', 'success');
    } catch (error) {
      console.error('Failed to create AI character:', error);
      showToast('AI 生成角色失败', 'error');
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
      showToast('AI 生成世界观成功', 'success');
    } catch (error) {
      console.error('Failed to create AI worldview:', error);
      showToast('AI 生成世界观失败', 'error');
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
      showToast('AI 生成情节点成功', 'success');
    } catch (error) {
      console.error('Failed to create AI plot point:', error);
      showToast('AI 生成情节点失败', 'error');
      throw error;
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
              console.log('handleCreateProject called from App');
              setIsCreateProjectDialogOpen(true);
            }}
            onDeleteProject={handleDeleteProject}
            onRenameProject={() => setIsProjectRenameDialogOpen(true)}
            onOpenSettings={() => {
              console.log('onOpenSettings called from App');
              setIsModelSettingsDialogOpen(true);
            }}
            onRefresh={() => {
              console.log('handleRefresh called from App');
              window.location.reload();
            }}
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
        <div className="flex border-b border-border">
          <button
            onClick={() => setRightPanelTab('chapters')}
            className={`flex-1 py-2 px-3 text-sm font-medium transition-colors ${
              rightPanelTab === 'chapters'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'
            }`}
          >
            章节
          </button>
          <button
            onClick={() => setRightPanelTab('plot')}
            className={`flex-1 py-2 px-3 text-sm font-medium transition-colors flex items-center justify-center gap-1 ${
              rightPanelTab === 'plot'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'
            }`}
          >
            <Layers className="w-4 h-4" />
            情节点
          </button>
          <button
            onClick={() => setRightPanelTab('worldview')}
            className={`flex-1 py-2 px-3 text-sm font-medium transition-colors flex items-center justify-center gap-1 ${
              rightPanelTab === 'worldview'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'
            }`}
          >
            <Globe className="w-4 h-4" />
            世界观
          </button>
          <button
            onClick={() => setRightPanelTab('relations')}
            className={`flex-1 py-2 px-3 text-sm font-medium transition-colors flex items-center justify-center gap-1 ${
              rightPanelTab === 'relations'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'
            }`}
          >
            <Network className="w-4 h-4" />
            关系
          </button>
          <button
            onClick={() => setRightPanelTab('knowledge')}
            className={`flex-1 py-2 px-3 text-sm font-medium transition-colors flex items-center justify-center gap-1 ${
              rightPanelTab === 'knowledge'
                ? 'border-b-2 border-blue-500 text-blue-500'
                : 'text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-200'
            }`}
          >
            <Database className="w-4 h-4" />
            知识库
          </button>
        </div>

        {rightPanelTab === 'chapters' ? (
          <div className="flex-1 flex flex-col overflow-hidden">
            <div className="flex-1 overflow-hidden">
              <ChapterList
                chapters={chapters}
                currentChapter={currentChapter}
                onSelectChapter={handleSelectChapter}
                onCreateChapter={handleCreateChapter}
                onDeleteChapter={handleDeleteChapter}
                onRenameChapter={() => setIsChapterRenameDialogOpen(true)}
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
        ) : rightPanelTab === 'plot' ? (
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
        ) : rightPanelTab === 'worldview' ? (
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
        ) : rightPanelTab === 'knowledge' ? (
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
        ) : (
          <div className="flex-1 flex flex-col overflow-hidden">
            {currentProject ? (
              <CharacterRelationGraph
                projectId={currentProject.id}
                characters={characters}
              />
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
          setInitialCharacterName('');
        }}
      />

      <ModelSettingsDialog
        open={isModelSettingsDialogOpen}
        onClose={() => setIsModelSettingsDialogOpen(false)}
      />

      {isPlotPointEditorOpen && (
        <PlotPointEditor
          plotPoint={editingPlotPoint}
          availableChapters={chapters.map(c => ({ id: c.id, title: c.title }))}
          availableParentPoints={[]}
          onClose={handlePlotPointEditorClose}
          onSave={handlePlotPointSaved}
        />
      )}

      {isWorldViewEditorOpen && (
        <WorldViewEditor
          worldView={editingWorldView}
          projectId={currentProject?.id || ''}
          initialTitle={initialWorldViewTitle}
          onClose={() => {
            handleWorldViewEditorClose();
            setInitialWorldViewTitle('');
          }}
          onSave={() => {
            handleWorldViewSaved();
            setInitialWorldViewTitle('');
          }}
        />
      )}
    </ResizableLayout>
  );
}

export default App;
