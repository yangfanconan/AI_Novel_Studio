import React, { useEffect, useState, useRef } from 'react';
import { TextEditor } from './components/TextEditor';
import { ProjectList } from './components/ProjectList';
import { ChapterList } from './components/ChapterList';
import { CharacterList } from './components/CharacterList';
import { CreateProjectDialog } from './components/CreateProjectDialog';
import { InputDialog } from './components/InputDialog';
import { CharacterDialog } from './components/CharacterDialog';
import { useToast, ToastContainer } from './components/Toast';
import { useProjectStore } from './stores/projectStore';
import { projectService, chapterService, characterService } from './services/api-logged';
import { logger } from './utils/logger';
import type { CreateProjectRequest, SaveChapterRequest, Character } from './types';

function App() {
  logger.info('App component mounting', { feature: 'app' });

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
  const [editingCharacter, setEditingCharacter] = useState<Character | undefined>();

  const [editorContent, setEditorContent] = useState('');

  useEffect(() => {
    logger.info('Loading projects on mount', { feature: 'app' });
    loadProjects();
  }, []);

  useEffect(() => {
    if (currentProject) {
      logger.info('Project selected, loading chapters and characters', { 
        feature: 'app', 
        projectId: currentProject.id 
      });
      loadChapters(currentProject.id);
      loadCharacters(currentProject.id);
    } else {
      setChapters([]);
      setCurrentChapter(null);
      setCharacters([]);
    }
  }, [currentProject?.id]);

  const loadProjects = async () => {
    const track = logger.trackAction('loadProjects');
    logger.info('Starting to load projects', { feature: 'app' });

    setIsLoading(true);
    try {
      const result = await projectService.getProjects();
      setProjects(result);
      logger.info(`Loaded ${result.length} projects`, { feature: 'app' });
    } catch (error) {
      logger.error('Failed to load projects', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'loadProjects',
      });
      showToast('加载项目失败', 'error');
    } finally {
      setIsLoading(false);
      track();
    }
  };

  const loadChapters = async (projectId: string) => {
    logger.info(`Loading chapters for project: ${projectId}`, { feature: 'app', projectId });
    try {
      const result = await chapterService.getChapters(projectId);
      setChapters(result);
      logger.info(`Loaded ${result.length} chapters`, { feature: 'app', projectId });
    } catch (error) {
      logger.error('Failed to load chapters', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'loadChapters',
        projectId,
      });
      showToast('加载章节失败', 'error');
    }
  };

  const loadCharacters = async (projectId: string) => {
    logger.info(`Loading characters for project: ${projectId}`, { feature: 'app', projectId });
    try {
      const result = await characterService.getCharacters(projectId);
      setCharacters(result);
      logger.info(`Loaded ${result.length} characters`, { feature: 'app', projectId });
    } catch (error) {
      logger.error('Failed to load characters', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'loadCharacters',
        projectId,
      });
      showToast('加载角色失败', 'error');
    }
  };

  const handleCreateProject = async (data: {
    name: string;
    description?: string;
    genre?: string;
  }) => {
    logger.info('Creating new project', { feature: 'app', data });
    try {
      const request: CreateProjectRequest = {
        name: data.name,
        description: data.description,
        genre: data.genre,
      };
      const newProject = await projectService.createProject(request);
      addProject(newProject);
      setCurrentProject(newProject);
      setIsCreateProjectDialogOpen(false);
      showToast('项目创建成功', 'success');
    } catch (error) {
      logger.error('Failed to create project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleCreateProject',
        data,
      });
      showToast('创建项目失败，请重试', 'error');
    }
  };

  const handleDeleteProject = async (projectId: string) => {
    logger.info(`Deleting project: ${projectId}`, { feature: 'app', projectId });
    try {
      await projectService.deleteProject(projectId);
      if (currentProject?.id === projectId) {
        setCurrentProject(null);
      }
      await loadProjects();
      showToast('项目删除成功', 'success');
    } catch (error) {
      logger.error('Failed to delete project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleDeleteProject',
        projectId,
      });
      showToast('删除项目失败', 'error');
    }
  };

  const handleRenameProject = async (newName: string) => {
    if (!currentProject) return;

    logger.info(`Renaming project: ${currentProject.id} to ${newName}`, { 
      feature: 'app', 
      projectId: currentProject.id,
      newName 
    });
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
      logger.error('Failed to rename project', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleRenameProject',
        projectId: currentProject?.id,
        newName,
      });
      showToast('重命名失败', 'error');
    }
  };

  const handleSelectChapter = (chapter: typeof currentChapter) => {
    logger.info(`Chapter selected: ${chapter?.id}`, { 
      feature: 'app', 
      chapterId: chapter?.id 
    });
    if (chapter) {
      setCurrentChapter(chapter);
      setEditorContent(chapter.content);
    }
  };

  const handleCreateChapter = () => {
    logger.info('Opening chapter creation dialog', { feature: 'app' });
    setIsChapterNameDialogOpen(true);
  };

  const handleChapterNameSubmit = async (title: string) => {
    if (!currentProject) return;

    logger.info(`Creating chapter: ${title}`, { 
      feature: 'app', 
      projectId: currentProject.id,
      title 
    });
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
      logger.error('Failed to create chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleChapterNameSubmit',
        projectId: currentProject?.id,
        title,
      });
      showToast('创建章节失败', 'error');
    }
  };

  const handleDeleteChapter = async (chapterId: string) => {
    logger.info(`Deleting chapter: ${chapterId}`, { feature: 'app', chapterId });
    try {
      await chapterService.deleteChapter(chapterId);
      if (currentChapter?.id === chapterId) {
        setCurrentChapter(null);
        setEditorContent('');
      }
      await loadChapters(currentProject!.id);
      removeChapter(chapterId);
      showToast('章节删除成功', 'success');
    } catch (error) {
      logger.error('Failed to delete chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleDeleteChapter',
        chapterId,
      });
      showToast('删除章节失败', 'error');
    }
  };

  const handleRenameChapter = async (newTitle: string) => {
    if (!currentChapter) return;

    logger.info(`Renaming chapter: ${currentChapter.id} to ${newTitle}`, { 
      feature: 'app', 
      chapterId: currentChapter.id,
      newTitle 
    });
    try {
      const updatedChapter = await chapterService.updateChapter(
        currentChapter.id,
        newTitle
      );
      await loadChapters(currentProject!.id);
      setCurrentChapter(updatedChapter);
      setIsChapterRenameDialogOpen(false);
      showToast('章节重命名成功', 'success');
    } catch (error) {
      logger.error('Failed to rename chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleRenameChapter',
        chapterId: currentChapter?.id,
        newTitle,
      });
      showToast('重命名失败', 'error');
    }
  };

  const handleEditorChange = (content: string) => {
    setEditorContent(content);
    if (currentChapter) {
      updateChapter(currentChapter.id, content);
      
      if (autoSaveTimerRef.current) {
        clearTimeout(autoSaveTimerRef.current);
      }
      
      autoSaveTimerRef.current = setTimeout(async () => {
        await autoSave();
      }, 3000);
    }
  };

  const autoSave = async () => {
    if (!currentChapter || !currentProject || isSaving) return;

    logger.info(`Auto-saving chapter: ${currentChapter.id}`, { 
      feature: 'app', 
      chapterId: currentChapter.id 
    });
    setIsSaving(true);
    try {
      const request: SaveChapterRequest = {
        project_id: currentProject.id,
        title: currentChapter.title,
        content: editorContent,
        sort_order: currentChapter.sort_order,
      };
      const updatedChapter = await chapterService.saveChapter(request);
      setCurrentChapter(updatedChapter);
      logger.info('Auto-save completed', { 
        feature: 'app', 
        chapterId: currentChapter.id,
        wordCount: editorContent.length 
      });
    } catch (error) {
      logger.error('Failed to auto save chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'autoSave',
        chapterId: currentChapter?.id,
      });
      showToast('自动保存失败', 'error');
    } finally {
      setIsSaving(false);
    }
  };

  const handleSaveChapter = async () => {
    if (!currentChapter || !currentProject) return;

    logger.info(`Manual save chapter: ${currentChapter.id}`, { 
      feature: 'app', 
      chapterId: currentChapter.id 
    });
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
      logger.error('Failed to save chapter', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleSaveChapter',
        chapterId: currentChapter?.id,
      });
      showToast('保存失败，请重试', 'error');
    }
  };

  const handleCreateCharacter = () => {
    logger.info('Opening character creation dialog', { feature: 'app' });
    setEditingCharacter(undefined);
    setIsCharacterDialogOpen(true);
  };

  const handleEditCharacter = (character: Character) => {
    logger.info(`Opening character edit dialog: ${character.id}`, { 
      feature: 'app', 
      characterId: character.id 
    });
    setEditingCharacter(character);
    setIsCharacterDialogOpen(true);
  };

  const handleDeleteCharacter = async (characterId: string) => {
    logger.info(`Deleting character: ${characterId}`, { feature: 'app', characterId });
    try {
      await characterService.deleteCharacter(characterId);
      await loadCharacters(currentProject!.id);
      showToast('角色删除成功', 'success');
    } catch (error) {
      logger.error('Failed to delete character', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleDeleteCharacter',
        characterId,
      });
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
    logger.info(`Saving character: ${data.name}`, { 
      feature: 'app', 
      data 
    });
    try {
      const request = {
        project_id: currentProject!.id,
        ...data,
      };

      if (editingCharacter) {
        await characterService.updateCharacter(editingCharacter.id, data);
        showToast('角色更新成功', 'success');
      } else {
        await characterService.createCharacter(request as any);
        showToast('角色创建成功', 'success');
      }

      await loadCharacters(currentProject!.id);
      setIsCharacterDialogOpen(false);
    } catch (error) {
      logger.error('Failed to save character', error instanceof Error ? error : new Error(String(error)), {
        feature: 'app',
        action: 'handleCharacterSubmit',
        data,
      });
      showToast(editingCharacter ? '更新角色失败' : '创建角色失败', 'error');
    }
  };

  return (
    <div className="flex h-screen bg-background">
      <ToastContainer toasts={toasts} onRemove={removeToast} />

      <div className="w-64 border-r border-border flex flex-col">
        <ProjectList
          projects={projects}
          currentProject={currentProject}
          onSelectProject={setCurrentProject}
          onCreateProject={() => setIsCreateProjectDialogOpen(true)}
          onDeleteProject={handleDeleteProject}
          onRenameProject={() => setIsProjectRenameDialogOpen(true)}
        />
      </div>

      <div className="flex-1 flex flex-col">
        {currentChapter ? (
          <TextEditor
            content={editorContent}
            onChange={handleEditorChange}
            onSave={handleSaveChapter}
            wordCount={currentChapter.word_count}
            isSaving={isSaving}
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
      </div>

      <div className="w-72 border-l border-border flex flex-col">
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
            onCreateCharacter={handleCreateCharacter}
            onEditCharacter={handleEditCharacter}
            onDeleteCharacter={handleDeleteCharacter}
          />
        </div>
      </div>

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
        onSubmit={handleCharacterSubmit}
        onCancel={() => setIsCharacterDialogOpen(false)}
      />
    </div>
  );
}

export default App;
