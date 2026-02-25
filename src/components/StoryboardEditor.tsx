import React, { useState, useCallback } from 'react';
import {
  Plus,
  Trash2,
  Edit2,
  Play,
  Save,
  ZoomIn,
  ZoomOut,
  Maximize2,
  ChevronLeft,
  ChevronRight,
  Film,
  Clock,
  Camera,
} from 'lucide-react';

export interface Shot {
  id: string;
  scene_number: number;
  shot_number: number;
  description: string;
  camera_angle: string;
  camera_movement: string;
  duration: number;
  action: string;
  dialogue?: string;
  sound?: string;
  notes?: string;
}

export interface StoryboardScene {
  id: string;
  scene_number: number;
  title: string;
  location: string;
  time_of_day: string;
  description: string;
  shots: Shot[];
  estimated_duration: number;
  color_mood: string;
}

interface StoryboardEditorProps {
  scenes: StoryboardScene[];
  onChange?: (scenes: StoryboardScene[]) => void;
  onSave?: () => void;
  readonly?: boolean;
}

export const StoryboardEditor: React.FC<StoryboardEditorProps> = ({
  scenes,
  onChange,
  onSave,
  readonly = false,
}) => {
  const [selectedScene, setSelectedScene] = useState<string | null>(null);
  const [selectedShot, setSelectedShot] = useState<string | null>(null);
  const [zoom, setZoom] = useState(100);
  const [showTimeline, setShowTimeline] = useState(true);
  const [editingShot, setEditingShot] = useState<Shot | null>(null);
  const [isFullscreen, setIsFullscreen] = useState(false);

  const handleAddScene = useCallback(() => {
    if (readonly || !onChange) return;

    const newScene: StoryboardScene = {
      id: `scene_${Date.now()}`,
      scene_number: scenes.length + 1,
      title: `场景 ${scenes.length + 1}`,
      location: '未设定',
      time_of_day: 'day',
      description: '',
      shots: [],
      estimated_duration: 0,
      color_mood: 'neutral',
    };

    onChange([...scenes, newScene]);
    setSelectedScene(newScene.id);
  }, [scenes, onChange, readonly]);

  const handleAddShot = useCallback((sceneId: string) => {
    if (readonly || !onChange) return;

    const scene = scenes.find((s) => s.id === sceneId);
    if (!scene) return;

    const newShot: Shot = {
      id: `shot_${Date.now()}`,
      scene_number: scene.scene_number,
      shot_number: scene.shots.length + 1,
      description: '新镜头',
      camera_angle: '中景',
      camera_movement: '固定',
      duration: 3,
      action: '',
    };

    const updatedScenes = scenes.map((s) =>
      s.id === sceneId
        ? { ...s, shots: [...s.shots, newShot] }
        : s
    );

    onChange(updatedScenes);
    setSelectedShot(newShot.id);
  }, [scenes, onChange, readonly]);

  const handleDeleteScene = useCallback((sceneId: string) => {
    if (readonly || !onChange) return;

    const updatedScenes = scenes.filter((s) => s.id !== sceneId);
    onChange(updatedScenes);

    if (selectedScene === sceneId) {
      setSelectedScene(null);
    }
  }, [scenes, onChange, readonly, selectedScene]);

  const handleDeleteShot = useCallback((sceneId: string, shotId: string) => {
    if (readonly || !onChange) return;

    const updatedScenes = scenes.map((s) =>
      s.id === sceneId
        ? { ...s, shots: s.shots.filter((shot) => shot.id !== shotId) }
        : s
    );

    onChange(updatedScenes);

    if (selectedShot === shotId) {
      setSelectedShot(null);
    }
  }, [scenes, onChange, readonly, selectedShot]);

  const handleUpdateShot = useCallback((sceneId: string, shotId: string, updates: Partial<Shot>) => {
    if (readonly || !onChange) return;

    const updatedScenes = scenes.map((s) =>
      s.id === sceneId
        ? {
            ...s,
            shots: s.shots.map((shot) =>
              shot.id === shotId ? { ...shot, ...updates } : shot
            ),
          }
        : s
    );

    onChange(updatedScenes);
  }, [scenes, onChange, readonly]);

  const handleMoveShot = useCallback((sceneId: string, shotId: string, direction: 'up' | 'down') => {
    if (readonly || !onChange) return;

    const scene = scenes.find((s) => s.id === sceneId);
    if (!scene) return;

    const shotIndex = scene.shots.findIndex((s) => s.id === shotId);
    if (shotIndex === -1) return;

    const newIndex = direction === 'up' ? shotIndex - 1 : shotIndex + 1;
    if (newIndex < 0 || newIndex >= scene.shots.length) return;

    const newShots = [...scene.shots];
    [newShots[shotIndex], newShots[newIndex]] = [newShots[newIndex], newShots[shotIndex]];

    const updatedScenes = scenes.map((s) =>
      s.id === sceneId ? { ...s, shots: newShots } : s
    );

    onChange(updatedScenes);
  }, [scenes, onChange, readonly]);

  const getTotalDuration = () => {
    return scenes.reduce((total, scene) => total + scene.estimated_duration, 0);
  };

  const formatDuration = (seconds: number) => {
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
  };

  return (
    <div className={`flex flex-col h-full bg-background ${isFullscreen ? 'fixed inset-0 z-50' : ''}`}>
      <div className="flex items-center justify-between px-4 py-2 border-b border-border bg-card">
        <div className="flex items-center gap-2">
          <Film className="w-4 h-4 text-primary" />
          <span className="text-sm font-medium">分镜编辑器</span>
          <span className="text-sm text-muted-foreground">
            ({scenes.length} 场景, {scenes.reduce((total, s) => total + s.shots.length, 0)} 镜头)
          </span>
        </div>

        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1 px-3 py-1 bg-muted rounded-md">
            <Clock className="w-3 h-3 text-muted-foreground" />
            <span className="text-sm font-medium">{formatDuration(getTotalDuration())}</span>
          </div>

          <button
            onClick={() => setShowTimeline(!showTimeline)}
            className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
            title={showTimeline ? '隐藏时间轴' : '显示时间轴'}
          >
            <Clock className="w-4 h-4" />
          </button>

          <button
            onClick={() => setZoom(zoom === 100 ? 75 : zoom === 75 ? 50 : 100)}
            className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
            title="缩放"
          >
            {zoom === 100 ? <ZoomIn className="w-4 h-4" /> : zoom === 75 ? <ZoomOut className="w-4 h-4" /> : <Maximize2 className="w-4 h-4" />}
          </button>

          <button
            onClick={() => setIsFullscreen(!isFullscreen)}
            className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
            title={isFullscreen ? '退出全屏' : '全屏'}
          >
            <Maximize2 className="w-4 h-4" />
          </button>

          {onSave && (
            <button
              onClick={onSave}
              className="flex items-center gap-1 px-3 py-1.5 text-sm text-primary-foreground bg-primary rounded-md hover:bg-primary/90 transition-colors"
            >
              <Save className="w-4 h-4" />
              保存
            </button>
          )}
        </div>
      </div>

      <div className="flex-1 overflow-y-auto p-4">
        <div className="max-w-7xl mx-auto space-y-6">
          {scenes.length === 0 ? (
            <div className="text-center py-12">
              <Film className="w-16 h-16 mx-auto text-muted-foreground mb-4" />
              <p className="text-muted-foreground mb-4">还没有分镜场景</p>
              {!readonly && (
                <button
                  onClick={handleAddScene}
                  className="flex items-center gap-2 px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors mx-auto"
                >
                  <Plus className="w-4 h-4" />
                  添加场景
                </button>
              )}
            </div>
          ) : (
            scenes.map((scene) => (
              <div
                key={scene.id}
                className={`border rounded-lg overflow-hidden transition-all ${
                  selectedScene === scene.id ? 'border-primary shadow-lg' : 'border-border'
                }`}
                onClick={() => setSelectedScene(scene.id)}
                style={{
                  backgroundColor: scene.color_mood === 'neutral' ? 'transparent' : `${scene.color_mood}10`,
                }}
              >
                <div className="px-4 py-3 border-b border-border bg-card/50">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <div className="flex items-center gap-2">
                        <span className="text-lg font-bold text-primary">
                          场景 {scene.scene_number}
                        </span>
                        <span className="text-sm text-muted-foreground">{scene.title}</span>
                      </div>
                      <div className="flex items-center gap-2 text-sm text-muted-foreground">
                        <Camera className="w-3 h-3" />
                        <span>{scene.location}</span>
                        <span>·</span>
                        <span>{scene.time_of_day}</span>
                      </div>
                    </div>

                    {!readonly && (
                      <div className="flex items-center gap-1">
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            handleAddShot(scene.id);
                          }}
                          className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
                          title="添加镜头"
                        >
                          <Plus className="w-4 h-4" />
                        </button>
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            handleDeleteScene(scene.id);
                          }}
                          className="p-1.5 text-red-500 hover:text-red-600 hover:bg-red-50 rounded-md transition-colors"
                          title="删除场景"
                        >
                          <Trash2 className="w-4 h-4" />
                        </button>
                      </div>
                    )}
                  </div>

                  <p className="mt-2 text-sm text-muted-foreground">{scene.description}</p>
                </div>

                {scene.shots.length > 0 && (
                  <div className="divide-y divide-border">
                    {scene.shots.map((shot, index) => (
                      <div
                        key={shot.id}
                        className={`px-4 py-3 hover:bg-muted/50 transition-colors ${
                          selectedShot === shot.id ? 'bg-muted/50' : ''
                        }`}
                        onClick={() => setSelectedShot(shot.id)}
                      >
                        <div className="flex items-start justify-between gap-4">
                          <div className="flex-1">
                            <div className="flex items-center gap-3 mb-2">
                              <span className="text-xs font-medium text-muted-foreground px-2 py-0.5 bg-muted rounded">
                                镜头 {shot.shot_number}
                              </span>
                              <span className="text-sm font-medium">{shot.description}</span>
                              <span className="text-sm text-muted-foreground">
                                ({shot.camera_angle} / {shot.camera_movement})
                              </span>
                              <span className="text-xs text-muted-foreground">
                                {shot.duration}s
                              </span>
                            </div>

                            {shot.action && (
                              <p className="text-sm text-muted-foreground mb-1">
                                <span className="font-medium">动作:</span> {shot.action}
                              </p>
                            )}

                            {shot.dialogue && (
                              <p className="text-sm text-muted-foreground mb-1">
                                <span className="font-medium">对话:</span> {shot.dialogue}
                              </p>
                            )}

                            {shot.sound && (
                              <p className="text-sm text-muted-foreground">
                                <span className="font-medium">音效:</span> {shot.sound}
                              </p>
                            )}
                          </div>

                          {!readonly && (
                            <div className="flex items-center gap-1 flex-shrink-0">
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  handleMoveShot(scene.id, shot.id, 'up');
                                }}
                                disabled={index === 0}
                                className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                title="上移"
                              >
                                <ChevronLeft className="w-4 h-4" />
                              </button>
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  handleMoveShot(scene.id, shot.id, 'down');
                                }}
                                disabled={index === scene.shots.length - 1}
                                className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                                title="下移"
                              >
                                <ChevronRight className="w-4 h-4" />
                              </button>
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  setEditingShot(shot);
                                }}
                                className="p-1.5 text-muted-foreground hover:text-foreground hover:bg-muted rounded-md transition-colors"
                                title="编辑"
                              >
                                <Edit2 className="w-4 h-4" />
                              </button>
                              <button
                                onClick={(e) => {
                                  e.stopPropagation();
                                  handleDeleteShot(scene.id, shot.id);
                                }}
                                className="p-1.5 text-red-500 hover:text-red-600 hover:bg-red-50 rounded-md transition-colors"
                                title="删除"
                              >
                                <Trash2 className="w-4 h-4" />
                              </button>
                            </div>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                )}

                {scene.shots.length === 0 && (
                  <div className="px-4 py-8 text-center text-sm text-muted-foreground">
                    还没有镜头，点击上方 + 添加
                  </div>
                )}
              </div>
            ))
          )}
        </div>
      </div>

      {!readonly && (
        <div className="fixed bottom-6 right-6">
          <button
            onClick={handleAddScene}
            className="flex items-center gap-2 px-4 py-3 bg-primary text-primary-foreground rounded-lg shadow-lg hover:bg-primary/90 transition-colors"
          >
            <Plus className="w-5 h-5" />
            添加场景
          </button>
        </div>
      )}
    </div>
  );
};
