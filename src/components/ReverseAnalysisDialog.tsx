import React, { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface ExtractedCharacter {
  name: string;
  aliases: string[];
  description: string;
  personality: string;
  appearance: string;
  role: string;
  first_appearance: string | null;
  mention_count: number;
}

interface ExtractedRelationship {
  character1: string;
  character2: string;
  relationship_type: string;
  description: string;
  strength: number;
}

interface ExtractedWorldview {
  name: string;
  category: string;
  description: string;
  details: string[];
}

interface ExtractedPlotPoint {
  chapter_index: number;
  title: string;
  description: string;
  plot_type: string;
  characters_involved: string[];
  importance: number;
}

interface OutlineArc {
  title: string;
  start_chapter: number;
  end_chapter: number;
  summary: string;
  key_events: string[];
}

interface ExtractedOutline {
  arcs: OutlineArc[];
}

interface StyleAnalysis {
  writing_style: string;
  narrative_voice: string;
  dialogue_ratio: number;
  description_ratio: number;
  average_sentence_length: number;
  vocabulary_richness: number;
  pacing: string;
  tone: string;
}

interface ReverseAnalysisResult {
  title: string;
  summary: string;
  total_words: number;
  chapter_count: number;
  characters: ExtractedCharacter[];
  relationships: ExtractedRelationship[];
  worldviews: ExtractedWorldview[];
  plot_points: ExtractedPlotPoint[];
  outline: ExtractedOutline;
  style_analysis: StyleAnalysis;
}

interface ReverseAnalysisDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onImportResults: (result: ReverseAnalysisResult) => void;
}

type AnalysisDepth = "basic" | "standard" | "deep";
type AnalysisTab =
  | "overview"
  | "characters"
  | "relationships"
  | "worldviews"
  | "plot"
  | "outline"
  | "style";

export default function ReverseAnalysisDialog({
  isOpen,
  onClose,
  onImportResults,
}: ReverseAnalysisDialogProps) {
  const [content, setContent] = useState("");
  const [title, setTitle] = useState("");
  const [depth, setDepth] = useState<AnalysisDepth>("standard");
  const [isLoading, setIsLoading] = useState(false);
  const [result, setResult] = useState<ReverseAnalysisResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<AnalysisTab>("overview");
  const [importOptions, setImportOptions] = useState({
    characters: true,
    worldviews: true,
    outline: true,
  });
  const fileInputRef = useRef<HTMLInputElement>(null);

  if (!isOpen) return null;

  const handleLoadFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "文本文件", extensions: ["txt", "md"] }],
      });

      if (selected && typeof selected === "string") {
        const fileContent = await invoke<string>("read_text_file", { path: selected });
        setContent(fileContent);
        const fileName = selected.split(/[/\\]/).pop() || "未知小说";
        setTitle(fileName.replace(/\.(txt|md)$/, ""));
        setError(null);
      }
    } catch (err) {
      console.error("读取文件失败:", err);
      setError("读取文件失败: " + String(err));
    }
  };

  const handleAnalyze = async () => {
    if (!content.trim()) {
      setError("请先输入或导入小说内容");
      return;
    }
    if (!title.trim()) {
      setError("请输入小说标题");
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const analysisResult = await invoke<ReverseAnalysisResult>("reverse_analyze_novel", {
        content: content,
        title: title,
        depth: depth,
      });
      setResult(analysisResult);
    } catch (err) {
      console.error("分析失败:", err);
      setError("分析失败: " + String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleImport = async () => {
    if (!result) return;

    setIsLoading(true);
    try {
      await invoke<ReverseAnalysisResult>("reverse_analyze_and_import", {
        content: content,
        title: title,
        import_characters: importOptions.characters,
        import_worldviews: importOptions.worldviews,
        import_outline: importOptions.outline,
      });
      onImportResults(result);
      handleClose();
    } catch (err) {
      console.error("导入失败:", err);
      setError("导入失败: " + String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setContent("");
    setTitle("");
    setResult(null);
    setError(null);
    setActiveTab("overview");
    onClose();
  };

  const renderOverview = () => (
    <div className="space-y-4">
      <div className="grid grid-cols-2 gap-4">
        <div className="p-4 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="text-xs text-slate-500 dark:text-slate-400">总字数</div>
          <div className="text-2xl font-bold text-slate-800 dark:text-slate-200">
            {result?.total_words.toLocaleString()}
          </div>
        </div>
        <div className="p-4 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="text-xs text-slate-500 dark:text-slate-400">章节数</div>
          <div className="text-2xl font-bold text-slate-800 dark:text-slate-200">
            {result?.chapter_count}
          </div>
        </div>
      </div>

      <div className="p-4 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
        <div className="text-xs text-slate-500 dark:text-slate-400 mb-1">摘要</div>
        <div className="text-sm text-slate-700 dark:text-slate-300">{result?.summary}</div>
      </div>

      <div className="grid grid-cols-4 gap-2">
        <div className="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg text-center">
          <div className="text-lg font-bold text-blue-600 dark:text-blue-400">
            {result?.characters.length || 0}
          </div>
          <div className="text-xs text-slate-500 dark:text-slate-400">角色</div>
        </div>
        <div className="p-3 bg-green-50 dark:bg-green-900/20 rounded-lg text-center">
          <div className="text-lg font-bold text-green-600 dark:text-green-400">
            {result?.relationships.length || 0}
          </div>
          <div className="text-xs text-slate-500 dark:text-slate-400">关系</div>
        </div>
        <div className="p-3 bg-purple-50 dark:bg-purple-900/20 rounded-lg text-center">
          <div className="text-lg font-bold text-purple-600 dark:text-purple-400">
            {result?.worldviews.length || 0}
          </div>
          <div className="text-xs text-slate-500 dark:text-slate-400">世界观</div>
        </div>
        <div className="p-3 bg-orange-50 dark:bg-orange-900/20 rounded-lg text-center">
          <div className="text-lg font-bold text-orange-600 dark:text-orange-400">
            {result?.plot_points.length || 0}
          </div>
          <div className="text-xs text-slate-500 dark:text-slate-400">情节</div>
        </div>
      </div>
    </div>
  );

  const renderCharacters = () => (
    <div className="space-y-2 max-h-96 overflow-y-auto">
      {result?.characters.map((char, index) => (
        <div key={index} className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="flex items-center justify-between mb-2">
            <div className="flex items-center gap-2">
              <span className="font-medium text-slate-800 dark:text-slate-200">{char.name}</span>
              <span
                className={`px-2 py-0.5 text-xs rounded ${
                  char.role === "主角"
                    ? "bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400"
                    : char.role === "配角"
                      ? "bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400"
                      : "bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400"
                }`}
              >
                {char.role}
              </span>
            </div>
            <span className="text-xs text-slate-500 dark:text-slate-400">
              出场 {char.mention_count} 次
            </span>
          </div>
          {char.description && (
            <div className="text-sm text-slate-600 dark:text-slate-400">{char.description}</div>
          )}
        </div>
      ))}
    </div>
  );

  const renderRelationships = () => (
    <div className="space-y-2 max-h-96 overflow-y-auto">
      {result?.relationships.map((rel, index) => (
        <div key={index} className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="flex items-center gap-2 mb-1">
            <span className="font-medium text-slate-800 dark:text-slate-200">{rel.character1}</span>
            <span className="text-slate-500 dark:text-slate-400">↔</span>
            <span className="font-medium text-slate-800 dark:text-slate-200">{rel.character2}</span>
            <span
              className={`px-2 py-0.5 text-xs rounded bg-slate-200 dark:bg-slate-600 text-slate-600 dark:text-slate-300`}
            >
              {rel.relationship_type}
            </span>
          </div>
          <div className="text-sm text-slate-600 dark:text-slate-400">{rel.description}</div>
        </div>
      ))}
    </div>
  );

  const renderWorldviews = () => (
    <div className="space-y-2 max-h-96 overflow-y-auto">
      {result?.worldviews.map((wv, index) => (
        <div key={index} className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="flex items-center justify-between mb-1">
            <span className="font-medium text-slate-800 dark:text-slate-200">{wv.name}</span>
            <span className="px-2 py-0.5 text-xs rounded bg-purple-100 text-purple-600 dark:bg-purple-900/30 dark:text-purple-400">
              {wv.category}
            </span>
          </div>
          {wv.description && (
            <div className="text-sm text-slate-600 dark:text-slate-400">{wv.description}</div>
          )}
        </div>
      ))}
    </div>
  );

  const renderPlotPoints = () => (
    <div className="space-y-2 max-h-96 overflow-y-auto">
      {result?.plot_points.map((pp, index) => (
        <div key={index} className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="flex items-center justify-between mb-1">
            <span className="font-medium text-slate-800 dark:text-slate-200">{pp.title}</span>
            <span className="text-xs text-slate-500 dark:text-slate-400">
              第 {pp.chapter_index + 1} 章
            </span>
          </div>
          <div className="text-sm text-slate-600 dark:text-slate-400">{pp.description}</div>
        </div>
      ))}
    </div>
  );

  const renderOutline = () => (
    <div className="space-y-3 max-h-96 overflow-y-auto">
      {result?.outline.arcs.map((arc, index) => (
        <div key={index} className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="flex items-center justify-between mb-2">
            <span className="font-medium text-slate-800 dark:text-slate-200">{arc.title}</span>
            <span className="text-xs text-slate-500 dark:text-slate-400">
              第 {arc.start_chapter} - {arc.end_chapter} 章
            </span>
          </div>
          <div className="text-sm text-slate-600 dark:text-slate-400">{arc.summary}</div>
        </div>
      ))}
    </div>
  );

  const renderStyle = () => (
    <div className="space-y-3">
      <div className="grid grid-cols-2 gap-4">
        <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="text-xs text-slate-500 dark:text-slate-400">叙事视角</div>
          <div className="text-sm font-medium text-slate-800 dark:text-slate-200">
            {result?.style_analysis.narrative_voice}
          </div>
        </div>
        <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="text-xs text-slate-500 dark:text-slate-400">节奏</div>
          <div className="text-sm font-medium text-slate-800 dark:text-slate-200">
            {result?.style_analysis.pacing}
          </div>
        </div>
        <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="text-xs text-slate-500 dark:text-slate-400">基调</div>
          <div className="text-sm font-medium text-slate-800 dark:text-slate-200">
            {result?.style_analysis.tone}
          </div>
        </div>
        <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
          <div className="text-xs text-slate-500 dark:text-slate-400">平均句长</div>
          <div className="text-sm font-medium text-slate-800 dark:text-slate-200">
            {result?.style_analysis.average_sentence_length.toFixed(1)} 字
          </div>
        </div>
      </div>
      <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
        <div className="text-xs text-slate-500 dark:text-slate-400 mb-2">对话/描写比例</div>
        <div className="flex items-center gap-2">
          <div className="flex-1 h-3 bg-slate-200 dark:bg-slate-600 rounded-full overflow-hidden">
            <div
              className="h-full bg-blue-500"
              style={{ width: `${(result?.style_analysis.dialogue_ratio || 0) * 100}%` }}
            />
          </div>
          <span className="text-xs text-slate-600 dark:text-slate-400">
            对话 {((result?.style_analysis.dialogue_ratio || 0) * 100).toFixed(1)}%
          </span>
        </div>
      </div>
    </div>
  );

  const tabs: { id: AnalysisTab; label: string; icon: string }[] = [
    { id: "overview", label: "概览", icon: "📊" },
    { id: "characters", label: "角色", icon: "👤" },
    { id: "relationships", label: "关系", icon: "🔗" },
    { id: "worldviews", label: "世界观", icon: "🌍" },
    { id: "plot", label: "情节", icon: "📖" },
    { id: "outline", label: "大纲", icon: "📑" },
    { id: "style", label: "风格", icon: "✨" },
  ];

  const renderTabContent = () => {
    switch (activeTab) {
      case "overview":
        return renderOverview();
      case "characters":
        return renderCharacters();
      case "relationships":
        return renderRelationships();
      case "worldviews":
        return renderWorldviews();
      case "plot":
        return renderPlotPoints();
      case "outline":
        return renderOutline();
      case "style":
        return renderStyle();
      default:
        return null;
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-4xl max-h-[90vh] overflow-hidden">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold text-slate-800 dark:text-slate-100">小说逆向分析</h2>
          <button
            onClick={handleClose}
            className="text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200"
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
        </div>

        <div className="p-4 overflow-y-auto" style={{ maxHeight: "calc(90vh - 140px)" }}>
          {!result ? (
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                    小说标题
                  </label>
                  <input
                    type="text"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                    placeholder="输入小说标题"
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                    分析深度
                  </label>
                  <select
                    value={depth}
                    onChange={(e) => setDepth(e.target.value as AnalysisDepth)}
                    className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200"
                  >
                    <option value="basic">基础分析</option>
                    <option value="standard">标准分析</option>
                    <option value="deep">深度分析</option>
                  </select>
                </div>
              </div>

              <div>
                <div className="flex items-center justify-between mb-2">
                  <label className="block text-sm font-medium text-slate-700 dark:text-slate-300">
                    小说内容
                  </label>
                  <button
                    onClick={handleLoadFile}
                    className="text-sm text-blue-500 hover:text-blue-600"
                  >
                    📁 从文件导入
                  </button>
                </div>
                <textarea
                  value={content}
                  onChange={(e) => setContent(e.target.value)}
                  placeholder="粘贴小说内容或点击上方按钮导入文件..."
                  rows={12}
                  className="w-full px-3 py-2 border border-slate-300 dark:border-slate-600 rounded-lg bg-white dark:bg-slate-700 text-slate-800 dark:text-slate-200 resize-none"
                />
                <div className="mt-1 text-xs text-slate-500 dark:text-slate-400">
                  当前字数: {content.length.toLocaleString()}
                </div>
              </div>

              {error && (
                <div className="p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
                  {error}
                </div>
              )}
            </div>
          ) : (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-medium text-slate-800 dark:text-slate-200">
                  {result.title} - 分析结果
                </h3>
                <button
                  onClick={() => setResult(null)}
                  className="text-sm text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200"
                >
                  重新分析
                </button>
              </div>

              <div className="flex gap-1 border-b border-slate-200 dark:border-slate-700">
                {tabs.map((tab) => (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={`px-3 py-2 text-sm flex items-center gap-1 border-b-2 transition-colors ${
                      activeTab === tab.id
                        ? "border-blue-500 text-blue-600 dark:text-blue-400"
                        : "border-transparent text-slate-500 dark:text-slate-400 hover:text-slate-700 dark:hover:text-slate-300"
                    }`}
                  >
                    <span>{tab.icon}</span>
                    <span>{tab.label}</span>
                  </button>
                ))}
              </div>

              {renderTabContent()}

              <div className="p-4 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                <div className="text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
                  导入选项
                </div>
                <div className="flex gap-4">
                  <label className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={importOptions.characters}
                      onChange={(e) =>
                        setImportOptions({ ...importOptions, characters: e.target.checked })
                      }
                      className="rounded border-slate-300"
                    />
                    <span className="text-sm text-slate-600 dark:text-slate-400">角色</span>
                  </label>
                  <label className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={importOptions.worldviews}
                      onChange={(e) =>
                        setImportOptions({ ...importOptions, worldviews: e.target.checked })
                      }
                      className="rounded border-slate-300"
                    />
                    <span className="text-sm text-slate-600 dark:text-slate-400">世界观</span>
                  </label>
                  <label className="flex items-center gap-2">
                    <input
                      type="checkbox"
                      checked={importOptions.outline}
                      onChange={(e) =>
                        setImportOptions({ ...importOptions, outline: e.target.checked })
                      }
                      className="rounded border-slate-300"
                    />
                    <span className="text-sm text-slate-600 dark:text-slate-400">大纲</span>
                  </label>
                </div>
              </div>

              {error && (
                <div className="p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
                  {error}
                </div>
              )}
            </div>
          )}
        </div>

        <div className="flex items-center justify-end gap-2 p-4 border-t border-slate-200 dark:border-slate-700">
          {!result ? (
            <>
              <button
                onClick={handleClose}
                className="px-4 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
              >
                取消
              </button>
              <button
                onClick={handleAnalyze}
                disabled={!content.trim() || !title.trim() || isLoading}
                className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? "分析中..." : "开始分析"}
              </button>
            </>
          ) : (
            <>
              <button
                onClick={handleClose}
                className="px-4 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
              >
                关闭
              </button>
              <button
                onClick={handleImport}
                disabled={isLoading}
                className="px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? "导入中..." : "导入到项目"}
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
