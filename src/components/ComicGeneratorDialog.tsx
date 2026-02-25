import React, { useState, useEffect } from "react";
import {
  X,
  Sparkles,
  Loader2,
  Copy,
  Download,
  Check,
  BookOpen,
  ChevronLeft,
  ChevronRight,
  Grid3X3,
  Image,
  MessageSquare,
} from "lucide-react";
import { multimediaService } from "../services/multimedia.service";
import type {
  Comic,
  ComicPage,
  ComicPanel,
  ComicDialogue,
  ComicGenerationOptions,
  VisualStyle,
  ComicPanelLayout,
} from "../types/multimedia";
import {
  VisualStyle as VS,
  ComicPanelLayout as CPL,
  VISUAL_STYLE_LABELS,
  COMIC_PANEL_LAYOUT_LABELS,
} from "../types/multimedia";
import type { Chapter } from "../types";

interface ComicGeneratorDialogProps {
  isOpen: boolean;
  onClose: () => void;
  projectId: string;
  chapters: Chapter[];
  currentChapterId?: string;
}

export function ComicGeneratorDialog({
  isOpen,
  onClose,
  projectId,
  chapters,
  currentChapterId,
}: ComicGeneratorDialogProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedChapterId, setSelectedChapterId] = useState<string>("");
  const [comic, setComic] = useState<Comic | null>(null);
  const [currentPage, setCurrentPage] = useState(0);
  const [copiedIndex, setCopiedIndex] = useState<number | null>(null);
  const [options, setOptions] = useState<ComicGenerationOptions>(
    multimediaService.getComicDefaults()
  );
  const [showOptions, setShowOptions] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setSelectedChapterId(currentChapterId || "");
      setComic(null);
      setError(null);
      setCurrentPage(0);
    }
  }, [isOpen, currentChapterId]);

  const handleGenerate = async () => {
    if (!selectedChapterId) {
      setError("请选择一个章节");
      return;
    }

    setLoading(true);
    setError(null);
    setComic(null);

    try {
      const result = await multimediaService.generateComic({
        chapterId: selectedChapterId,
        options,
      });
      setComic(result);
      setCurrentPage(0);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`生成失败: ${errorMessage}`);
    } finally {
      setLoading(false);
    }
  };

  const handleCopyPanel = async (panel: ComicPanel, index: number) => {
    const text = formatPanelText(panel);
    try {
      await navigator.clipboard.writeText(text);
      setCopiedIndex(index);
      setTimeout(() => setCopiedIndex(null), 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  };

  const handleExport = () => {
    if (!comic) return;
    const text = formatComicText(comic);
    const blob = new Blob([text], { type: "text/plain;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `漫画脚本_${comic.title}_${new Date().toISOString().slice(0, 10)}.txt`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const formatDialogueText = (dialogue: ComicDialogue): string => {
    let text = `${dialogue.character}: "${dialogue.text}"`;
    if (dialogue.balloonType !== "speech") {
      text += ` [${dialogue.balloonType}]`;
    }
    return text;
  };

  const formatPanelText = (panel: ComicPanel): string => {
    let text = `【分格 ${panel.panelNumber}】\n`;
    text += `形状: ${panel.shape}\n`;
    if (panel.shot) {
      text += `景别: ${panel.shot.shotType}\n`;
      text += `描述: ${panel.shot.description}\n`;
    }
    if (panel.caption) text += `旁白: ${panel.caption}\n`;
    if (panel.dialogue?.length > 0) {
      text += `对白:\n${panel.dialogue.map((d) => `  ${formatDialogueText(d)}`).join("\n")}\n`;
    }
    if (panel.soundEffects?.length) text += `音效: ${panel.soundEffects.join(", ")}\n`;
    if (panel.visualPrompt) text += `视觉提示词: ${panel.visualPrompt}\n`;
    return text;
  };

  const formatPageText = (page: ComicPage): string => {
    let text = `\n${"═".repeat(50)}\n`;
    text += `第 ${page.pageNumber} 页\n`;
    text += `布局: ${COMIC_PANEL_LAYOUT_LABELS[page.layout] || page.layout}\n`;
    text += `${"═".repeat(50)}\n`;
    page.panels.forEach((panel) => {
      text += formatPanelText(panel) + "\n";
    });
    return text;
  };

  const formatComicText = (c: Comic): string => {
    let text = `漫画脚本: ${c.title}\n`;
    text += `风格: ${VISUAL_STYLE_LABELS[c.style as VisualStyle] || c.style}\n`;
    text += `页数: ${c.pages.length}\n`;
    text += `生成时间: ${c.metadata.generatedAt}\n\n`;

    if (c.characters?.length > 0) {
      text += `角色列表:\n`;
      c.characters.forEach((char) => {
        text += `  - ${char.name}`;
        if (char.appearance) text += ` (${char.appearance})`;
        text += "\n";
      });
      text += "\n";
    }

    c.pages.forEach((page) => {
      text += formatPageText(page);
    });

    return text;
  };

  const getBalloonTypeColor = (type: string) => {
    switch (type) {
      case "speech":
        return "bg-white dark:bg-slate-100";
      case "thought":
        return "bg-blue-50 dark:bg-blue-900/30";
      case "whisper":
        return "bg-gray-50 dark:bg-gray-800";
      case "shout":
        return "bg-red-50 dark:bg-red-900/30";
      case "electronic":
        return "bg-green-50 dark:bg-green-900/30";
      default:
        return "bg-white dark:bg-slate-100";
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-5xl bg-white dark:bg-slate-800 rounded-lg shadow-xl max-h-[90vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-2">
            <BookOpen className="w-5 h-5 text-orange-500" />
            <h3 className="font-semibold text-slate-900 dark:text-slate-100">AI 漫画分镜生成</h3>
          </div>
          <button
            onClick={onClose}
            className="p-1 hover:bg-slate-100 dark:hover:bg-slate-700 rounded"
          >
            <X className="w-5 h-5 text-slate-500" />
          </button>
        </div>

        <div className="p-4 border-b border-slate-200 dark:border-slate-700">
          <div className="flex items-center gap-4 mb-4">
            <div className="flex-1">
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">
                选择章节
              </label>
              <select
                value={selectedChapterId}
                onChange={(e) => setSelectedChapterId(e.target.value)}
                className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100 focus:outline-none focus:ring-2 focus:ring-orange-500"
              >
                <option value="">请选择章节</option>
                {chapters.map((chapter) => (
                  <option key={chapter.id} value={chapter.id}>
                    {chapter.title}
                  </option>
                ))}
              </select>
            </div>
            <button
              onClick={handleGenerate}
              disabled={loading || !selectedChapterId}
              className="flex items-center gap-2 px-4 py-2 bg-orange-500 hover:bg-orange-600 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed mt-5"
            >
              {loading ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  生成中...
                </>
              ) : (
                <>
                  <Sparkles className="w-4 h-4" />
                  生成漫画分镜
                </>
              )}
            </button>
          </div>

          <button
            onClick={() => setShowOptions(!showOptions)}
            className="flex items-center gap-1 text-sm text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
          >
            {showOptions ? "▼" : "▶"} 高级选项
          </button>

          {showOptions && (
            <div className="mt-3 p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg grid grid-cols-3 gap-4">
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  视觉风格
                </label>
                <select
                  value={options.style}
                  onChange={(e) => setOptions({ ...options, style: e.target.value as VisualStyle })}
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                >
                  {Object.entries(VISUAL_STYLE_LABELS).map(([key, label]) => (
                    <option key={key} value={key}>
                      {label}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  页面布局
                </label>
                <select
                  value={options.pageLayout}
                  onChange={(e) =>
                    setOptions({ ...options, pageLayout: e.target.value as ComicPanelLayout })
                  }
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                >
                  {Object.entries(COMIC_PANEL_LAYOUT_LABELS).map(([key, label]) => (
                    <option key={key} value={key}>
                      {label}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 dark:text-slate-400 mb-1">
                  每页格数
                </label>
                <input
                  type="number"
                  min={1}
                  max={8}
                  value={options.panelsPerPage}
                  onChange={(e) =>
                    setOptions({ ...options, panelsPerPage: parseInt(e.target.value) || 4 })
                  }
                  className="w-full px-2 py-1.5 text-sm border border-slate-300 dark:border-slate-600 rounded bg-white dark:bg-slate-700 text-slate-900 dark:text-slate-100"
                />
              </div>
              <div className="col-span-3 flex items-center gap-4">
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.includeCaptions}
                    onChange={(e) => setOptions({ ...options, includeCaptions: e.target.checked })}
                    className="rounded border-slate-300"
                  />
                  包含旁白
                </label>
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.includeSoundEffects}
                    onChange={(e) =>
                      setOptions({ ...options, includeSoundEffects: e.target.checked })
                    }
                    className="rounded border-slate-300"
                  />
                  包含音效
                </label>
                <label className="flex items-center gap-1.5 text-sm text-slate-600 dark:text-slate-400">
                  <input
                    type="checkbox"
                    checked={options.generateImages}
                    onChange={(e) => setOptions({ ...options, generateImages: e.target.checked })}
                    className="rounded border-slate-300"
                  />
                  生成图像 (需要图像服务)
                </label>
              </div>
            </div>
          )}

          {error && (
            <div className="mt-3 p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
              {error}
            </div>
          )}
        </div>

        {comic && comic.pages.length > 0 && (
          <div className="p-4 border-b border-slate-200 dark:border-slate-700 flex items-center justify-between">
            <div className="flex items-center gap-4">
              <button
                onClick={() => setCurrentPage(Math.max(0, currentPage - 1))}
                disabled={currentPage === 0}
                className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50"
              >
                <ChevronLeft className="w-5 h-5 text-slate-500" />
              </button>
              <span className="text-sm text-slate-600 dark:text-slate-400">
                第 {currentPage + 1} / {comic.pages.length} 页
              </span>
              <button
                onClick={() => setCurrentPage(Math.min(comic.pages.length - 1, currentPage + 1))}
                disabled={currentPage === comic.pages.length - 1}
                className="p-1 rounded hover:bg-slate-100 dark:hover:bg-slate-700 disabled:opacity-50"
              >
                <ChevronRight className="w-5 h-5 text-slate-500" />
              </button>
            </div>
            <div className="flex items-center gap-2">
              <button
                onClick={handleExport}
                className="flex items-center gap-1 px-3 py-1.5 text-sm bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 rounded-lg transition-colors"
              >
                <Download className="w-4 h-4" />
                导出脚本
              </button>
            </div>
          </div>
        )}

        <div className="flex-1 overflow-y-auto p-4">
          {!comic && !loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <BookOpen className="w-12 h-12 mb-4" />
              <p className="text-sm">选择章节后点击"生成漫画分镜"</p>
              <p className="text-xs mt-1">AI 将根据章节内容自动生成漫画分镜脚本</p>
            </div>
          )}

          {loading && (
            <div className="flex flex-col items-center justify-center h-64 text-slate-400">
              <Loader2 className="w-12 h-12 animate-spin mb-4" />
              <p className="text-sm">正在生成漫画分镜...</p>
              <p className="text-xs mt-1">AI 正在分析章节内容并生成漫画分镜</p>
            </div>
          )}

          {comic && comic.pages.length > 0 && (
            <div>
              {comic.characters && comic.characters.length > 0 && (
                <div className="mb-4 p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                  <div className="flex items-center gap-2 mb-2">
                    <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                      角色
                    </span>
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {comic.characters.map((char, index) => (
                      <span
                        key={index}
                        className="px-2 py-1 bg-white dark:bg-slate-600 rounded text-xs text-slate-600 dark:text-slate-300"
                      >
                        {char.name}
                      </span>
                    ))}
                  </div>
                </div>
              )}

              <div className="bg-slate-100 dark:bg-slate-700 rounded-lg p-4">
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center gap-2">
                    <Grid3X3 className="w-4 h-4 text-slate-500" />
                    <span className="text-sm font-medium text-slate-700 dark:text-slate-300">
                      第 {comic.pages[currentPage].pageNumber} 页
                    </span>
                    <span className="text-xs text-slate-500 dark:text-slate-400">
                      (
                      {
                        COMIC_PANEL_LAYOUT_LABELS[
                          comic.pages[currentPage].layout as ComicPanelLayout
                        ]
                      }
                      )
                    </span>
                  </div>
                </div>

                <div
                  className="grid gap-4"
                  style={{
                    gridTemplateColumns:
                      comic.pages[currentPage].layout === CPL.SINGLE
                        ? "1fr"
                        : comic.pages[currentPage].layout === CPL.TWO_HORIZONTAL
                          ? "repeat(2, 1fr)"
                          : comic.pages[currentPage].layout === CPL.THREE_HORIZONTAL
                            ? "repeat(3, 1fr)"
                            : comic.pages[currentPage].layout === CPL.FOUR_GRID
                              ? "repeat(2, 1fr)"
                              : comic.pages[currentPage].layout === CPL.SIX_GRID
                                ? "repeat(3, 1fr)"
                                : "1fr",
                  }}
                >
                  {comic.pages[currentPage].panels.map((panel, panelIndex) => (
                    <div
                      key={panelIndex}
                      className="bg-white dark:bg-slate-800 rounded-lg border border-slate-200 dark:border-slate-600 overflow-hidden"
                    >
                      <div className="p-3 border-b border-slate-100 dark:border-slate-700">
                        <div className="flex items-center justify-between">
                          <div className="flex items-center gap-2">
                            <span className="px-2 py-0.5 bg-orange-500 text-white text-xs rounded">
                              分格 {panel.panelNumber}
                            </span>
                            <span className="text-xs text-slate-500 dark:text-slate-400">
                              {panel.shape}
                            </span>
                          </div>
                          <button
                            onClick={() => handleCopyPanel(panel, panelIndex)}
                            className="flex items-center gap-1 px-2 py-0.5 text-xs bg-slate-100 dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 rounded"
                          >
                            {copiedIndex === panelIndex ? (
                              <>
                                <Check className="w-3 h-3 text-green-500" />
                                已复制
                              </>
                            ) : (
                              <>
                                <Copy className="w-3 h-3" />
                                复制
                              </>
                            )}
                          </button>
                        </div>
                      </div>

                      <div className="p-3">
                        {panel.shot && (
                          <div className="mb-3">
                            <div className="flex items-center gap-1 text-xs text-slate-500 dark:text-slate-400 mb-1">
                              <Image className="w-3 h-3" />
                              {panel.shot.shotType}
                            </div>
                            <p className="text-sm text-slate-700 dark:text-slate-300">
                              {panel.shot.description}
                            </p>
                          </div>
                        )}

                        {panel.caption && (
                          <div className="mb-2 p-2 bg-slate-50 dark:bg-slate-700 rounded text-xs italic text-slate-600 dark:text-slate-400">
                            📝 {panel.caption}
                          </div>
                        )}

                        {panel.dialogue && panel.dialogue.length > 0 && (
                          <div className="space-y-2 mb-2">
                            {panel.dialogue.map((d, dIndex) => (
                              <div
                                key={dIndex}
                                className={`p-2 rounded border text-xs ${getBalloonTypeColor(d.balloonType)} border-slate-200 dark:border-slate-600`}
                              >
                                <div className="flex items-center gap-1 mb-0.5">
                                  <MessageSquare className="w-3 h-3 text-slate-400" />
                                  <span className="font-medium text-slate-700 dark:text-slate-300">
                                    {d.character}
                                  </span>
                                  {d.balloonType !== "speech" && (
                                    <span className="text-xs text-slate-400">
                                      [{d.balloonType}]
                                    </span>
                                  )}
                                </div>
                                <p className="text-slate-600 dark:text-slate-400">"{d.text}"</p>
                              </div>
                            ))}
                          </div>
                        )}

                        {panel.soundEffects && panel.soundEffects.length > 0 && (
                          <div className="flex flex-wrap gap-1 mb-2">
                            {panel.soundEffects.map((sfx, sfxIndex) => (
                              <span
                                key={sfxIndex}
                                className="px-2 py-0.5 bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-400 text-xs rounded font-bold"
                              >
                                {sfx}
                              </span>
                            ))}
                          </div>
                        )}

                        {panel.visualPrompt && (
                          <div className="mt-2 p-2 bg-orange-50 dark:bg-orange-900/20 rounded border border-orange-200 dark:border-orange-800">
                            <div className="text-xs text-orange-600 dark:text-orange-400 font-medium mb-1">
                              视觉提示词
                            </div>
                            <p className="text-xs text-orange-800 dark:text-orange-300">
                              {panel.visualPrompt}
                            </p>
                          </div>
                        )}

                        {panel.imageData && (
                          <div className="mt-2">
                            <img
                              src={panel.imageData}
                              alt={`Panel ${panel.panelNumber}`}
                              className="w-full rounded border border-slate-200 dark:border-slate-600"
                            />
                          </div>
                        )}
                      </div>
                    </div>
                  ))}
                </div>

                {comic.pages[currentPage].notes && (
                  <div className="mt-3 p-2 bg-slate-50 dark:bg-slate-600 rounded text-xs text-slate-500 dark:text-slate-400">
                    备注: {comic.pages[currentPage].notes}
                  </div>
                )}
              </div>
            </div>
          )}
        </div>

        <div className="p-4 border-t border-slate-200 dark:border-slate-700 flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 rounded-lg font-medium transition-colors"
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}
