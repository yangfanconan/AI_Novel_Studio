import React, { useState } from 'react';
import { BookOpen, Plus, MoreHorizontal, Trash2, Edit2 } from 'lucide-react';
import { ConfirmDialog } from './ConfirmDialog';
import type { Chapter } from '../types';

interface ChapterListProps {
  chapters: Chapter[];
  currentChapter: Chapter | null;
  onSelectChapter: (chapter: Chapter) => void;
  onCreateChapter: () => void;
  onDeleteChapter: (chapterId: string) => void;
  onRenameChapter: () => void;
}

export const ChapterList: React.FC<ChapterListProps> = ({
  chapters,
  currentChapter,
  onSelectChapter,
  onCreateChapter,
  onDeleteChapter,
  onRenameChapter,
}) => {
  const [activeMenuId, setActiveMenuId] = useState<string | null>(null);
  const [deleteConfirm, setDeleteConfirm] = useState<{
    isOpen: boolean;
    chapterId: string | null;
    chapterTitle: string;
  }>({
    isOpen: false,
    chapterId: null,
    chapterTitle: '',
  });

  const handleMenuClick = (e: React.MouseEvent, chapterId: string) => {
    e.stopPropagation();
    setActiveMenuId(activeMenuId === chapterId ? null : chapterId);
  };

  const handleDeleteClick = (e: React.MouseEvent, chapterId: string, chapterTitle: string) => {
    e.stopPropagation();
    e.preventDefault();
    setDeleteConfirm({
      isOpen: true,
      chapterId,
      chapterTitle,
    });
    setActiveMenuId(null);
  };

  const handleDeleteConfirm = () => {
    if (deleteConfirm.chapterId) {
      onDeleteChapter(deleteConfirm.chapterId);
    }
    setDeleteConfirm({ isOpen: false, chapterId: null, chapterTitle: '' });
  };

  const handleDeleteCancel = () => {
    setDeleteConfirm({ isOpen: false, chapterId: null, chapterTitle: '' });
  };

  const handleRename = (e: React.MouseEvent) => {
    e.stopPropagation();
    e.preventDefault();
    onRenameChapter();
    setActiveMenuId(null);
  };

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center justify-between px-4 py-3 border-b border-border">
        <h2 className="font-semibold text-foreground">章节列表</h2>
        <button
          onClick={onCreateChapter}
          className="flex items-center gap-1 px-2 py-1 text-sm text-primary-foreground bg-primary rounded-md hover:bg-primary/90 transition-colors"
        >
          <Plus className="w-4 h-4" />
          新建
        </button>
      </div>

      <div className="flex-1 overflow-y-auto">
        {chapters.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-muted-foreground">
            <BookOpen className="w-12 h-12 mb-2 opacity-50" />
            <p className="text-sm">暂无章节</p>
            <p className="text-xs mt-1">点击"新建"开始创作</p>
          </div>
        ) : (
          <div className="p-2 space-y-1">
            {chapters.map((chapter, index) => (
              <div key={chapter.id} className="relative">
                <button
                  onClick={() => onSelectChapter(chapter)}
                  className={`w-full text-left px-3 py-2 rounded-md transition-colors ${
                    currentChapter?.id === chapter.id
                      ? 'bg-primary text-primary-foreground'
                      : 'hover:bg-accent hover:text-accent-foreground'
                  }`}
                >
                  <div className="flex items-start justify-between">
                    <div className="flex-1 min-w-0">
                      <p className="font-medium truncate">
                        第{index + 1}章 {chapter.title}
                      </p>
                      <p className="text-xs mt-1 opacity-60">
                        {chapter.word_count} 字
                      </p>
                    </div>
                    <button
                      onClick={(e) => handleMenuClick(e, chapter.id)}
                      className="ml-2 opacity-60 hover:opacity-100"
                    >
                      <MoreHorizontal className="w-4 h-4" />
                    </button>
                  </div>
                  <span className="text-xs px-2 py-0.5 rounded-full bg-background/20">
                    {chapter.status}
                  </span>
                </button>

                {activeMenuId === chapter.id && (
                  <div className="absolute right-2 top-10 z-10 bg-popover border border-border rounded-md shadow-lg py-1 min-w-[100px]">
                    <button
                      onClick={(e) => handleRename(e)}
                      className="w-full px-3 py-2 text-left text-sm hover:bg-accent flex items-center gap-2"
                    >
                      <Edit2 className="w-4 h-4" />
                      重命名
                    </button>
                    <button
                      onClick={(e) => handleDeleteClick(e, chapter.id, chapter.title)}
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
        title="删除章节"
        message={`确定要删除章节"${deleteConfirm.chapterTitle}"吗？此操作不可恢复。`}
        confirmText="删除"
        cancelText="取消"
        variant="danger"
        onConfirm={handleDeleteConfirm}
        onCancel={handleDeleteCancel}
      />
    </div>
  );
};
