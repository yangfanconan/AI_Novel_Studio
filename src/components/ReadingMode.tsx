import React, { useState, useEffect, useCallback, useMemo } from 'react';
import {
  X,
  ChevronLeft,
  ChevronRight,
  Settings,
  Minus,
  Plus,
  Sun,
  Moon,
  BookOpen,
  Clock,
  AlignJustify,
} from 'lucide-react';
import type { Chapter } from '../types';

interface ReadingModeProps {
  chapter: Chapter;
  chapters: Chapter[];
  onClose: () => void;
  onNavigate: (chapter: Chapter) => void;
}

type FontSize = 'small' | 'medium' | 'large' | 'xlarge';
type BackgroundTheme = 'light' | 'sepia' | 'dark';
type LineHeight = 'compact' | 'normal' | 'relaxed' | 'loose';

interface ReadingSettings {
  fontSize: FontSize;
  backgroundTheme: BackgroundTheme;
  lineHeight: LineHeight;
}

const fontSizeMap: Record<FontSize, string> = {
  small: 'text-base',
  medium: 'text-lg',
  large: 'text-xl',
  xlarge: 'text-2xl',
};

const fontSizeValueMap: Record<FontSize, number> = {
  small: 16,
  medium: 18,
  large: 20,
  xlarge: 24,
};

const lineHeightMap: Record<LineHeight, string> = {
  compact: 'leading-relaxed',
  normal: 'leading-loose',
  relaxed: 'leading-[2]',
  loose: 'leading-[2.5]',
};

const lineHeightValueMap: Record<LineHeight, number> = {
  compact: 1.6,
  normal: 1.8,
  relaxed: 2,
  loose: 2.5,
};

const backgroundThemeMap: Record<BackgroundTheme, { bg: string; text: string; name: string }> = {
  light: { bg: 'bg-white', text: 'text-gray-900', name: '白色' },
  sepia: { bg: 'bg-amber-50', text: 'text-amber-950', name: '米黄' },
  dark: { bg: 'bg-gray-900', text: 'text-gray-100', name: '深色' },
};

export const ReadingMode: React.FC<ReadingModeProps> = ({
  chapter,
  chapters,
  onClose,
  onNavigate,
}) => {
  const [showSettings, setShowSettings] = useState(false);
  const [settings, setSettings] = useState<ReadingSettings>(() => {
    const saved = localStorage.getItem('reading-settings');
    if (saved) {
      try {
        return JSON.parse(saved);
      } catch {
        return {
          fontSize: 'medium',
          backgroundTheme: 'light',
          lineHeight: 'normal',
        };
      }
    }
    return {
      fontSize: 'medium',
      backgroundTheme: 'light',
      lineHeight: 'normal',
    };
  });

  // 计算当前章节索引
  const currentIndex = useMemo(
    () => chapters.findIndex((c) => c.id === chapter.id),
    [chapters, chapter.id]
  );

  // 计算上一章/下一章
  const prevChapter = currentIndex > 0 ? chapters[currentIndex - 1] : null;
  const nextChapter = currentIndex < chapters.length - 1 ? chapters[currentIndex + 1] : null;

  // 计算阅读时间估算（假设阅读速度为 300 字/分钟）
  const estimatedReadingTime = useMemo(() => {
    const wordsPerMinute = 300;
    const minutes = Math.ceil(chapter.word_count / wordsPerMinute);
    if (minutes < 1) return '不到 1 分钟';
    if (minutes < 60) return `约 ${minutes} 分钟`;
    const hours = Math.floor(minutes / 60);
    const remainingMinutes = minutes % 60;
    return `约 ${hours} 小时 ${remainingMinutes} 分钟`;
  }, [chapter.word_count]);

  // 格式化内容为段落
  const paragraphs = useMemo(() => {
    return chapter.content.split('\n').filter((p) => p.trim());
  }, [chapter.content]);

  // 保存设置到 localStorage
  useEffect(() => {
    localStorage.setItem('reading-settings', JSON.stringify(settings));
  }, [settings]);

  // 键盘快捷键
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      } else if (e.key === 'ArrowLeft' && prevChapter) {
        onNavigate(prevChapter);
      } else if (e.key === 'ArrowRight' && nextChapter) {
        onNavigate(nextChapter);
      }
    },
    [onClose, onNavigate, prevChapter, nextChapter]
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  // 禁止页面滚动
  useEffect(() => {
    document.body.style.overflow = 'hidden';
    return () => {
      document.body.style.overflow = '';
    };
  }, []);

  const themeStyle = backgroundThemeMap[settings.backgroundTheme];
  const isDark = settings.backgroundTheme === 'dark';

  const cycleFontSize = () => {
    const sizes: FontSize[] = ['small', 'medium', 'large', 'xlarge'];
    const currentIndex = sizes.indexOf(settings.fontSize);
    const nextIndex = (currentIndex + 1) % sizes.length;
    setSettings((prev) => ({ ...prev, fontSize: sizes[nextIndex] }));
  };

  const cycleLineHeight = () => {
    const heights: LineHeight[] = ['compact', 'normal', 'relaxed', 'loose'];
    const currentIndex = heights.indexOf(settings.lineHeight);
    const nextIndex = (currentIndex + 1) % heights.length;
    setSettings((prev) => ({ ...prev, lineHeight: heights[nextIndex] }));
  };

  const setBackgroundTheme = (theme: BackgroundTheme) => {
    setSettings((prev) => ({ ...prev, backgroundTheme: theme }));
  };

  return (
    <div
      className={`fixed inset-0 z-50 flex flex-col ${themeStyle.bg} ${themeStyle.text} transition-colors duration-300`}
    >
      {/* 顶部工具栏 */}
      <header
        className={`flex items-center justify-between px-6 py-4 border-b transition-colors duration-300 ${
          isDark ? 'border-gray-700 bg-gray-800/95' : 'border-gray-200 bg-white/95'
        } backdrop-blur-sm`}
      >
        <div className="flex items-center gap-4">
          <h1 className="text-lg font-semibold max-w-md truncate">
            {chapter.title}
          </h1>
          <span
            className={`text-sm ${isDark ? 'text-gray-400' : 'text-gray-500'}`}
          >
            第 {currentIndex + 1} / {chapters.length} 章
          </span>
        </div>

        <div className="flex items-center gap-2">
          {/* 阅读统计 */}
          <div
            className={`hidden sm:flex items-center gap-4 text-sm ${
              isDark ? 'text-gray-400' : 'text-gray-500'
            }`}
          >
            <div className="flex items-center gap-1">
              <BookOpen className="w-4 h-4" />
              <span>{chapter.word_count.toLocaleString()} 字</span>
            </div>
            <div className="flex items-center gap-1">
              <Clock className="w-4 h-4" />
              <span>{estimatedReadingTime}</span>
            </div>
          </div>

          {/* 设置按钮 */}
          <button
            onClick={() => setShowSettings(!showSettings)}
            className={`p-2 rounded-lg transition-colors ${
              showSettings
                ? isDark
                  ? 'bg-gray-700 text-white'
                  : 'bg-gray-200 text-gray-900'
                : isDark
                ? 'hover:bg-gray-700 text-gray-300'
                : 'hover:bg-gray-100 text-gray-600'
            }`}
            title="阅读设置"
          >
            <Settings className="w-5 h-5" />
          </button>

          {/* 关闭按钮 */}
          <button
            onClick={onClose}
            className={`p-2 rounded-lg transition-colors ${
              isDark
                ? 'hover:bg-gray-700 text-gray-300'
                : 'hover:bg-gray-100 text-gray-600'
            }`}
            title="退出阅读模式 (Esc)"
          >
            <X className="w-5 h-5" />
          </button>
        </div>
      </header>

      {/* 设置面板 */}
      {showSettings && (
        <div
          className={`absolute top-[73px] right-6 z-10 p-4 rounded-lg shadow-lg border transition-colors ${
            isDark
              ? 'bg-gray-800 border-gray-700'
              : 'bg-white border-gray-200'
          }`}
        >
          {/* 字体大小 */}
          <div className="mb-4">
            <label
              className={`block text-sm font-medium mb-2 ${
                isDark ? 'text-gray-300' : 'text-gray-700'
              }`}
            >
              字体大小
            </label>
            <div className="flex items-center gap-2">
              <button
                onClick={() =>
                  setSettings((prev) => ({
                    ...prev,
                    fontSize: 'small',
                  }))
                }
                className={`w-8 h-8 rounded flex items-center justify-center transition-colors ${
                  settings.fontSize === 'small'
                    ? isDark
                      ? 'bg-blue-600 text-white'
                      : 'bg-blue-500 text-white'
                    : isDark
                    ? 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                <span className="text-xs">A</span>
              </button>
              <button
                onClick={() =>
                  setSettings((prev) => ({
                    ...prev,
                    fontSize: 'medium',
                  }))
                }
                className={`w-8 h-8 rounded flex items-center justify-center transition-colors ${
                  settings.fontSize === 'medium'
                    ? isDark
                      ? 'bg-blue-600 text-white'
                      : 'bg-blue-500 text-white'
                    : isDark
                    ? 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                <span className="text-sm">A</span>
              </button>
              <button
                onClick={() =>
                  setSettings((prev) => ({
                    ...prev,
                    fontSize: 'large',
                  }))
                }
                className={`w-8 h-8 rounded flex items-center justify-center transition-colors ${
                  settings.fontSize === 'large'
                    ? isDark
                      ? 'bg-blue-600 text-white'
                      : 'bg-blue-500 text-white'
                    : isDark
                    ? 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                <span className="text-base">A</span>
              </button>
              <button
                onClick={() =>
                  setSettings((prev) => ({
                    ...prev,
                    fontSize: 'xlarge',
                  }))
                }
                className={`w-8 h-8 rounded flex items-center justify-center transition-colors ${
                  settings.fontSize === 'xlarge'
                    ? isDark
                      ? 'bg-blue-600 text-white'
                      : 'bg-blue-500 text-white'
                    : isDark
                    ? 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                <span className="text-lg font-bold">A</span>
              </button>
            </div>
          </div>

          {/* 行距 */}
          <div className="mb-4">
            <label
              className={`block text-sm font-medium mb-2 ${
                isDark ? 'text-gray-300' : 'text-gray-700'
              }`}
            >
              行距
            </label>
            <div className="flex items-center gap-2">
              {(['compact', 'normal', 'relaxed', 'loose'] as LineHeight[]).map(
                (height) => (
                  <button
                    key={height}
                    onClick={() =>
                      setSettings((prev) => ({ ...prev, lineHeight: height }))
                    }
                    className={`px-3 py-1.5 rounded text-sm transition-colors ${
                      settings.lineHeight === height
                        ? isDark
                          ? 'bg-blue-600 text-white'
                          : 'bg-blue-500 text-white'
                        : isDark
                        ? 'bg-gray-700 text-gray-300 hover:bg-gray-600'
                        : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                    }`}
                  >
                    {height === 'compact'
                      ? '紧凑'
                      : height === 'normal'
                      ? '适中'
                      : height === 'relaxed'
                      ? '宽松'
                      : '超宽'}
                  </button>
                )
              )}
            </div>
          </div>

          {/* 背景主题 */}
          <div>
            <label
              className={`block text-sm font-medium mb-2 ${
                isDark ? 'text-gray-300' : 'text-gray-700'
              }`}
            >
              背景色
            </label>
            <div className="flex items-center gap-2">
              <button
                onClick={() => setBackgroundTheme('light')}
                className={`w-8 h-8 rounded-full bg-white border-2 transition-all ${
                  settings.backgroundTheme === 'light'
                    ? 'border-blue-500 ring-2 ring-blue-200'
                    : 'border-gray-300'
                }`}
                title="白色"
              />
              <button
                onClick={() => setBackgroundTheme('sepia')}
                className={`w-8 h-8 rounded-full bg-amber-50 border-2 transition-all ${
                  settings.backgroundTheme === 'sepia'
                    ? 'border-blue-500 ring-2 ring-blue-200'
                    : 'border-gray-300'
                }`}
                title="米黄"
              />
              <button
                onClick={() => setBackgroundTheme('dark')}
                className={`w-8 h-8 rounded-full bg-gray-900 border-2 transition-all ${
                  settings.backgroundTheme === 'dark'
                    ? 'border-blue-500 ring-2 ring-blue-400'
                    : 'border-gray-600'
                }`}
                title="深色"
              />
            </div>
          </div>

          {/* 快捷键提示 */}
          <div
            className={`mt-4 pt-4 border-t text-xs ${
              isDark ? 'border-gray-700 text-gray-500' : 'border-gray-200 text-gray-400'
            }`}
          >
            <div className="flex items-center gap-4">
              <span>
                <kbd className="px-1.5 py-0.5 rounded bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300">
                  Esc
                </kbd>{' '}
                退出
              </span>
              <span>
                <kbd className="px-1.5 py-0.5 rounded bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300">
                  ←
                </kbd>{' '}
                上一章
              </span>
              <span>
                <kbd className="px-1.5 py-0.5 rounded bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300">
                  →
                </kbd>{' '}
                下一章
              </span>
            </div>
          </div>
        </div>
      )}

      {/* 内容区域 */}
      <main className="flex-1 overflow-y-auto">
        <article
          className={`max-w-3xl mx-auto px-6 py-8 sm:px-12 sm:py-12 ${
            fontSizeMap[settings.fontSize]
          } ${lineHeightMap[settings.lineHeight]}`}
        >
          {/* 章节标题 */}
          <header className="mb-8 sm:mb-12">
            <h2
              className={`text-2xl sm:text-3xl font-bold mb-2 ${
                isDark ? 'text-white' : 'text-gray-900'
              }`}
            >
              {chapter.title}
            </h2>
            <div
              className={`flex items-center gap-4 text-sm ${
                isDark ? 'text-gray-500' : 'text-gray-400'
              }`}
            >
              <span>{chapter.word_count.toLocaleString()} 字</span>
              <span>·</span>
              <span>{estimatedReadingTime}</span>
            </div>
          </header>

          {/* 段落内容 */}
          <div className="space-y-4">
            {paragraphs.map((paragraph, index) => (
              <p
                key={index}
                className={`text-justify indent-[2em] ${
                  isDark ? 'text-gray-300' : 'text-gray-800'
                }`}
              >
                {paragraph}
              </p>
            ))}
          </div>

          {/* 章节导航 */}
          <nav
            className={`mt-12 pt-8 border-t ${
              isDark ? 'border-gray-700' : 'border-gray-200'
            }`}
          >
            <div className="flex items-center justify-between">
              {prevChapter ? (
                <button
                  onClick={() => onNavigate(prevChapter)}
                  className={`flex items-center gap-2 px-4 py-2 rounded-lg transition-colors ${
                    isDark
                      ? 'hover:bg-gray-800 text-gray-300'
                      : 'hover:bg-gray-100 text-gray-600'
                  }`}
                >
                  <ChevronLeft className="w-5 h-5" />
                  <div className="text-left">
                    <div className="text-xs text-gray-500">上一章</div>
                    <div className="text-sm font-medium truncate max-w-[200px]">
                      {prevChapter.title}
                    </div>
                  </div>
                </button>
              ) : (
                <div />
              )}

              {nextChapter ? (
                <button
                  onClick={() => onNavigate(nextChapter)}
                  className={`flex items-center gap-2 px-4 py-2 rounded-lg transition-colors ${
                    isDark
                      ? 'hover:bg-gray-800 text-gray-300'
                      : 'hover:bg-gray-100 text-gray-600'
                  }`}
                >
                  <div className="text-right">
                    <div className="text-xs text-gray-500">下一章</div>
                    <div className="text-sm font-medium truncate max-w-[200px]">
                      {nextChapter.title}
                    </div>
                  </div>
                  <ChevronRight className="w-5 h-5" />
                </button>
              ) : (
                <div />
              )}
            </div>
          </nav>
        </article>
      </main>

      {/* 底部进度条 */}
      <footer
        className={`px-6 py-3 border-t transition-colors ${
          isDark ? 'border-gray-700 bg-gray-800/95' : 'border-gray-200 bg-white/95'
        } backdrop-blur-sm`}
      >
        <div className="max-w-3xl mx-auto">
          <div className="flex items-center justify-between mb-2">
            <span
              className={`text-sm ${isDark ? 'text-gray-400' : 'text-gray-500'}`}
            >
              {Math.round(((currentIndex + 1) / chapters.length) * 100)}% 已读
            </span>
            <span
              className={`text-sm ${isDark ? 'text-gray-400' : 'text-gray-500'}`}
            >
              第 {currentIndex + 1} 章 / 共 {chapters.length} 章
            </span>
          </div>
          <div
            className={`h-1 rounded-full overflow-hidden ${
              isDark ? 'bg-gray-700' : 'bg-gray-200'
            }`}
          >
            <div
              className={`h-full rounded-full transition-all duration-300 ${
                isDark ? 'bg-blue-500' : 'bg-blue-600'
              }`}
              style={{
                width: `${((currentIndex + 1) / chapters.length) * 100}%`,
              }}
            />
          </div>
        </div>
      </footer>

      {/* 移动端阅读统计 */}
      <div
        className={`sm:hidden fixed bottom-20 left-0 right-0 px-6 py-2 text-center text-xs ${
          isDark ? 'text-gray-500' : 'text-gray-400'
        }`}
      >
        <span>{chapter.word_count.toLocaleString()} 字</span>
        <span className="mx-2">·</span>
        <span>{estimatedReadingTime}</span>
      </div>
    </div>
  );
};
