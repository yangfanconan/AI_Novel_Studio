import React, { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface ImportedChapter {
  title: string;
  content: string;
  word_count: number;
}

interface ImportResult {
  success: boolean;
  title: string;
  content: string;
  chapter_count: number;
  word_count: number;
  chapters: ImportedChapter[];
  message: string | null;
}

interface ImportDialogProps {
  isOpen: boolean;
  onClose: () => void;
  projectId?: string;
  onImportSuccess: (result: ImportResult) => void;
}

type ImportFormat = "txt" | "md" | "docx";

interface FormatInfo {
  id: ImportFormat;
  name: string;
  icon: string;
  description: string;
}

const supportedFormats: FormatInfo[] = [
  { id: "txt", name: "TXT 纯文本", icon: "📝", description: "支持章节自动识别" },
  { id: "md", name: "Markdown", icon: "✍️", description: "支持标题解析和YAML前置信息" },
  { id: "docx", name: "Word 文档", icon: "📘", description: "支持.docx格式" },
];

export default function ImportDialog({
  isOpen,
  onClose,
  projectId,
  onImportSuccess,
}: ImportDialogProps) {
  const [selectedFormat, setSelectedFormat] = useState<ImportFormat>("txt");
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [importResult, setImportResult] = useState<ImportResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  if (!isOpen) return null;

  const handleSelectFile = async () => {
    try {
      const extensions =
        selectedFormat === "docx"
          ? ["docx"]
          : selectedFormat === "md"
            ? ["md", "markdown"]
            : ["txt"];

      const selected = await open({
        multiple: false,
        filters: [{ name: "文档文件", extensions }],
      });

      if (selected && typeof selected === "string") {
        setSelectedFile(selected);
        setImportResult(null);
        setError(null);
      }
    } catch (err) {
      console.error("选择文件失败:", err);
      setError("选择文件失败");
    }
  };

  const handleImport = async () => {
    if (!selectedFile) {
      setError("请先选择要导入的文件");
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const result = await invoke<ImportResult>(projectId ? "import_to_project" : "import_file", {
        request: {
          file_path: selectedFile,
          format: selectedFormat,
        },
        ...(projectId && { project_id: projectId }),
      });

      if (result.success) {
        setImportResult(result);
      } else {
        setError("导入失败");
      }
    } catch (err) {
      console.error("导入失败:", err);
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  };

  const handleConfirm = () => {
    if (importResult) {
      onImportSuccess(importResult);
      handleClose();
    }
  };

  const handleClose = () => {
    setSelectedFile(null);
    setImportResult(null);
    setError(null);
    setSelectedFormat("txt");
    onClose();
  };

  const getFileName = (path: string) => {
    return path.split(/[/\\]/).pop() || path;
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-slate-800 rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden">
        <div className="flex items-center justify-between p-4 border-b border-slate-200 dark:border-slate-700">
          <h2 className="text-lg font-semibold text-slate-800 dark:text-slate-100">导入文件</h2>
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

        <div className="p-4 space-y-4 overflow-y-auto">
          {!importResult ? (
            <>
              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  选择格式
                </label>
                <div className="grid grid-cols-3 gap-2">
                  {supportedFormats.map((format) => (
                    <button
                      key={format.id}
                      onClick={() => {
                        setSelectedFormat(format.id);
                        setSelectedFile(null);
                      }}
                      className={`p-3 rounded-lg border-2 text-center transition-colors ${
                        selectedFormat === format.id
                          ? "border-blue-500 bg-blue-50 dark:bg-blue-900/20"
                          : "border-slate-200 dark:border-slate-600 hover:border-slate-300 dark:hover:border-slate-500"
                      }`}
                    >
                      <div className="text-2xl mb-1">{format.icon}</div>
                      <div className="text-sm font-medium text-slate-700 dark:text-slate-300">
                        {format.name}
                      </div>
                      <div className="text-xs text-slate-500 dark:text-slate-400 mt-1">
                        {format.description}
                      </div>
                    </button>
                  ))}
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                  选择文件
                </label>
                <div
                  onClick={handleSelectFile}
                  className="border-2 border-dashed border-slate-300 dark:border-slate-600 rounded-lg p-6 text-center cursor-pointer hover:border-blue-500 transition-colors"
                >
                  {selectedFile ? (
                    <div>
                      <div className="text-3xl mb-2">📄</div>
                      <div className="text-sm font-medium text-slate-700 dark:text-slate-300">
                        {getFileName(selectedFile)}
                      </div>
                    </div>
                  ) : (
                    <div>
                      <div className="text-3xl mb-2">📁</div>
                      <div className="text-sm text-slate-500 dark:text-slate-400">
                        点击选择 {supportedFormats.find((f) => f.id === selectedFormat)?.name} 文件
                      </div>
                    </div>
                  )}
                </div>
              </div>

              {error && (
                <div className="p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg text-sm">
                  {error}
                </div>
              )}
            </>
          ) : (
            <div className="space-y-4">
              <div className="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg">
                <div className="flex items-center gap-2 text-green-600 dark:text-green-400 mb-2">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M5 13l4 4L19 7"
                    />
                  </svg>
                  <span className="font-medium">导入解析成功</span>
                </div>
                <div className="text-sm text-slate-600 dark:text-slate-400">
                  {importResult.message}
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                  <div className="text-xs text-slate-500 dark:text-slate-400">标题</div>
                  <div className="font-medium text-slate-800 dark:text-slate-200">
                    {importResult.title}
                  </div>
                </div>
                <div className="p-3 bg-slate-50 dark:bg-slate-700/50 rounded-lg">
                  <div className="text-xs text-slate-500 dark:text-slate-400">章节/字数</div>
                  <div className="font-medium text-slate-800 dark:text-slate-200">
                    {importResult.chapter_count} 章 / {importResult.word_count} 字
                  </div>
                </div>
              </div>

              {importResult.chapters.length > 0 && (
                <div>
                  <div className="text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
                    章节预览
                  </div>
                  <div className="max-h-40 overflow-y-auto border border-slate-200 dark:border-slate-600 rounded-lg">
                    {importResult.chapters.map((chapter, index) => (
                      <div
                        key={index}
                        className="p-2 border-b border-slate-100 dark:border-slate-700 last:border-b-0 flex justify-between items-center"
                      >
                        <span className="text-sm text-slate-700 dark:text-slate-300">
                          {chapter.title}
                        </span>
                        <span className="text-xs text-slate-500 dark:text-slate-400">
                          {chapter.word_count} 字
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>

        <div className="flex items-center justify-end gap-2 p-4 border-t border-slate-200 dark:border-slate-700">
          {importResult ? (
            <>
              <button
                onClick={handleClose}
                className="px-4 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
              >
                取消
              </button>
              <button
                onClick={handleConfirm}
                className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
              >
                确认导入
              </button>
            </>
          ) : (
            <>
              <button
                onClick={handleClose}
                className="px-4 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200"
              >
                取消
              </button>
              <button
                onClick={handleImport}
                disabled={!selectedFile || isLoading}
                className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? "导入中..." : "开始导入"}
              </button>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
