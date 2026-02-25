import React, { useState, useEffect, useCallback } from "react";
import {
  Play,
  Pause,
  Square,
  RefreshCw,
  Plus,
  Trash2,
  Settings,
  Film,
  Image as ImageIcon,
  Video,
  CheckCircle,
  XCircle,
  Clock,
  ChevronDown,
  ChevronUp,
  AlertCircle,
} from "lucide-react";
import {
  moyinService,
  BatchProductionJob,
  BatchProductionConfig,
  CreateBatchJobRequest,
  ProductionProgress,
  ScriptScene,
  CharacterBible,
  SceneStatistics,
  ParsedScreenplay,
} from "../services/moyin.service";

interface BatchProductionPanelProps {
  projectId: string;
  dbPath: string;
  onSceneSelect?: (scene: ScriptScene) => void;
}

export const BatchProductionPanel: React.FC<BatchProductionPanelProps> = ({
  projectId,
  dbPath,
  onSceneSelect,
}) => {
  const [jobs, setJobs] = useState<BatchProductionJob[]>([]);
  const [scenes, setScenes] = useState<ScriptScene[]>([]);
  const [characters, setCharacters] = useState<CharacterBible[]>([]);
  const [statistics, setStatistics] = useState<SceneStatistics | null>(null);
  const [selectedJob, setSelectedJob] = useState<BatchProductionJob | null>(null);
  const [progress, setProgress] = useState<ProductionProgress | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [configDialogOpen, setConfigDialogOpen] = useState(false);
  const [previewScreenplay, setPreviewScreenplay] = useState<ParsedScreenplay | null>(null);
  const [expandedSceneId, setExpandedSceneId] = useState<string | null>(null);

  const [newJobForm, setNewJobForm] = useState<{
    name: string;
    sourceType: "NovelText" | "AiGenerated" | "ChapterContent" | "ExistingScenes";
    sourceContent: string;
    sceneCount: number;
    config: BatchProductionConfig;
  }>({
    name: "",
    sourceType: "NovelText",
    sourceContent: "",
    sceneCount: 5,
    config: moyinService.createDefaultBatchConfig(),
  });

  const loadJobs = useCallback(async () => {
    try {
      const data = await moyinService.getProjectBatchJobs(projectId);
      setJobs(data);
    } catch (err) {
      console.error("加载批量任务失败:", err);
    }
  }, [projectId]);

  const loadScenes = useCallback(async () => {
    try {
      const data = await moyinService.getProjectScriptScenes(projectId, dbPath);
      setScenes(data);
    } catch (err) {
      console.error("加载场景失败:", err);
    }
  }, [projectId, dbPath]);

  const loadCharacters = useCallback(async () => {
    try {
      const data = await moyinService.getCharacterBibles(projectId);
      setCharacters(data);
    } catch (err) {
      console.error("加载角色圣经失败:", err);
    }
  }, [projectId]);

  const loadStatistics = useCallback(async () => {
    try {
      const stats = await moyinService.getSceneStatistics(projectId, dbPath);
      setStatistics(stats);
    } catch (err) {
      console.error("加载统计失败:", err);
    }
  }, [projectId, dbPath]);

  useEffect(() => {
    loadJobs();
    loadScenes();
    loadCharacters();
    loadStatistics();
  }, [loadJobs, loadScenes, loadCharacters, loadStatistics]);

  useEffect(() => {
    if (!selectedJob) return;

    const interval = setInterval(async () => {
      try {
        const prog = await moyinService.getBatchJobProgress(selectedJob.id);
        setProgress(prog);

        if (prog && (prog.current_status === "completed" || prog.current_status === "failed")) {
          loadJobs();
          loadScenes();
          loadStatistics();
        }
      } catch (err) {
        console.error("更新进度失败:", err);
      }
    }, 2000);

    return () => clearInterval(interval);
  }, [selectedJob, loadJobs, loadScenes, loadStatistics]);

  const handleCreateJob = async () => {
    if (!newJobForm.name.trim()) {
      setError("请输入任务名称");
      return;
    }

    try {
      setLoading(true);

      if (newJobForm.sourceType === "NovelText" && newJobForm.sourceContent) {
        const screenplay = await moyinService.parseNovelToScreenplay(
          newJobForm.sourceContent,
          newJobForm.sceneCount
        );
        setPreviewScreenplay(screenplay);
      }

      const request: CreateBatchJobRequest = {
        project_id: projectId,
        name: newJobForm.name,
        source_type: newJobForm.sourceType,
        source_content: newJobForm.sourceContent || undefined,
        scene_count: newJobForm.sceneCount,
        config: newJobForm.config,
      };

      const job = await moyinService.createBatchProductionJob(request);
      setJobs([job, ...jobs]);
      setSelectedJob(job);
      setCreateDialogOpen(false);
      setNewJobForm({
        name: "",
        sourceType: "NovelText",
        sourceContent: "",
        sceneCount: 5,
        config: moyinService.createDefaultBatchConfig(),
      });
      setError(null);
    } catch (err) {
      setError("创建任务失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handlePauseJob = async (jobId: string) => {
    try {
      await moyinService.pauseBatchJob(jobId);
      loadJobs();
    } catch (err) {
      setError("暂停任务失败");
      console.error(err);
    }
  };

  const handleResumeJob = async (jobId: string) => {
    try {
      await moyinService.resumeBatchJob(jobId);
      loadJobs();
    } catch (err) {
      setError("恢复任务失败");
      console.error(err);
    }
  };

  const handleCancelJob = async (jobId: string) => {
    if (!confirm("确定要取消这个任务吗？")) return;

    try {
      await moyinService.cancelBatchJob(jobId);
      loadJobs();
      if (selectedJob?.id === jobId) {
        setSelectedJob(null);
        setProgress(null);
      }
    } catch (err) {
      setError("取消任务失败");
      console.error(err);
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "Pending":
        return <Clock className="w-4 h-4 text-gray-400" />;
      case "Running":
        return <RefreshCw className="w-4 h-4 text-blue-500 animate-spin" />;
      case "Paused":
        return <Pause className="w-4 h-4 text-yellow-500" />;
      case "Completed":
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case "Failed":
        return <XCircle className="w-4 h-4 text-red-500" />;
      case "Cancelled":
        return <Square className="w-4 h-4 text-gray-500" />;
      default:
        return <Clock className="w-4 h-4 text-gray-400" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case "Pending":
        return "bg-gray-100 text-gray-600";
      case "Running":
        return "bg-blue-100 text-blue-600";
      case "Paused":
        return "bg-yellow-100 text-yellow-600";
      case "Completed":
        return "bg-green-100 text-green-600";
      case "Failed":
        return "bg-red-100 text-red-600";
      case "Cancelled":
        return "bg-gray-100 text-gray-500";
      default:
        return "bg-gray-100 text-gray-600";
    }
  };

  const getSceneStatusIcon = (status: string) => {
    switch (status) {
      case "pending":
        return <Clock className="w-4 h-4 text-gray-400" />;
      case "processing":
        return <RefreshCw className="w-4 h-4 text-blue-500 animate-spin" />;
      case "image_ready":
        return <ImageIcon className="w-4 h-4 text-purple-500" />;
      case "completed":
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case "failed":
        return <XCircle className="w-4 h-4 text-red-500" />;
      default:
        return <Clock className="w-4 h-4 text-gray-400" />;
    }
  };

  const renderStatistics = () => {
    if (!statistics) return null;

    return (
      <div className="statistics grid grid-cols-6 gap-2 p-3 bg-gray-50 rounded-lg mb-4">
        <div className="stat-item text-center">
          <div className="text-2xl font-bold text-gray-700">{statistics.total}</div>
          <div className="text-xs text-gray-500">总计</div>
        </div>
        <div className="stat-item text-center">
          <div className="text-2xl font-bold text-gray-400">{statistics.pending}</div>
          <div className="text-xs text-gray-500">等待中</div>
        </div>
        <div className="stat-item text-center">
          <div className="text-2xl font-bold text-blue-500">{statistics.processing}</div>
          <div className="text-xs text-gray-500">处理中</div>
        </div>
        <div className="stat-item text-center">
          <div className="text-2xl font-bold text-purple-500">{statistics.image_ready}</div>
          <div className="text-xs text-gray-500">图像就绪</div>
        </div>
        <div className="stat-item text-center">
          <div className="text-2xl font-bold text-green-500">{statistics.completed}</div>
          <div className="text-xs text-gray-500">已完成</div>
        </div>
        <div className="stat-item text-center">
          <div className="text-2xl font-bold text-red-500">{statistics.failed}</div>
          <div className="text-xs text-gray-500">失败</div>
        </div>
      </div>
    );
  };

  const renderJobList = () => (
    <div className="job-list space-y-2 mb-4">
      <div className="flex items-center justify-between mb-2">
        <h3 className="font-medium">批量任务</h3>
        <button
          onClick={() => setCreateDialogOpen(true)}
          className="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 flex items-center gap-1 text-sm"
        >
          <Plus className="w-4 h-4" />
          新建任务
        </button>
      </div>

      {jobs.length === 0 ? (
        <div className="empty-state p-4 text-center text-gray-500 bg-gray-50 rounded">
          <Film className="w-10 h-10 mx-auto mb-2 opacity-50" />
          <p>暂无批量任务</p>
        </div>
      ) : (
        jobs.map((job) => (
          <div
            key={job.id}
            className={`job-item p-3 border rounded-lg cursor-pointer hover:shadow-sm transition-shadow ${
              selectedJob?.id === job.id ? "border-blue-500 bg-blue-50" : ""
            }`}
            onClick={() => setSelectedJob(job)}
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                {getStatusIcon(job.status)}
                <span className="font-medium">{job.name}</span>
              </div>
              <span className={`px-2 py-0.5 rounded-full text-xs ${getStatusColor(job.status)}`}>
                {job.status}
              </span>
            </div>
            <div className="mt-2 flex items-center justify-between text-sm text-gray-500">
              <span>
                进度: {job.completed_scenes}/{job.total_scenes}
              </span>
              <div className="flex gap-1">
                {job.status === "Running" && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handlePauseJob(job.id);
                    }}
                    className="p-1 hover:bg-gray-200 rounded"
                    title="暂停"
                  >
                    <Pause className="w-4 h-4" />
                  </button>
                )}
                {job.status === "Paused" && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleResumeJob(job.id);
                    }}
                    className="p-1 hover:bg-gray-200 rounded"
                    title="继续"
                  >
                    <Play className="w-4 h-4" />
                  </button>
                )}
                {(job.status === "Running" || job.status === "Paused") && (
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleCancelJob(job.id);
                    }}
                    className="p-1 hover:bg-red-100 text-red-500 rounded"
                    title="取消"
                  >
                    <Square className="w-4 h-4" />
                  </button>
                )}
              </div>
            </div>
            {job.status === "Running" && job.total_scenes > 0 && (
              <div className="mt-2">
                <div className="w-full bg-gray-200 rounded-full h-1.5">
                  <div
                    className="bg-blue-500 h-1.5 rounded-full transition-all"
                    style={{ width: `${(job.completed_scenes / job.total_scenes) * 100}%` }}
                  />
                </div>
              </div>
            )}
          </div>
        ))
      )}
    </div>
  );

  const renderSceneList = () => (
    <div className="scene-list">
      <h3 className="font-medium mb-2">场景列表</h3>
      {scenes.length === 0 ? (
        <div className="empty-state p-4 text-center text-gray-500 bg-gray-50 rounded">
          <ImageIcon className="w-10 h-10 mx-auto mb-2 opacity-50" />
          <p>暂无场景</p>
          <p className="text-sm">创建批量任务来生成场景</p>
        </div>
      ) : (
        <div className="space-y-2 max-h-96 overflow-y-auto">
          {scenes.map((scene) => (
            <div key={scene.id} className="scene-item border rounded-lg overflow-hidden">
              <div
                className="p-3 cursor-pointer hover:bg-gray-50 flex items-center justify-between"
                onClick={() => setExpandedSceneId(expandedSceneId === scene.id ? null : scene.id)}
              >
                <div className="flex items-center gap-2">
                  {getSceneStatusIcon(scene.status)}
                  <span className="font-medium">场景 {scene.scene_index + 1}</span>
                </div>
                <div className="flex items-center gap-2">
                  {scene.generated_image_url && (
                    <span title="已生成图像">
                      <ImageIcon className="w-4 h-4 text-purple-500" />
                    </span>
                  )}
                  {scene.generated_video_url && (
                    <span title="已生成视频">
                      <Video className="w-4 h-4 text-blue-500" />
                    </span>
                  )}
                  {expandedSceneId === scene.id ? (
                    <ChevronUp className="w-4 h-4" />
                  ) : (
                    <ChevronDown className="w-4 h-4" />
                  )}
                </div>
              </div>
              {expandedSceneId === scene.id && (
                <div className="p-3 bg-gray-50 border-t text-sm">
                  <div className="mb-2">
                    <span className="text-gray-500">旁白:</span>
                    <p className="text-gray-700">{scene.narration || "无"}</p>
                  </div>
                  <div className="mb-2">
                    <span className="text-gray-500">视觉内容:</span>
                    <p className="text-gray-700">{scene.visual_content || "无"}</p>
                  </div>
                  <div className="mb-2">
                    <span className="text-gray-500">动作:</span>
                    <p className="text-gray-700">{scene.action || "无"}</p>
                  </div>
                  <div className="mb-2">
                    <span className="text-gray-500">镜头:</span>
                    <span className="ml-1 px-2 py-0.5 bg-blue-100 text-blue-700 rounded text-xs">
                      {scene.camera || "Medium Shot"}
                    </span>
                  </div>
                  {scene.generated_image_url && (
                    <div className="mb-2">
                      <span className="text-gray-500">生成图像:</span>
                      <img
                        src={scene.generated_image_url}
                        alt="生成图像"
                        className="mt-1 max-w-full h-32 object-cover rounded"
                      />
                    </div>
                  )}
                  <button
                    onClick={() => onSceneSelect?.(scene)}
                    className="mt-2 px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded text-sm"
                  >
                    编辑场景
                  </button>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );

  const renderCreateDialog = () => (
    <div className="dialog-overlay fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="dialog bg-white rounded-lg shadow-lg w-full max-w-2xl max-h-[80vh] overflow-y-auto p-4">
        <div className="dialog-header flex items-center justify-between mb-4">
          <h3 className="font-medium">创建批量生产任务</h3>
          <button
            onClick={() => setCreateDialogOpen(false)}
            className="text-gray-400 hover:text-gray-600"
          >
            ×
          </button>
        </div>
        <div className="dialog-body space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-600 mb-1">任务名称 *</label>
            <input
              type="text"
              value={newJobForm.name}
              onChange={(e) => setNewJobForm({ ...newJobForm, name: e.target.value })}
              className="border rounded px-2 py-1 w-full"
              placeholder="输入任务名称"
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-600 mb-1">来源类型</label>
            <select
              value={newJobForm.sourceType}
              onChange={(e) =>
                setNewJobForm({
                  ...newJobForm,
                  sourceType: e.target.value as typeof newJobForm.sourceType,
                })
              }
              className="border rounded px-2 py-1 w-full"
            >
              <option value="NovelText">小说文本</option>
              <option value="AiGenerated">AI 生成</option>
              <option value="ChapterContent">章节内容</option>
              <option value="ExistingScenes">已有场景</option>
            </select>
          </div>

          {newJobForm.sourceType === "NovelText" && (
            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">小说文本</label>
              <textarea
                value={newJobForm.sourceContent}
                onChange={(e) => setNewJobForm({ ...newJobForm, sourceContent: e.target.value })}
                className="border rounded px-2 py-1 w-full h-40 resize-none"
                placeholder="粘贴小说文本，系统将自动解析为场景..."
              />
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-gray-600 mb-1">场景数量</label>
            <input
              type="number"
              value={newJobForm.sceneCount}
              onChange={(e) =>
                setNewJobForm({ ...newJobForm, sceneCount: parseInt(e.target.value) || 5 })
              }
              className="border rounded px-2 py-1 w-32"
              min={1}
              max={50}
            />
          </div>

          <div className="border-t pt-4">
            <h4 className="font-medium mb-2">生成配置</h4>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm text-gray-600 mb-1">图像提供商</label>
                <select
                  value={newJobForm.config.image_provider || "openai"}
                  onChange={(e) =>
                    setNewJobForm({
                      ...newJobForm,
                      config: { ...newJobForm.config, image_provider: e.target.value },
                    })
                  }
                  className="border rounded px-2 py-1 w-full"
                >
                  <option value="openai">OpenAI DALL-E</option>
                  <option value="stability">Stability AI</option>
                  <option value="comfyui">ComfyUI</option>
                </select>
              </div>
              <div>
                <label className="block text-sm text-gray-600 mb-1">并发任务数</label>
                <input
                  type="number"
                  value={newJobForm.config.max_concurrent_tasks}
                  onChange={(e) =>
                    setNewJobForm({
                      ...newJobForm,
                      config: {
                        ...newJobForm.config,
                        max_concurrent_tasks: parseInt(e.target.value) || 3,
                      },
                    })
                  }
                  className="border rounded px-2 py-1 w-full"
                  min={1}
                  max={10}
                />
              </div>
            </div>
            <div className="mt-2">
              <label className="flex items-center gap-2">
                <input
                  type="checkbox"
                  checked={newJobForm.config.retry_failed_tasks}
                  onChange={(e) =>
                    setNewJobForm({
                      ...newJobForm,
                      config: { ...newJobForm.config, retry_failed_tasks: e.target.checked },
                    })
                  }
                />
                <span className="text-sm text-gray-600">自动重试失败任务</span>
              </label>
            </div>
          </div>
        </div>
        <div className="dialog-footer flex justify-end gap-2 mt-4 pt-4 border-t">
          <button
            onClick={() => setCreateDialogOpen(false)}
            className="px-4 py-2 bg-gray-100 rounded hover:bg-gray-200"
          >
            取消
          </button>
          <button
            onClick={handleCreateJob}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            disabled={loading || !newJobForm.name.trim()}
          >
            {loading ? "创建中..." : "创建任务"}
          </button>
        </div>
      </div>
    </div>
  );

  return (
    <div className="batch-production-panel h-full flex flex-col p-4 overflow-y-auto">
      <div className="panel-header mb-4">
        <h2 className="text-lg font-medium flex items-center gap-2">
          <Film className="w-5 h-5" />
          批量生产
        </h2>
        <p className="text-sm text-gray-500 mt-1">管理AI影视内容的批量生成任务</p>
      </div>

      {error && (
        <div className="error-banner mb-4 px-3 py-2 bg-red-100 text-red-700 rounded flex items-center gap-2">
          <AlertCircle className="w-4 h-4" />
          {error}
          <button onClick={() => setError(null)} className="ml-auto text-red-500">
            ×
          </button>
        </div>
      )}

      {renderStatistics()}
      {renderJobList()}
      {renderSceneList()}
      {createDialogOpen && renderCreateDialog()}
    </div>
  );
};

export default BatchProductionPanel;
