import React, { useState, useEffect } from 'react';
import {
  GitBranch,
  Clock,
  Save,
  RotateCcw,
  Trash2,
  GitCompare,
  FileText,
  Users,
  Globe,
  LayoutList,
  CheckCircle,
  XCircle,
  Plus,
} from 'lucide-react';
import {
  versionControlService,
  ProjectSnapshot,
  VersionControlConfig,
} from '../services/versionControl.service';

interface VersionControlPanelProps {
  projectId: string;
  onRestore?: () => void;
}

export const VersionControlPanel: React.FC<VersionControlPanelProps> = ({
  projectId,
  onRestore,
}) => {
  const [snapshots, setSnapshots] = useState<ProjectSnapshot[]>([]);
  const [loading, setLoading] = useState(false);
  const [config, setConfig] = useState<VersionControlConfig | null>(null);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showCompareModal, setShowCompareModal] = useState(false);
  const [selectedSnapshot, setSelectedSnapshot] = useState<ProjectSnapshot | null>(null);
  const [compareFrom, setCompareFrom] = useState<ProjectSnapshot | null>(null);
  const [compareTo, setCompareTo] = useState<ProjectSnapshot | null>(null);
  const [newSnapshotVersion, setNewSnapshotVersion] = useState('');
  const [newSnapshotDescription, setNewSnapshotDescription] = useState('');

  useEffect(() => {
    loadSnapshots();
    loadConfig();
  }, [projectId]);

  const loadSnapshots = async () => {
    setLoading(true);
    try {
      const data = await versionControlService.getSnapshots(projectId);
      setSnapshots(data);
    } catch (error) {
      console.error('Failed to load snapshots:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadConfig = async () => {
    try {
      const data = await versionControlService.getVersionConfig();
      setConfig(data);
    } catch (error) {
      console.error('Failed to load config:', error);
    }
  };

  const handleCreateSnapshot = async () => {
    if (!newSnapshotVersion.trim()) return;

    setLoading(true);
    try {
      await versionControlService.createSnapshot(
        projectId,
        newSnapshotVersion,
        newSnapshotDescription,
        false
      );
      setShowCreateModal(false);
      setNewSnapshotVersion('');
      setNewSnapshotDescription('');
      loadSnapshots();
    } catch (error) {
      console.error('Failed to create snapshot:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleRestoreSnapshot = async (snapshotId: string) => {
    if (!confirm('确定要回滚到此版本吗？当前的所有更改将会丢失。')) {
      return;
    }

    setLoading(true);
    try {
      await versionControlService.restoreSnapshot(snapshotId);
      loadSnapshots();
      onRestore?.();
    } catch (error) {
      console.error('Failed to restore snapshot:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteSnapshot = async (snapshotId: string) => {
    if (!confirm('确定要删除此快照吗？此操作无法撤销。')) {
      return;
    }

    setLoading(true);
    try {
      await versionControlService.deleteSnapshot(snapshotId);
      loadSnapshots();
    } catch (error) {
      console.error('Failed to delete snapshot:', error);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  return (
    <div className="border-b border-border bg-card">
      <div className="px-4 py-2">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <GitBranch className="w-4 h-4 text-primary" />
            <span className="text-sm font-medium">版本控制</span>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setShowCreateModal(true)}
              className="flex items-center gap-2 px-3 py-1.5 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
            >
              <Save className="w-3 h-3" />
              创建快照
            </button>
          </div>
        </div>

        {config && (
          <div className="mt-2 p-3 bg-muted rounded-md">
            <div className="flex items-center justify-between text-xs">
              <div className="flex items-center gap-4">
                <div className="flex items-center gap-1">
                  <span className="text-muted-foreground">自动保存:</span>
                  <span className={config.auto_save_enabled ? 'text-green-600' : 'text-muted-foreground'}>
                    {config.auto_save_enabled ? '开启' : '关闭'}
                  </span>
                </div>
                <div className="flex items-center gap-1">
                  <span className="text-muted-foreground">最大快照数:</span>
                  <span>{config.max_snapshots_per_project}</span>
                </div>
              </div>
              <div className="text-muted-foreground">
                {snapshots.length} / {config.max_snapshots_per_project}
              </div>
            </div>
          </div>
        )}

        {loading && (
          <div className="mt-3 text-center text-sm text-muted-foreground">
            加载中...
          </div>
        )}

        {!loading && snapshots.length === 0 && (
          <div className="mt-3 p-4 text-center text-sm text-muted-foreground">
            暂无快照，点击"创建快照"保存当前版本
          </div>
        )}

        {!loading && snapshots.length > 0 && (
          <div className="mt-3 space-y-2 max-h-96 overflow-y-auto">
            {snapshots.map((snapshot) => (
              <div
                key={snapshot.id}
                className="p-3 border rounded-md hover:border-primary transition-colors"
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="font-medium text-sm truncate">
                        {snapshot.version}
                      </span>
                      {snapshot.auto_generated && (
                        <span className="px-1.5 py-0.5 text-xs bg-blue-100 text-blue-800 rounded">
                          自动
                        </span>
                      )}
                    </div>
                    <div className="text-xs text-muted-foreground mb-2">
                      {formatDate(snapshot.timestamp)}
                    </div>
                    {snapshot.description && (
                      <div className="text-xs text-muted-foreground mb-2 truncate">
                        {snapshot.description}
                      </div>
                    )}
                    <div className="flex items-center gap-3 text-xs">
                      <div className="flex items-center gap-1">
                        <FileText className="w-3 h-3" />
                        <span>{snapshot.metadata.total_chapters} 章节</span>
                      </div>
                      <div className="flex items-center gap-1">
                        <Users className="w-3 h-3" />
                        <span>{snapshot.metadata.total_characters} 角色</span>
                      </div>
                      <div className="flex items-center gap-1">
                        <Globe className="w-3 h-3" />
                        <span>{snapshot.world_views.length} 设定</span>
                      </div>
                      <div className="flex items-center gap-1">
                        <LayoutList className="w-3 h-3" />
                        <span>{snapshot.plot_points.length} 剧情点</span>
                      </div>
                      <div className="flex items-center gap-1">
                        <Clock className="w-3 h-3" />
                        <span>{snapshot.metadata.total_words.toLocaleString()} 字</span>
                      </div>
                    </div>
                  </div>

                  <div className="flex items-center gap-1 ml-2">
                    <button
                      onClick={() => {
                        setCompareFrom(snapshots[0]);
                        setCompareTo(snapshot);
                        setShowCompareModal(true);
                      }}
                      className="p-1.5 text-muted-foreground hover:text-primary hover:bg-muted rounded transition-colors"
                      title="比较版本"
                    >
                      <GitCompare className="w-4 h-4" />
                    </button>
                    <button
                      onClick={() => handleRestoreSnapshot(snapshot.id)}
                      className="p-1.5 text-muted-foreground hover:text-green-600 hover:bg-muted rounded transition-colors"
                      title="回滚到此版本"
                    >
                      <RotateCcw className="w-4 h-4" />
                    </button>
                    <button
                      onClick={() => handleDeleteSnapshot(snapshot.id)}
                      className="p-1.5 text-muted-foreground hover:text-red-600 hover:bg-muted rounded transition-colors"
                      title="删除快照"
                    >
                      <Trash2 className="w-4 h-4" />
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {showCreateModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-card rounded-lg shadow-lg p-6 w-full max-w-md">
            <h3 className="text-lg font-semibold mb-4">创建快照</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">版本号</label>
                <input
                  type="text"
                  value={newSnapshotVersion}
                  onChange={(e) => setNewSnapshotVersion(e.target.value)}
                  placeholder="例如: v1.0.0"
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">描述</label>
                <textarea
                  value={newSnapshotDescription}
                  onChange={(e) => setNewSnapshotDescription(e.target.value)}
                  placeholder="描述此版本的主要变更..."
                  rows={3}
                  className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary resize-none"
                />
              </div>
              <div className="flex justify-end gap-2">
                <button
                  onClick={() => setShowCreateModal(false)}
                  className="px-4 py-2 text-sm text-muted-foreground hover:bg-muted rounded-md transition-colors"
                >
                  取消
                </button>
                <button
                  onClick={handleCreateSnapshot}
                  disabled={!newSnapshotVersion.trim() || loading}
                  className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                  {loading ? '创建中...' : '创建'}
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {showCompareModal && compareFrom && compareTo && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-card rounded-lg shadow-lg p-6 w-full max-w-2xl max-h-[80vh] overflow-y-auto">
            <h3 className="text-lg font-semibold mb-4">版本比较</h3>
            <div className="space-y-4">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <div className="text-sm font-medium mb-1">从版本</div>
                  <div className="p-3 bg-muted rounded-md">
                    <div className="font-medium">{compareFrom.version}</div>
                    <div className="text-xs text-muted-foreground mt-1">
                      {formatDate(compareFrom.timestamp)}
                    </div>
                  </div>
                </div>
                <div>
                  <div className="text-sm font-medium mb-1">到版本</div>
                  <div className="p-3 bg-muted rounded-md">
                    <div className="font-medium">{compareTo.version}</div>
                    <div className="text-xs text-muted-foreground mt-1">
                      {formatDate(compareTo.timestamp)}
                    </div>
                  </div>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4 text-sm">
                <div className="p-3 bg-green-50 border border-green-200 rounded-md">
                  <div className="font-medium text-green-800 mb-2">统计变化</div>
                  <div className="space-y-1">
                    <div>章节: {compareTo.metadata.total_chapters - compareFrom.metadata.total_chapters > 0 ? '+' : ''}{compareTo.metadata.total_chapters - compareFrom.metadata.total_chapters}</div>
                    <div>角色: {compareTo.metadata.total_characters - compareFrom.metadata.total_characters > 0 ? '+' : ''}{compareTo.metadata.total_characters - compareFrom.metadata.total_characters}</div>
                    <div>字数: {compareTo.metadata.total_words - compareFrom.metadata.total_words > 0 ? '+' : ''}{(compareTo.metadata.total_words - compareFrom.metadata.total_words).toLocaleString()}</div>
                  </div>
                </div>
                <div className="p-3 bg-blue-50 border border-blue-200 rounded-md">
                  <div className="font-medium text-blue-800 mb-2">元数据变化</div>
                  <div className="space-y-1">
                    <div>设定: {compareTo.world_views.length - compareFrom.world_views.length > 0 ? '+' : ''}{compareTo.world_views.length - compareFrom.world_views.length}</div>
                    <div>剧情点: {compareTo.plot_points.length - compareFrom.plot_points.length > 0 ? '+' : ''}{compareTo.plot_points.length - compareFrom.plot_points.length}</div>
                  </div>
                </div>
              </div>

              <div className="flex justify-end gap-2">
                <button
                  onClick={() => setShowCompareModal(false)}
                  className="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors"
                >
                  关闭
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
