import React, { useState, useEffect } from "react";
import {
  Save,
  X,
  Image as ImageIcon,
  Video,
  RefreshCw,
  Wand2,
  ChevronLeft,
  ChevronRight,
  Camera,
  User,
  Film,
  Settings,
} from "lucide-react";
import {
  moyinService,
  ScriptScene,
  UpdateSceneRequest,
  CharacterBible,
  AIScene,
  AICharacter,
} from "../services/moyin.service";

interface SceneEditorProps {
  scene: ScriptScene;
  dbPath: string;
  projectId: string;
  characters: CharacterBible[];
  onClose?: () => void;
  onPrevious?: () => void;
  onNext?: () => void;
  onSaved?: (scene: ScriptScene) => void;
  hasPrevious?: boolean;
  hasNext?: boolean;
}

export const SceneEditor: React.FC<SceneEditorProps> = ({
  scene,
  dbPath,
  projectId,
  characters,
  onClose,
  onPrevious,
  onNext,
  onSaved,
  hasPrevious = false,
  hasNext = false,
}) => {
  const [editForm, setEditForm] = useState<UpdateSceneRequest>({
    id: scene.id,
    narration: scene.narration,
    visual_content: scene.visual_content,
    action: scene.action,
    camera: scene.camera,
    character_description: scene.character_description,
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [generatedPrompt, setGeneratedPrompt] = useState<string | null>(null);
  const [showPromptPreview, setShowPromptPreview] = useState(false);
  const [selectedCharacters, setSelectedCharacters] = useState<string[]>([]);

  const cameraOptions = [
    { value: "Close-up", label: "特写" },
    { value: "Medium Shot", label: "中景" },
    { value: "Wide Shot", label: "全景" },
    { value: "Long Shot", label: "远景" },
    { value: "Extreme Close-up", label: "极特写" },
    { value: "Over-the-Shoulder", label: "过肩镜头" },
    { value: "POV", label: "主观视角" },
    { value: "Two Shot", label: "双人镜头" },
    { value: "Establishing Shot", label: "建立镜头" },
  ];

  useEffect(() => {
    setEditForm({
      id: scene.id,
      narration: scene.narration,
      visual_content: scene.visual_content,
      action: scene.action,
      camera: scene.camera,
      character_description: scene.character_description,
    });
  }, [scene]);

  const handleSave = async () => {
    try {
      setLoading(true);
      const updated = await moyinService.updateScriptScene(editForm, dbPath);
      if (updated && onSaved) {
        onSaved(updated);
      }
      setError(null);
    } catch (err) {
      setError("保存失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleGeneratePrompt = async () => {
    try {
      setLoading(true);

      const aiScene: AIScene = {
        scene_id: scene.scene_index,
        narration: editForm.narration || "",
        visual_content: editForm.visual_content || "",
        action: editForm.action || "",
        camera: editForm.camera || "Medium Shot",
        character_description: editForm.character_description || "",
      };

      const aiCharacters: AICharacter[] = characters
        .filter((c) => selectedCharacters.includes(c.id))
        .map((c) => ({
          id: c.id,
          name: c.name,
          type: c.type,
          visual_traits: c.visual_traits,
          style_tokens: c.style_tokens,
          color_palette: c.color_palette,
        }));

      const styleTokens = moyinService.createDefaultStyleTokens();
      const qualityTokens = moyinService.createDefaultQualityTokens();

      const prompt = await moyinService.compileImagePrompt(
        aiScene,
        aiCharacters,
        styleTokens,
        qualityTokens
      );

      setGeneratedPrompt(prompt);
      setShowPromptPreview(true);
      setError(null);
    } catch (err) {
      setError("生成提示词失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const handleUpdateStatus = async (status: string) => {
    try {
      setLoading(true);
      await moyinService.updateSceneGenerationStatus(scene.id, status, dbPath);
      setError(null);
    } catch (err) {
      setError("更新状态失败");
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const toggleCharacterSelection = (characterId: string) => {
    setSelectedCharacters((prev) =>
      prev.includes(characterId) ? prev.filter((id) => id !== characterId) : [...prev, characterId]
    );
  };

  const getStatusBadge = () => {
    const statusColors: Record<string, string> = {
      pending: "bg-gray-100 text-gray-600",
      processing: "bg-blue-100 text-blue-600",
      image_ready: "bg-purple-100 text-purple-600",
      completed: "bg-green-100 text-green-600",
      failed: "bg-red-100 text-red-600",
    };

    const statusLabels: Record<string, string> = {
      pending: "待处理",
      processing: "处理中",
      image_ready: "图像就绪",
      completed: "已完成",
      failed: "失败",
    };

    return (
      <span
        className={`px-2 py-1 rounded-full text-xs ${statusColors[scene.status] || statusColors.pending}`}
      >
        {statusLabels[scene.status] || scene.status}
      </span>
    );
  };

  return (
    <div className="scene-editor h-full flex flex-col bg-white">
      <div className="editor-header p-3 border-b flex items-center justify-between bg-gray-50">
        <div className="flex items-center gap-2">
          <Film className="w-5 h-5 text-gray-600" />
          <h3 className="font-medium">场景 {scene.scene_index + 1}</h3>
          {getStatusBadge()}
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={onPrevious}
            disabled={!hasPrevious}
            className="p-1 hover:bg-gray-200 rounded disabled:opacity-50"
            title="上一个场景"
          >
            <ChevronLeft className="w-5 h-5" />
          </button>
          <button
            onClick={onNext}
            disabled={!hasNext}
            className="p-1 hover:bg-gray-200 rounded disabled:opacity-50"
            title="下一个场景"
          >
            <ChevronRight className="w-5 h-5" />
          </button>
          <button onClick={onClose} className="p-1 hover:bg-gray-200 rounded" title="关闭">
            <X className="w-5 h-5" />
          </button>
        </div>
      </div>

      {error && (
        <div className="error-banner px-3 py-2 bg-red-100 text-red-700 text-sm flex items-center justify-between">
          {error}
          <button onClick={() => setError(null)} className="text-red-500">
            ×
          </button>
        </div>
      )}

      <div className="editor-content flex-1 overflow-y-auto p-4">
        <div className="grid grid-cols-2 gap-4">
          <div className="left-column space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">旁白 / 叙述</label>
              <textarea
                value={editForm.narration || ""}
                onChange={(e) => setEditForm({ ...editForm, narration: e.target.value })}
                className="border rounded px-2 py-1 w-full h-24 resize-none"
                placeholder="场景的叙述性描述..."
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">视觉内容</label>
              <textarea
                value={editForm.visual_content || ""}
                onChange={(e) => setEditForm({ ...editForm, visual_content: e.target.value })}
                className="border rounded px-2 py-1 w-full h-24 resize-none"
                placeholder="场景的视觉元素..."
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">动作描述</label>
              <textarea
                value={editForm.action || ""}
                onChange={(e) => setEditForm({ ...editForm, action: e.target.value })}
                className="border rounded px-2 py-1 w-full h-20 resize-none"
                placeholder="角色的动作..."
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">
                <Camera className="w-4 h-4 inline mr-1" />
                镜头类型
              </label>
              <select
                value={editForm.camera || "Medium Shot"}
                onChange={(e) => setEditForm({ ...editForm, camera: e.target.value })}
                className="border rounded px-2 py-1 w-full"
              >
                {cameraOptions.map((opt) => (
                  <option key={opt.value} value={opt.value}>
                    {opt.label} ({opt.value})
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">角色描述</label>
              <textarea
                value={editForm.character_description || ""}
                onChange={(e) =>
                  setEditForm({ ...editForm, character_description: e.target.value })
                }
                className="border rounded px-2 py-1 w-full h-20 resize-none"
                placeholder="场景中角色的外观..."
              />
            </div>
          </div>

          <div className="right-column space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">
                <User className="w-4 h-4 inline mr-1" />
                关联角色
              </label>
              <div className="border rounded p-2 max-h-32 overflow-y-auto">
                {characters.length === 0 ? (
                  <p className="text-sm text-gray-500">暂无角色圣经</p>
                ) : (
                  <div className="space-y-1">
                    {characters.map((char) => (
                      <label
                        key={char.id}
                        className="flex items-center gap-2 p-1 hover:bg-gray-50 rounded cursor-pointer"
                      >
                        <input
                          type="checkbox"
                          checked={selectedCharacters.includes(char.id)}
                          onChange={() => toggleCharacterSelection(char.id)}
                        />
                        <span className="text-sm">{char.name}</span>
                        <span className="text-xs text-gray-400">({char.type})</span>
                      </label>
                    ))}
                  </div>
                )}
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-600 mb-1">生成资源</label>
              <div className="border rounded p-3 space-y-2">
                {scene.generated_image_url ? (
                  <div>
                    <img
                      src={scene.generated_image_url}
                      alt="生成图像"
                      className="w-full h-40 object-cover rounded"
                    />
                    <a
                      href={scene.generated_image_url}
                      target="_blank"
                      rel="noopener noreferrer"
                      className="text-xs text-blue-500 hover:underline block mt-1"
                    >
                      查看原图
                    </a>
                  </div>
                ) : (
                  <div className="h-24 bg-gray-100 rounded flex items-center justify-center text-gray-400">
                    <ImageIcon className="w-8 h-8" />
                    <span className="ml-2">暂无图像</span>
                  </div>
                )}

                {scene.generated_video_url ? (
                  <div className="mt-2">
                    <video
                      src={scene.generated_video_url}
                      controls
                      className="w-full h-24 object-cover rounded"
                    />
                  </div>
                ) : (
                  <div className="h-16 bg-gray-100 rounded flex items-center justify-center text-gray-400">
                    <Video className="w-6 h-6" />
                    <span className="ml-2 text-sm">暂无视频</span>
                  </div>
                )}
              </div>
            </div>

            <div className="actions space-y-2">
              <button
                onClick={handleGeneratePrompt}
                disabled={loading}
                className="w-full px-3 py-2 bg-purple-500 text-white rounded hover:bg-purple-600 flex items-center justify-center gap-2"
              >
                <Wand2 className="w-4 h-4" />
                生成提示词
              </button>

              <div className="grid grid-cols-2 gap-2">
                <button
                  onClick={() => handleUpdateStatus("processing")}
                  disabled={loading}
                  className="px-3 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 flex items-center justify-center gap-1"
                >
                  <RefreshCw className="w-4 h-4" />
                  重新处理
                </button>
                <button
                  onClick={() => handleUpdateStatus("completed")}
                  disabled={loading}
                  className="px-3 py-2 bg-green-500 text-white rounded hover:bg-green-600 flex items-center justify-center gap-1"
                >
                  标记完成
                </button>
              </div>
            </div>
          </div>
        </div>

        {showPromptPreview && generatedPrompt && (
          <div className="prompt-preview mt-4 p-3 bg-gray-50 border rounded">
            <div className="flex items-center justify-between mb-2">
              <h4 className="font-medium text-sm">生成的提示词</h4>
              <button
                onClick={() => setShowPromptPreview(false)}
                className="text-gray-400 hover:text-gray-600"
              >
                ×
              </button>
            </div>
            <pre className="text-xs text-gray-700 whitespace-pre-wrap bg-white p-2 rounded border">
              {generatedPrompt}
            </pre>
            <button
              onClick={() => navigator.clipboard.writeText(generatedPrompt)}
              className="mt-2 px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded text-sm"
            >
              复制到剪贴板
            </button>
          </div>
        )}
      </div>

      <div className="editor-footer p-3 border-t bg-gray-50 flex justify-between">
        <div className="text-sm text-gray-500">
          创建: {new Date(scene.created_at).toLocaleString()}
          <span className="mx-2">|</span>
          更新: {new Date(scene.updated_at).toLocaleString()}
        </div>
        <div className="flex gap-2">
          <button onClick={onClose} className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300">
            取消
          </button>
          <button
            onClick={handleSave}
            disabled={loading}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 flex items-center gap-2"
          >
            <Save className="w-4 h-4" />
            保存
          </button>
        </div>
      </div>
    </div>
  );
};

export default SceneEditor;
