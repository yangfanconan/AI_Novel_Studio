import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { X, CheckCircle, XCircle, Clock, Loader2, Trash2 } from "lucide-react";

interface BackgroundTask {
  id: string;
  task_type: string;
  status: string;
  payload: string;
  progress: number;
  result?: string;
  error_message?: string;
  created_at: string;
  started_at?: string;
  completed_at?: string;
}

interface TaskProgressPanelProps {
  onClose: () => void;
}

const taskTypeLabels: Record<string, string> = {
  vectorization: "向量化处理",
  summary_generation: "摘要生成",
  ai_review: "AI评审",
  guardrails_check: "护栏检查",
  snapshot: "快照创建",
  export: "导出任务",
};

const getStatusIcon = (status: string) => {
  switch (status) {
    case "completed":
      return <CheckCircle className="w-5 h-5 text-green-500" />;
    case "failed":
      return <XCircle className="w-5 h-5 text-red-500" />;
    case "running":
      return <Loader2 className="w-5 h-5 text-blue-500 animate-spin" />;
    case "cancelled":
      return <XCircle className="w-5 h-5 text-gray-500" />;
    default:
      return <Clock className="w-5 h-5 text-yellow-500" />;
  }
};

const getStatusLabel = (status: string) => {
  switch (status) {
    case "pending":
      return "等待中";
    case "running":
      return "进行中";
    case "completed":
      return "已完成";
    case "failed":
      return "失败";
    case "cancelled":
      return "已取消";
    default:
      return status;
  }
};

export default function TaskProgressPanel({ onClose }: TaskProgressPanelProps) {
  const [tasks, setTasks] = useState<BackgroundTask[]>([]);
  const [loading, setLoading] = useState(true);
  const [filter, setFilter] = useState<"all" | "running" | "completed" | "failed">("all");

  useEffect(() => {
    loadTasks();
    const interval = setInterval(loadTasks, 3000);
    return () => clearInterval(interval);
  }, []);

  const loadTasks = async () => {
    try {
      const allTasks = await invoke<BackgroundTask[]>("get_all_tasks");
      setTasks(allTasks || []);
    } catch (error) {
      console.error("Failed to load tasks:", error);
    } finally {
      setLoading(false);
    }
  };

  const cancelTask = async (taskId: string) => {
    try {
      await invoke("cancel_task", { taskId });
      loadTasks();
    } catch (error) {
      console.error("Failed to cancel task:", error);
    }
  };

  const cleanupTasks = async () => {
    try {
      await invoke("cleanup_completed_tasks", { daysOld: 0 });
      loadTasks();
    } catch (error) {
      console.error("Failed to cleanup tasks:", error);
    }
  };

  const filteredTasks = tasks.filter((task) => {
    if (filter === "all") return true;
    return task.status === filter;
  });

  const runningCount = tasks.filter((t) => t.status === "running").length;
  const pendingCount = tasks.filter((t) => t.status === "pending").length;
  const completedCount = tasks.filter((t) => t.status === "completed").length;
  const failedCount = tasks.filter((t) => t.status === "failed").length;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-background rounded-lg shadow-xl w-full max-w-2xl max-h-[80vh] flex flex-col">
        <div className="flex items-center justify-between p-4 border-b border-border">
          <div className="flex items-center gap-4">
            <h2 className="text-xl font-semibold">后台任务</h2>
            <div className="flex items-center gap-2 text-sm">
              {runningCount > 0 && (
                <span className="px-2 py-0.5 bg-blue-100 text-blue-700 rounded-full">
                  {runningCount} 进行中
                </span>
              )}
              {pendingCount > 0 && (
                <span className="px-2 py-0.5 bg-yellow-100 text-yellow-700 rounded-full">
                  {pendingCount} 等待中
                </span>
              )}
            </div>
          </div>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="flex items-center gap-2 p-4 border-b border-border">
          <button
            onClick={() => setFilter("all")}
            className={`px-3 py-1 rounded-md text-sm ${
              filter === "all" ? "bg-primary text-primary-foreground" : "bg-muted hover:bg-accent"
            }`}
          >
            全部 ({tasks.length})
          </button>
          <button
            onClick={() => setFilter("running")}
            className={`px-3 py-1 rounded-md text-sm ${
              filter === "running" ? "bg-primary text-primary-foreground" : "bg-muted hover:bg-accent"
            }`}
          >
            进行中 ({runningCount})
          </button>
          <button
            onClick={() => setFilter("completed")}
            className={`px-3 py-1 rounded-md text-sm ${
              filter === "completed" ? "bg-primary text-primary-foreground" : "bg-muted hover:bg-accent"
            }`}
          >
            已完成 ({completedCount})
          </button>
          <button
            onClick={() => setFilter("failed")}
            className={`px-3 py-1 rounded-md text-sm ${
              filter === "failed" ? "bg-primary text-primary-foreground" : "bg-muted hover:bg-accent"
            }`}
          >
            失败 ({failedCount})
          </button>
          <div className="flex-1" />
          <button
            onClick={cleanupTasks}
            className="px-3 py-1 text-sm text-muted-foreground hover:text-foreground flex items-center gap-1"
          >
            <Trash2 className="w-4 h-4" />
            清理已完成
          </button>
        </div>

        <div className="flex-1 overflow-auto">
          {loading ? (
            <div className="flex items-center justify-center h-32">
              <Loader2 className="w-6 h-6 animate-spin text-muted-foreground" />
            </div>
          ) : filteredTasks.length === 0 ? (
            <div className="flex items-center justify-center h-32 text-muted-foreground">
              暂无任务
            </div>
          ) : (
            <div className="divide-y divide-border">
              {filteredTasks.map((task) => (
                <div key={task.id} className="p-4 hover:bg-muted/50">
                  <div className="flex items-start justify-between">
                    <div className="flex items-start gap-3">
                      {getStatusIcon(task.status)}
                      <div>
                        <div className="font-medium">
                          {taskTypeLabels[task.task_type] || task.task_type}
                        </div>
                        <div className="text-sm text-muted-foreground mt-1">
                          {task.status === "running" && (
                            <div className="flex items-center gap-2 mt-2">
                              <div className="flex-1 h-2 bg-muted rounded-full overflow-hidden">
                                <div
                                  className="h-full bg-primary transition-all"
                                  style={{ width: `${task.progress * 100}%` }}
                                />
                              </div>
                              <span className="text-xs">{Math.round(task.progress * 100)}%</span>
                            </div>
                          )}
                          {task.error_message && (
                            <p className="text-red-500 mt-1">{task.error_message}</p>
                          )}
                          {task.result && task.status === "completed" && (
                            <p className="text-green-600 mt-1">{task.result}</p>
                          )}
                        </div>
                        <div className="text-xs text-muted-foreground mt-2">
                          创建于 {new Date(task.created_at).toLocaleString()}
                        </div>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      {task.status === "pending" && (
                        <button
                          onClick={() => cancelTask(task.id)}
                          className="text-sm text-red-500 hover:text-red-600"
                        >
                          取消
                        </button>
                      )}
                      <span
                        className={`text-xs px-2 py-0.5 rounded ${
                          task.status === "completed"
                            ? "bg-green-100 text-green-700"
                            : task.status === "failed"
                            ? "bg-red-100 text-red-700"
                            : task.status === "running"
                            ? "bg-blue-100 text-blue-700"
                            : "bg-gray-100 text-gray-700"
                        }`}
                      >
                        {getStatusLabel(task.status)}
                      </span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
