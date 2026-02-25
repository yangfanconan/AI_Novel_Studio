import React, { useState } from 'react';
import { FileText, Download, X } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';

interface ExportDialogProps {
  isOpen: boolean;
  onClose: () => void;
  projectId?: string | null;
  chapterId?: string | null;
  projectName?: string;
}

type ExportFormat = 'pdf' | 'epub' | 'txt' | 'md' | 'docx';

interface ExportFormatInfo {
  id: ExportFormat;
  name: string;
  extension: string;
  icon: string;
  description: string;
}

const exportFormats: ExportFormatInfo[] = [
  {
    id: 'docx',
    name: 'Word文档',
    extension: '.docx',
    icon: '📘',
    description: 'Microsoft Word格式'
  },
  {
    id: 'pdf',
    name: 'PDF文档',
    extension: '.pdf',
    icon: '📄',
    description: '适合打印和阅读的PDF格式'
  },
  {
    id: 'epub',
    name: 'EPUB电子书',
    extension: '.epub',
    icon: '📚',
    description: '适用于电子书阅读器'
  },
  {
    id: 'txt',
    name: '纯文本',
    extension: '.txt',
    icon: '📝',
    description: '简单纯文本格式'
  },
  {
    id: 'md',
    name: 'Markdown',
    extension: '.md',
    icon: '✍️',
    description: 'Markdown格式，便于编辑'
  }
];

export const ExportDialog: React.FC<ExportDialogProps> = ({
  isOpen,
  onClose,
  projectId,
  chapterId,
  projectName,
}) => {
  const [selectedFormat, setSelectedFormat] = useState<ExportFormat>('pdf');
  const [isExporting, setIsExporting] = useState(false);
  const [exportResult, setExportResult] = useState<{
    success: boolean;
    filePath?: string;
    fileSize?: string;
    message: string;
  } | null>(null);

  const handleExport = async () => {
    if (!projectId && !chapterId) {
      setExportResult({
        success: false,
        message: '请选择要导出的项目或章节'
      });
      return;
    }

    setIsExporting(true);
    setExportResult(null);

    try {
      let result: { success: boolean; output_path: string; file_size: number; format: string };

      if (projectId) {
        result = await invoke('export_project', {
          projectId,
          format: selectedFormat
        }) as any;
      } else if (chapterId) {
        result = await invoke('export_chapter', {
          chapterId,
          format: selectedFormat
        }) as any;
      } else {
        throw new Error('无效的导出参数');
      }

      const fileSizeMB = (result.file_size / (1024 * 1024)).toFixed(2);
      
      setExportResult({
        success: result.success,
        filePath: result.output_path,
        fileSize: `${fileSizeMB} MB`,
        message: '导出成功！'
      });
    } catch (error) {
      console.error('Export failed:', error);
      setExportResult({
        success: false,
        message: `导出失败: ${(error as Error).message}`
      });
    } finally {
      setIsExporting(false);
    }
  };

  const handleClose = () => {
    if (!isExporting) {
      setExportResult(null);
      onClose();
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background border border-border rounded-lg shadow-xl w-full max-w-lg mx-4">
        <div className="flex items-center justify-between p-4 border-b border-border">
          <div className="flex items-center gap-2">
            <FileText className="w-5 h-5 text-primary" />
            <h2 className="text-lg font-semibold">导出文档</h2>
          </div>
          <button
            onClick={handleClose}
            disabled={isExporting}
            className="p-1 hover:bg-accent rounded-md transition-colors disabled:opacity-50"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-6">
          {projectName && (
            <div className="mb-4 p-3 bg-accent rounded-lg">
              <p className="text-sm text-muted-foreground">
                导出目标: <span className="font-semibold text-foreground">{projectName}</span>
              </p>
            </div>
          )}

          <div className="mb-4">
            <label className="block text-sm font-medium mb-2 text-foreground">
              选择导出格式
            </label>
            <div className="grid grid-cols-2 gap-3">
              {exportFormats.map((format) => (
                <button
                  key={format.id}
                  onClick={() => setSelectedFormat(format.id)}
                  disabled={isExporting}
                  className={`p-4 border-2 rounded-lg transition-all ${
                    selectedFormat === format.id
                      ? 'border-primary bg-primary/10 ring-2 ring-primary'
                      : 'border-border hover:border-primary/50 hover:bg-accent'
                  } ${isExporting ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
                >
                  <div className="text-3xl mb-2">{format.icon}</div>
                  <div className="font-medium text-foreground mb-1">{format.name}</div>
                  <div className="text-xs text-muted-foreground">{format.extension}</div>
                  <div className="text-xs text-muted-foreground mt-1">
                    {format.description}
                  </div>
                </button>
              ))}
            </div>
          </div>

          {exportResult && (
            <div className={`mb-4 p-4 rounded-lg ${
              exportResult.success ? 'bg-green-500/10 border border-green-500/20' : 'bg-red-500/10 border border-red-500/20'
            }`}>
              <div className="flex items-start gap-2">
                {exportResult.success ? (
                  <Download className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" />
                ) : (
                  <X className="w-5 h-5 text-red-500 flex-shrink-0 mt-0.5" />
                )}
                <div className="flex-1">
                  <p className={`font-medium ${exportResult.success ? 'text-green-700' : 'text-red-700'}`}>
                    {exportResult.message}
                  </p>
                  {exportResult.filePath && (
                    <p className="text-xs text-muted-foreground mt-1">
                      文件路径: {exportResult.filePath}
                    </p>
                  )}
                  {exportResult.fileSize && (
                    <p className="text-xs text-muted-foreground">
                      文件大小: {exportResult.fileSize}
                    </p>
                  )}
                </div>
              </div>
            </div>
          )}

          <div className="flex justify-end gap-2">
            <button
              onClick={handleClose}
              disabled={isExporting}
              className="px-4 py-2 text-sm text-foreground hover:bg-accent rounded-md transition-colors disabled:opacity-50"
            >
              取消
            </button>
            <button
              onClick={handleExport}
              disabled={isExporting}
              className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors disabled:opacity-50 flex items-center gap-2"
            >
              {isExporting ? (
                <>
                  <div className="w-4 h-4 border-2 border-primary-foreground border-t-transparent rounded-full animate-spin" />
                  导出中...
                </>
              ) : (
                <>
                  <Download className="w-4 h-4" />
                  开始导出
                </>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
