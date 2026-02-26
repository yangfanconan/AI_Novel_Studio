import { useState } from "react";
import { X, Upload } from "lucide-react";

interface InstallPluginDialogProps {
  onClose: () => void;
  onInstall: () => void;
}

export default function InstallPluginDialog({ onClose, onInstall }: InstallPluginDialogProps) {
  const [dragActive, setDragActive] = useState(false);

  const handleDrag = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);

    const files = e.dataTransfer.files;
    if (files.length > 0) {
      handleFileUpload(files[0]);
    }
  };

  const handleFileUpload = async (file: File) => {
    try {
      const content = await file.text();
      console.log("File uploaded:", file.name, content);
      onInstall();
    } catch (error) {
      console.error("Failed to upload file:", error);
      alert("文件上传失败");
    }
  };

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      handleFileUpload(file);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background rounded-lg shadow-xl w-full max-w-2xl mx-4">
        <div className="flex items-center justify-between p-4 border-b border-border">
          <h2 className="text-xl font-semibold">安装插件</h2>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-6">
          <div
            onDragEnter={handleDrag}
            onDragOver={handleDrag}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            className={`border-2 border-dashed rounded-lg p-12 text-center transition-colors ${
              dragActive
                ? "border-primary bg-primary/10"
                : "border-border hover:border-primary/50"
            }`}
          >
            <Upload className="w-12 h-12 mx-auto mb-4 text-muted-foreground" />
            <p className="text-lg font-medium mb-2">拖拽插件文件到此处</p>
            <p className="text-sm text-muted-foreground mb-4">
              或点击下方按钮选择文件
            </p>
            <label className="inline-block">
              <input
                type="file"
                accept=".wasm,.json"
                onChange={handleFileSelect}
                className="hidden"
              />
              <button className="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors">
                选择文件
              </button>
            </label>
          </div>

          <div className="mt-6 p-4 bg-muted rounded-md">
            <h3 className="font-medium mb-2">支持的插件格式</h3>
            <ul className="text-sm text-muted-foreground space-y-1">
              <li>• WASM WebAssembly 插件 (.wasm)</li>
              <li>• JSON 配置插件 (.json)</li>
              <li>• ZIP 压缩包 (.zip)</li>
            </ul>
          </div>
        </div>

        <div className="flex justify-end gap-2 p-4 border-t border-border">
          <button
            onClick={onClose}
            className="px-4 py-2 text-sm rounded-md hover:bg-accent transition-colors"
          >
            取消
          </button>
        </div>
      </div>
    </div>
  );
}
