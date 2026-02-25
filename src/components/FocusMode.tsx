import React, { useState, useEffect, useCallback } from "react";

interface FocusModeProps {
  content: string;
  title: string;
  onChange: (content: string) => void;
  onSave: () => void;
  onExit: () => void;
  wordCount: number;
}

type ThemeType = "dark" | "light" | "sepia";
type FontSizeType = "small" | "medium" | "large" | "xlarge";

interface ThemeConfig {
  bg: string;
  text: string;
  secondary: string;
  border: string;
}

interface FontSizeConfig {
  name: string;
  size: string;
}

const themes: Record<ThemeType, ThemeConfig> = {
  dark: {
    bg: "bg-slate-900",
    text: "text-slate-100",
    secondary: "text-slate-400",
    border: "border-slate-700",
  },
  light: {
    bg: "bg-white",
    text: "text-slate-800",
    secondary: "text-slate-500",
    border: "border-slate-200",
  },
  sepia: {
    bg: "bg-amber-50",
    text: "text-amber-900",
    secondary: "text-amber-700",
    border: "border-amber-200",
  },
};

const fontSizes: Record<FontSizeType, FontSizeConfig> = {
  small: { name: "小", size: "text-base" },
  medium: { name: "中", size: "text-lg" },
  large: { name: "大", size: "text-xl" },
  xlarge: { name: "特大", size: "text-2xl" },
};

export default function FocusMode({
  content,
  title,
  onChange,
  onSave,
  onExit,
  wordCount,
}: FocusModeProps) {
  const [theme, setTheme] = useState<ThemeType>("dark");
  const [fontSize, setFontSize] = useState<FontSizeType>("medium");
  const [showSettings, setShowSettings] = useState(false);
  const [isTyping, setIsTyping] = useState(true);
  const [showToolbar, setShowToolbar] = useState(true);

  const currentTheme = themes[theme];
  const currentFontSize = fontSizes[fontSize];

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onExit();
      }
      if ((e.metaKey || e.ctrlKey) && e.key === "s") {
        e.preventDefault();
        onSave();
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [onExit, onSave]);

  useEffect(() => {
    let timeout: NodeJS.Timeout;
    const handleMouseMove = () => {
      setShowToolbar(true);
      clearTimeout(timeout);
      timeout = setTimeout(() => {
        if (!showSettings) {
          setShowToolbar(false);
        }
      }, 3000);
    };

    document.addEventListener("mousemove", handleMouseMove);
    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      clearTimeout(timeout);
    };
  }, [showSettings]);

  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      onChange(e.target.value);
      setIsTyping(true);
    },
    [onChange]
  );

  const cycleTheme = () => {
    const themeOrder: ThemeType[] = ["dark", "light", "sepia"];
    const currentIndex = themeOrder.indexOf(theme);
    setTheme(themeOrder[(currentIndex + 1) % themeOrder.length]);
  };

  const cycleFontSize = () => {
    const sizeOrder: FontSizeType[] = ["small", "medium", "large", "xlarge"];
    const currentIndex = sizeOrder.indexOf(fontSize);
    setFontSize(sizeOrder[(currentIndex + 1) % sizeOrder.length]);
  };

  return (
    <div className={`fixed inset-0 z-50 ${currentTheme.bg} transition-colors duration-300`}>
      <div
        className={`absolute top-0 left-0 right-0 p-4 flex items-center justify-between transition-transform duration-300 ${
          showToolbar ? "translate-y-0" : "-translate-y-full"
        }`}
      >
        <div className="flex items-center gap-4">
          <button
            onClick={onExit}
            className={`p-2 rounded-lg hover:bg-white/10 transition-colors ${currentTheme.text}`}
            title="退出专注模式 (Esc)"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
          <div className={`${currentTheme.text} font-medium`}>{title}</div>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={onSave}
            className={`px-3 py-1.5 rounded-lg hover:bg-white/10 transition-colors ${currentTheme.text} text-sm`}
            title="保存 (Cmd/Ctrl + S)"
          >
            保存
          </button>
          <div className={`px-3 py-1.5 ${currentTheme.secondary} text-sm`}>{wordCount} 字</div>
          <button
            onClick={cycleTheme}
            className={`p-2 rounded-lg hover:bg-white/10 transition-colors ${currentTheme.text}`}
            title="切换主题"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"
              />
            </svg>
          </button>
          <button
            onClick={cycleFontSize}
            className={`p-2 rounded-lg hover:bg-white/10 transition-colors ${currentTheme.text}`}
            title={`字体大小: ${currentFontSize.name}`}
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 6h16M4 12h16m-7 6h7"
              />
            </svg>
          </button>
        </div>
      </div>

      <div className="h-full flex items-center justify-center pt-16 pb-8 px-8">
        <textarea
          value={content}
          onChange={handleChange}
          className={`w-full max-w-3xl h-full ${currentFontSize.size} ${currentTheme.text} bg-transparent resize-none outline-none leading-relaxed`}
          placeholder="开始写作..."
          autoFocus
        />
      </div>

      <div
        className={`absolute bottom-0 left-0 right-0 p-4 flex items-center justify-center gap-4 transition-transform duration-300 ${
          showToolbar ? "translate-y-0" : "translate-y-full"
        }`}
      >
        <div className={`flex items-center gap-4 ${currentTheme.secondary} text-sm`}>
          <span>主题: {theme === "dark" ? "深色" : theme === "light" ? "浅色" : "护眼"}</span>
          <span>|</span>
          <span>字体: {currentFontSize.name}</span>
          <span>|</span>
          <span>按 Esc 退出专注模式</span>
        </div>
      </div>
    </div>
  );
}
