import React, { useState } from "react";
import {
  Storyboard,
  StoryboardScene,
  StoryboardShot,
  VisualStyle,
  ShotType,
  CameraAngle,
  CameraMovement,
  TimeOfDay,
  CreateStoryboardRequest,
  CreateStoryboardSceneRequest,
  CreateStoryboardShotRequest,
  UpdateStoryboardShotRequest,
} from "../services/moyin.service";

const VISUAL_STYLES: VisualStyle[] = [
  "Realistic",
  "Anime2D",
  "Anime3D",
  "StopMotion",
  "Watercolor",
  "OilPainting",
  "Sketch",
];

const SHOT_TYPES: ShotType[] = [
  "ExtremeCloseUp",
  "CloseUp",
  "MediumCloseUp",
  "MediumShot",
  "MediumFullShot",
  "FullShot",
  "WideShot",
  "ExtremeWideShot",
  "TwoShot",
  "OverTheShoulder",
  "PointOfView",
  "Establishing",
];

const CAMERA_ANGLES: CameraAngle[] = [
  "EyeLevel",
  "LowAngle",
  "HighAngle",
  "DutchAngle",
  "BirdEye",
  "WormEye",
];

const CAMERA_MOVEMENTS: CameraMovement[] = [
  "Static",
  "PanLeft",
  "PanRight",
  "TiltUp",
  "TiltDown",
  "ZoomIn",
  "ZoomOut",
  "DollyIn",
  "DollyOut",
  "TruckLeft",
  "TruckRight",
  "PedestalUp",
  "PedestalDown",
  "Arc",
  "Crane",
  "Handheld",
  "Steadicam",
];

const TIME_OF_DAY: TimeOfDay[] = [
  "Dawn",
  "Morning",
  "Noon",
  "Afternoon",
  "Evening",
  "Night",
  "Midnight",
];

interface CameraStats {
  total_shots: number;
  shot_types: Record<string, number>;
  angles: Record<string, number>;
  movements: Record<string, number>;
}

export default function StoryboardEditor() {
  const [storyboard, setStoryboard] = useState<Storyboard | null>(null);
  const [selectedSceneId, setSelectedSceneId] = useState<string | null>(null);
  const [selectedShotId, setSelectedShotId] = useState<string | null>(null);
  const [showNewSceneDialog, setShowNewSceneDialog] = useState(false);
  const [showNewShotDialog, setShowNewShotDialog] = useState(false);
  const [showEditShotDialog, setShowEditShotDialog] = useState(false);
  const [activeTab, setActiveTab] = useState<"overview" | "scenes" | "shots" | "export">(
    "overview"
  );

  const [newScene, setNewScene] = useState<Partial<CreateStoryboardSceneRequest>>({
    storyboard_id: "",
    scene_number: 1,
    location: "",
    time_of_day: "Morning",
  });

  const [newShot, setNewShot] = useState<Partial<CreateStoryboardShotRequest>>({
    scene_id: "",
    shot_number: 1,
    shot_type: "MediumShot",
    camera_angle: "EyeLevel",
    camera_movement: "Static",
    subject: "",
    action: "",
    dialogue: "",
    duration: 3.0,
    description: "",
  });

  const [editShot, setEditShot] = useState<Partial<UpdateStoryboardShotRequest>>({
    visual_reference: undefined,
    video_reference: undefined,
    audio_reference: undefined,
    notes: undefined,
  });

  const handleCreateStoryboard = () => {
    const name = prompt("输入分镜名称:");
    if (!name) return;

    const created: Storyboard = {
      id: crypto.randomUUID(),
      project_id: "default",
      name,
      description: "",
      scenes: [],
      visual_style: "Realistic",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    setStoryboard(created);
  };

  const handleCreateScene = () => {
    if (!storyboard || !newScene.location) return;

    const scene: StoryboardScene = {
      id: crypto.randomUUID(),
      storyboard_id: storyboard.id,
      scene_number: newScene.scene_number || 1,
      location: newScene.location,
      time_of_day: newScene.time_of_day || "Morning",
      shots: [],
    };

    setStoryboard({
      ...storyboard,
      scenes: [...storyboard.scenes, scene],
      updated_at: new Date().toISOString(),
    });

    setNewScene({
      ...newScene,
      storyboard_id: storyboard.id,
      scene_number: (newScene.scene_number || 1) + 1,
    });
    setShowNewSceneDialog(false);
  };

  const handleCreateShot = () => {
    if (!storyboard || !selectedSceneId || !newShot.subject || !newShot.action) return;

    const scene = storyboard.scenes.find((s) => s.id === selectedSceneId);
    if (!scene) return;

    const shot: StoryboardShot = {
      id: crypto.randomUUID(),
      scene_id: selectedSceneId,
      shot_number: newShot.shot_number || 1,
      shot_type: newShot.shot_type || "MediumShot",
      camera_angle: newShot.camera_angle || "EyeLevel",
      camera_movement: newShot.camera_movement || "Static",
      subject: newShot.subject,
      action: newShot.action,
      dialogue: newShot.dialogue || undefined,
      duration: newShot.duration || 3.0,
      description: newShot.description || "",
      visual_reference: undefined,
      video_reference: undefined,
      audio_reference: undefined,
      notes: undefined,
    };

    const updatedScenes = storyboard.scenes.map((s) => {
      if (s.id === selectedSceneId) {
        return { ...s, shots: [...s.shots, shot] };
      }
      return s;
    });

    setStoryboard({
      ...storyboard,
      scenes: updatedScenes,
      updated_at: new Date().toISOString(),
    });

    setNewShot({
      ...newShot,
      scene_id: selectedSceneId,
      shot_number: (newShot.shot_number || 1) + 1,
    });
    setShowNewShotDialog(false);
  };

  const handleUpdateShot = () => {
    if (!storyboard || !selectedShotId) return;

    const updatedScenes = storyboard.scenes.map((scene) => {
      if (scene.id === editShot.id) {
        const updatedShots = scene.shots.map((shot) => {
          if (shot.id === selectedShotId) {
            return {
              ...shot,
              visual_reference: editShot.visual_reference,
              video_reference: editShot.video_reference,
              audio_reference: editShot.audio_reference,
              notes: editShot.notes,
            };
          }
          return shot;
        });
        return { ...scene, shots: updatedShots };
      }
      return scene;
    });

    setStoryboard({
      ...storyboard,
      scenes: updatedScenes,
      updated_at: new Date().toISOString(),
    });
    setShowEditShotDialog(false);
  };

  const handleDeleteScene = (sceneId: string) => {
    if (!storyboard) return;
    if (!confirm("确定要删除这个场景吗？")) return;

    setStoryboard({
      ...storyboard,
      scenes: storyboard.scenes.filter((s) => s.id !== sceneId),
      updated_at: new Date().toISOString(),
    });
  };

  const handleDeleteShot = (shotId: string) => {
    if (!storyboard) return;
    if (!confirm("确定要删除这个镜头吗？")) return;

    const updatedScenes = storyboard.scenes.map((scene) => ({
      ...scene,
      shots: scene.shots.filter((s) => s.id !== shotId),
    }));

    setStoryboard({
      ...storyboard,
      scenes: updatedScenes,
      updated_at: new Date().toISOString(),
    });
  };

  const getCameraStats = (): CameraStats | null => {
    if (!storyboard) return null;

    const allShots = storyboard.scenes.flatMap((s) => s.shots);
    const shotTypes: Record<string, number> = {};
    const angles: Record<string, number> = {};
    const movements: Record<string, number> = {};

    allShots.forEach((shot) => {
      shotTypes[shot.shot_type] = (shotTypes[shot.shot_type] || 0) + 1;
      angles[shot.camera_angle] = (angles[shot.camera_angle] || 0) + 1;
      movements[shot.camera_movement] = (movements[shot.camera_movement] || 0) + 1;
    });

    return {
      total_shots: allShots.length,
      shot_types: shotTypes,
      angles,
      movements,
    };
  };

  const calculateTotalDuration = (): number => {
    if (!storyboard) return 0;
    return storyboard.scenes.flatMap((s) => s.shots).reduce((sum, s) => sum + s.duration, 0);
  };

  const exportToCSV = () => {
    if (!storyboard) return;

    let csv = "Scene,Shot,Type,Angle,Movement,Subject,Action,Dialogue,Duration\n";

    for (const scene of storyboard.scenes) {
      for (const shot of scene.shots) {
        csv += `${scene.scene_number},${shot.shot_number},${shot.shot_type},${shot.camera_angle},${shot.camera_movement},${shot.subject},${shot.action},"${shot.dialogue || ""}",${shot.duration}\n`;
      }
    }

    const blob = new Blob([csv], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${storyboard.name}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const exportToJSON = () => {
    if (!storyboard) return;

    const json = JSON.stringify(storyboard, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${storyboard.name}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const selectedScene = storyboard?.scenes.find((s) => s.id === selectedSceneId);
  const selectedShot = selectedScene?.shots.find((s) => s.id === selectedShotId);
  const stats = getCameraStats();
  const totalDuration = calculateTotalDuration();

  return (
    <div className="h-full flex flex-col">
      <div className="flex border-b">
        <button
          onClick={() => setActiveTab("overview")}
          className={`px-4 py-2 text-sm font-medium ${activeTab === "overview" ? "border-b-2 border-blue-500 text-blue-500" : ""}`}
        >
          概览
        </button>
        <button
          onClick={() => setActiveTab("scenes")}
          className={`px-4 py-2 text-sm font-medium ${activeTab === "scenes" ? "border-b-2 border-blue-500 text-blue-500" : ""}`}
        >
          场景
        </button>
        <button
          onClick={() => setActiveTab("shots")}
          className={`px-4 py-2 text-sm font-medium ${activeTab === "shots" ? "border-b-2 border-blue-500 text-blue-500" : ""}`}
        >
          镜头
        </button>
        <button
          onClick={() => setActiveTab("export")}
          className={`px-4 py-2 text-sm font-medium ${activeTab === "export" ? "border-b-2 border-blue-500 text-blue-500" : ""}`}
        >
          导出
        </button>
        <div className="flex-1" />
        {!storyboard && (
          <button
            onClick={handleCreateStoryboard}
            className="px-4 py-2 bg-blue-500 text-white text-sm rounded hover:bg-blue-600"
          >
            新建分镜
          </button>
        )}
      </div>

      <div className="flex-1 overflow-auto p-4">
        {!storyboard ? (
          <div className="flex items-center justify-center h-full text-gray-500">
            <p>点击"新建分镜"开始创建</p>
          </div>
        ) : activeTab === "overview" ? (
          <div className="space-y-4">
            <div className="bg-gray-50 p-4 rounded">
              <h3 className="text-lg font-semibold mb-2">{storyboard.name}</h3>
              <p className="text-gray-600">{storyboard.description || "暂无描述"}</p>
              <div className="mt-2 text-sm text-gray-500">风格: {storyboard.visual_style}</div>
            </div>

            {stats && (
              <div className="grid grid-cols-2 gap-4">
                <div className="bg-blue-50 p-4 rounded">
                  <div className="text-2xl font-bold text-blue-600">{stats.total_shots}</div>
                  <div className="text-sm text-gray-600">总镜头数</div>
                </div>
                <div className="bg-green-50 p-4 rounded">
                  <div className="text-2xl font-bold text-green-600">
                    {totalDuration.toFixed(1)}s
                  </div>
                  <div className="text-sm text-gray-600">总时长</div>
                </div>
              </div>
            )}

            {stats && Object.keys(stats.shot_types).length > 0 && (
              <div className="bg-gray-50 p-4 rounded">
                <h4 className="font-semibold mb-2">镜头类型统计</h4>
                <div className="grid grid-cols-3 gap-2 text-sm">
                  {Object.entries(stats.shot_types).map(([type, count]) => (
                    <div key={type} className="bg-white p-2 rounded">
                      <span className="font-medium">{type}</span>: {count}
                    </div>
                  ))}
                </div>
              </div>
            )}

            {stats && Object.keys(stats.angles).length > 0 && (
              <div className="bg-gray-50 p-4 rounded">
                <h4 className="font-semibold mb-2">拍摄角度统计</h4>
                <div className="grid grid-cols-3 gap-2 text-sm">
                  {Object.entries(stats.angles).map(([angle, count]) => (
                    <div key={angle} className="bg-white p-2 rounded">
                      <span className="font-medium">{angle}</span>: {count}
                    </div>
                  ))}
                </div>
              </div>
            )}

            {stats && Object.keys(stats.movements).length > 0 && (
              <div className="bg-gray-50 p-4 rounded">
                <h4 className="font-semibold mb-2">运镜方式统计</h4>
                <div className="grid grid-cols-3 gap-2 text-sm">
                  {Object.entries(stats.movements).map(([movement, count]) => (
                    <div key={movement} className="bg-white p-2 rounded">
                      <span className="font-medium">{movement}</span>: {count}
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        ) : activeTab === "scenes" ? (
          <div className="space-y-4">
            <button
              onClick={() => {
                setNewScene({
                  storyboard_id: storyboard.id,
                  scene_number: storyboard.scenes.length + 1,
                  location: "",
                  time_of_day: "Morning",
                });
                setShowNewSceneDialog(true);
              }}
              className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
            >
              新建场景
            </button>

            {storyboard.scenes.map((scene) => (
              <div
                key={scene.id}
                className={`border rounded p-4 cursor-pointer ${selectedSceneId === scene.id ? "border-blue-500 bg-blue-50" : ""}`}
                onClick={() => setSelectedSceneId(scene.id)}
              >
                <div className="flex justify-between items-start">
                  <div>
                    <h4 className="font-semibold">
                      场景 {scene.scene_number}: {scene.location}
                    </h4>
                    <p className="text-sm text-gray-600">时间: {scene.time_of_day}</p>
                    <p className="text-sm text-gray-600">镜头数: {scene.shots.length}</p>
                  </div>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleDeleteScene(scene.id);
                    }}
                    className="px-2 py-1 text-red-500 hover:bg-red-50 rounded"
                  >
                    删除
                  </button>
                </div>
              </div>
            ))}
          </div>
        ) : activeTab === "shots" ? (
          <div className="space-y-4">
            {selectedScene ? (
              <>
                <div className="flex justify-between items-center">
                  <h3 className="text-lg font-semibold">
                    场景 {selectedScene.scene_number}: {selectedScene.location}
                  </h3>
                  <button
                    onClick={() => {
                      setNewShot({
                        scene_id: selectedScene.id,
                        shot_number: selectedScene.shots.length + 1,
                        shot_type: "MediumShot",
                        camera_angle: "EyeLevel",
                        camera_movement: "Static",
                        subject: "",
                        action: "",
                        dialogue: "",
                        duration: 3.0,
                        description: "",
                      });
                      setShowNewShotDialog(true);
                    }}
                    className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
                  >
                    新建镜头
                  </button>
                </div>

                {selectedScene.shots.map((shot) => (
                  <div
                    key={shot.id}
                    className={`border rounded p-4 ${selectedShotId === shot.id ? "border-blue-500 bg-blue-50" : ""}`}
                    onClick={() => setSelectedShotId(shot.id)}
                  >
                    <div className="grid grid-cols-4 gap-2 text-sm mb-2">
                      <div>
                        <span className="font-medium">类型:</span> {shot.shot_type}
                      </div>
                      <div>
                        <span className="font-medium">角度:</span> {shot.camera_angle}
                      </div>
                      <div>
                        <span className="font-medium">运镜:</span> {shot.camera_movement}
                      </div>
                      <div>
                        <span className="font-medium">时长:</span> {shot.duration}s
                      </div>
                    </div>
                    <div className="text-sm mb-2">
                      <span className="font-medium">主体:</span> {shot.subject}
                    </div>
                    <div className="text-sm mb-2">
                      <span className="font-medium">动作:</span> {shot.action}
                    </div>
                    {shot.dialogue && <div className="text-sm mb-2 italic">"{shot.dialogue}"</div>}
                    {shot.description && (
                      <div className="text-sm text-gray-600 mb-2">{shot.description}</div>
                    )}
                    {(shot.visual_reference ||
                      shot.video_reference ||
                      shot.audio_reference ||
                      shot.notes) && (
                      <div className="text-sm text-gray-500 mt-2 border-t pt-2">
                        {shot.visual_reference && <div>视觉参考: {shot.visual_reference}</div>}
                        {shot.video_reference && <div>视频参考: {shot.video_reference}</div>}
                        {shot.audio_reference && <div>音频参考: {shot.audio_reference}</div>}
                        {shot.notes && <div>备注: {shot.notes}</div>}
                      </div>
                    )}
                    <div className="flex gap-2 mt-2">
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setEditShot({
                            id: selectedScene.id,
                            visual_reference: shot.visual_reference,
                            video_reference: shot.video_reference,
                            audio_reference: shot.audio_reference,
                            notes: shot.notes,
                          });
                          setShowEditShotDialog(true);
                        }}
                        className="px-2 py-1 text-blue-500 hover:bg-blue-50 rounded text-sm"
                      >
                        编辑参考
                      </button>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          handleDeleteShot(shot.id);
                        }}
                        className="px-2 py-1 text-red-500 hover:bg-red-50 rounded text-sm"
                      >
                        删除
                      </button>
                    </div>
                  </div>
                ))}
              </>
            ) : (
              <p className="text-gray-500">请先选择一个场景</p>
            )}
          </div>
        ) : (
          <div className="space-y-4">
            <div className="flex gap-4">
              <button
                onClick={exportToJSON}
                className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
              >
                导出 JSON
              </button>
              <button
                onClick={exportToCSV}
                className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                导出 CSV
              </button>
            </div>
            <div className="bg-gray-50 p-4 rounded">
              <h4 className="font-semibold mb-2">导出信息</h4>
              <p className="text-sm text-gray-600">分镜名称: {storyboard.name}</p>
              <p className="text-sm text-gray-600">场景数: {storyboard.scenes.length}</p>
              <p className="text-sm text-gray-600">
                镜头数: {storyboard.scenes.reduce((sum, s) => sum + s.shots.length, 0)}
              </p>
              <p className="text-sm text-gray-600">总时长: {totalDuration.toFixed(1)}s</p>
            </div>
          </div>
        )}
      </div>

      {showNewSceneDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
            <h3 className="text-lg font-semibold mb-4">新建场景</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">场景编号</label>
                <input
                  type="number"
                  value={newScene.scene_number || 1}
                  onChange={(e) =>
                    setNewScene({ ...newScene, scene_number: parseInt(e.target.value) })
                  }
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">地点</label>
                <input
                  type="text"
                  value={newScene.location || ""}
                  onChange={(e) => setNewScene({ ...newScene, location: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                  placeholder="例如: 客厅、公园等"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">时间</label>
                <select
                  value={newScene.time_of_day || "Morning"}
                  onChange={(e) =>
                    setNewScene({ ...newScene, time_of_day: e.target.value as TimeOfDay })
                  }
                  className="w-full border rounded px-3 py-2"
                >
                  {TIME_OF_DAY.map((tod) => (
                    <option key={tod} value={tod}>
                      {tod}
                    </option>
                  ))}
                </select>
              </div>
            </div>
            <div className="flex justify-end gap-2 mt-6">
              <button
                onClick={() => setShowNewSceneDialog(false)}
                className="px-4 py-2 border rounded hover:bg-gray-50"
              >
                取消
              </button>
              <button
                onClick={handleCreateScene}
                disabled={!newScene.location}
                className={`px-4 py-2 rounded ${!newScene.location ? "bg-gray-400 cursor-not-allowed" : "bg-blue-500 text-white hover:bg-blue-600"}`}
              >
                创建
              </button>
            </div>
          </div>
        </div>
      )}

      {showNewShotDialog && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-lg overflow-auto max-h-[90vh]">
            <h3 className="text-lg font-semibold mb-4">新建镜头</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">镜头编号</label>
                <input
                  type="number"
                  value={newShot.shot_number || 1}
                  onChange={(e) =>
                    setNewShot({ ...newShot, shot_number: parseInt(e.target.value) })
                  }
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">镜头类型</label>
                <select
                  value={newShot.shot_type || "MediumShot"}
                  onChange={(e) =>
                    setNewShot({ ...newShot, shot_type: e.target.value as ShotType })
                  }
                  className="w-full border rounded px-3 py-2"
                >
                  {SHOT_TYPES.map((type) => (
                    <option key={type} value={type}>
                      {type}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">拍摄角度</label>
                <select
                  value={newShot.camera_angle || "EyeLevel"}
                  onChange={(e) =>
                    setNewShot({ ...newShot, camera_angle: e.target.value as CameraAngle })
                  }
                  className="w-full border rounded px-3 py-2"
                >
                  {CAMERA_ANGLES.map((angle) => (
                    <option key={angle} value={angle}>
                      {angle}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">运镜方式</label>
                <select
                  value={newShot.camera_movement || "Static"}
                  onChange={(e) =>
                    setNewShot({ ...newShot, camera_movement: e.target.value as CameraMovement })
                  }
                  className="w-full border rounded px-3 py-2"
                >
                  {CAMERA_MOVEMENTS.map((movement) => (
                    <option key={movement} value={movement}>
                      {movement}
                    </option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">主体</label>
                <input
                  type="text"
                  value={newShot.subject || ""}
                  onChange={(e) => setNewShot({ ...newShot, subject: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                  placeholder="例如: 主角、汽车等"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">动作</label>
                <textarea
                  value={newShot.action || ""}
                  onChange={(e) => setNewShot({ ...newShot, action: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                  rows={2}
                  placeholder="描述主体的动作"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">对白 (可选)</label>
                <input
                  type="text"
                  value={newShot.dialogue || ""}
                  onChange={(e) => setNewShot({ ...newShot, dialogue: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                  placeholder="角色对白"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">时长 (秒)</label>
                <input
                  type="number"
                  step="0.1"
                  value={newShot.duration || 3.0}
                  onChange={(e) => setNewShot({ ...newShot, duration: parseFloat(e.target.value) })}
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">描述</label>
                <textarea
                  value={newShot.description || ""}
                  onChange={(e) => setNewShot({ ...newShot, description: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                  rows={2}
                  placeholder="镜头的详细描述"
                />
              </div>
            </div>
            <div className="flex justify-end gap-2 mt-6">
              <button
                onClick={() => setShowNewShotDialog(false)}
                className="px-4 py-2 border rounded hover:bg-gray-50"
              >
                取消
              </button>
              <button
                onClick={handleCreateShot}
                disabled={!newShot.subject || !newShot.action}
                className={`px-4 py-2 rounded ${!newShot.subject || !newShot.action ? "bg-gray-400 cursor-not-allowed" : "bg-blue-500 text-white hover:bg-blue-600"}`}
              >
                创建
              </button>
            </div>
          </div>
        </div>
      )}

      {showEditShotDialog && selectedShot && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
            <h3 className="text-lg font-semibold mb-4">编辑镜头参考</h3>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">视觉参考 URL</label>
                <input
                  type="text"
                  value={editShot.visual_reference || ""}
                  onChange={(e) => setEditShot({ ...editShot, visual_reference: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">视频参考 URL</label>
                <input
                  type="text"
                  value={editShot.video_reference || ""}
                  onChange={(e) => setEditShot({ ...editShot, video_reference: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">音频参考 URL</label>
                <input
                  type="text"
                  value={editShot.audio_reference || ""}
                  onChange={(e) => setEditShot({ ...editShot, audio_reference: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">备注</label>
                <textarea
                  value={editShot.notes || ""}
                  onChange={(e) => setEditShot({ ...editShot, notes: e.target.value })}
                  className="w-full border rounded px-3 py-2"
                  rows={3}
                />
              </div>
            </div>
            <div className="flex justify-end gap-2 mt-6">
              <button
                onClick={() => setShowEditShotDialog(false)}
                className="px-4 py-2 border rounded hover:bg-gray-50"
              >
                取消
              </button>
              <button
                onClick={handleUpdateShot}
                className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
              >
                保存
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
