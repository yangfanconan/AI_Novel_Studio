import React, { useState, useEffect } from 'react';
import { Save, FileText, Loader2, Bot } from 'lucide-react';
import { AIToolbar } from './AIToolbar';
import WritingAssistant from './WritingAssistant';
import type { Chapter } from '../types';

interface TextEditorProps {
  content: string;
  onChange: (content: string) => void;
  onSave?: () => void;
  wordCount?: number;
  isSaving?: boolean;
  projectId?: string;
  chapters?: Chapter[];
  currentChapterId?: string;
  onCreateCharacter?: (name: string) => void;
  onCreateWorldView?: (title: string) => void;
}

export const TextEditor: React.FC<TextEditorProps> = ({
  content,
  onChange,
  onSave,
  wordCount,
  isSaving = false,
  projectId,
  chapters = [],
  currentChapterId,
  onCreateCharacter,
  onCreateWorldView,
}) => {
  const [localContent, setLocalContent] = useState(content);
  const [showAssistant, setShowAssistant] = useState(false);

  useEffect(() => {
    setLocalContent(content);
  }, [content]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setLocalContent(e.target.value);
    onChange(e.target.value);
  };

  const handleAIInsert = (text: string) => {
    const newContent = localContent + '\n\n' + text;
    setLocalContent(newContent);
    onChange(newContent);
  };

  const handleAIRewrite = (text: string) => {
    setLocalContent(text);
    onChange(text);
  };

  const handleChoiceSelected = (preview: string) => {
    handleAIInsert(preview);
  };

  const handleCreateCharacter = (name: string) => {
    if (onCreateCharacter) {
      onCreateCharacter(name);
    }
  };

  return (
    <div className="flex flex-col h-full">
      {/* Toolbar */}
      <div className="flex items-center justify-between px-4 py-2 border-b border-border bg-card">
        <div className="flex items-center gap-2">
          <FileText className="w-4 h-4 text-muted-foreground" />
          <span className="text-sm text-muted-foreground">章节编辑</span>
        </div>
        <div className="flex items-center gap-4">
          <button
            onClick={() => setShowAssistant(!showAssistant)}
            className={`flex items-center gap-1 px-3 py-1 text-sm rounded-md transition-colors ${
              showAssistant
                ? 'bg-blue-500 text-white'
                : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
            }`}
          >
            <Bot className="w-4 h-4" />
            写作助手
          </button>
          <span className="text-sm text-muted-foreground">
            {wordCount || localContent.length} 字
          </span>
          {onSave && (
            <button
              onClick={onSave}
              disabled={isSaving}
              className="flex items-center gap-1 px-3 py-1 text-sm text-primary-foreground bg-primary rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {isSaving ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  保存中...
                </>
              ) : (
                <>
                  <Save className="w-4 h-4" />
                  保存
                </>
              )}
            </button>
          )}
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 flex overflow-hidden">
        {/* Editor Area */}
        <div className="flex-1 flex flex-col overflow-hidden">
          {/* AI Toolbar */}
          <AIToolbar
            content={localContent}
            onInsert={handleAIInsert}
            onRewrite={handleAIRewrite}
            disabled={isSaving}
            projectId={projectId}
            chapters={chapters}
            currentChapterId={currentChapterId}
          />

          {/* Editor */}
          <div className="flex-1 overflow-hidden">
            <textarea
              value={localContent}
              onChange={handleChange}
              className="w-full h-full p-6 resize-none focus:outline-none bg-background text-foreground leading-relaxed"
              placeholder="开始创作..."
              spellCheck={false}
            />
          </div>
        </div>

        {/* Writing Assistant Sidebar */}
        {showAssistant && projectId && currentChapterId && (
          <WritingAssistant
            projectId={projectId}
            chapterId={currentChapterId}
            currentContent={localContent}
            onChoiceSelected={handleChoiceSelected}
            onCreateCharacter={handleCreateCharacter}
            onCreateWorldView={onCreateWorldView}
          />
        )}
      </div>
    </div>
  );
};
