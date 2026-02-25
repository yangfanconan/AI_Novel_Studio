import React, { useState, useCallback, useEffect } from "react";
import {
  BookOpen,
  FileText,
  Layers,
  Sparkles,
  Settings,
  History,
  Save,
  RotateCcw,
  ChevronLeft,
  ChevronRight,
  Maximize2,
  Minimize2,
  X,
  Play,
  Pause,
  Check,
  Clock,
} from "lucide-react";
import type { Chapter, Character } from "../types";

type WritingMode = "draft" | "outline" | "notes" | "compare";
type ViewMode = "editor" | "preview" | "split";

interface WritingWorkspaceProps {
  chapter: Chapter;
  characters: Character[];
  content: string;
  onChange: (content: string) => void;
  onSave: () => Promise<void>;
  wordCount: number;
  isSaving: boolean;
  mode: WritingMode;
  onModeChange: (mode: WritingMode) => void;
  viewMode: ViewMode;
  onViewModeChange: (mode: ViewMode) => void;
}

const WritingWorkspace: React.FC<WritingWorkspaceProps> = ({
  chapter,
  characters,
  content,
  onChange,
  onSave,
  wordCount,
  isSaving,
  mode,
  onModeChange,
  viewMode,
  onViewModeChange,
}) => {
  const [lastSaved, setLastSaved] = useState<Date | null>(null);
  const [autoSaveEnabled, setAutoSaveEnabled] = useState(true);

  const handleSave = useCallback(async () => {
    await onSave();
    setLastSaved(new Date());
  }, [onSave]);

  useEffect(() => {
    if (!autoSaveEnabled) return;
    const timer = setTimeout(() => {
      handleSave();
    }, 30000);
    return () => clearTimeout(timer);
  }, [content, autoSaveEnabled, handleSave]);

  const modeConfig = {
    draft: { icon: <FileText className="w-4 h-4" />, label: "草稿" },
    outline: { icon: <Layers className="w-4 h-4" />, label: "大纲" },
    notes: { icon: <BookOpen className="w-4 h-4" />, label: "笔记" },
    compare: { icon: <History className="w-4 h-4" />, label: "对比" },
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-4 py-3 border-b border-border bg-gray-50">
        <div className="flex items-center gap-2">
          <h2 className="text-sm font-medium">{chapter.title}</h2>
          <span className="text-xs text-muted-foreground">
            {wordCount} 字
          </span>
          {lastSaved && (
            <span className="text-xs text-muted-foreground flex items-center gap-1">
              <Clock className="w-3 h-3" />
              已保存 {new Date(lastSaved).toLocaleTimeString()}
            </span>
          )}
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => setAutoSaveEnabled(!autoSaveEnabled)}
            className={`px-2 py-1 rounded text-xs flex items-center gap-1 ${
              autoSaveEnabled
                ? "bg-green-100 text-green-700"
                : "bg-gray-100 text-gray-600"
            }`}
            title={autoSaveEnabled ? "自动保存已启用" : "自动保存已禁用"}
          >
            {autoSaveEnabled ? (
              <Check className="w-3 h-3" />
            ) : (
              <Pause className="w-3 h-3" />
            )}
            自动保存
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="px-3 py-1 bg-primary text-primary-foreground text-xs rounded hover:bg-primary/90 flex items-center gap-1 disabled:opacity-50"
          >
            {isSaving ? (
              <>
                <Play className="w-3 h-3 animate-spin" />
                保存中
              </>
            ) : (
              <>
                <Save className="w-3 h-3" />
                保存
              </>
            )}
          </button>
        </div>
      </div>

      <div className="flex border-b border-border bg-white">
        {(Object.keys(modeConfig) as WritingMode[]).map((m) => (
          <button
            key={m}
            onClick={() => onModeChange(m)}
            className={`flex items-center gap-1.5 px-4 py-2 text-sm transition-colors ${
              mode === m
                ? "border-b-2 border-blue-500 text-blue-600 bg-blue-50"
                : "text-gray-600 hover:text-gray-900 hover:bg-gray-50"
            }`}
          >
            {modeConfig[m].icon}
            {modeConfig[m].label}
          </button>
        ))}
        <div className="flex-1" />
        <div className="flex items-center gap-1 px-2 border-l border-border">
          <button
            onClick={() => onViewModeChange("editor")}
            className={`p-2 rounded ${
              viewMode === "editor" ? "bg-blue-100 text-blue-600" : "hover:bg-gray-100"
            }`}
            title="编辑器视图"
          >
            <FileText className="w-4 h-4" />
          </button>
          <button
            onClick={() => onViewModeChange("preview")}
            className={`p-2 rounded ${
              viewMode === "preview" ? "bg-blue-100 text-blue-600" : "hover:bg-gray-100"
            }`}
            title="预览视图"
          >
            <BookOpen className="w-4 h-4" />
          </button>
          <button
            onClick={() => onViewModeChange("split")}
            className={`p-2 rounded ${
              viewMode === "split" ? "bg-blue-100 text-blue-600" : "hover:bg-gray-100"
            }`}
            title="分屏视图"
          >
            <Layers className="w-4 h-4" />
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-hidden flex">
        {viewMode === "editor" && (
          <div className="flex-1 h-full">
            <textarea
              value={content}
              onChange={(e) => onChange(e.target.value)}
              className="w-full h-full p-6 resize-none focus:outline-none text-base leading-relaxed"
              placeholder="开始写作..."
            />
          </div>
        )}
        {viewMode === "preview" && (
          <div className="flex-1 h-full overflow-auto p-6">
            <div className="prose prose-sm max-w-none">
              {content.split("\n").map((paragraph, idx) => (
                <p key={idx} className="mb-4">
                  {paragraph}
                </p>
              ))}
            </div>
          </div>
        )}
        {viewMode === "split" && (
          <div className="flex-1 flex h-full">
            <div className="flex-1 h-full border-r border-border">
              <textarea
                value={content}
                onChange={(e) => onChange(e.target.value)}
                className="w-full h-full p-6 resize-none focus:outline-none text-base leading-relaxed"
                placeholder="开始写作..."
              />
            </div>
            <div className="flex-1 h-full overflow-auto p-6 bg-gray-50">
              <div className="prose prose-sm max-w-none">
                {content.split("\n").map((paragraph, idx) => (
                  <p key={idx} className="mb-4">
                    {paragraph}
                  </p>
                ))}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

interface WritingSidebarProps {
  characters: Character[];
  selectedCharacters: string[];
  onToggleCharacter: (id: string) => void;
  isCollapsed: boolean;
  onToggle: () => void;
}

const WritingSidebar: React.FC<WritingSidebarProps> = ({
  characters,
  selectedCharacters,
  onToggleCharacter,
  isCollapsed,
  onToggle,
}) => {
  if (isCollapsed) {
    return (
      <button
        onClick={onToggle}
        className="w-8 border-l border-border hover:bg-gray-100 flex items-center justify-center text-muted-foreground"
        title="展开工具栏"
      >
        <ChevronRight className="w-4 h-4" />
      </button>
    );
  }

  return (
    <div className="w-72 border-l border-border bg-white flex flex-col">
      <div className="flex items-center justify-between px-4 py-3 border-b border-border">
        <h3 className="text-sm font-medium">工具栏</h3>
        <button
          onClick={onToggle}
          className="p-1 hover:bg-gray-100 rounded text-muted-foreground"
          title="收起工具栏"
        >
          <ChevronLeft className="w-4 h-4" />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-6">
        <div>
          <h4 className="text-xs font-semibold text-muted-foreground mb-2 flex items-center gap-1">
            <Sparkles className="w-3 h-3" />
            参与角色
          </h4>
          <div className="space-y-2">
            {characters.map((character) => (
              <button
                key={character.id}
                onClick={() => onToggleCharacter(character.id)}
                className={`w-full flex items-center gap-2 px-3 py-2 rounded-lg text-left transition-colors ${
                  selectedCharacters.includes(character.id)
                    ? "bg-blue-100 text-blue-700"
                    : "hover:bg-gray-100"
                }`}
              >
                <div className="w-8 h-8 rounded-full bg-gradient-to-br from-purple-400 to-pink-400 flex items-center justify-center text-white text-sm font-medium">
                  {character.name.charAt(0)}
                </div>
                <div className="flex-1 min-w-0">
                  <p className="text-sm font-medium truncate">{character.name}</p>
                  {character.age && (
                    <p className="text-xs opacity-70">{character.age}岁</p>
                  )}
                </div>
                {selectedCharacters.includes(character.id) && (
                  <Check className="w-4 h-4" />
                )}
              </button>
            ))}
          </div>
        </div>

        <div>
          <h4 className="text-xs font-semibold text-muted-foreground mb-2 flex items-center gap-1">
            <Layers className="w-3 h-3" />
            章节大纲
          </h4>
          <div className="text-sm text-muted-foreground bg-gray-50 rounded-lg p-3">
            当前章节大纲占位符
          </div>
        </div>

        <div>
          <h4 className="text-xs font-semibold text-muted-foreground mb-2 flex items-center gap-1">
            <BookOpen className="w-3 h-3" />
            写作笔记
          </h4>
          <textarea
            placeholder="添加写作笔记..."
            className="w-full h-32 px-3 py-2 border border-border rounded-lg text-sm focus:outline-none focus:ring-2 focus:ring-blue-500/50 resize-none"
          />
        </div>
      </div>
    </div>
  );
};

interface WritingDeskProps {
  chapter: Chapter | null;
  chapters: Chapter[];
  characters: Character[];
  content: string;
  onChange: (content: string) => void;
  onSave: () => Promise<void>;
  onSelectChapter: (chapter: Chapter) => void;
  wordCount: number;
  isSaving: boolean;
}

export const WritingDesk: React.FC<WritingDeskProps> = ({
  chapter,
  chapters,
  characters,
  content,
  onChange,
  onSave,
  onSelectChapter,
  wordCount,
  isSaving,
}) => {
  const [mode, setMode] = useState<WritingMode>("draft");
  const [viewMode, setViewMode] = useState<ViewMode>("editor");
  const [selectedCharacters, setSelectedCharacters] = useState<string[]>([]);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  const handleToggleCharacter = useCallback((id: string) => {
    setSelectedCharacters((prev) =>
      prev.includes(id)
        ? prev.filter((c) => c !== id)
        : [...prev, id]
    );
  }, []);

  if (!chapter) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        <div className="text-center">
          <FileText className="w-16 h-16 mx-auto mb-4 opacity-30" />
          <p className="text-lg font-medium">欢迎使用写作台</p>
          <p className="text-sm mt-2">选择一个章节开始创作</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full">
      <div className="flex-1 flex flex-col min-w-0">
        <WritingWorkspace
          chapter={chapter}
          characters={characters}
          content={content}
          onChange={onChange}
          onSave={onSave}
          wordCount={wordCount}
          isSaving={isSaving}
          mode={mode}
          onModeChange={setMode}
          viewMode={viewMode}
          onViewModeChange={setViewMode}
        />
      </div>
      <WritingSidebar
        characters={characters}
        selectedCharacters={selectedCharacters}
        onToggleCharacter={handleToggleCharacter}
        isCollapsed={sidebarCollapsed}
        onToggle={() => setSidebarCollapsed(!sidebarCollapsed)}
      />
    </div>
  );
};

export default WritingDesk;
