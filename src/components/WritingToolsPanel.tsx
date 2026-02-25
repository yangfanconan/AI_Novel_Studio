import React, { useState } from "react";
import {
  Shield,
  SpellCheck,
  CheckSquare,
  FileCheck,
  AlertTriangle,
  CheckCircle,
  Zap,
} from "lucide-react";
import { writingToolsService, FullWritingToolsAnalysis } from "../services/writingTools.service";

interface WritingToolsPanelProps {
  text: string;
}

export const WritingToolsPanel: React.FC<WritingToolsPanelProps> = ({ text }) => {
  const [analysis, setAnalysis] = useState<FullWritingToolsAnalysis | null>(null);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<"sensitive" | "typos" | "grammar" | "format">(
    "sensitive"
  );

  const runAnalysis = async () => {
    setLoading(true);
    try {
      const result = await writingToolsService.runFullWritingTools(text);
      setAnalysis(result);
    } catch (error) {
      console.error("Analysis failed:", error);
    } finally {
      setLoading(false);
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case "low":
        return "text-yellow-600";
      case "medium":
        return "text-orange-600";
      case "high":
        return "text-red-600";
      default:
        return "text-gray-600";
    }
  };

  const getSeverityBadge = (severity: string) => {
    switch (severity) {
      case "low":
        return "bg-yellow-100 text-yellow-800";
      case "medium":
        return "bg-orange-100 text-orange-800";
      case "high":
        return "bg-red-100 text-red-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  return (
    <div className="border-b border-border bg-card">
      <div className="px-4 py-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <FileCheck className="w-4 h-4 text-primary" />
            <span className="text-sm font-medium">写作工具</span>
          </div>

          <button
            onClick={runAnalysis}
            disabled={loading || !text.trim()}
            className="flex items-center gap-2 px-4 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {loading ? (
              <>
                <div className="w-4 h-4 border-2 border-primary-foreground border-t-transparent rounded-full animate-spin" />
                检查中...
              </>
            ) : (
              <>
                <Zap className="w-4 h-4" />
                开始检查
              </>
            )}
          </button>
        </div>

        {analysis && (
          <>
            <div className="mt-3 flex gap-1 border-b border-border pb-2">
              <button
                onClick={() => setActiveTab("sensitive")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "sensitive"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <Shield className="w-3 h-3 inline mr-1" />
                敏感词
                {analysis.sensitive_words.total_count > 0 && (
                  <span className="ml-1 px-1.5 py-0.5 text-xs bg-red-500 text-white rounded-full">
                    {analysis.sensitive_words.total_count}
                  </span>
                )}
              </button>
              <button
                onClick={() => setActiveTab("typos")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "typos"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <SpellCheck className="w-3 h-3 inline mr-1" />
                错别字
                {analysis.typos.total_count > 0 && (
                  <span className="ml-1 px-1.5 py-0.5 text-xs bg-yellow-500 text-white rounded-full">
                    {analysis.typos.total_count}
                  </span>
                )}
              </button>
              <button
                onClick={() => setActiveTab("grammar")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "grammar"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <CheckSquare className="w-3 h-3 inline mr-1" />
                语法
                {analysis.grammar.total_count > 0 && (
                  <span className="ml-1 px-1.5 py-0.5 text-xs bg-orange-500 text-white rounded-full">
                    {analysis.grammar.total_count}
                  </span>
                )}
              </button>
              <button
                onClick={() => setActiveTab("format")}
                className={`px-3 py-1.5 text-sm rounded-md transition-colors ${
                  activeTab === "format"
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                }`}
              >
                <FileCheck className="w-3 h-3 inline mr-1" />
                格式
                {analysis.format.changes.length > 0 && (
                  <span className="ml-1 px-1.5 py-0.5 text-xs bg-blue-500 text-white rounded-full">
                    {analysis.format.changes.length}
                  </span>
                )}
              </button>
            </div>

            <div className="mt-3 space-y-3">
              {activeTab === "sensitive" && (
                <div className="space-y-3">
                  <div className="p-3 bg-muted rounded-md">
                    <div className="flex items-center justify-between">
                      <div>
                        <div className="text-xs text-muted-foreground mb-1">敏感词检测</div>
                        <div className="flex items-center gap-2">
                          <span className="text-lg font-semibold">
                            {analysis.sensitive_words.total_count} 个
                          </span>
                          <span
                            className={`px-2 py-0.5 text-xs rounded-md ${getSeverityBadge(analysis.sensitive_words.severity)}`}
                          >
                            {analysis.sensitive_words.severity === "high"
                              ? "高风险"
                              : analysis.sensitive_words.severity === "medium"
                                ? "中风险"
                                : "低风险"}
                          </span>
                        </div>
                      </div>
                      {analysis.sensitive_words.total_count === 0 ? (
                        <CheckCircle className="w-8 h-8 text-green-600" />
                      ) : (
                        <AlertTriangle
                          className={`w-8 h-8 ${getSeverityColor(analysis.sensitive_words.severity)}`}
                        />
                      )}
                    </div>
                  </div>

                  {analysis.sensitive_words.sensitive_words.length > 0 ? (
                    <div className="space-y-2">
                      {analysis.sensitive_words.sensitive_words.map((item, index) => (
                        <div
                          key={index}
                          className={`p-3 border-l-4 rounded-md ${
                            item.severity === "high"
                              ? "bg-red-50 border-red-500"
                              : item.severity === "medium"
                                ? "bg-orange-50 border-orange-500"
                                : "bg-yellow-50 border-yellow-500"
                          }`}
                        >
                          <div className="flex items-center gap-2 mb-1">
                            <span className="font-medium">"{item.word}"</span>
                            <span
                              className={`px-2 py-0.5 text-xs rounded-md ${getSeverityBadge(item.severity)}`}
                            >
                              {item.severity === "high"
                                ? "高"
                                : item.severity === "medium"
                                  ? "中"
                                  : "低"}
                            </span>
                          </div>
                          <div className="text-sm text-muted-foreground">
                            上下文: {item.context}
                          </div>
                          <div className="text-xs text-muted-foreground mt-1">
                            位置: 第 {item.position + 1} 词
                          </div>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <div className="p-4 text-center text-muted-foreground text-sm">
                      未发现敏感词
                    </div>
                  )}
                </div>
              )}

              {activeTab === "typos" && (
                <div className="space-y-3">
                  <div className="p-3 bg-muted rounded-md">
                    <div className="flex items-center justify-between">
                      <div>
                        <div className="text-xs text-muted-foreground mb-1">错别字检测</div>
                        <div className="text-lg font-semibold">{analysis.typos.total_count} 个</div>
                      </div>
                      {analysis.typos.total_count === 0 ? (
                        <CheckCircle className="w-8 h-8 text-green-600" />
                      ) : (
                        <SpellCheck className="w-8 h-8 text-yellow-600" />
                      )}
                    </div>
                  </div>

                  {analysis.typos.typos.length > 0 ? (
                    <div className="space-y-2">
                      {analysis.typos.typos.map((item, index) => (
                        <div
                          key={index}
                          className="p-3 bg-yellow-50 border border-yellow-200 rounded-md"
                        >
                          <div className="flex items-center gap-2 mb-1">
                            <span className="text-red-600 font-medium">"{item.original}"</span>
                            <span className="text-muted-foreground">→</span>
                            <span className="text-green-600 font-medium">"{item.correction}"</span>
                          </div>
                          <div className="text-sm text-muted-foreground">
                            上下文: {item.context}
                          </div>
                          <div className="text-xs text-muted-foreground mt-1">
                            位置: 第 {item.position + 1} 词
                          </div>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <div className="p-4 text-center text-muted-foreground text-sm">
                      未发现错别字
                    </div>
                  )}
                </div>
              )}

              {activeTab === "grammar" && (
                <div className="space-y-3">
                  <div className="p-3 bg-muted rounded-md">
                    <div className="flex items-center justify-between">
                      <div>
                        <div className="text-xs text-muted-foreground mb-1">语法检查</div>
                        <div className="text-lg font-semibold">
                          {analysis.grammar.total_count} 个问题
                        </div>
                      </div>
                      {analysis.grammar.total_count === 0 ? (
                        <CheckCircle className="w-8 h-8 text-green-600" />
                      ) : (
                        <CheckSquare className="w-8 h-8 text-orange-600" />
                      )}
                    </div>
                  </div>

                  {analysis.grammar.grammar_issues.length > 0 ? (
                    <div className="space-y-2">
                      {analysis.grammar.grammar_issues.map((issue, index) => (
                        <div
                          key={index}
                          className="p-3 bg-orange-50 border border-orange-200 rounded-md"
                        >
                          <div className="flex items-center gap-2 mb-1">
                            <AlertTriangle className="w-4 h-4 text-orange-600" />
                            <span className="text-sm font-medium">{issue.issue_type}</span>
                          </div>
                          <div className="text-sm text-muted-foreground mb-2">
                            {issue.description}
                          </div>
                          <div className="text-sm text-blue-600">建议: {issue.suggestion}</div>
                          <div className="text-xs text-muted-foreground mt-1">
                            位置: 第 {issue.position + 1} 行
                          </div>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <div className="p-4 text-center text-muted-foreground text-sm">
                      未发现语法问题
                    </div>
                  )}
                </div>
              )}

              {activeTab === "format" && (
                <div className="space-y-3">
                  <div className="p-3 bg-muted rounded-md">
                    <div className="flex items-center justify-between">
                      <div>
                        <div className="text-xs text-muted-foreground mb-1">格式规范化</div>
                        <div className="text-lg font-semibold">
                          {analysis.format.changes.length} 处修改
                        </div>
                      </div>
                      {analysis.format.changes.length === 0 ? (
                        <CheckCircle className="w-8 h-8 text-green-600" />
                      ) : (
                        <FileCheck className="w-8 h-8 text-blue-600" />
                      )}
                    </div>
                  </div>

                  {analysis.format.changes.length > 0 ? (
                    <div className="space-y-2">
                      {analysis.format.changes.map((change, index) => (
                        <div
                          key={index}
                          className="p-3 bg-blue-50 border border-blue-200 rounded-md"
                        >
                          <div className="flex items-center gap-2 mb-2">
                            <span className="px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded-md">
                              {change.change_type}
                            </span>
                            <span className="text-xs text-muted-foreground">
                              位置: 第 {change.position + 1} 处
                            </span>
                          </div>
                          <div className="flex items-center gap-2 text-sm">
                            <span className="text-red-600 line-through">{change.original}</span>
                            <span className="text-muted-foreground">→</span>
                            <span className="text-green-600">{change.corrected}</span>
                          </div>
                        </div>
                      ))}
                      <div className="mt-4">
                        <div className="text-sm text-muted-foreground mb-2">
                          规范化后的文本预览:
                        </div>
                        <div className="p-3 bg-muted rounded-md text-sm whitespace-pre-wrap max-h-64 overflow-y-auto">
                          {analysis.format.normalized}
                        </div>
                      </div>
                    </div>
                  ) : (
                    <div className="p-4 text-center text-muted-foreground text-sm">
                      格式符合规范，无需修改
                    </div>
                  )}
                </div>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
};
