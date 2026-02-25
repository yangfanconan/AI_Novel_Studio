import React, { useState, useEffect } from "react";
import { X, Download, Trash2, ChevronRight } from "lucide-react";

export const DebugPanel: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [logs, setLogs] = useState<any[]>([]);
  const [filter, setFilter] = useState("");

  useEffect(() => {
    if (!isOpen) return;

    const loadLogs = () => {
      const debugLogs = (window as any).debugLogger?.getLogs() || [];
      setLogs(debugLogs);
    };

    loadLogs();
    const interval = setInterval(loadLogs, 1000);

    return () => clearInterval(interval);
  }, [isOpen]);

  const filteredLogs = logs.filter((log) => {
    const searchTerm = filter.toLowerCase();
    return (
      !searchTerm ||
      JSON.stringify(log).toLowerCase().includes(searchTerm) ||
      log.message.toLowerCase().includes(searchTerm)
    );
  });

  const handleExport = async () => {
    try {
      await (window as any).exportDebugLogs();
      alert("调试日志已导出到 debug_logs.log");
    } catch (error) {
      console.error("Failed to export logs:", error);
      alert("导出失败");
    }
  };

  const handleClear = () => {
    (window as any).debugLogger?.clearLogs();
    setLogs([]);
  };

  const getLevelColor = (level: string) => {
    switch (level) {
      case "ERROR":
        return "text-red-500";
      case "WARN":
        return "text-yellow-500";
      case "INFO":
        return "text-blue-500";
      case "DEBUG":
        return "text-gray-500";
      default:
        return "text-gray-400";
    }
  };

  const getSourceColor = (source: string) => {
    switch (source) {
      case "backend":
        return "text-purple-500";
      case "system":
        return "text-green-500";
      default:
        return "text-blue-400";
    }
  };

  if (!isOpen) {
    return (
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-4 right-4 z-50 bg-primary text-primary-foreground px-3 py-2 rounded-full shadow-lg hover:scale-105 transition-transform"
      >
        <span className="font-mono text-xs">DEBUG</span>
      </button>
    );
  }

  return (
    <div className="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm flex items-center justify-center">
      <div className="w-[90%] h-[90%] bg-background border border-border rounded-lg shadow-2xl flex flex-col overflow-hidden">
        <div className="flex items-center justify-between px-4 py-3 border-b border-border bg-card">
          <div className="flex items-center gap-2">
            <h2 className="font-semibold text-foreground">调试日志</h2>
            <span className="text-xs text-muted-foreground bg-muted px-2 py-0.5 rounded">
              {filteredLogs.length} 条记录
            </span>
          </div>
          <button
            onClick={() => setIsOpen(false)}
            className="p-1 hover:bg-accent rounded-md transition-colors"
          >
            <X className="w-5 h-5 text-foreground" />
          </button>
        </div>

        <div className="flex items-center gap-2 px-4 py-2 border-b border-border bg-muted/50">
          <input
            type="text"
            placeholder="搜索日志..."
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
            className="flex-1 px-3 py-1.5 text-sm bg-background border border-border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
          />
          <button
            onClick={handleExport}
            className="flex items-center gap-1 px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
          >
            <Download className="w-4 h-4" />
            导出
          </button>
          <button
            onClick={handleClear}
            className="flex items-center gap-1 px-3 py-1.5 text-sm bg-destructive text-destructive-foreground rounded-md hover:bg-destructive/90 transition-colors"
          >
            <Trash2 className="w-4 h-4" />
            清空
          </button>
        </div>

        <div className="flex-1 overflow-auto">
          <table className="w-full text-sm">
            <thead className="sticky top-0 bg-background">
              <tr className="border-b border-border">
                <th className="px-2 py-2 text-left text-xs font-medium text-muted-foreground w-32">
                  时间
                </th>
                <th className="px-2 py-2 text-left text-xs font-medium text-muted-foreground w-16">
                  级别
                </th>
                <th className="px-2 py-2 text-left text-xs font-medium text-muted-foreground w-16">
                  来源
                </th>
                <th className="px-2 py-2 text-left text-xs font-medium text-muted-foreground">
                  功能
                </th>
                <th className="px-2 py-2 text-left text-xs font-medium text-muted-foreground">
                  组件
                </th>
                <th className="px-2 py-2 text-left text-xs font-medium text-muted-foreground flex-1">
                  消息
                </th>
              </tr>
            </thead>
            <tbody>
              {filteredLogs.length === 0 ? (
                <tr>
                  <td colSpan={6} className="px-4 py-8 text-center text-muted-foreground">
                    暂无日志记录
                  </td>
                </tr>
              ) : (
                filteredLogs.map((log, index) => (
                  <tr key={index} className="border-b border-border hover:bg-muted/30">
                    <td className="px-2 py-2 text-xs text-muted-foreground">
                      {new Date(log.timestamp).toLocaleTimeString()}
                    </td>
                    <td className={`px-2 py-2 text-xs font-mono ${getLevelColor(log.level)}`}>
                      {log.level}
                    </td>
                    <td className={`px-2 py-2 text-xs font-mono ${getSourceColor(log.source)}`}>
                      {log.source}
                    </td>
                    <td className="px-2 py-2 text-xs">{log.feature || "-"}</td>
                    <td className="px-2 py-2 text-xs">{log.component || "-"}</td>
                    <td className="px-2 py-2 text-xs text-foreground break-all max-w-md">
                      <div className="font-medium">{log.message}</div>
                      {log.data && (
                        <details className="mt-1">
                          <summary className="cursor-pointer text-muted-foreground hover:text-foreground flex items-center gap-1">
                            <ChevronRight className="w-3 h-3" />
                            数据
                          </summary>
                          <pre className="mt-1 p-2 bg-muted rounded text-xs overflow-x-auto">
                            {JSON.stringify(log.data, null, 2)}
                          </pre>
                        </details>
                      )}
                      {log.error && (
                        <div className="mt-1 text-destructive font-mono text-xs">{log.error}</div>
                      )}
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};
